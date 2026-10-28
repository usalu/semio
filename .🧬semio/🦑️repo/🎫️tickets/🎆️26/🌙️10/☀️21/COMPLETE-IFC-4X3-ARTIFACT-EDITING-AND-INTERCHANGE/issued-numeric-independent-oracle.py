import pathlib,importlib.util,json,collections
p=pathlib.Path(__file__).resolve().parent;spec=importlib.util.spec_from_file_location('numeric_owned',p/'issued-express-numeric-oracle.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
rows=[m.GeneratedHelperOracle().observe(c)for c in m.data['functions']];sql=[m.SqliteArithmeticOracle().observe(c)for c in m.data['functions']if c['owner']in('IFCCONSTRAINTSPARAMBSPLINE','IFCCORRECTDIMENSIONS','IFCDERIVEDIMENSIONALEXPONENTS')]
result={'ifcopenshell':m.ifcopenshell.version,'sqlite':m.sqlite3.sqlite_version,'functions':rows,'arithmetic':sql}
(p/'🗑️generated/issued-numeric-independent-oracle.json').write_text(json.dumps(result,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({'helpers':dict(collections.Counter(x['status']for x in rows)),'sql':dict(collections.Counter(x['status']for x in sql))}))
