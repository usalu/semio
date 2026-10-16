"""🔗 POS union builder: reads the source closure of each family crate (working tree or a git revision, read-only), rewrites it for the single-crate union and writes it into a sandbox only.
Each crate gets its own copy `<sandbox>/<__m_crate>/<repo-relative path>` so relative `#[path]` and `include!` keep resolving and a file shared by two crates stays independent.
Rewrites: `crate::` -> `crate::__m_<crate>::`, `pub(crate)` -> `pub(in crate::__m_<crate>)`, family crate heads -> `crate::__m_<dep>`, other crate heads -> `crate::__ext::<dep>`, per-crate feature cfgs, env!, derives/attributes of external proc macros."""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ftclib import *
from ftcstub import mask, DEFINED_RX

PATH_RX = re.compile(r"#\[[^\]]*?\bpath\s*=\s*\"([^\"]+)\"")
INC_CODE_RX = re.compile(r"\binclude!\s*\(\s*\"([^\"]+)\"\s*\)")
INC_RAW_RX = re.compile(r"\binclude_(?:str|bytes)!\s*\(\s*\"([^\"]+)\"\s*\)")
INC_MAN_RX = re.compile(r"\binclude(_str|_bytes|)!\s*\(\s*concat!\s*\(\s*env!\s*\(\s*\"CARGO_MANIFEST_DIR\"\s*\)\s*,\s*\"([^\"]+)\"\s*\)\s*\)")
MOD_RX = re.compile(r"^[ \t]*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_]\w*)\s*;", re.M)
STD_DERIVES = {"Debug", "Clone", "Copy", "PartialEq", "Eq", "Hash", "PartialOrd", "Ord", "Default"}
DROP_INNER = re.compile(r"^(feature|no_std|no_main|crate_type|crate_name|recursion_limit|type_length_limit|forbid|deny|warn|expect|cfg_attr\s*\(.*(?:no_std|feature|forbid|deny|warn).*\))", re.S)
MAX_RAW = 6_000_000


def reach(src, lib_rel, manifest_dir):
    """Transitive closure of files mounted through #[path], include!, include_str!/include_bytes! and implicit `mod x;`. Returns ({rel: bytes}, {code rels}, [missing])."""
    files, code, missing = {}, set(), []
    stack = [(lib_rel, True)]
    seen = set()
    while stack:
        rel, is_code = stack.pop()
        if (rel, is_code) in seen:
            continue
        seen.add((rel, is_code))
        data = src.read(rel)
        if data is None:
            missing.append(rel)
            continue
        if len(data) > MAX_RAW and not is_code:
            data = b""
        files[rel] = data
        if not is_code:
            continue
        code.add(rel)
        text = data.decode("utf-8", "replace")
        d = os.path.dirname(rel)
        for m in PATH_RX.finditer(text):
            stack.append((norm(os.path.join(d, m.group(1))), True))
        for m in INC_CODE_RX.finditer(text):
            tgt = norm(os.path.join(d, m.group(1)))
            stack.append((tgt, tgt.endswith(".rs")))
        for m in INC_RAW_RX.finditer(text):
            stack.append((norm(os.path.join(d, m.group(1))), False))
        for m in INC_MAN_RX.finditer(text):
            tgt = norm(os.path.join(manifest_dir, m.group(2).lstrip("/")))
            stack.append((tgt, False))
        stem = os.path.splitext(os.path.basename(rel))[0]
        masked = mask(text)
        for m in MOD_RX.finditer(masked):
            n = m.group(1)
            for cand in (os.path.join(d, n + ".rs"), os.path.join(d, n, "mod.rs"), os.path.join(d, stem, n + ".rs"), os.path.join(d, stem, n, "mod.rs")):
                cand = norm(cand)
                if src.exists(cand):
                    stack.append((cand, True))
    return files, code, missing


def balanced_end(text, i):
    depth = 0
    n = len(text)
    while i < n:
        c = text[i]
        if c == "[":
            depth += 1
        elif c == "]":
            depth -= 1
            if depth == 0:
                return i + 1
        i += 1
    return n


class Crate:
    def __init__(self, pkg, man, src, lib_rel, mans):
        self.pkg = pkg
        self.man = man
        self.src = src
        self.lib_rel = lib_rel
        self.id = ident(pkg)
        self.mod = "__m_" + self.id
        self.libname = mans.lib_name(man)
        self.files = {}
        self.code = set()
        self.missing = []
        self.features = set()
        self.externs = {}
        self.aliases = {}
        self.deps = []
        self.macros = []


