#!/usr/bin/env python3
"""🔬 Probe: which text-showing operators does the bachelor thesis use, and what does a naive
Tj/TJ operand concatenation recover on each page? Mirrors the lopdf-based oracle reader."""
import re, sys, zlib

path = sys.argv[1]
data = open(path, "rb").read()
print("size", len(data), "header", data[:9])

# brute force: every "N G obj ... endobj"
objs = {}
for m in re.finditer(rb"(\d+)\s+(\d+)\s+obj", data):
    objs[int(m.group(1))] = m.end()

def read_obj(num):
    start = objs.get(num)
    if start is None:
        return None
    end = data.find(b"endobj", start)
    return data[start:end]

# find trailer Root
roots = re.findall(rb"/Root\s+(\d+)\s+(\d+)\s+R", data)
print("roots", roots[-3:])
root = int(roots[-1][0])
rb_ = read_obj(root)
print("root obj", rb_[:200])
pages_ref = re.search(rb"/Pages\s+(\d+)\s+(\d+)\s+R", rb_)
pages_num = int(pages_ref.group(1))

def kids_of(num, depth=0, seen=None):
    if seen is None: seen = set()
    if num in seen: return []
    seen.add(num)
    body = read_obj(num)
    if body is None: return []
    if b"/Kids" in body:
        ks = body[body.find(b"/Kids"):]
        ks = ks[ks.find(b"["): ks.find(b"]")+1]
        out = []
        for m in re.finditer(rb"(\d+)\s+(\d+)\s+R", ks):
            out.extend(kids_of(int(m.group(1)), depth+1, seen))
        return out
    return [num]

leaves = kids_of(pages_num)
print("page count", len(leaves))

def page_content(num):
    body = read_obj(num)
    refs = []
    m = re.search(rb"/Contents\s+(\d+)\s+\d+\s+R", body)
    if m:
        refs = [int(m.group(1))]
    else:
        m = re.search(rb"/Contents\s*\[([^\]]*)\]", body)
        if m:
            refs = [int(x.group(1)) for x in re.finditer(rb"(\d+)\s+\d+\s+R", m.group(1))]
    out = b""
    for r in refs:
        ob = read_obj(r)
        if ob is None: continue
        s = ob.find(b"stream")
        if s < 0: continue
        payload = ob[s+6:]
        payload = payload.lstrip(b"\r\n")
        e = payload.rfind(b"endstream")
        if e >= 0: payload = payload[:e]
        payload = payload.rstrip(b"\r\n")
        try:
            out += zlib.decompress(payload)
        except Exception as ex:
            out += payload
    return out, body

def media_box(body):
    m = re.search(rb"/MediaBox\s*\[([^\]]*)\]", body)
    return m.group(1).split() if m else None

OPS = re.compile(rb"(?<![A-Za-z])(Tj|TJ|'|\")(?![A-Za-z])")
c1, body1 = page_content(leaves[0])
print("page1 mediabox", media_box(body1))
print("page1 content len", len(c1))
from collections import Counter
print("op counts page1", Counter(m.group(1) for m in OPS.finditer(c1)))

allops = Counter()
for n in leaves:
    c, _ = page_content(n)
    allops.update(m.group(1) for m in OPS.finditer(c))
print("op counts whole document", allops)
