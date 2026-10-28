"""🎬 Review fullscreen icons embedded in window names while retaining content-only playback."""
import importlib,json,os,shutil,sys,time
from pathlib import Path
import av
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT));sys.dont_write_bytecode=True
base='🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring'
p=importlib.import_module(base+'.🎬️clip.🪟️presentation.🐍️')
r=importlib.import_module(base+'.🎬️clip.⚙️runtime.🐍️')
ticket=Path(__file__).resolve().parent
output=Path(os.environ['SEMIO_TEACHING_OUTPUT_ROOT']);output.mkdir(parents=True,exist_ok=True)
settings={'language':'de','audience':'spectator','quality':'-ql','resolution':'1280,720','fps':8}
body=output/'body.mp4'
r.run_command([*r.ffmpeg_command(),'-hide_banner','-loglevel','error','-y','-i',str(ticket/'content-camera.mp4'),'-vf','trim=start_frame=29:end_frame=37,setpts=N/(8*TB)','-an','-r','8','-fps_mode','cfr','-frames:v','8','-c:v','libx264','-preset','fast','-crf','0','-pix_fmt','yuv420p',str(body)],log=output/f'extract-{time.time_ns()}.log')
movie=output/'title-chip.mp4'
p.render_presentation('physical_fundamentals',body,movie,settings)
shutil.copy2(movie,ticket/'title-chip.mp4')
with av.open(str(movie)) as source:
    frames=list(source.decode(video=0))
    for name,index in [('overview',0),('opened',23),('body',32),('control',47)]:frames[min(index,len(frames)-1)].to_image().save(ticket/('title-chip-'+name+'.png'))
    info={'frames':len(frames),'fps':8,'seconds':len(frames)/8,'body_start':round(p.transition_duration('focus')*8)}
(output/'preview-info.json').write_text(json.dumps(info),encoding='utf8')
(ticket/'title-chip-preview.md').write_text('# Window Name Chip Preview\n\n[Review video](./title-chip.mp4) shows fullscreen controls embedded in the overview and opened window name chips. Ansehen remains a text-only right footer chip. Content-only playback and camera transitions are preserved. The one-second source input was extracted from the already retained physical review.\n\n'+json.dumps(info,indent=2)+'\n',encoding='utf8')
print('[DEBUG] Window name fullscreen chip preview retained',flush=True)
