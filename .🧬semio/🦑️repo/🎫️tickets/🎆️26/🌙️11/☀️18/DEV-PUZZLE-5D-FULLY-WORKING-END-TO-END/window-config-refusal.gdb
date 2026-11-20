set pagination off
set debuginfod enabled off
set auto-solib-add off
set print frame-arguments none
set environment RUST_MIN_STACK 67108864
starti
python
import gdb
import os

inferior = gdb.selected_inferior()
mapping = next(row for row in open("/proc/%d/maps" % inferior.pid) if "semio_s_artifact_puzzle_5d_v1-f62a359f9cc1d4ad" in row and row.split()[2] == "00000000")
base = int(mapping.split("-")[0], 16)

class WindowRefusal(gdb.Breakpoint):
    def stop(self):
        frame = gdb.newest_frame()
        cursor = frame
        for _ in range(22):
            if cursor is None:
                return False
            name = cursor.name() or ""
            if "RetainedClonePreparation" in name and "WorldWindowConfig" in name:
                size = int(frame.read_register("rcx"))
                start = int(frame.read_register("rdx"))
                print("[DEBUG] WorldWindowConfig refusal:", inferior.read_memory(start, min(size, 512)).tobytes().decode("utf8", "replace"))
                print(gdb.execute("bt 12", to_string=True))
                return True
            cursor = cursor.older()
        return False

literal = int(os.getenv("SEMIO_WINDOW_LITERAL_ADDRESS", "0xda8fc60"), 16)
WindowRefusal("*0x%x" % (base + literal))

class ReceivingWork(gdb.Breakpoint):
    def stop(self):
        frame = gdb.newest_frame()
        reported = int(frame.read_register("rsi"))
        observed = int(frame.read_register("rdx"))
        caller = frame.older()
        name = caller.name() or ""
        if reported and observed > reported and "RetainedClonePreparation" in name and "WorldWindowConfig" in name:
            print("[DEBUG] WorldWindowConfig work receipt:", reported, "reported;", observed, "observed")
            if os.getenv("SEMIO_WINDOW_TRACE_FULL_WORK") == "1":
                gdb.execute("set $rsi = %d" % observed)
        return False

work = int(os.getenv("SEMIO_WINDOW_WORK_ADDRESS", "0xc3a8fc0"), 16)
ReceivingWork("*0x%x" % (base + work))
end
continue
