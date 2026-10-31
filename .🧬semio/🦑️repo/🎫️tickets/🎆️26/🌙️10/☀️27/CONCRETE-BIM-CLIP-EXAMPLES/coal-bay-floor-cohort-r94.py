import importlib,importlib.util,json,sys,pathlib,unittest
from unittest.mock import patch
B=pathlib.Path(__file__).parent;sys.path.insert(0,str(pathlib.Path.cwd()))
spec=importlib.util.spec_from_file_location('coal_author',B/'coal-bay-floor-author-r94.py');H=importlib.util.module_from_spec(spec);spec.loader.exec_module(H)
m=H.A.load_model();H.apply(m,json.loads((B/'🗑️generated/native-r92/coal-bay-floor-candidate-r94.json').read_text()))
f=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪵️floors')
with patch.object(H.A,'load_model',lambda:m):
 f.test_original_ground_wearing_materials_support_layers_and_exposed_cement()
print('[DEBUG] full ground floor grammar/quantity/coverage PASS',flush=True)
