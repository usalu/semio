"""⚖ Compare canonical named diagnostic identities; oracle implementation errors remain visible."""
import json,pathlib,re,sys
output=pathlib.Path(sys.argv[1]);catalog=json.loads(pathlib.Path(sys.argv[2]).read_text(encoding='utf-8-sig'))
def canonical(owner,attribute):
 owner=owner.upper();attribute=attribute.upper()
 while owner:
  declaration=catalog['entities'].get(owner)
  if not declaration:break
  if any(value.split(':',1)[0].strip().upper()==attribute for value in declaration['inverse']):return owner,attribute
  owner=declaration['supertype']
 return None
reports=[]
for name in ['triangulated','alignment','structural','adversarial']:
 own=json.loads((output/f'express-{name}.json').read_text(encoding='utf-8'));oracle=json.loads((output/f'express-oracle-{name}.json').read_text(encoding='utf-8'))
 actual={(str(row.get('entityId','None')),row['owner'],row['rule']) for row in own['diagnostics']}
 expected=set();implementation_errors=[]
 for row in oracle:
  attribute=row.get('attribute','');message=row['message'];identity=None
  if row['type']=='schema':
   match=re.search(r'Rule (\w+)\.(\w+):',message)
   if match:identity=(match[1].upper(),match[2].upper())
   elif '.' in attribute:identity=canonical(*attribute.split('.',1))
  elif row['type']=='global_rule':identity=(attribute.upper(),'WR1')
  elif '.' in attribute:identity=tuple(part.upper() for part in attribute.split('.',1))
  if not identity:raise AssertionError(f'Unclassified oracle diagnostic {row}')
  expected.add((str(row.get('instance','None')),*identity))
  if 'expecting value of type' in message or 'unsupported operand type' in message:implementation_errors.append(attribute)
 reports.append({'model':name,'ours':len(actual),'oracle':len(expected),'missing':sorted(expected-actual),'extra':sorted(actual-expected),'oracleImplementationErrors':implementation_errors,'complete':own['complete']})
 if actual!=expected:raise AssertionError(reports[-1])
print(json.dumps(reports,indent=2))
