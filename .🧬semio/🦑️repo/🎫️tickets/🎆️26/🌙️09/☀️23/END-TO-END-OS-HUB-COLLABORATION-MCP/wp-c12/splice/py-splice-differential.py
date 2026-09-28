"""🐍️ C12 14c (usage: python3 py-splice-differential.py [<random vectors from `bun random-vectors.ts`>]): the prepared Python second implementation of the splice (from c12-splice-patch.py's 🐍️.py hunk) replays every
application + concurrent vector of the language-neutral text-splice fixture and must agree with the TS reference's answers."""
import json, sys
src = open("patch/c12-splice-patch.py", encoding="utf-8").read().rsplit("\nmain()", 1)[0]
scope = {"__file__": "patch/c12-splice-patch.py"}
exec(src, scope)
block = next(new for path, old, new in scope["HUNKS"] if path.endswith("🐍️.py") and "def splice_locate" in new)
code = block.split("# region 🔖️Splice\n", 1)[1].split("# endregion 🔖️Splice", 1)[0]
module = {}
exec(code, module)
fixture = json.load(open("patch/tree/🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/✂️text-splice/🔣️.json", encoding="utf-8"))
failures = 0
for row in fixture["applications"]:
    at, length, clamped = module["splice_locate"](row["text"], row["splice"])
    result, inverse = module["splice_apply"](row["text"], row["splice"])
    got = ({"start": at, "deleteLength": length, "clamped": clamped}, result, inverse)
    want = (row["located"], row["result"], row["inverse"])
    if got != want:
        failures += 1
        print("FAIL application", row["id"], got, want)
for row in fixture["concurrent"]:
    text = row["base"]
    for entry in row["order"]:
        text = module["splice_apply"](text, entry["splice"])[0]
    if text != row["expected"]["text"]:
        failures += 1
        print("FAIL concurrent", row["id"], repr(text), repr(row["expected"]["text"]))
for row in fixture["rebases"]:
    text = row["remote"]
    for splice in row["unapplied"]:
        text = module["splice_apply"](text, splice)[0]
    if text != row["expected"]["text"]:
        failures += 1
        print("FAIL rebase", row["id"])
brief = "# Mission Brief\n\nHold the current draft.\n"
vector = {"start": 17, "deleted": "Draft: ", "insert": "", "before": "# Mission Brief\n\n", "after": "Hold the current draft.\n"}
print("no-op vector:", module["splice_locate"](brief, vector), module["splice_apply"](brief, vector)[0] == brief)
print("applications %d, concurrent %d, rebases %d, failures %d" % (len(fixture["applications"]), len(fixture["concurrent"]), len(fixture["rebases"]), failures))
if len(sys.argv) > 1:
    rows = json.load(open(sys.argv[1], encoding="utf-8"))
    disagreements = 0
    for row in rows:
        at, length, clamped = module["splice_locate"](row["target"], row["splice"])
        text, inverse = module["splice_apply"](row["target"], row["splice"])
        disagreements += ({"start": at, "deleteLength": length, "clamped": clamped}, text, inverse) != (row["located"], row["text"], row["inverse"])
    print("random vectors %d (clamped %d): python/TS disagreements %d" % (len(rows), sum(row["located"]["clamped"] for row in rows), disagreements))
    failures += disagreements
sys.exit(1 if failures else 0)
