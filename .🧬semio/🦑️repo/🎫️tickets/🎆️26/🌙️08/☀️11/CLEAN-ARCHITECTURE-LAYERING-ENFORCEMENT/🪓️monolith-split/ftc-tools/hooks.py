"""🪝 Hook generator: widen restricted struct fields / inherent methods by name, and (re)generate the `testkit` block of a module file.
usage:
  hooks.py fields <name,name,..> <file>...         widen restricted fields (private, pub(crate), pub(super), pub(in ..)) to `pub`; `#[doc(hidden)]` is added when the owning struct is `pub`
  hooks.py methods <name,..> <file>...             widen restricted inherent methods of `impl Type {}` blocks to `pub`
  hooks.py kit <plan.json>                         plan: {"<repo-relative file>": {"alias":["Name",..], "forward":["fn",..], "const":["NAME",..], "reexport":["path as name",..]}}
Writes only the listed repo files, each atomically, and prints a diff summary."""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ftcstub import mask

ROOT = "/Users/ueli/Documents/semio/"
RESTRICT = r"(?:pub\s*\(\s*(?:crate|super|self|in\s+[\w:]+)\s*\)\s*)"


def write(p, s):
    tmp = p + ".ftc-tmp"
    open(tmp, "w", encoding="utf-8").write(s)
    os.replace(tmp, p)


def find_blocks(t, kw_rx):
    """Yield (head_start, head_text, body_start, body_end) for items whose head matches kw_rx followed by `{`."""
    for m in re.finditer(kw_rx, t):
        j = t.find("{", m.end())
        semi = t.find(";", m.end())
        if j < 0 or (0 <= semi < j):
            continue
        depth = 1
        k = j + 1
        while k < len(t) and depth:
            depth += (t[k] == "{") - (t[k] == "}")
            k += 1
        yield m.start(), t[m.start():j], j + 1, k - 1


def widen_fields(path, names):
    src = open(path, encoding="utf-8").read()
    t = mask(src)
    edits = []
    for hs, head, bs, be in find_blocks(t, r"(?m)^[ \t]*(?:pub(?:\s*\([^)]*\))?\s+)?struct\s+\w+(?:<[^{;(]*?>)?(?:\s+where[^{]*)?\s*(?=\{)"):
        pub_struct = re.match(r"\s*pub\s+struct", head) is not None
        body = t[bs:be]
        depth = 0
        i = 0
        start = 0
        segs = []
        while i <= len(body):
            ch = body[i] if i < len(body) else ","
            if ch in "<([{":
                depth += 1
            elif ch in ">)]}":
                depth -= 1 if not (ch == ">" and i > 0 and body[i - 1] == "-") else 0
            if ch == "," and depth == 0:
                segs.append((start, i))
                start = i + 1
            i += 1
        for a, b in segs:
            seg = body[a:b]
            fm = re.match(r"^(\s*(?:#\[[^\]]*\]\s*)*)(pub\s*\(\s*(?:crate|super|self|in\s+[\w:]+)\s*\)\s+|pub\s+)?([A-Za-z_]\w*)\s*:(?!:)", seg)
            if not fm or fm.group(3) not in names:
                continue
            if fm.group(2) and fm.group(2).strip() == "pub":
                continue
            abs_start = bs + a + len(fm.group(1))
            abs_end = abs_start + (len(fm.group(2)) if fm.group(2) else 0)
            repl = ("#[doc(hidden)] " if pub_struct else "") + "pub "
            edits.append((abs_start, abs_end, repl, fm.group(3)))
    out = src
    for a, b, r, n in sorted(edits, reverse=True):
        out = out[:a] + r + out[b:]
    if edits:
        write(path, out)
    return [e[3] for e in edits]


def widen_methods(path, names):
    src = open(path, encoding="utf-8").read()
    t = mask(src)
    edits = []
    for hs, head, bs, be in find_blocks(t, r"(?m)^[ \t]*impl\b[^{;]*?(?=\{)"):
        if re.search(r"\bfor\b", head.split("where")[0]):
            continue
        body = t[bs:be]
        depth = 0
        for m in re.finditer(r"[{}]|(?<![\w])(?:(" + RESTRICT + r")|(?=(?:async\s+|const\s+|unsafe\s+)*fn\s))((?:async\s+|const\s+|unsafe\s+)*)fn\s+(\w+)", body):
            tok = m.group(0)
            if tok == "{":
                depth += 1
                continue
            if tok == "}":
                depth -= 1
                continue
            if depth != 0 or m.group(3) not in names:
                continue
            pre = body[max(0, m.start() - 4):m.start()]
            if re.search(r"\bpub\s*$", body[max(0, m.start() - 6):m.start()]) and not m.group(1):
                continue
            a = bs + m.start()
            b = a + (len(m.group(1)) if m.group(1) else 0)
            edits.append((a, b, "pub " if m.group(1) else "pub ", m.group(3)))
    out = src
    for a, b, r, n in sorted(set(edits), reverse=True):
        out = out[:a] + r + out[b:]
    if edits:
        write(path, out)
    return [e[3] for e in edits]


def generics_of(t, name, kw):
    m = re.search(r"\b" + kw + r"\s+" + re.escape(name) + r"\s*(<)", t)
    if not m:
        return "", ""
    i = m.end()
    depth = 1
    j = i
    while j < len(t) and depth:
        depth += (t[j] == "<") - (t[j] == ">" and t[j - 1] != "-")
        j += 1
    inner = t[i:j - 1]
    parts, d, cur = [], 0, ""
    for ch in inner:
        if ch in "<([":
            d += 1
        if ch in ">)]":
            d -= 1
        if ch == "," and d == 0:
            parts.append(cur); cur = ""
        else:
            cur += ch
    parts.append(cur)
    decl, use = [], []
    for p in parts:
        p = p.strip()
        if not p:
            continue
        nm = re.match(r"(const\s+)?('?\w+)", p)
        decl.append(p.split("=")[0].strip())
        use.append(nm.group(2))
    return "<" + ", ".join(decl) + ">", "<" + ", ".join(use) + ">"


