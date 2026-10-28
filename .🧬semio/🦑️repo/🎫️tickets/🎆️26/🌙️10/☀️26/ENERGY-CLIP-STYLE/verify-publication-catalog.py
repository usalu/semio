"""📸 Verify publication certificates own the actual finite catalog input."""
import importlib,sys
from pathlib import Path
from unittest.mock import patch
root=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(root));sys.dont_write_bytecode=True
base='🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip'
p=importlib.import_module(base+'.🪟️presentation.🐍️')
r=importlib.import_module(base+'.⚙️runtime.🐍️')
h=p._hierarchy
one=h.hierarchy_contract();two=h.hierarchy_contract();two['identity']['brand']='changed-capture'
with patch.object(p,'source_files',return_value=h.source_files()):
    with h.capture_catalog(one):first=r.presentation_state('physical_fundamentals',{'language':'de'},'de')
    with h.capture_catalog(two):second=r.presentation_state('physical_fundamentals',{'language':'de'},'de')
assert first.get('catalog')==one,'Publication certificate omits its actual captured catalog'
assert second.get('catalog')==two and first!=second
catalog_path=str(Path(h.__file__).resolve().parent.parent/'🔣️.json')
assert catalog_path not in first['sources'],'Live catalog hash duplicates the captured model input'
first['catalog']['identity']['brand']='caller'
assert one['identity']['brand']!='caller'
print('[DEBUG] Publication captured-catalog identity verified',flush=True)
