set pagination off
set debuginfod enabled off
set environment RUST_MIN_STACK 268435456
rbreak retain_original_ownership_receipts
run
disable 1
set $receipt_rejected = $pc + 1153
python
import collections
import re
import gdb
calls = collections.deque(maxlen=40)
watch = [
 "advance_mounted_window_config_refresh",
 "advance_apply_batch_with_turn_ceiling",
 "advance_pending_result_page",
 "advance_acknowledged_result_page",
 "advance_mounted_committed_store_handoff",
 "admit_actor_capture",
 "settle_tool_operation",
 "snapshot_clone_turn_admitted",
 "reserve_store_result",
 "handoff_mounted_durable_publication",
 "fault_stalled_typed_operation_publication",
]
class Probe(gdb.Breakpoint):
    def __init__(self, address, name):
        super().__init__("*" + address, internal=True)
        self.name = name
    def stop(self):
        calls.append((self.name, hex(int(gdb.newest_frame().older().pc()))))
        if self.name == "fault_stalled_typed_operation_publication":
            print("[DEBUG] Last publication calls:")
            for call in calls:
                print(call)
            return True
        return False
class Receipt(gdb.Breakpoint):
    def stop(self):
        gdb.execute("set $pc = $receipt_rejected + 54")
        return False
receipt = Receipt("*$receipt_rejected", internal=True)
probe = []
for name in watch:
    lines = gdb.execute("info functions " + name, to_string=True).splitlines()
    for line in lines:
        match = re.match(r"(0x[0-9a-fA-F]+)\s+(.*)", line)
        if match and name in match[2] and "closure" not in match[2]:
            probe.append(Probe(match[1], name))
print("[DEBUG] Watching", len(probe), "publication addresses")
end
continue
bt 8
