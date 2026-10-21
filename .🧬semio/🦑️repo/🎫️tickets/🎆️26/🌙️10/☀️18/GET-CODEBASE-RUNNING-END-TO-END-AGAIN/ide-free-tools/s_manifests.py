"""Group manifests (decision D1): the VS Code extension workspace row in all six root manifests and the VS Code API tooling in the root manifest.
Every edit is anchored on the preceding line, so the replacement is never empty: both this engine and lock-sync's apply.py recognise an applied edit (idempotent in any order)."""
import re
from blib import *

g = "manifests"
ROW = M + "/💻️client/🧩️vscode/📦️packages/🟦️typescript"


def drop(path, line, tag):
    text = read(path)
    m = re.search(r"([^\n]*\n)" + re.escape(line), text)
    if not m or text.count(m.group(0)) != 1: raise SystemExit("anchor not unique: " + path + " " + line[:60])
    ED(g, path, m.group(0), m.group(1), (1,), tag)


drop("package.json", f'    "{ROW}",\n', "manifests-root")
drop("package.json", '    "@types/vscode": "^1.104.0",\n', "manifests-root")
drop("package.json", '    "@vscode/test-cli": "^0.0.10",\n', "manifests-root")
drop("package.json", '    "@vscode/vsce": "^3.5.0",\n', "manifests-root")
for root in ("♻️mit-bestand", "✏️s", "🌎️hub", "🎓️teaching", "🏢️semio-tech"):
    drop(root + "/package.json", f'    "../{ROW}",\n', "manifests-" + root)
