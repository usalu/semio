"""🔎 Inspect descendants of one explicitly selected Windows test process."""
import ctypes,json,sys
from ctypes import wintypes
class Entry(ctypes.Structure):
    _fields_=[("dwSize",wintypes.DWORD),("cntUsage",wintypes.DWORD),("th32ProcessID",wintypes.DWORD),("th32DefaultHeapID",ctypes.c_size_t),("th32ModuleID",wintypes.DWORD),("cntThreads",wintypes.DWORD),("th32ParentProcessID",wintypes.DWORD),("pcPriClassBase",wintypes.LONG),("dwFlags",wintypes.DWORD),("szExeFile",wintypes.WCHAR*260)]
k=ctypes.WinDLL("kernel32",use_last_error=True)
k.CreateToolhelp32Snapshot.restype=wintypes.HANDLE
h=k.CreateToolhelp32Snapshot(2,0)
e=Entry();e.dwSize=ctypes.sizeof(e);rows=[]
ok=k.Process32FirstW(h,ctypes.byref(e))
while ok:
    rows.append({"pid":e.th32ProcessID,"parent":e.th32ParentProcessID,"name":e.szExeFile})
    ok=k.Process32NextW(h,ctypes.byref(e))
parents={int(sys.argv[1])}
for _ in range(8):
    parents.update(row["pid"] for row in rows if row["parent"] in parents)
print(json.dumps([r for r in rows if r["pid"] in parents],indent=2))
