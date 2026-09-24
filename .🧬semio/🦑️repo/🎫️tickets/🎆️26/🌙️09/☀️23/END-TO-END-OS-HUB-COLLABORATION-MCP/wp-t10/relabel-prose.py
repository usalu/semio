"""✍️ Rewrites the outcome prose of the relabelled scenario tests so it names the `no-op` class instead of `applied`."""
import re
root = "/Users/ueli/Documents/semio/"
files = [l.strip() for l in open(root + ".tmp-ticket/wp-t10/generated/relabel-tests-changed.txt") if l.strip()]
SUBS = [
    (r"The declared outcome — applied, with", "The declared outcome — `no-op`, with"),
    (r"The declared outcome — `applied` (with|carrying) one", r"The declared outcome — `no-op` \1 one"),
    (r"The declared outcome is `applied` WITH", "The declared outcome is `no-op` WITH"),
    (r"The declared outcome holds: `applied`,", "The declared outcome holds: `no-op`,"),
    (r"The declared outcome holds: an `applied` status", "The declared outcome holds: a `no-op` status"),
    (r"is `applied` with a single Warning", "is `no-op` with a single Warning"),
    (r"(?m)^(?!\s*\"applied\" =>)(.*): declared applied but the mutation was rejected", r"\1: declared no-op but the mutation was rejected"),
    (r"a no-op is applied, not rejected", "a no-op is its own outcome class, not a rejection"),
    (r"an APPLIED outcome carrying a", "a NO-OP outcome carrying a"),
    (r"APPLIED case with (an EMPTY diff|the artifact's `Default`)", r"NO-OP case with \1"),
    (r"APPLIED no-op still leaves", "accepted no-op still leaves"),
    (r"an APPLIED outcome\.", "a NO-OP outcome."),
    (r"is APPLIED (with|, not rejected)", r"is a NO-OP \1"),
    (r"a no-op is a Warning — applied, but with nothing to change", "a no-op is a Warning — never a refusal, it just changes nothing"),
    (r"a no-op is a Warning — the document is still applied, just unchanged", "a no-op is a Warning — never a refusal, the document is just unchanged"),
]
for f in files:
    t = open(root + f, encoding="utf-8").read(); n = t
    for a, b in SUBS: n = re.sub(a, b, n)
    n = n.replace("is a NO-OP , not rejected", "is a NO-OP, not rejected")
    if n != t: open(root + f, "w", encoding="utf-8").write(n)
for f in files:
    for i, line in enumerate(open(root + f, encoding="utf-8"), 1):
        if re.search(r"APPLIED|`applied`|\bapplied outcome|declared applied", line) and '"applied" =>' not in line:
            print(f"{f.split('/🧬️mutations/')[-1]}:{i}: {line.strip()[:170]}")
