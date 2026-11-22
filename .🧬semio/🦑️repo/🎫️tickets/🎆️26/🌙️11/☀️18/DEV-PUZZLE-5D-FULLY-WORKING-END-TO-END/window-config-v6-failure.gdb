set confirm off
set pagination off
set print frame-arguments none
tbreak '<semio_plugin_window_config::retained::command::WindowConfigCommandLoad>::advance'
run
python
class ValueRefusal(gdb.Breakpoint):
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
            print("[DEBUG] Window config refusal", message)
            gdb.execute("bt 10")
        return len(self.seen) >= 16

ValueRefusal("'<semio_framework_value_control_physical::ValueError>::literal'")
end
continue
