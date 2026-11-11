"""🧹 Cancel the verified stale presentation lifecycle worker and its descendants."""
import ctypes
from ctypes import wintypes

kernel = ctypes.WinDLL("kernel32", use_last_error=True)
nt = ctypes.WinDLL("ntdll", use_last_error=True)
kernel.OpenProcess.restype = wintypes.HANDLE
kernel.CreateToolhelp32Snapshot.restype = wintypes.HANDLE
kernel.CloseHandle.argtypes = [wintypes.HANDLE]
kernel.TerminateProcess.argtypes = [wintypes.HANDLE, wintypes.UINT]
nt.NtQueryInformationProcess.argtypes = [wintypes.HANDLE, wintypes.ULONG, ctypes.c_void_p, wintypes.ULONG, ctypes.POINTER(wintypes.ULONG)]

class Entry(ctypes.Structure):
    _fields_ = [("size", wintypes.DWORD), ("usage", wintypes.DWORD), ("pid", wintypes.DWORD),
                ("heap", ctypes.c_size_t), ("module", wintypes.DWORD), ("threads", wintypes.DWORD),
                ("parent", wintypes.DWORD), ("priority", ctypes.c_long), ("flags", wintypes.DWORD),
                ("name", wintypes.WCHAR * 260)]

class UnicodeString(ctypes.Structure):
    _fields_ = [("length", wintypes.USHORT), ("maximum", wintypes.USHORT), ("buffer", ctypes.c_void_p)]

def command(pid):
    handle = kernel.OpenProcess(0x1000, False, pid)
    if not handle:
        raise OSError(ctypes.get_last_error())
    try:
        size = wintypes.ULONG()
        nt.NtQueryInformationProcess(handle, 60, None, 0, ctypes.byref(size))
        buffer = ctypes.create_string_buffer(size.value)
        status = nt.NtQueryInformationProcess(handle, 60, buffer, size, ctypes.byref(size))
        if status:
            raise OSError(status)
        value = UnicodeString.from_buffer(buffer)
        return ctypes.wstring_at(value.buffer, value.length // 2)
    finally:
        kernel.CloseHandle(handle)

kernel.Process32FirstW.argtypes = [wintypes.HANDLE, ctypes.POINTER(Entry)]
kernel.Process32NextW.argtypes = [wintypes.HANDLE, ctypes.POINTER(Entry)]
snapshot = kernel.CreateToolhelp32Snapshot(2, 0)
entry = Entry()
entry.size = ctypes.sizeof(entry)
parents = {}
ok = kernel.Process32FirstW(snapshot, ctypes.byref(entry))
while ok:
    parents[entry.pid] = entry.parent
    ok = kernel.Process32NextW(snapshot, ctypes.byref(entry))
kernel.CloseHandle(snapshot)
pid, parent = 50148, 42356
assert parents.get(pid) == parent
line, parent_line = command(pid), command(parent)
assert all(term in line for term in ("unittest", "grasshopper", "excel", "presentation"))
assert all(term in parent_line for term in ("unittest", "grasshopper", "excel", "presentation"))
children, frontier = [], [pid]
while frontier:
    frontier = [p for p, owner in parents.items() if owner in frontier]
    children.extend(frontier)
print("verified", pid, "parent", parent, "descendants", children, flush=True)
for target in [pid, *reversed(children)]:
    handle = kernel.OpenProcess(1 | 0x1000, False, target)
    if handle:
        if not kernel.TerminateProcess(handle, 1):
            raise OSError(ctypes.get_last_error())
        kernel.CloseHandle(handle)
print("terminated", [pid, *children], flush=True)
