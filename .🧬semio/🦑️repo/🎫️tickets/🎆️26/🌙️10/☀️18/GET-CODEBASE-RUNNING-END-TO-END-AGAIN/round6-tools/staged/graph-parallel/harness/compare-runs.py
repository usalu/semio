#!/usr/bin/env python3
"""🧪 Compares two harness runs of the same sample: the project values byte for byte, the facts consumed over all projects as sets, and how many projects attribute the same facts to themselves.
usage: compare-runs.py TAG_A TAG_B [DIRECTORY]   (reads DIRECTORY/TAG.projects.json and TAG.rows.json written by nodes-run.mjs; default /private/tmp/graph-parallel)
Exit 0 when the project values are byte-identical and the unions of facts are equal. Stdlib only."""
import json, pathlib, sys

a, b = sys.argv[1], sys.argv[2]
directory = pathlib.Path(sys.argv[3] if len(sys.argv) > 3 else "/private/tmp/graph-parallel")
pa, pb = (directory / (a + ".projects.json")).read_bytes(), (directory / (b + ".projects.json")).read_bytes()
values_equal = pa == pb
ra, rb = json.loads((directory / (a + ".rows.json")).read_text(encoding="utf8")), json.loads((directory / (b + ".rows.json")).read_text(encoding="utf8"))
union_a, union_b = set().union(*map(set, ra.values())), set().union(*map(set, rb.values()))
same_owners = set(ra) == set(rb)
identical = sum(1 for k in ra if k in rb and set(ra[k]) == set(rb[k]))
print(json.dumps({"projects": len(json.loads(pa)), "valuesByteIdentical": values_equal, "ownersEqual": same_owners, "unionA": len(union_a), "unionB": len(union_b), "unionEqual": union_a == union_b, "onlyA": len(union_a - union_b), "onlyB": len(union_b - union_a), "ownersWithIdenticalFacts": identical, "owners": len(ra)}))
raise SystemExit(0 if values_equal and union_a == union_b and same_owners else 1)
