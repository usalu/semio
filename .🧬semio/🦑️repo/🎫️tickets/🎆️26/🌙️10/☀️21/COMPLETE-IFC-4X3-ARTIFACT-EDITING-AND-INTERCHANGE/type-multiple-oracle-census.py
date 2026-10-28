from pathlib import Path
p=Path(__file__).with_name('type-multiple-oracle.py');s=p.read_text(encoding='utf8');s=s.replace("rows=[]\n",'''maximum=0;names=[]
for declaration in schema.declarations():
 entity=declaration.as_entity()
 if entity is None: continue
 count=0
 for attribute in entity.attributes():
  named=attribute.type_of_attribute().as_named_type()
  if not attribute.optional() and named is not None and named.declared_type().as_entity() is not None: count+=1
 if count>maximum: maximum=count;names=[entity.name().upper()]
 elif count==maximum: names.append(entity.name().upper())
assert (maximum,names)==(4,['IFCSURFACESTYLELIGHTING']),(maximum,names)
rows=[]
''');s=s.replace("'declarations':rows", "'releasedMaximum':maximum,'releasedMaximumEntities':names,'declarations':rows");p.write_text(s,encoding='utf8')
