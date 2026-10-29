import importlib,json,time,traceback,hashlib
from pathlib import Path
root=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip');ticket=Path("/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES");gen=ticket/'🗑️generated'
base='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.'
selected=[('🏗️foundations','test_foundation_materials_and_vertical_contacts_are_physical'),('🏛️remodel','test_salon_radiator_containment_and_retired_pipe_absence'),('🏛️remodel','test_vehicle_sweep_against_independent_geometry'),('📐️representations','test_sections_and_elevations_use_actual_xz_solids'),('📐️representations','test_stair_quantities_equal_occupied_solid_union_including_landings')]
record={'source_before':hashlib.sha256((root/'🧱️model/🔣️.json').read_bytes()).hexdigest(),'results':[]};started=time.monotonic()
for module,name in selected:
    (gen/'consumer-r78-progress.json').write_text(json.dumps({'current':base+module+'.'+name,'completed':len(record['results'])}))
    start=time.monotonic()
    try:getattr(importlib.import_module(base+module),name)();result={'id':base+module+'.'+name,'pass':True}
    except Exception:result={'id':base+module+'.'+name,'pass':False,'traceback':traceback.format_exc()}
    result['seconds']=time.monotonic()-start;record['results'].append(result)
    (gen/'consumer-r78-result.json').write_text(json.dumps(record))
record['seconds']=time.monotonic()-started;record['source_after']=hashlib.sha256((root/'🧱️model/🔣️.json').read_bytes()).hexdigest()
(gen/'consumer-r78-result.json').write_text(json.dumps(record));print(record)
