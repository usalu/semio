"""⏱️ Diagnose current full-model geometry without editing production or bypassing admission."""
import faulthandler,hashlib,importlib,json,sys,time
from pathlib import Path
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));out=Path(__file__).parent/'🗑️generated';log=(out/'geometry-clipping-profile-r92.log').open('w',encoding='utf8');faulthandler.dump_traceback_later(30,repeat=True,file=log)
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');source=api.load_model();raw=Path(api.__file__).with_name('🔣️.json');before=hashlib.sha256(raw.read_bytes()).hexdigest();api_before=hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest();original=api._prism_residuals;rows=[];start=time.monotonic()
def observed(outline,cutters,exact=False):
    """🔎 Record costly source-operand slices while preserving every original return value."""
    began=time.monotonic()
    if exact:log.write('[DEBUG] exact begin '+json.dumps({'vertices':len(outline),'cutters':len(cutters),'bounds':[min(p[k] for p in outline) for k in (0,1)]})+'\n');log.flush()
    try:return original(outline,cutters,exact=exact)
    finally:
        duration=time.monotonic()-began
        if exact or duration>1:
            row={'exact':exact,'seconds':duration,'vertices':len(outline),'cutters':len(cutters)};rows.append(row);log.write('[DEBUG] '+json.dumps(row)+'\n');log.flush()
api._prism_residuals=observed
try:
    g=api.building_geometry(source)
    result={'seconds':time.monotonic()-start,'source_before':before,'api_before':api_before,'api_after':hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest(),'source_after':hashlib.sha256(raw.read_bytes()).hexdigest(),'rows':rows,'categories':{k:len(v) for k,v in g.items() if isinstance(v,list)}}
    (out/'geometry-clipping-profile-r92.json').write_text(json.dumps(result,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in result.items() if k!='rows'}),flush=True)
finally:api._prism_residuals=original;faulthandler.cancel_dump_traceback_later();log.close()
