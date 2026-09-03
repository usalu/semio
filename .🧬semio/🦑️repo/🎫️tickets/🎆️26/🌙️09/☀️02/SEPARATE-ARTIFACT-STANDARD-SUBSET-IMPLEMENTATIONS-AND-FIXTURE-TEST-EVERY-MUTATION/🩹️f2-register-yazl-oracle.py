#!/usr/bin/env python3
"""🩹️ F2 — registers yazl 2.5.1 (MIT, same maintainer/family as the already-registered yauzl reader)
as a second, WRITER-side third-party-library oracle in both zip/2.0 subsets this shard touches, the
same reader/writer split precedent shard E2 already used for wav (hound+riff). Needed because the
fixture generator (🔨️f2-gen-zip-fixtures.cjs) writes real archives with yazl and it would be dishonest
to credit that generation to yauzl (a reader with no writer API at all). Idempotent: safe to re-run.
"""
import json
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
SUBSETS = [
    (ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base/🧪️oracle/🔣️.json", "yazl-zip-2-0-base-mutate-writer", "zip-2-0-mutate"),
    (ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️iso21320/🧪️oracle/🔣️.json", "yazl-zip-2-0-iso21320-mutate-writer", "zip-2-0-mutate"),
]


def main() -> None:
    for path, oracle_id, capability in SUBSETS:
        data = json.loads(path.read_text())
        ids = {o["id"] for o in data["oracles"]}
        if oracle_id in ids:
            print(f"skip (already present): {oracle_id}")
            continue
        data["oracles"].append({
            "id": oracle_id,
            "kind": "third-party-library",
            "ecosystem": "javascript",
            "package": "yazl",
            "version": "2.5.1",
            "engine": {"family": "yazl", "implementation": "yazl streaming ZIP writer", "version": "2.5.1"},
            "capabilities": [capability],
            "license": "MIT",
            "testOnly": True,
            "productionReachable": False,
            "networkDuringExecution": False,
            "platforms": ["darwin-arm64", "darwin-x64", "linux-x64", "linux-arm64", "win32-x64"],
            "homepage": "https://github.com/thejoshwolfe/yazl",
            "rationale": (
                "📖️ A WRITER, the sibling this subset's already-registered `yauzl` reader needs: yauzl "
                "has no encoder at all, so it cannot be the generator of a `third-party-generated` "
                "fixture on its own. yazl is the same author's (thejoshwolfe) companion ZIP writer, "
                "same reader/writer split precedent as this ticket's own wav artifact (hound+riff). "
                "Every fixture this oracle generates is written by yazl's own ZipFile encoder and then "
                "independently RE-READ and verified (entries, compression method, comment) with the "
                "already-registered yauzl reader before being committed -- see "
                "🔨️f2-gen-zip-fixtures.cjs in this ticket's folder for the live verification output."
            ),
            "comparisonProfiles": ["exact-bytes-v1"],
        })
        path.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n")
        print(f"registered {oracle_id} in {path}")


if __name__ == "__main__":
    main()
