"""🛬️ S5-RUNTIME landing waves (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, design §22.1/§22.2/§22.6/§22.7).

Every wave is a list of files, each with exact-anchor replacements (an anchor must occur exactly once in the live file) or a
whole-file producer. `check` applies a wave to the live files in memory and writes the results under
`🗑️generated/s5-runtime/wave-<name>/after/` (nothing in the tree changes) — `check <wave> <earlier waves…>` applies it on
top of the checked results of waves that have not landed yet; `land` re-applies it to the live files, keeps each replaced
file under `…/before/` and writes the tree (run it only while holding the `landing` lock, rule 51); `revert` puts the kept
files back whole (only while nobody landed onto them since); `unland` takes a wave's exact-anchor replacements out of the
LIVE files again — each replacement must still occur exactly once — and leaves everything peers landed since in place (a
whole-file producer has no inverse: its file is named and left). Run from the repo root:
`python3 <this file> <check|land|revert|unland> <wave> [<earlier waves…>]`.
"""

import importlib.util
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
TICKET = pathlib.Path(__file__).resolve().parent
SCRATCH = TICKET / "🗑️generated/s5-runtime"

FW = "🧰️framework/🔨️modules"
OSM = "🧰️framework/🛍️products/💻️os/🔨️modules"
KERNEL = f"{FW}/🎠️kernel"
FWT = f"{FW}/⏪️time-travel"
PLUGIN = f"{OSM}/🔌️plugin"
TT = f"{PLUGIN}/⏪️time-travel/🦀️.rs"
PLG = f"{PLUGIN}/🦀️.rs"
MANIFEST = f"{FW}/🛂️manifest/🦀️.rs"


def replace(text, pairs, name):
    for old, new in pairs:
        count = text.count(old)
        if count != 1:
            raise SystemExit(f"{name}: anchor occurs {count} times, expected 1:\n{old[:240]}")
        text = text.replace(old, new)
    return text


def module(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), TICKET / f"🧪️s5-runtime-wave-{name}.py")
    loaded = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(loaded)
    return loaded


def current(path, earlier):
    """The text a wave applies to: the newest checked result of an earlier wave that has not landed, else the live file."""
    for wave in reversed(earlier):
        staged = SCRATCH / f"wave-{wave}/after" / path
        if staged.exists():
            return staged.read_text(encoding="utf-8")
    return (ROOT / path).read_text(encoding="utf-8") if (ROOT / path).exists() else None


def produce(wave, earlier=()):
    """The wave's files after its change: `{relative path: text}` computed from the live tree (and `earlier` checked waves)."""
    after = {}
    for path, change in module(wave).files(ROOT).items():
        text = current(path, earlier)
        after[path] = change(text) if callable(change) else replace(text, change, path)
    return after


def main():
    verb, wave = sys.argv[1], sys.argv[2]
    base = SCRATCH / f"wave-{wave}"
    if verb == "revert":
        kept = json.loads((base / "before/index.json").read_text(encoding="utf-8"))
        for path, existed in kept.items():
            if existed:
                (ROOT / path).write_text((base / "before" / path).read_text(encoding="utf-8"), encoding="utf-8")
            else:
                (ROOT / path).unlink(missing_ok=True)
        print(f"reverted wave {wave}: {len(kept)} files")
        return
    if verb == "unland":
        for path, change in module(wave).files(ROOT).items():
            if callable(change):
                print(f"  {path}: a whole-file producer, left as it is")
                continue
            live = ROOT / path
            live.write_text(replace(live.read_text(encoding="utf-8"), [(new, old) for old, new in reversed(change)], path), encoding="utf-8")
            print(f"  {path}: {len(change)} replacements taken out")
        print(f"unlanded wave {wave}")
        return
    after = produce(wave, sys.argv[3:] if verb == "check" else ())
    target = base / ("after" if verb == "check" else "before")
    if verb == "land":
        index = {}
        for path in after:
            live = ROOT / path
            index[path] = live.exists()
            if live.exists():
                (target / path).parent.mkdir(parents=True, exist_ok=True)
                (target / path).write_text(live.read_text(encoding="utf-8"), encoding="utf-8")
        target.mkdir(parents=True, exist_ok=True)
        (target / "index.json").write_text(json.dumps(index, ensure_ascii=False, indent=2), encoding="utf-8")
        for path, text in after.items():
            (ROOT / path).parent.mkdir(parents=True, exist_ok=True)
            (ROOT / path).write_text(text, encoding="utf-8")
        print(f"landed wave {wave}: {len(after)} files")
        return
    for path, text in after.items():
        (target / path).parent.mkdir(parents=True, exist_ok=True)
        (target / path).write_text(text, encoding="utf-8")
    print(f"checked wave {wave}: {len(after)} files → {target.relative_to(ROOT)}")
    for path in after:
        print(f"  {path}")


if __name__ == "__main__":
    main()
