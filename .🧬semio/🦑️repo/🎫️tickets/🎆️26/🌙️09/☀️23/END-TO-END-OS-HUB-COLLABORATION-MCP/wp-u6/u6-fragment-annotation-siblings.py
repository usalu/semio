#!/usr/bin/env python3
"""🧬️ U6 window-3 set C4 (framework schema, next train): large documents stay editable — a `$ref` annotated with `x-…` keys is
followed by fragment validation instead of forcing a whole-container projection.

Measured 03:24 (`.🧬semio/🌐hub/s14-u6-logs/a3-check-test.txt`, wav `large_sample_edit_publishes_cancels_undoes_redoes_and_preserves_metadata`,
2 097 152 samples): one sample edit at `/data/value/1500000` is refused `schema.fragment.context-required: … validation array exceeds
its bounded node budget at /data`. Root cause (`🧰️framework/🔨️modules/🧬️schema/✅️validator/🦀️.rs`): every generated artifact schema
annotates its properties (`"data": {"$ref": "#/$defs/WavData", "x-semio-state": "artifact"}`); `schema_has_observing_siblings` counts
the `x-semio-state` annotation as an instance-observing keyword, so `normalize_fragment_cursor` stops at the `$ref` and
`fragment_ancestor_needs_context` demands the post-edit projection of the WHOLE container — the 2M-sample array — although the
target is one sample under a discriminated union whose tag (`/data/kind`) is all the validator needs. The schema-node validator of the
same file already treats `x-…` keywords as extensions (`extension if extension.starts_with("x-") => {}`); the fragment path now agrees
(`schema_has_observing_siblings`, `fragment_union_is_standalone`). Every artifact property is shaped like this, so every fragment edit
under an annotated reference was projecting its container.

Laws: the fragment-validation fixture (Rust law + AJV oracle, `🧫️fixtures/🩹️fragment-validation-vectors.json`) gains an annotated
reference to a discriminated sample union; its boundary/overflow sample edits must request exactly `/samples/kind` (the tag), never
the container — witnessed failing on the current validator (it requests `/samples`). End to end: the wav 2M-sample test (samples
generated in the test, nothing committed) must emit its sample edit within 2 s.

Usage: u6-fragment-annotation-siblings.py [--dry-run | --write | --revert] [--root <repo root>]"""
import hashlib
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
REVERT = "--revert" in sys.argv
BACKUP = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-backup/fragment-annotation-siblings")
SCHEMA = "🧰️framework/🔨️modules/🧬️schema/"
VALIDATOR = SCHEMA + "✅️validator/🦀️.rs"
FIXTURE = SCHEMA + "🧫️fixtures/🩹️fragment-validation-vectors.json"
LAW = SCHEMA + "🧪️tests/🔬️component-unit/🦀️.rs"
WAV_TEST = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"

