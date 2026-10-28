"""🎞️ Share exact frame normalization across body and transition source snapshots."""
from pathlib import Path
import ast
import json
import os

ROOT = Path(__file__).resolve().parents[7]
BASE = ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation"
WORK = Path(__file__).parent / "🗑️generated/content-camera"
WORK.mkdir(parents=True, exist_ok=True)


def publish(relative, value):
    """💾 Publish the latest exact owned source without touching unrelated spans."""
    target = BASE / relative
    if target.suffix == ".py":
        ast.parse(value)
    temporary = WORK / "finish.tmp"
    temporary.write_text(value, encoding="utf-8")
    os.replace(temporary, target)


relative = "🐍️.py"
value = (BASE / relative).read_text(encoding="utf-8")
start, end = value.index("def render_shell("), value.index("def _transition(")
value = value[:start] + value[end:]
before = '''    run(["-i", str(content), "-vf", fit, "-frames:v", "1", "-update", "1", str(first)], "first.log")
    run(["-sseof", "-1", "-i", str(content), "-vf", "reverse," + fit, "-frames:v", "1", "-update", "1", str(last)], "last.log")'''
after = '''    normalized = (f"setpts=N/({source_fps}*TB),tpad=stop_mode=clone:stop_duration={1 / source_fps},"
                  f"fps={fps}:eof_action=pass,trim=end_frame={body_count},{fit}")
    run(["-i", str(content), "-vf", normalized, "-frames:v", "1", "-update", "1", str(first)], "first.log")
    run(["-i", str(content), "-vf", normalized + f",select=eq(n\\\\,{body_count - 1})", "-frames:v", "1", "-update", "1", str(last)], "last.log")'''
if before not in value:
    raise RuntimeError("Snapshot normalization span changed")
value = value.replace(before, after, 1)
before = '''        f"[0:v]setpts=N/({source_fps}*TB),tpad=stop_mode=clone:stop_duration={1 / source_fps},"
        f"fps={fps}:eof_action=pass,trim=end_frame={body_count},"
        f"{fit},format=yuv420p[body];"'''
if before not in value:
    raise RuntimeError("Body normalization span changed")
value = value.replace(before, '        f"[0:v]{normalized},format=yuv420p[body];"', 1)
publish(relative, value)
relative = "🥞️hierarchy/🖌️render/🐍️.py"
value = (BASE / relative).read_text(encoding="utf-8")
start, end = value.index('        if request.get("shell"):'), value.index('        output.append(metadata)')
value = value[:start] + value[end:]
value = value[:value.index("\n\n\ndef shell(")] + "\n"
value = value.replace('not request.get("shell", False) and timeline["content_camera"] == 0', 'timeline["content_camera"] == 0', 1)
publish(relative, value)
relative = "🌐️browser/🧬️schema.json"
schema = json.loads((BASE / relative).read_text(encoding="utf-8"))
del schema["$defs"]["frame"]["properties"]["transparent_body"]
publish(relative, json.dumps(schema, ensure_ascii=False, indent=2) + "\n")
print("[DEBUG] Shared snapshot/body normalization and obsolete shell removal published")
