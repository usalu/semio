"""🏷️ Stages the missing bilingual PDF history action labels."""

import hashlib
import json
from pathlib import Path
import re
import sys

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "vt-labels"
MUTATIONS = ["📦️insert-object", "🧹️remove-object", "🔧️set-object-value"]
SNAPSHOT = Path("📸️snapshot/🔣️.json")
DIFF_LABELS = {
    "removed": ("Removed roles", "Entfernte Rollen"),
    "modified": ("Modified roles", "Geänderte Rollen"),
    "added": ("Added roles", "Hinzugefügte Rollen"),
}
REFERENCE_LABELS = {
    "artifactId": ("Artifact ID", "Artefakt-ID"),
    "dialect": ("Artifact dialect", "Artefaktdialekt"),
    "artifactKind": ("Artifact kind", "Artefaktart"),
    "standard": ("Standard", "Standard"),
    "subset": ("Subset", "Teilmenge"),
}


def label_block(source, start, labels, stop=None):
    offset = source.index(start)
    end = source.index(stop, offset) if stop else len(source)
    target = source[offset:end]
    for key, (en, de) in labels.items():
        marker = re.compile(rf'^( +)"{re.escape(key)}": \{{$', re.MULTILINE)
        found = list(marker.finditer(target))
        assert len(found) == 1, (start, key)
        indentation = found[0][1] + "  "
        annotation = {"x-semio-ui": {"label": {"en": en, "de": de}}}
        lines = json.dumps(annotation, ensure_ascii=False, indent=2).splitlines()[1:-1]
        addition = "\n" + "\n".join(indentation + line[2:] for line in lines) + ","
        target = marker.sub(lambda match: match[0] + addition, target, count=1)
    return source[:offset] + target + source[end:]


def stage(base, action):
    files = {}
    for name in MUTATIONS:
        relative = Path(f"🧬️mutations/{name}/🧬️schema/🔣️.json")
        before = (base / relative).read_bytes()
        after = label_block(before.decode("utf8"), '    "admittedStreamRoles": {', DIFF_LABELS)
        files[relative] = (before, after.encode("utf8"))
    before = (base / SNAPSHOT).read_bytes()
    after = before.decode("utf8")
    after = label_block(after, '    "PdfGraphIdentity": {', {"owner": ("Owning object", "Zugehöriges Objekt")}, '\n    },')
    after = label_block(after, '    "PdfAdmittedStreamRole": {', {"dependencies": ("Dependencies", "Abhängigkeiten")}, '\n    },')
    after = label_block(after, '    "ArtifactRef": {', REFERENCE_LABELS, '\n    },')
    files[SNAPSHOT] = (before, after.encode("utf8"))
    receipt = {}
    for index, (relative, (before, after)) in enumerate(files.items()):
        json.loads(after)
        path = OUT / f"{index}.json"
        receipt[relative.as_posix()] = {
            "before": hashlib.sha256(before).hexdigest(),
            "after": hashlib.sha256(after).hexdigest(),
            "overlay": path.name,
        }
        if action == "--prepare":
            OUT.mkdir(parents=True, exist_ok=True)
            path.write_bytes(after)
        else:
            assert path.read_bytes() == after
            assert (base / relative).read_bytes() == before
            assert len(str((base / relative).resolve()).encode("utf-16-le")) // 2 <= 256
    witness = OUT / "receipt.json"
    if action == "--prepare":
        witness.write_text(json.dumps(receipt, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    else:
        assert json.loads(witness.read_bytes()) == receipt
        if action == "--apply-reviewed":
            for relative, (before, after) in files.items():
                assert (base / relative).read_bytes() == before
                (base / relative).write_bytes(after)
    print(f"[DEBUG] {action} {len(receipt)} PDF label schemas")


if __name__ == "__main__":
    action = sys.argv[2]
    base = Path(sys.argv[1])
    if action == "--verify-live":
        receipt = json.loads((OUT / "receipt.json").read_bytes())
        for relative, row in receipt.items():
            authored = (base / relative).read_bytes()
            assert hashlib.sha256(authored).hexdigest() == row["after"], relative
            assert authored == (OUT / row["overlay"]).read_bytes()
            print(f"[DEBUG] Live {relative} {row['after']}")
    else:
        assert action in ("--prepare", "--verify", "--apply-reviewed")
        stage(base, action)
