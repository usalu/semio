"""🧬️ W0-I mutation check: applies one deliberate regression to the tool-run driver at a time, runs the plugin tool-run
laws, records which laws went red, and restores the file byte-identically (verified by comparing bytes)."""
import subprocess
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
DRIVER = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs"
OUT = ROOT / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️13/INTERACTIVE-TOOLS-VISIBLE-PROCESS/🗑️generated/W0-I"

MUTATIONS = {
    "no-rebind": ("""            if !restart {
                self.tool_runs.retarget_current_job(|job, identity| {
                    job.rebind(identity);
                    true
                });
            }""", ""),
    "no-chord-resolution": ("let live = if names_identity { None } else { self.tool_runs.slot() };", "let live: Option<ToolRunSlot> = None;"),
    "settings-ignore-values": ("        if values == entry.settings_values {\n            return;\n        }", ""),
    "refold-always-boundary": ("        if !refold.boundary {\n            self.refold = Some(refold);\n            return ToolRunRefoldTurn::AwaitingBoundary;\n        }", ""),
    "no-in-place-reconfigure": ("self.tool_runs.retarget_current_job(|job, identity| job.reconfigure(identity, config));", "{ drop(config); self.tool_runs.close_current_job(); }"),
}

original = DRIVER.read_bytes()
selected = sys.argv[1:] or list(MUTATIONS)
try:
    for name in selected:
        needle, replacement = MUTATIONS[name]
        text = original.decode()
        assert text.count(needle) == 1, f"{name}: needle not unique"
        DRIVER.write_text(text.replace(needle, replacement))
        run = subprocess.run(["cargo", "test", "-p", "semio-framework-plugin", "--features", "artifact-app-testing", "-j", "4", "--lib", "--", "tool_run"], cwd=ROOT, capture_output=True, text=True)
        (OUT / f"mutation-{name}.txt").write_text(run.stdout + run.stderr)
        red = [line for line in run.stdout.splitlines() if line.endswith("FAILED") and line.startswith("test ")]
        summary = [line for line in run.stdout.splitlines() if line.startswith("test result")]
        print(f"[w0i] {name}: {len(red)} red {summary}")
        for line in red:
            print(f"    {line}")
        DRIVER.write_bytes(original)
finally:
    DRIVER.write_bytes(original)
    assert DRIVER.read_bytes() == original
    print("[w0i] driver restored byte-identical")
