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
x/5gd $r8
x/5gd $r9
printf "[DEBUG] copy bytes %lu\n", $rcx
set $store_top=$pc
si
x/45i $pc
info functions minimum_advance_items
bt 5
