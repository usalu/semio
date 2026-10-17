#!/usr/bin/env python3
"""🧪️ Spec of the test-bin group GT: every facade test root moves into a companion bin (map in PI2_MAP, bins in PI2_MAP.bins.json); the provider hosts that other bins call are mounted without `cfg(test)`; hub lines that name moved hosts are dropped."""
from lib import *
import groupdrv as G
import json, os, re

BINS = json.load(open(os.environ["PI2_MAP"] + ".bins.json")) if os.environ.get("PI2_MAP") else {}
H3 = PLUG + "/🪝️mounts/👥️wp3-owners/🦀️.rs"
H2 = PLUG + "/🪝️mounts/📜️wp2-declaration/🦀️.rs"
ROOT = PLUG + "/🦀️.rs"
MOVED_USES = [
    r"    #\[cfg\(test\)\]\n    pub\(crate\) use super::(?:child_test_hosts|publication_mounted_test_hosts|retained_command_test_hosts)::[^;]*;\n",
    r"    #\[cfg\(any\(test, feature = \"media-turn-testing\"\)\)\]\n    pub\(crate\) use super::media_publication_tests_host::[^;]*;\n",
    r"    #\[cfg\(test\)\]\n    pub\(crate\) use super::declarations_fixture_tests::\{fixture\};\n",
]


def drop_moved_uses(text):
    for rx in MOVED_USES:
        text = re.sub(rx, "", text)
    return text


class S(G.Spec):
    name = os.environ.get("PI2_GT_NAME", "gt")
    group = sorted(BINS)
    roots_b = {}
    hub_file = PLUG + "/🪝️mounts/🧵️wp7-instance-tier2/🦀️.rs"
    hub_units = {}
    providers = {"tests-01": ["retained_command_test_hosts"], "tests-02": ["child_test_hosts", "publication_mounted_test_hosts"], "tests-03": ["reactor_core_test_hosts"], "tests-04": ["testing_laws_unit_1"], "tests-05": ["plugin_runtime_plugin_builder_contract_support_1"], "tests-06": ["tool_run_support_1", "time_travel_support_1"]}
    extra_edits = {H3: drop_moved_uses, H2: drop_moved_uses}


if __name__ == "__main__":
    G.main(S)
