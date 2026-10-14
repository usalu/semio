#!/usr/bin/env python3
"""🔎️ Audits every artifact crate's `crate::<head>::` paths against what the crate can resolve (lane P1): `self` heads
(the crate's own alias), `unknown` heads (neither an alias, nor a module/item/use name declared at the crate root, nor an
inline module), per crate with file and count; covers `crate::a::`, `crate::Item` and `crate::{a, b::c}` heads. usage: 🔎️crate-path-audit.py [--files]
"""
import importlib.util, os, re, sys, collections
spec = importlib.util.spec_from_file_location("artifact_lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "✂️artifact_lib.py"))
L = importlib.util.module_from_spec(spec); spec.loader.exec_module(L)
PKG_ALIAS = {v: k for k, v in L.ALIASES.items()}
def root_names(text):
    s = L.strip(text); names = set()
    for m in re.finditer(r"(?m)^(?:pub(?:\([^)]*\))?\s+)?(?:mod|fn|struct|enum|const|static|type|trait|macro_rules!)\s+(\w+)", s): names.add(m.group(1))
    for m in re.finditer(r"(?m)^extern crate \w+ as (\w+);", s): names.add(m.group(1))
    for m in re.finditer(r"(?m)^(?:pub(?:\([^)]*\))?\s+)?use\s+([^;]+);", s):
        body = m.group(1)
        if body.lstrip().startswith("crate"): continue
        for leaf in re.findall(r"(?:\bas\s+(\w+))|(\w+)\s*(?=[,}]|$)", body): names.add(leaf[0] or leaf[1])
    return names
report = collections.defaultdict(lambda: collections.Counter())
for r, dirs, fs in os.walk(L.ART):
    dirs[:] = [x for x in dirs if x not in ("target", "node_modules")]
    if "Cargo.toml" not in fs or not r.endswith(os.path.join("📦️packages", "🦀️rust")): continue
    t = L.rd(os.path.join(r, "Cargo.toml")); m = re.search(r'(?m)^name = "([^"]+)"', t)
    if not m: continue
    pkg = m.group(1); own = PKG_ALIAS.get(pkg); owner = os.path.normpath(os.path.join(r, "..", ".."))
    roots = [os.path.join(owner, "🦀️.rs")] + [os.path.normpath(os.path.join(r, p)) for p in re.findall(r'(?m)^path = "([^"]+)"', t.split("[lib]", 1)[-1]) if p != "../../🦀️.rs"]
    for root in roots:
        if not os.path.exists(root): continue
        names = root_names(L.rd(root))
        for f in L.closure_files(root):
            text = L.strip(L.rd(f))
            heads = re.findall(r"(?<![\w:])crate\s*::\s*(\w+)\b", text)
            for group in re.findall(r"(?<![\w:])crate\s*::\s*\{([^{}]*(?:\{[^{}]*\}[^{}]*)*)\}", text):
                depth, cur, parts = 0, "", []
                for ch in group:
                    depth += ch == "{"; depth -= ch == "}"
                    if ch == "," and depth == 0: parts.append(cur); cur = ""
                    else: cur += ch
                parts.append(cur)
                heads += [re.match(r"\s*(\w+)", x).group(1) for x in parts if re.match(r"\s*(\w+)", x)]
            for head in heads:
                kind = "self" if head == own and root.endswith(os.path.join(owner, "🦀️.rs")) else None if head in names else "unknown"
                if kind: report[pkg][(kind, head, os.path.relpath(f, L.ANY) if "--files" in sys.argv else "")] += 1
for pkg in sorted(report):
    print(pkg)
    for (kind, head, f), n in sorted(report[pkg].items()): print(f"   {kind:7s} {head:28s} {n:4d} {f}")
