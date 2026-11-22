set confirm off
set pagination off
set print frame-arguments none
handle SIGTRAP stop print nopass
start
python
import os, signal, threading
prefix = "<semio_framework_value_retained_map_edit::ordered_map::insert_cursor::RetainedOrderedMapInsertCursor<alloc::string::String, f64>>::"

class MapFrontier(gdb.Breakpoint):
    def __init__(self, action):
        super().__init__("'" + prefix + action + "'")
        self.action, self.hits = action, 0

    def stop(self):
        self.hits += 1
        if self.hits <= 3 or self.hits in {8, 64, 1024}:
            print("[DEBUG] Config insertion frontier", self.action, self.hits)
            gdb.execute("info registers rdi rsi rdx rcx")
            gdb.execute("x/14gx $rsi")
        return self.hits == 1024

class EditFrontier(gdb.Breakpoint):
    def __init__(self, action):
        super().__init__("'<semio_framework_value_retained_map_edit::ordered_map::edit_input::EditInput<alloc::string::String, f64>>::" + action + "'")
        self.action, self.hits = action, 0

    def stop(self):
        self.hits += 1
        if self.hits <= 18 or self.hits == 64:
            print("[DEBUG] Config input frontier", self.action, self.hits)
            if self.action == "advance_into":
                gdb.execute("x/ub $rsi+0x2761")
            else:
                gdb.execute("info registers rsi rdx rcx")
                gdb.execute("x/4gu $rdx")
                gdb.execute("x/10gu $rsi")
        return self.hits == 64

probes = [EditFrontier(action) for action in ("advance_into", "funded")]
pid = gdb.selected_inferior().pid
timer = threading.Timer(35, lambda: os.kill(pid, signal.SIGTRAP))
timer.daemon = True
timer.start()
end
continue
thread apply all bt 26
kill
