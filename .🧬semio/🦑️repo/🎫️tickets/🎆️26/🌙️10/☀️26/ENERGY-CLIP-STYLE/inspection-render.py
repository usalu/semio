"""🎬 Persist the complete catalog inspection render and its progress."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time

ROOT = next(p for p in Path(__file__).resolve().parents if (p / '🧰️framework').is_dir())
TICKET = Path(__file__).resolve().parent
OUTPUT = TICKET / "🗑️generated" / "i"
ENTRY = ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🖌️render/🐍️.py"
os.environ["PYTHONUTF8"] = "1"
os.environ["PYTHONIOENCODING"] = "utf-8"
REPO = os.environ.get("SEMIO_REPO_EXE") or shutil.which("repo") or "repo"
COMMAND = [REPO, "tools", "exec", "python", "--light", "--", "-X", "utf8", str(ENTRY),
           "--language", "de", "--audience", "spectator", "--resolution", "480,270",
           "--fps", "4", "--jobs", "8", "--log-output", "--no-play",
           "--output-root", str(OUTPUT)]

_parser = argparse.ArgumentParser(description=__doc__)
_parser.add_argument('--repair', action='append', choices=('ventilation', 'cooling', 'energy_balance'))
_parser.add_argument('--inventory', action='append', choices=('ventilation', 'cooling', 'energy_balance'))
_args = _parser.parse_args()
REPAIR = list(dict.fromkeys(_args.repair or []))
INVENTORY = list(dict.fromkeys(_args.inventory or REPAIR))
REPAIR_KEY = '-'.join(REPAIR)
if REPAIR:
    COMMAND[COMMAND.index('--jobs') + 1] = '8'
    for identity in REPAIR:
        COMMAND.extend(['--video', identity])

def save_status(**values):
    """🧾 Publish progress atomically for independent inspection."""
    destination = OUTPUT / (f'repair-{REPAIR_KEY}-status.json' if REPAIR else 'status.json')
    temporary = OUTPUT / (f'repair-{REPAIR_KEY}-status.new' if REPAIR else 'status.new')
    temporary.write_text(json.dumps(values, ensure_ascii=False, indent=2), encoding="utf-8")
    temporary.replace(destination)

def main():
    """▶️ Run all manifest videos with a durable log and capacity inherited from repo."""
    OUTPUT.mkdir(parents=True, exist_ok=True)
    started = time.time()
    command_path = OUTPUT / (f'repair-{REPAIR_KEY}-command.json' if REPAIR else 'command.json')
    command_path.write_text(json.dumps(COMMAND, ensure_ascii=False, indent=2), encoding="utf-8")
    save_status(state="starting", started=started, command=COMMAND)
    with (OUTPUT / (f'repair-{REPAIR_KEY}.log' if REPAIR else 'master.log')).open("w", encoding="utf-8", buffering=1) as log:
        print("[DEBUG] Starting production inspection render", flush=True)
        if INVENTORY:
            subprocess.run([REPO, 'tools', 'exec', 'python', '--light', '--', '-X', 'utf8', str(TICKET / 'inspection-repair-plan.py'), *INVENTORY], cwd=ROOT, check=True)
        process = subprocess.Popen(COMMAND, cwd=ROOT, stdout=subprocess.PIPE,
                                   stderr=subprocess.STDOUT, text=True, encoding="utf-8",
                                   errors="replace", bufsize=1)
        save_status(state="running", started=started, pid=process.pid, command=COMMAND)
        try:
            for line in process.stdout:
                log.write(line)
                print(line, end="", flush=True)
                save_status(state="running", started=started, updated=time.time(),
                            pid=process.pid, last_line=line.strip(), command=COMMAND)
            result = process.wait()
        except KeyboardInterrupt:
            process.terminate()
            process.wait()
            result = 130
        save_status(state="completed" if result == 0 else "failed", started=started,
                    completed=time.time(), exit_code=result, command=COMMAND)
    return result

if __name__ == "__main__":
    raise SystemExit(main())