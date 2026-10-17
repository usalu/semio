#!/usr/bin/env python3
"""🧱️ Splits the facade test hosts `job` (unit-3: the tool-run pieces) and `testing-laws` (unit-1) into one host file per group of mounted test modules, so that each host is its own test root; usage: split_hosts_w7.py [--apply]"""
import re, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
import ed
H3 = PLUG + "/🪝️mounts/👥️wp3-owners"
H7 = PLUG + "/🪝️mounts/🧵️wp7-instance-tier2"
MOUNT_RX = re.compile(r'(?m)^#\[cfg\(test\)\]\n#\[path = "([^"]+)"\]\n((?:pub(?:\([a-z]+\))? )?mod ([A-Za-z0-9_]+);)\n')
edits = {}      # path -> text
deletes = []


def unit_split(text):
    ms = list(MOUNT_RX.finditer(text))
    header = text[:ms[0].start()]
    return header, {m.group(3): m.group(0) for m in ms}


# --- job: unit-3 carries the two tool-run pieces ------------------------------------------------------------------
u3 = H3 + "/🧪️test-hosts/👷️job/🧱️unit-3/🦀️.rs"
t = read(u3)
header, mounts = unit_split(t)
header = header.replace("#[cfg(test)]\nuse crate::app_testing::mutation_fixture;\n", "")
assert "mutation_fixture" not in header
names = sorted(mounts)
assert names == ["tool_run_tests_1", "tool_run_tests_2"], names
u5 = H3 + "/🪝️mounts-placeholder"
edits[u3] = header + mounts["tool_run_tests_1"]
u5 = H3 + "/🧪️test-hosts/👷️job/🧱️unit-5/🦀️.rs"
edits[u5] = header.replace("Unit 3 of the facade test host of `job`", "Unit 5 of the facade test host of `job`") + mounts["tool_run_tests_2"]
jobhost = H3 + "/🧪️test-hosts/👷️job/🦀️.rs"
jt = read(jobhost)
old = '#[cfg(test)]\n#[path = "🧱️unit-3/🦀️.rs"]\npub(crate) mod job_unit_3;\n'
assert jt.count(old) == 1
edits[jobhost] = jt.replace(old, "")
hub3 = H3 + "/🦀️.rs"
h3 = read(hub3)
old = '#[cfg(test)]\n#[path = "🧪️test-hosts/👷️job/🦀️.rs"]\npub(crate) mod job_test_hosts;\n'
assert h3.count(old) == 1
edits[hub3] = h3.replace(old, old + '#[cfg(test)]\n#[path = "🧪️test-hosts/👷️job/🧱️unit-3/🦀️.rs"]\npub(crate) mod job_tool_run_host_1;\n#[cfg(test)]\n#[path = "🧪️test-hosts/👷️job/🧱️unit-5/🦀️.rs"]\npub(crate) mod job_tool_run_host_2;\n')

# --- testing-laws: unit-1 -> four units ---------------------------------------------------------------------------------
u1 = H7 + "/🧪️test-hosts/⚖️testing-laws/🧱️unit-1/🦀️.rs"
t = read(u1)
header, mounts = unit_split(t)
groups = [
    ["time_travel_tests_1"],
    ["time_travel_tests_2"],
    ["time_travel_tests_3", "mutation_fixture", "empty_state_contract", "bounded_reload_tests"],
    ["publication_retirement_authority", "supersede_ledger_tests", "history_alternatives_tests", "history_label_reload_tests", "composed_child_history_tests", "node_graph_delete_row_tests"],
]
assert sorted(sum(groups, [])) == sorted(mounts), (sorted(mounts), sorted(sum(groups, [])))
mod_lines = []
for i, g in enumerate(groups, 1):
    p = H7 + f"/🧪️test-hosts/⚖️testing-laws/🧱️unit-{i}/🦀️.rs"
    body = header if i == 1 else header.replace("Unit 1 of the facade test host of `testing-laws`", f"Unit {i} of the facade test host of `testing-laws`")
    edits[p] = body + "".join(mounts[m] for m in g)
    mod_lines.append(f'#[cfg(test)]\n#[path = "🧪️test-hosts/⚖️testing-laws/🧱️unit-{i}/🦀️.rs"]\npub(crate) mod testing_laws_unit_{i};\n')
hub7 = H7 + "/🦀️.rs"
h7 = read(hub7)
old = '#[cfg(test)]\n#[path = "🧪️test-hosts/⚖️testing-laws/🦀️.rs"]\npub(crate) mod testing_laws_test_hosts;\n'
assert h7.count(old) == 1
h7 = h7.replace(old, "".join(mod_lines))
old2 = "    #[cfg(test)]\n    pub(crate) use super::testing_laws_test_hosts::testing_laws_unit_1::mutation_fixture;\n"
assert h7.count(old2) == 1
h7 = h7.replace(old2, "")
edits[hub7] = h7
deletes.append(H7 + "/🧪️test-hosts/⚖️testing-laws/🦀️.rs")
for p, tx in edits.items():
    print(rel(p)[-75:], len(tx))
if "--apply" in sys.argv:
    for p, tx in edits.items():
        ed.put(p, tx)
    for p in deletes:
        ed.delete(p)
    print("applied")
