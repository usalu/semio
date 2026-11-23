"""🧵️ Run the command lease fixture against Rust and an independent Python deque with the fresh-grant flag."""

from collections import deque
import json
from pathlib import Path
import subprocess
import sys


def oracle(op, held, waiters, armed):
    """🗝️ Apply one fixture operation to an independent holder, FIFO and fresh-grant flag."""
    action = op[0]
    key = op[1]
    if action == "idle":
        return held is None and not waiters, held
    if action in ("claim", "poor"):
        if key == held:
            armed[0] = False
            return True, held
        if action == "claim":
            if held is None and not op[2] and (not waiters or waiters[0] == key):
                if waiters:
                    waiters.popleft()
                held = key
                armed[0] = True
            elif key not in waiters:
                waiters.append(key)
        return False, held
    if action == "holds":
        return key == held, held
    if action == "queued":
        return key in waiters, held
    assert action in ("leave", "idlePublication")
    if action == "idlePublication" and (op[2] or (armed[0] and key == held)):
        return False, held
    changed = key == held or key in waiters
    if key == held:
        armed[0] = False
    if key in waiters:
        waiters.remove(key)
    return changed, None if key == held else held


def rust_expression(op, legacy):
    """📐️ Render the public lease call used by the shared fixture."""
    action, key = op[:2]
    key = repr(key)
    pending = json.dumps(op[2]) if len(op) > 3 else "false"
    if action in ("claim", "poor"):
        paid = json.dumps(action == "claim")
        return f"lease.arbitrate::<std::convert::Infallible>({key}, {pending}, || Ok({paid})).unwrap()"
    if action == "idlePublication":
        return f"lease.leave({key})" if legacy else f"lease.leave_if_idle({key}, {pending})"
    if action == "idle":
        return "lease.is_idle()"
    assert action in ("holds", "queued", "leave")
    return f"lease.{action}({key})"


ticket = Path(__file__).resolve().parent
workspace = ticket.parents[6]
routing = workspace / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📨️dispatch/📥️staging/📣️host-event-routing"
text = (routing / "🦀️.rs").read_text()
generic = text[text.index("pub const COMMAND_LEASE_WAITERS"):text.index("impl CommandLease{")]
generic = "\n".join(line for line in generic.splitlines() if not line.startswith(("pub type LeaseKey", "pub type CommandLease")))
cases = json.loads((routing / "🧪️tests/🔣️.json").read_text())["scripts"]
mode = sys.argv[1]
assert mode in ("red", "green")
if mode == "red":
    generic = generic.replace("self.armed=true", "self.armed=false")
rows = [generic, "fn main() {"]
for name, script in cases.items():
    held, waiters, armed = None, deque(), [False]
    rows.append("let mut lease = Lease::<char>::default();")
    for i, op in enumerate(script):
        expected, held = oracle(op, held, waiters, armed)
        assert expected == op[-1], (name, i, op, expected)
        rows.append(f'assert_eq!({rust_expression(op, False)}, {json.dumps(expected)}, "{name}:{i}");')
rows.append("}")
generated = ticket / "🗑️generated"
source = generated / f"command-lease-{mode}.rs"
binary = generated / f"command-lease-{mode}"
source.write_text("\n".join(rows))
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "--edition=2021", str(source), "-o", str(binary)], check=True)
result = subprocess.run([str(binary)], text=True, capture_output=True)
if mode == "red":
    assert result.returncode != 0 and "aFreshGrantSurvivesIdlePublicationUntilItsFirstClaim:1" in result.stderr, result
    print("[DEBUG] Unarmed Rust releases the fresh grant against the Python oracle")
else:
    assert result.returncode == 0, result.stderr
    print(f"[DEBUG] Rust and Python agree on all {len(cases)} fixture sequences")
