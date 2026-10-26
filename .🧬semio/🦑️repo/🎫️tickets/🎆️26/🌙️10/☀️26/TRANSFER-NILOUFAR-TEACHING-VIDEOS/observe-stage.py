"""🧾 Record staged native process lifetimes and immutable attempt logs without control actions."""

import ctypes
from ctypes import wintypes
from datetime import datetime, timezone
import json
from pathlib import Path
import re
import subprocess
import time

PRE = Path(__file__).resolve().parent / "🗑️generated/pre"
KERNEL = ctypes.WinDLL("kernel32", use_last_error=True)
KERNEL.OpenProcess.argtypes = [wintypes.DWORD, wintypes.BOOL, wintypes.DWORD]
KERNEL.OpenProcess.restype = wintypes.HANDLE
KERNEL.GetExitCodeProcess.argtypes = [wintypes.HANDLE, ctypes.POINTER(wintypes.DWORD)]
KERNEL.GetExitCodeProcess.restype = wintypes.BOOL
KERNEL.CloseHandle.argtypes = [wintypes.HANDLE]
handles = {}
finished = []
query = "[Console]::OutputEncoding = [Text.UTF8Encoding]::new(); Get-CimInstance Win32_Process | Where-Object { $_.Name -eq 'python.exe' -and $_.CommandLine -like '*generated\\pre\\*' -and $_.CommandLine -like '*cpython-*' } | Select-Object ProcessId,ParentProcessId,CommandLine | ConvertTo-Json -Compress"


def event(data):
    """📝 Persist a diagnostic event beneath the ticket's generated directory."""
    with (PRE / "process-events.log").open("a", encoding="utf-8") as log:
        log.write("[DEBUG] " + json.dumps(data) + "\n")


while True:
    snapshot = subprocess.run(["powershell", "-NoProfile", "-Command", query], capture_output=True, encoding="utf-8", check=True)
    records = json.loads(snapshot.stdout) if snapshot.stdout.strip() else []
    if isinstance(records, dict):
        records = [records]
    for record in records:
        match = re.search(r"\\pre\\(\d+)\\render.cfg", record["CommandLine"])
        pid = record["ProcessId"]
        if not match or pid in handles or any(item["pid"] == pid for item in finished):
            continue
        handle = KERNEL.OpenProcess(0x1000, False, pid)
        if handle:
            details = {"job": int(match.group(1)), "pid": pid, "parent": record["ParentProcessId"],
                       "observed": datetime.now(timezone.utc).isoformat()}
            handles[pid] = (handle, details)
            event({"event": "observed", **details})
    for pid, (handle, details) in list(handles.items()):
        code = wintypes.DWORD()
        if KERNEL.GetExitCodeProcess(handle, ctypes.byref(code)) and code.value != 259:
            result = {**details, "exit_code": code.value, "finished": datetime.now(timezone.utc).isoformat()}
            finished.append(result)
            event({"event": "exited", **result})
            source = PRE / str(details["job"]) / "render.log"
            if source.is_file():
                destination = source.with_name(f"attempt-{pid}.log")
                try:
                    with destination.open("xb") as log:
                        log.write(source.read_bytes())
                except FileExistsError:
                    pass
            KERNEL.CloseHandle(handle)
            del handles[pid]
            print(f"[DEBUG] Observed completed stage job{details['job']} PID{pid} exit{code.value}", flush=True)
    (PRE / "process-exits.json").write_text(json.dumps(finished, indent=2) + "\n", encoding="utf-8")
    if sum(1 for _ in PRE.glob("*/v/*.render.json")) >= 23 and not handles:
        break
    time.sleep(5)
