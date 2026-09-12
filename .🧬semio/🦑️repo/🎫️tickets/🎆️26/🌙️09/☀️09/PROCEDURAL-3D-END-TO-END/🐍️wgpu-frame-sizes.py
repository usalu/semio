#!/usr/bin/env python3
"""🧵️ Reads aarch64 prologue stack reservations (`sub sp, sp, #N` / `sub xN, sp, #N` probe form)
out of a linked test binary, per symbol.

Instrument only — LLVM `objdump -d` is the third-party oracle; nothing here ships.
Usage: frame-sizes.py <binary> <out.tsv>
"""
import re, subprocess, sys

binary, out_path = sys.argv[1], sys.argv[2]
sym = re.compile(r"^[0-9a-f]+ <(.+)>:$")
sub_sp = re.compile(r"\bsub\s+sp,\s+sp,\s+#(0x[0-9a-f]+|\d+)(?:,\s+lsl\s+#(\d+))?")
sub_x_sp = re.compile(r"\bsub\s+x\d+,\s+sp,\s+#(0x[0-9a-f]+|\d+)(?:,\s+lsl\s+#(\d+))?")
stp_pre = re.compile(r"\bstp\s+x\d+,\s+x\d+,\s+\[sp,\s+#-(0x[0-9a-f]+|\d+)\]!")

def val(m):
    n = int(m.group(1), 0)
    if m.group(2):
        n <<= int(m.group(2))
    return n

proc = subprocess.Popen(["objdump", "-d", binary], stdout=subprocess.PIPE, text=True, bufsize=1 << 20)
rows, cur, total, seen = [], None, 0, 0
def flush():
    if cur is not None and total:
        rows.append((total, cur))
for line in proc.stdout:
    line = line.rstrip("\n")
    m = sym.match(line.strip())
    if m:
        flush()
        cur, total, seen = m.group(1), 0, 0
        continue
    if cur is None or seen > 24:
        continue
    seen += 1
    for rx, getter in ((sub_x_sp, val), (sub_sp, val), (stp_pre, lambda mm: int(mm.group(1), 0))):
        m2 = rx.search(line)
        if m2:
            total += getter(m2)
flush()
proc.wait()
rows.sort(reverse=True)
with open(out_path, "w") as fh:
    for n, name in rows:
        fh.write(f"{n}\t{name}\n")
print(f"{len(rows)} symbols -> {out_path}")
