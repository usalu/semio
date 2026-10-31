"""🕵️ Probe only known owned process IDs without sending any signal on Windows."""
import ctypes
from ctypes import wintypes
from datetime import datetime,timezone
import json
import os
from pathlib import Path
PIDS=(38336,39684,14812)
def alive(pid):
    """🔬 Read native process exit state; POSIX uses its non-destructive signal-zero query."""
    if os.name!='nt':
        try:
            os.kill(pid,0)
            return True
        except ProcessLookupError:
            return False
    kernel=ctypes.WinDLL('kernel32',use_last_error=True)
    kernel.OpenProcess.argtypes=(wintypes.DWORD,wintypes.BOOL,wintypes.DWORD)
    kernel.OpenProcess.restype=wintypes.HANDLE
    kernel.GetExitCodeProcess.argtypes=(wintypes.HANDLE,ctypes.POINTER(wintypes.DWORD))
    kernel.GetExitCodeProcess.restype=wintypes.BOOL
    kernel.CloseHandle.argtypes=(wintypes.HANDLE,)
    handle=kernel.OpenProcess(0x1000,False,pid)
    if not handle:
        error=ctypes.get_last_error()
        if error==87:
            return False
        raise ctypes.WinError(error)
    try:
        code=wintypes.DWORD()
        if not kernel.GetExitCodeProcess(handle,ctypes.byref(code)):
            raise ctypes.WinError(ctypes.get_last_error())
        return code.value==259
    finally:
        kernel.CloseHandle(handle)
record={'at':datetime.now(timezone.utc).isoformat(),'processes':{str(pid):{'alive':alive(pid)} for pid in PIDS}}
(Path(__file__).resolve().parent/'🗑️generated/owned-process-exit-check.json').write_text(json.dumps(record,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] '+json.dumps(record),flush=True)
