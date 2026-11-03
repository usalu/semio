"""🧭 One-shot: renames fixture kind folders that still carry `sourceMutationDirectoryName` to the `mutationDirectoryName` their oracle manifest declares.

`python -I align_dirs_with_oracle.py [--apply]`. A folder is renamed only when the declared name is absent and the source name is present.
"""
import json, os, pathlib, sys

REPO = pathlib.Path(__file__).resolve().parents[9]
ART = REPO / "✏️s/🔌️plugins/📕️norm/🗿️artifacts"
SUBSET = "🏅️standards/🔖️1/🪆️subsets/✳️any"


def main(argv):
    apply = "--apply" in argv
    renamed = 0
    for artifact in sorted(ART.iterdir()):
        manifest = artifact / SUBSET / "🔮️oracles/🔣️.json"
        root = artifact / SUBSET / "🧪️tests/🧫️fixtures/🧬️mutations"
        if not manifest.is_file() or not root.is_dir():
            continue
        for catalog in json.loads(manifest.read_text(encoding="utf-8")).get("mutationCatalogs", []):
            for vector in catalog.get("vectors", []):
                source, declared = vector.get("sourceMutationDirectoryName"), vector["mutationDirectoryName"]
                if source and source != declared and (root / source).is_dir() and not (root / declared).exists():
                    print(artifact.name, source, "->", declared)
                    renamed += 1
                    if apply:
                        os.rename(root / source, root / declared)
    print("renamed", renamed, "applied" if apply else "dry-run")


if __name__ == "__main__":
    main(sys.argv[1:])
