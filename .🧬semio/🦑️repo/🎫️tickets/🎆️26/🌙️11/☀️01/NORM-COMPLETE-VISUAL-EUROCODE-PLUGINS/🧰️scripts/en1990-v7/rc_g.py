"""Task 1 layer G: every JSON fixture (text edits keep the formatting): reliabilityClass N -> "RCN", kFiDeclared dropped, newReliabilityClass N -> "RCN"."""
from lib import *
RCS = re.compile(r'"reliabilityClass":(\s*)(\d)')
NEW = re.compile(r'"newReliabilityClass":(\s*)(\d)')
KFI = re.compile(r'"kFiDeclared":\s*(?:null|-?[0-9][0-9.eE+-]*),\s*')
def cls(n):
    return "RC%s" % ("3" if n == "0" else n)
n = 0
for dp, dn, fn in os.walk(ART):
    for f in fn:
        if not f.endswith(".json"):
            continue
        p = os.path.join(dp, f)
        t = rd(p)
        if "kFiDeclared" not in t and "eliabilityClass" not in t:
            continue
        u = RCS.sub(lambda m: '"reliabilityClass":%s"%s"' % (m.group(1), cls(m.group(2))), t)
        u = NEW.sub(lambda m: '"newReliabilityClass":%s"%s"' % (m.group(1), cls(m.group(2))), u)
        u = KFI.sub("", u)
        if u != t:
            wr(p, u); n += 1
print("fixtures edited", n)
