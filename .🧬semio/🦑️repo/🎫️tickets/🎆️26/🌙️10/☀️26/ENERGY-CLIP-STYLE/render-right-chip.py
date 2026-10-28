"""🎬 Show the single right-side fullscreen chip through the actual layered UI."""
import importlib,json,os,shutil,sys,time
from pathlib import Path
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT));sys.dont_write_bytecode=True
base='🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring'
p=importlib.import_module(base+'.🎬️clip.🪟️presentation.🐍️')
r=importlib.import_module(base+'.🎬️clip.⚙️runtime.🐍️')
output=Path(os.environ['SEMIO_TEACHING_OUTPUT_ROOT']).resolve();output.mkdir(parents=True,exist_ok=True)
settings={'language':'de','quality':'-ql','resolution':'1600,900','fps':8,'audience':'spectator'}
requests=[]
for phase in ['focus','restore']:
    if phase=='restore':
        for i in range(8):requests.append({'video_id':'physical_fundamentals','phase':'focus','t':1,'output':output/'f'/f'{len(requests):04d}.png','transition_elapsed':125})
    for i in range(24):requests.append({'video_id':'physical_fundamentals','phase':phase,'t':i/23,'output':output/'f'/f'{len(requests):04d}.png'})
frames=p._hierarchy_render.frame_batch(requests,settings)
(output/'metadata.json').write_text(json.dumps(frames,ensure_ascii=False,default=str),encoding='utf8')
r.run_command([*r.ffmpeg_command(),'-hide_banner','-loglevel','error','-y','-framerate','8','-i',str(output/'f'/'%04d.png'),'-an','-c:v','libx264','-preset','fast','-crf','16','-pix_fmt','yuv420p','-movflags','+faststart',str(output/'right-chip.mp4')],log=output/f'encode-{time.time_ns()}.log')
shutil.copy2(output/'right-chip.mp4',Path(__file__).with_name('right-chip.mp4'))
for name,index in [('overview',0),('video',14),('fullscreen',27)]:shutil.copy2(output/'f'/f'{index:04d}.png',Path(__file__).with_name('right-chip-'+name+'.png'))
print('[DEBUG] Right-side fused-chip preview retained',flush=True)
