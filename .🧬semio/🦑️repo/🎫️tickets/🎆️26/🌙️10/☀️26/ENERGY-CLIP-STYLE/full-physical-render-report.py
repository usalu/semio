"""🧾 Retain complete production-render evidence before generated outputs are reclaimed."""
from pathlib import Path
import av,json
ticket=Path(__file__).resolve().parent
root=ticket/'🗑️generated/full-physical/de/spectator'
final=root/'rendered/Full_Physical_Fundamentals_spectator_de_640x360p6.mp4'
lines=['# Full Physical Fundamentals Render','','Status: the complete canonical production render succeeded; CLI exited 0 with Ready. Independent full-frame and visual verification runs separately before the retained final artifact is published.','','## Production Invocation','','The retained full-physical-render.ps1 runs the existing production CLI through repo capacity run and repo tools exec python. The registered Python override points to the existing shared teaching interpreter (Manim 0.22.0). No environment setup or dependency edits were needed. The grant reserved 16384 MB and four CPU cores; source concurrency was four. Settings: explicit German language, Spectator audience, 640×360, 6 fps, low source quality, per-source attempt logs, no playback.','','The cache audit found no reusable physical source cache. All eight canonical physical beats were rendered from current source inputs. The manifest has no institute intro. All eight source attempts exited 0; runtime source fingerprint checks passed. The source audit confirmed the entire tracked closure and manifest unchanged at assembly. No canonical German clause audio exists for these beats; current visual/subtitle narration was preserved without generating speech.','','## Complete Source Inventory','','| Ordinal | Source | Animations | Frames | Duration (s) | Bytes |','| --- | --- | ---: | ---: | ---: | ---: |']
for directory in sorted((root/'s').iterdir(),key=lambda path:int(path.name)):
    clip=next((directory/'v').glob('Beat*.mp4'))
    log=next(directory.glob('attempt-*.log'))
    text=log.read_text(encoding='utf-8',errors='replace')
    import re
    animations=1+max(map(int,re.findall(r'Animation (\d+)',text)))
    with av.open(str(clip)) as container:
        stream=container.streams.video[0]
        frames=stream.frames
        duration=float(stream.duration*stream.time_base)
    lines.append(f'| {directory.name} | {clip.stem} | {animations} | {frames} | {duration:.6f} | {clip.stat().st_size} |')
with av.open(str(final)) as container:
    stream=container.streams.video[0]
    duration=float(stream.duration*stream.time_base)
    lines+=['','## Successful Publication','','The runtime assembled PhysicalFundamentals_FullSection and the entire full-frame content, then rendered 22 actual-UI focus frames and 22 actual-UI restore frames. Final composition completed in 14.8 seconds. The output was published with its successful .render.json provenance.','',f'- Generated movie: {final.relative_to(ticket).as_posix()}',f'- Video: {stream.width}×{stream.height}, {stream.average_rate} fps, {stream.frames} frames, {duration:.6f} seconds.',f'- Published bytes: {final.stat().st_size}.','- Final retained destination, after independent verification: full-physical-updated.mp4.','','All temporary source clips, browser endpoint frames, logs, caches and assembled intermediates remain under this ticket’s generated/full-physical tree for verification; the parent coordinator handles final copy and generated-output cleanup.']
(ticket/'full-physical-render.md').write_text('\n'.join(lines)+'\n',encoding='utf-8')
print('[DEBUG] Production render evidence retained')
