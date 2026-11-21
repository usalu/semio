"""🗺️ Count the native probe compiler's retained metadata file mappings."""

import ctypes
from collections import Counter
from ctypes import wintypes
import sys
import time


class MemoryRegion(ctypes.Structure):
    _fields_ = [
        ("base", ctypes.c_void_p),
        ("allocation", ctypes.c_void_p),
        ("allocation_protection", wintypes.DWORD),
        ("partition", wintypes.WORD),
        ("extent", ctypes.c_size_t),
        ("state", wintypes.DWORD),
        ("protection", wintypes.DWORD),
        ("kind", wintypes.DWORD),
    ]


def bind(library, name, arguments, result):
    method = getattr(library, name)
    method.argtypes = arguments
    method.restype = result
    return method


def mapped_files(handle):
    address = 0
    region = MemoryRegion()
    name = ctypes.create_unicode_buffer(32768)
    found = {}
    while query(handle, address, ctypes.byref(region), ctypes.sizeof(region)):
        if region.kind == 0x40000 and mapped_name(handle, region.base, name, len(name)):
            path = name.value
            if path.endswith((".rmeta", ".rlib")):
                found[path] = found.get(path, 0) + region.extent
        address = (region.base or 0) + region.extent
    return found


kernel = ctypes.WinDLL("kernel32", use_last_error=True)
psapi = ctypes.WinDLL("psapi", use_last_error=True)
open_process = bind(kernel, "OpenProcess", [wintypes.DWORD, wintypes.BOOL, wintypes.DWORD], wintypes.HANDLE)
close_handle = bind(kernel, "CloseHandle", [wintypes.HANDLE], wintypes.BOOL)
query = bind(kernel, "VirtualQueryEx", [wintypes.HANDLE, ctypes.c_void_p, ctypes.POINTER(MemoryRegion), ctypes.c_size_t], ctypes.c_size_t)
mapped_name = bind(psapi, "GetMappedFileNameW", [wintypes.HANDLE, ctypes.c_void_p, wintypes.LPWSTR, wintypes.DWORD], wintypes.DWORD)

handle = open_process(0x0410, False, int(sys.argv[1]))
if not handle:
    raise ctypes.WinError(ctypes.get_last_error())
sys.stdout.reconfigure(encoding="utf-8")
try:
    previous = {}
    for sample in range(int(sys.argv[2])):
        current = mapped_files(handle)
        added = sorted(current.keys() - previous.keys())
        volumes = Counter((path.split("\\")[2], path.rsplit(".", 1)[-1]) for path in current)
        print(f"[DEBUG] metadata sample {sample}: {len(current)} files, {sum(current.values()):,} mapped bytes, {len(added)} additions, {dict(volumes)}", flush=True)
        if sample:
            for path in added:
                print(f"[DEBUG] mapped {path}", flush=True)
        previous = current
        if sample + 1 < int(sys.argv[2]):
            time.sleep(20)
finally:
    close_handle(handle)
