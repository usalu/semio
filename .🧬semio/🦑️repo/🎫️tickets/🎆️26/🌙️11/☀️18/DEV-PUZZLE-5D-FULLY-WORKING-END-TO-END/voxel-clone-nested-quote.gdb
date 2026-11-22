set pagination off
set debuginfod enabled off
set environment RUST_MIN_STACK 268435456
rbreak retain_original_ownership_receipts
run
disable 1
set $receipt_rejected = $pc + 1153
break *$receipt_rejected
commands
silent
set $pc = $receipt_rejected + 54
continue
end
break *0x55555cbb298d
ignore 3 349
continue
tbreak *0x55555d54af70
continue
set $result = $rdi
set $effect = $r8
python
import gdb
import re
def words(address, n):
    memory = gdb.selected_inferior().read_memory(address, n*8).tobytes()
    return [int.from_bytes(memory[i:i+8], "little") for i in range(0, n*8, 8)]
class Probe(gdb.Breakpoint):
    def stop(self):
        tag = re.sub(r"::<.*", "", self.name)
        frame = gdb.newest_frame().older()
        print("[DEBUG]", tag, hex(int(frame.pc())), frame.name().split("::")[-1])
        if "grant_admits" in self.name:
            try:
                print("grant", words(int(gdb.parse_and_eval("$rdi")), 5), "demand", words(int(gdb.parse_and_eval("$rsi")), 4))
            except Exception:
                pass
        return False
probes = []
for name in ("retained_clone_grant_admits", "advance_reply"):
    for line in gdb.execute("info functions " + name, to_string=True).splitlines():
        match = re.match(r"(0x[0-9a-fA-F]+)\s+(.*)", line)
        if match and ("grant_admits" in match[2] or "WorldWindowConfig" in match[2]) and "closure" not in match[2]:
            p = Probe("*" + match[1], internal=True)
            p.name = match[2]
            probes.append(p)
end
finish
x/10gd $result
