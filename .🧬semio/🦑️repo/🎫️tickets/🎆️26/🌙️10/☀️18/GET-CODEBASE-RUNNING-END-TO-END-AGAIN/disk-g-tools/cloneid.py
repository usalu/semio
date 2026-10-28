"""Prints the APFS clone id of each path read from stdin (macOS getattrlist ATTR_CMNEXT_CLONEID); equal ids share their blocks. Streams, holds one path at a time."""
import ctypes, ctypes.util, sys, struct

libc = ctypes.CDLL(ctypes.util.find_library("c"), use_errno=True)


class AttrList(ctypes.Structure):
    _fields_ = [("bitmapcount", ctypes.c_ushort), ("reserved", ctypes.c_ushort), ("commonattr", ctypes.c_uint32), ("volattr", ctypes.c_uint32), ("dirattr", ctypes.c_uint32), ("fileattr", ctypes.c_uint32), ("forkattr", ctypes.c_uint32)]


def clone_id(path):
    al = AttrList(5, 0, 0, 0, 0, 0, 0x100)
    buf = ctypes.create_string_buffer(64)
    r = libc.getattrlist(path.encode("utf8"), ctypes.byref(al), buf, 64, 0x20)
    if r != 0:
        return None
    return struct.unpack_from("<Q", buf.raw, 4)[0]


for line in sys.stdin:
    p = line.rstrip("\n")
    print(clone_id(p), p)
