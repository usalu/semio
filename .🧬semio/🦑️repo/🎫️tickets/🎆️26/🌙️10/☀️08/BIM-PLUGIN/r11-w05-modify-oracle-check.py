import importlib.util, os, re, sys
sys.stdout.reconfigure(encoding="utf-8")
B = "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any"
spec = importlib.util.spec_from_file_location("oracle", B + "/🧪️tests/🏙️mutate-model-1-any/🐍️.py")
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
root = B + "/🧫️fixtures/🧬️mutations"
mine = {"copy-elements", "mirror-elements", "array-elements", "align-elements", "offset-wall", "trim-extend-wall", "split-slab", "split-beam", "set-wall-end-join", "delete-elements", "delete-site", "delete-building", "delete-storey"}
checked = 0; bad = 0
for leaf in sorted(os.listdir(root)):
    kind = re.sub(r"^[^a-z0-9]+", "", leaf)
    if kind not in mine: continue
    for scenario in sorted(os.listdir(os.path.join(root, leaf))):
        before = os.path.join(root, leaf, scenario, *m.BEFORE)
        if not os.path.exists(before): continue
        try:
            row = m.row_at(before)
            problems = m.problems_of(row) + m.payload_problems(row, kind)
        except Exception as error:
            problems = ["exception: %s" % error]
        checked += 1
        for p in problems:
            bad += 1; print("[FAIL] %s/%s: %s" % (leaf, scenario, p))
print("checked %d quintets of the modify and delete kinds, %d problem(s)" % (checked, bad))
