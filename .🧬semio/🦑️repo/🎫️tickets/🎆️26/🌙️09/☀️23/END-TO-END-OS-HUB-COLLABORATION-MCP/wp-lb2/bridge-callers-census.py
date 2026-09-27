#!/usr/bin/env python3
"""🧾️ LB2 census (read-only): per plugin, the apps its staged descriptor declares vs the app-level bridge-law calls
(`assert_declared_actions_bridge_to_commands::<…>`) its Rust tests make — an app with no call is never proven to read only
its declared arguments. Usage: bridge-callers-census.py"""
import json
import re
import subprocess
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
calls = subprocess.run(["git", "grep", "-n", "assert_declared_actions_bridge_to_commands::<", "--", "✏️s/*.rs"], cwd=ROOT, capture_output=True, text=True).stdout.splitlines()
per_plugin = {}
for line in calls:
    plugin = line.split("/")[2]
    per_plugin.setdefault(plugin, []).append(re.search(r"::<(.+?)>\(", line).group(1) if re.search(r"::<(.+?)>\(", line) else "?")
total_apps = total_called = 0
for descriptor in sorted((ROOT / "✏️s/🔌️plugins").glob("*/🔣️.json")):
    plugin = descriptor.parent.name
    manifest = json.loads(descriptor.read_text(encoding="utf-8"))["manifest"]
    apps = [app["id"] for app in manifest["apps"] if app["id"].endswith("#editor") or "#" not in app["id"]]
    called = per_plugin.get(plugin, [])
    total_apps += len(apps)
    total_called += len(called)
    print(f"{plugin}\teditor-apps={len(apps)}\tbridge-law-calls={len(called)}\t{', '.join(called)[:160]}")
print(f"TOTAL editor apps {total_apps}, bridge-law calls {total_called}")
