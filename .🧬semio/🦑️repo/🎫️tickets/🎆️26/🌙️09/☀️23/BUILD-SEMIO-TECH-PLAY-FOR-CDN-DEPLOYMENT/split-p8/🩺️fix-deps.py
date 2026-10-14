#!/usr/bin/env python3
"""🩺️ Adds the missing workspace dependencies a cargo short-format log reports (`could not find `X` in the list of
imported crates`, `unresolved module or unlinked crate `X``, `can't find crate for `X``) to the package that failed,
as `[dependencies]` rows (`[dev-dependencies]` when only test units failed), if the workspace root declares `X`.

usage: 🩺️fix-deps.py <cargo.log> <workspace Cargo.toml> [apply]
"""
import collections, os, re, sys
log, ws_path = sys.argv[1], sys.argv[2]
apply = len(sys.argv) > 3 and sys.argv[3] == "apply"
ws_dir = os.path.dirname(os.path.abspath(ws_path))
ws = open(ws_path, encoding="utf-8").read()
declared = set(re.findall(r"(?m)^\[workspace\.dependencies\.([A-Za-z0-9_-]+)\]", ws)) | set(re.findall(r"(?m)^([A-Za-z0-9_-]+)\s*=\s*\{", ws.split("[workspace.dependencies]", 1)[-1])) if "[workspace.dependencies]" in ws else set(re.findall(r"(?m)^\[workspace\.dependencies\.([A-Za-z0-9_-]+)\]", ws))
by_ident = {name.replace("-", "_"): name for name in declared}
aliases = {"protocol": "semio-framework-os-kernel", "store": "semio-framework-os-kernel", "dsl": "semio-framework-os-kernel", "pack": "semio-framework-pack"}
need = collections.defaultdict(lambda: collections.defaultdict(set))
text = open(log, encoding="utf-8", errors="replace").read()
pattern = re.compile(r"^(?P<file>[^\s:]+?/📦️packages/🦀️rust)/[^:]*:\d+:\d+: error\[E04(?:33|32|63)\]: .*?(?:could not find `(?P<a>\w+)` in the list of imported crates|unresolved module or unlinked crate `(?P<b>\w+)`|can't find crate for `(?P<c>\w+)`)", re.M)
units = {}
for m in pattern.finditer(text):
    ident = m.group("a") or m.group("b") or m.group("c")
    name = by_ident.get(ident)
    if not name:
        continue
    need[m.group("file")][name].add(m.start())
fails = re.findall(r"could not compile `([^`]+)` \((lib|lib test|test \"[^\"]+\")\)", text)
for pkg_dir, names in need.items():
    if "🗄️stdio/" not in pkg_dir:
        print("SKIP (out of scope)", pkg_dir, sorted(names)); continue
    manifest = os.path.join(ws_dir, pkg_dir, "Cargo.toml") if not os.path.isabs(pkg_dir) else os.path.join(pkg_dir, "Cargo.toml")
    if not os.path.exists(manifest):
        print("NO MANIFEST", manifest); continue
    mtext = open(manifest, encoding="utf-8").read()
    pkg = re.search(r'(?m)^name\s*=\s*"([^"]+)"', mtext).group(1)
    lib_failed = (pkg, "lib") in fails
    section = "dependencies" if lib_failed or not any(p == pkg for p, _ in fails) else "dev-dependencies"
    for name in sorted(names):
        if re.search(r"(?m)^" + re.escape(name) + r"\s*=", mtext.split("[dev-dependencies]")[0] if section == "dependencies" else mtext):
            continue
        row = f"{name} = {{ workspace = true }}\n"
        m = re.search(r"(?ms)^\[" + re.escape(section) + r"\]\s*\n", mtext)
        mtext = mtext[:m.end()] + row + mtext[m.end():] if m else mtext.rstrip("\n") + f"\n\n[{section}]\n{row}"
        print(f"{pkg}: +{section} {name}")
    if apply:
        open(manifest, "w", encoding="utf-8").write(mtext)
