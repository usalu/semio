#!/usr/bin/env python3
"""📐️ WG11 item 3: heaviest static stack path of an aarch64 Mach-O test binary from a root symbol (no debugger).

Reads `objdump -d --no-show-raw-insn -C <bin>` on stdin; per function: own frame (prologue `sub sp`, inline probe `sub x9`,
`___chkstk_darwin` x15×16 — as wg11-stack-frames.sh) and its `bl` callees. Prints, for every function whose name matches the
root regex, the deepest cumulative path (cycles cut), frame by frame.
usage: objdump … | python3 wg11-stack-paths.py build <graph.pickle>
       python3 wg11-stack-paths.py query <graph.pickle> <root regex> [top N paths] [min frame bytes to print]
"""
import pickle
import re
import sys
import threading
from functools import lru_cache

mode, graph_path = sys.argv[1], sys.argv[2]
sys.setrecursionlimit(1_000_000)

header = re.compile(r"^([0-9a-f]+) <(.*)>:$")
call = re.compile(r"\sbl\s+0x[0-9a-f]+ <(.*)>$")
immediate = re.compile(r"#(0x[0-9a-f]+|\d+)")

frames, callees = {}, {}
name, size, x15, seen, probe, probed = None, 0, 0, 0, 0, 0


def value(text):
    match = immediate.search(text)
    return int(match.group(1), 0) if match else 0


def flush():
    if name is not None:
        frames[name] = max(frames.get(name, 0), size)


for raw in sys.stdin if mode == "build" else ():
    line = raw.rstrip("\n")
    head = header.match(line)
    if head:
        flush()
        name, size, x15, seen, probe, probed = head.group(2), 0, 0, 0, 0, 0
        callees.setdefault(name, set())
        continue
    if name is None:
        continue
    target = call.search(line)
    if target:
        callees[name].add(re.sub(r"\+0x[0-9a-f]+$", "", target.group(1)))
    seen += 1
    if seen > 40:
        continue
    if re.search(r"mov\s+x15, #", line):
        x15 = value(line.split("x15,", 1)[1])
    elif re.search(r"movk\s+x15, #", line):
        v = value(line.split("x15,", 1)[1])
        x15 += v * (65536 if "lsl #16" in line else 4294967296 if "lsl #32" in line else 1)
    elif "___chkstk_darwin" in line:
        size = max(size, x15 * 16)
    elif re.search(r"sub\s+sp, sp, #", line) and probe == 0:
        v = value(line.split("sp, sp,", 1)[1])
        size += v * 4096 if "lsl #12" in line else v
    elif re.search(r"sub\s+sp, sp, #", line) and probe == 1 and "lsl #12" not in line:
        size = probed + value(line.split("sp, sp,", 1)[1])
        probe = 2
    elif re.search(r"sub\s+x9, (sp|x9), #", line):
        v = value(line[line.index("#"):])
        probed += v * 4096 if "lsl #12" in line else v
        probe = 1
        size = max(size, probed)
flush()
if mode == "build":
    with open(graph_path, "wb") as sink:
        pickle.dump((frames, callees), sink)
    print(f"graph functions={len(frames)} saved {graph_path}")
    sys.exit(0)
with open(graph_path, "rb") as source:
    frames, callees = pickle.load(source)
root_pattern = re.compile(sys.argv[3])
top = int(sys.argv[4]) if len(sys.argv) > 4 else 3
threshold = int(sys.argv[5]) if len(sys.argv) > 5 else 16384
on_path = set()


@lru_cache(maxsize=None)
def heaviest(function):
    on_path.add(function)
    best, best_path = 0, ()
    for child in callees.get(function, ()):
        if child in on_path or child not in frames:
            continue
        depth, path = heaviest(child)
        if depth > best:
            best, best_path = depth, path
    on_path.discard(function)
    return frames.get(function, 0) + best, (function,) + best_path


def report():
    roots = sorted((heaviest(function) for function in frames if root_pattern.search(function)), key=lambda item: -item[0])
    print(f"functions={len(frames)} roots={len(roots)}")
    for depth, path in roots[:top]:
        print(f"\n== {depth} bytes ({depth / 1048576:.2f} MiB) from {path[0]}")
        running = 0
        for function in path:
            running += frames[function]
            if frames[function] >= threshold:
                print(f"{frames[function]:>9} {running:>10}  {function}")


threading.stack_size(1 << 29)
worker = threading.Thread(target=report)
worker.start()
worker.join()
