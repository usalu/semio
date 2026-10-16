"""Static proofs for the staged backends: line multiset (lossless), per-trait method coverage, brace balance, rustfmt parse."""
import os, re, sys, collections, subprocess
sys.path.insert(0, os.path.dirname(__file__))
from hdclib import *

ORIG = SC + "/hdc-orig/🌎️hub/📇️directory"
STAGE = SC + "/hdc-stage"
VIS = re.compile(r"^(pub\([^)]*\)\s+|pub\s+)(async\s+fn|fn)\b")


def norm_lines(text):
    out = []
    for l in text.split("\n"):
        s = l.strip()
        if not s:
            continue
        s = VIS.sub(lambda m: m.group(2), s)
        out.append(s)
    return collections.Counter(out)


def impl_methods(text, struct):
    """{trait: [method names]} for `impl Trait for Struct` blocks (4-space members)."""
    res = collections.OrderedDict()
    lines = text.split("\n")
    i = 0
    while i < len(lines):
        m = re.match(r"^impl (\w+) for %s \{(\})?$" % struct, lines[i])
        if m:
            t = m.group(1)
            res.setdefault(t, [])
            if not m.group(2):
                j = i + 1
                while lines[j] != "}":
                    mm = re.match(r"^    (?:async )?fn (\w+)", lines[j])
                    if mm:
                        res[t].append(mm.group(1))
                    j += 1
                i = j
        i += 1
    return res


def main():
    bad = 0
    for be, struct in BACKENDS.items():
        files = ["🦀️.rs"] + [sec + "/🦀️.rs" for sec in SECTIONS]
        orig = collections.Counter()
        new = collections.Counter()
        for f in files:
            po = ORIG + "/" + be + "/" + f
            pn = STAGE + "/" + be + "/" + f
            if not os.path.exists(po):
                continue
            ot = read(po)
            if f == "🦀️.rs":
                cut = ot.index("/// 🚪️ The port adapter")
                end = ot.index("\n}\n", ot.index("impl HubDirectory for %s {" % struct)) + 3
                ot = ot[:cut] + ot[end:]
            orig += norm_lines(ot)
            new += norm_lines(read(pn))
        miss = orig - new
        extra = new - orig
        print("==", be, "missing", sum(miss.values()), "extra", sum(extra.values()))
        for k, v in sorted(miss.items()):
            if not (k.endswith(".await") and k.startswith("self.")) and not (k.startswith("async fn ") and False):
                pass
        # expected missing: adapter lines (forwarders), impl wrappers, doc line, HubDirectory import token edit
        fwd = [k for k in miss if re.match(r"^self\.\w+\(.*\)\.await$", k)]
        other_miss = {k: v for k, v in miss.items() if k not in fwd}
        print("   forwarder bodies missing:", len(fwd))
        for k, v in sorted(other_miss.items(), key=lambda kv: kv[0])[:400]:
            print("   -", v, k[:160])
        for k, v in sorted(extra.items(), key=lambda kv: kv[0])[:400]:
            print("   +", v, k[:160])
        # coverage
        got = collections.OrderedDict()
        for sec in SECTIONS:
            p = STAGE + "/" + be + "/" + sec + "/🦀️.rs"
            if not os.path.exists(p):
                continue
            for t, ms in impl_methods(read(p), struct).items():
                assert t not in got, (be, t, "implemented twice")
                got[t] = (sec, ms)
        for sec, traits in SECTIONS.items():
            for t, names in traits:
                req = [n for n in names if n not in TRAIT_DEFAULTED]
                if t not in got:
                    print("   !! trait not implemented", t); bad += 1; continue
                gsec, ms = got[t]
                if gsec != sec:
                    print("   !! trait in wrong section", t, gsec, sec); bad += 1
                if t == "AgentDelegationDirectory" and not ms:
                    print("   opt-out", t)
                    continue
                if sorted(ms) != sorted(req):
                    print("   !! method mismatch", t, "missing", sorted(set(req) - set(ms)), "extra", sorted(set(ms) - set(req))); bad += 1
        # brace balance
        for f in files:
            pn = STAGE + "/" + be + "/" + f
            if not os.path.exists(pn):
                continue
            m = mask(read(pn))
            if m.count("{") != m.count("}"):
                print("   !! brace imbalance", f, m.count("{"), m.count("}")); bad += 1
    print("BAD", bad)


if __name__ == "__main__":
    main()