def forwarder(t, name):
    m = re.search(r"(?m)^[ \t]*(?:pub(?:\s*\([^)]*\))?\s+)?((?:async\s+|const\s+|unsafe\s+)*)fn\s+" + re.escape(name) + r"\b", t)
    if not m:
        raise SystemExit(f"free fn {name} not found")
    i = m.end()
    gen = ""
    if t[i:i + 1] == "<":
        depth = 1
        j = i + 1
        while depth:
            depth += (t[j] == "<") - (t[j] == ">" and t[j - 1] != "-")
            j += 1
        gen = t[i:j]
        i = j
    assert t[i] == "(", name
    depth = 1
    j = i + 1
    while depth:
        depth += (t[j] == "(") - (t[j] == ")")
        j += 1
    params = t[i + 1:j - 1]
    k = t.find("{", j)
    tail = t[j:k].strip()
    args = []
    d, cur = 0, ""
    plist = []
    prev = ""
    for ch in params:
        if ch in "<([":
            d += 1
        if ch in ">)]" and not (ch == ">" and prev == "-"):
            d -= 1
        prev = ch
        if ch == "," and d == 0:
            plist.append(cur); cur = ""
        else:
            cur += ch
    plist.append(cur)
    for p in plist:
        p = p.strip()
        if not p:
            continue
        pm = re.match(r"(?:mut\s+)?(\w+)\s*:", p)
        if not pm:
            raise SystemExit(f"unsupported parameter pattern in {name}: {p}")
        args.append(pm.group(1))
    asyn = m.group(1)
    call = f"super::{name}({', '.join(args)})" + (".await" if "async" in asyn else "")
    return f"    pub {asyn}fn {name}{gen}({params}) {tail} {{\n        {call}\n    }}\n".replace("  {\n", " {\n").replace(" {\n", "\n    {\n", 0)


def const_hook(t, name):
    m = re.search(r"\b(const|static)\s+" + re.escape(name) + r"\s*:\s*([^=]+)=", t)
    if not m:
        raise SystemExit(f"const {name} not found")
    return f"    pub {m.group(1)} {name}: {m.group(2).strip()} = super::{name};\n"


def alias_hook(t, name):
    kw = re.search(r"\b(struct|enum|union|type|trait)\s+" + re.escape(name) + r"\b", t)
    if not kw:
        raise SystemExit(f"type {name} not found")
    if kw.group(1) == "trait":
        raise SystemExit("trait aliases are not supported; re-export with a widened trait")
    decl, use = generics_of(t, name, kw.group(1))
    return f"    pub type {name}{decl} = super::{name}{use};\n"


def mark_of(spec):
    return '#[cfg(feature = "' + spec.get("feature", "testkit") + '")]\n#[doc(hidden)]\npub mod ' + spec.get("name", "testkit") + ' {\n'


def block_of(src, t, spec):
    body = []
    if spec.get("forward"):
        body.append("    use super::*;\n")
    for r in spec.get("reexport", []):
        body.append(f"    pub use {r};\n")
    for r in spec.get("raw", []):
        body.append("    " + r + "\n")
    for n in spec.get("alias", []):
        body.append(alias_hook(t, n))
    for n in spec.get("const", []):
        body.append(const_hook(t, n))
    for n in spec.get("forward", []):
        body.append(forwarder(t, n))
    for mod, fns in spec.get("modfwd", {}).items():
        fws = [forwarder(t, n).replace(f"super::{n}(", f"inner::{n}(") for n in fns]
        need = {w for f_ in fws for w in re.findall(r"\b[A-Z]\w*", f_.split("{")[0])} - {"Vec", "String", "Result", "Option", "Box", "Self"}
        body.append(f"    pub mod {mod} {{\n" + ("        use super::super::*;\n" if need else "") + f"        use super::super::{mod} as inner;\n")
        for fw in fws:
            body.append("".join("    " + l + "\n" if l else "\n" for l in fw.rstrip("\n").split("\n")))
        body.append("    }\n")
    return mark_of(spec) + "".join(body) + "}\n"


def kit(plan):
    for rel, specs in plan.items():
        specs = specs if isinstance(specs, list) else [specs]
        path = ROOT + rel
        src = open(path, encoding="utf-8").read()
        cuts = [src.index(mark_of(sp)) for sp in specs if mark_of(sp) in src]
        if cuts:
            src = src[: min(cuts)].rstrip("\n") + "\n"
        t = mask(src)
        text = src.rstrip("\n") + "\n"
        for sp in specs:
            text += "\n" + block_of(src, t, sp)
        write(path, text)
        print(f"kit {rel.split('/')[-2]}: " + "; ".join(f"{sp.get('name', 'testkit')} alias={sp.get('alias', [])} forward={sp.get('forward', [])} const={sp.get('const', [])} reexport={sp.get('reexport', [])}" for sp in specs))


if __name__ == "__main__":
    a = sys.argv[1:]
    if a[0] == "fields":
        names = set(a[1].split(","))
        for f in a[2:]:
            print("fields", os.path.basename(os.path.dirname(os.path.abspath(f))), widen_fields(f, names))
    elif a[0] == "methods":
        names = set(a[1].split(","))
        for f in a[2:]:
            print("methods", os.path.basename(os.path.dirname(os.path.abspath(f))), widen_methods(f, names))
    elif a[0] == "kit":
        kit(json.load(open(a[1], encoding="utf-8")))
