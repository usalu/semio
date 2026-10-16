"""Names the baseline cannot resolve (candidates for glob_names / traits / seeds). usage: missing.py <cid>"""
import re, sys, os, json, collections
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
import union
cid = sys.argv[1]
c = union.cfg_of(cid)
A = union.build_baseline(cid)
e = union.diags(union.run_rustc(A, c["p_lib"], "crate_a", ["--test"], c.get("base_features", [])), A)
names = collections.defaultdict(set)
for d in e:
    m = re.match(r"cannot find (trait|type|value|function|struct, variant or union type|tuple struct or tuple variant|function, tuple struct or tuple variant|derive macro|macro|attribute|macro `?\w*`?) `(\w+)`", d["message"])
    if m:
        names[m.group(1)].add(m.group(2))
    m = re.match(r"(?:unresolved imports? |)`?(\w+)`? ?", d["message"])
    mi = re.match(r"unresolved imports? (.*)", d["message"])
    if mi:
        for n in re.findall(r"`(?:[\w$]+::)*(\w+)`", mi.group(1)):
            names["unresolved-import"].add(n)
    m2 = re.match(r"expected trait, found struct `(?:[\w$]+::)*(\w+)`", d["message"])
    if m2:
        names["E0404-struct-should-be-trait"].add(m2.group(1))
for k, v in names.items():
    print(k, sorted(v)[:60])
print("total errors", len(e))
