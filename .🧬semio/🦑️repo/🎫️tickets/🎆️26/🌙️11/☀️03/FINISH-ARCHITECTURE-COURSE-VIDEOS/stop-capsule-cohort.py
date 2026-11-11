"""Inspect and stop only the source 121 cohort after strict argv verification."""
import ctypes
from ctypes import wintypes
import sys, time, shutil
from pathlib import Path
k=ctypes.WinDLL("kernel32",use_last_error=True)
nt=ctypes.WinDLL("ntdll")
sh=ctypes.WinDLL("shell32",use_last_error=True)
class Entry(ctypes.Structure):
    _fields_=[("size",wintypes.DWORD),("usage",wintypes.DWORD),("pid",wintypes.DWORD),("heap",ctypes.c_size_t),("module",wintypes.DWORD),("threads",wintypes.DWORD),("parent",wintypes.DWORD),("priority",ctypes.c_long),("flags",wintypes.DWORD),("exe",wintypes.WCHAR*260)]
class Unicode(ctypes.Structure):
    _fields_=[("length",ctypes.c_ushort),("maximum",ctypes.c_ushort),("buffer",ctypes.c_void_p)]
k.OpenProcess.restype=wintypes.HANDLE
k.CreateToolhelp32Snapshot.restype=wintypes.HANDLE
sh.CommandLineToArgvW.restype=ctypes.POINTER(wintypes.LPWSTR)
nt.NtQueryInformationProcess.argtypes=[wintypes.HANDLE,ctypes.c_ulong,ctypes.c_void_p,ctypes.c_ulong,ctypes.POINTER(ctypes.c_ulong)]
def processes():
    snap=k.CreateToolhelp32Snapshot(2,0); e=Entry(); e.size=ctypes.sizeof(e); d={}
    ok=k.Process32FirstW(snap,ctypes.byref(e))
    while ok:
        d[e.pid]=e.parent
        ok=k.Process32NextW(snap,ctypes.byref(e))
    k.CloseHandle(snap)
    return d
def argv(pid):
    h=k.OpenProcess(0x1010,False,pid)
    if not h: return []
    n=ctypes.c_ulong(); nt.NtQueryInformationProcess(h,60,None,0,ctypes.byref(n))
    if not n.value: k.CloseHandle(h); return []
    b=ctypes.create_string_buffer(n.value); status=nt.NtQueryInformationProcess(h,60,b,n,ctypes.byref(n)); k.CloseHandle(h)
    if status: return []
    u=Unicode.from_buffer(b); command=ctypes.wstring_at(u.buffer,u.length//2)
    count=ctypes.c_int(); ptr=sh.CommandLineToArgvW(command,ctypes.byref(count))
    if not ptr: return []
    result=[ptr[i] for i in range(count.value)]
    k.LocalFree(ptr)
    return result
def main():
    parents=processes()
    worker=62844
    chain=[]; p=worker
    while p in parents and p not in chain:
        chain.append(p); p=parents[p]
    for p in chain:
        a=argv(p)
        print(p,parents[p],repr(a),flush=True)
    if "--stop" not in sys.argv: return
    workers=argv(worker)
    if not any(workers[i:i+2]==["-m","manim"] for i in range(len(workers)-1)) or "Beat2_Capsule" not in workers: raise RuntimeError("worker guard")
    candidates=[]
    for p in chain[1:]:
        a=argv(p); pairs=[a[i+1] for i,x in enumerate(a[:-1]) if x=="--video"]
        if len(pairs)==1 and pairs[0]=="grasshopper_capsule" and a and Path(a[0]).name.lower()=="python.exe" and len(a)>1 and Path(a[1]).name=="🐍️.py": candidates.append(p)
    if len(candidates)!=2: raise RuntimeError("main guard "+repr(candidates))
    parent=candidates[-1]; tree={parent}
    while True:
        new=tree|{p for p,q in parents.items() if q in tree}
        if new==tree: break
        tree=new
    if worker not in tree: raise RuntimeError("ancestry guard")
    print("Stopping",sorted(tree),flush=True)
    for p in sorted(tree,key=lambda x: x==parent):
        h=k.OpenProcess(1,False,p)
        if h:
            k.TerminateProcess(h,1); k.CloseHandle(h)
    for _ in range(100):
        if not (tree & set(processes())): break
        time.sleep(.1)
    remaining=tree & set(processes())
    if remaining: raise RuntimeError("workers alive "+repr(remaining))
    partial=Path(".🧬semio/🎓️teaching/⚡️cache/videos/en/speaker/s/121/p/Beat2_Capsule").resolve()
    expected=Path(".🧬semio/🎓️teaching/⚡️cache/videos/en/speaker/s/121/p").resolve()
    if partial.parent!=expected or partial.name!="Beat2_Capsule": raise RuntimeError("path guard")
    if partial.exists(): shutil.rmtree(partial)
    print("Stopped and removed only",partial,flush=True)
if __name__=="__main__": main()
