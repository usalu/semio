"""Group manifests (round-5 addendum, decision D1): the VS Code extension workspace row in all six root manifests and the VS Code API tooling in the root manifest."""
from blib import *

g = "manifests"
ROW = M + "/💻️client/🧩️vscode/📦️packages/🟦️typescript"
ED(g, "package.json", f'    "{ROW}",\n', "", (1,), "manifests-root")
ED(g, "package.json", '    "@types/vscode": "^1.104.0",\n', "", (1,), "manifests-root")
ED(g, "package.json", '    "@vscode/test-cli": "^0.0.10",\n    "@vscode/vsce": "^3.5.0",\n', "", (1,), "manifests-root")
for root in ("♻️mit-bestand", "✏️s", "🌎️hub", "🎓️teaching", "🏢️semio-tech"):
    ED(g, root + "/package.json", f'    "../{ROW}",\n', "", (1,), "manifests-" + root)
