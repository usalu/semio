"""🌂 Build the market parasol from its four authored scales and sewn, area-preserving radial folds."""
from pathlib import Path
from runpy import run_path
import json, sys
T=Path(__file__).resolve().parent
model=run_path(str(T/'digital-canopy-pleats.py'))
catalog=model['CAT'];source=T/'volumes/objects/market-parasol.json'
def build():
    doc=json.loads(catalog.read_text());a=doc['asset']
    a['volumes']=json.loads(source.read_text())['volumes']
    model['author'](a)
    return doc
if __name__=='__main__':
    doc=build();out=T/'🗑️generated/r5-market-parasol/🔣️.json';out.parent.mkdir(parents=True,exist_ok=True)
    out.write_text(json.dumps(doc,ensure_ascii=False,indent=2)+'\n')
    if 'apply' in sys.argv:run_path(str(T/'digital-publish-asset.py'))['publish'](model['author'],catalog,out)
