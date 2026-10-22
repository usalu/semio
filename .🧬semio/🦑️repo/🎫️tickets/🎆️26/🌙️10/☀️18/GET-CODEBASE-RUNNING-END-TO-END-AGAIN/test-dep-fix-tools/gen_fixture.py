#!/usr/bin/env python3
"""Writes the withheld-dependency fixture: the case inputs below plus the expected block computed by the independent oracle model (reviewed by eye after generation)."""
import hashlib, importlib.util, json, subprocess, sys
D = sys.argv[1]
fx = D + "/🔣️.json"
def sha(text): return hashlib.sha256(text.encode()).hexdigest()
EMPTY = hashlib.sha256(b"").hexdigest()
def test(root, *names, extra=()):
    return {"root": root, "tags": ["type:test", "owner:" + root.rsplit("/", 1)[0]], "targets": {"test": {"inputs": ["{workspaceRoot}/" + root + "/**/*", {"input": "nativeSources", "projects": list(names)}, {"env": "SEMIO_TEST_LEVEL"}, *extra]}}}
def plain(root): return {"root": root, "targets": {"build": {"inputs": []}}}
cases = [
 {"id": "complete-inventory-withholds-nothing", "projects": {"crate-a": plain("libs/a"), "test-one": test("x/tests", "crate-a")}, "configs": {"test-one": "x/tests/case/spec.feature"}, "digests": {"x/tests/case/spec.feature": sha("one")}},
 {"id": "project-created-after-the-snapshot", "projects": {"crate-a": plain("libs/a"), "test-one": test("x/tests", "crate-a", "crate-new")}, "configs": {"test-one": "x/tests/case/spec.feature"}, "digests": {"x/tests/case/spec.feature": sha("one")}},
 {"id": "missing-names-in-code-point-order", "projects": {"test-one": test("x/tests", "b-crate", "é-crate", "B-crate")}, "configs": {"test-one": "x/tests/case/spec.feature"}, "digests": {"x/tests/case/spec.feature": sha("one")}},
 {"id": "only-the-affected-test-project-waits", "projects": {"crate-a": plain("libs/a"), "test-one": test("x/tests", "crate-a"), "test-two": test("y/tests", "crate-a", "crate-gone"), "test-three": test("z/tests", "crate-a")}, "configs": {"test-one": "x/tests/case/spec.feature", "test-two": "y/tests/case/spec.feature", "test-three": "z/tests/case/spec.feature"}, "digests": {"x/tests/case/spec.feature": sha("one"), "y/tests/case/spec.feature": sha("two"), "z/tests/case/spec.feature": sha("three")}},
 {"id": "a-project-that-is-no-test-is-not-checked", "projects": {"crate-a": {"root": "libs/a", "targets": {"build": {"inputs": [{"input": "nativeSources", "projects": ["crate-gone"]}]}}}}, "configs": {}, "digests": {}},
 {"id": "a-vanished-declaring-file-has-the-digest-of-no-bytes", "projects": {"test-one": test("x/tests", "crate-new")}, "configs": {"test-one": "x/tests/case/spec.feature"}, "digests": {}},
 {"id": "no-declaring-file-falls-back-to-the-project-root", "projects": {"test-one": test("x/tests", "crate-new")}, "configs": {}, "digests": {"x/tests": sha("dir")}},
 {"id": "a-test-without-native-sources-waits-for-nothing", "projects": {"test-one": {"root": "x/tests", "tags": ["type:test"], "targets": {"lint": {"inputs": ["{workspaceRoot}/x/**/*"]}}}}, "configs": {}, "digests": {}},
]
doc = {"version": 1, "emptyDigest": EMPTY, "cases": [dict(case, expected=None) for case in cases]}
json.dump(doc, open(fx, "w", encoding="utf8"), ensure_ascii=False, indent=2)
out = json.loads(subprocess.run(["python3", "-I", D + "/🐍️oracle.py", fx, sys.argv[2]], capture_output=True, text=True, check=True).stdout)
for case in doc["cases"]: case["expected"] = out[case["id"]]
json.dump(doc, open(fx, "w", encoding="utf8"), ensure_ascii=False, indent=2)
open(fx, "a").write("\n")
