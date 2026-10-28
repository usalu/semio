"""🎥 Review transition-only UI with captured physical source content filling the playback frame."""
import hashlib,importlib,json,os,shutil,sys,time
from pathlib import Path
import av
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT));sys.dont_write_bytecode=True
base='🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring'
p=importlib.import_module(base+'.🎬️clip.🪟️presentation.🐍️')
r=importlib.import_module(base+'.🎬️clip.⚙️runtime.🐍️')
ticket=Path(__file__).resolve().parent
output=Path(os.environ['SEMIO_TEACHING_OUTPUT_ROOT']).resolve();output.mkdir(parents=True,exist_ok=True)
settings={'language':'de','audience':'spectator','quality':'-ql','resolution':'1280,720','fps':8}
captured=ticket/'layered-physical.mp4'
with av.open(str(captured)) as movie:
    source=movie.streams.video[0]
    assert(source.width,source.height,float(source.average_rate))==(1280,720,8)
content=output/'captured-body.mp4'
if '--use-captured-body' not in sys.argv:
    r.run_command([*r.ffmpeg_command(),'-hide_banner','-loglevel','error','-y','-i',str(captured),'-vf','trim=start_frame=24:end_frame=72,setpts=PTS-STARTPTS,crop=1148:646:66:46,scale=1280:720:flags=lanczos,setsar=1','-an','-c:v','libx264','-preset','fast','-crf','0','-pix_fmt','yuv420p',str(content)],log=output/f'extract-{time.time_ns()}.log')
result=output/'content-camera.mp4'
p.render_presentation('physical_fundamentals',content,result,settings)
shutil.copy2(result,ticket/'content-camera.mp4')
with av.open(str(result)) as movie:
    stream=movie.streams.video[0]
    frames=list(movie.decode(stream));duration=len(frames)/float(stream.average_rate)
    for name,second in [('overview',0),('zoom',3.25),('body',5),('return',10)]:frames[min(len(frames)-1,round(second*8))].to_image().save(ticket/('content-camera-'+name+'.png'))
with av.open(str(content)) as source:decoded_source_frames=sum(1 for _ in source.decode(video=0))
info={'decoded_source_frames':decoded_source_frames,'padding_frames':[max(3,round(p.transition_duration(phase)*8)) for phase in ['focus','restore']],'source':captured.name,'source_sha256':hashlib.sha256(captured.read_bytes()).hexdigest(),'source_frame_interval':[24,72],'source_ink_crop':[66,46,1148,646],'source_output':[1280,720],'frames':len(frames),'seconds':duration,'fps':8}
(output/'preview-info.json').write_text(json.dumps(info),encoding='utf8')
(ticket/'content-camera-preview.md').write_text('# Transition-Only UI Preview\n\n[Review video](./content-camera.mp4) contains a captured physical source excerpt, with the updated opening and closing UI/camera transitions. Playback fills the frame without the outer virtual UI.\n\nThe source excerpt was recovered from the retained earlier 24-second physical review film, frames 24–71, cropping its recorded source-image ink rectangle before resizing to 1280 × 720. It is a visual review input, not a newly certified full lesson render.\n\n'+json.dumps(info,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
print('[DEBUG] Content-only playback preview retained',flush=True)
