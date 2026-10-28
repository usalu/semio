import pathlib,json,importlib.util,collections
p=pathlib.Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('owned',p/'issued-express-owner-oracle.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
d=p.parents[6]/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️coverage'
cases=json.loads((d/'cases.json').read_text(encoding='utf-8-sig'))
rows=[m.IfcOpenShellOracle().function(c)for c in cases['functions']];arrays=[m.SqliteArrayOracle().function(c)for c in cases['functions']if c['owner']in('IFCLISTTOARRAY','IFCMAKEARRAYOFARRAY')]
result={'ifcopenshell':m.ifcopenshell.version,'functions':rows,'arrays':arrays,'globalPhysicalReplay':'not repeated; original full validation raw observations reviewed separately'}
(p/'🗑️generated/issued-owner-independent-oracle.json').write_text(json.dumps(result,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({'functions':dict(collections.Counter(x['status']for x in rows)),'arrays':dict(collections.Counter(x['status']for x in arrays))}))
