"""🔬️ Replay the exact private Generator core with a process-group memory bound."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import time
from urllib.request import urlopen


def browser_rss(pid):
    """📏️ Sum resident memory for Chromium's own process group."""
    if os.name == "nt":
        return 0
    try:
        table = subprocess.check_output(["ps", "-axo", "pgid=,rss="], text=True)
    except (OSError, subprocess.SubprocessError):
        return 0
    return sum(int(rss) for group, rss in map(str.split, table.splitlines()) if int(group) == pid)


def stop_group(pid):
    """🛑️ Terminate the exact Chromium group created by this probe."""
    if pid is None:
        return
    if os.name == "nt":
        subprocess.run(["taskkill", "/pid", str(pid), "/t", "/f"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, check=False)
        return
    try:
        if os.getpgid(pid) == pid:
            os.killpg(pid, signal.SIGTERM)
    except ProcessLookupError:
        pass


def observed_memory():
    """🏛️ Read the repository governor's physical charge for this invocation."""
    try:
        raw = subprocess.check_output(["repo", "capacity", "status", "--json", "--lang=en"], text=True, timeout=3)
        snapshot = json.loads(raw)
    except (OSError, ValueError, subprocess.SubprocessError):
        return 0
    inherited = os.environ.get("SEMIO_HOST_CAPACITY_GRANT")
    return next((grant.get("observedMegabytes", 0) or 0 for grant in snapshot["grants"] if grant["id"] == inherited or grant["pid"] == os.getppid()), 0)


def native_headroom():
    """📏️ Reads the repository governor's actual remaining headroom for a native replay."""
    raw = subprocess.check_output(["repo", "capacity", "status", "--json", "--lang=en"], text=True, timeout=5)
    snapshot = json.loads(raw)
    return snapshot["evaluation"]["headroomMegabytes"], snapshot["host"]["pressure"]


def replay(args, url, ticket, generated):
    """▶️ Capture a single locale while continuously bounding this browser."""
    variant = "yield" if args.yield_drain else "ack" if args.acknowledgements_only else "full"
    if args.wire:
        variant += "-wire"
    if args.native_diagnostics:
        variant += "-native"
    if args.double_item_grant:
        variant += "-double-items"
    if args.label:
        variant += f"-{args.label}"
    if args.profile:
        variant += "-profile"
    output = generated / f"generator-live-{args.locale}-{variant}.log"
    flags = ["--turn-status", "--handoff-status", "--frames", "--shell-action", "--timestamps", "--browser-pid", f"--locale={args.locale}"]
    flags.append(f"--browsers={os.environ['PLAYWRIGHT_BROWSERS_PATH']}")
    if args.liftoff:
        flags.append("--liftoff-only")
    if args.acknowledgements_only:
        flags.append("--acknowledgements-only")
    if args.yield_drain:
        flags.append("--yield-drain")
    if args.wire:
        flags.append("--wire")
    if args.trace_host:
        flags.append("--trace-host")
    if args.trace_guest:
        flags.append("--trace-guest")
    if args.pause_guest:
        flags.append("--pause-guest")
    if args.wasm_breakpoint:
        flags.append(f"--wasm-breakpoint={args.wasm_breakpoint}")
    if args.stop_on_control_turns:
        flags.append("--stop-on-control-turns")
    if args.profile:
        flags.append(f"--profile={generated / ('generator-' + variant)}")
    if args.native_diagnostics:
        flags.append("--diagnostics")
    if args.double_item_grant:
        flags.append("--double-item-grant")
    command = ["repo", "tools", "exec", "--light", "node", "--", str(ticket / "generator-command-probe.cjs"), url, str(args.seconds * 1000), *flags]
    browser = None
    with output.open("w", buffering=1) as log:
        for asset in args.assets:
            sha = hashlib.sha256()
            with asset.open("rb") as body:
                for chunk in iter(lambda: body.read(1024 * 1024), b""):
                    sha.update(chunk)
            print(f"[DEBUG] SHA-256 {sha.hexdigest()} {asset.name}", file=log)
        if args.native_min_spare:
            while True:
                spare, pressure = native_headroom()
                if spare >= args.native_min_spare and pressure != "critical":
                    print(f"[DEBUG] Native replay begins with {spare} MiB host headroom under {pressure} pressure", file=log)
                    break
                print(f"[DEBUG] Native replay waiting for {args.native_min_spare} MiB spare, have {spare} MiB under {pressure} pressure", file=log)
                time.sleep(5)
        process = subprocess.Popen(command, stdout=log, stderr=subprocess.STDOUT)
        start = time.monotonic()
        next_report = start
        try:
            while process.poll() is None and time.monotonic() - start < args.seconds + 15:
                observed = output.read_text(errors="replace")
                if browser is None:
                    match = re.search(r"generator Chromium PID (\d+)", observed)
                    if match:
                        browser = int(match[1])
                if browser is not None:
                    rss = browser_rss(browser)
                    if args.native_min_spare and native_headroom()[1] == "critical":
                        print("[DEBUG] Native Generator replay stopped under critical host memory pressure", file=log)
                        return 2
                    if rss > args.memory * 1024:
                        print(f"[DEBUG] Generator replay exceeded its {args.memory} MiB cap at {rss // 1024} MiB", file=log)
                        return 2
                    if time.monotonic() >= next_report:
                        charged = observed_memory()
                        print(f"[DEBUG] Chromium resident memory {rss // 1024} MiB and governor charge {charged} MiB", file=log)
                        if charged > args.memory:
                            return 2
                        next_report = time.monotonic() + 5
                if (args.stop and args.stop in observed) or (args.stop_on_control_turns and "generator wire control-only 3" in observed):
                    return 0
                time.sleep(1)
            return process.returncode if process.returncode is not None else 124
        finally:
            if process.poll() is None:
                stop_group(browser)
                process.terminate()
            process.wait(timeout=5)


def main():
    """🌐️ Serve existing product files and replay a private Procedural build."""
    parser = argparse.ArgumentParser()
    parser.add_argument("locale", choices=["en-US", "de-DE"])
    parser.add_argument("--port", type=int, default=8819)
    parser.add_argument("--seconds", type=int, default=125)
    parser.add_argument("--memory", type=int, default=2560)
    parser.add_argument("--native-min-spare", type=int, default=0)
    parser.add_argument("--liftoff", action="store_true")
    parser.add_argument("--acknowledgements-only", action="store_true")
    parser.add_argument("--yield-drain", action="store_true")
    parser.add_argument("--wire", action="store_true")
    parser.add_argument("--trace-host", action="store_true")
    parser.add_argument("--trace-guest", action="store_true")
    parser.add_argument("--pause-guest", action="store_true")
    parser.add_argument("--wasm-breakpoint")
    parser.add_argument("--native-diagnostics", action="store_true")
    parser.add_argument("--double-item-grant", action="store_true")
    parser.add_argument("--label", choices=["remaining", "fresh", "after-retirement", "host-tail", "guest-tail", "unreplied", "wy17-refusal"])
    parser.add_argument("--site", type=Path, default=Path(".cache/b/p/m/vite/release/5dc1b077e86de2aa/out"))
    parser.add_argument("--procedural-dir", type=Path)
    parser.add_argument("--profile", action="store_true")
    parser.add_argument("--stop-on-control-turns", action="store_true")
    parser.add_argument("--stop")
    args = parser.parse_args()
    ticket = Path(__file__).resolve().parent
    generated = ticket / "🗑️generated"
    site = args.site.resolve()
    modules = Path(".cache/b/p/m/browser/release/modules").resolve()
    procedural = args.procedural_dir or generated / "probe-wqks1dx-65558" / "🌀️procedural"
    assert (site / "index.html").is_file() and procedural.is_dir() and modules.is_dir()
    module_files = sorted(entry for entry in procedural.iterdir() if entry.is_file())
    args.assets = [site / "index.html", *sorted((site / "assets").glob("*.js")), *module_files]
    os.environ.setdefault("PLAYWRIGHT_BROWSERS_PATH", str(Path(".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright").resolve()))
    launch = ["repo", "tools", "exec", "--light", "node", "--", str(ticket / "overlay-serve.mjs"), str(site), str(modules), str(args.port), procedural.name, str(procedural)]
    with (generated / "generator-live-server.log").open("w") as server_log:
        server = subprocess.Popen(launch, stdout=server_log, stderr=subprocess.STDOUT)
        try:
            url = f"http://cdn.localhost:{args.port}"
            for _ in range(100):
                if server.poll() is not None:
                    raise RuntimeError(f"Generator overlay exited {server.returncode}")
                try:
                    with urlopen(f"http://127.0.0.1:{args.port}", timeout=1) as home:
                        if home.status == 200:
                            break
                except OSError:
                    time.sleep(0.1)
            else:
                raise TimeoutError("Generator overlay did not open")
            return replay(args, url + "/#generator", ticket, generated)
        finally:
            server.terminate()
            server.wait(timeout=5)


if __name__ == "__main__":
    raise SystemExit(main())
