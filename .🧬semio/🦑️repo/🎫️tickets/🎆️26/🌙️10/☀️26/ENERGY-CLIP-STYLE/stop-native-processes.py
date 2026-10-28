"""🛑 Stop explicitly selected child PIDs only after verifying one owned process tree."""
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
selected={int(value) for value in sys.argv[2:]}
if not selected or int(sys.argv[1]) in selected or not selected <= parents:
    raise ValueError("Explicit child PIDs must belong to the selected root process")
names={row["pid"]:row["name"] for row in rows}
if any(names[pid] != "python.exe" and not names[pid].startswith("ffmpeg") for pid in selected):
    raise ValueError("Only inspected Python and FFmpeg test children may be stopped")
k.OpenProcess.argtypes=[wintypes.DWORD,wintypes.BOOL,wintypes.DWORD]
k.OpenProcess.restype=wintypes.HANDLE
k.TerminateProcess.argtypes=[wintypes.HANDLE,wintypes.UINT]
k.CloseHandle.argtypes=[wintypes.HANDLE]
for pid in map(int,sys.argv[2:]):
    handle=k.OpenProcess(1,False,pid)
    if handle:
        print("[DEBUG] Stopping verified native test process",pid)
        k.TerminateProcess(handle,1)
        k.CloseHandle(handle)

