"""🧪️ C5F probe: Python emulation of readModuleTreeSource (parent first, then every `#[path]` child in declaration order, depth first, canonical `🧪️tests` mounts excluded) and the source-text assertions of the three re-pointed package 5 readers. Used because the workspace packages that the TS reader imports are not linked at this moment."""
import os, re
PL = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/"
RX = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*;')
def tree(path, seen=None):
    t = open(path, encoding="utf-8").read()
    out = [t if t.endswith("\n") else t + "\n"]
    for m in RX.finditer(t):
        rel = m.group(1)
        if "🧪️tests" in rel.split("/"):
            continue
        out.append(tree(os.path.normpath(os.path.join(os.path.dirname(path), rel))))
    return "".join(out)
rt = tree(PL + "🧠️interpreter/🏃️runtime/🦀️.rs")
bad = 0
def check(n, ok):
    global bad
    print("ok  " if ok else "FAIL", n)
    bad += 0 if ok else 1
check("admission test: OriginalActorFixedReceipt::read", "OriginalActorFixedReceipt::read" in rt)
check("host script: typed decoder", "result.map_err(|error| decode_guest_fault_bytes(&error))" in rt)
a, b = rt.find("pub fn resume_snapshot("), rt.find("/// 🪶️ The six owned")
check("microsecond-clock: anchors ordered", 0 <= a < b)
resume = rt[a:b]
check("microsecond-clock: deadline_us: u64", "deadline_us: u64" in resume)
check("microsecond-clock: fuel, deadline_us, OwnedDeadline::Original", "fuel, deadline_us, OwnedDeadline::Original" in resume)
check("microsecond-clock: banned patterns absent", not re.search(r"NoFuelProgress|deadline_ms|\*1000", resume))
print("resume slice chars", len(resume), "tree chars", len(rt), "bad", bad)
