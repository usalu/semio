#!/usr/bin/env python3
"""🔒️ Landlock launcher of the read oracle: restricts this process to the READ_FILE grants of a plan, then replaces itself with the command, so every child inherits the restriction.
usage: 🐍️.py --probe                 prints the Landlock ABI version (exit 0) or the reason it is unavailable (exit 125)
       🐍️.py <plan.json> -- cmd...   plan = {"read": ["/absolute/path", ...]}: files and directory trees that stay readable; nothing else can be read (directory listings stay allowed)
Unprivileged, no helper binary: only the Python standard library and the kernel (Linux 5.13+ with Landlock enabled)."""
import ctypes, json, os, struct, sys

READ_FILE = 1 << 2
CREATE_RULESET, ADD_RULE, RESTRICT_SELF, VERSION_FLAG, PATH_BENEATH, NO_NEW_PRIVS = 444, 445, 446, 1, 1, 38
UNAVAILABLE = 125


def refuse(text):
    sys.stderr.write(f"landlock-unavailable: {text}\n")
    sys.exit(UNAVAILABLE)


if not sys.platform.startswith("linux"):
    refuse(f"needs Linux, this is {sys.platform}")
libc = ctypes.CDLL(None, use_errno=True)
libc.syscall.restype = ctypes.c_long
libc.prctl.argtypes = [ctypes.c_int, ctypes.c_ulong, ctypes.c_ulong, ctypes.c_ulong, ctypes.c_ulong]


def kernel(number, *args):
    values = [ctypes.c_long(number)] + [arg if isinstance(arg, ctypes._SimpleCData) else ctypes.c_long(arg) for arg in args]
    result = libc.syscall(*values)
    return result, ctypes.get_errno()


abi, error = kernel(CREATE_RULESET, ctypes.c_void_p(None), 0, VERSION_FLAG)
if abi < 1:
    refuse(f"landlock_create_ruleset version probe failed: {os.strerror(error)}")
if sys.argv[1:] == ["--probe"]:
    print(abi)
    sys.exit(0)
if len(sys.argv) < 4 or sys.argv[2] != "--":
    sys.exit("usage: 🐍️.py --probe | 🐍️.py <plan.json> -- command...")
with open(sys.argv[1], encoding="utf-8") as handle:
    plan = json.load(handle)
attribute = ctypes.create_string_buffer(struct.pack("=Q", READ_FILE), 8)
ruleset, error = kernel(CREATE_RULESET, ctypes.c_void_p(ctypes.addressof(attribute)), 8, 0)
if ruleset < 0:
    refuse(f"landlock_create_ruleset failed: {os.strerror(error)}")
granted = 0
for path in plan["read"]:
    try:
        descriptor = os.open(path, os.O_PATH | os.O_CLOEXEC)
    except OSError:
        continue
    rule = ctypes.create_string_buffer(struct.pack("=Qi", READ_FILE, descriptor), 12)
    result, error = kernel(ADD_RULE, ruleset, PATH_BENEATH, ctypes.c_void_p(ctypes.addressof(rule)), 0)
    os.close(descriptor)
    if result < 0:
        refuse(f"landlock_add_rule failed for {path}: {os.strerror(error)}")
    granted += 1
if libc.prctl(NO_NEW_PRIVS, 1, 0, 0, 0) != 0:
    refuse("prctl(PR_SET_NO_NEW_PRIVS) failed")
result, error = kernel(RESTRICT_SELF, ruleset, 0)
if result < 0:
    refuse(f"landlock_restrict_self failed: {os.strerror(error)}")
os.close(ruleset)
os.execvp(sys.argv[3], sys.argv[3:])
