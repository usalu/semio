from pathlib import Path
import urllib.request, hashlib, json
root=Path(__file__).parent/'🗑️generated'/'alignment-geometry';root.mkdir(parents=True,exist_ok=True)
base='https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/'
paths={'concept.html':'concepts/Product_Shape/Product_Geometric_Representation/Alignment_Geometry/Alignment_Geometry_-_Horizontal,_Vertical_and_Cant/content.html','segment.html':'lexical/IfcAlignmentSegment.htm','measure.html':'lexical/IfcCurveMeasureSelect.htm','horizontal.html':'lexical/IfcAlignmentHorizontalSegment.htm'}
rows=[]
for name,path in paths.items():
 url=base+path
 data=urllib.request.urlopen(url).read();(root/name).write_bytes(data);rows.append({'file':name,'url':url,'sha256':hashlib.sha256(data).hexdigest()})
api='https://api.github.com/repos/buildingSMART/IFC4.x-specification-models/commits/main'
sha=json.load(urllib.request.urlopen(api))['sha']
url=f'https://raw.githubusercontent.com/buildingSMART/IFC4.x-specification-models/{sha}/models/alignment-geometries-and-linear-positioning/sectioned-solid-horizontal/sectioned-solid-horizontal.ifc'
data=urllib.request.urlopen(url).read();(root/'official-example.ifc').write_bytes(data);rows.append({'file':'official-example.ifc','url':url,'commit':sha,'sha256':hashlib.sha256(data).hexdigest()})
(root/'sources.json').write_text(json.dumps(rows,indent=2),encoding='utf-8')
print(json.dumps(rows))
