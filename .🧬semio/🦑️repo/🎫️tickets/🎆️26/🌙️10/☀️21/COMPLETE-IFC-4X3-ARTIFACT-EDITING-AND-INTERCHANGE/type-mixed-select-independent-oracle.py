import pathlib,json
p=pathlib.Path(__file__).resolve().parent
interface=p/'type-mixed-select-oracle.py'
scope={'__file__':str(interface),'__name__':'independent_mixed_interface'}
exec(compile(interface.read_text(encoding='utf-8-sig').split('\nlaw=')[0],str(interface),'exec'),scope)
law=json.loads((p.parents[6]/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/type-mixed-select-input.json').read_text())
backend=scope['IfcOpenShellMixed'](law);rows=[backend.observe(row)for row in law['cases']]
assert all(row['exact']and row['admitted']==row['expectedAdmitted']for row in rows if row['physical'])
(p/'🗑️generated/type-mixed-select-independent-oracle.json').write_text(json.dumps({'version':backend.version,'rows':rows},indent=2),encoding='utf8')
print(json.dumps({'version':backend.version,'physical':sum(row['physical']for row in rows),'drafts':sum(not row['physical']for row in rows),'allPhysicalMatch':True}))
