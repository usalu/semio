set confirm off
set pagination off
set print frame-arguments none
set startup-with-shell off
set env RUST_MIN_STACK 67108864
starti
python
import os
binary = gdb.current_progspace().filename
base = next(int(line.split()[0], 16) for line in gdb.execute("info proc mappings", to_string=True).splitlines() if line.strip().endswith(binary))
inferior = gdb.selected_inferior()
def read(address, words):
    data = inferior.read_memory(address, 8*words)
    return tuple(int.from_bytes(data[8*i:8*(i+1)], "little") for i in range(words))
class LimitReturn(gdb.FinishBreakpoint):
    def stop(self):
        old = int(gdb.parse_and_eval("$rax"))
        assert old == 32768, old
        value = int(os.environ.get("SEMIO_WINDOW_TURN_DIAGNOSTIC", old))
        gdb.execute(f"set $rax = {value}")
        print("[DEBUG] Window fixture turn horizon", old, value, flush=True)
        return False
class Limit(gdb.Breakpoint):
    def stop(self):
        caller = gdb.newest_frame().older()
        if caller is not None and "restore_fixture_window_config" in (caller.name() or ""):
            LimitReturn(gdb.newest_frame(), internal=True)
        return False
class Seed(gdb.Breakpoint):
    count = 0
    def stop(self):
        self.count += 1
        if self.count in (1, 100, 500, 1000, 2000, 5000, 10000, 20000, 30000, 50000, 100000):
            stack = int(gdb.parse_and_eval("$rsp"))
            print("[DEBUG] seed admission", self.count, read(stack+0x150, 4), read(stack+0x170, 5), flush=True)
        return False
class Backing(gdb.Breakpoint):
    counts = {}
    def stop(self):
        owner = int(gdb.parse_and_eval("$rsi"))
        count = self.counts.get(owner, 0) + 1
        self.counts[owner] = count
        if count in (1, 64, 128, 256, 402, 1000, 5000, 20000):
            print("[DEBUG] actual causal backing callback", hex(owner), count, flush=True)
        return False
Limit("*" + hex(base + 0xaf1fa90), internal=True)
Seed("*" + hex(base + 0x83dda11), internal=True)
Backing("*" + hex(base + 0xe3acce0), internal=True)
end
continue
