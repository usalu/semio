"""v13 step 2: adds the stiffness basis columns to the four en1998 DSL assets (idempotent, python -I)."""
import glob, re

ART = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998"
for p in glob.glob(ART + "/**/🗣️.dsl.semio", recursive=True):
    t = open(p, encoding="utf-8").read()
    if "storey-stiffness-basis" in t:
        continue
    t = t.replace(" storey-stiffness-ratio:NUM]", " storey-stiffness-basis:ENUM storey-stiffness-ratio:NUM]")
    out, rows = [], 0
    for l in t.split("\n"):
        l = re.sub(r"effective-stiffness-ratio=([0-9.]+)", lambda m: ("effective-stiffness-basis=declared effective-stiffness-ratio=" + m.group(1)) if float(m.group(1)) else "effective-stiffness-basis=simplified effective-stiffness-ratio=1", l)
        m = re.match(r"^(.*\} \] (?:true|false) (?:true|false) .*) ([0-9.]+)$", l)
        if m:
            rows += 1
            l = m.group(1) + (" declared " + m.group(2) if float(m.group(2)) else " simplified 1")
        out.append(l)
    print(p[-60:], rows)
    open(p, "w", encoding="utf-8").write("\n".join(out))
