set confirm off
set pagination off
set print frame-arguments none
start
python
import gdb
symbol = "<semio_plugin_window_config::retained::typed_pack_load::native::TypedWindowConfigPackLoad<semio_s_artifact_puzzle_5d_tests::mirror::editor::puzzle5d::window::Puzzle5dBoardWindowConfigOwner>>::advance_native"
class WindowStage(gdb.Breakpoint):
    stage = None
    count = 0
    def stop(self):
        owner = int(gdb.parse_and_eval("$rsi"))
        stage = bytes(gdb.selected_inferior().read_memory(owner + 0x16739, 1))[0]
        if stage != self.stage:
            self.stage, self.count = stage, 0
            print("[DEBUG] Native Window stage", stage, hex(owner))
        self.count += 1
        if self.count in {50, 200, 1000, 5000, 10000, 20000, 29000}:
            print("[DEBUG] Native Window unchanged stage", stage, self.count)
        return self.count == 29000
WindowStage("'" + symbol + "'")
end
continue
bt 22
kill
