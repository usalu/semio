#!/usr/bin/env python3
"""🔌️ Self-wires each artifact's fixture cases into its OWN mutations-root `🦀️component.rs`.

`📦️glue.rs` is shared with concurrent lanes and is deliberately NOT touched. A `#[path]` on a
module declared at the top level of a non-mod-rs file resolves relative to that file's own
directory, so `#[path = "."] mod fixture_tests { … }` anchors the children at `🧬️mutations/`.
Ticket scratch, not a permanent script.
"""

import importlib.util
import os
import re
import sys

_spec = importlib.util.spec_from_file_location("emit_common", os.path.join(os.path.dirname(__file__), "\U0001f4dc️emit-common.py"))
common = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(common)

REGION_OPEN = "//#region \U0001f9ea️FixtureTests"
REGION_CLOSE = "//#endregion \U0001f9ea️FixtureTests"


def _ident(text):
    return re.sub(r"[^0-9a-zA-Z]+", "_", text).strip("_")


def wire(artifact_dir, kinds):
    """🧵 `kinds` is the artifact's list of kebab mutation kinds, one fixture case per kind."""
    root = common.mutations_root(artifact_dir)
    entries = []
    for kind in sorted(kinds):
        leaf_dir = common.resolve_leaf_dir(artifact_dir, kind)
        tests_dir = os.path.join(root, leaf_dir, common.TESTS_DIR)
        cases = sorted(entry for entry in os.listdir(tests_dir) if os.path.isdir(os.path.join(tests_dir, entry)))
        if not cases:
            raise SystemExit("no case for %s/%s" % (artifact_dir, kind))
        for case in cases:
            relative = "%s/%s/%s/%s" % (leaf_dir, common.TESTS_DIR, case, common.TEST_RS)
            absolute = os.path.join(root, relative)
            if not os.path.isfile(absolute):
                raise SystemExit("missing %s" % absolute)
            entries.append((relative, "tests_%s_%s" % (_ident(kind), _ident(case))))

    block = [
        REGION_OPEN,
        "/// \U0001f9ea️ Handcrafted mutation fixtures (contract D1, ticket `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`),",
        "/// self-wired here rather than in `\U0001f4e6️glue.rs`: that file is shared with the other artifact lanes",
        "/// running concurrently, and a `#[path]` on a module declared at the top level of this non-mod-rs",
        "/// file already resolves relative to this very directory.",
        "#[cfg(test)]",
        "#[path = \".\"]",
        "mod fixture_tests {",
    ]
    for relative, module in entries:
        block.append("    #[path = \"%s\"]" % relative)
        block.append("    mod %s;" % module)
    block.append("}")
    block.append(REGION_CLOSE)
    block = "\n".join(block) + "\n"

    path = os.path.join(root, common.TEST_RS)
    with open(path, encoding="utf-8") as handle:
        source = handle.read()
    if REGION_OPEN in source:
        head, _, rest = source.partition(REGION_OPEN)
        _, _, tail = rest.partition(REGION_CLOSE + "\n")
        source = head + tail
    source = source.rstrip("\n") + "\n\n" + block
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(source)
    print("%s: wired %d fixture module(s)" % (artifact_dir, len(entries)))


if __name__ == "__main__":
    sys.exit("import this module from a per-artifact table script")
