#!/usr/bin/env python3
"""🧾️ W2-R norm: validates every `x-semio-ui` in the norm leaf payload schemas against the manifest `$defs/InputUi`
meta-schema (Python `jsonschema`, third party) and re-validates every committed fixture mutation payload in scope
against its leaf schema as it is now and as it was before the rollout (`--before <json>` snapshot), so an added hard
bound can never reject a committed payload that the unannotated schema accepted.

    .venv/bin/python 🧪️w2-r-norm-check.py [--before <leaf-schemas-before.json>] [--snapshot <out.json>]
"""
import glob
import json
import os
import re
import sys

from jsonschema import validators

REPO = "/Users/ueli/Documents/semio"
SCOPE = "✏️s/🔌️plugins/📕️norm"
MANIFEST = "🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json"


def kind_of(name):
    for at, character in enumerate(name):
        if character.isascii() and character.isalpha():
            return name[at:]
    return name


def leaf_schemas():
    return sorted(glob.glob(f"{SCOPE}/**/🧬️mutations/*/🧬️schema/🔣️.json", recursive=True))


def annotations(node, pointer=""):
    if isinstance(node, dict):
        for key, value in node.items():
            if key == "x-semio-ui":
                yield pointer, value
            else:
                yield from annotations(value, f"{pointer}/{key}")
    elif isinstance(node, list):
        for at, value in enumerate(node):
            yield from annotations(value, f"{pointer}/{at}")


def leaf_key(path):
    artifact = path.split("/🏅️standards")[0] if "/🏅️standards" in path else path.split("/🧬️schema/🧬️mutations")[0]
    return artifact, kind_of(path.split("🧬️mutations/")[1].split("/")[0])


def payload_candidates(payload):
    if isinstance(payload, dict) and len(payload) == 1:
        (tag, inner), = payload.items()
        if re.fullmatch(r"[A-Z][A-Za-z0-9]*", tag) and isinstance(inner, dict):
            payload = inner
    if isinstance(payload, dict) and "mutation" in payload:
        payload = {key: value for key, value in payload.items() if key != "mutation"}
    return payload


def verdict(schema, payload):
    validator = validators.validator_for(schema)(schema)
    return not any(True for _ in validator.iter_errors(payload))


def main():
    os.chdir(REPO)
    args = sys.argv[1:]
    manifest = json.load(open(MANIFEST, encoding="utf-8"))
    ui_schema = {"$schema": manifest["$schema"], "$ref": "#/$defs/InputUi", "$defs": manifest["$defs"]}
    ui_validator = validators.validator_for(ui_schema)(ui_schema)
    schemas = {path: json.load(open(path, encoding="utf-8")) for path in leaf_schemas()}
    if "--snapshot" in args:
        target = args[args.index("--snapshot") + 1]
        json.dump({path: open(path, encoding="utf-8").read() for path in schemas}, open(target, "w", encoding="utf-8"), ensure_ascii=False)
    count = 0
    errors = 0
    for path, schema in schemas.items():
        for pointer, value in annotations(schema):
            count += 1
            for error in ui_validator.iter_errors(value):
                errors += 1
                print(f"UI {path}#{pointer}: {error.message}")
    print(f"x-semio-ui annotations={count} meta-schema errors={errors}")
    before = {}
    if "--before" in args:
        before = {path: json.loads(text) for path, text in json.load(open(args[args.index("--before") + 1], encoding="utf-8")).items()}
    by_leaf = {leaf_key(path): path for path in schemas}
    fixtures = sorted(glob.glob(f"{SCOPE}/**/🦠️mutation/🔣️.json", recursive=True))
    tally = {"fixtures": len(fixtures), "unmapped": 0, "valid-now": 0, "valid-before": 0, "regressed": 0}
    for fixture in fixtures:
        artifact = fixture.split("/🏅️standards")[0]
        leaf = kind_of(fixture.split("🧬️mutations/")[1].split("/")[0])
        path = by_leaf.get((artifact, leaf))
        if path is None:
            tally["unmapped"] += 1
            continue
        payload = payload_candidates(json.load(open(fixture, encoding="utf-8")))
        now = verdict(schemas[path], payload)
        was = verdict(before[path], payload) if path in before else now
        tally["valid-now"] += now
        tally["valid-before"] += was
        if was and not now:
            tally["regressed"] += 1
            print(f"REGRESSED {fixture}")
    print("fixtures " + " ".join(f"{key}={value}" for key, value in tally.items()))
    return 1 if errors or tally["regressed"] else 0


if __name__ == "__main__":
    sys.exit(main())
