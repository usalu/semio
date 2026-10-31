"""🧾 Classify only high-memory Python command metadata against this ticket, without exposing unrelated arguments."""
import ctypes as c,hashlib,json,os
from ctypes import wintypes as w
from pathlib import Path
T=Path(__file__).parent;prior=json.loads((T/'🗑️generated/process-memory-query-r92.json').read_text());targets=[r for r in prior['processes'] if r['name'].lower()=='python.exe' and r['resident_mb']>4000];by_pid={r['pid']:r for r in prior['processes']};seen={r['pid'] for r in targets}
for _ in range(4):
    parents=[by_pid[r['parent']] for r in targets if r['parent'] in by_pid and r['parent'] not in seen]
    targets.extend(parents);seen.update(r['pid'] for r in parents)
result=[]
class Unicode(c.Structure):
    _fields_=[('length',w.USHORT),('maximum',w.USHORT),('buffer',c.c_void_p)]
k=c.WinDLL('kernel32',use_last_error=True);n=c.WinDLL('ntdll');s=c.WinDLL('shell32')
k.OpenProcess.argtypes=[w.DWORD,w.BOOL,w.DWORD];k.OpenProcess.restype=w.HANDLE;k.CloseHandle.argtypes=[w.HANDLE];k.CloseHandle.restype=w.BOOL;k.LocalFree.argtypes=[w.HLOCAL];k.LocalFree.restype=w.HLOCAL
n.NtQueryInformationProcess.argtypes=[w.HANDLE,w.ULONG,c.c_void_p,w.ULONG,c.POINTER(w.ULONG)];n.NtQueryInformationProcess.restype=w.LONG
s.CommandLineToArgvW.argtypes=[w.LPCWSTR,c.POINTER(c.c_int)];s.CommandLineToArgvW.restype=c.POINTER(w.LPWSTR)
for row in targets:
    p={k:v for k,v in row.items()};h=k.OpenProcess(0x1000,False,row['pid'])
    if not h:p['metadata_available']=False;result.append(p);continue
    try:
        need=w.ULONG();n.NtQueryInformationProcess(h,60,None,0,c.byref(need))
        if not 0<need.value<1048576:raise ValueError('Bounded command metadata unavailable')
        b=c.create_string_buffer(need.value);status=n.NtQueryInformationProcess(h,60,b,need.value,c.byref(need))
        if status<0:raise ValueError('Command metadata status '+str(status))
        text=c.cast(b,c.POINTER(Unicode)).contents
        if not text.buffer or not c.addressof(b)<=text.buffer<=c.addressof(b)+len(b)-text.length:raise ValueError('Command metadata buffer is not owned')
        command=c.wstring_at(text.buffer,text.length//2);decoded=__import__('re').sub(r'\\[Uu]([0-9a-fA-F]{8}|[0-9a-fA-F]{4})',lambda match:chr(int(match[1],16)),command);argc=c.c_int();argv=s.CommandLineToArgvW(command,c.byref(argc))
        try:arguments=[argv[i] for i in range(argc.value)]
        finally:k.LocalFree(c.cast(argv,w.HLOCAL))
        marker='CONCRETE-BIM-CLIP-EXAMPLES';paths=[arg for arg in arguments if marker in arg and arg.lower().endswith(('.py','.mjs','.js'))]
        p.update({'metadata_available':True,'ticket_match':marker in command,'workspace_match':'git\\semio' in command.lower() or 'git/semio' in command.lower(),'bim_module_match':'🏗️bim' in command,'argc':len(arguments),'python_mode':next((v for v in arguments[1:3] if v in ('-c','-m','-u')),None),'bim_identifiers':[v for v in ('🏗️bim','villa','savoye','building_geometry','_slab_geometry','_occupied_volume','clip_stock','ifcopenshell','CONCRETE-BIM') if v in decoded.lower() or v in decoded],'parent_in_snapshot':row['parent'] in by_pid,'owned_script_names':[Path(arg.replace('\\','/')).name for arg in paths],'command_sha256':hashlib.sha256(command.encode()).hexdigest()})
    except Exception as error:p.update({'metadata_available':False,'error':str(error)})
    finally:k.CloseHandle(h)
    result.append(p)
(T/'🗑️generated/process-ticket-ownership-r92.json').write_text(json.dumps({'rows':result},indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({'rows':result}),flush=True)
