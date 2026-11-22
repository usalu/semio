"""🧹 Identify native staffage workers by their actual interpreter and script arguments."""
import ctypes,os,subprocess,sys
from pathlib import Path
t=Path(__file__).resolve().parent
def mac_arguments(pid):
    import struct
    ctl=ctypes.CDLL(None,use_errno=True).sysctl
    query=(ctypes.c_int*3)(1,49,pid);length=ctypes.c_size_t()
    if ctl(query,3,None,ctypes.byref(length),None,0):return []
    buf=ctypes.create_string_buffer(length.value)
    if ctl(query,3,buf,ctypes.byref(length),None,0):return []
    s=buf.raw[:length.value];count=struct.unpack_from('i',s)[0];i=s.find(b'\0',4)
    while i<len(s) and s[i]==0:i+=1
    return [part.decode(errors='replace') for part in s[i:].split(b'\0',count)[:count]]
def procs():
    if sys.platform=='win32':
        import json
        cmd='Get-CimInstance Win32_Process | Select-Object ProcessId,ParentProcessId,Name,CommandLine | ConvertTo-Json -Compress'
        for r in json.loads(subprocess.check_output(['powershell','-NoProfile','-Command',cmd],text=True)):
            if r['CommandLine'] and 'STAFFAGE' in r['CommandLine']:
                from ctypes import wintypes
                count=ctypes.c_int();split=ctypes.windll.shell32.CommandLineToArgvW;split.restype=ctypes.POINTER(wintypes.LPWSTR)
                ptr=split(r['CommandLine'],ctypes.byref(count))
                try:yield r['ProcessId'],r['ParentProcessId'],[ptr[i] for i in range(count.value)]
                finally:ctypes.windll.kernel32.LocalFree(ctypes.cast(ptr,ctypes.c_void_p))
    else:
        for line in subprocess.check_output(['ps','-axo','pid=,ppid=,comm='],text=True).splitlines():
            bits=line.split(None,2)
            if len(bits)!=3:continue
            pid,parent,exe=bits
            if not Path(exe).name.lower().startswith(('python','bun','node','zsh','sh')):continue
            try:
                args=mac_arguments(int(pid)) if sys.platform=='darwin' else [a.decode(errors='replace') for a in Path('/proc',pid,'cmdline').read_bytes().split(b'\0') if a]
            except (PermissionError,FileNotFoundError,ProcessLookupError):continue
            yield int(pid),int(parent),args
def script(args):
    i=1
    while i<len(args) and args[i] in ('--smol','-u','--'):i+=1
    if i>=len(args) or args[i].startswith('-'):return None
    p=Path(args[i]);p=p.resolve() if not p.is_absolute() else p
    return p if p.parent==t else None
rows=list(procs());current={pid:(parent,p) for pid,parent,argv in rows if (p:=script(argv)) is not None}
nested=[]
parents={pid:parent for pid,parent,_ in rows}
for pid,(p,s) in current.items():
    if s.name!='equipment-guard.py':continue
    while p in parents:
        if p in current and current[p][1].name=='equipment-guard.py':nested.append([pid,p])
        p=parents[p]
print('Ticket interpreters',len(current),'adopted',[(p,s.name)for p,(parent,s) in current.items()if parent==1],'nested guards',nested)
