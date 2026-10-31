import importlib.util,json,sys
from pathlib import Path
B=Path(__file__).parent;sys.path.insert(0,str(Path.cwd()))
spec=importlib.util.spec_from_file_location('grade_author',B/'garage-approach-authoring-r94.py');G=importlib.util.module_from_spec(spec);spec.loader.exec_module(G)
m=G.API.load_model();obj=G.apply_candidate(m,json.loads((B/'🗑️generated/native-r92/garage-approach-candidate-r94.json').read_text()))
keys=['UniformExtension','GarageApproachInferred','GarageApproachRun','GarageApproachRise','GarageApproachFanSegments','GarageApproachMaximumHeightError']
result={'terrain':obj['psets']['Pset_TerrainReconstruction'],'nominal_depth_source':obj['attributes']['nominal_depth']['source'],'uncertainty':obj['historical_evidence']['uncertainty'],'property_names':{k:m['property_names'][k] for k in keys}}
(B/'🗑️generated/native-r92/floor-grade-metadata-r94.json').write_text(json.dumps(result,ensure_ascii=True,indent=2),encoding='utf-8')
print('[DEBUG] qualified metadata payload ready',flush=True)
