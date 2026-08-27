#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import json, re, os

PLAN = json.load(open("/tmp/architect_plan_full.json", encoding="utf-8"))
NAME2MOD = json.load(open("/tmp/architect_name_to_module.json", encoding="utf-8"))
MUT_DIR = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
GLUE = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/📦️glue.rs"
DISPATCH = f"{MUT_DIR}/🦀️component.rs"

# order plan by struct_name for stable, predictable output; but for the dispatch enum we want to
# preserve the ORIGINAL variant ordering (read from the existing dispatch file) so the diff is
# minimal and the existing tests below the enum remain valid without reordering assertions.
dispatch_src = open(DISPATCH, encoding="utf-8").read()

# 1) Rewrite each `super::<old_module>::mutation::<Type>` variant line to
#    `super::<new_module>::mutation::<Type>` using NAME2MOD (keyed by struct name).
def rewrite_variant_line(m):
    struct_name = m.group("type")
    new_mod = NAME2MOD[struct_name]
    return f"    {m.group('variant')}(super::{new_mod}::mutation::{struct_name}),"

variant_line_re = re.compile(r"^    (?P<variant>[A-Za-z0-9]+)\(super::\w+::mutation::(?P<type>[A-Za-z0-9]+)\),$", re.MULTILINE)
new_dispatch_enum_part, n = variant_line_re.subn(rewrite_variant_line, dispatch_src)
print("enum variant lines rewritten:", n)

# 2) Rewrite every other in-file reference `super::<old_module>::mutation::<Type>` (used inside the
#    #[cfg(test)] block for constructing payloads) the same way. Regex captures generic form.
generic_ref_re = re.compile(r"super::(\w+)::mutation::([A-Za-z0-9]+)")
def rewrite_generic(m):
    old_mod, struct_name = m.group(1), m.group(2)
    new_mod = NAME2MOD.get(struct_name)
    if not new_mod:
        return m.group(0)
    return f"super::{new_mod}::mutation::{struct_name}"

final_dispatch, n2 = generic_ref_re.subn(rewrite_generic, new_dispatch_enum_part)
print("generic super::X::mutation refs rewritten:", n2)

open(DISPATCH, "w", encoding="utf-8").write(final_dispatch)
