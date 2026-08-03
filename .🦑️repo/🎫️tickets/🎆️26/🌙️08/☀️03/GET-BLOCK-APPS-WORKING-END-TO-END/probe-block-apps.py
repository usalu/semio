#!/usr/bin/env python3
"""Boot block2d/3d/5d sequentially, wait for Vite ready, probe HTTP, log results."""
from __future__ import annotations

import json
import os
import signal
import subprocess
import time
import urllib.error
import urllib.request
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = Path(subprocess.check_output(["git", "rev-parse", "--show-toplevel"], cwd=TICKET, text=True).strip())

APPS = [
    {"id": "block2d", "script": ["bun", "run", "dev:block:2d"], "port": 6024, "env": "BLOCK_2D_PLAY_PORT"},
    {"id": "block3d", "script": ["bun", "run", "dev:block:3d"], "port": 6025, "env": "BLOCK_3D_PLAY_PORT"},
    {"id": "block5d", "script": ["bun", "run", "dev:block:5d"], "port": 6026, "env": "BLOCK_5D_PLAY_PORT"},
]

READY_BUDGET_S = 60 * 25  # plugin wasm builds can be long


def probe(port: int) -> dict:
    url = f"http://127.0.0.1:{port}/"
    try:
        with urllib.request.urlopen(url, timeout=10) as resp:
            body = resp.read(4000).decode("utf-8", errors="replace")
            return {"ok": True, "status": resp.status, "url": url, "snippet": body[:500]}
    except Exception as e:
        return {"ok": False, "url": url, "error": str(e)}


def wait_ready(proc: subprocess.Popen, log_path: Path, port: int) -> dict:
    t0 = time.time()
    needle = f":{port}"
    while time.time() - t0 < READY_BUDGET_S:
        if proc.poll() is not None:
            return {"ok": False, "error": f"process exited early with {proc.returncode}", "log_tail": log_path.read_text(encoding="utf-8", errors="replace")[-4000:]}
        text = log_path.read_text(encoding="utf-8", errors="replace")
        # Vite prints Local: http://127.0.0.1:PORT/
        if ("Local:" in text or "ready in" in text.lower()) and needle in text:
            # give a beat for listen
            time.sleep(1.5)
            result = probe(port)
            result["boot_s"] = round(time.time() - t0, 1)
            result["log_tail"] = text[-2500:]
            return result
        # also accept if port already serves
        if probe(port)["ok"]:
            result = probe(port)
            result["boot_s"] = round(time.time() - t0, 1)
            result["log_tail"] = text[-2500:]
            return result
        time.sleep(2)
    return {"ok": False, "error": "budget exceeded waiting for ready", "log_tail": log_path.read_text(encoding="utf-8", errors="replace")[-4000:]}


def run_one(app: dict) -> dict:
    log_path = TICKET / f"{app['id']}-dev.log"
    env = os.environ.copy()
    env[app["env"]] = str(app["port"])
    # Engines already built; still allow plugin builds.
    print(f"[boot] starting {app['id']} on {app['port']}", flush=True)
    with log_path.open("w", encoding="utf-8") as fh:
        proc = subprocess.Popen(app["script"], cwd=ROOT, env=env, stdout=fh, stderr=subprocess.STDOUT, start_new_session=True)
    try:
        result = wait_ready(proc, log_path, app["port"])
        result["id"] = app["id"]
        result["port"] = app["port"]
        result["pid"] = proc.pid
        return result
    finally:
        try:
            os.killpg(proc.pid, signal.SIGTERM)
        except ProcessLookupError:
            pass
        try:
            proc.wait(timeout=15)
        except subprocess.TimeoutExpired:
            try:
                os.killpg(proc.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            proc.wait(timeout=5)


def main() -> None:
    results = []
    for app in APPS:
        results.append(run_one(app))
        print(json.dumps({k: v for k, v in results[-1].items() if k != "log_tail" and k != "snippet"}, ensure_ascii=False), flush=True)
    out = TICKET / "e2e-probe-results.json"
    out.write_text(json.dumps(results, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print("wrote", out, flush=True)
    if not all(r.get("ok") for r in results):
        raise SystemExit(1)


if __name__ == "__main__":
    main()
