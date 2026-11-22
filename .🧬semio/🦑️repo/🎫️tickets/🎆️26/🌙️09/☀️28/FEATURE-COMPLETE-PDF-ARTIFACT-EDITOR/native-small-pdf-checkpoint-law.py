"""🧪 Exercises the actual natural-file checkpoint decision against an independent schedule."""
import os
from pathlib import Path
import re
import runpy
import subprocess
import tempfile

ticket = Path(__file__).resolve().parent
root = next(parent for parent in ticket.parents if (parent / "Cargo.toml").exists())
stage = runpy.run_path(str(ticket / "native-small-pdf-checkpoint-stage.py"))
rust = next(after for path, (_, after) in stage["changes"].items() if "📥️natural-file" in str(path))
start = rust.index("fn publication_checkpoint_due(")
end = rust.index("\n}", start) + 2
helper = rust[start:end]
proof = r'''
fn main() {
    let mut work = 0u64;
    for i in 1..=600 {
        let previous = work;
        work += if i % 7 == 0 { 0 } else { 64 };
        if publication_checkpoint_due(previous, work, 4096, i == 600) {
            println!("{i},{work}");
        }
    }
    assert!(publication_checkpoint_due(0, 0, 0, true));
    assert!(!publication_checkpoint_due(3, 3, 0, false));
    assert!(!publication_checkpoint_due(u64::MAX, u64::MAX, 4096, false));
    assert!(publication_checkpoint_due(4095, 8193, 4096, false));
}
'''
prefix = ["repo", "tools", "exec", "--light"]
output = stage["scope"]
patch = output / "publication-checkpoint.patch"
if patch.stat().st_size:
    subprocess.run(prefix + ["git", "--", "apply", "--check", str(patch)], cwd=root, check=True)
toolchain = re.search(r'^channel = "([^"]+)"', (root / "rust-toolchain.toml").read_text(), re.MULTILINE).group(1)
with tempfile.TemporaryDirectory(dir=output) as build:
    build = Path(build)
    source = build / "checkpoint.rs"
    source.write_text(helper + "\n" + proof)
    binary = build / ("checkpoint.exe" if os.name == "nt" else "checkpoint")
    subprocess.run(prefix + ["rustup", "--", "run", toolchain, "rustc", "--edition=2024",
                             str(source), "-o", str(binary)], cwd=root, check=True)
    actual = subprocess.run([binary], check=True, capture_output=True, text=True).stdout.splitlines()

checkpoints = []
next_threshold = 4096
for poll in range(1, 601):
    work = (poll - poll // 7) * 64
    if work >= next_threshold:
        checkpoints.append(f"{poll},{work}")
        next_threshold += 4096
if not checkpoints or checkpoints[-1].split(",")[0] != "600":
    checkpoints.append(f"600,{(600 - 600 // 7) * 64}")
assert actual == checkpoints
legacy_pages = 600 - 600 // 7
assert len(actual) == 9 and legacy_pages == 515
print(f"{legacy_pages} old checkpoint payloads reduced to {len(actual)} across {work} work units")
