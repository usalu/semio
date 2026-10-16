#!/usr/bin/env python3
# 🔬️ Analysis-only rustc oracle (`--emit=metadata -o /dev/null`) per crate root: baseline monolith, split crates one by one, and the union of all producers.
import collections
import os
import re
import subprocess
import sys

sys.path.insert(0, os.path.dirname(__file__))
from lib import *  # noqa: E402,F401,F403

SCRATCH = os.environ["SCRATCH"]
TOOLCHAIN = "+nightly-2026-07-20"
REL = "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract"
BASE_ROOT = SCRATCH + "/baseline/" + REL
SPLIT_TREE = SCRATCH + "/split"
SPLIT_C = SPLIT_TREE + "/" + REL
CONFIGS = {
    "native": [],
    "native-testing": ["--cfg", 'feature="testing"'],
    "test": ["--test"],
    "wasip2": ["--target", "wasm32-wasip2"],
    "wasip2-test": ["--target", "wasm32-wasip2", "--test"],
    "wasm": ["--target", "wasm32-unknown-unknown"],
    "wasm-test": ["--target", "wasm32-unknown-unknown", "--test"],
    "test-testing": ["--test", "--cfg", 'feature="testing"'],
    "wasip2-test-testing": ["--target", "wasm32-wasip2", "--test", "--cfg", 'feature="testing"'],
    "wasm-test-testing": ["--target", "wasm32-unknown-unknown", "--test", "--cfg", 'feature="testing"'],
}
EXTERNAL = re.compile(r"`(pack|base64|json_patch|serde_json|semio_framework_[a-z0-9_]+|semio_s_artifact_stdio_contract[a-z0-9_]*)`")


def sync_split():
    subprocess.run(["rm", "-rf", SPLIT_TREE], check=True)
    os.makedirs(SPLIT_TREE + "/✏️s/🔌️plugins/🗄️stdio", exist_ok=True)
    subprocess.run(["rsync", "-a", "--exclude", "node_modules", "--exclude", "dist", "--exclude", "target", ROOT + "/✏️s/🔌️plugins/🗄️stdio/📇️registry", SPLIT_TREE + "/✏️s/🔌️plugins/🗄️stdio/"], check=True)
    os.makedirs(SPLIT_TREE + "/🧰️framework/🔨️modules/🌱️value/🧬️bytes/🧪️tests", exist_ok=True)
    subprocess.run(["rsync", "-a", ROOT + "/🧰️framework/🔨️modules/🌱️value/🧬️bytes/🧪️tests/🧬️base64", SPLIT_TREE + "/🧰️framework/🔨️modules/🌱️value/🧬️bytes/🧪️tests/"], check=True)


def run(name, root, flags, extra=()):
    command = ["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", name, "--emit=metadata", "-o", "/dev/null", *flags, *extra, root]
    return subprocess.run(command, capture_output=True, text=True, cwd=os.path.dirname(root)).stderr


def blocks(stderr):
    for block in re.split(r"\n(?=(?:error|warning))", stderr):
        lines = block.split("\n")
        if lines and lines[0].startswith("error"):
            head = lines[0]
            if head.startswith("error: aborting") or head.startswith("error: could not compile"):
                continue
            location = next((line for line in lines[1:] if "-->" in line), "")
            yield head, re.sub(r":\d+:\d+$", "", location.split("-->")[-1].strip()), block


def internal_heads(stderr):
    rows = collections.Counter()
    for head, _, block in blocks(stderr):
        if EXTERNAL.search(head) or "no external crate" in block or head.startswith("error: cannot find attribute"):
            continue
        rows[head] += 1
    return rows


def external_count(stderr):
    return sum(1 for head, _, block in blocks(stderr) if EXTERNAL.search(head) or "no external crate" in block)


def roots():
    out = {"baseline": ("semio_s_artifact_stdio_contract", BASE_ROOT + "/🦀️.rs")}
    for crate in CRATES:
        out[crate["key"]] = (crate["lib"], SPLIT_C + "/" + crate["node"] + "/🦀️.rs")
    out["facade"] = ("semio_s_artifact_stdio_contract", SPLIT_C + "/🦀️.rs")
    return out


if __name__ == "__main__":
    which = sys.argv[1]
    configs = sys.argv[2].split(",") if len(sys.argv) > 2 else list(CONFIGS)
    out = SCRATCH + "/probe"
    os.makedirs(out, exist_ok=True)
    if which == "sync":
        sync_split()
        sys.exit(0)
    table = roots()
    for config in configs:
        for key, (name, root) in table.items():
            if which != "all" and which != key:
                continue
            stderr = run(name, root, CONFIGS[config])
            open(f"{out}/{key}-{config}.err", "w", encoding="utf8").write(stderr)
            print(f"{key:16s} {config:14s} internal={sum(internal_heads(stderr).values()):4d} external={external_count(stderr):4d}")
