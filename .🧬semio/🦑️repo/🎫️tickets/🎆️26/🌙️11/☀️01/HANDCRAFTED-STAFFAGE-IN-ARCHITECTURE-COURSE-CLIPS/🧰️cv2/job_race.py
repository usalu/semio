import importlib
import subprocess
import time

S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
COMMAND = ["cmd.exe", "/c", "ping -n 40 127.0.0.1 >nul"]


def pings() -> int:
    return subprocess.run(["tasklist", "/FI", "IMAGENAME eq PING.EXE", "/NH", "/FO", "CSV"], capture_output=True, text=True, check=False).stdout.lower().count("ping.exe")


def clean() -> None:
    subprocess.run(["taskkill", "/IM", "PING.EXE", "/F"], capture_output=True, check=False)
    time.sleep(0.5)


clean()
print("pings before:", pings())
for _ in range(10):
    plain = subprocess.Popen(COMMAND, stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    time.sleep(0.25)
    plain.kill()
print("control (plain kill of the parent, no job), 10 runs, grandchildren left:", pings())
clean()
RUNS = 150
for _ in range(RUNS):
    tree = S._Tree(COMMAND)
    time.sleep(0.25)
    tree.kill()
    tree.close()
    tree.process.stderr.close()
time.sleep(1.0)
print(f"job object, {RUNS} runs, grandchildren that escaped the job:", pings())
clean()
