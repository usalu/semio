"""🧪️ Move the three existing reducer-fault laws to their own retained-command test target."""

import hashlib
import json
from pathlib import Path
import re

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
PACKAGE = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command")
TEST = PACKAGE / "🧪️tests/🔬️unit/🦀️.rs"
MAIN = PACKAGE / "🦀️.rs"
MANIFEST = PACKAGE / "📦️packages/🦀️rust/Cargo.toml"
OUT = TICKET / "🗑️generated/raw-retained-cap-overlay.json"
TARGET = '''[[test]]
name = "reducer-fault-detail"
path = "../../🧪️tests/🔬️unit/🦀️.rs"

'''
OWNER = '''#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub mod tests;
'''


def stage():
    preimages = {str(p): hashlib.sha256((ROOT / p).read_bytes()).hexdigest() for p in (TEST, MAIN, MANIFEST)}
    source = {str(p): (ROOT / p).read_text(encoding="utf8") for p in (TEST, MAIN, MANIFEST)}
    original = source[str(TEST)]
    owner = source[str(MAIN)]
    manifest = source[str(MANIFEST)]
    if owner.count(OWNER) != 1 or manifest.count('[[test]]') != 1 or original.count("crate::reducer_fault_detail") != 3:
        raise ValueError("Retained reducer fault ownership drifted")
    roster = re.findall(r"^#\[test\]\nfn ([A-Za-z0-9_]+)\(", original, re.MULTILINE)
    if len(set(roster)) != 3:
        raise ValueError("Retained reducer fault roster drifted")
    external = original.replace("crate::reducer_fault_detail", "semio_plugin_retained_command::reducer_fault_detail")
    external = external.replace("crate::ARTIFACT_COMMAND_FAULT_DETAIL_MAXIMUM_BYTES",
                                "semio_plugin_retained_work::ARTIFACT_COMMAND_FAULT_DETAIL_MAXIMUM_BYTES")
    if re.findall(r"^#\[test\]\nfn ([A-Za-z0-9_]+)\(", external, re.MULTILINE) != roster:
        raise ValueError("Retained test split changed the exact roster")
    source[str(TEST)] = external
    source[str(MAIN)] = owner.replace(OWNER, "")
    source[str(MANIFEST)] = manifest.replace("[[test]]", TARGET + "[[test]]")
    for path, expected in preimages.items():
        if hashlib.sha256((ROOT / path).read_bytes()).hexdigest() != expected:
            raise ValueError(f"Retained source changed before staging: {path}")
    OUT.parent.mkdir(exist_ok=True)
    OUT.write_text(json.dumps({"preimages": preimages, "candidate": source, "roster": roster}, ensure_ascii=False), encoding="utf8")
    print("[DEBUG] Guarded the existing retained-command reducer-fault roster and explicit test target")


if __name__ == "__main__":
    stage()
