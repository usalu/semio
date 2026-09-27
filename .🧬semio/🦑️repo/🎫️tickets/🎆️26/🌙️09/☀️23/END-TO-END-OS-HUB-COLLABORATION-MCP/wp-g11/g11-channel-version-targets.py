"""🔖️ G11 pending set (lands after W3's PUBLISH DONE: `📋️project.json` edits are frozen during the rebuild, session-13 rule 4):
the nx targets + launch rows of the channel version authority (`🧑‍💻dev/🔖️channel-version/🟦️.ts`, verb `channel-version`,
already registered in the dev `📜️script.ts`). Targets `channel-version-generate` / `channel-version-check` on
`@semio-tech/framework-os-dev`; launch rows `📦️check📡️channel-version` + `🛠️dev📡️channel-version🏭️generate` (seed + launch.json,
identical) right after `📦️check🧹️fixture-sweep` (4_build, 206.171 / 206.172).
usage: python3 g11-channel-version-targets.py [--apply]"""
import json
import sys

ROOT = "/Users/ueli/Documents/semio"
PROJECT = f"{ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json"
LAUNCHES = [f"{ROOT}/.vscode/🧩️launch.seed.jsonc", f"{ROOT}/.vscode/launch.json"]
CWD = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
TARGETS = {
    "channel-version-check": {"executor": "nx:run-commands", "outputs": [], "options": {"cwd": CWD, "command": "bun ./📜️script.ts channel-version check"}},
    "channel-version-generate": {"executor": "nx:run-commands", "outputs": [], "options": {"cwd": CWD, "command": "bun ./📜️script.ts channel-version generate", "forwardAllArgs": True}},
}
ROWS = """    {
      "name": "📦️check📡️channel-version",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:channel-version-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.171
      }
    },
    {
      "name": "🛠️dev📡️channel-version🏭️generate",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:channel-version-generate",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.172
      }
    },
"""
ANCHOR = '      "name": "📦️check🧹️fixture-sweep",'


def main():
    apply = "--apply" in sys.argv
    project_text = open(PROJECT, encoding="utf-8").read()
    project = json.loads(project_text)
    clash = [name for name in TARGETS if name in project["targets"]]
    launch_texts = {path: open(path, encoding="utf-8").read() for path in LAUNCHES}
    missing = [path for path, text in launch_texts.items() if text.count(ANCHOR) != 1 or "channel-version-check" in text]
    print(f"targets clash {clash}, launch anchors missing/applied {missing}")
    if clash or missing:
        sys.exit(1)
    if not apply:
        print("dry run clean")
        return
    anchor = '    "host-handle-lint": {'
    if project_text.count(anchor) != 1:
        sys.exit("project anchor missing")
    block = "".join(f'    "{name}": {json.dumps(target, indent=2, ensure_ascii=False).replace(chr(10), chr(10) + "    ")},\n' for name, target in TARGETS.items())
    patched = project_text.replace(anchor, block + anchor)
    json.loads(patched)
    open(PROJECT, "w", encoding="utf-8").write(patched)
    for path, text in launch_texts.items():
        start = text.index(ANCHOR)
        end = text.index("    },\n", start) + len("    },\n")
        open(path, "w", encoding="utf-8").write(text[:end] + ROWS + text[end:])
    print("applied")


main()
