set confirm off
set pagination off
set print frame-arguments none
python
import os
want = set(os.environ["SEMIO_5D_ORIGINAL_REFUSAL"].split("|"))
class Refusal(gdb.Breakpoint):
    def stop(self):
        address = int(gdb.parse_and_eval("$rdx"))
        count = int(gdb.parse_and_eval("$rcx"))
        if not 0 < count < 1024:
            return False
        try:
            message = bytes(gdb.selected_inferior().read_memory(address, count)).decode()
        except (gdb.MemoryError, UnicodeError):
            return False
        if message not in want:
            return False
        print("[DEBUG] Native original refusal", message)
        gdb.execute("bt 33")
        return True
Refusal("'<semio_framework_value_control_physical::ValueError>::literal'")
end
run
frame 3
x/24i $pc-48
kill
