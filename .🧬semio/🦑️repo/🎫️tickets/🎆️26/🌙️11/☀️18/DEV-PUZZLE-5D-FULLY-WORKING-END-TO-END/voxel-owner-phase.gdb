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
python
import gdb
import re
for name in ("advance_with_turn_ceiling", "advance_local", "seal_turn", "preparation_work_demand", "snapshot_clone_turn_admitted"):
    for line in gdb.execute("info functions " + name, to_string=True).splitlines():
        match = re.match(r"(0x[0-9a-fA-F]+)\s+(.*)", line)
        if match and ("WorldWindowConfig" in match[2] and "RetainedClonePreparation" in match[2] or "snapshot_clone_turn_admitted" in match[2] and "WorldWindowConfig" in match[2]) and "closure" not in match[2]:
            print("[DEBUG]",name,match[1])
            if name == "advance_with_turn_ceiling":
                gdb.execute("tbreak *"+match[1])
end
continue
set $prep = $rsi
x/125i $pc
x/12gx $prep
bt 3
