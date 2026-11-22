"""🧪️ Run the original two-locale Generator browser case with native process custody."""

import argparse
import json
import os
from pathlib import Path
import signal
import subprocess
import time
from urllib.parse import quote
from urllib.request import urlopen


def capacity():
    """📏️ Read the same host floor and pressure as the queued repository check."""
    raw = subprocess.check_output(["repo", "capacity", "status", "--lang=en", "--json"], text=True, timeout=5)
    snapshot = json.loads(raw)
    return snapshot["evaluation"]["headroomMegabytes"], snapshot["host"]["pressure"]


def descendants(root):
    """👥️ Includes each Playwright and Chromium descendant even if Chromium creates its own group."""
    table = subprocess.check_output(["ps", "-axo", "pid=,ppid=,rss="], text=True, timeout=3)
    rows = [tuple(map(int, line.split())) for line in table.splitlines()]
    owned = {root}
    while True:
        children = {pid for pid, parent, _ in rows if parent in owned}
        if children <= owned:
            return {pid: rss for pid, _, rss in rows if pid in owned}
        owned.update(children)


def stop(process, owned):
    """🛑️ Stops the check and its own browser children, then waits for the Playwright launcher."""
    for pid in set(owned) - {process.pid}:
        try:
            os.kill(pid, signal.SIGTERM)
        except ProcessLookupError:
            pass
    try:
        os.killpg(process.pid, signal.SIGTERM)
    except ProcessLookupError:
        pass
    try:
        process.wait(timeout=5)
    except subprocess.TimeoutExpired:
        os.killpg(process.pid, signal.SIGKILL)
        process.wait(timeout=5)


def play(command, env, cwd, log):
    """⏳️ Waits for the authorized launch floor and monitors the one native browser tree."""
    while True:
        spare, pressure = capacity()
        if spare >= 2560 and pressure != "critical":
            break
        print(f"[DEBUG] Native browser waiting for 2560 MiB spare, have {spare} MiB under {pressure} pressure", flush=True)
        time.sleep(5)
    print(f"[DEBUG] Native browser launch with {spare} MiB host headroom under {pressure} pressure", flush=True)
    process = subprocess.Popen(command, cwd=cwd, env=env, stdout=log, stderr=subprocess.STDOUT, start_new_session=True)
    started = time.monotonic()
    peak = 0
    owned = {}
    try:
        while process.poll() is None:
            owned = descendants(process.pid)
            charged = sum(owned.values()) // 1024
            if charged > peak:
                peak = charged
                print(f"[DEBUG] Native browser process tree peak {peak} MiB", flush=True)
            _, pressure = capacity()
            if charged > 2560 or pressure == "critical" or time.monotonic() - started > 300:
                print(f"[DEBUG] Native browser stopped at {charged} MiB under {pressure} pressure", flush=True)
                return 124
            time.sleep(1)
        return process.returncode
    finally:
        if process.poll() is None:
            stop(process, owned)
        print(f"[DEBUG] Native browser process tree max {peak} MiB", flush=True)


def main():
    """🧾️ Attests the immutable core and uses the official target regex, projects and worker count."""
    parser = argparse.ArgumentParser()
    parser.add_argument("site", type=Path)
    parser.add_argument("overlay", type=Path)
    parser.add_argument("expected_core")
    parser.add_argument("--port", type=int, default=8821)
    args = parser.parse_args()
    ticket = Path(__file__).resolve().parent
    root = ticket.parents[6]
    demo = root / "♻️mit-bestand" / "🧺️demonstrator"
    generated = ticket / "🗑️generated"
    modules = root / ".cache/b/p/m/browser/release/modules"
    node = ["repo", "tools", "exec", "--light", "node", "--"]
    origin = f"http://127.0.0.1:{args.port}"
    server_command = [*node, ticket / "overlay-serve.mjs", args.site.resolve(), modules, str(args.port), args.overlay.name, args.overlay.resolve()]
    with (generated / "generator-native-fallback-server.log").open("w") as server_log:
        server = subprocess.Popen(server_command, stdout=server_log, stderr=subprocess.STDOUT, start_new_session=True)
        try:
            for _ in range(60):
                if server.poll() is not None:
                    raise RuntimeError(f"Generator overlay exited {server.returncode}")
                try:
                    with urlopen(origin + "/", timeout=1) as response:
                        if response.status == 200:
                            break
                except OSError:
                    time.sleep(0.5)
            else:
                raise TimeoutError("Generator native acceptance overlay did not open")
            core = "semio_hub_procedural_component.core.wasm"
            route = quote(f"🔌️plugin-modules/{args.overlay.name}/{core}")
            receipt = subprocess.check_output([*node, ticket / "probe-custody.mjs", "verify", args.overlay / core, f"{origin}/{route}"], text=True)
            print(receipt.strip(), flush=True)
            if receipt.split()[0] != args.expected_core:
                raise RuntimeError("Generator overlay core differs from the requested core")
            renderer = next(args.site.glob("assets/🌐️-*.js"))
            subprocess.run([*node, ticket / "probe-custody.mjs", "verify", renderer, f"{origin}/assets/{quote(renderer.name)}"], check=True)
            finder = 'const path=require("node:path");console.log(path.join(path.dirname(require.resolve("@playwright/test/package.json")),"cli.js"))'
            cli = subprocess.check_output([*node, "-e", finder], cwd=demo, text=True).strip()
            env = dict(os.environ, DEMONSTRATOR_VERIFY_CDN="1", PLAYWRIGHT_BASE_URL=f"http://cdn.localhost:{args.port}", PLAYWRIGHT_BROWSERS_PATH=str(root / ".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"))
            command = [
                *node, cli, "test", "--config", "./🔨️modules/🧪️e2e/🎚️config/🟦️.ts", "--workers=1",
                "--project", "en", "--project", "de", "--grep", "(generator) boot$", "--output",
                generated / "generator-native-fallback-pw", "--reporter=list",
            ]
            with (generated / "generator-native-fallback-playwright.log").open("w") as log:
                return play(command, env, demo, log)
        finally:
            if server.poll() is None:
                stop(server, descendants(server.pid))


if __name__ == "__main__":
    raise SystemExit(main())
