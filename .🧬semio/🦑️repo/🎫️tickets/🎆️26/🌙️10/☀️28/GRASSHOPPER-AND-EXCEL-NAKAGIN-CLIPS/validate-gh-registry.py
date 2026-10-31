"""📇 Record the current Grasshopper registry with all dynamic scene and curriculum inputs explicit."""
import importlib
import json
from pathlib import Path
import runpy
import sys
sys.dont_write_bytecode=True
TICKET=Path(__file__).resolve().parent
ROOT=next(p for p in TICKET.parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
CLIP=ROOT/'🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip'
TARGET=CLIP/'🧪️tests/📇️registry/🐍️.py'
registry=runpy.run_path(str(TARGET))
cases=registry['CASES']
inputs=[Path(__file__),TARGET.with_name('🔣️.cases.json'),ROOT/'🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json',registry['AUTHORING']/'🪟️presentation/🔣️.json',registry['AUTHORING']/'🪟️presentation/📚️collection/🔣️.json',CLIP/'🐍️.py']
def owned(module):
    """🧩 Include modules reached through the entry loader as well as registered imports."""
    filename=getattr(module,'__file__',None)
    if filename:
        path=Path(filename).resolve()
        if path.is_relative_to(ROOT) and not {'site-packages','dist-packages','__pycache__'}&set(path.parts):
            inputs.append(path)
    for value in vars(module).values():
        path=getattr(value,'__file__',None)
        if path and Path(path).resolve().is_relative_to(ROOT) and not {'site-packages','dist-packages','__pycache__'}&set(Path(path).parts):
            inputs.append(Path(path).resolve())
for chapter in cases['chapters']:
    owned(importlib.import_module(f"🎓️teaching.🏛️architecture.{cases['folder']}.🎬️clip.{chapter['folder']}.🐍️"))
    entry=ROOT/'🎓️teaching'/chapter['entry']
    inputs.append(entry)
    owned(registry['load'](chapter['video_id'],entry))
owned(registry['load'](cases['series']+'_playlist',CLIP/'🐍️.py'))
inputs.extend(CLIP/f'🗣️transcript.{language}.md' for language in cases['languages'])
args=[str(TICKET/'validate-renderer-contracts.py'),'--output-name','gh-registry-final','--target',str(TARGET.relative_to(ROOT))]
for path in dict.fromkeys(inputs):
    args.extend(('--input',str(path.relative_to(ROOT))))
sys.argv=args
runpy.run_path(str(TICKET/'validate-renderer-contracts.py'),run_name='__main__')
