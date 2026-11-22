set confirm off
set pagination off
set startup-with-shell off
set env RUST_MIN_STACK 67108864
starti
python
import gdb
program = gdb.current_progspace().filename
for line in gdb.execute("info proc mappings", to_string=True).splitlines():
    if line.strip().endswith(program):
        address = int(line.split()[0], 16)
        gdb.write(f"Position-independent executable base {address:#x}\n")
        for relative in [228990365]:
            gdb.Breakpoint(f"*{address + relative:#x}")
        break
else:
    raise RuntimeError("native test executable mapping not found")
end
continue
bt 48
x/18i $pc-32
