"""⚡ Profile complete native geometry without changing a single physical value."""
from copy import deepcopy
import hashlib
import importlib
import json
from pathlib import Path
import sys
import time

ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
sys.dont_write_bytecode=True
TICKET=Path(__file__).parent
OUT=TICKET/'🗑️generated/native-r92/geometry-memo'
OUT.mkdir(parents=True,exist_ok=True)
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
from numpy.testing import assert_equal


def inputs():
    """🔐 Freeze the actual geometry producer, source, analysis and owned profile input."""
    paths=[Path(api.__file__),Path(api.__file__).with_name('🔣️.json'),
        Path(api.__file__).parent/'🧮️analysis/🐍️.py',Path(__file__)]
    return {str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}


def canonical(value):
    """🧾 Preserve exact values and list order in independent serialized comparisons."""
    return json.dumps(value,sort_keys=True,separators=(',',':'),ensure_ascii=False,allow_nan=False)


def size(value, seen=None):
    """📏 Count owned native tree bytes once per shared object identity."""
    seen=set() if seen is None else seen
    if id(value) in seen:
        return 0
    seen.add(id(value))
    total=sys.getsizeof(value)
    if isinstance(value,dict):
        total+=sum(size(k,seen)+size(v,seen) for k,v in value.items())
    elif isinstance(value,(tuple,list)):
        total+=sum(size(v,seen) for v in value)
    return total


def stage(name, callback):
    """🚦 Publish finite stage progress and preserve external/cooperative cancellation."""
    if (TICKET/'native-geometry-profile-r92-cancel').exists():
        raise KeyboardInterrupt('Geometry profile cancelled')
    (OUT/'progress.json').write_text(json.dumps({'stage':name,'time':time.time()}),encoding='utf-8')
    print('[DEBUG] Geometry memo '+name,flush=True)
    started=time.perf_counter()
    value=callback()
    timings[name]=time.perf_counter()-started
    print('[DEBUG] '+name+' '+str(timings[name])+' seconds',flush=True)
    return value

before=inputs()
model=api.load_model()
timings={}
reference=stage('uncached-whole-model',lambda:api._building_geometry_uncached(model))
expected=canonical(reference)
api._geometry_models_cached.cache_clear()
first=stage('memo-first-after-subcache-warmup',lambda:api.building_geometry(model))
assert_equal(reference,first)
assert canonical(first)==expected
warm=stage('memo-warm-equivalent-new-dict',lambda:api.building_geometry(deepcopy(model)))
assert_equal(reference,warm)
assert canonical(warm)==expected
first['grid']['x'][0]+=99
isolated=stage('memo-warm-after-output-mutation',lambda:api.building_geometry(model))
assert canonical(isolated)==expected
service=max(reference['services'],key=lambda obj:len(obj.get('parts',())))
actual_service=stage('seeded-service-query',lambda:api.service_geometry(service['tag'],model))
assert_equal(service,actual_service)
solids=[obj for group in ('details','furnishings','beams','foundations','railings') for obj in reference[group]]
solid=max(solids,key=lambda obj:len(obj.get('parts',())))
actual_solid=stage('seeded-solid-query',lambda:api.solid_geometry(solid['tag'],model))
assert_equal(solid,actual_solid)
after=inputs()
report={'success':before==after,'stable':before==after,'inputs_before':before,'inputs_after':after,
    'timings':timings,'canonical_sha256':hashlib.sha256(expected.encode()).hexdigest(),
    'canonical_bytes':len(expected.encode()),'native_tree_bytes':size(reference),
    'parts':sum(len(obj.get('parts',())) for group in reference.values() if isinstance(group,list) for obj in group if isinstance(obj,dict)),
    'service':service['tag'],'service_parts':len(service.get('parts',())),
    'solid':solid['tag'],'solid_parts':len(solid.get('parts',())),
    'cache_info':api._geometry_models_cached.cache_info()._asdict()}
(OUT/'result.json').write_text(json.dumps(report,indent=2),encoding='utf-8')
raise SystemExit(0 if report['success'] else 1)
