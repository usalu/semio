import sys, os, re, json
import xml.etree.ElementTree as ET
out = sys.argv[1]
rows = []
for line in open(os.path.join(out, "summary.txt"), encoding="utf8"):
    code, f, key, p, fl = line.rstrip("\n").split("|")
    rows.append((code, f, key, p, fl))
fails = set(); passes = set(); filestat = {}
for code, f, key, p, fl in rows:
    ff = set(); pp = set(); err = ""
    xmlp = os.path.join(out, key + ".xml")
    try:
        root = ET.parse(xmlp).getroot()
        def walk(node, trail):
            for child in node:
                if child.tag == "testsuite":
                    walk(child, trail + [child.get("name", "")])
                elif child.tag == "testcase":
                    name = " > ".join(t for t in trail[1:] if t) + (" > " if len(trail) > 1 else "") + child.get("name", "")
                    bad = any(c.tag in ("failure", "error") for c in child)
                    (ff if bad else pp).add(name)
        walk(root, [])
    except Exception as e:
        err = "no-junit:" + str(e)[:60]
    if code != "0" and not ff:
        try:
            t = re.sub(r"\x1b\[[0-9;]*m", "", open(os.path.join(out, key + ".log"), encoding="utf8", errors="replace").read())
        except Exception:
            t = ""
        m = re.search(r"(error: .*|Cannot find module .*|SyntaxError.*|TypeError.*|ReferenceError.*)", t)
        err = (err + " " + (m.group(1)[:200] if m else "unknown")).strip()
        ff.add("<file-level> " + (m.group(1)[:120] if m else "unknown"))
    for n in ff: fails.add(f + "::" + n)
    for n in pp: passes.add(f + "::" + n)
    filestat[f] = {"code": code, "pass": p, "fail": fl, "fileerror": err}
json.dump({"fails": sorted(fails), "passes": sorted(passes), "files": filestat}, open(sys.argv[2], "w"), ensure_ascii=False, indent=0)
print(len(rows), "files;", len(passes), "passing tests;", len(fails), "failing tests;", sum(1 for v in filestat.values() if v["fileerror"]), "files with file-level error")
