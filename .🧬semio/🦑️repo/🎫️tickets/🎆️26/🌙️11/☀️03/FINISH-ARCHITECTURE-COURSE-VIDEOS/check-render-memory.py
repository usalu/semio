"""🧮 Read renderer resident and peak working sets without starting a heavy probe."""
import ctypes
from ctypes import wintypes

class Counters(ctypes.Structure):
    _fields_ = [("cb", wintypes.DWORD), ("page_faults", wintypes.DWORD), ("peak_working_set", ctypes.c_size_t),
                ("working_set", ctypes.c_size_t), ("peak_paged_pool", ctypes.c_size_t), ("paged_pool", ctypes.c_size_t),
                ("peak_nonpaged_pool", ctypes.c_size_t), ("nonpaged_pool", ctypes.c_size_t),
                ("pagefile", ctypes.c_size_t), ("peak_pagefile", ctypes.c_size_t), ("private", ctypes.c_size_t)]

kernel, psapi = ctypes.WinDLL("kernel32", use_last_error=True), ctypes.WinDLL("psapi", use_last_error=True)
kernel.OpenProcess.restype = wintypes.HANDLE
for pid in (35216, 62844, 58560):
    handle = kernel.OpenProcess(0x1000 | 0x10, False, pid)
    if not handle:
        print(pid, "gone")
        continue
    counters = Counters()
    counters.cb = ctypes.sizeof(counters)
    if psapi.GetProcessMemoryInfo(handle, ctypes.byref(counters), counters.cb):
        print(pid, round(counters.working_set / 2**30, 2), round(counters.peak_working_set / 2**30, 2))
    else:
        print(pid, "unavailable")
    kernel.CloseHandle(handle)
