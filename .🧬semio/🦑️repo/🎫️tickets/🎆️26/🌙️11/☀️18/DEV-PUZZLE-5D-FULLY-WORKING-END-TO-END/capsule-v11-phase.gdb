set confirm off
set pagination off
set startup-with-shell off
set env RUST_MIN_STACK 67108864
set env SEMIO_RUNTIME_DIAGNOSTICS 1
starti
python
import gdb

binary = gdb.current_progspace().filename
base = next(int(line.split()[0], 16) for line in gdb.execute("info proc mappings", to_string=True).splitlines() if line.strip().endswith(binary))
owner = 0

def value(address, length):
    return int.from_bytes(gdb.selected_inferior().read_memory(address, length), "little")

class Phase(gdb.Breakpoint):
    def __init__(self, address):
        super().__init__("*" + hex(address), gdb.BP_WATCHPOINT, internal=True)
    def stop(self):
        print("[DEBUG] Capsule v11 publication phase", value(owner + 0x3fa5, 1))
        gdb.execute("bt 3")
        return False

class Capture(gdb.Breakpoint):
    def stop(self):
        global owner
        owner = int(gdb.parse_and_eval("$rdx"))
        print("[DEBUG] Capsule v11 publication owner", hex(owner))
        Phase(owner + 0x3fa5)
        self.enabled = False
        return False

class Stall(gdb.Breakpoint):
    def stop(self):
        print("[DEBUG] Capsule v11 stalled publication phase", value(owner + 0x3fa5, 1))
        gdb.execute("x/20gx " + hex(owner + 0x3f30))
        return True

Capture("*" + hex(base + 0x5ec6eb0), internal=True)
Stall("*" + hex(base + 0x62cee80), internal=True)
end
continue
bt 12
kill
