import os, sys, json, hashlib, difflib
sys.path.insert(0, "/private/tmp/oo")
import ops_code, ops_tests

ROOT = "/Users/ueli/Documents/semio"
E = lambda code: chr(code) + "️"
SHELL = os.path.join(E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F4BB) + "os", E(0x1F528) + "modules", E(0x1F5A5) + "shell", E(0x1F4E6) + "packages", E(0x1F980) + "rust", E(0x1F4CB) + "project.json")

META = '      "metadata": {\n        "semio": {\n          "sourceInputObservation": "freshness"\n        }\n      },\n'
HEAD = '    "%s": {\n      "cache": true,\n      "executor": "repo:script",\n'


def manifest(text):
    for name in ("check", "test", "preview-generated"):
        head = HEAD % name
        if text.count(head) != 1:
            raise SystemExit("manifest anchor %s: %d" % (name, text.count(head)))
        text = text.replace(head, head + META)
    head = HEAD % "schema-check"
    if text.count(head) != 1:
        raise SystemExit("manifest anchor schema-check")
    return text.replace(head, head + META + '      "dependsOn": [\n        "typegen"\n      ],\n')


FILES = [(SHELL, manifest)]


def hunks(old, new):
    a, b = old.split("\n"), new.split("\n")
    matcher = difflib.SequenceMatcher(None, a, b, autojunk=False)
    rows = []
    for tag, i1, i2, j1, j2 in matcher.get_opcodes():
        if tag == "equal":
            continue
        before, after = i1, i2
        lo, hi = i1, i2
        while True:
            chunk = "\n".join(a[lo:hi])
            if chunk and old.count(chunk) == 1:
                break
            if lo > 0:
                lo -= 1
            elif hi < len(a):
                hi += 1
            else:
                raise SystemExit("no unique anchor")
        pre = b[j1 - (i1 - lo):j1] if i1 - lo >= 0 else []
        new_chunk = "\n".join(a[lo:i1] + b[j1:j2] + a[i2:hi])
        rows.append({"old": chunk, "new": new_chunk})
    return rows


def main(out):
    ops, report = [], []
    for rel, fn in FILES:
        path = os.path.join(ROOT, rel)
        old = open(path, encoding="utf8").read()
        new = fn(old)
        rows = hunks(old, new)
        check = old
        for row in rows:
            assert check.count(row["old"]) == 1, "anchor not unique after earlier hunks: " + rel
            check = check.replace(row["old"], row["new"])
        assert check == new, "hunk replay differs: " + rel
        ops.append({"path": rel.replace(os.sep, "/"), "preimageSha256": hashlib.sha256(old.encode()).hexdigest(), "resultSha256": hashlib.sha256(new.encode()).hexdigest(), "hunks": rows})
        report.append((rel, len(old), len(new), len(rows)))
    json.dump({"version": 1, "ops": ops}, open(out, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    for row in report:
        print(row[1], "->", row[2], "hunks", row[3], os.path.basename(row[0]))


if __name__ == "__main__":
    main(sys.argv[1])
