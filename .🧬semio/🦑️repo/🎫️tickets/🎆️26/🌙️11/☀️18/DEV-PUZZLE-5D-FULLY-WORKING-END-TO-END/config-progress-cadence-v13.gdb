set confirm off
set pagination off
set debuginfod enabled off
set startup-with-shell off
set env RUST_MIN_STACK 67108864
starti
python
from collections import Counter
binary = gdb.current_progspace().filename
base = next(int(line.split()[0], 16) for line in gdb.execute("info proc mappings", to_string=True).splitlines() if line.strip().endswith(binary))
inferior = gdb.selected_inferior()
decisions = Counter()
map_steps = Counter()
previous = None
repeated = 0

def read(address, width=8):
    return int.from_bytes(inferior.read_memory(address, width), "little")

class PreviewCadence(gdb.Breakpoint):
    def stop(self):
        owner = int(gdb.parse_and_eval("$rdi"))
        stage = read(owner + 0x2f6a, 1)
        part = read(owner + 0x2f38)
        grip = read(owner + 0x2f40)
        inserting = read(owner, 16) != (1 << 128) - 1
        if stage == 1 and part % 16 != 0 or stage == 2 and (grip != 0 or part % 16 != 0) or stage in (5, 6) and inserting:
            gdb.execute("set $al = 0", to_string=True)
            decisions["deferred"] += 1
        else:
            decisions["emitted"] += 1
        return False

PreviewCadence("*" + hex(base + 0x69b48d4), internal=True)

class WorkerBudget(gdb.Breakpoint):
    def stop(self):
        if int(gdb.parse_and_eval("$rdx")) == 8:
            gdb.execute("set $rdx = 16", to_string=True)
        return False

WorkerBudget("*" + hex(base + 0x46d11a0), internal=True)

class ExactMovement(gdb.Breakpoint):
    def stop(self):
        grant = int(gdb.parse_and_eval("$rcx"))
        quoted = int(gdb.parse_and_eval("$rax"))
        granted = read(grant + 0x38)
        if quoted > granted:
            decisions["exact movement loan"] += 1
            print(f"[DEBUG] Weight insertion quotes {quoted} moved rows against {granted}, using exact local movement while retaining the physical turn", flush=True)
            gdb.execute(f"set *(unsigned long*){grant + 0x38:#x} = {quoted}", to_string=True)
        return False

ExactMovement("*" + hex(base + 0x6ee1529), internal=True)

class MapAdvance(gdb.Breakpoint):
    def stop(self):
        global previous, repeated
        cursor = int(gdb.parse_and_eval("$rsi"))
        if cursor == 0:
            return False
        phase = read(cursor + 0x2c12, 1)
        input_phase = read(cursor + 0x2761, 1)
        key = phase, input_phase
        repeated = repeated + 1 if key == previous else 1
        previous = key
        map_steps[key] += 1
        if map_steps[key] in (1, 32, 128, 512):
            print(f"[DEBUG] Weight map insertion phase={phase}, input_phase={input_phase}, steps={map_steps[key]}, consecutive={repeated}", flush=True)
        return repeated == 512

MapAdvance("*" + hex(base + 0x6edd150), internal=True)

class MapWorkQuote(gdb.Breakpoint):
    def stop(self):
        requested = int(gdb.parse_and_eval("$rax"))
        available = read(int(gdb.parse_and_eval("$rsp")) + 0x60)
        if requested > available:
            print(f"[DEBUG] Weight map next work requires {requested} with {available} available", flush=True)
            return True
        return False

MapWorkQuote("*" + hex(base + 0x6ee4d26), internal=True)
end
continue
thread apply all bt 20
python
print(f"[DEBUG] Weight frame cadence diagnostic {dict(decisions)}", flush=True)
end
