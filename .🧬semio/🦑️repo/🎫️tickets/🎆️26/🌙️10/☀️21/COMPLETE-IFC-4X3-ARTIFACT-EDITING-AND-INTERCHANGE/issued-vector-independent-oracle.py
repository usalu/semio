"""🧭 Independently invoke the owned external interfaces without overwriting worker evidence."""
from pathlib import Path
import importlib.util, json
p=Path(__file__).parent
spec=importlib.util.spec_from_file_location('vector_interface',p/'issued-express-vector-oracle.py'); m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
o=m.GeneratedVectorOracle();s=m.SqliteVectorArithmeticOracle(o.model);rows=[o.observe(c) for c in m.data['functions']];sql=[s.observe(c) for c in m.data['functions']];result={'functions':rows,'arithmetic':sql,'version':m.ifcopenshell.version}
(p/'🗑️generated/issued-vector-independent-oracle.json').write_text(json.dumps(result,indent=2,default=str),encoding='utf8')
print('[DEBUG]',sum(r['status']=='agree' for r in rows),sum(r['status']=='agree' for r in sql))
