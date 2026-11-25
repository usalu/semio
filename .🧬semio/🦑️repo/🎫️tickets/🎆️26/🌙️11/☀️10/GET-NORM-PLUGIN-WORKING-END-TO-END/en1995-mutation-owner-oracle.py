"""🪵 Verify the baseline EN 1995 wire order and inverse owner boundaries."""

from collections import Counter
from hashlib import sha256
import json
from pathlib import Path
import re
import sys


def normalize(name):
    return "".join(char for char in name if char.isalnum())


root = Path(sys.argv[1])
rust = (root / "🦀️.rs").read_text(encoding="utf-8")
schema = json.loads((root / "🔣️.json").read_text(encoding="utf-8"))
proto = (root / "🛰️.proto").read_text(encoding="utf-8")

tags = [next(iter(branch["properties"])) for branch in schema["oneOf"]]
rust_variants = re.findall(r"^    (\w+)\((\w+)::(\w+)\),$", rust, re.MULTILINE)
oneof = re.search(r"message En1995Mutation\s*\{\s*oneof kind\s*\{([^}]+)\}", proto)[1]
proto_variants = re.findall(r"^\s*(\w+) \w+ = (\d+);$", oneof, re.MULTILINE)
native_tags = [tag for tag, _, _ in rust_variants]
native_order = sha256("\n".join(native_tags).encode("utf-8")).hexdigest()
assert native_order == "147bb0c9e83f1c04a26343597002dcb2103ad39dabc70c457cc37fffcc1b7e4f"
assert len(native_tags) == len(tags)
assert set(native_tags) == set(tags)
assert [tag for tag, _ in proto_variants] == tags
assert [int(number) for _, number in proto_variants] == list(range(1, len(tags) + 1))
assert len(set(tags)) == len(tags)

ownership = {
    normalize(module): owner
    for owner, module in re.findall(
        r"^pub use semio_s_artifact_norm_en1995_mut_(\w+)::(\w+);$", rust, re.MULTILINE
    )
}
assert len(ownership) == len(tags)
ordered_modules = [normalize(module) for _, module, _ in rust_variants]
assert set(ordered_modules) == set(ownership)
assert len(set(ordered_modules)) == len(tags)

groups = Counter(ownership[module] for module in ordered_modules)
inverse_count = Counter()
for path in root.glob("**/↩️inverse/🦀️.rs"):
    source = normalize(path.parent.parent.name)
    helper = path.read_text(encoding="utf-8")
    destination = re.findall(
        r"^pub fn inverse\([^\n]*\) -> Option<crate::(\w+)::(\w+)>", helper, re.MULTILINE
    )
    assert len(destination) == 1, path
    module, payload = destination[0]
    assert ownership[source] == ownership[normalize(module)], path
    assert len(re.findall(rf"\bSome\((?:\w+::)*{payload}\s*\{{", helper)) == 1, path
    assert "Vec<Op>" not in helper and "Op::from" not in helper, path
    caller = (path.parent.parent / "🦀️.rs").read_text(encoding="utf-8")
    wrapper = re.search(
        r"Ok\(match crate::\w+::inverse::inverse\(self, base\) \{"
        r"\s*Some\(inverse\) => vec!\[Op::from\(inverse\)\],"
        r"\s*None => Vec::new\(\),\s*\}\)",
        caller,
    )
    assert wrapper, path
    inverse_count[source] += 1

assert inverse_count == Counter(ordered_modules)
ordinals_differ = sum(wire != native for wire, native in zip(tags, native_tags))
print(f"{len(tags)} JSON/Rust/Protobuf leaves, {ordinals_differ} distinct native ordinals, "
      f"{len(groups)} owners, "
      f"{max(groups.values())} leaves in largest owner, {sum(inverse_count.values())} local inverses")
