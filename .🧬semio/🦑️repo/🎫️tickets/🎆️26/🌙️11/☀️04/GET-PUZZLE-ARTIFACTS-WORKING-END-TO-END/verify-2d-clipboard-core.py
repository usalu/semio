"""📋️ Runs the native clipboard core against one coherent warm pack artifact graph."""

import os
from pathlib import Path
import re
import shutil
import signal
import subprocess
import sys
import time

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
generated = ticket / "🗑️generated"
editor = root / "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
kernel = root / "🧰️framework/🔨️modules/🎠️kernel"
recorder = editor / "📼️recorder/🦀️.rs"
snapshot = editor / "📸️snapshot"
ids = snapshot / "🏷️node-labels-ids-and-placement-geometry/🦀️.rs"
constants = snapshot / "🔖️constants-examples-and-scene/🦀️.rs"

def function(path, name):
    """🔎️ Reads the current Rust definition used in the isolated harness."""
    source = path.read_text()
    signature = re.search(rf"(?m)^([ ]*)(?:pub )?fn {name}(?=[<(])", source)
    return source[signature.start():source.index(f"\n{signature[1]}}}", signature.end()) + len(signature[1]) + 2]

def constant(path, name):
    """📏️ Reads one shared constant declaration."""
    return next(line for line in path.read_text().splitlines() if line.startswith(f"pub const {name}:"))

def section(path, first, following):
    """🧭️ Reads the clipboard placement definition from the kernel."""
    source = path.read_text()
    return source[source.index(first):source.index(following)]

output = generated / "clipboard-2d-core.rs"
fixture = editor / "🧪️tests/🧫️fixtures/clipboard-topology.json"
output.write_text("\n\n".join([
    f"const FIXTURE: &str = include_str!(r\"{fixture}\");",
    f"#[path = r\"{kernel / '📋️clipboard/🦀️.rs'}\"] mod kernel_clipboard;",
    section(kernel / "🦀️.rs", "/// ⚓️", "#[path=\"📋️clipboard/🦀️.rs\"]"),
    constant(ids, "PUZZLE2D_CLIPBOARD_SCHEMA"),
    constant(ids, "PUZZLE2D_PASTE_OFFSET"),
    constant(constants, "PUZZLE2D_PLAY_CONTROLLER_ID"),
    *(function(constants, name) for name in ("board_snapshot_nodes", "board_snapshot_edges", "kind_catalog_entries")),
    *(function(ids, name) for name in ("new_node_id", "puzzle2d_selection_is_locked", "puzzle2d_kind_catalog_label")),
    section(ids, "fn puzzle2d_label_root", "pub fn puzzle2d_kind_ids"),
    *(function(recorder, name) for name in ("decode", "puzzle2d_copy_fragment_from", "puzzle2d_paste_operations_on")),
    "impl Puzzle2dRecorder {\n" + function(recorder, "clone_fragment") + "\n}",
    (ticket / "verify-2d-clipboard-core.rs").read_text(),
]), encoding="utf8")

artifacts = generated / "cargo-puzzle-clipboard-build/debug/build"
pack_name = "semio_framework_pack_json"
pack_meta = max(
    (path for path in (artifacts / "semio-framework-pack-json").glob(f"*/out/lib{pack_name}-*.rmeta")
     if path.with_suffix(".rlib").is_file()),
    key=lambda path: path.stat().st_mtime_ns,
)
metadata = subprocess.check_output(["rustc", "-Z", "ls=all", pack_meta], text=True, cwd=generated)
fingerprints = dict(re.findall(r"^\d+ ([a-z][a-z0-9_]+)-([0-9a-f]{16}) hash ", metadata, re.MULTILINE))
fingerprints[pack_name] = pack_meta.stem.rsplit("-", 1)[1]
names = {f"lib{name}-{fingerprint}" for name, fingerprint in fingerprints.items()}
dependencies = generated / "clipboard-2d-core-deps"
dependencies.mkdir(exist_ok=True)
for path in artifacts.glob("*/*/out"):
    for library in path.iterdir():
        if library.stem in names and library.suffix in {".rmeta", ".rlib", ".so", ".dylib", ".dll"}:
            link = dependencies / library.name
            if not link.exists():
                try:
                    link.symlink_to(library)
                except OSError:
                    os.link(library, link)

argv = ["rustc", "--edition=2021", "-A", "dead_code", "-Cdebuginfo=0", "-Zthreads=1",
        "-L", f"dependency={dependencies}", str(output)]
if sys.platform.startswith("linux"):
    argv += ["-Clinker-features=+lld", "-Clink-self-contained=+linker", "-Zunstable-options"]
for name in (pack_name, "semio_framework_value", "serde", "serde_json"):
    library = dependencies / f"lib{name}-{fingerprints[name]}.rlib"
    argv += ["--extern", f"{name}={library}", "--extern", f"{name}={library.with_suffix('.rmeta')}"]
binary = generated / "clipboard-2d-core"
environment = dict(os.environ, ZIG_GLOBAL_CACHE_DIR=str(generated / "zig-clipboard/global"),
                   ZIG_LOCAL_CACHE_DIR=str(generated / "zig-clipboard/local"))

def storage_and_memory_available():
    """💾️ Preserves workspace capacity and host memory for concurrent native lanes."""
    if shutil.disk_usage(generated).free < 8 * 1024**3:
        return False
    state = Path("/proc/meminfo")
    if state.is_file():
        memory = {name: int(kib) for name, kib in re.findall(r"^(MemAvailable|SwapFree):\s+(\d+)", state.read_text(), re.MULTILINE)}
        return sum(memory.values()) * 1024 >= 4 * 1024**3
    return True

if not storage_and_memory_available():
    raise SystemExit("[DEBUG] Native clipboard check deferred by workspace storage or memory floor")
compiler = subprocess.Popen(argv + ["-o", str(binary)], cwd=generated, env=environment, start_new_session=os.name == "posix")
while compiler.poll() is None:
    if not storage_and_memory_available():
        if os.name == "posix":
            os.killpg(compiler.pid, signal.SIGTERM)
        else:
            compiler.terminate()
        compiler.wait()
        raise SystemExit("[DEBUG] Native clipboard check stopped at the workspace storage or memory floor")
    time.sleep(1)
if compiler.returncode:
    raise SystemExit(compiler.returncode)
for case in ("normal", "handle-namespace"):
    subprocess.run([binary, case], check=True, cwd=root)
