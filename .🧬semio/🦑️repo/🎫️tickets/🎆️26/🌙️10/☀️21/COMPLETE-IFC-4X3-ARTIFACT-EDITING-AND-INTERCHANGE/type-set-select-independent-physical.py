import pathlib,json
p=pathlib.Path(__file__).resolve().parent;interface=p/'type-set-select-oracle.py';scope={'__file__':str(interface),'__name__':'independent_set_interface'}
exec(compile(interface.read_text(encoding='utf-8-sig').split('\nlaw=')[0],str(interface),'exec'),scope)
law=json.loads((p.parents[6]/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/type-set-select-input.json').read_text())
backend=scope['IfcOpenShellSet'](law);rows=[backend.observe(row)for row in law['cases']]
assert all(row['exact']for row in rows if row['physical'])
(p/'🗑️generated/type-set-select-independent-physical.json').write_text(json.dumps({'version':backend.version,'rows':rows},indent=2),encoding='utf8')
print(json.dumps({'version':backend.version,'physical':sum(r['physical']for r in rows),'drafts':sum(not r['physical']for r in rows),'rawMismatches':[r['id']for r in rows if r['physical']and r['admitted']!=r['expectedAdmitted']]}))
