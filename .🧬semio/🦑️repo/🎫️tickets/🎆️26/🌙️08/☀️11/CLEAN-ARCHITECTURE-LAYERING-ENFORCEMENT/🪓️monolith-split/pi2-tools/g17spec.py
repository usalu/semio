#!/usr/bin/env python3
"""🧱️ Spec of group G17: builder, testing-laws, testing-acceptance (ranks 57 to 59)."""
from lib import *
import groupdrv as G


def authoring_macro(text):
    return text.replace("crate::with_authoring_identity", "semio_framework_plugin_testing_laws::with_authoring_identity")


def drop_self_macro(text):
    import re
    new, k = re.subn(r"(?m)^(?:#\[[^\n]*\]\n)*use crate::with_authoring_identity;\n", "", text)
    return new


class S(G.Spec):
    name = "g17"
    group = ["builder", "testing-laws", "testing-acceptance"]
    roots_b = {"builder": PLUG + "/🏗️builder/🦀️.rs", "testing-laws": PLUG + "/🧪️testing/🦀️.rs", "testing-acceptance": PLUG + "/🧪️tests/🧪️history-edit-acceptance/🦀️.rs"}
    hub_file = PLUG + "/🪝️mounts/🧵️wp7-instance-tier2/🦀️.rs"
    hub_units = {}
    skip_files = set()
    extra_edits = {PLUG + "/🧪️testing/🦀️.rs": drop_self_macro, PLUG + "/🧪️tests/🧪️history-edit-acceptance/🦀️.rs": authoring_macro, PLUG + "/🧪️tests/🧪️node-drag-history/🦀️.rs": authoring_macro}


if __name__ == "__main__":
    G.main(S)
