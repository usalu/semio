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
python
import collections
import gdb
last = collections.deque(maxlen=8)
current = []
def reg(name):
    return int(gdb.parse_and_eval("$"+name))
def words(address, n):
    memory = gdb.selected_inferior().read_memory(address, n*8).tobytes()
    return [int.from_bytes(memory[i:i+8], "little") for i in range(0, n*8, 8)]
class Entry(gdb.Breakpoint):
    def stop(self):
        current[:] = [reg("rdi"), reg("rdx"), reg("rcx"), words(reg("r8"), 5)]
        return False
class Exit(gdb.Breakpoint):
    def stop(self):
        if current:
            last.append((current[2], current[3], words(current[0], 14)))
        return False
class Stop(gdb.Breakpoint):
    def stop(self):
        for turn in last:
            print("[DEBUG] latest device store turn", turn)
        return True
entry = Entry("*0x55555cbb298d", internal=True)
exit_ = Exit("*0x55555cbb2993", internal=True)
stop = Stop("*0x55555a109500", internal=True)
end
continue
bt 3
