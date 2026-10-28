"""🔮 Independent EXPRESS semantic oracle, test dependency only."""
import json,pathlib,sys
sys.path.insert(0,sys.argv[1])
import ifcopenshell,ifcopenshell.validate
for name in sys.argv[4:] or ['triangulated','alignment','structural']:
 source=pathlib.Path(sys.argv[2])/f'{name}.ifc'
 model=ifcopenshell.file.from_string(source.read_text(encoding='utf-8'))
 logger=ifcopenshell.validate.json_logger()
 ifcopenshell.validate.validate(model,logger,express_rules=True)
 rows=[]
 for item in logger.statements:
  row={key:(value.id() if isinstance(value,ifcopenshell.entity_instance) else str(value)) for key,value in item.items()}
  rows.append(row)
 pathlib.Path(sys.argv[3],f'express-oracle-{name}.json').write_text(json.dumps(rows,indent=2),encoding='utf-8')
 print(name,len(rows),json.dumps(rows[:12]))

