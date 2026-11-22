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
tbreak *0x555562a51b20
continue
python
import gdb
frame = gdb.newest_frame()
base = 0x555562a51780
for _ in range(220):
    now=gdb.newest_frame()
    if not now.name() or "ControlledCore>::step_original" not in now.name():
        print("[DEBUG] left function at", now.name())
        break
    instruction=now.architecture().disassemble(now.pc(), count=1)[0]["asm"]
    if instruction.startswith(("call","j","ret")):
        print("[DEBUG]", hex(now.pc()-base), instruction)
    gdb.execute("nexti", to_string=True)
end
bt 4
