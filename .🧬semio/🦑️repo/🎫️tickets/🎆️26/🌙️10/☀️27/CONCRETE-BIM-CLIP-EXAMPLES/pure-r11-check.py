import importlib,inspect,json,traceback,unittest
from pathlib import Path
prefix='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.'
results=[]
for name in ['🧱️model.🧪️tests.📏️columns','🧱️model.🧪️tests.🚘️profiles']:
    try:
        mod=importlib.import_module(prefix+name);importlib.reload(mod)
        checked=mod.run(Path('/repo'));results.append({'module':name,'pass':True,'cases':checked})
    except Exception as e:results.append({'module':name,'pass':False,'error':traceback.format_exc()})
for name in ['🧱️model.🧪️tests.🪨️regions','🧱️model.🧪️tests.📐️representations']:
    mod=importlib.import_module(prefix+name);importlib.reload(mod)
    for case,fn in inspect.getmembers(mod,inspect.isfunction):
        if case.startswith('test_') and 'measured_archival' not in case:
            try:fn();results.append({'case':case,'pass':True})
            except Exception as e:results.append({'case':case,'pass':False,'error':traceback.format_exc()})
for name in ['🧱️model.🧪️tests.📍️placements','🎨️visuals.🏛️savoye.🧊️axon.📏️geometry','🎨️visuals.🧮️analysis.🧪️profiles.🐍️','🎨️visuals.🏛️savoye.🧱️core.🧪️surfaces']:
    mod=importlib.import_module(prefix+name);importlib.reload(mod)
    result=unittest.TextTestRunner().run(unittest.defaultTestLoader.loadTestsFromModule(mod))
    results.append({'module':name,'pass':result.wasSuccessful(),'count':result.testsRun})
Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES/🗑️generated/pure-r11-results.json').write_text(json.dumps(results,indent=2))
print(json.dumps({'passed_groups':sum(r['pass'] for r in results),'groups':len(results),'failed':[r for r in results if not r['pass']]}))
