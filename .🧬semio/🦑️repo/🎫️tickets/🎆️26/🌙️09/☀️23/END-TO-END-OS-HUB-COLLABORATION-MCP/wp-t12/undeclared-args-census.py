#!/usr/bin/env python3
"""🧾️ Ticket-local measurement (heuristic, not a law): for every staged descriptor (restage4), every action that declares
NO args, look up its `"<id>" =>` arm in the plugin's Rust `command_from_action` and flag it when that arm reads an argument
(`args`, `text(`, `lookup(`, `str_field(`, `fold(`, `number(`, `flag(`, `value_number(`). The law is per plugin
(note: `every_note_verb_that_reads_arguments_declares_them`); this lists the candidates for the same law elsewhere."""
import json
import re
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
STAGED = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules"
READS = re.compile(r"\bargs\b|text\(|lookup\(|str_field\(|fold\(|number\(|flag\(|value_number\(|value_boolean\(|arg\(|string_arg\(|text_arg\(")
SKIP = {"undo", "redo", "commitCheckpoint", "createAlternative", "switchAlternative", "checkoutCheckpoint", "copy", "cut", "paste", "clearSelection", "selectAll", "setSelectionMode", "setInteractionGranularity", "interactionSelect", "interactionHover", "revertToCommand", "setHistoryCommandFilter", "noteShellCommand"}
sources = {}
for plugin_dir in sorted((ROOT / "✏️s/🔌️plugins").iterdir()):
    if plugin_dir.is_dir():
        sources[plugin_dir.name] = [path for path in plugin_dir.rglob("🦀️.rs") if "🧪️tests" not in path.parts and "dist" not in path.parts]
total = 0
for descriptor_path in sorted(STAGED.glob("*/🔣️.json")):
    descriptor = json.loads(descriptor_path.read_text(encoding="utf-8"))
    manifest = descriptor["manifest"]
    plugin = manifest["pluginId"]
    plugin_dir = next((name for name in sources if name.endswith(plugin.split("-")[0])), None)
    if plugin_dir is None:
        continue
    texts = [(path, path.read_text(encoding="utf-8")) for path in sources[plugin_dir]]
    flagged = []
    for app in manifest["apps"]:
        actions = list(app.get("actions", [])) + [action for window in app.get("windowKinds", []) for action in window.get("actions", [])]
        for action in actions:
            if action.get("args") or action["id"] in SKIP or action["id"] in {entry[1] for entry in flagged}:
                continue
            arm = re.compile(r'^\s*"' + re.escape(action["id"]) + r'"\s*(?:\|[^=]*)?=>(.*)$', re.M)
            for path, text in texts:
                match = arm.search(text)
                if match and READS.search(text[match.start(): match.start() + 600].split("\n        \"")[0]):
                    flagged.append((app["id"], action["id"], path.relative_to(ROOT)))
                    break
    if flagged:
        total += len(flagged)
        print(f"{plugin}: {len(flagged)}")
        for app_id, action_id, path in flagged:
            print(f"  {action_id:28} {app_id}")
print(f"TOTAL candidates {total}")
