import pathlib,json,hashlib,collections,math
import ifcopenshell
p=pathlib.Path(__file__).resolve().parent;root=p.parents[6]
r=root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official'
result={'version':ifcopenshell.version,'policy':'Exact finite binary64 coordinate tuples; signed zero equivalent; no tolerance welding','files':[]}
for name in ['triangulated-item.ifc','slab-tessellated-unique-vertices.ifc','beam-straight-i-shape-tessellated.ifc','beam-curved-i-shape-tessellated.ifc']:
 path=r/name;raw=path.read_bytes();f=ifcopenshell.open(str(path));items=[]
 for entity in f.by_type('IfcTriangulatedFaceSet'):
  coords=list(entity.Coordinates.CoordList);pn=entity.PnIndex;normals=entity.Normals;keys={};ids=[];groups=collections.defaultdict(list)
  for index,point in enumerate(coords,1):
   if len(point)!=3 or not all(math.isfinite(x) for x in point):raise ValueError('nonfinite coordinate')
   key=tuple(0. if x==0 else x for x in point)
   groups[key].append(index)
   if key not in keys:keys[key]=len(keys)
   ids.append(keys[key])
  edges=collections.defaultdict(list);triangles=collections.defaultdict(list);usedraw=set();usedcoord=set();degen=[];noncollinear=[]
  for ordinal,triangle in enumerate(entity.CoordIndex,1):
   if len(triangle)!=3:raise ValueError('triangle shape')
   geom=[]
   for index in triangle:
    if not isinstance(index,int) or index<1:raise ValueError('index kind/range')
    usedraw.add(index)
    if pn is not None:
     if index>len(pn):raise ValueError('Pn lookup')
     actual=pn[index-1]
    else:actual=index
    if not isinstance(actual,int) or actual<1 or actual>len(coords):raise ValueError('coordinate lookup')
    usedcoord.add(actual);geom.append(ids[actual-1])
   if len(set(geom))<3:degen.append(ordinal)
   triangles[tuple(sorted(geom))].append(ordinal)
   a,b,c=[coords[(pn[x-1] if pn is not None else x)-1] for x in triangle]
   u=[b[i]-a[i] for i in range(3)];v=[c[i]-a[i] for i in range(3)];cross=[u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]]
   if all(x==0 for x in cross):noncollinear.append(ordinal)
   for i in range(3):
    x,y=geom[i],geom[(i+1)%3];edges[tuple(sorted((x,y)))].append({'triangle':ordinal,'direction':1 if x<y else -1 if x>y else 0})
  bad=[{'vertices':list(key),'owners':owners} for key,owners in edges.items() if len(owners)!=2 or sum(x['direction'] for x in owners)!=0]
  items.append({'id':entity.id(),'closed':entity.Closed,'sourceCoordinates':len(coords),'exactCoordinates':len(keys),'aliasGroups':[{'coordinates':list(k),'sourceIndices':v} for k,v in groups.items() if len(v)>1],'triangles':len(entity.CoordIndex),'geometricEdges':len(edges),'edgeOwnerCountHistogram':dict(collections.Counter(len(v) for v in edges.values())),'badGeometricEdges':bad,'degenerateIdentityTriangles':degen,'zeroCrossProductTriangles':noncollinear,'duplicateGeometricTriangles':[v for v in triangles.values() if len(v)>1],'unusedCoordinates':[i for i in range(1,len(coords)+1) if i not in usedcoord],'pnIndex':None if pn is None else list(pn),'unusedPnIndexSlots':None if pn is None else [i for i in range(1,len(pn)+1) if i not in usedraw],'normals':None if normals is None else len(normals),'unusedNormalSlots':None if normals is None else [i for i in range(1,len(normals)+1) if i not in usedraw],'missingReferencedNormals':[] if normals is None else sorted(i for i in usedraw if i>len(normals))})
 result['files'].append({'name':name,'sha256':hashlib.sha256(raw).hexdigest(),'schema':list(f.header.file_schema.schema_identifiers),'items':items})
out=p/'🗑️generated/tfs-admission/exact-closure.json';out.write_text(json.dumps(result,indent=2),encoding='utf8')
print(json.dumps([{'name':x['name'],'items':[{'id':a['id'],'source':a['sourceCoordinates'],'exact':a['exactCoordinates'],'triangles':a['triangles'],'edges':a['geometricEdges'],'badEdges':len(a['badGeometricEdges']),'degenerate':len(a['zeroCrossProductTriangles']),'duplicate':len(a['duplicateGeometricTriangles']),'unusedCoordinates':len(a['unusedCoordinates'])} for a in x['items']]} for x in result['files']]))
