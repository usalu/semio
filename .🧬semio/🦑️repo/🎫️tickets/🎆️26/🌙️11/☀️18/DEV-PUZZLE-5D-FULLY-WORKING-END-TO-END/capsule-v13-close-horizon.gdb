set confirm off
set pagination off
set debuginfod enabled off
set startup-with-shell off
set env RUST_MIN_STACK 67108864
starti
python
binary = gdb.current_progspace().filename
base = next(int(line.split()[0], 16) for line in gdb.execute("info proc mappings", to_string=True).splitlines() if line.strip().endswith(binary))
class CloseHorizon(gdb.FinishBreakpoint):
    def stop(self):
        before = int(gdb.parse_and_eval("$rax"))
        if before == 524288:
            gdb.execute("set $rax = 8388608", to_string=True)
            print("[DEBUG] Diagnostic Capsule close horizon 8388608 with unchanged original retirement stock", flush=True)
        return False
class Fixture(gdb.Breakpoint):
    def stop(self):
        CloseHorizon(gdb.newest_frame(), internal=True)
        return False
Fixture("*" + hex(base + 0xaeee820), internal=True)
class Closed(gdb.Breakpoint):
    def stop(self):
        counts = gdb.parse_and_eval("(unsigned long*)($rsp + 0x2c8)")
        turns, active, idle = (int((counts + index).dereference()) for index in range(3))
        print(f"[DEBUG] Original fixture close counters turns={turns} active={active} idle={idle}", flush=True)
        return False
for address in (0x652b0db, 0x652bd08):
    Closed("*" + hex(base + address), internal=True)
end
continue
