set pagination off
set debuginfod enabled off
set startup-with-shell off
set env RUST_MIN_STACK 67108864
starti
python
import gdb

program = gdb.current_progspace().filename
base = next(int(line.split()[0], 16) for line in gdb.execute('info proc mappings', to_string=True).splitlines() if line.strip().endswith(program))
entry = gdb.Breakpoint(f'*{base + 0x587c690:#x}', internal=True)
entry.ignore_count = 28
end
continue
set $output = $rdi + 8
set *(unsigned long*)$output = 999
python entry.enabled = False
watch -location *(unsigned long*)$output
continue
x/16i $pc-32
bt 8
