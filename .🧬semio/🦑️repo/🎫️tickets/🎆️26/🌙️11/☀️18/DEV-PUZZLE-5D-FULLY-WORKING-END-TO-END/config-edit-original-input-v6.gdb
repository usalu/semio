set pagination off
set debuginfod enabled off
set startup-with-shell off
set print thread-events off
set env RUST_MIN_STACK 67108864
starti
python
import gdb
program = gdb.current_progspace().filename
base = next(int(line.split()[0], 16) for line in gdb.execute('info proc mappings', to_string=True).splitlines() if line.strip().endswith(program))
gdb.Breakpoint(f'*{base + 0x7100cd0:#x}', internal=True)
end
continue
bt 15
up
x/18i $pc-30
info registers rax rbx rcx rdx rsi rdi rsp rbp
x/30gx $rsp
up
x/14i $pc-26
