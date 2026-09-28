#!/usr/bin/env python3
"""🌳 H14 14c one-off: folds a macOS `sample` call graph into stacks, drops idle leaves (cvwait/kevent/park), keeps only our
crates' frames (db_*, os_hub, semio_hub, semio_framework_*, pack, protocol, zopfli, sha2/blake3) and prints the heaviest folded
stacks by self samples. usage: h14-sample-tree.py <sample.txt> [top=25]"""
import re, sys
from collections import Counter
path = sys.argv[1]; top = int(sys.argv[2]) if len(sys.argv) > 2 else 25
line_re = re.compile(r"^(?P<indent>[ +!:|]*)(?P<count>\d+) (?P<frame>.*)$")
ours = re.compile(r"(db_[a-z_]+|os_hub|semio_hub|semio_framework_[a-z_]+|4pack|8protocol|zopfli|sha2|blake3|serde_json)")
idle = re.compile(r"(__psynch_cvwait|kevent|__semwait|mach_msg|__workq_kernreturn|__psynch_mutexwait|nanosleep|__select)")
def short(frame):
    name = frame.split("  (in ")[0]
    name = re.sub(r"Cs[0-9A-Za-z_]{8,14}_", "", name)
    parts = re.findall(r"\d+([A-Za-z_][A-Za-z0-9_]*)", name)
    return "::".join(p for p in parts if len(p) > 1 and not p.startswith("B"))[-90:] or name[:90]
stack = []; folded = Counter(); leaf_self = Counter(); prev = None
entries = []
for raw in open(path, encoding="utf-8", errors="replace"):
    if raw.startswith("Total number in stack"): break
    m = line_re.match(raw.rstrip("\n"))
    if not m: continue
    entries.append((len(m.group("indent")), int(m.group("count")), m.group("frame")))
for index, (depth, count, frame) in enumerate(entries):
    while stack and stack[-1][0] >= depth: stack.pop()
    stack.append((depth, frame))
    children = 0
    for later in entries[index + 1:]:
        if later[0] <= depth: break
        if later[0] == min(e[0] for e in entries[index + 1:index + 2]) : pass
    nxt = entries[index + 1] if index + 1 < len(entries) else None
    child_sum = 0
    if nxt and nxt[0] > depth:
        child_depth = nxt[0]
        for later in entries[index + 1:]:
            if later[0] <= depth: break
            if later[0] == child_depth: child_sum += later[1]
    own = count - child_sum
    if own <= 0 or idle.search(frame) or frame.startswith("Thread_"): continue
    key = " > ".join(short(f) for _, f in stack if ours.search(f))
    folded[key or "(runtime/libc) " + short(frame)] += own
    leaf_self[short(frame)] += own
total = sum(folded.values())
print(f"busy samples {total}")
for key, value in folded.most_common(top): print(f"{value:6d} {key[-400:]}")
print("-- leaves")
for key, value in leaf_self.most_common(12): print(f"{value:6d} {key}")
