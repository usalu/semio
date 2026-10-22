"""🧩 Macro path rewrite: inside `macro_rules!` bodies, hub paths (`$crate::p` of a hub-defined macro, or `<hub ident>::p` of a macro defined elsewhere) are replaced by `$crate::__private::<producer ident>::<rest>`, and the defining crate gets one `#[doc(hidden)] pub mod __private { pub use <producer>; .. }`. Library: fix_macros(text, resolve, hub_idents, mode) -> (edits, idents, errors); edits are [start, end, old, new] spans of the original text."""
import re, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from repoint_core import skeleton


def macro_bodies(text, sk):
    out = []
    for m in re.finditer(r"macro_rules!\s+(\w+)\s*\{", sk):
        d, j = 1, m.end()
        while j < len(sk) and d:
            d += (sk[j] == "{") - (sk[j] == "}")
            j += 1
        out.append((m.group(1), m.end(), j))
    return out


def fix_macros(text, resolve, hub_idents, mode, own_keep=lambda segs: False, only=None, explicit=None):
    """mode 'crate': rewrite `$crate::<p>` (macro lives in the hub); mode 'ident': rewrite `<hub ident>::<p>`.
    resolve(segs) -> ('rewrite', ident, newsegs) | ('own',) | ('fail', why). own_keep(segs) accepts own items that stay `$crate::<p>`."""
    sk = skeleton(text)
    edits, idents, errors = [], set(), []
    for name, a, b in macro_bodies(text, sk):
        if only and name not in only:
            continue
        body = sk[a:b]
        if mode == "crate":
            pat = re.compile(r"\$crate::([A-Za-z0-9_]+(?:::[A-Za-z0-9_]+)*)")
        else:
            pat = re.compile(r"(?<![A-Za-z0-9_$])(?:::)?(?:%s)::([A-Za-z0-9_]+(?:::[A-Za-z0-9_]+)*)" % "|".join(map(re.escape, sorted(hub_idents))))
        for m in pat.finditer(body):
            segs = m.group(1).split("::")
            if mode == "crate" and segs[0] == "__private":
                continue
            r = ("fail", "unresolved")
            cut = len(segs)
            while cut >= 1:
                r = resolve(segs[:cut])
                if r[0] != "fail":
                    break
                cut -= 1
            s0, s1 = a + m.start(), a + m.start() + len(m.group(0))
            if r[0] == "own" or r[0] == "modalias":
                if mode == "crate" and own_keep(segs):
                    continue
                errors.append(f"{name}: own item {'::'.join(segs)}")
                continue
            if r[0] != "rewrite":
                errors.append(f"{name}: {'::'.join(segs)}: {r[1]}")
                continue
            _, ident, newsegs = r
            tail = segs[cut:]
            path = "::".join([ident] + list(newsegs) + tail)
            end = s1
            if cut < len(segs):
                pass
            idents.add(ident)
            lead = "::" if text[s0:s0 + 2] == "::" else ""
            edits.append([s0, s1, text[s0:s1], "$crate::__private::" + path])
        if explicit is not None:
            ex = re.compile(r"(?<![A-Za-z0-9_$:])(?:::)?(semio_framework_[a-z0-9_]+)(::[A-Za-z0-9_]+(?:::[A-Za-z0-9_]+)*)?(?![A-Za-z0-9_])")
            for m in ex.finditer(body):
                ident = m.group(1)
                if not explicit(ident):
                    continue
                s0, s1 = a + m.start(), a + m.end()
                if text[max(0, s0 - 11):s0] == "__private::":
                    continue
                if any(e[0] <= s0 < e[1] for e in edits):
                    continue
                idents.add(ident)
                edits.append([s0, s1, text[s0:s1], "$crate::__private::" + ident + (m.group(2) or "")])
    return edits, idents, errors


def private_module(idents, gate=None):
    """gate: {ident: cfg attribute text} for producers that only exist under a target cfg or a crate feature."""
    gate = gate or {}
    body = "".join((f"    #[cfg({gate[i]})]\n" if i in gate else "") + f"    pub use {i};\n" for i in sorted(idents))
    return "\n/// 🔒️ Producer crates the exported macros expand against, so invoking crates need no dependency on them.\n#[doc(hidden)]\npub mod __private {\n" + body + "}\n"
