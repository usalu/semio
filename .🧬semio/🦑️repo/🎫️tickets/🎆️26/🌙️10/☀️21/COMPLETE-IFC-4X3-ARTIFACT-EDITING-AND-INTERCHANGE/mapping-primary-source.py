"""🗺️ Pin released mapping definitions from primary immutable buildingSMART bytes."""
import urllib.request,hashlib,json
from pathlib import Path
p=Path(__file__).parent;rows=[]
for name in ['IfcRepresentationMap','IfcMappedItem','IfcCartesianTransformationOperator3D','IfcCartesianTransformationOperator3DnonUniform','IfcBaseAxis']:
 url='https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/lexical/'+name+'.htm'
 raw=urllib.request.urlopen(url,timeout=30).read();(p/'🗑️generated'/('mapping-'+name+'.html')).write_bytes(raw);rows.append(dict(name=name,url=url,sha256=hashlib.sha256(raw).hexdigest()))
(p/'🗑️generated/mapping-primary-source.json').write_text(json.dumps(rows,indent=2)+'\n');print('[DEBUG] pinned5 primary mapping pages')
