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
break *0x55555d5dce70
continue
set $result = $rdi
finish
printf "[DEBUG] Closed minimum item quote result\n"
x/8gd $result
disable 4
commands 3
silent
set *(unsigned long*)$r8 = *(unsigned long*)$r9
continue
end
continue
