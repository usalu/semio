"""🧵️ Samples crate names and CPU time of one governed macOS compiler tree."""

import ctypes as C
from pathlib import Path
import re
import struct
import sys
import time

pid = int(sys.argv[1])
proc = C.CDLL("/usr/lib/libproc.dylib", use_errno=True)
libc = C.CDLL(None, use_errno=True)
clock_scale = (C.c_uint32 * 2)()
assert libc.mach_timebase_info(C.byref(clock_scale)) == 0
nanoseconds_per_tick = clock_scale[0] / clock_scale[1]

class Task(C.Structure):
    _fields_ = [(k, C.c_uint64) for k in ["virtual", "resident", "user", "system", "thread_user", "thread_system"]] + [(k, C.c_int32) for k in ["policy", "faults", "pageins", "cow", "sent", "received", "unix", "mach", "switches", "threads", "running", "priority"]]

def small_argv(pid):
    mib = (C.c_int * 3)(1, 49, pid)
    buf = C.create_string_buffer(262144)
    size = C.c_size_t(len(buf))
    if libc.sysctl(mib, 3, buf, C.byref(size), None, 0) != 0:
        return []
    data = buf.raw[:size.value]
    argc = struct.unpack_from("=i", data)[0]
    if argc <= 0:
        return []
    after_path = data.index(b"\0", 4)
    start = after_path + 1
    while data[start] == 0:
        start += 1
    return [part.decode("utf-8", "replace") for part in data[start:].split(b"\0", argc)[:argc]]

def sample():
    todo = [pid]
    visited = set()
    out = {}
    while todo:
        parent = todo.pop()
        if parent in visited:
            continue
        visited.add(parent)
        child_pids = (C.c_int * 2048)()
        proc.proc_listchildpids(parent, child_pids, C.sizeof(child_pids))
        todo += [child for child in child_pids if child > 0]
        a = small_argv(parent)
        if "--crate-name" not in a:
            continue
        crate = a[a.index("--crate-name") + 1]
        binary = a[0].rsplit("/", 1)[-1]
        t = Task()
        if proc.proc_pidinfo(parent, 4, 0, C.byref(t), C.sizeof(t)) == C.sizeof(t):
            out[parent] = (crate, binary, (t.user + t.system) * nanoseconds_per_tick / 1e9, t.resident // 2**20, t.running)
    return out

def owner_tree(start):
    def show(current, depth, relation):
        args = small_argv(current)
        if not args:
            return
        wanted = re.compile(r"(?:task|run|build|test|graph-plan|activate|serve|wasm|component|baseline|browser|stdio|\.ts$|\.mjs$)", re.I)
        safe = re.compile(r"^[a-zA-Z0-9@/_.:=,+*%\-]+$")
        task_target = re.compile(r"^(?:@?[a-zA-Z0-9_.-]+(?:/[a-zA-Z0-9_.-]+)?):[a-zA-Z0-9_.-]+$")
        keys = ("--projects=", "--targets=", "--target=", "--configuration=", "--operation=", "--jobs=", "--parallel=")
        tokens = []
        for arg in args[1:]:
            if len(arg) > 200 or not safe.fullmatch(arg):
                continue
            if re.search(r"secret|token|password|cookie|authorization|credential", arg, re.I):
                continue
            if arg.startswith(keys) or task_target.fullmatch(arg) or not arg.startswith("-") and wanted.search(arg):
                tokens.append(arg.rsplit("/", 1)[-1] if arg.endswith((".ts", ".mjs", ".js")) else arg)
        info = C.create_string_buffer(256)
        parent = None
        if proc.proc_pidinfo(current, 3, 0, info, C.sizeof(info)) >= 20:
            parent = struct.unpack_from("=5I", info.raw)[4]
        task = Task()
        resident = None
        cpu = None
        if proc.proc_pidinfo(current, 4, 0, C.byref(task), C.sizeof(task)) == C.sizeof(task):
            resident = task.resident // 2**20
            cpu = round((task.user + task.system) * nanoseconds_per_tick / 1e9, 2)
        markers = {m for m in ("pdf", "stdio", "playground", "browser", "activate", "serve", "artifact", "component", "norm-din4108", "repo-mcp", "wasm", "report", "graph", "catalog", "workspace") if any(m in arg.lower() for arg in args[1:])}
        source = next((Path(arg).name for arg in args[1:] if len(arg) <= 240 and (arg.startswith("./") or arg.startswith("/Users/ueli/Documents/semio/")) and Path(arg).is_file()), None)
        short = [arg for arg in args[1:] if re.fullmatch(r"(?:-?[a-z]{1,10}|[0-9]{1,4})", arg) and arg not in {"secret", "token", "password"}]
        shape = [(i, len(a), a.split("=", 1)[0] if a.startswith("--") and re.fullmatch(r"--[a-zA-Z0-9_-]+", a.split("=", 1)[0]) else None) for i, a in enumerate(args)][:12]
        print(f"[DEBUG] {relation} pid={current} parent={parent} executable={args[0].rsplit('/', 1)[-1]} rss_mib={resident} cpu_seconds={cpu} visible_targets={tokens[:12]} known_markers={sorted(markers)} local_source={source} argv_shape={shape} short_positional={short[:12]}")
        if depth > 0:
            children = (C.c_int * 2048)()
            proc.proc_listchildpids(current, children, C.sizeof(children))
            for child in children:
                if child > 0:
                    show(child, depth - 1, relation + " child")
        return parent
    parent = show(start, 4, "owner")
    for _ in range(3):
        if not parent or parent <= 1:
            return
        parent = show(parent, 0, "ancestor")

if "--owners" in sys.argv:
    owner_tree(pid)
else:
    before = sample()
    time.sleep(3)
    for process, (crate, binary, cpu, rss, running) in sorted(sample().items()):
        delta = cpu - before.get(process, (None, None, cpu, None, None))[2]
        print(f"[DEBUG] PID {process} {binary} {crate} · CPU {cpu:.2f}s (+{delta:.2f}s/3s) · RSS {rss} MiB · runnable threads {running}")