def feature_closure(crates, cfg_test, mans):
    """Cargo-style feature unification over the family: defaults of every member + features requested by family dependents."""
    en = {c.pkg: set() for c in crates}
    fam = set(en)
    for c in crates:
        tab = mans.features_table(c.man)
        if "default" in tab:
            en[c.pkg].add("default")
        for kind, name, pkg, feats, opt, renamed in c.deps:
            if pkg in fam and (kind == "dependencies" or (cfg_test and kind == "dev-dependencies")):
                en[pkg].update(feats)
    changed = True
    while changed:
        changed = False
        for c in crates:
            tab = mans.features_table(c.man)
            for f in list(en[c.pkg]):
                for entry in tab.get(f, []):
                    if entry.startswith("dep:"):
                        continue
                    if "/" in entry:
                        dname, ff = entry.split("/", 1)
                        dname = dname.rstrip("?")
                        for kind, name, pkg, feats, opt, renamed in c.deps:
                            if name == dname and pkg in fam and ff not in en[pkg]:
                                en[pkg].add(ff)
                                changed = True
                    elif entry not in en[c.pkg]:
                        en[c.pkg].add(entry)
                        changed = True
    return en


def prepare(src, mans, pkgs, cfg_test):
    """Build Crate objects (sources read, extern tables resolved) for the family packages that exist at this source."""
    crates = []
    for pkg in pkgs:
        man = mans.manifest(pkg, src)
        if man is None:
            continue
        lib_rel = mans.lib_rel(man)
        c = Crate(pkg, man, src, lib_rel, mans)
        c.files, c.code, c.missing = reach(src, lib_rel, man["dir"])
        if lib_rel not in c.files:
            continue
        c.deps = mans.deps(man)
        crates.append(c)
    fam = {c.pkg: c for c in crates}
    for c in crates:
        c.externs[ident(c.libname)] = ("fam", c.pkg)
        c.externs[c.id] = ("fam", c.pkg)
        for kind, name, pkg, feats, opt, renamed in c.deps:
            if renamed:
                ext_name = ident(name)
            else:
                dm = mans.manifest(pkg, src) or mans.manifest(pkg)
                ext_name = ident(mans.lib_name(dm)) if dm else ident(name)
            c.externs[ext_name] = ("fam", pkg) if pkg in fam else ("ext", pkg)
        for rel in c.code:
            t = c.files[rel].decode("utf-8", "replace")
            for m in re.finditer(r"^\s*extern\s+crate\s+self\s+as\s+(\w+)\s*;", t, re.M):
                c.aliases[m.group(1)] = ("fam", c.pkg)
    en = feature_closure(crates, cfg_test, mans)
    for c in crates:
        c.features = en[c.pkg]
    return crates


def defined_names(crates):
    out = set()
    for c in crates:
        for rel in c.code:
            for m in DEFINED_RX.finditer(mask(c.files[rel].decode("utf-8", "replace"))):
                out.add(m.group(1) or m.group(2) or m.group(3))
    return out




def lib_notest(text):
    """Replace the bare `test` predicate inside every cfg(...)/cfg_attr(...) of a lib that its companion tests as a normal dependency (cargo builds it without cfg(test))."""
    m = mask(text)
    out = list(text)
    for cm in re.finditer(r"\bcfg(?:_attr)?!?\s*\(", m):
        i = cm.end()
        depth = 1
        j = i
        while j < len(m) and depth:
            if m[j] == "(":
                depth += 1
            elif m[j] == ")":
                depth -= 1
            j += 1
        for tm in re.finditer(r"(?<![\w:.$])test(?![\w])", m[i:j - 1]):
            a = i + tm.start()
            out[a:a + 4] = list("ftcN")
    return "".join(out)

