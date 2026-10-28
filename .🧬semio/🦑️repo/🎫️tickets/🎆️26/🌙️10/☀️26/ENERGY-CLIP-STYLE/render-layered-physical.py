"""🎬 Render a current physical beat with shared layered teaching navigation."""
import argparse,importlib,json,os,shutil,sys
from pathlib import Path
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
sys.dont_write_bytecode=True
BASE='🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring'
runtime=importlib.import_module(BASE+'.🎬️clip.⚙️runtime.🐍️')
args=argparse.ArgumentParser()
args.add_argument('--source-only',action='store_true')
args.add_argument('--use-captured-excerpt',action='store_true')
option=args.parse_args()
output=Path(os.environ['SEMIO_TEACHING_OUTPUT_ROOT']).resolve()
output.mkdir(parents=True,exist_ok=True)
settings={'quality':'-ql','resolution':'1280,720','fps':8,'language':'de','audience':'spectator'}
segment={'script':'🏛️architecture/⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py','scene':'Beat1_Leistung'}
content=output/'physical-excerpt.mp4'
if option.use_captured_excerpt:
    if not content.is_file():raise FileNotFoundError(content)
else:
    source=runtime.render_segment(segment,output/'source',settings,output_root=output,log_output=True)
    runtime.cut_clip(source,content,0,18)
if option.source_only:
    print('[DEBUG] Current physical source excerpt ready',flush=True)
    sys.exit(0)
p=importlib.import_module(BASE+'.🎬️clip.🪟️presentation.🐍️')
result=output/'layered-physical.mp4'
p.render_presentation('physical_fundamentals',content,result,settings)
shutil.copy2(result,Path(__file__).with_name('layered-physical.mp4'))
print('[DEBUG] Layered physical preview retained',flush=True)
