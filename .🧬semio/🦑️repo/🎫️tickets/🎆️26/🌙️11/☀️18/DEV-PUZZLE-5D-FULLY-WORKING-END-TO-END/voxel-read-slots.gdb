set pagination off
set debuginfod enabled off
set startup-with-shell off
set env RUST_MIN_STACK 67108864
starti
python
import gdb
program = gdb.current_progspace().filename
base = next(int(line.split()[0], 16) for line in gdb.execute('info proc mappings', to_string=True).splitlines() if line.strip().endswith(program))
gdb.write(f'[DEBUG] executable_base {base:#x}\n')
for entry in (0x8c4fa60, 0x48aeb20, 0x48af460, 0x8c503a0, 0x777da20, 0x777e360, 0x777eca0, 0x48afda0, 0x48b06e0):
    gdb.Breakpoint(f'*{base + entry + 0x184:#x}', internal=True)
end
continue
bt 34
info registers rax rdi rsi rsp
disassemble $pc-40, $pc+20
