"""🔄 Replace original child renders whose owning agent sessions ended."""

from concurrent.futures import ThreadPoolExecutor
import importlib.util
import json
from pathlib import Path
import sys

sys.dont_write_bytecode = True
ticket = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("parallel_monitor", ticket / "parallel-monitor.py")
monitor = importlib.util.module_from_spec(spec)
spec.loader.exec_module(monitor)
jobs = []
for number, video_id in [(22, "heating"), (23, "energy_balance")]:
    index, video = next((index, video) for index, video in enumerate(monitor.load_manifest(), 1) if video["id"] == video_id)
    jobs.append((number, index, video, monitor.ROOT / video["script"], video["scenes"][0], None))
mapping_path = monitor.PRE / "jobs.json"
mapping = json.loads(mapping_path.read_text(encoding="utf-8"))
mapping.extend({"job": number, "video_index": index, "video": video["id"], "source": str(source),
                "scene": name, "pid": pid, "media": str(monitor.PRE / str(number))}
               for number, index, video, source, name, pid in jobs)
mapping_path.write_text(json.dumps(mapping, indent=2) + "\n", encoding="utf-8")
with ThreadPoolExecutor(max_workers=2) as pool:
    list(pool.map(monitor.render, jobs))
print("[DEBUG] Replacement original chapters staged successfully", flush=True)
