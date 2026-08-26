import struct
h = "38001c1340808e616500010ea08199999999999e19fa04000000000001c17e000000000001c17f800000000000701fe000000000001c07e805110f66fe0e00260140808ee17400010eb040d4443f869da500560a808443aa81a0202892080218"
b = bytes.fromhex(h)
payload = b[3:59]

class R:
    def __init__(self, data, pos=0):
        self.d = data; self.p = pos
    def bit(self):
        if self.p >= len(self.d)*8: raise EOFError("underflow")
        v = (self.d[self.p//8] >> (7 - self.p % 8)) & 1
        self.p += 1
        return v
    def bits(self, n):
        v = 0
        for _ in range(n): v = (v<<1) | self.bit()
        return v
    def b(self): return self.bit()
    def bb(self): return self.bits(2)
    def rc(self): return self.bits(8)
    def rs(self): 
        a=self.rc(); c=self.rc(); return a | (c<<8)
    def rl(self):
        return self.rs() | (self.rs()<<16)
    def rd(self):
        return struct.unpack("<d", bytes(self.rc() for _ in range(8)))[0]
    def bs(self):
        c=self.bb()
        return self.rs() if c==0 else (self.rc() if c==1 else (0 if c==2 else 256))
    def bl(self):
        c=self.bb()
        return self.rl() if c==0 else (self.rc() if c==1 else 0)
    def bd(self):
        c=self.bb()
        return self.rd() if c==0 else (1.0 if c==1 else 0.0)
    def bot(self):
        c=self.bb()
        return self.rc() if c==0 else (self.rc()+0x1f0 if c==1 else self.rs())
    def handle(self):
        head=self.rc(); code=head>>4; ln=head&0xF
        v=0
        for _ in range(ln): v=(v<<8)|self.rc()
        return code, v
    def dd(self, default):
        c=self.bb()
        if c==0: return default
        if c==1:
            raw=struct.pack("<d", default); nb=bytes(self.rc() for _ in range(4))
            return struct.unpack("<d", nb+raw[4:])[0]
        if c==2:
            raw=struct.pack("<d", default); nb=bytes(self.rc() for _ in range(6))
            return struct.unpack("<d", raw[4:6]+nb[0:2]+nb[2:6])[0]
        return self.rd()

r = R(payload)
print("bot", r.bot(), "pos", r.p)
print("handle", r.handle(), "pos", r.p)
print("eed bs", r.bs(), "pos", r.p)
print("graphic", r.b(), "pos", r.p)
print("entmode", r.bb(), "pos", r.p)
print("numreactors", r.bl(), "pos", r.p)
print("xdic_missing", r.b(), "pos", r.p)
mark = r.p
for nolinks in (True, False):
    r.p = mark
    tag = "WITH nolinks" if nolinks else "NO nolinks"
    if nolinks: print(tag, "nolinks", r.b())
    col = r.bs()
    if col & 0x8000: r.rl()
    if col & 0x2000: r.rl()
    print(tag, "color", col & 0x1ff, "flags", hex(col>>8), "pos", r.p)
    print(tag, "ltype_scale", r.bd(), "pos", r.p)
    print(tag, "ltype_flags", r.bb(), "plotstyle", r.bb(), "material", r.bb(), "pos", r.p)
    print(tag, "shadow", r.rc(), "pos", r.p)
    if not nolinks:
        print(tag, "visualstyles", r.b(), r.b(), r.b(), "pos", r.p)
    print(tag, "invis", r.bs(), "pos", r.p)
    print(tag, "lineweight", r.rc(), "pos", r.p)
    try:
        flags = r.bs()
        print(tag, "LWPOLY flags", flags, "pos", r.p)
        if flags & 4: print(tag, " const_width", r.bd(), "pos", r.p)
        elev = r.bd() if flags & 8 else 0.0
        print(tag, " elevation", elev, "pos", r.p)
        if flags & 2: print(tag, " thickness", r.bd(), "pos", r.p)
        if flags & 1: print(tag, " normal", [r.bd() for _ in range(3)], "pos", r.p)
        n = r.bl(); print(tag, " count", n, "pos", r.p)
        nb = r.bl() if flags & 16 else 0
        nv = r.bl() if flags & 1024 else 0
        nw = r.bl() if flags & 32 else 0
        print(tag, " bulges",nb,"vids",nv,"widths",nw,"pos", r.p)
        pts=[]
        if n>0 and n < 1000:
            pts.append((r.rd(), r.rd()))
            for _ in range(1, n):
                pts.append((r.dd(pts[-1][0]), r.dd(pts[-1][1])))
            print(tag, " points", pts, "pos", r.p, "of", len(payload)*8, "data_end", len(payload)*8-28)
    except Exception as e:
        print(tag, "  FAILED:", e, "pos", r.p)
    print("---")
