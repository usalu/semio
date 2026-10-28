"""⚖️ Observe current physical optional nodal slots with the independent cached backend."""
import json,sys,ctypes,hashlib,importlib.util
from pathlib import Path
p=Path(__file__).parent;dep=str(p/'🗑️generated/oracle')
if sys.platform=='win32':
 b=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,b,len(b));dep=b.value or dep
sys.path.insert(0,dep)
spec=importlib.util.spec_from_file_location('nodal_oracle',p/'fem-nodal-physical-oracle.py');module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module);oracle=module.IfcOpenShellOracle();folder=p/'🗑️generated/fem-nodal-physical';inputs=json.loads((folder/'input.json').read_text(encoding='utf-8'));rows=[]
for expected in inputs:
 path=folder/(expected['id']+'.ifc');row=dict(sha256=hashlib.sha256(path.read_bytes()).hexdigest(),**oracle.inspect(path,expected));assert row['schema']=='IFC4X3_ADD2'and not row['diagnostics'];rows.append(row);print('[DEBUG] '+json.dumps(dict(id=row['id'],version=row['version'],selectedBits=row['selectedBits'],diagnostics=len(row['diagnostics']))),flush=True)
(p/'🗑️generated/fem-nodal-independent-physical.json').write_text(json.dumps(dict(python=sys.executable,rows=rows),indent=2,default=str)+'\n',encoding='utf-8')
