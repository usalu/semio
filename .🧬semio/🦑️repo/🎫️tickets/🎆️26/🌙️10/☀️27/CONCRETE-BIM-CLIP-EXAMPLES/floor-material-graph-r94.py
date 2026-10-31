import importlib,sys,pathlib
sys.path.insert(0,str(pathlib.Path.cwd()))
m=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🔗️materials')
m.test_all_source_material_relations_refer_to_authored_assemblies_or_constituents()
print('[DEBUG] published global source material graph PASS',flush=True)
