#!/usr/bin/env python3
# 🪚 Stage 3: rewires the leaf nodes (part21, list-delta) from `crate::` mounts to crate imports.
import os

SCRATCH = os.environ["SCRATCH"]
C = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract"
BASE = SCRATCH + "/baseline/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract"


def read(path):
    return open(path, encoding="utf8").read()


def write(path, text):
    open(path, "w", encoding="utf8").write(text)


def replace_once(text, old, new):
    assert text.count(old) == 1, (old[:80], text.count(old))
    return text.replace(old, new)


part21 = read(BASE + "/📐️part21/🦀️.rs")
part21 = replace_once(part21, "use crate::value_derive;\n", "use semio_framework_value_derive as value_derive;\n")
part21 = replace_once(part21, "\nuse semio_framework_value::{edit_through_value,", "\n#[cfg(test)]\n#[global_allocator]\nstatic ORIGINAL_OWNER_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;\n\nuse semio_framework_value::{edit_through_value,")
write(C + "/📐️part21/🦀️.rs", part21)

cohort = read(BASE + "/📐️part21/🧪️tests/🚦️sqlite-cohort/🦀️.rs")
count = cohort.count("crate::testing::decode_control(")
cohort = cohort.replace("crate::testing::decode_control(", "decode_control(")
cohort = replace_once(cohort, "use super::*;\n", "use super::*;\nuse semio_s_artifact_stdio_contract_testing::decode_control;\n")
write(C + "/📐️part21/🧪️tests/🚦️sqlite-cohort/🦀️.rs", cohort)
print("decode_control rewrites", count)

delta = read(BASE + "/🪡️list-delta/🦀️.rs")
count = delta.count("$crate::list_delta::")
delta = delta.replace("$crate::list_delta::", "$crate::")
delta = replace_once(delta, "#[doc(hidden)]\npub use semio_framework_value_derive as __value_derive;\n", "pub use semio_framework_os_kernel as kernel;\n\n#[doc(hidden)]\npub use semio_framework_value_derive as __value_derive;\n")
write(C + "/🪡️list-delta/🦀️.rs", delta)
print("list_delta macro paths rewritten", count)
