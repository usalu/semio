"""🧩 Parses a decompressed GH_IO binary archive into an indented text tree."""
import struct, sys
d = open(sys.argv[1], "rb").read(); out = open(sys.argv[2], "w", encoding="utf-8"); p = 0
def u8():
    global p; v = d[p]; p += 1; return v
def i32():
    global p; v = struct.unpack_from("<i", d, p)[0]; p += 4; return v
def s():
    global p; n = 0; sh = 0
    while True:
        b = u8(); n |= (b & 127) << sh; sh += 7
        if b < 128: break
    v = d[p:p+n].decode("utf-8", "replace"); p += n; return v
def take(fmt):
    global p; v = struct.unpack_from("<" + fmt, d, p); p += struct.calcsize("<" + fmt); return v
def item(ind):
    global p
    name = s(); idx = i32(); t = i32()
    if t == 1: v = take("?")[0]
    elif t == 2: v = u8()
    elif t == 3: v = i32()
    elif t == 4: v = take("q")[0]
    elif t == 5: v = take("f")[0]
    elif t == 6: v = take("d")[0]
    elif t == 7: v = take("16s")[0].hex()
    elif t == 8: v = take("q")[0]
    elif t == 9: v = take("16s")[0].hex()
    elif t == 10: v = s()
    elif t == 20: n = i32(); v = "<%d bytes>" % n; p += n
    elif t == 21: n = i32(); v = take("%dd" % n)
    elif t == 30: v = take("2i")
    elif t == 31: v = take("2f")
    elif t == 32: v = take("2i")
    elif t == 33: v = take("2f")
    elif t == 34: v = take("4i")
    elif t == 35: v = take("4f")
    elif t == 36: v = take("i")
    elif t == 37: v = take("i")
    elif t == 40: v = take("2d")
    elif t == 41: v = take("3d")
    elif t == 42: v = take("4d")
    elif t == 50: v = take("2d")
    elif t == 51: v = take("3d")
    elif t == 52: v = take("4d")
    elif t == 53: v = take("4d")
    elif t == 54: v = take("6d")
    elif t == 55: v = take("12d")
    elif t == 56: v = take("16d")
    elif t == 57: v = take("9d")
    elif t == 58: v = take("9d")
    elif t == 60: v = take("2d")
    elif t == 61: v = take("4d")
    elif t == 62: v = take("5d")
    elif t == 63: v = take("5d")
    elif t == 70: v = take("2d")
    elif t == 71: v = take("4d")
    elif t == 72: v = take("9d")
    elif t == 73: v = take("6d")
    elif t == 74: v = take("9d")
    elif t == 80: v = take("3i")
    else: raise Exception("type %d at %d name %s" % (t, p, name))
    print("  " * ind + name + "[" + str(idx) + "] = " + repr(v)[:4000], file=out)
def chunk(ind):
    name = s(); idx = i32(); ni = i32(); nc = i32()
    out.write("%s<%s %d>\n" % ("  " * ind, name, idx))
    for _ in range(ni): item(ind + 1)
    for _ in range(nc): chunk(ind + 1)
try: chunk(0)
finally: out.close(); print(p, len(d))
