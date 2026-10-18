"""🧾 C5D resolve-row name audit: the unresolved names that are new in the post pos log (resolve section of cfg lib/test) must all be names that no file of the five crates defines (external names behind the root's globs and aliases); a crate-local name would be a real regression. usage: namecheck.py pre.log post.log"""
import re, sys, os, glob
R = "/Users/ueli/Documents/semio/"
PL = R + "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/"
def names(path):
    out = {}
    cfg = None
    sec = None
    for l in open(path, encoding="utf-8"):
        m = re.match(r"-- cfg (\w+):", l)
        if m: cfg = m.group(1)
        if re.match(r"   (privacy|trait-scope|resolve|type|other):", l):
            sec = l.split(":")[0].strip()
            continue
        if sec == "resolve" and re.match(r"      E0\d+ \[", l):
            for n in re.findall(r"cannot find (?:[a-z, ]+ )?`([^`]+)`", l) + re.findall(r"unresolved import `([^`]+)`", l):
                out.setdefault(cfg, set()).add(n.split("::")[-1])
    return out
pre, post = names(sys.argv[1]), names(sys.argv[2])
files = []
for d in ["⏪️time-travel", "⏯️tool-run", "⚛️reactor/🔄️turn", "⏪️time-travel/🧵️instance", "⏯️tool-run/🧵️instance"]:
    top = PL + d
    for dp, dn, fn in os.walk(top):
        dn[:] = [x for x in dn if x != "📦️packages" and not (os.path.isdir(os.path.join(dp, x, "📦️packages")) and os.path.join(dp, x) != top)]
        for f in fn:
            if f.endswith(".rs") and "/📦️packages/" not in dp:
                files.append(os.path.join(dp, f))
text = "\n".join(open(f, encoding="utf-8", errors="ignore").read() for f in files)
rx = lambda n: re.compile(r"(?:^|\s)(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?(?:const\s+|static\s+)?(?:fn|struct|enum|trait|type|const|static|mod|union|macro_rules!)\s+" + re.escape(n) + r"\b|static\s+" + re.escape(n) + r"\b", re.M)
bad = 0
for cfg in post:
    new = post[cfg] - pre.get(cfg, set())
    local = sorted(n for n in new if rx(n).search(text))
    print(f"{cfg}: unresolved names pre {len(pre.get(cfg, ()))} post {len(post[cfg])} new {len(new)}; new names defined inside the crates: {local}")
    bad += len(local)
print("NAMECHECK", "no crate-local name among the new unresolved names" if not bad else "LOCAL NAMES PRESENT")
