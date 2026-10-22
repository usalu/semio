#!/usr/bin/env python3
"""Copy of the tool-iface set at the ticket root (`r6-tool-iface-tools`, same depth as the original: it derives the repository root from its location) with the catalogue anchors re-based on rcache-server's `remote-cache` operation, and the cargo-relay funnel anchor on the text runtime-seam leaves (runtime-seam now precedes tool-iface)."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/tool-iface-tools", "r6-tool-iface-tools", beside=True)
E = T + "/build/edits.py"
import re
_root = repo_root()
_cat = open(os.path.join(_root, "\U0001F9F0\ufe0fframework/\U0001F6CD\ufe0fproducts/\U0001F991\ufe0frepo/\U0001F528\ufe0fmodules/\U0001F4DA\ufe0flibrary/\U0001F3D7\ufe0fbuild/\u2699\ufe0foperations/\U0001F523\ufe0fcatalog.json"), encoding="utf8").read()
_m = re.search(r'"name": "--json",\n      "value": "none",\n      "operations": \[([^\]]*)\],', _cat)
_cur = _m.group(1)
_text = open(E, encoding="utf8").read()
_old = re.search(r'JSON_OLD = \'\'\'      "name": "--json",\n      "value": "none",\n      "operations": \[[^\]]*\],\'\'\'', _text)
_new = re.search(r'JSON_NEW = \'\'\'      "name": "--json",\n      "value": "none",\n      "operations": \[[^\]]*\],\'\'\'', _text)
assert _old and _new
_head = '      "name": "--json",\n      "value": "none",\n      "operations": ['
_text = _text.replace(_old.group(0), "JSON_OLD = \'\'\'" + _head + _cur + "],\'\'\'").replace(_new.group(0), "JSON_NEW = \'\'\'" + _head + _cur + ', "tools"],\'\'\'')
open(E, "w", encoding="utf8").write(_text)
text = open(E, encoding="utf8").read()
pattern = r"SCRIPT_OLD = \'\'\'  const\{withBuildCapacityV1\}=await import\((\"[^\"]+\")\);\n(  const\{withImplementationPinV1\}=await import\(\"[^\"]+\"\);)?"
found = re.findall(pattern, text)
if len(found) != 1:
    raise SystemExit("maker: tool-iface SCRIPT_OLD not found once")
text = re.sub(pattern, lambda m: "SCRIPT_OLD = \'\'\'  const{withBuildCapacityV1}=await profilePhaseV1(\"door.load\",()=>import(" + m.group(1) + "));" + ("\n" + m.group(2) if m.group(2) else ""), text)
open(E, "w", encoding="utf8").write(text)
patch(T + "/build/edits_funnel.py", 'spawnSync(process.versions.bun ? process.execPath : "bun", [CARGO_RELAY_SCRIPT', 'spawnSync(runtimeExecutableV1(), [CARGO_RELAY_SCRIPT')
patch(T + "/build/migrate.py", "        if header_end is not None and not re.fullmatch(", "        if text[:found.start()].count(\"`\") % 2 == 1:\n            break\n        if header_end is not None and not re.fullmatch(")
patch(T + "/build/migrate.py", '        replacement = ("import { " + ", ".join(keep)', '        keep += [name for name in remaining if name not in keep and re.search(r"typeof\\s+" + re.escape(re.sub(r"^type\\s+", "", name).split(" as ")[0].strip()) + r"\\b", text)]\n        replacement = ("import { " + ", ".join(keep)')
import subprocess
subprocess.run([sys.executable, os.path.join(HERE, "fixups", "tool-iface-langs-pin.py"), T, repo_root()], check=True)
subprocess.run([sys.executable, os.path.join(HERE, "fixups", "tool-iface-after-seam.py"), T], check=True)
print("tool-iface copy:", T)
