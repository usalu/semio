import importlib
from pathlib import Path
r=importlib.import_module('🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️')
root=Path('🎓️teaching').resolve()
target=(root/'🏛️architecture/📊️excel/🎬️clip/📍️references/🐍️.py').resolve()
for video in r.load_manifest():
 found=[]
 for chapter in video['chapters']:
  for segment in chapter['segments']:
   if target in {p.resolve() for p in r.source_dependencies(root/segment['script'])}:found.append(segment['scene'])
 if found:print(video['id'],found)
