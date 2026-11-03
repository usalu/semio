import subprocess, sys, time
from pathlib import Path

out = Path(sys.argv[1]) / "memory.log"
while True:
    rows = subprocess.run(
        ["powershell", "-NoProfile", "-Command",
         "Get-CimInstance Win32_Process | Where-Object { $_.CommandLine -like '*agent-bv-bim-verification*' } | "
         "ForEach-Object { '{0} ws={1}MiB' -f $_.ProcessId, [int]($_.WorkingSetSize/1MB) }"],
        capture_output=True, text=True).stdout.strip().replace("\n", " | ")
    free = subprocess.run(
        ["powershell", "-NoProfile", "-Command", "[int]((Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory/1024)"],
        capture_output=True, text=True).stdout.strip()
    with out.open("a", encoding="utf-8") as handle:
        handle.write(f"{time.strftime('%H:%M:%S')} free={free}MiB procs: {rows or 'NONE'}\n")
    time.sleep(300)
