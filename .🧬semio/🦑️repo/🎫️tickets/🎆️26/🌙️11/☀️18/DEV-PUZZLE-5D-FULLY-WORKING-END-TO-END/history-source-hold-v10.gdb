set confirm off
set pagination off
set print frame-arguments none
python
class OriginalRefusal(gdb.Breakpoint):
    seen = set()

    def stop(self):
        address = int(gdb.parse_and_eval("$rdx"))
        length = int(gdb.parse_and_eval("$rcx"))
        if not 0 < length < 1024:
            return False
        try:
            message = bytes(gdb.selected_inferior().read_memory(address, length)).decode()
        except (gdb.MemoryError, UnicodeError):
            return False
        if message not in self.seen:
            self.seen.add(message)
            print("[DEBUG] Original history refusal", message)
            gdb.execute("bt 40" if message=="source owner changed original Source or Work" else "bt 9")
        return message == "source owner changed original Source or Work"

OriginalRefusal("'<semio_framework_value_control_physical::ValueError>::literal'")
end
run
