"""🔺 Observe exact raw coordinate carriers separately from SDK-normalized aggregate storage."""
from pathlib import Path
import json,hashlib,re
import ifcopenshell,ifcopenshell.validate
p=Path(__file__).parent/'🗑️generated/tfs-integer-carrier-independent';results=[]
for row in json.loads((p/'emitted.json').read_text(encoding='utf8')):
    raw=Path(row['path']).read_bytes();source=ifcopenshell.open(row['path']);entity=source.by_id(int(row['coordinateId']));log=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(source,log,express_rules=True)
    results.append({'id':row['id'],'sha256':hashlib.sha256(raw).hexdigest(),'rawRecord':next(line for line in raw.decode().splitlines() if line.startswith('#'+row['coordinateId']+'=')),'sdkRecord':str(entity),'sdkCoordinates':entity.CoordList,'sdkKinds':[[type(v).__name__ for v in point] for point in entity.CoordList],'diagnostics':log.statements})
(p/'physical.json').write_text(json.dumps({'version':ifcopenshell.version,'rows':results},indent=2,default=str),encoding='utf8');print('[DEBUG]',[(r['id'],len(r['diagnostics']),r['sdkKinds']) for r in results])
