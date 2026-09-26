#!/usr/bin/env python3
"""🧰️ Prepared patch (rule 20, apply after W2's `--packages all`): `🧰️mutate-semio-kit`'s identity round trip is red in
BOTH roles because the capsule tower's binary twin `🧫️fixtures/🧰️mutate-semio-kit/🎒️.pack.semio` (2026-08-26) predates
the 2026-09-20 edit of its text twin that made the properties child's target artifact id equal its child id
(`kit-props` → `props-01`, the composed-child identity rule). The two decode to kits that differ in exactly that one
member. The twin is regenerated from the committed text through the reference's pack writer — which first has to
prove it is byte-faithful by re-encoding the committed twin to the identical 50 019 bytes — and the result must
decode back to the text's kit.
Usage: kit-tower-pack.py --dry-run | --write [--root <dir>]"""
import importlib.util
import sys
from pathlib import Path

REPO = Path("/Users/ueli/Documents/semio")
root = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else REPO
KIT = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit"
TEXT = root / KIT / "🧫️fixtures/🧰️mutate-semio-kit/🏢️nakagin-capsule-tower/🗣️.dsl.semio"
PACK = root / KIT / "🧫️fixtures/🧰️mutate-semio-kit/🎒️.pack.semio"
spec = importlib.util.spec_from_file_location("semio_repo_test", REPO / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py")
host = importlib.util.module_from_spec(spec)
sys.modules["semio_repo_test"] = host
spec.loader.exec_module(host)
spec = importlib.util.spec_from_file_location("kit_reference", REPO / KIT / "🧪️tests/🧰️mutate-semio-kit/🐍️.py")
reference = importlib.util.module_from_spec(spec)
spec.loader.exec_module(reference)


def differences(left, right, path=""):
    """🔍️ Every member path at which two decoded kits disagree."""
    if type(left) is not type(right):
        return [path]
    if isinstance(left, dict):
        return [found for key in sorted(set(left) | set(right)) for found in (differences(left[key], right[key], f"{path}/{key}") if key in left and key in right else [f"{path}/{key}"])]
    if isinstance(left, list):
        return ([path] if len(left) != len(right) else []) + [found for at, (a, b) in enumerate(zip(left, right)) for found in differences(a, b, f"{path}/{at}")]
    return [] if left == right else [path]


write = "--write" in sys.argv
problems = []
committed = PACK.read_bytes()
text = reference.parse_dsl(TEXT.read_text(encoding="utf-8"))
twin = reference.parse_pack(committed)
if reference.pack_bytes(twin) != committed:
    problems.append("the reference's pack writer does not reproduce the committed twin byte-exactly, so it cannot regenerate it")
drift = differences(text, twin)
if drift != ["/properties/target/artifactId"]:
    problems.append(f"the text and its twin disagree at {drift[:6]}, not only at the properties target id")
regenerated = reference.pack_bytes(text)
if reference.parse_pack(regenerated) != text:
    problems.append("the regenerated twin does not decode back to the text's kit")
print(f"files=1 problems={len(problems)} write={write} bytes {len(committed)} -> {len(regenerated)}")
for problem in problems:
    print("PROBLEM", problem)
if write and not problems:
    PACK.write_bytes(regenerated)