def rewrite_code(c, text, rel, fam, strip_attrs, notes):
    mod = c.mod
    t = lib_notest(text) if c.pkg in notes.get("notest", ()) else text
    t = re.sub(r"feature\s*=\s*\"([^\"]+)\"", lambda m: f'feature = "{c.id}:{m.group(1)}"', t)
    mdir = os.path.join(SBX_CURRENT_DIR[0], mod, c.man["dir"]) if SBX_CURRENT_DIR else ""
    t = re.sub(r"\b(option_)?env!\s*\(\s*\"CARGO_MANIFEST_DIR\"\s*\)", lambda m: (f'Some("{mdir}")' if m.group(1) else f'"{mdir}"'), t)
    t = re.sub(r"\b(option_)?env!\s*\(\s*\"CARGO_PKG_NAME\"\s*\)", lambda m: (f'Some("{c.pkg}")' if m.group(1) else f'"{c.pkg}"'), t)
    t = re.sub(r"\b(option_)?env!\s*\(\s*\"CARGO_CRATE_NAME\"\s*\)", lambda m: (f'Some("{c.id}")' if m.group(1) else f'"{c.id}"'), t)
    t = re.sub(r"\b(option_)?env!\s*\(\s*\"CARGO_PKG_VERSION(?:_[A-Z]+)?\"\s*\)", lambda m: ('Some("0.1.0")' if m.group(1) else '"0.1.0"'), t)
    t = re.sub(r"(?:#\[[^\]]*\]\s*)*#\[global_allocator\]\s*(?:pub\s+)?static\s+[^;]*;", "", t)

    def derive_fix(m):
        items = [s.strip() for s in m.group(1).split(",") if s.strip()]
        keep = []
        for s in items:
            leaf = s.split("::")[-1].strip()
            if leaf in STD_DERIVES and (("::" not in s) or s.startswith(("core::", "std::", "::core::", "::std::"))):
                keep.append(s)
        return "derive(" + ", ".join(keep) + ")"

    t = re.sub(r"derive\(([^()]*)\)", derive_fix, t)
    for a in strip_attrs:
        t = re.sub(r"#!?\[\s*" + re.escape(a) + r"\s*(?:\((?:[^()\]]|\((?:[^()]|\([^()]*\))*\))*\))?\s*\]", "", t)
    m = mask(t)
    feats = []
    spans = []
    for im in re.finditer(r"#!\[", m):
        end = balanced_end(m, im.end() - 1)
        body = t[im.end():end - 1].strip()
        if DROP_INNER.match(body):
            spans.append((im.start(), end))
            fm = re.match(r"feature\s*\(([^)]*)\)", body)
            if fm:
                feats += [x.strip() for x in fm.group(1).split(",") if x.strip()]
    if spans:
        for a, b in reversed(spans):
            t = t[:a] + " " * 0 + t[b:]
        m = mask(t)
    edits = []
    for km in re.finditer(r"pub\s*\(\s*crate\s*\)", m):
        edits.append((km.start(), km.end(), f"pub(in crate::{mod})"))
    for km in re.finditer(r"\$crate\s*::", m):
        edits.append((km.start(), km.end(), f"$crate::{mod}::"))
    for km in re.finditer(r"^\s*extern\s+crate\s+(\w+)(?:\s+as\s+\w+)?\s*;", m, re.M):
        edits.append((km.start(), km.end(), ""))
    names = [n for n, v in c.externs.items() if v[0] == "fam"] + [n for n, v in c.externs.items() if v[0] == "ext"] + list(c.aliases)
    heads = sorted(set(names), key=len, reverse=True)
    localmods = notes.setdefault("localmods", {})
    if "headrx" not in notes:
        notes["headrx"] = {}
    key = c.pkg
    if key not in notes["headrx"]:
        excl = notes.get("excluded", {}).get(key, set())
        alt = "|".join(re.escape(h) for h in heads if h not in excl)
        notes["headrx"][key] = re.compile(r"(?<![\w$:.])(::)?\b(crate|" + alt + r")\b(?=\s*::)") if alt else re.compile(r"(?<![\w$:.])(::)?\b(crate)\b(?=\s*::)")
    for hm in notes["headrx"][key].finditer(m):
        name = hm.group(2)
        if name == "crate":
            if hm.group(1):
                continue
            edits.append((hm.start(), hm.end(), f"crate::{mod}"))
            continue
        tgt = c.externs.get(name) or c.aliases.get(name)
        if tgt[0] == "fam":
            edits.append((hm.start(), hm.end(), f"crate::__m_{ident(tgt[1])}"))
        else:
            edits.append((hm.start(), hm.end(), f"crate::__ext::{name}"))
    for um in re.finditer(r"\buse\s+(?:::)?(\w+)(\s+as\s+(\w+))?\s*;", m):
        name = um.group(1)
        if name in c.externs and name not in ("self", "crate", "super"):
            vis = re.search(r"pub(?:\s*\([^)]*\))?\s*$", m[max(0, um.start() - 40):um.start()])
            if (um.group(3) and um.group(3) != name) or vis:
                tgt = c.externs[name]
                edits.append((um.start(1), um.end(1), f"crate::__m_{ident(tgt[1])}" if tgt[0] == "fam" else f"crate::__ext::{name}"))
            else:
                edits.append((um.start(), um.end(), ""))
    edits.sort(key=lambda e: (e[0], -e[1]))
    out, last = [], 0
    for a, b, rep in edits:
        if a < last:
            continue
        out.append(t[last:a])
        out.append(rep)
        last = b
    out.append(t[last:])
    return "".join(out), feats


