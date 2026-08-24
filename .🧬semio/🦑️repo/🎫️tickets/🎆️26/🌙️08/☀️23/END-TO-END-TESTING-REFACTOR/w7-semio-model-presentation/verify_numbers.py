#!/usr/bin/env python3
"""Second independent check: every NON-DEFAULT number in the committed before-fixture must be
recoverable from the real .dsl.semio asset — literally (model) or as an IEEE-754 bit pattern
(presentation). Zeros and ones are excluded: the DSL prints them as bare 0/1 tokens that also
appear as option/enum markers, so they carry no discriminating evidence."""
import json, re, struct, sys

ART = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio"
TESTS = f"{ART}/🧪️tests"


def numbers(node, out):
    if isinstance(node, dict):
        for value in node.values():
            numbers(value, out)
    elif isinstance(node, list):
        for value in node:
            numbers(value, out)
    elif isinstance(node, bool):
        pass
    elif isinstance(node, (int, float)):
        out.append(float(node))
    return out


def check(label, asset_path, fixture_path):
    text = open(asset_path, encoding="utf-8").read()
    body = "\n".join(text.splitlines()[1:])
    tokens = set(re.findall(r"-?\d+(?:\.\d+)?", body))
    literal = set()
    for token in tokens:
        literal.add(float(token))
        if token.isdigit() and len(token) > 6:
            try:
                literal.add(struct.unpack(">d", struct.pack(">Q", int(token)))[0])
            except (OverflowError, struct.error):
                pass
        if len(token) % 2 == 0 and re.fullmatch(r"[0-9a-f]+", token):
            literal.update(float(b) for b in bytes.fromhex(token))
    wanted = {n for n in numbers(json.load(open(fixture_path, encoding="utf-8")), []) if n not in (0.0, 1.0)}
    missing = sorted(n for n in wanted if n not in literal)
    print(f"{label}: {len(wanted)} distinct non-trivial fixture numbers, {len(missing)} not recoverable from the real asset")
    for n in missing:
        print(f"  MISSING {n!r}")
    return not missing


ok = check("model", f"{ART}/🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🏢️building/🖼️assets/🗣️example.dsl.semio", f"{TESTS}/mutate-semio-model/🧫️fixtures/no-mutation/⬅️before.json")
ok &= check("presentation", f"{ART}/🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📽️deck/🖼️assets/🗣️example.dsl.semio", f"{TESTS}/mutate-semio-presentation/🧫️fixtures/no-mutation/⬅️before.json")
sys.exit(0 if ok else 1)
