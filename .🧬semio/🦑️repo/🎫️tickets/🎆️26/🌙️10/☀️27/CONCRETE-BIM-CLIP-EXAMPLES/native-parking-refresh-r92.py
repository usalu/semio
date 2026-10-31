"""🚗 Stage an independently derived current-source parking delta; apply only against explicit frozen hashes."""
import argparse,hashlib,importlib,json,math,os,sys,time
from collections import Counter
from copy import deepcopy
from pathlib import Path
import numpy as np
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'package.json').is_file())
sys.path.insert(0,str(ROOT))
sys.dont_write_bytecode=True
TICKET=Path(__file__).parent
OUT=TICKET/'🗑️generated/native-r92/parking'
OUT.mkdir(parents=True,exist_ok=True)
os.environ.update(SEMIO_TEACHING_LANGUAGE='en',SEMIO_TEACHING_AUDIENCE='speaker',SEMIO_TEACHING_OUTPUT_ROOT=str(OUT))
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--apply',action='store_true')
parser.add_argument('--expect-source-sha')
parser.add_argument('--expect-api-sha')
args=parser.parse_args()
parking=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚗️parking')
api=parking.api
source_path=parking.PATH.parent.parent/'🔣️.json'
paths=[source_path,Path(parking.__file__),parking.PATH,Path(__file__),Path(api.__file__).parent/'🧬️schema/🔣️.schema.json']+[p for p in Path(api.__file__).parent.rglob('*.py')if '🧪️tests'not in str(p)]
def hashes():
    """🔐 Bind derived physical expectations to the exact source, producers and prior fixture."""
    return {str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
BEFORE=hashes()
if args.apply:
    assert args.expect_source_sha and args.expect_api_sha,'Applying requires explicit frozen source/API hashes'
    assert BEFORE[str(source_path.relative_to(ROOT))]==args.expect_source_sha,'Source differs from frozen contract'
    assert BEFORE[str(Path(api.__file__).relative_to(ROOT))]==args.expect_api_sha,'API differs from frozen contract'
START=time.perf_counter()
REPORT={'inputs_before':BEFORE,'stages':[],'applied':False,'success':False}
def progress(stage):
    """🛑 Report expensive native stages and allow cooperative or keyboard cancellation."""
    if (TICKET/'native-r92-cancel').exists():
        raise KeyboardInterrupt('Parking oracle regeneration cancelled')
    REPORT['stages'].append({'stage':stage,'seconds':time.perf_counter()-START})
    (OUT/'progress.json').write_text(json.dumps(REPORT,indent=2),encoding='utf-8')
    print('[DEBUG] Independent parking '+stage,flush=True)
def reserves(source,fixture):
    """🚪 Reconstruct source clear portals while preserving declared pedestrian reserve depth and side."""
    registry={e['tag']:e for e in source['elements']+source['types']}
    result=deepcopy(fixture['door_access_reserves'])
    for reserve in result:
        opening=registry[reserve['opening']]
        wall=registry[opening['geometry']['wall']]
        assert len(wall['geometry']['path'])==2,'Access reserve needs independently registered straight host'
        a,b=np.asarray(wall['geometry']['path'],dtype=float)
        direction=(b-a)/np.linalg.norm(b-a)
        filling=next(e for e in source['elements'] if e['geometry'].get('opening')==opening['tag'])
        width=registry[filling['type']]['filling']['frame_width']['value']
        portal=[a+direction*(opening['attributes']['start']['value']+width),a+direction*(opening['attributes']['end']['value']-width)]
        indices=reserve['portal_indices']
        old=np.asarray(reserve['polygon'],dtype=float)
        old_portal=[old[index] for index in indices]
        normal=np.array([-direction[1],direction[0]])
        sign=1 if np.dot(old.mean(axis=0)-np.mean(old_portal,axis=0),normal)>0 else -1
        polygon=[]
        for index,point in enumerate(old):
            if index in indices:
                polygon.append(portal[indices.index(index)].tolist())
            else:
                nearest=int(np.argmin([np.linalg.norm(point-p) for p in old_portal]))
                polygon.append((portal[nearest]+sign*reserve['depth']*normal).tolist())
        reserve['polygon']=polygon
    return result
try:
    source=json.loads(source_path.read_text(encoding='utf-8'))
    old=json.loads(parking.PATH.read_text(encoding='utf-8'))
    candidate=deepcopy(old)
    low,high=old['car']['z0'],old['car']['z0']+old['car']['height']
    progress('native occupied-part catalogue')
    records=api._vehicle_parts(source)
    progress('independent SciPy/GEOS band projections')
    obstacles=parking._independent_band_records(records,low,high)
    candidate.update(physical_obstacles=obstacles,source_signature=parking._signature(source,old['car']),active_column_tags=sorted({p['tag'] for p in parking._columns(source,old['car'])}),outline=deepcopy(next(s['outline'] for s in source['spaces'] if s['tag']==old['garage'])),door_access_reserves=reserves(source,old))
    candidate['source_vehicle_observed']={key:source['car'][key]['value'] for key in ('length','width','height')}
    progress('live native vs independent vs candidate height/overlay contract')
    subject=parking.ParkingContract('test_physical_oracle_matches_native_height_intersections')
    subject.fixture,subject.source=candidate,source
    subject.test_physical_oracle_matches_native_height_intersections()
    progress('independent GEOS distances/intersections and retained control poses')
    results={case['id']:parking.evaluate(candidate,source,case['poses']) for case in [candidate['verified_control']]+candidate['rejected_demonstrations']}
    subject.results=results
    candidate['expected_results']=results
    subject.test_fixture_is_bound_to_current_physical_source()
    subject.test_two_car_control_is_physically_verified()
    subject.test_three_car_demonstrations_are_rejected_without_infeasibility_claim()
    subject.test_numpy_corners_and_javascript_metrics_match_geos()
    subject.test_exact_physical_overlay_matches_portable_and_source_cases()
    assert candidate['target']['status']=='unverified-current-source' and candidate['target']['global_infeasibility_proven'] is False
    after=hashes()
    stable=BEFORE==after
    before_counts=Counter((p['tag'],p['kind'],p['role']) for p in old['physical_obstacles'])
    after_counts=Counter((p['tag'],p['kind'],p['role']) for p in obstacles)
    fields=('physical_obstacles','source_signature','active_column_tags','outline','door_access_reserves','source_vehicle_observed','expected_results')
    delta={key:candidate[key] for key in fields}
    REPORT.update(inputs_after=after,stable=stable,success=stable,independent_records=len(obstacles),source_part_count=len(records),delta_counts=[{'tag':key[0],'kind':key[1],'role':key[2],'before':before_counts[key],'after':after_counts[key]} for key in sorted(set(before_counts)|set(after_counts)) if before_counts[key]!=after_counts[key]],controls=results,derived_delta=delta)
    (OUT/'candidate-delta.json').write_text(json.dumps(REPORT,indent=2,ensure_ascii=False),encoding='utf-8')
    if args.apply:
        assert stable,'Inputs changed while deriving parking fixture'
        assert hashes()==BEFORE,'Inputs changed before fixture publication'
        original=parking.PATH.stat().st_size
        payload=(json.dumps(candidate,indent=2,ensure_ascii=False)+'\n').encode('utf-8')
        if len(payload)<original:
            payload+=b' '*(original-len(payload))
        with parking.PATH.open('r+b') as stream:
            stream.write(payload)
            stream.flush()
            os.fsync(stream.fileno())
        assert json.loads(parking.PATH.read_text(encoding='utf-8'))==candidate
        REPORT['applied']=True
        REPORT['published_fixture_sha256']=hashlib.sha256(parking.PATH.read_bytes()).hexdigest()
        REPORT['inputs_after_publication']=hashes()
        assert all(REPORT['inputs_after_publication'][key]==value for key,value in BEFORE.items() if key!=str(parking.PATH.relative_to(ROOT))),'Producer changed during publication'
    progress('complete')
    (OUT/'result.json').write_text(json.dumps(REPORT,indent=2,ensure_ascii=False),encoding='utf-8')
except BaseException as error:
    REPORT.update(inputs_after=hashes(),error=repr(error),seconds=time.perf_counter()-START)
    (OUT/'result.json').write_text(json.dumps(REPORT,indent=2,ensure_ascii=False),encoding='utf-8')
    raise
