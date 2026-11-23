import ctypes, sys, re
from ctypes import wintypes
exe, log = sys.argv[1], sys.argv[2]
lo, hi = int(sys.argv[3]), int(sys.argv[4])
text = open(log, encoding='utf-8', errors='replace').read()
frames = [(int(m.group(1)), int(m.group(2), 16)) for m in re.finditer(r'^\s*(\d+):\s+0x([0-9a-f]+)', text, re.M)]
dbg = ctypes.WinDLL('dbghelp.dll')
dbg.SymInitialize.argtypes = [wintypes.HANDLE, wintypes.LPCSTR, wintypes.BOOL]
dbg.SymSetOptions(0x2 | 0x10)
h = ctypes.c_void_p(0x1234)
assert dbg.SymInitialize(h, None, False)
dbg.SymLoadModuleEx.restype = ctypes.c_uint64
dbg.SymLoadModuleEx.argtypes = [wintypes.HANDLE, wintypes.HANDLE, wintypes.LPCSTR, wintypes.LPCSTR, ctypes.c_uint64, wintypes.DWORD, ctypes.c_void_p, wintypes.DWORD]
size = 0x20000000
BASE = 0x140000000
base = dbg.SymLoadModuleEx(h, None, exe.encode(), None, BASE, size, None, 0)
assert base, ctypes.get_last_error()
class SYM(ctypes.Structure):
    _fields_ = [('SizeOfStruct', wintypes.ULONG), ('TypeIndex', wintypes.ULONG), ('Reserved', ctypes.c_uint64 * 2), ('Index', wintypes.ULONG), ('Size', wintypes.ULONG), ('ModBase', ctypes.c_uint64), ('Flags', wintypes.ULONG), ('Value', ctypes.c_uint64), ('Address', ctypes.c_uint64), ('Register', wintypes.ULONG), ('Scope', wintypes.ULONG), ('Tag', wintypes.ULONG), ('NameLen', wintypes.ULONG), ('MaxNameLen', wintypes.ULONG), ('Name', ctypes.c_char * 8192)]
def lookup(addr):
    s = SYM()
    s.SizeOfStruct = 88
    s.MaxNameLen = 8192
    disp = ctypes.c_uint64()
    if dbg.SymFromAddr(h, ctypes.c_uint64(addr), ctypes.byref(disp), ctypes.byref(s)):
        return s.Name.decode(errors='replace')
    return None
ip8 = [ip for n, ip in frames if n == 12][0]
minip = min(ip for _, ip in frames)
found = None
for rva_guess in range(lo, hi, 0x10000):
    b = (ip8 & ~0xFFFF) - rva_guess
    name = lookup(BASE + (ip8 - b))
    if name and 'panicking' in name and 'end_short_backtrace' in name:
        found = b
        break
print('base', hex(found) if found else None)
if found:
    for n, ip in frames:
        if n >= int(sys.argv[5]) and n <= int(sys.argv[6]):
            print(n, lookup(BASE + (ip - found)))
