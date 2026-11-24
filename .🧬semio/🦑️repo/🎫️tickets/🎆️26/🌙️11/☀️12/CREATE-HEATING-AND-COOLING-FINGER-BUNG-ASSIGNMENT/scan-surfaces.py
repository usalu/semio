"""📐 Lists plausible coordinate doubles inside every raw-deflated OpenNURBS blob of a decompressed GH_IO archive."""
import re, struct, sys, zlib
d = open(sys.argv[1], "rb").read()
for i, m in enumerate(re.finditer(rb"\x07ON_Data\xff\xff\xff\xff\x14\x00\x00\x00", d)):
    n = struct.unpack_from("<i", d, m.end())[0]; part = zlib.decompressobj(-15).decompress(d[m.end() + 4:m.end() + 4 + n])
    open(sys.argv[2] + "/surface-%d.bin" % i, "wb").write(part)
    vals = []
    o = 0
    while o < len(part) - 7:
        v = struct.unpack_from("<d", part, o)[0]
        if v == v and 1e-3 < abs(v) < 100 and abs(v * 1e4 - round(v * 1e4)) < 1e-6: vals.append((o, round(v, 4))); o += 8
        else: o += 1
    print(i, n, len(part), sorted(set(re.findall(rb"ON_[A-Za-z]+", part)))); print("  ", vals)
