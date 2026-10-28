"""🎥 Apply the measured content camera implementation through owned atomic writes."""
from pathlib import Path
import json
import os
import time

ROOT = Path(__file__).resolve().parents[7]
BASE = ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation"
WORK = Path(__file__).parent / "🗑️generated/content-camera"
WORK.mkdir(parents=True, exist_ok=True)


def publish(relative, value):
    """💾 Replace only the named source using one short owned temporary path."""
    target = BASE / relative
    temporary = WORK / (str(len(list(WORK.iterdir()))) + ".tmp")
    temporary.write_text(value, encoding="utf-8")
    for attempt in range(10):
        try:
            os.replace(temporary, target)
            return
        except OSError:
            if attempt == 9:
                raise
            time.sleep(.1)


def edit(relative, changes):
    """🧩 Preserve every unrelated source span while applying exact owned edits."""
    value = (BASE / relative).read_text(encoding="utf-8")
    for before, after in changes:
        if before not in value:
            raise RuntimeError(f"Missing source span in {relative}: {before[:100]}")
        value = value.replace(before, after, 1)
    publish(relative, value)


schema = json.loads((BASE / "🧬️schema.json").read_text(encoding="utf-8"))
schema["required"].append("contentCamera")
schema["properties"]["contentCamera"] = {"type": "object", "required": ["duration"], "properties": {"duration": {"type": "number", "exclusiveMinimum": 0}}, "additionalProperties": False}
publish("🧬️schema.json", json.dumps(schema, ensure_ascii=False, indent=2) + "\n")
edit("🔣️.json", [('  "timing": {', '  "contentCamera": {\n    "duration": 0.6\n  },\n  "timing": {')])
schema = json.loads((BASE / "🌐️browser/🧬️schema.json").read_text(encoding="utf-8"))
schema["$defs"]["frame"]["properties"]["content_camera"] = {"type": "object", "required": ["video_id", "progress"], "properties": {"video_id": {"type": "string", "minLength": 1}, "progress": {"type": "number", "minimum": 0, "maximum": 1}}, "additionalProperties": False}
publish("🌐️browser/🧬️schema.json", json.dumps(schema, ensure_ascii=False, indent=2) + "\n")

edit("🐍️.py", [
('def _settings(settings: dict)', '''def transition_duration(phase: str) -> float:
    """⏱️ Preserve native navigation timing and reserve a distinct content camera move."""
    if phase not in ("focus", "restore"):
        raise ValueError("A focus or restore transition phase is required")
    contract = presentation_contract()
    return sum(contract["timing" if phase == "focus" else "restoreTiming"].values()) + contract["contentCamera"]["duration"]


def _settings(settings: dict)'''),
('    timing = presentation_contract()["timing" if phase == "focus" else "restoreTiming"]\n    count = max(3, round(sum(timing.values()) * fps))', '    count = max(3, round(transition_duration(phase) * fps))'),
('    import shutil\n    import time', '    import time'),
('    run(["-i", str(content), "-frames:v", "1", "-update", "1", str(first)], "first.log")\n    run(["-sseof", "-1", "-i", str(content), "-vf", "reverse", "-frames:v", "1", "-update", "1", str(last)], "last.log")', '''    dark = _theme.color("dark").replace("#", "0x")
    fit = (f"scale={width}:{height}:force_original_aspect_ratio=decrease,"
           f"pad={width}:{height}:(ow-iw)/2:(oh-ih)/2:color={dark},setsar=1")
    run(["-i", str(content), "-vf", fit, "-frames:v", "1", "-update", "1", str(first)], "first.log")
    run(["-sseof", "-1", "-i", str(content), "-vf", "reverse," + fit, "-frames:v", "1", "-update", "1", str(last)], "last.log")'''),
('''    outward, _ = _transition(video_id, last, work, settings, session, phase="restore")
    shell = render_shell(video_id, work / "shell.png", settings)
    activity = runtime.cursor_intervals(content)
    hover_shell = render_shell(video_id, work / "hover.png", settings, hovered=True) if activity else None
    viewport = focus_viewport(width, height)
    x, y, w, h = (round(viewport[key]) for key in ("x", "y", "width", "height"))
    w, h = w // 2 * 2, h // 2 * 2
    padding = count / fps
    total_count = body_count + 2 * count''', '''    outward, restore_count = _transition(video_id, last, work, settings, session, phase="restore")
    padding = count / fps
    total_count = body_count + count + restore_count'''),
('''    dark = _theme.color("dark").replace("#", "0x")
    shell_filter = f"[3:v]trim=duration={duration},setpts=PTS-STARTPTS[shell];"
    if activity:
        condition = "+".join(f"gte(t,{item['start']:.9f})*lt(t,{item['end']:.9f})" for item in activity)
        shell_filter = (f"[3:v]trim=duration={duration},setpts=PTS-STARTPTS[base];"
                        f"[4:v]trim=duration={duration},setpts=PTS-STARTPTS[hover];"
                        f"[base][hover]overlay=0:0:shortest=1:enable='{condition}'[shell];")
''', ''),
('''        f"scale={w}:{h}:force_original_aspect_ratio=decrease,"
        f"pad={w}:{h}:(ow-iw)/2:(oh-ih)/2:color={dark},setsar=1[body];"
        + shell_filter +
        f"[shell][body]overlay={x}:{y}:shortest=1,format=yuv420p[focused];"''', '''        f"{fit},format=yuv420p[body];"'''),
('[intro][focused][outro]', '[intro][body][outro]'),
('''            "-loop", "1", "-framerate", str(fps), "-i", str(shell),
            *(["-loop", "1", "-framerate", str(fps), "-i", str(hover_shell)] if hover_shell else []),
''', ''),
('    duration = body_count / fps\n', ''),
])

