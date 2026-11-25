"""↩️ Compare EN 1995 inverse computations across the concrete owner extraction."""

import json
from pathlib import Path
import re
import subprocess
import sys


def logic(source):
    before, function = source.split("pub fn inverse", 1)
    signature, code = function.split(" {\n", 1)
    body, after = re.split(r"^}\n", code, maxsplit=1, flags=re.MULTILINE)
    body = body.rstrip()
    if body.startswith("    Ok((|| {"):
        body = body.removeprefix("    Ok((|| {\n").removesuffix("\n    })())")
    elif body.startswith("    Ok(match "):
        body = body.replace("    Ok(match ", "    match ", 1).removesuffix(")")
    body = re.sub(r"Ok\((Vec::new\(\))\)", r"\1", body)
    body = re.sub(r"Ok\((vec!\[Op::from\(.*\)\])\)", r"\1", body)
    body = body.replace("Vec::new()", "None").replace("vec![Op::from(", "Some(").replace("})]", "})")
    match = re.search(r"(?:From<|Option<)(crate::\w+::\w+)>", signature)
    return before, match[1], body.rstrip(), after


root = Path(sys.argv[1])
revision = sys.argv[2]
paths = sorted(root.glob("**/↩️inverse/🦀️.rs"))
requests = b"".join(f"{revision}:{path.as_posix()}\n".encode() for path in paths)
baseline = subprocess.run(["git", "cat-file", "--batch"], input=requests, check=True, capture_output=True).stdout
cursor = 0
for path in paths:
    end = baseline.index(b"\n", cursor)
    _object, kind, length = baseline[cursor:end].split()
    assert kind == b"blob", path
    cursor = end + 1
    end = cursor + int(length)
    old = baseline[cursor:end].decode()
    assert logic(old) == logic(path.read_text(encoding="utf-8")), path
    cursor = end + 1
assert cursor == len(baseline)
assert len(paths) == 206

fixture = root.parents[1] / "🧪️tests/🧫️fixtures/🧬️mutations/📐️change-taper-taper-angle/✅apply"
before, after, envelope = (
    json.loads((fixture / f"🔣️.{name}.json").read_text(encoding="utf-8"))
    for name in ("before", "after", "mutation")
)
mutation = envelope["ChangeTaperTaperAngle"]
taper_id = mutation["taperId"]
old_value = next(row["taperAngleRad"] for row in before["tapers"] if row["id"] == taper_id)
new_value = next(row["taperAngleRad"] for row in after["tapers"] if row["id"] == taper_id)
assert (old_value, new_value, mutation["newValue"]) == (0.1, 1.15, 1.15)

print(f"{len(paths)} unchanged inverse expressions and concrete return payloads")
print(f"{taper_id} fixture restores taper angle {old_value} after change to {new_value}")
