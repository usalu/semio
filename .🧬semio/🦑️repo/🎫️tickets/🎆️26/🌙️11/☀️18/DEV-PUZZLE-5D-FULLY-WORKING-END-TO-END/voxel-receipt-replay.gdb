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
printf "[DEBUG] preparation grant and actual receipt\n"
x/5gd $rsp+0x290
x/4gd $rsp+0x2b8
set $pc = $receipt_rejected + 54
continue
end
continue
