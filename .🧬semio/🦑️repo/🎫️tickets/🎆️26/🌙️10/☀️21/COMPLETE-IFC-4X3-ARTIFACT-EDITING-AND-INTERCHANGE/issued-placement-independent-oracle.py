"""📐 Independently invoke unchanged placement oracle interfaces into isolated evidence."""
from pathlib import Path
import importlib.util,json
p=Path(__file__).parent;s=importlib.util.spec_from_file_location('placement_interface',p/'issued-express-placement-oracle.py');m=importlib.util.module_from_spec(s);s.loader.exec_module(m);o=m.GeneratedPlacementOracle();sql=m.SqliteEpsilonOracle(o.model);f=[o.observe(c) for c in m.data['functions']];a=[sql.observe(c) for c in m.data['functions']];(p/'🗑️generated/issued-placement-independent-oracle.json').write_text(json.dumps({'version':m.ifcopenshell.version,'functions':f,'arithmetic':a},indent=2,default=str),encoding='utf8');print('[DEBUG]',sum(r['status']=='agree' for r in f),sum(r['status']=='agree' for r in a))
