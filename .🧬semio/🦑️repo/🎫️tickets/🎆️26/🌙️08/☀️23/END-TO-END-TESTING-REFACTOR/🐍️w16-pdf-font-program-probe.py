import re, sys, zlib, hashlib

def objects(data):
    out = {}
    for m in re.finditer(rb'(?m)^(\d+)\s+(\d+)\s+obj\b', data):
        num = int(m.group(1)); start = m.end()
        e = data.find(b'endobj', start)
        out[num] = data[start:e if e != -1 else len(data)]
    return out

def fontfile_refs(data):
    refs = set()
    for m in re.finditer(rb'/FontFile[23]?\s+(\d+)\s+(\d+)\s+R', data):
        refs.add(int(m.group(1)))
    return refs

def stream_bytes(body):
    m = re.search(rb'stream\r?\n', body)
    if not m: return None, None
    s = m.end()
    e = body.rfind(b'endstream')
    if e == -1: return None, None
    raw = body[s:e]
    if raw.endswith(b'\r\n'): raw = raw[:-2]
    elif raw.endswith(b'\n') or raw.endswith(b'\r'): raw = raw[:-1]
    dec = None
    try:
        dec = zlib.decompress(raw)
    except Exception:
        try:
            dec = zlib.decompressobj().decompress(raw)
        except Exception:
            dec = None
    return raw, dec

a, b = sys.argv[1], sys.argv[2]
da = open(a,'rb').read(); db = open(b,'rb').read()
oa, ob = objects(da), objects(db)
ra, rb_ = fontfile_refs(da), fontfile_refs(db)
print("oracle fontfile objs:", len(ra), " subject:", len(rb_), " same numbers:", ra == rb_)
diffc = 0; diffd = 0; nodec = 0
for n in sorted(ra & rb_):
    ca, dca = stream_bytes(oa[n]); cb, dcb = stream_bytes(ob[n])
    if ca is None or cb is None:
        print("  obj", n, "NO STREAM"); continue
    if dca is None or dcb is None:
        nodec += 1
        print("  obj", n, "UNDECODABLE", len(ca), len(cb)); continue
    if len(ca) != len(cb):
        diffc += 1
        same = dca == dcb
        if not same: diffd += 1
        print(f"  obj {n} compressed {len(ca)} -> {len(cb)}  decompressed {len(dca)} -> {len(dcb)}  contentSame {same}  sha {hashlib.sha256(dca).hexdigest()[:12]} {hashlib.sha256(dcb).hexdigest()[:12]}")
    elif dca != dcb:
        diffd += 1
        print(f"  obj {n} SAME compressed len but DIFFERENT decoded content")
print("compressed-length differs:", diffc, " decoded-content differs:", diffd, " undecodable:", nodec)