SETS = {
    VALIDATOR: [
        (
            'fn schema_has_observing_siblings(schema: &Object) -> bool {\n    schema.iter().any(|(key, _)| {\n        !matches!(\n            key,\n            "$ref" | "$id" | "$schema" | "$comment" | "title" | "description" | "default" | "examples" | "readOnly" | "writeOnly" | "deprecated" | "definitions" | "$defs"\n        )\n    })\n}\n',
            'fn schema_has_observing_siblings(schema: &Object) -> bool {\n    schema.iter().any(|(key, _)| !key.starts_with("x-") && !matches!(key, "$ref" | "$id" | "$schema" | "$comment" | "title" | "description" | "default" | "examples" | "readOnly" | "writeOnly" | "deprecated" | "definitions" | "$defs"))\n}\n',
            1,
        ),
        (
            'fn fragment_union_is_standalone(schema: &Object) -> bool {\n    schema.iter().all(|(key, _)| {\n        matches!(\n            key,\n            "oneOf"\n                | "anyOf"\n                | "$id"\n                | "$schema"\n                | "$comment"\n                | "title"\n                | "description"\n                | "default"\n                | "examples"\n                | "readOnly"\n                | "writeOnly"\n                | "deprecated"\n                | "definitions"\n                | "$defs"\n        )\n    })\n}\n',
            'fn fragment_union_is_standalone(schema: &Object) -> bool {\n    schema.iter().all(|(key, _)| key.starts_with("x-") || matches!(key, "oneOf" | "anyOf" | "$id" | "$schema" | "$comment" | "title" | "description" | "default" | "examples" | "readOnly" | "writeOnly" | "deprecated" | "definitions" | "$defs"))\n}\n',
            1,
        ),
    ],
    FIXTURE: [
        ('    "required": ["metadata", "variant", "bytes", "payload"],\n', '    "required": ["metadata", "variant", "bytes", "samples", "payload"],\n', 1),
        (
            '      "payload": { "type": "string" }\n    }\n  },\n',
            '      "samples": { "$ref": "#/definitions/Samples", "x-semio-state": "artifact" },\n'
            '      "payload": { "type": "string" }\n    },\n'
            '    "definitions": {\n'
            '      "Samples": {\n'
            '        "oneOf": [\n'
            '          {\n'
            '            "type": "object",\n'
            '            "required": ["kind", "value"],\n'
            '            "additionalProperties": false,\n'
            '            "properties": {\n'
            '              "kind": { "const": "pcm8" },\n'
            '              "value": { "type": "array", "items": { "type": "integer", "minimum": 0, "maximum": 255 } }\n'
            '            }\n'
            '          },\n'
            '          {\n'
            '            "type": "object",\n'
            '            "required": ["kind", "value"],\n'
            '            "additionalProperties": false,\n'
            '            "properties": {\n'
            '              "kind": { "const": "pcm16" },\n'
            '              "value": { "type": "array", "items": { "type": "integer", "minimum": -32768, "maximum": 32767 } }\n'
            '            }\n'
            '          }\n'
            '        ]\n'
            '      }\n'
            '    }\n  },\n',
            1,
        ),
        ('    "bytes": [0]\n  },\n', '    "bytes": [0],\n    "samples": { "kind": "pcm8", "value": [0, 0] }\n  },\n', 1),
        (
            '    { "id": "discriminated-union-tag-incomplete", "operation": "set", "path": ["variant", "kind"], "candidate": "named", "accepted": false },\n',
            '    { "id": "discriminated-union-tag-incomplete", "operation": "set", "path": ["variant", "kind"], "candidate": "named", "accepted": false },\n'
            '    { "id": "annotated-reference-union-boundary", "operation": "set", "path": ["samples", "value", 1], "candidate": 255, "accepted": true },\n'
            '    { "id": "annotated-reference-union-overflow", "operation": "set", "path": ["samples", "value", 1], "candidate": 256, "accepted": false },\n',
            1,
        ),
    ],
    LAW: [
        (
            """        if matches!(case["id"].as_str(), Some("discriminated-union-boundary" | "discriminated-union-overflow")) {
            assert_eq!(requested, ["/variant/kind"], "{} projected more than its discriminating tag", case["id"]);
        }
""",
            """        if matches!(case["id"].as_str(), Some("discriminated-union-boundary" | "discriminated-union-overflow")) {
            assert_eq!(requested, ["/variant/kind"], "{} projected more than its discriminating tag", case["id"]);
        }
        if matches!(case["id"].as_str(), Some("annotated-reference-union-boundary" | "annotated-reference-union-overflow")) {
            assert_eq!(requested, ["/samples/kind"], "{}: an `x-` annotation beside `$ref` must not force the container projection", case["id"]);
        }
""",
            1,
        ),
    ],
    WAV_TEST: [
        (
            """    let emit = <WavEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("sample edit emits");
    let [mutation] = emit.artifact_mutations.as_slice() else { panic!("sample edit must emit one mutation") };
    assert!(matches!(mutation, WavMutation::PatchData(_)));
""",
            """    let started = std::time::Instant::now();
    let emit = <WavEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("sample edit emits");
    assert!(started.elapsed() < std::time::Duration::from_secs(2), "one sample edit of a {sample_count}-sample document took {:?}", started.elapsed());
    let [mutation] = emit.artifact_mutations.as_slice() else { panic!("sample edit must emit one mutation") };
    assert!(matches!(mutation, WavMutation::PatchData(_)));
""",
            1,
        ),
    ],
}


def key(rel: str) -> str:
    return hashlib.sha256(rel.encode()).hexdigest()[:16]


def main() -> int:
    if REVERT:
        for rel in SETS:
            backup = BACKUP / key(rel)
            if backup.exists():
                (ROOT / rel).write_bytes(backup.read_bytes())
                backup.unlink()
                print(f"REVERTED {rel}")
        return 0
    problems, planned = 0, []
    for rel, hunks in SETS.items():
        path = ROOT / rel
        text = path.read_text()
        for old, new, count in hunks:
            found = text.count(old)
            if found != count:
                print(f"PROBLEM {rel}: {'already applied' if new in text else f'anchor count {found} != {count}'}: {old[:70]!r}")
                problems += 1
                continue
            text = text.replace(old, new)
        planned.append((rel, path, text))
        print(f"{'WRITE' if WRITE else 'DRY'} {rel.rsplit('/', 3)[-3:]}: {len(hunks)} hunks")
    if WRITE and problems == 0:
        BACKUP.mkdir(parents=True, exist_ok=True)
        for rel, path, text in planned:
            (BACKUP / key(rel)).write_bytes(path.read_bytes())
            path.write_text(text)
    print(f"{'write' if WRITE and problems == 0 else 'dry-run'}: {len(SETS)} files, {problems} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
