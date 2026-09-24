"""🧮️ Classifies every print-subject TeX failure of the last parity run by the first error line of its logs."""
import pathlib, re, collections
work = pathlib.Path("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tests/work")
causes = collections.Counter(); per = collections.defaultdict(set)
for d in work.glob("test-framework-products-print-*-subject-typescript"):
    case = d.name.split("-print-")[1].split("-", 1)[1].rsplit("-subject", 1)[0]
    for log in d.rglob("*.log"):
        t = log.read_text(errors="replace")
        m = re.search(r"^! (.*)$", t, re.M)
        if not m: continue
        line = m.group(1)
        if "patterns" in t and "I did not find the tikz library" in t: cause = "tikz library patterns (awaiting approval)"
        else: cause = line[:110]
        causes[cause] += 1; per[cause].add(case)
for c, n in causes.most_common(): print(n, c, "|", ", ".join(sorted(per[c])))
