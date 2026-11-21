"""🔑 Repairs the access of existing types exposed to the separate native WGPU kernel-pool."""
from hashlib import sha256
import json
from pathlib import Path
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
OUT = TICKET / "🗑️generated/native-wgpu-save/kernel-visibility"
DIRECTORY = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu"
changes = {
    "📡️kernel-protocol/📥️lifted/🏊️kernel-pool/🦀️.rs": [
        ("struct KernelRequestQueueState {", "pub struct KernelRequestQueueState {"),
    ],
    "📡️kernel-protocol/📥️lifted/📬️kernel-replay-client/🔁️spawn-recovery/🦀️.rs": [
        ("pub(super) struct RawSpawnJobOwner {", "pub struct RawSpawnJobOwner {"),
    ],
    "📡️kernel-protocol/📥️lifted/📬️kernel-replay-client/📄️typed-result/🦀️.rs": [
        ("struct ResponseState {", "pub struct ResponseState {"),
    ],
    "🧊️renderer/📦️kernel-runtime/🧮️kernel-core/🦀️.rs": [
        ("struct ExtensionRecord {", "pub struct ExtensionRecord {"),
    ],
    "🧊️renderer/📦️kernel-runtime/🗃️kernel-retained-state/🦀️.rs": [
        ("enum RetainedPatchApply {", "pub enum RetainedPatchApply {"),
        ("struct RetainedDocumentBuild {", "pub struct RetainedDocumentBuild {"),
        ("struct MountedReplayRecoveryToken {", "pub struct MountedReplayRecoveryToken {"),
    ],
}
OUT.mkdir(parents=True, exist_ok=True)
entries = []
for i, (name, pairs) in enumerate(changes.items()):
    path = DIRECTORY / name
    before = path.read_bytes()
    after = before.decode("utf8")
    for first, second in pairs:
        assert after.count("\n" + first) == 1, (path, first)
        after = after.replace("\n" + first, "\n" + second)
    staged = OUT / f"{i:02}.rs"
    staged.write_text(after, encoding="utf8")
    entries.append({"file": path.relative_to(ROOT).as_posix(), "before": sha256(before).hexdigest(),
        "after": sha256(staged.read_bytes()).hexdigest(), "source": staged.name})

receipt = OUT / "receipt.json"
receipt.write_text(json.dumps(entries, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
if "--apply" in sys.argv:
    assert all(sha256((ROOT / row["file"]).read_bytes()).hexdigest() == row["before"] for row in entries)
    for row in entries:
        path, staged = ROOT / row["file"], OUT / row["source"]
        assert sha256(path.read_bytes()).hexdigest() == row["before"]
        ready = OUT / (row["source"] + ".new")
        ready.write_bytes(staged.read_bytes())
        ready.replace(path)
        assert sha256(path.read_bytes()).hexdigest() == row["after"]
print(f"[DEBUG] {'Installed' if '--apply' in sys.argv else 'Staged'} {len(entries)} WGPU kernel visibility paths")
