import pathlib,re,json,hashlib
p=pathlib.Path(__file__).resolve().parent;root=p.parents[6];fixtures=root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures';rows=[]
for folder in ['official','official-cert']:
 for path in sorted((fixtures/folder).glob('*.ifc')):
  data=path.read_bytes();text=data.decode('utf8',errors='replace');counts={name:len(re.findall(r'#\d+\s*=\s*'+name+r'\s*\(',text,re.I))for name in ['IFCRIGHTCIRCULARCYLINDER','IFCRIGHTCIRCULARCONE','IFCCSGSOLID','IFCBLOCK','IFCSPHERE','IFCRECTANGULARPYRAMID']};rows.append({'path':str(path.relative_to(root)),'bytes':len(data),'sha256':hashlib.sha256(data).hexdigest(),'counts':counts,'schema':re.findall(r'FILE_SCHEMA\s*\((.*?)\)\s*;',text,re.S)})
result={'scan':'original bytes declaration lexical census, not parser/reachability or MVD admission','files':len(rows),'rows':rows,'totals':{name:sum(r['counts'][name]for r in rows)for name in rows[0]['counts']}}
(p/'🗑️generated/csg-round-independent/official-census.json').write_text(json.dumps(result,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({'files':len(rows),'totals':result['totals']}))