SBX_CURRENT_DIR = []


def find_exports(c):
    names = []
    for rel in sorted(c.code):
        mt = mask(c.files[rel].decode("utf-8", "replace"))
        for m in re.finditer(r"#\[\s*macro_export\s*(?:\([^\]]*\))?\s*\](?:\s*#\[[^\]]*\])*\s*macro_rules!\s*(\w+)", mt):
            names.append(m.group(1))
    return names


def excluded_heads(crates):
    """Extern names that collide with a module/alias declared inside the crate: left unrewritten (noted)."""
    res = {}
    for c in crates:
        local = set()
        for rel in c.code:
            for m in re.finditer(r"\bmod\s+(\w+)\b", mask(c.files[rel].decode("utf-8", "replace"))):
                local.add(m.group(1))
        res[c.pkg] = {n for n in c.externs if n in local and n != c.id} | {n for n in c.aliases if n in local}
    return res


def build(sb, outdir, crates, strip_attrs, notest=()):
    """Write the rewritten crate copies into `outdir` (inside the sandbox). Returns (root_mods, masked_texts, ext_globs, features_cfgs, root_features)."""
    SBX_CURRENT_DIR[:] = [outdir]
    notes = {"excluded": excluded_heads(crates), "notest": set(notest)}
    fam = {c.pkg: c for c in crates}
    all_exports = {}
    for c in crates:
        c.macros = find_exports(c)
        for n in c.macros:
            all_exports.setdefault(n, []).append(c.pkg)
    dup = {n: p for n, p in all_exports.items() if len(p) > 1}
    notes["dup_macros"] = dup
    masked_all = []
    ext_globs = {}
    root_feats = set()
    for c in crates:
        base = os.path.join(outdir, c.mod)
        for rel, data in c.files.items():
            dest = os.path.join(base, rel)
            if rel in c.code:
                text = data.decode("utf-8", "replace")
                new, feats = rewrite_code(c, text, rel, fam, strip_attrs, notes)
                for n in dup:
                    if c.pkg in dup[n] and c.pkg != dup[n][0]:
                        new = re.sub(r"#\[\s*macro_export\s*(?:\([^\]]*\))?\s*\](\s*(?:#\[[^\]]*\]\s*)*macro_rules!\s*" + re.escape(n) + r"\b)", r"\1", new)
                root_feats.update(feats)
                if rel == c.lib_rel:
                    exp = [n for n in c.macros if not (n in dup and c.pkg != dup[n][0])]
                    new += "\n" + "".join(f"#[allow(unused_imports)]\npub use crate::{n};\n" for n in exp)
                sb.write(dest, new)
                mt = mask(new)
                masked_all.append(mt)
                for m in re.finditer(r"\buse\s+crate::__ext::([^;]*?)\*\s*;", mt):
                    prefix = [s.strip() for s in m.group(1).split("::") if s.strip()]
                    if prefix and "{" not in m.group(1):
                        ext_globs.setdefault(os.path.join(c.mod, rel), []).append(prefix)
            else:
                sb.write(dest, data)
    return masked_all, ext_globs, root_feats, notes


def cfg_args(crates):
    args = []
    for c in crates:
        for f in sorted(c.features):
            args += ["--cfg", f'feature="{c.id}:{f}"']
    return args


def root_text(crates, stub_text, root_feats):
    lines = ["#![allow(warnings)]"]
    if root_feats:
        lines.append("#![feature(" + ", ".join(sorted(root_feats)) + ")]")
    for c in crates:
        lines.append(f'#[path = "{c.mod}/{c.lib_rel}"]\npub mod {c.mod};')
    return "\n".join(lines) + "\n" + stub_text
