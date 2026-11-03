"""🧭 One-shot: shortens the 13 module folders whose files breach the 240 byte path budget; renames the folder and every `#[path]`, `include_str!` and Cargo `path` that names it.

`python -I shorten_module_dirs.py [--apply]` (dry-run without the flag). Folder names are unique per rename, so the text replacement is exact.
"""
import os, pathlib, sys

REPO = pathlib.Path(__file__).resolve().parents[9]
ART = REPO / "✏️s/🔌️plugins/📕️norm/🗿️artifacts"
SUB = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences"
RENAMES = [
    ("🫨️en1998", "🌍️full-seismic-check", "🧱️structural-system-checks", "🧱️system-checks"),
    ("🏛️en1992", "⚖️checks", "📋️member-evaluation", "📋️members"),
    ("🔩️en1993", "⚖️checks/🧱️member-checks/🧾️vid-checks", "🧪️material-fatigue-fire-vids", "🧪️fatigue-fire-vids"),
    ("🔩️en1993", "⚖️checks", "🔗️special-element-checks", "🔗️special-checks"),
    ("⚡️din18599", "⚖️checks/🩺️document-evaluation", "💡️lighting-hot-water-and-cooling-checks", "💡️lighting-water-cooling-checks"),
    ("🏭️vdi3805", "📚️document-checks", "🗃️catalog-integrity", "🗃️catalog"),
    ("🏭️vdi3805", "📚️document-checks/🗃️catalog", "🗃️product-and-index-checks", "🗃️product-index"),
    ("🏭️vdi3805", "📚️document-checks/🗃️catalog", "🗓️correction-date-and-strict-mode-checks", "🗓️correction-strict-checks"),
]
TEXT = (".rs", ".toml", ".json", ".py", ".ts", ".md", ".feature")


def references(artifact, old):
    for directory, dirs, files in os.walk(ART / artifact):
        dirs[:] = [d for d in dirs if d not in {"target", "node_modules", "__pycache__"}]
        for name in files:
            if name.endswith(TEXT):
                path = pathlib.Path(directory) / name
                try:
                    if old in path.read_text(encoding="utf-8"):
                        yield path
                except UnicodeDecodeError:
                    continue


def main(argv):
    apply = "--apply" in argv
    for artifact, parent, old, new in RENAMES:
        base = ART / artifact / SUB / parent
        source, target = base / old, base / new
        if not source.is_dir() and target.is_dir():
            print("done already", artifact, new)
            continue
        assert source.is_dir() and not target.exists(), (artifact, old)
        touched = list(references(artifact, old))
        print(artifact, old, "->", new, "| files naming it:", len(touched))
        if apply:
            os.rename(source, target)
            for path in touched:
                path.write_text(path.read_text(encoding="utf-8").replace(old, new), encoding="utf-8", newline="")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
