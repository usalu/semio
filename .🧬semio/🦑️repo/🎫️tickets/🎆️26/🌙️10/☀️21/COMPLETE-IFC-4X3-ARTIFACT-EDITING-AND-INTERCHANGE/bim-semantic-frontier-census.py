import pathlib,json,re,hashlib
p=pathlib.Path(__file__).resolve().parent;r=p.parents[6];folder=r/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official';names=['IFCRELDEFINESBYPROPERTIES','IFCPROPERTYSINGLEVALUE','IFCRELDEFINESBYTYPE','IFCRELASSOCIATESMATERIAL','IFCRELASSOCIATESCLASSIFICATION','IFCRELASSIGNSTOGROUP','IFCRELVOIDSELEMENT'];rows=[]
for file in sorted(folder.glob('*.ifc')):
 b=file.read_bytes();s=b.decode('utf8',errors='replace');counts={n:len(re.findall(r'#\d+\s*=\s*'+n+r'\s*\(',s,re.I))for n in names};rows.append({'name':file.name,'sha256':hashlib.sha256(b).hexdigest(),'counts':counts})
result={'scope':'46-original lexical relation declarations; not complete semantic reachability','files':len(rows),'totals':{n:sum(x['counts'][n]for x in rows)for n in names},'rows':rows}
(p/'🗑️generated/bim-semantic-frontier-census.json').write_text(json.dumps(result,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps(result['totals']))
