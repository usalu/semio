"""🧪 Capture and prepare the current six chapters with independent physical callback proofs."""
import hashlib
import importlib
import json
import runpy
import time
import traceback
from pathlib import Path

CLIP=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip')
TICKET=Path(__file__).parent
OUTPUT=TICKET/'🗑️generated'/'clips-cohort-r33.json'

def code_hashes():
    """📦 Record production code identities without copying live source files."""
    return {str(p.relative_to(CLIP)):hashlib.sha256(p.read_bytes()).hexdigest() for p in CLIP.rglob('*.py') if not any('🧪' in part for part in p.relative_to(CLIP).parts)}

def mark(stage):
    """⏳ Publish bounded preparation progress and current error evidence."""
    OUTPUT.write_text(json.dumps({'debug':'[DEBUG] six chapters','stage':stage,'raw_source':raw_before,'provider':provider_before,'errors':errors,'seconds':time.monotonic()-started},ensure_ascii=False))

started=time.monotonic()
raw_before=hashlib.sha256((CLIP/'🧱️model'/'🔣️.json').read_bytes()).hexdigest()
code_before=code_hashes()
API=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CAPTURED=API.load_model()
provider_before=hashlib.sha256(json.dumps(CAPTURED,sort_keys=True,separators=(',',':')).encode()).hexdigest()
errors={}
results={}
for stage,script in (('six preparation methods','clips-preparation-r11.py'),('native material callbacks','chapter-native-drawings-r24.py')):
    mark(stage)
    try:
        results[stage]=runpy.run_path(str(TICKET/script))
    except Exception:
        errors[stage]=traceback.format_exc()
    mark(stage+' complete')
raw_after=hashlib.sha256((CLIP/'🧱️model'/'🔣️.json').read_bytes()).hexdigest()
provider_after=hashlib.sha256(json.dumps(API.load_model(),sort_keys=True,separators=(',',':')).encode()).hexdigest()
code_after=code_hashes()
summary={'products':len(CAPTURED['elements']),'raw_before':raw_before,'raw_after':raw_after,'provider_before':provider_before,'provider_after':provider_after,'source_stable':raw_before==raw_after,'provider_stable':provider_before==provider_after,'code_stable':code_before==code_after,'changed_code':[p for p in sorted(set(code_before)|set(code_after)) if code_before.get(p)!=code_after.get(p)],'errors':errors,'seconds':time.monotonic()-started,'native_frames':'unverified'}
OUTPUT.write_text(json.dumps(summary,ensure_ascii=False))
print('SIX_CHAPTERS',json.dumps(summary,ensure_ascii=False))
