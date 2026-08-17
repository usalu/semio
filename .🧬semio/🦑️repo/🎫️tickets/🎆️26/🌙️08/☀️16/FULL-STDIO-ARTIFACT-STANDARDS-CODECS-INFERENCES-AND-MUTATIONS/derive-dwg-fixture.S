.text
.globl _call_rust_sret0
_call_rust_sret0:
    mov x8, x0
    br x1

.globl _call_rust_sret1
_call_rust_sret1:
    mov x8, x0
    mov x0, x1
    br x2
