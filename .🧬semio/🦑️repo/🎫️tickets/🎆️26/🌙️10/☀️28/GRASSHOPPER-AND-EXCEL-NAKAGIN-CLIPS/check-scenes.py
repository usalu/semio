"""🔬 Compile and import every authored Grasshopper and Excel scene without running its lifecycle."""
import ast
import importlib
from pathlib import Path
import sys
root=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(root))
sys.stdout.reconfigure(errors='backslashreplace')
for series in ('🦗️grasshopper','📊️excel'):
    clip=root/'🎓️teaching/🏛️architecture'/series/'🎬️clip'
    for folder in sorted(clip.iterdir()):
        source=folder/'🐍️.py'
        if not folder.is_dir() or not source.is_file() or folder.name in ('🎨️visuals','🧱️model','🧪️tests','🧪️cases'):
            continue
        tree=ast.parse(source.read_text(encoding='utf8'),filename=str(source))
        if not any(isinstance(n,ast.ClassDef) and any(isinstance(s,ast.Assign) and any(getattr(t,'id','')=='NARRATION' for t in s.targets) for s in n.body) for n in tree.body):
            continue
        compile(tree,str(source),'exec')
        importlib.import_module('.'.join((*clip.relative_to(root).parts,folder.name,'🐍️')))
        print('[DEBUG] Compiled and imported '+str(source),flush=True)
