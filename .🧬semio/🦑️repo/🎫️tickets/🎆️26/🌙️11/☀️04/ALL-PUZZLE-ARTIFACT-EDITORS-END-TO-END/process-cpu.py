import ctypes
import sys
import time
from ctypes import wintypes

kernel = ctypes.WinDLL("kernel32", use_last_error=True)
kernel.OpenProcess.restype = wintypes.HANDLE
kernel.OpenProcess.argtypes = [wintypes.DWORD, wintypes.BOOL, wintypes.DWORD]
kernel.GetProcessTimes.argtypes = [wintypes.HANDLE] + [ctypes.POINTER(wintypes.FILETIME)] * 4
kernel.CloseHandle.argtypes = [wintypes.HANDLE]
class IoCounters(ctypes.Structure):
    _fields_ = [(name, ctypes.c_ulonglong) for name in ("read_ops", "write_ops", "other_ops", "read_bytes", "write_bytes", "other_bytes")]
kernel.GetProcessIoCounters.argtypes = [wintypes.HANDLE, ctypes.POINTER(IoCounters)]
handle = kernel.OpenProcess(0x1000, False, int(sys.argv[1]))
if not handle:
    raise ctypes.WinError(ctypes.get_last_error())
def total():
    times = [wintypes.FILETIME() for _ in range(4)]
    if not kernel.GetProcessTimes(handle, *(ctypes.byref(value) for value in times)):
        raise ctypes.WinError(ctypes.get_last_error())
    return sum((value.dwHighDateTime << 32) | value.dwLowDateTime for value in times[2:])
def io():
    counters = IoCounters()
    if not kernel.GetProcessIoCounters(handle, ctypes.byref(counters)):
        raise ctypes.WinError(ctypes.get_last_error())
    return [getattr(counters, name) for name, _ in counters._fields_]
first = total()
before = io()
time.sleep(3)
second = total()
after = io()
kernel.CloseHandle(handle)
print(f"pid={sys.argv[1]} cpu_delta_seconds={(second - first) / 10_000_000:.3f} sample_seconds=3")
print(dict(zip((name for name, _ in IoCounters._fields_), (end - start for start, end in zip(before, after)))))
