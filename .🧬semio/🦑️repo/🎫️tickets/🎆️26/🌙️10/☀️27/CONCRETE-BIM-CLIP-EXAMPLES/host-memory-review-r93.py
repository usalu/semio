"""🪟 Read native process working sets without controlling any process or changing capacity policy."""
import ctypes,json,os
from ctypes import wintypes as W
from pathlib import Path
if os.name!='nt':raise SystemExit('This diagnostic reads the current Windows host only')
class Counters(ctypes.Structure):
    """📏 Match PROCESS_MEMORY_COUNTERS_EX in the native Windows ABI."""
    _fields_=[('cb',W.DWORD),('PageFaultCount',W.DWORD)]+[(k,ctypes.c_size_t)for k in ('PeakWorkingSetSize','WorkingSetSize','QuotaPeakPagedPoolUsage','QuotaPagedPoolUsage','QuotaPeakNonPagedPoolUsage','QuotaNonPagedPoolUsage','PagefileUsage','PeakPagefileUsage','PrivateUsage')]
k=ctypes.WinDLL('kernel32',use_last_error=True);p=ctypes.WinDLL('psapi',use_last_error=True);k.OpenProcess.argtypes=[W.DWORD,W.BOOL,W.DWORD];k.OpenProcess.restype=W.HANDLE;k.CloseHandle.argtypes=[W.HANDLE];k.CloseHandle.restype=W.BOOL;k.QueryFullProcessImageNameW.argtypes=[W.HANDLE,W.DWORD,W.LPWSTR,ctypes.POINTER(W.DWORD)];k.QueryFullProcessImageNameW.restype=W.BOOL;p.EnumProcesses.argtypes=[ctypes.POINTER(W.DWORD),W.DWORD,ctypes.POINTER(W.DWORD)];p.EnumProcesses.restype=W.BOOL;p.GetProcessMemoryInfo.argtypes=[W.HANDLE,ctypes.c_void_p,W.DWORD];p.GetProcessMemoryInfo.restype=W.BOOL
ids=(W.DWORD*65536)();used=W.DWORD();assert p.EnumProcesses(ids,ctypes.sizeof(ids),ctypes.byref(used));rows=[]
for pid in ids[:used.value//ctypes.sizeof(W.DWORD)]:
    h=k.OpenProcess(0x1010,False,pid)
    if not h:continue
    try:
        info=Counters();info.cb=ctypes.sizeof(info);text=ctypes.create_unicode_buffer(32768);length=W.DWORD(len(text))
        if p.GetProcessMemoryInfo(h,ctypes.byref(info),info.cb)and k.QueryFullProcessImageNameW(h,0,text,ctypes.byref(length)):rows.append({'pid':pid,'image':Path(text.value).name,'rss':info.WorkingSetSize,'private':info.PrivateUsage})
    finally:k.CloseHandle(h)
rows=sorted(rows,key=lambda r:r['rss'],reverse=True)[:12];ticket=Path(__file__).parent;(ticket/'🗑️generated/host-memory-review-r93.json').write_text(json.dumps(rows,indent=2),encoding='utf8');(ticket/'host-memory-review-r93.md').write_text('# Read-Only Host Memory Review\n\nRepository admission reported no active managed jobs while the host remained under raised/critical memory pressure. A native read-only process working-set snapshot was retained under generated. No process is stopped, reprioritized or reconfigured. No capacity limit, reservation or admission rule is changed.\n',encoding='utf8');print('[DEBUG] '+json.dumps(rows))
