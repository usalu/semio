#!/usr/bin/env python3
"""🩹️ Parity check of the Python test host's ``patched_snapshot`` against the vectors of the Rust host's unit law
``a_patch_snapshot_row_is_its_one_pointer_operation_on_the_reference_reading`` (same inputs, same expected outputs, same
two refusals), and every patch followed by `snapshot_patch_inverse` restoring the snapshot exactly (member order
included). Exits 1 on the first divergence.

@see ../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py
@see ../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/⚖️law/🧪️tests/🔬️unit/🦀️.rs
"""
import importlib.util
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
SPEC = importlib.util.spec_from_file_location("semio_repo_test_host", ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py")
HOST = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(HOST)

SNAPSHOT = {"name": "a", "list": [1, 2, 3], "meta": {"x": 1, "y/z": 2}, "text": "Grüße"}
CASES = [
    ({"operation": "set", "path": "/name", "value": "b"}, '{"name":"b","list":[1,2,3],"meta":{"x":1,"y/z":2},"text":"Grüße"}'),
    ({"operation": "insert", "path": "/meta/w", "value": 0, "index": 0}, '{"name":"a","list":[1,2,3],"meta":{"w":0,"x":1,"y/z":2},"text":"Grüße"}'),
    ({"operation": "remove", "path": "/meta/y~1z"}, '{"name":"a","list":[1,2,3],"meta":{"x":1},"text":"Grüße"}'),
    ({"operation": "move", "from": "/list/0", "path": "/list/-"}, '{"name":"a","list":[2,3,1],"meta":{"x":1,"y/z":2},"text":"Grüße"}'),
    ({"operation": "rename", "path": "/meta/x", "key": "v"}, '{"name":"a","list":[1,2,3],"meta":{"v":1,"y/z":2},"text":"Grüße"}'),
    ({"operation": "splice", "path": "/list", "offset": 1, "remove": 1, "value": [7, 8]}, '{"name":"a","list":[1,7,8,3],"meta":{"x":1,"y/z":2},"text":"Grüße"}'),
    ({"operation": "splice", "path": "/text", "offset": 4, "remove": 2, "value": "ss"}, '{"name":"a","list":[1,2,3],"meta":{"x":1,"y/z":2},"text":"Grüsse"}'),
]
REFUSED = [{"operation": "splice", "path": "/text", "offset": 3, "remove": 1, "value": ""}, {"operation": "set", "path": "/missing/0", "value": 1}]


def main() -> int:
    passed = 0
    for patch, expected in CASES:
        actual = json.dumps(HOST.patched_snapshot(SNAPSHOT, patch), ensure_ascii=False, separators=(",", ":"))
        if actual != expected:
            print(f"DIVERGED {patch}: {actual}")
            return 1
        passed += 1
    for patch in REFUSED:
        try:
            HOST.patched_snapshot(SNAPSHOT, patch)
        except AssertionError:
            passed += 1
            continue
        print(f"NOT REFUSED {patch}")
        return 1
    for patch, _ in CASES:
        after = HOST.patched_snapshot(SNAPSHOT, patch)
        restored = HOST.patched_snapshot(after, HOST.snapshot_patch_inverse(SNAPSHOT, patch))
        if json.dumps(restored, ensure_ascii=False) != json.dumps(SNAPSHOT, ensure_ascii=False):
            print(f"INVERSE DIVERGED {patch}: {restored}")
            return 1
        passed += 1
    print(f"python patched_snapshot + snapshot_patch_inverse parity: {passed} passed / 0 failed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
