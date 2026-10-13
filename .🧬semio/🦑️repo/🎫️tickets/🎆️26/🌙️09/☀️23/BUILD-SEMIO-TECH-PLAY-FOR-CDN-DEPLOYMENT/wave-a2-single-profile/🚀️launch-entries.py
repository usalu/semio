#!/usr/bin/env python3
"""🚀️ Rewrites the wave A2 launch entries in `.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json` in place: the retired
`component-dev`/`materialize-dev`/`support-dev`/`support-release` commands become the single-profile targets and the
`workspace:components` describe and profile-independence commands are registered beside the release driver."""
import re
import sys

ENTRY = '''    {{
      "name": "{name}",
      "type": "node-terminal",
      "request": "launch",
      "command": "{command}",
      "cwd": "${{workspaceFolder}}",
      "env": {{
        "NX_TUI": "false"
      }},
      "presentation": {{
        "group": "{group}",
        "order": {order}
      }}
    }},
'''
ADDED = [
    ("📦️describe🦑️workspace🧩️components", "bun nx run workspace:components describe --all", "4_build", "11.013"),
    ("⚖️gate🦑️workspace🧩️profile-independence", "bun nx run workspace:components profile-independence --sample 8", "4_gate", "11.013"),
]


def replace_once(text, old, new, path):
    if text.count(old) != 1:
        raise SystemExit(f"{path}: expected exactly one {old!r}, found {text.count(old)}")
    return text.replace(old, new)


def rewrite(path):
    text = open(path, encoding="utf-8").read()
    text = replace_once(text, "bun nx run @semio-tech/demonstrator-plugin:component-dev ", "bun nx run @semio-tech/demonstrator-plugin:component ", path)
    text = replace_once(text, '"name": "📦️materialize-dev🖍️draw🦀️"', '"name": "📦️iterate🖍️draw🦀️"', path)
    text = replace_once(text, '"command": "bun nx run @semio-tech/draw-plugin:materialize-dev --skip-nx-cache"', '"command": "bun nx run workspace:components iterate draw"', path)
    text = replace_once(text, '"command": "bun nx run @semio-tech/framework-plugin-host-test-component:component-dev"', '"command": "bun nx run @semio-tech/framework-plugin-host-test-component:component"', path)
    if "support-dev🔌️plugin🟦️" in text:
        dev = re.compile(r'    \{\n      "name": "🛠️support-dev🔌️plugin🟦️",.*?\n    \},\n', re.S)
        text, count = dev.subn("", text)
        if count != 1:
            raise SystemExit(f"{path}: support-dev entry not unique")
        text = replace_once(text, '"name": "📦️support-release🔌️plugin🟦️"', '"name": "📦️support🔌️plugin🟦️"', path)
        text = replace_once(text, '"command": "bun nx run @semio-tech/framework-plugin-web:support-release"', '"command": "bun nx run @semio-tech/framework-plugin-web:support"', path)
    anchor = re.compile(r'(    \{\n      "name": "📦️build🦑️workspace🚀️release🆕️fresh",.*?\n    \},\n)', re.S)
    if any(name in text for name, *_ in ADDED):
        raise SystemExit(f"{path}: components entries already present")
    text, count = anchor.subn(lambda match: match.group(1) + "".join(ENTRY.format(name=n, command=c, group=g, order=o) for n, c, g, o in ADDED), text)
    if count != 1:
        raise SystemExit(f"{path}: release-fresh anchor not unique")
    open(path, "w", encoding="utf-8").write(text)


for target in sys.argv[1:]:
    rewrite(target)
