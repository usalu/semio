import os, sys
OV = os.environ.get("LD_OV", "/private/tmp/claude-501/ld")
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"


def sub(rel, old, new, count=1, ov=OV):
    path = os.path.join(ov, rel)
    if os.path.islink(path):
        raise SystemExit("not an overlay copy: " + rel)
    text = open(path, encoding="utf-8").read()
    have = text.count(old)
    if have != count:
        raise SystemExit("anchor count %d != %d in %s: %r" % (have, count, rel, old[:100]))
    open(path, "w", encoding="utf-8").write(text.replace(old, new))
    import json
    with open(ov + ".edits.jsonl", "a", encoding="utf-8") as log:
        log.write(json.dumps({"rel": rel, "old": old, "new": new, "count": count}, ensure_ascii=False) + "\n")


def replay(ov=OV):
    import json
    log = ov + ".edits.jsonl"
    if not os.path.exists(log):
        return
    rows = [json.loads(line) for line in open(log, encoding="utf-8")]
    os.remove(log)
    for row in rows:
        text = open(os.path.join(ov, row["rel"]), encoding="utf-8").read()
        if text.count(row["old"]) != row["count"] and row["new"] and text.count(row["new"]) >= row["count"]:
            print("already applied live:", row["rel"][-70:])
            with open(ov + ".edits.jsonl", "a", encoding="utf-8") as handle:
                handle.write(json.dumps(row, ensure_ascii=False) + "\n")
            continue
        sub(row["rel"], row["old"], row["new"], row["count"], ov)


def reset(rel, ov=OV):
    import json, shutil
    log = ov + ".edits.jsonl"
    rows = [json.loads(line) for line in open(log, encoding="utf-8")] if os.path.exists(log) else []
    rows = [row for row in rows if row["rel"] != rel]
    shutil.copy2(os.path.join("/Users/ueli/Documents/semio", rel), os.path.join(ov, rel))
    with open(log, "w", encoding="utf-8") as handle:
        for row in rows:
            handle.write(json.dumps(row, ensure_ascii=False) + "\n")


def write(rel, text, ov=OV):
    import json
    path = os.path.join(ov, rel)
    old = open(path, encoding="utf-8").read()
    reset(rel, ov)
    live = open(path, encoding="utf-8").read()
    open(path, "w", encoding="utf-8").write(text)
    with open(ov + ".edits.jsonl", "a", encoding="utf-8") as log:
        log.write(json.dumps({"rel": rel, "old": live, "new": text, "count": 1}, ensure_ascii=False) + "\n")


DIRECT = 'import { runScriptProgram } from "./🟦️.ts";\n\nif (import.meta.main) await runScriptProgram();\n'


def direct(rel):
    write(rel, DIRECT)
