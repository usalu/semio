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
tbreak *0x55555d5b8420
continue
set $owner = $rsi
set $result = $rdi
printf "[DEBUG] phase, cancellation and close\n"
x/6ub $owner+0x5dc8
tbreak *0x55555d54af70
tbreak *0x55555d5546d0
tbreak *0x55555d5b8ad1
continue
bt 5
x/12gd $result
x/4gd $owner+0x5860
x/2gd $owner+0x5dc0
x/10i $pc
