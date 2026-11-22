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
ignore 3 1499
continue
tbreak *0x55555d54af70
continue
set $result = $rdi
printf "[DEBUG] local grant and body\n"
x/5gd $rcx
printf "[DEBUG] body %lu\n", $rdx
finish
printf "[DEBUG] local result\n"
x/12gd $result
