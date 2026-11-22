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
set $count=0
break *0x55555cbb2993
commands
silent
set $count=$count+1
if $count < 350
 continue
end
end
continue
x/22i $pc-65
info registers
x/50gx $rsp
bt 8
