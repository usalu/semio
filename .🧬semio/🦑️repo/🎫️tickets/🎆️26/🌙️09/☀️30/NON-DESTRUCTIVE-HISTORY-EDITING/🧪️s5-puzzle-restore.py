"""↩️ Restores one landed S5-PUZZLE wave to its pre-wave text (fleet rule 67: every train line names its restore).

`python3 <this file> <wave> [--check]` with `<wave>` one of:
- `precondition-drifted` — inverse of `🧪️s5-puzzle-precondition-drifted.py` (every hunk back, the Python oracle from `HEAD`);
- `tool-mismatch` — inverse of the four sites `🧪️s5-gates-tool-mismatch.py --root ✏️s/🔌️plugins/🧩️puzzle --apply` rewrote.

All or nothing: every replacement text must be present exactly as the wave wrote it (the stated number of times) before any
file is written. `--check` writes nothing.
"""

import importlib.util
import pathlib
import subprocess
import sys

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[6]
EDITORS = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/{}/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
FAULT = 'Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("{}"), "{}")'
TOOL_MISMATCH = [
    (EDITORS.format("🧊️3d"), FAULT.format("app.command.tool-mismatch", "puzzle3d-command-tool-mismatch"), 'Fault::from("puzzle3d-command-tool-mismatch")', 1),
    (EDITORS.format("🖐️5d"), FAULT.format("app.command.tool-mismatch", "puzzle5d-command-tool-mismatch"), 'Fault::from("puzzle5d-command-tool-mismatch")', 1),
    (EDITORS.format("◻️2d"), FAULT.format("app.command.tool-mismatch", "puzzle2d-command-tool-mismatch"), 'Fault::from("puzzle2d-command-tool-mismatch")', 1),
    (EDITORS.format("◻️2d"), FAULT.format("app.command.unsupported", "puzzle2d-command-tool-unmapped"), 'Fault::from("puzzle2d-command-tool-unmapped")', 1),
]


def wave(name):
    spec = importlib.util.spec_from_file_location("wave", HERE / name)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def inverse(hunks):
    for path, old, new in reversed(hunks):
        yield path, new, old, 1


def main():
    name, check = sys.argv[1], "--check" in sys.argv[2:]
    whole = {}
    if name == "precondition-drifted":
        module = wave("🧪️s5-puzzle-precondition-drifted.py")
        hunks = list(inverse(module.HUNKS))
        kept = HERE / "🗑️generated" / "s5-puzzle" / "pre-22-13" / "history-edit-runtime-oracle.py"
        committed = kept.read_text(encoding="utf-8") if kept.exists() else subprocess.run(["git", "show", f"HEAD:{module.ORACLE}"], cwd=ROOT, capture_output=True, check=True).stdout.decode("utf-8")
        if committed.count(module.ORACLE_BASE) != 1:
            sys.exit(f"{module.ORACLE}: neither the kept copy nor HEAD holds the pre-wave oracle")
        whole[module.ORACLE] = committed
    elif name == "tool-mismatch":
        hunks = TOOL_MISMATCH
    else:
        sys.exit(__doc__)
    contents, pending = dict(whole), len(whole)
    for path, written, original, expected in hunks:
        text = contents.get(path)
        if text is None:
            text = (ROOT / path).read_text(encoding="utf-8")
        count = text.count(written)
        if count != expected:
            sys.exit(f"{path}: the wave's text resolves {count} times, expected {expected}:\n{written[:200]}")
        contents[path] = text.replace(written, original)
        pending += 1
    if check:
        print(f"{pending} changes would be restored in {len(contents)} files")
        return
    for path, text in contents.items():
        if (ROOT / path).read_text(encoding="utf-8") != text:
            (ROOT / path).write_text(text, encoding="utf-8")
    print(f"restored {pending} changes in {len(contents)} files")


if __name__ == "__main__":
    main()
