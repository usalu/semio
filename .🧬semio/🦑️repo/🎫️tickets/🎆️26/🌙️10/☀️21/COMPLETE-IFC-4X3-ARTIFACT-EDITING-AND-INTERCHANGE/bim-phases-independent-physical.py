"""🗓️ Replay the owned physical phase interface with the independent cached validator."""
import json,sys,ctypes,hashlib,importlib.util
from pathlib import Path
p=Path(__file__).parent;dep=str(p/'🗑️generated/oracle')
if sys.platform=='win32':
 b=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,b,len(b));dep=b.value or dep
sys.path.insert(0,dep)
spec=importlib.util.spec_from_file_location('phase_oracle',p/'bim-phases-physical-oracle.py');module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module);oracle=module.IfcOpenShellOracle();folder=p/'🗑️generated/bim-phases-physical';inputs=json.loads((folder/'input.json').read_text(encoding='utf-8'));rows=[]
for expected in inputs:
 path=folder/(expected['id']+'.ifc');row=dict(id=expected['id'],sha256=hashlib.sha256(path.read_bytes()).hexdigest(),**oracle.inspect(path));assert row['schema']=='IFC4X3_ADD2'and row['phase']is None and row['countType']=='IfcInteger'and row['phases']==expected['phases']and row['count']==len(expected['phases'])and all(t=='IfcText'for t in row['itemTypes'])and not row['diagnostics'];rows.append(row);print('[DEBUG] '+json.dumps(dict(id=row['id'],version=row['version'],count=row['count'],diagnostics=len(row['diagnostics']))),flush=True)
(p/'🗑️generated/bim-phases-independent-physical.json').write_text(json.dumps(dict(python=sys.executable,rows=rows),indent=2,default=str)+'\n',encoding='utf-8')