edit("🥞️hierarchy/🖌️render/🐍️.py", [
('    seconds = t * boundaries[-1]\n', '''    camera_duration = _hierarchy.hierarchy_contract()["contentCamera"]["duration"]
    elapsed = t * (boundaries[-1] + camera_duration)
    offset = camera_duration if phase == "restore" else 0
    seconds = max(0, min(boundaries[-1], elapsed - offset))
    fraction = max(0, min(1, (elapsed - boundaries[-1]) / camera_duration if phase == "focus" else elapsed / camera_duration))
    eased = fraction * fraction * (3 - 2 * fraction)
    content_camera = eased if phase == "focus" else 1 - eased
'''),
('''    events = [{"type": kind, "time": at, "point": list(position)} for at, kind, position, _ in milestones if at <= seconds]
''', '''    events = [{"type": kind, "time": at + offset, "point": list(position)} for at, kind, position, _ in milestones if at + offset <= elapsed]
'''),
('''               for at, _, _, effect in milestones if at <= seconds and effect]
    return {"phase": phase, "t": t, "seconds": seconds, "stage": stages[stage],''', '''               for at, _, _, effect in milestones if at + offset <= elapsed and effect]
    return {"phase": phase, "t": t, "seconds": elapsed, "navigation_seconds": seconds, "content_camera": content_camera, "stage": stages[stage],'''),
('''                        "events": [], "effects": [], "action": None}
''', '''                        "events": [], "effects": [], "action": None, "content_camera": 0}
'''),
('''"visible": not request.get("shell", False),''', '''"visible": not request.get("shell", False) and timeline["content_camera"] == 0,'''),
('''        projected.append((frame, timeline, request))''', '''        frame["content_camera"] = {"video_id": video_id, "progress": timeline["content_camera"]}
        projected.append((frame, timeline, request))'''),
('''        if request.get("shell"):
''', '''        if timeline["content_camera"] == 1 and request.get("snapshot"):
            from PIL import Image
            import shutil

            with Image.open(request["snapshot"]) as source:
                if source.size == (width, height):
                    shutil.copyfile(request["snapshot"], frame["output"])
        if request.get("shell"):
'''),
])

edit("🌐️browser/🟦️.tsx", [
('''  debugGeometry(){''', '''  contentCamera(value:any){
    const app=document.getElementById("app")!;
    app.style.transform="";app.style.transformOrigin="0 0";
    const progress=value?.progress??0;
    if(!progress)return {progress:0,scale:1,translation:[0,0],target:null};
    const host=[...document.querySelectorAll("[data-presentation-body]")].find(element=>element.getAttribute("data-presentation-body")===value.video_id),image=host?.querySelector("img"),box=host?.getBoundingClientRect();
    if(!box)throw Error("Content camera requires an actual video body");
    const aspect=image?.naturalWidth&&image?.naturalHeight?image.naturalWidth/image.naturalHeight:1600/900;
    const width=Math.min(box.width,box.height*aspect),height=width/aspect,target={x:box.x+(box.width-width)/2,y:box.y+(box.height-height)/2,width,height};
    const scale=1+progress*(Math.min(1600/width,900/height)-1),translation=[progress*(800-(target.x+width/2)*Math.min(1600/width,900/height)),progress*(450-(target.y+height/2)*Math.min(1600/width,900/height))];
    app.style.transform=`matrix(${scale},0,0,${scale},${translation[0]},${translation[1]})`;
    return {progress,scale,translation,target};
  },
  debugGeometry(){'''),
])

edit("🌐️browser/🟨️render.mjs", [
('''    const frame=batch.frames[index];''', '''    const frame=batch.frames[index];
    await page.evaluate(()=>window.semioPresentation.contentCamera(null));'''),
('''    await fs.mkdir(path.dirname(frame.output),{recursive:true});''', '''    info.content_camera=await page.evaluate(value=>window.semioPresentation.contentCamera(value),frame.content_camera??null);
    await fs.mkdir(path.dirname(frame.output),{recursive:true});'''),
])
print("[DEBUG] Content camera implementation published")
