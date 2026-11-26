"""📏 Record host memory and warden activity while an owned Draw build is alive."""
import json
import os
from pathlib import Path
import subprocess
import sys
import time

def process_alive(pid):
    if os.name == "nt":
        from ctypes import WinDLL
        from ctypes.wintypes import BOOL, DWORD, HANDLE
        api = WinDLL("kernel32", use_last_error=True)
        api.OpenProcess.argtypes = [DWORD, BOOL, DWORD]
        api.OpenProcess.restype = HANDLE
        api.WaitForSingleObject.argtypes = [HANDLE, DWORD]
        api.WaitForSingleObject.restype = DWORD
        api.CloseHandle.argtypes = [HANDLE]
        handle = api.OpenProcess(0x00100000, False, pid)
        if not handle:
            return False
        try:
            return api.WaitForSingleObject(handle, 0) == 258
        finally:
            api.CloseHandle(handle)
    try:
        os.kill(pid, 0)
    except ProcessLookupError:
        return False
    return True


pid = int(sys.argv[1])
destination = Path(sys.argv[2])
with destination.open("x", encoding="utf-8") as stream:
    while process_alive(pid):
        try:
            probe = subprocess.run(["repo", "capacity", "status", "--json"], capture_output=True, text=True, timeout=15, check=True)
            status = json.loads(probe.stdout)
            host, warden = status["host"], status["warden"]
            sample = {key: host[key] for key in ("atMs", "availableMegabytes", "pressure", "swapOutMegabytesPerSecond")}
            sample.update(wardenLevel=warden["level"], counters=warden["counters"])
        except (OSError, ValueError, KeyError, subprocess.SubprocessError) as error:
            sample = {"sampleError": str(error), "atMs": int(time.time() * 1000)}
        print(json.dumps(sample, separators=(",", ":")), file=stream, flush=True)
        time.sleep(15)
