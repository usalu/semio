"""👁️ Summarize only current GH renderer attempts and receipt status."""
from pathlib import Path
import re
root=Path(".🧬semio/🎓️teaching/⚡️cache/videos/en/speaker/s")
for number, scene in (("120","Beat1_Anatomy"),("121","Beat2_Capsule"),("124","Beat5_DataTrees")):
    directory=root/number
    logs=list(directory.glob("attempt-*.log"))
    if not logs:
        continue
    latest=max(logs,key=lambda path:path.stat().st_mtime)
    with latest.open("rb") as stream:
        stream.seek(max(0,latest.stat().st_size-32768))
        tail=stream.read().decode("utf-8",errors="replace")
    progress=re.findall(r"Animation \d+:[^\r\n]*",tail)
    errors=[line for line in tail.splitlines() if "Traceback" in line or "Error:" in line or "Exception:" in line]
    receipt=directory/"v"/(scene+".render.json")
    print(number,latest.name,"bytes",latest.stat().st_size,"receipt",receipt.exists(),"progress",progress[-1] if progress else "-", "errors",errors[-2:])
