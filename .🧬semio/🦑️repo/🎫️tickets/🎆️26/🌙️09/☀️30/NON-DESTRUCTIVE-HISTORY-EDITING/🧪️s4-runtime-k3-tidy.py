#!/usr/bin/env python3
"""🧹️ K3 tidy after `🧪️s4-runtime-k3-transient-root.py --apply`: drops `fn retained_bytes` helpers no item calls any more, empty
`//#region`/`//#endregion` pairs, `semio_framework_plugin` imports whose names the file no longer uses, and wraps the two
macro invocations in one `//#region 🔖️Owner` when the file uses regions. Usage: s4-runtime-k3-tidy.py <file>..."""
import re
import sys

for path in sys.argv[1:]:
    s = open(path, encoding="utf-8").read()
    if "retained_bytes(" in s and s.count("retained_bytes(") == 1:
        s = re.sub(r"\n(///[^\n]*\n)*fn retained_bytes\([^\n]*\{\n(    [^\n]*\n)*\}\n", "\n", s)
    changed = True
    while changed:
        before = s
        s = re.sub(r"//#region ([^\n]+)\n\s*//#endregion \1\n", "", s)
        changed = s != before
    def trim(match):
        names = [name.strip() for name in match.group(1).split(",")]
        rest = s.replace(match.group(0), "")
        used = [name for name in names if re.search(rf"\b{re.escape(name)}\b", rest)]
        return "" if not used else f"use semio_framework_plugin::{{{', '.join(used)}}};\n"
    match = re.search(r"use semio_framework_plugin::\{([^}]+)\};\n", s)
    if match:
        s = s.replace(match.group(0), trim(match))
    if "//#region" in s and "//#region 🔖️Owner" not in s:
        start = s.index("semio_framework_plugin::transient_root!")
        end_match = list(re.finditer(r"\n\}\n", s))
        tail = s.find("\n#[cfg(test)]", start)
        stop = tail if tail != -1 else len(s.rstrip("\n"))
        block = s[start:stop].rstrip("\n")
        s = s[:start] + "//#region 🔖️Owner\n" + block + "\n//#endregion 🔖️Owner\n" + s[stop:].lstrip("\n").join(["", ""]) if False else s[:start] + "//#region 🔖️Owner\n" + block + "\n//#endregion 🔖️Owner\n" + ("\n" + s[stop:].lstrip("\n") if tail != -1 else "")
    while "\n\n\n" in s:
        s = s.replace("\n\n\n", "\n\n")
    open(path, "w", encoding="utf-8").write(s.rstrip("\n") + "\n")
    print("tidied", path)
