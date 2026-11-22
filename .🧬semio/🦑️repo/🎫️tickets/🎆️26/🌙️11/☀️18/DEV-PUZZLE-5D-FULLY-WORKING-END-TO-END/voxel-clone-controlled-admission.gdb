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
tbreak *0x555562a52b59
continue
printf "[DEBUG] Controlled original virtual method\n"
info symbol $rax
si
bt 4
finish
x/12i $pc
