"""📦️ Prepares the verified image and Flate-history changes for one shared-worktree handoff."""

from hashlib import sha256
from pathlib import Path
import runpy
import sys

TICKET = Path(__file__).resolve().parent
OUTPUT = TICKET / "🗑️generated"
IMAGE = Path("🔨️modules/🖼️images/🦀️.rs")
CHILD = IMAGE.parent / "🎞️controlled/🦀️.rs"
FIXTURE = Path("🧪️tests/🧫️fixtures/🎞️controlled-images/🔣️.json")
BASELINE = "6fdf8d2cd56b63e8192d7e8e5da30ebfcafab1961daa01c1ed4e84c8075388fc"


def prepare(base, apply):
    integration = runpy.run_path(str(TICKET / "🎞️integration-stage.py"))
    history = runpy.run_path(str(TICKET / "🎞️history-stage.py"))
    tests = runpy.run_path(str(TICKET / "🎞️tests-stage.py"))
    integration["stage"](base, OUTPUT)
    history["stage"](base)
    tests["stage"](base)
    assert sha256((base / IMAGE).read_bytes()).hexdigest() == BASELINE, "image owner changed after source preparation"
    assert not (base / CHILD).exists(), CHILD
    assert not (base / FIXTURE).exists(), FIXTURE
    planned = {
        IMAGE: OUTPUT / "🎞️images-parent.rs",
        CHILD: OUTPUT / "🎞️image-cursor.rs",
        FIXTURE: TICKET / "🎞️fixture.json",
        integration["DOCUMENT"]: OUTPUT / "🎞️document-integration.rs",
        integration["PROGRAM"]: OUTPUT / "🎞️program-integration.rs",
        integration["IMPORT"]: OUTPUT / "🎞️import-integration.rs",
        tests["LIFTER"]: OUTPUT / "🎞️lifter-integration.rs",
        tests["TESTKIT"]: OUTPUT / "🎞️testkit-integration.rs",
        tests["EDITOR"]: OUTPUT / "🎞️editor-integration.rs",
    }
    planned.update({source: OUTPUT / f"🎞️{name}-history.rs" for name, source, _, _ in history["PATCHES"]})
    if apply:
        proposed = {base / relative: source.read_bytes() for relative, source in planned.items()}
        originals = {path: path.read_bytes() if path.exists() else None for path in proposed}
        for path in proposed:
            existing = path.read_bytes() if path.exists() else None
            assert existing == originals[path], f"shared owner changed: {path}"
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(proposed[path])
    for relative in planned:
        print(f"[DEBUG] {'updated' if apply else 'staged'} {relative}")


if __name__ == "__main__":
    prepare(Path(sys.argv[1]), len(sys.argv) > 2 and sys.argv[2] == "--apply")
