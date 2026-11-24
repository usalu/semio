"""🔬 Dumps readable strings of a Grasshopper .gh archive (raw deflate GH_IO stream)."""
import re, sys, zlib
raw = open(sys.argv[1], "rb").read()
data = None
for wbits in (-15, 15, 31):
    for skip in range(0, 64):
        try:
            data = zlib.decompressobj(wbits).decompress(raw[skip:])
            if len(data) > len(raw): break
        except Exception: data = None
    if data: break
open(sys.argv[2], "wb").write(data)
strings = re.findall(rb"[\x20-\x7e\xc2-\xf4\x80-\xbf]{3,}", data)
open(sys.argv[3], "w", encoding="utf-8").write("\n".join(s.decode("utf-8", "replace") for s in strings))
print(len(raw), len(data), len(strings))
