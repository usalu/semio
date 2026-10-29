"""🎨 Record fresh actual material callbacks and point-container boundaries after final repairs."""
import hashlib,importlib,json,runpy,time,traceback
from pathlib import Path
CLIP=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip')
TICKET=Path(__file__).parent
source=CLIP/'🧱️model/🔣️.json'
OUTPUT=TICKET/'🗑️generated/clips-final-callback-r68.json'
def hashes():
    """🧮 Capture current production identities without saving their source contents."""
    return {str(p.relative_to(CLIP)):hashlib.sha256(p.read_bytes()).hexdigest() for p in CLIP.rglob('*.py') if not any('🧪' in part for part in p.relative_to(CLIP).parts)}
started=time.monotonic()
raw_before=hashlib.sha256(source.read_bytes()).hexdigest()
assert raw_before==EXPECTED_SOURCE
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
point=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.📏️geometry').P
assert type(point([1.,2.])) is tuple
assert all(type(value) is float for value in point([1.,2.]))
provider_before=hashlib.sha256(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode()).hexdigest()
code_before=hashes()
callback_input=TICKET/'chapter-native-drawings-r24.py'
input_before=hashlib.sha256(callback_input.read_bytes()).hexdigest()
OUTPUT.write_text(json.dumps({'debug':'[DEBUG] final actual callbacks','phase':'actual tuple-P and native material callback script','source':raw_before,'provider':provider_before}))
error=None
try:
    callbacks=runpy.run_path(str(callback_input))
except Exception:
    error=traceback.format_exc()
raw_after=hashlib.sha256(source.read_bytes()).hexdigest()
provider_after=hashlib.sha256(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode()).hexdigest()
code_after=hashes()
summary={'source_before':raw_before,'source_after':raw_after,'provider_before':provider_before,'provider_after':provider_after,'code_stable':code_before==code_after,'changed_code':[p for p in sorted(set(code_before)|set(code_after)) if code_before.get(p)!=code_after.get(p)],'callback_input_stable':input_before==hashlib.sha256(callback_input.read_bytes()).hexdigest(),'point_container':'builtin float tuple','seconds':time.monotonic()-started,'error':error,'native_frames':'unverified'}
OUTPUT.write_text(json.dumps(summary))
print('FINAL_ACTUAL_CALLBACKS',json.dumps(summary))
