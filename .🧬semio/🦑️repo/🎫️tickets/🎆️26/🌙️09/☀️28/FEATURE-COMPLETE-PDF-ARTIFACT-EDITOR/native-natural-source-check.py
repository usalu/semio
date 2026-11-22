"""📄 Execute the staged Rust sender and independent fixture oracle."""
import importlib.util
import json
import os
from pathlib import Path
import re
import subprocess
import sys

ticket = Path(__file__).resolve().parent
root = next(parent for parent in ticket.parents if (parent / "Cargo.toml").exists())
generated = ticket / "🗑️generated/native-natural-source"
fixture = ticket / "🗑️generated/pdf-segmented-source.pdf"
capture = generated / "received.pdf"
assert fixture.is_file(), "Run the shared segmented PDF source oracle to create the fixture"


def repo(*args, **options):
    subprocess.run(["repo", "tools", "exec", "--light", *args], cwd=root, check=True, **options)


repo("node", "--", ticket / "native-natural-source-oracle.mjs", fixture)
stage = importlib.util.spec_from_file_location("native_source_stage", ticket / "native-natural-source-stage.py")
sys.dont_write_bytecode = True
stage.loader.exec_module(importlib.util.module_from_spec(stage))
repo("git", "--", "apply", "--check", generated / "native-natural-source.patch")
inputs = json.loads((ticket / "native-natural-source.json").read_text())
source = json.loads((ticket / "🔣️segmented-pdf-source.json").read_text())
env = os.environ | {
    f"SEMIO_NATIVE_SOURCE_{key}": str(value)
    for key, value in {
        "PAGE_BYTES": inputs["pageBytes"], "PAGE_COUNT": inputs["pageCount"], "TAIL_BYTES": inputs["tailBytes"],
        "INSTANCE": inputs["instance"], "SEQUENCE": inputs["sequence"], "GENERATION": inputs["generation"],
        "OPERATION": inputs["operation"], "EXTENT": source["sourceBytes"], "FIXTURE": fixture, "CAPTURE": capture,
    }.items()
}


def extract(text, marker):
    start = text.index(marker)
    return text[start:text.index("\n}", start) + 2]


def wire_records(text):
    text = re.sub(r"^//!.*$|^#\[(?:serde|value)\(.*\)\]$", "", text, flags=re.MULTILINE)
    text = re.sub(r",?\s*(?:serde|semio_framework_value(?:_derive)?)::\w+", "", text)
    return text


ingress = next(root.glob("🧰*/🔨*/📡*/📡*/🎮*/📥*"))
command = (ingress / "🦀️.rs").read_text()
page = (next(ingress.glob("📄*")) / "🦀️.rs").read_text()
status = extract(command, "pub enum CommandIngressStatus {")
if "NaturalSourceAck" not in status:
    status = status.replace("    Idle,", "    Idle,\n    NaturalSourceAck(NaturalSourceWireAck),")
wire = generated / "wire.rs"
wire.write_text("\n".join([
    re.search(r"^pub const COMMAND_PAGE_MAXIMUM_BYTES:.*$", command, re.MULTILINE).group(),
    wire_records((ticket / "natural-pdf-wire-records.rs").read_text()),
    wire_records((ingress / "🎞️original-media.rs").read_text()),
    "#[derive(Debug, Clone, PartialEq, Eq)]",
    extract(command, "pub struct CommandPageCursor {"),
    extract(command, "pub struct FixedCommandPage {"),
    extract(page, "fn try_reserve_command_page_block()"),
    extract(page, "impl FixedCommandPage {"),
    status,
]))
env["SEMIO_NATIVE_SOURCE_WIRE"] = str(wire)

for name, path in {
    "PUMP": ticket / "native-natural-source-pump.rs", "QUEUE": generated / "queue.rs",
}.items():
    code = path.read_text().split("\n", 1)[1]
    include_path = generated / f"{name.lower()}-include.rs"
    include_path.write_text(code)
    env[f"SEMIO_NATIVE_SOURCE_{name}"] = str(include_path)

channel = re.search(r'^channel = "([^"]+)"', (root / "rust-toolchain.toml").read_text(), re.MULTILINE).group(1)
test = generated / ("native-source-test.exe" if os.name == "nt" else "native-source-test")
repo("rustup", "--", "run", channel, "rustc", "--edition", "2024", "--test",
     str(ticket / "native-natural-source-test.rs"), "-o", str(test), env=env)
subprocess.run([test], cwd=root, check=True, env=env)
repo("node", "--", ticket / "native-natural-source-oracle.mjs", fixture, capture)
