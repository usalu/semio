"""🪪️ Fix-forward of the press wave (`🧪️s5-tools-press.py`): `drive_press` compares the stamped gesture with the held one, and a
generic caller only knows `T::Gesture: PartialEq` for the projection, not for `GestureState<M>` — so the three generic entry
points state `M: PartialEq` themselves. Counted anchors, idempotent. Usage (cwd: repo root, under `landing`):
python3 🧪️s5-tools-press-bound.py [--apply]"""

import pathlib
import sys

REPO = pathlib.Path(__file__).resolve().parents[7]
TM = "🧰️framework/🔨️modules/🛠️tool-machine/🦀️.rs"
PLUGIN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/🦀️.rs"
EDITS = {
    TM: [
        ("pub fn drive_press<T: GestureTool<Gesture = GestureState<M>, Mutation = M>, M>(", "pub fn drive_press<T: GestureTool<Gesture = GestureState<M>, Mutation = M>, M: PartialEq>("),
        (
            "        base_revision: &str,\n    ) -> Result<Option<(TransactionRef, Vec<M>)>, ToolRefusal> {\n        let drive = drive_press::<T, M>(",
            "        base_revision: &str,\n    ) -> Result<Option<(TransactionRef, Vec<M>)>, ToolRefusal>\n    where\n        M: PartialEq,\n    {\n        let drive = drive_press::<T, M>(",
        ),
    ],
    PLUGIN: [
        (
            "authoring_seed: &str) -> Result<Option<(protocol::TransactionRef, Vec<M>)>, Fault> {\n        let mut state = self.state.lock()",
            "authoring_seed: &str) -> Result<Option<(protocol::TransactionRef, Vec<M>)>, Fault>\n    where\n        M: PartialEq,\n    {\n        let mut state = self.state.lock()",
        ),
    ],
}


def main():
    staged, pending = [], 0
    for relative, edits in EDITS.items():
        path = REPO / relative
        text = path.read_text(encoding="utf-8")
        for old, new in edits:
            if text.count(old) == 1:
                text, pending = text.replace(old, new), pending + 1
            elif text.count(new) != 1:
                raise SystemExit(f"{relative}: anchor moved: {old[:70]!r}")
        staged.append((path, text))
    if "--apply" in sys.argv:
        for path, text in staged:
            path.write_text(text, encoding="utf-8")
    print(f"{'applied' if '--apply' in sys.argv else 'would apply'} {pending} bound(s)")


if __name__ == "__main__":
    main()
