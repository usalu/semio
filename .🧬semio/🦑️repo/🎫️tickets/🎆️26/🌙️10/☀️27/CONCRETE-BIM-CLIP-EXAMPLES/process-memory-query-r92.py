"""🔎 Read native process memory counters without WMI, touching no process state."""
import json,os,sys,time
from pathlib import Path
rows=[];started=time.monotonic()
if os.name=='nt':
    import ctypes as c
    from ctypes import wintypes as w
    class Entry(c.Structure):
        _fields_=[('size',w.DWORD),('usage',w.DWORD),('pid',w.DWORD),('heap',c.c_size_t),('module',w.DWORD),('threads',w.DWORD),('parent',w.DWORD),('priority',w.LONG),('flags',w.DWORD),('name',w.WCHAR*260)]
    class Memory(c.Structure):
        _fields_=[('size',w.DWORD),('faults',w.DWORD),('peak',c.c_size_t),('resident',c.c_size_t),('paged_peak',c.c_size_t),('paged',c.c_size_t),('nonpaged_peak',c.c_size_t),('nonpaged',c.c_size_t),('commit',c.c_size_t),('commit_peak',c.c_size_t),('private',c.c_size_t)]
    kernel=c.WinDLL('kernel32',use_last_error=True);psapi=c.WinDLL('psapi',use_last_error=True)
    kernel.CreateToolhelp32Snapshot.argtypes=[w.DWORD,w.DWORD];kernel.CreateToolhelp32Snapshot.restype=w.HANDLE
    kernel.Process32FirstW.argtypes=[w.HANDLE,c.POINTER(Entry)];kernel.Process32FirstW.restype=w.BOOL
    kernel.Process32NextW.argtypes=[w.HANDLE,c.POINTER(Entry)];kernel.Process32NextW.restype=w.BOOL
    kernel.OpenProcess.argtypes=[w.DWORD,w.BOOL,w.DWORD];kernel.OpenProcess.restype=w.HANDLE
    kernel.CloseHandle.argtypes=[w.HANDLE];kernel.CloseHandle.restype=w.BOOL
    psapi.GetProcessMemoryInfo.argtypes=[w.HANDLE,c.POINTER(Memory),w.DWORD];psapi.GetProcessMemoryInfo.restype=w.BOOL
    snapshot=kernel.CreateToolhelp32Snapshot(2,0);entry=Entry();entry.size=c.sizeof(entry)
    try:
        valid=kernel.Process32FirstW(snapshot,c.byref(entry))
        while valid:
            handle=kernel.OpenProcess(0x410,False,entry.pid)
            if handle:
                try:
                    value=Memory();value.size=c.sizeof(value)
                    if psapi.GetProcessMemoryInfo(handle,c.byref(value),value.size):rows.append({'pid':entry.pid,'parent':entry.parent,'name':entry.name,'resident_mb':round(value.resident/1048576,2),'private_mb':round(value.private/1048576,2)})
                finally:kernel.CloseHandle(handle)
            valid=kernel.Process32NextW(snapshot,c.byref(entry))
    finally:kernel.CloseHandle(snapshot)
elif Path('/proc').is_dir():
    for path in Path('/proc').iterdir():
        if not path.name.isdigit():continue
        try:
            status=dict(line.split(':',1) for line in (path/'status').read_text().splitlines() if ':' in line);rows.append({'pid':int(path.name),'parent':int(status['PPid']),'name':status['Name'].strip(),'resident_mb':round(int(status.get('VmRSS','0 kB').split()[0])/1024,2),'private_mb':None})
        except (OSError,KeyError,ValueError):pass
rows.sort(key=lambda r:r['resident_mb'],reverse=True);result={'platform':sys.platform,'seconds':time.monotonic()-started,'processes':rows,'total_resident_mb':sum(r['resident_mb'] for r in rows),'note':'Read-only counters; shared resident pages can be counted in multiple processes; no command lines or process memory contents are read.'}
(Path(__file__).parent/'🗑️generated/process-memory-query-r92.json').write_text(json.dumps(result,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({'seconds':result['seconds'],'total_resident_mb':result['total_resident_mb'],'largest':rows[:18]},ensure_ascii=False),flush=True)
