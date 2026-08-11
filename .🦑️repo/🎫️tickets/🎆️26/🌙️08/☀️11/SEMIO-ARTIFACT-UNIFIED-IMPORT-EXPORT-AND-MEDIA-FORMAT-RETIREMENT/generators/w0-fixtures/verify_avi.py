#!/usr/bin/env python3
"""Re-parse the handcrafted AVI file as a generic RIFF walker and assert
chunk sizes sum correctly, hdrl/strl/movi/idx1 are all present, and the
frame count + fourccs match expectations."""
import struct
import sys

path = sys.argv[1]
data = open(path, "rb").read()

def read_chunk(buf, off):
    fourcc = buf[off:off+4]
    size = struct.unpack("<I", buf[off+4:off+8])[0]
    body_start = off + 8
    body = buf[body_start:body_start+size]
    next_off = body_start + size
    if size % 2 == 1:
        next_off += 1  # pad byte
    return fourcc, size, body, next_off

assert data[0:4] == b"RIFF", "not a RIFF file"
riff_size = struct.unpack("<I", data[4:8])[0]
assert riff_size == len(data) - 8, f"RIFF size {riff_size} != file-8 {len(data)-8}"
form_type = data[8:12]
assert form_type == b"AVI ", f"unexpected form type {form_type}"

off = 12
found = {}
frame_chunks = []
while off < len(data):
    fourcc, size, body, next_off = read_chunk(data, off)
    if fourcc == b"LIST":
        list_type = body[0:4]
        found[list_type.decode()] = (off, size)
        if list_type == b"hdrl":
            inner_off = 4
            while inner_off < len(body):
                ifourcc, isize, ibody, inext = read_chunk(body, inner_off)
                if ifourcc == b"avih":
                    avih = struct.unpack("<IIIIIIIIIIIIII", ibody)
                    found["avih_fields"] = avih
                elif ifourcc == b"LIST" and ibody[0:4] == b"strl":
                    sinner = 4
                    while sinner < len(ibody):
                        sfourcc, ssize, sbody, snext = read_chunk(ibody, sinner)
                        if sfourcc == b"strh":
                            found["strh_fcc_type"] = sbody[0:4]
                            found["strh_fcc_handler"] = sbody[4:8]
                            found["strh_length"] = struct.unpack("<I", sbody[36:40])[0]
                        elif sfourcc == b"strf":
                            found["strf_width"] = struct.unpack("<i", sbody[4:8])[0]
                            found["strf_height"] = struct.unpack("<i", sbody[8:12])[0]
                        sinner = snext
                inner_off = inext
        elif list_type == b"movi":
            minner = 4
            while minner < len(body):
                mfourcc, msize, mbody, mnext = read_chunk(body, minner)
                frame_chunks.append((mfourcc, msize, minner))
                minner = mnext
    else:
        found[fourcc.decode()] = (off, size)
        if fourcc == b"idx1":
            entries = []
            for i in range(0, len(body), 16):
                efourcc, eflags, eoff, esize = struct.unpack("<4sIII", body[i:i+16])
                entries.append((efourcc, eflags, eoff, esize))
            found["idx1_entries"] = entries
    off = next_off

print("RIFF size field:", riff_size, "actual file-8:", len(data) - 8, "MATCH" if riff_size == len(data)-8 else "MISMATCH")
print("Top-level chunks found:", [k for k in found.keys() if not k.startswith(("avih", "strh", "strf", "idx1_"))])
print("avih (14 dwords):", found.get("avih_fields"))
print("strh fccType/fccHandler:", found.get("strh_fcc_type"), found.get("strh_fcc_handler"), "length:", found.get("strh_length"))
print("strf width/height:", found.get("strf_width"), found.get("strf_height"))
print("movi frame chunk count:", len(frame_chunks))
for fc, sz, o in frame_chunks:
    print("  frame chunk:", fc, "size:", sz, "offset-in-movi-data:", o)
print("idx1 entries:", found.get("idx1_entries"))

assert "hdrl" in found
assert "movi" in found
assert "idx1" in found
assert len(frame_chunks) >= 2
for fc, sz, o in frame_chunks:
    assert fc == b"00dc", f"unexpected frame fourcc {fc}"
idx1_entries = found["idx1_entries"]
assert len(idx1_entries) == len(frame_chunks)
for (efourcc, eflags, eoff, esize), (mfourcc, msize, moff) in zip(idx1_entries, frame_chunks):
    assert efourcc == b"00dc"
    assert eoff == moff, f"idx1 offset {eoff} != actual movi-relative offset {moff}"
    assert esize == msize, f"idx1 size {esize} != actual chunk size {msize}"

print("\nALL STRUCTURAL ASSERTIONS PASSED")
