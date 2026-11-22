set confirm off
set pagination off
set debuginfod enabled off
set startup-with-shell off
set env RUST_MIN_STACK 67108864
starti
python
from collections import Counter
import os
import signal
from threading import Timer
binary = gdb.current_progspace().filename
base = next(int(line.split()[0], 16) for line in gdb.execute("info proc mappings", to_string=True).splitlines() if line.strip().endswith(binary))
inferior = gdb.selected_inferior()
current = {}
outcomes = Counter()
return_admission = Counter()
semantic_steps = Counter()
folds = 0
consecutive = 0

def register(name):
    return int(gdb.parse_and_eval("$" + name))

def value(address, width=8):
    return int.from_bytes(inferior.read_memory(address, width), "little")

class Advance(gdb.Breakpoint):
    def stop(self):
        current["output"], current["publication"], current["body"] = (register(name) for name in ("rdi", "rdx", "rcx"))
        return False

class Fold(gdb.Breakpoint):
    def stop(self):
        global folds
        folds += 1
        return False

class Outcome(gdb.Breakpoint):
    def stop(self):
        global consecutive
        buffer = bytes(inferior.read_memory(current["output"], 112))
        tag = buffer[8]
        key = (tag, value(current["publication"] + 0x3fa5, 1), value(current["publication"] + 0x3f90))
        consecutive = consecutive + 1 if outcomes and key == current.get("key") else 1
        outcomes[key] += 1
        current["key"] = key
        if tag == 3 and outcomes[key] == 2:
            current["two_publications"] = True
            timer = Timer(12, lambda: os.kill(inferior.pid, signal.SIGTRAP))
            timer.daemon = True
            timer.start()
        if consecutive in (32, 128, 512, 1024, 4096):
            receipt = tuple(int.from_bytes(buffer[start:start + 8], "little") for start in range(64, 104, 8))
            print(f"[DEBUG] Config original publication after {sum(outcomes.values())} turns, folds={folds}, key={key}, streak={consecutive}, body={current['body']}, receipt={receipt}, outcomes={dict(outcomes)}", flush=True)
        return consecutive == 1024 and key[2] == 0

def sample_return(reason):
    if not current.get("two_publications"):
        return False
    return_admission[reason] += 1
    count = return_admission[reason]
    if count in (1, 8, 32, 128, 512, 1024):
        print(f"[DEBUG] Config next worker return admission {reason}, hit={count}, semantic_steps={dict(semantic_steps)}", flush=True)
    return count == 128 and reason != "source turn admitted"

class ReturnOrigin(gdb.Breakpoint):
    def stop(self):
        output = register("rax")
        result = value(output)
        reason = ("source turn admitted" if value(output + 8, 1) else "source turn pending") if result == (1 << 64) - 3 else "source turn refused"
        return sample_return(reason)

class Claim(gdb.Breakpoint):
    def stop(self):
        reason = bytes(inferior.read_memory(register("rsi"), register("rdx"))).decode("utf8")
        return sample_return(reason)

def row_counts(table):
    return dict(Counter(bytes(inferior.read_memory(table + 0x1730, 32))))

class Capacity(gdb.Breakpoint):
    def stop(self):
        if not current.get("two_publications"):
            return False
        table = register("rdi")
        counts = row_counts(table)
        if 0 not in counts and "full_table" not in current:
            current["full_table"] = table
            print(f"[DEBUG] Config Source table is full, rows={counts}", flush=True)
        return False

class TableChange(gdb.Breakpoint):
    def __init__(self, offset, kind):
        super().__init__("*" + hex(base + offset), internal=True)
        self.kind = kind

    def stop(self):
        table = register("rsi")
        if table != current.get("full_table"):
            return False
        key = self.kind, row_counts(table).get(0, 0)
        return_admission[key] += 1
        if return_admission[key] in (1, 8):
            print(f"[DEBUG] Config return table before {self.kind}, rows={row_counts(table)}", flush=True)
            gdb.execute("bt 8")
        return False

class Semantic(gdb.Breakpoint):
    def stop(self):
        owner = register("rsi")
        tool = bytes(inferior.read_memory(value(owner + 0x2c40), value(owner + 0x2c48))).decode("utf8")
        semantic_steps[tool] += 1
        if semantic_steps[tool] in (1, 32, 128, 256, 512, 1024):
            tail = tuple(value(owner + offset) for offset in range(0x2c40, 0x2c78, 8))
            print(f"[DEBUG] Config semantic step tool={tool}, hit={semantic_steps[tool]}, tail={tail}", flush=True)
        return False

Advance("*" + hex(base + 0x4f7f6e0), internal=True)
Outcome("*" + hex(base + 0x4f7f784), internal=True)
Fold("*" + hex(base + 0x5efc3c0), internal=True)
ReturnOrigin("*" + hex(base + 0xe108ba4), internal=True)
Claim("*" + hex(base + 0xe0f5b90), internal=True)
Capacity("*" + hex(base + 0xe1dea00), internal=True)
TableChange(0xe1de550, "reclaim")
TableChange(0xe1dffd0, "reserve")
Semantic("*" + hex(base + 0x69ad750), internal=True)
end
continue
thread apply all bt 28
