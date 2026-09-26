#!/usr/bin/env python3
"""DB1: fold a macOS `sample` call graph into `frame;frame;... count` lines (self samples per leaf path).
usage: sample-fold.py <sample.txt> [thread-substring] > folded.txt"""
import re, sys
path = sys.argv[1]
want = sys.argv[2] if len(sys.argv) > 2 else None
line_re = re.compile(r'^(?P<indent>[ +!:|]*)(?P<count>\d+) (?P<frame>.*)$')
def short(frame):
    frame = re.sub(r'\s+\(in [^)]*\).*$', '', frame).strip()
    frame = re.sub(r'\s+\+ \d+.*$', '', frame)
    return frame
out = {}
stack = []
in_graph = False
thread_ok = True
with open(path, encoding='utf-8', errors='replace') as f:
    for raw in f:
        raw = raw.rstrip('\n')
        if raw.startswith('Call graph:'):
            in_graph = True; continue
        if not in_graph:
            continue
        if raw.startswith('Total number in stack') or raw.startswith('Sort by top'):
            break
        m = line_re.match(raw)
        if not m:
            continue
        depth = len(m.group('indent'))
        count = int(m.group('count'))
        frame = short(m.group('frame'))
        if frame.startswith('Thread_'):
            thread_ok = want is None or want in frame
        while stack and stack[-1][0] >= depth:
            stack.pop()
        stack.append((depth, frame, count))
        # self samples = count minus children (computed later)
        if thread_ok:
            key = tuple(s[1] for s in stack)
            out[key] = out.get(key, 0) + count
# convert inclusive to self
selfc = dict(out)
for key, count in out.items():
    if len(key) > 1:
        parent = key[:-1]
        if parent in selfc:
            selfc[parent] -= count
for key, count in selfc.items():
    if count > 0:
        print(';'.join(key), count)
