"""🔒 Lists Windows processes using the specified Cargo lock files."""

import ctypes
import os
import sys
from ctypes import wintypes

if os.name != "nt":
    raise SystemExit("Windows Restart Manager is required")


class Process(ctypes.Structure):
    _fields_ = [("pid", wintypes.DWORD), ("started", wintypes.FILETIME)]


class ProcessInfo(ctypes.Structure):
    _fields_ = [
        ("process", Process), ("name", wintypes.WCHAR * 256),
        ("service", wintypes.WCHAR * 64), ("kind", wintypes.DWORD),
        ("status", wintypes.DWORD), ("session", wintypes.DWORD),
        ("restartable", wintypes.BOOL),
    ]


rm = ctypes.WinDLL("rstrtmgr")
handle = wintypes.DWORD()
key = ctypes.create_unicode_buffer(33)
def checked(result):
    if result:
        raise ctypes.WinError(result)


checked(rm.RmStartSession(ctypes.byref(handle), 0, key))
try:
    paths = (wintypes.LPCWSTR * len(sys.argv[1:]))(*map(os.path.abspath, sys.argv[1:]))
    checked(rm.RmRegisterResources(handle, len(paths), paths, 0, None, 0, None))
    count, needed, reason = wintypes.UINT(), wintypes.UINT(), wintypes.DWORD()
    result = rm.RmGetList(handle, ctypes.byref(needed), ctypes.byref(count), None, ctypes.byref(reason))
    while result == 234:
        count.value = needed.value
        rows = (ProcessInfo * count.value)()
        result = rm.RmGetList(handle, ctypes.byref(needed), ctypes.byref(count), rows, ctypes.byref(reason))
    checked(result)
    for index in range(count.value):
        print(rows[index].process.pid, rows[index].name)
finally:
    checked(rm.RmEndSession(handle))
