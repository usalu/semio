"""🔚 OHH close-loop migrator: rewrites the old `loop { match STORE.close_owned_step(1, MAX).expect(MSG) { …SnapshotRetirementStep::{Complete,Pending,Blocked} } }` block to the current retained-grant close loop
(`while !terminal { demand → grant → close_owned_step(grant) }`), and drops `SnapshotRetirementStep` from the file's `use` list. Edits only the files given, anchor-checked (each file must match the block exactly once per expected count).
usage: closeloop.py <file> <expected-blocks> [<directory-prefix>]"""
import re, sys
p, n = sys.argv[1], int(sys.argv[2])
pre = sys.argv[3] if len(sys.argv) > 3 else "directory"
t = open(p, encoding="utf-8").read()
RX = re.compile(r'(?P<ind>[ \t]*)loop \{\n[ \t]*match (?P<st>[\w.]+)\.close_owned_step\(1, [^\n]*?\)\.expect\((?P<msg>"[^"]*")\) \{\n[ \t]*(?:[\w:]*)SnapshotRetirementStep::Complete => break,\n[ \t]*(?:[\w:]*)SnapshotRetirementStep::Pending \{ \.\. \} => \{\}\n[ \t]*(?:[\w:]*)SnapshotRetirementStep::Blocked => panic!\("[^"]*"\),\n[ \t]*\}\n[ \t]*\}\n')
def rep(m):
    i, st, msg = m.group("ind"), m.group("st"), m.group("msg")
    q = '"the store quotes its next close turn"'
    return (f"{i}while !{st}.close_owned_terminal_is_empty() {{\n"
            f"{i}    let demand = {st}.close_owned_demands(0).expect({q});\n"
            f"{i}    let grant = {pre}::os_store::RetainedCloneGrant {{ maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }};\n"
            f"{i}    {st}.close_owned_step(grant).expect({msg});\n"
            f"{i}}}\n")
t2, k = RX.subn(rep, t)
assert k == n, (p, k, n)
t2 = re.sub(r"(use [^;\n]*?os_store::\{[^}\n]*?), SnapshotRetirementStep\}", r"\1}", t2)
t2 = re.sub(r"(use [^;\n]*?os_store::\{)SnapshotRetirementStep, ", r"\1", t2)
assert "SnapshotRetirementStep" not in t2, (p, "residual")
open(p, "w", encoding="utf-8").write(t2)
print("migrated", k, p[-60:])
