"""🔐 Inspect Windows Restart Manager's process list for named Cargo cache files."""

import ctypes as C
from ctypes import wintypes as W
import json
import sys


class UniqueProcess(C.Structure):
    _fields_ = [("pid", W.DWORD), ("start", W.FILETIME)]


class ProcessInfo(C.Structure):
    _fields_ = [
        ("process", UniqueProcess), ("app", W.WCHAR * 256),
        ("service", W.WCHAR * 64), ("kind", C.c_int),
        ("status", W.ULONG), ("session", W.DWORD), ("restartable", W.BOOL),
    ]


def main(paths):
    api = C.WinDLL("rstrtmgr")
    session = W.DWORD()
    key = C.create_unicode_buffer(33)
    result = api.RmStartSession(C.byref(session), 0, key)
    if result:
        raise C.WinError(result)
    try:
        names = (W.LPCWSTR * len(paths))(*paths)
        result = api.RmRegisterResources(session, len(paths), names, 0, None, 0, None)
        if result:
            raise C.WinError(result)
        needed, count, reasons = W.UINT(), W.UINT(1024), W.DWORD()
        info = (ProcessInfo * count.value)()
        result = api.RmGetList(session, C.byref(needed), C.byref(count), info, C.byref(reasons))
        if result:
            raise C.WinError(result)
        print(json.dumps([{"pid": info[i].process.pid, "application": info[i].app}
                          for i in range(count.value)]))
    finally:
        api.RmEndSession(session)


if __name__ == "__main__":
    main(sys.argv[1:])
