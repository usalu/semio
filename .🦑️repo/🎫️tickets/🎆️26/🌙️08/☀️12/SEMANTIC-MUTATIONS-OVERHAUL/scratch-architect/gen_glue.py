#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import json

PLAN = json.load(open("/tmp/architect_plan_full.json", encoding="utf-8"))
GLUE = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/📦️glue.rs"
REL_BASE = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"

glue = open(GLUE, encoding="utf-8").read()

start_marker = "                            pub mod mutations {\n"
start = glue.index(start_marker)
# find start of the block that begins right after `#[path = "."]\n` above start_marker line's own
# opening — we already know exact structure: header lines are:
#   #[path = "."]\n                            pub mod mutations {\n#[path=".../mutations/component.rs"]\n mod component;\n pub use component::*;\n#[path=".../text/component.rs"]\npub mod text;\n#[path=".../binary/component.rs"]\npub mod binary;\n
# then the per-register blocks begin. We'll splice: keep everything up to and including
# `pub mod binary;\n                            }\n` (the binary block's own closing brace, i.e.
# the boilerplate before register blocks start), find first `pub mod information {` and the
# matching close of the WHOLE `pub mod mutations { ... }` block, then replace everything between.

first_register_marker = '                                pub mod information {\n'
first_idx = glue.index(first_register_marker)

# Find the matching close brace for `pub mod mutations {` opened at `start`.
open_idx = glue.index("{", start)
depth = 1
i = open_idx + 1
while depth > 0:
    if glue[i] == "{":
        depth += 1
    elif glue[i] == "}":
        depth -= 1
    i += 1
mutations_block_end = i  # index just after the matching closing brace

prefix = glue[:first_idx]
suffix = glue[mutations_block_end - 1:]  # start from the closing '}' of `pub mod mutations {`

INDENT = " " * 32
blocks = []
for p in PLAN:
    mod = p["kind_slug"].replace("-", "_")
    d = p["new_dir"]
    blocks.append(
        f'{INDENT}#[path = "."]\n'
        f'{INDENT}pub mod {mod} {{\n'
        f'{INDENT}    #[path = "{REL_BASE}/{d}/🦠️mutation/🦀️component.rs"]\n'
        f'{INDENT}    pub mod mutation;\n'
        f'{INDENT}    #[path = "{REL_BASE}/{d}/🔺️diff/🦀️component.rs"]\n'
        f'{INDENT}    pub mod diff;\n'
        f'{INDENT}    #[path = "{REL_BASE}/{d}/↩️inverse/🦀️component.rs"]\n'
        f'{INDENT}    pub mod inverse;\n'
        f'{INDENT}}}\n'
    )

new_glue = prefix + "".join(blocks) + suffix
open(GLUE, "w", encoding="utf-8").write(new_glue)
print("glue.rs rewritten. New size:", len(new_glue), "old size:", len(glue))
