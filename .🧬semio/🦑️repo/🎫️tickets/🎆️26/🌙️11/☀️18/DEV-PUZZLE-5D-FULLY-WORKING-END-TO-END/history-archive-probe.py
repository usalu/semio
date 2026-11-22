import collections
import gdb
import struct

base = int(gdb.parse_and_eval("(uintptr_t)&main")) - 0x4448390
counts = collections.Counter()
history = collections.deque(maxlen=14)
archive = 0

def memory(address, length):
    return gdb.selected_inferior().read_memory(address, length).tobytes()

def words(address, length):
    return struct.unpack("<" + "Q" * length, memory(address, 8 * length))

def register(name):
    return int(gdb.parse_and_eval("$" + name))

class Pulse(gdb.Breakpoint):
    def __init__(self, name, offset):
        self.name = name
        super().__init__("*" + hex(base + offset), internal=True)
    def stop(self):
        global archive
        kind = self.name
        counts[kind] += 1
        count = counts[kind]
        if kind == "archive advance":
            archive = register("r8")
        if kind == "archive terminal" and (count <= 3 or count in [10, 50, 100, 500, 2000]):
            print("[DEBUG]", kind, count, "grant", words(register("rcx"), 5))
        if kind in ["archive fault", "initializer default refuse", "initializer normal refuse"]:
            print("[DEBUG]", kind, count)
            gdb.execute("bt 8")
        if kind in ["archive close", "envelope close"]:
            history.append((kind, register("rsi")))
        if kind == "envelope quote" and count in [10, 50, 100, 500]:
            print("[DEBUG]", kind, count, "owner", hex(register("rsi")), "body", register("rdx"))
            gdb.execute("bt 33")
        if kind == "envelope quote" and count == 550:
            print("[DEBUG] totals", counts, "last closes", list(history))
            return True
        return False

for name, offset in {
    "initializer drive": 0x580e080,
    "initializer normal retire": 0x5bae890,
    "initializer default retire": 0x5bbc720,
    "initializer normal refuse": 0x5bae750,
    "initializer default refuse": 0x5bbc5e0,
    "initializer normal effect": 0x5ba6e80,
    "initializer default effect": 0x5bb4ca0,
    "worker close": 0x58b5170,
    "initializer normal close": 0x5ca5fd0,
    "initializer default close": 0x5ca7580,
    "archive advance": 0x5e7c220,
    "archive terminal": 0x5e82fb0,
    "archive fault": 0x44bbbf0,
    "archive close": 0x5e7ace0,
    "envelope quote": 0x6d8c590,
    "envelope close": 0x6d9b160,
}.items():
    Pulse(name, offset)
