"""🧭 One-shot: moves every mutation vector bundle of the shared harness users (norm, block) from the nested role tree to flat role files.

`<kind>/<case>/📸️snapshot/⬅️before/🔣️.json` becomes `<kind>/<case>/🔣️.before.json`; `➡️after`, `🦠️mutation`, `🎯️outcome` and `🔺️diff` likewise.
The empty-diff marker `🔺️diff/🚫️.absent` is dropped: no `🔣️.diff.json` is the absence. Nothing is left behind.

`python -I flatten_fixtures.py [--apply]` (dry-run without the flag; prints counts and refuses unknown files).
"""
import os, sys, pathlib

REPO = pathlib.Path(__file__).resolve().parents[9]
PLUGINS = REPO / "✏️s/🔌️plugins"
ROLES = {
    "📸️snapshot/⬅️before/🔣️.json": "🔣️.before.json",
    "📸️snapshot/➡️after/🔣️.json": "🔣️.after.json",
    "🦠️mutation/🔣️.json": "🔣️.mutation.json",
    "🎯️outcome/🔣️.json": "🔣️.outcome.json",
    "🔺️diff/🔣️.json": "🔣️.diff.json",
}
DROPPED = {"🔺️diff/🚫️.absent"}
EXTRA = [PLUGINS / "🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🫧️transient/🧪️tests/🧫️fixtures"]


def roots():
    found = []
    for plugin in ("📕️norm", "🧱️block"):
        for directory, dirs, _ in os.walk(PLUGINS / plugin):
            dirs[:] = [d for d in dirs if d not in {"target", "node_modules", "__pycache__"}]
            if directory.endswith("🧪️tests/🧫️fixtures/🧬️mutations"):
                found.append(pathlib.Path(directory))
                dirs[:] = []
    return sorted(found + EXTRA)


def files_below(case):
    return sorted(str(p.relative_to(case)) for p in case.rglob("*") if p.is_file())


def main(argv):
    apply = "--apply" in argv
    moved = dropped = bundles = 0
    problems = []
    for root in roots():
        for kind in sorted(p for p in root.iterdir() if p.is_dir()):
            for case in sorted(p for p in kind.iterdir() if p.is_dir()):
                relative = files_below(case)
                unknown = [r for r in relative if r not in ROLES and r not in DROPPED and r not in ROLES.values()]
                if unknown:
                    problems.append((str(case), unknown))
                    continue
                bundles += 1
                for old, new in ROLES.items():
                    source, target = case / old, case / new
                    if source.is_file():
                        if target.exists():
                            problems.append((str(case), ["collision " + new]))
                            continue
                        moved += 1
                        if apply:
                            os.rename(source, target)
                for marker in DROPPED:
                    if (case / marker).is_file():
                        dropped += 1
                        if apply:
                            (case / marker).unlink()
                if apply:
                    for directory in sorted((p for p in case.rglob("*") if p.is_dir()), key=lambda p: -len(p.parts)):
                        if not any(directory.iterdir()):
                            directory.rmdir()
    print("bundles", bundles, "files moved", moved, "markers dropped", dropped, "problems", len(problems), "applied" if apply else "dry-run")
    for case, unknown in problems[:20]:
        print("  PROBLEM", case, unknown)
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
