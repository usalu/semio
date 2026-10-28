"""🔎️ Independently replay owned released metadata/SQL query observations."""
import json,sys,ctypes,importlib.util
from pathlib import Path
p=Path(__file__).parent;dep=str(p/'🗑️generated/oracle')
if sys.platform=='win32':
 b=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,b,len(b));dep=b.value or dep
sys.path.insert(0,dep);spec=importlib.util.spec_from_file_location('query_oracle',p/'native-query-owner-oracle.py');module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module);oracle=module.IfcOpenShellSqliteOracle();cases=json.loads(Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📚️catalog/🔎️query/🧪️tests/cases.json').read_text(encoding='utf-8'))['cases'];rows=[]
for case in cases:
 actual=oracle.query(case);assert actual=={k:case[k]for k in ['status','values','pointers']};rows.append(dict(id=case['id'],**actual))
force=oracle.fields('IfcStructuralLoadSingleForce',True);assert all(r['optional']and r['owner']=='IFCSTRUCTURALLOADSINGLEFORCE'for r in force)and len(force)==6;(p/'🗑️generated/native-query-owner-independent-oracle.json').write_text(json.dumps(dict(version=oracle.version,sqlite=module.sqlite3.sqlite_version,rows=rows,force=force,nativeExecuted=False),indent=2)+'\n',encoding='utf-8');print('[DEBUG]10queryoutputs+6OPTIONALforceowners match '+oracle.version)
