set confirm off
set pagination off
handle SIGTRAP stop print nopass
start
python
import os, signal, threading
class Frontier(gdb.Breakpoint):
    def __init__(self, name, registers, count):
        super().__init__("'" + name + "'")
        self.registers = registers
        self.count = count
        self.hits = 0
    def stop(self):
        self.hits += 1
        if self.hits <= self.count:
            print("[DEBUG] Config funding", self.location, self.hits)
            for command in self.registers:
                gdb.execute(command)
        return self.hits == 64
map_edit = "<semio_framework_value_retained_map_edit::ordered_map::edit_input::EditInput<alloc::string::String, f64>>::"
insertion = "<semio_framework_value_retained_map_edit::ordered_map::insert_cursor::RetainedOrderedMapInsertCursor<alloc::string::String, f64>>::"
birth = "<semio_framework_value_clone_source::source_custody::admission::RetainedCloneSourceAdmissionBirth<semio_framework_value_retained_map_edit::ordered_map::edit_input::EditWorkspace<alloc::string::String, f64>, semio_framework_value_retained_map_edit::ordered_map::edit_input::EditWorkspace<alloc::string::String, f64>>>::"
probes = [Frontier(insertion + "funded", ["info registers rcx", "x/4gu $rdx", "x/6gu $rsi+48"], 56), Frontier(map_edit + "funded", ["x/4gu $rdx", "x/14gu $rsi"], 32), Frontier(birth + "advance", ["x/14gu $rcx", "x/10gu $rsi", "bt 6"], 10)]
pid = gdb.selected_inferior().pid
timer = threading.Timer(40, lambda: os.kill(pid, signal.SIGTRAP))
timer.daemon = True
timer.start()
end
continue
thread apply all bt 24
kill
