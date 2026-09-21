"""🌉️ S9 — installs `norm_command_from_action!` in every norm editor that lacks the shells' bridge.

Fourteen of the fifteen norm editors never overrode `ArtifactEditor::command_from_action`, so every
Actions-rail row of those apps was inert in the React and wgpu shells (ticket 26/09/18, S9 §3).
The anchor is the editor's own `fn command_id`, asserted unique in each file before any edit; the
macro invocation is appended directly after it. Idempotent: a file that already invokes the macro or
hand-writes the bridge is left alone and reported.
"""

import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
ANCHOR = re.compile(r"    fn command_id\(command: &(\w+)\) -> &'static str \{\n        command\.command_id\(\)\n    \}\n")


def main() -> int:
    changed, skipped = [], []
    for artifact in sorted(ROOT.iterdir()):
        editor = artifact / "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
        if not editor.is_file():
            continue
        text = editor.read_text(encoding="utf-8")
        if "norm_command_from_action!" in text or "fn command_from_action" in text:
            skipped.append(f"{artifact.name}: already bridged")
            continue
        matches = ANCHOR.findall(text)
        if len(matches) != 1:
            skipped.append(f"{artifact.name}: anchor matched {len(matches)} times — not edited")
            continue
        command = matches[0]
        variant = artifact.name.lstrip("⚖️⚡️🌍️🌬️🏋️🏛️🏭️📇️🔩️🧩️🧱️🪨️🪵️🪶️🫨️ ")
        decoder = f"crate::standards::v1::subsets::any::schema::snapshot::decode_{variant}_snapshot_json"
        insert = f"\n    semio_s_artifact_norm_contract::norm_command_from_action!({command}, {decoder});\n"
        text = ANCHOR.sub(lambda match: match.group(0) + insert, text, count=1)
        editor.write_text(text, encoding="utf-8")
        changed.append(f"{artifact.name}: {command} / {decoder.rsplit('::', 1)[1]}")
    for row in changed:
        print(f"[s9] bridged {row}")
    for row in skipped:
        print(f"[s9] skipped {row}")
    print(f"[s9] {len(changed)} bridged, {len(skipped)} skipped")
    return 0


if __name__ == "__main__":
    sys.exit(main())
