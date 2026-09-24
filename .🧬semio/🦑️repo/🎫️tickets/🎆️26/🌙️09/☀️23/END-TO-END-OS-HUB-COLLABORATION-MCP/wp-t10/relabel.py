"""🏷️ Relabels committed vectors whose outcome is a warned `mutation.no-op` from `applied` to the protocol class `no-op`,
and updates each scenario test that asserts the declared status. `--dry` reports only; writes a review list of every
touched test file to wp-t10/generated/relabel-review.txt."""
import json, os, re, subprocess, sys
root = "/Users/ueli/Documents/semio/"
dry = "--dry" in sys.argv
rel = json.load(open(root + ".tmp-ticket/wp-t10/generated/vector-census.json"))["relabel"]
touched, review, problems = [], [], []
MATCH_OLD = re.compile(r'(\n(\s*)"applied" => assert!\(applied, "([^"]+): declared applied but the mutation was rejected"\),\n)')
for r in rel:
    path = root + r
    text = open(path, encoding="utf-8").read()
    new = text.replace('"status": "applied"', '"status": "no-op"', 1)
    if new == text: problems.append(f"{r}: no applied status"); continue
    touched.append(r)
    if not dry: open(path, "w", encoding="utf-8").write(new)
    scen = r.split("/")[:-2]
    needle = f"{scen[-2]}/{scen[-1]}/🎯️outcome"
    tests = subprocess.run(["/usr/bin/grep", "-rl", "--include=*.rs", needle, root + "/".join(r.split("/")[:3])], capture_output=True, text=True).stdout.split()
    for t in tests:
        src = open(t, encoding="utf-8").read()
        out = src
        if src.count("🎯️outcome/🔣️.json") == 1 and src.count('Some("applied")') == 1:
            out = out.replace('Some("applied")', 'Some("no-op")')
            out = re.sub(r"declares an applied outcome", "declares a no-op outcome", out)
            out = re.sub(r"this fixture declares an applied outcome", "this fixture declares a no-op outcome", out)
        m = MATCH_OLD.search(out)
        if m and src.count("🎯️outcome/🔣️.json") == 1:
            pad, label = m.group(2), m.group(3)
            out = out.replace(m.group(1), m.group(1) + f'{pad}"no-op" => {{\n{pad}    assert!(applied, "{label}: declared no-op but the mutation was rejected");\n{pad}    assert_eq!(snapshot, before(), "{label}: a no-op must leave the snapshot untouched");\n{pad}}}\n', 1)
        if out != src:
            touched.append(t[len(root):])
            if not dry: open(t, "w", encoding="utf-8").write(out)
        review.append(t[len(root):] + ("  CHANGED" if out != src else "  unchanged"))
open(root + ".tmp-ticket/wp-t10/generated/relabel-review.txt", "w").write("\n".join(review) + "\n")
print(f"{'would touch' if dry else 'touched'} {len(touched)} file(s); problems {len(problems)}")
for p in problems: print("PROBLEM", p)
