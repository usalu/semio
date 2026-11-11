import importlib
from pathlib import Path
r=importlib.import_module('🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️')
root=Path('🎓️teaching').resolve()
base=root/'🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/📐️drafting/🧩️surfaces/🐍️.py'
region=root/'🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/📐️drafting/🧩️regions/🐍️.py'
for v in r.load_manifest():
 hits=set()
 for chapter in v['chapters']:
  for seg in chapter['segments']:
   deps={p.resolve() for p in r.source_dependencies(root/seg['script'])}
   if base in deps:hits.add('surfaces')
   if region in deps:hits.add('regions')
 if hits:print(v['id'],','.join(sorted(hits)))
