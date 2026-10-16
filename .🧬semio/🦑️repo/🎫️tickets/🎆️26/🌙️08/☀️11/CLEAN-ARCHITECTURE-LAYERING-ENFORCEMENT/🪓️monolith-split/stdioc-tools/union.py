#!/usr/bin/env python3
# 🧬️ Union oracle: every producer crate and the facade mounted as modules of ONE crate (crate-name paths rewritten to module paths, `pub(crate)` narrowed to the owning module),
# so cross-crate paths, visibility and the facade re-exports are resolved by the full-resolution oracle. Also probes every public path the dependents use.
import collections
import os
import re
import shutil
import subprocess
import sys

sys.path.insert(0, os.path.dirname(__file__))
import probe  # noqa: E402
from lib import *  # noqa: E402,F401,F403

SCRATCH = os.environ["SCRATCH"]
UNION = SCRATCH + "/union"
REL = probe.REL
UNION_C = UNION + "/" + REL
BASE_UNION = SCRATCH + "/union-baseline"
NAMES = [crate["lib"] for crate in CRATES] + ["semio_s_artifact_stdio_contract"]


def rewrite(path, module):
    text = read(path)
    original = text
    def dollar(match):
        name, bang = match.group(1), match.group(2)
        return f"$crate::{name}{bang}" if bang == "!" else f"$crate::{module}::{name}{bang}"

    text = re.sub(r"\$crate::([A-Za-z_][A-Za-z0-9_]*)(!?)", dollar, text)

    def plain(match):
        name, bang = match.group(1), match.group(2)
        return f"crate::{name}{bang}" if bang == "!" else f"crate::{module}::{name}{bang}"

    text = re.sub(r"(?<![\w$:])crate::([A-Za-z_][A-Za-z0-9_]*)(!?)", plain, text)
    text = re.sub(r"(?<![\w$:])crate::\{", f"crate::{module}::{{", text)
    for name in NAMES:
        text = re.sub(r"(?<![\w:])" + name + r"\b", "crate::" + name, text)
    text = text.replace("pub(crate)", f"pub(in crate::{module})")
    if text != original:
        write(path, text)


def build_union():
    if os.path.exists(UNION):
        shutil.rmtree(UNION)
    shutil.copytree(probe.SPLIT_TREE, UNION, symlinks=True)
    modules = {}
    for crate in CRATES:
        root = UNION_C + "/" + crate["node"] + "/🦀️.rs"
        modules[crate["lib"]] = root
    modules["semio_s_artifact_stdio_contract"] = UNION_C + "/🦀️.rs"
    for module, root in modules.items():
        for path in closure(root):
            rewrite(path, module)
    macros = ["stdio_list_delta", "impl_serde_op_codec", "bounded_native_editing_editor", "snapshot_details_editor_support", "snapshot_editing_bounded_first_step_tool_proofs", "snapshot_editing_command_roster", "snapshot_editing_publication_authority"]
    facade_path = UNION_C + "/🦀️.rs"
    facade = read(facade_path)
    for macro in macros:
        facade = re.sub(r"pub use crate::[a-z_0-9]+::" + macro + r";\n", "", facade)
        facade = re.sub(r"(?<![\w:])" + macro + r"\b,?\s*", "", facade)
    facade = re.sub(r",\s*\}", "}", facade)
    write(facade_path, facade)
    part21 = UNION_C + "/📐️part21/🦀️.rs"
    text = read(part21)
    text = re.sub(r"#\[cfg\(test\)\]\n#\[global_allocator\]\nstatic[^\n]*\n", "", text)
    write(part21, text)
    union_root = UNION + "/union.rs"
    extra = {"headroom_details": UNION_C + "/🧪️tests/🎟️details-arena-headroom/🦀️.rs", "headroom_table": UNION_C + "/🧪️tests/📊️table-arena-headroom/🦀️.rs"}
    for module, root in extra.items():
        rewrite(root, module)
    write(union_root, "".join(f'#[path = "{root}"]\npub mod {module};\n' for module, root in {**modules, **extra}.items()))
    return union_root


def build_base_union():
    if os.path.exists(BASE_UNION):
        shutil.rmtree(BASE_UNION)
    shutil.copytree(SCRATCH + "/baseline", BASE_UNION, symlinks=True)
    root = BASE_UNION + "/" + REL + "/🦀️.rs"
    for path in closure(root):
        rewrite(path, "semio_s_artifact_stdio_contract")
    extra = {"headroom_details": BASE_UNION + "/" + REL + "/🧪️tests/🎟️details-arena-headroom/🦀️.rs", "headroom_table": BASE_UNION + "/" + REL + "/🧪️tests/📊️table-arena-headroom/🦀️.rs"}
    for module, path in extra.items():
        rewrite(path, module)
    write(BASE_UNION + "/union.rs", f'#[path = "{root}"]\npub mod semio_s_artifact_stdio_contract;\n' + "".join(f'#[path = "{path}"]\npub mod {module};\n' for module, path in extra.items()))
    return BASE_UNION + "/union.rs"


def uses_probe(path, uses):
    text = "".join(f"#[allow(unused_imports)]\nuse crate::semio_s_artifact_stdio_contract::{use_path};\n" for use_path in uses)
    write(path, text)


if __name__ == "__main__":
    configs = sys.argv[1].split(",") if len(sys.argv) > 1 else list(probe.CONFIGS)
    probe.sync_split()
    union_root = build_union()
    base_root = build_base_union()
    macro_paths = {"stdio_list_delta", "impl_serde_op_codec", "bounded_native_editing_editor", "snapshot_details_editor_support", "snapshot_editing_bounded_first_step_tool_proofs", "snapshot_editing_command_roster", "snapshot_editing_publication_authority"}
    uses = [line.strip() for line in open(ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/stdioc-api-uses.txt", encoding="utf8") if line.strip() and line.strip() not in macro_paths]
    os.makedirs(SCRATCH + "/probe", exist_ok=True)
    for label, root, directory in (("baseline-union", base_root, BASE_UNION), ("split-union", union_root, UNION)):
        uses_probe(directory + "/uses.rs", uses)
        text = read(root)
        if "mod uses;" not in text:
            write(root, text + f'#[path = "{directory}/uses.rs"]\npub mod uses;\n')
    for config in configs:
        flags = probe.CONFIGS[config]
        results = {}
        for label, root in (("baseline-union", base_root), ("split-union", union_root)):
            stderr = probe.run("union", root, flags)
            open(f"{SCRATCH}/probe/{label}-{config}.err", "w", encoding="utf8").write(stderr)
            results[label] = probe.internal_heads(stderr)
        base, split = results["baseline-union"], results["split-union"]
        print(f"== {config}: baseline-union internal {sum(base.values())} split-union internal {sum(split.values())}")
        for message in sorted(set(base) | set(split)):
            if base[message] != split[message]:
                print(f"   {base[message]:3d} -> {split[message]:3d}  {message}")
