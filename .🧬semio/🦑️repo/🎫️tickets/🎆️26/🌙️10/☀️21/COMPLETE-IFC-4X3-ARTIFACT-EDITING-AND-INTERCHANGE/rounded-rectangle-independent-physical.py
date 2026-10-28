"""▢ Observe exact emitted rounded-profile source through installed SDK, never reconstruct."""
from pathlib import Path
import json,hashlib,math,re
import ifcopenshell,ifcopenshell.validate,ifcopenshell.geom
p=Path(__file__).parent;manifest=p/'🗑️generated/rounded-rectangle-actual/emitted.json';rows=json.loads(manifest.read_text(encoding='utf8'))['records'];out=[]
for row in rows:
 path=Path(row['path']);raw=path.read_bytes();r={'id':row['id'],'sha256':hashlib.sha256(raw).hexdigest(),'hashMatches':hashlib.sha256(raw).hexdigest()==row['sha256'],'declaredFormal':row['case']['formal'],'declaredGeometry':row['case']['geometry']}
 try:
  f=ifcopenshell.open(str(path));r['schema']=f.schema;r['entities']=len(list(f));profile=f.by_id(int(row['profile']));solid=f.by_id(int(row['solid']));r['profile']={'class':profile.is_a(),'rawSTEP':profile.to_string(),'sourceLine':next((s for s in raw.decode('utf8').splitlines() if s.startswith('#'+row['profile']+'=')),None),'XDim':profile.XDim,'YDim':profile.YDim,'RoundingRadius':profile.RoundingRadius,'radiusType':type(profile.RoundingRadius).__name__,'position':profile.Position.is_a() if profile.Position else None};r['solid']={'class':solid.is_a(),'depth':solid.Depth,'depthType':type(solid.Depth).__name__,'position':solid.Position.is_a() if solid.Position else None,'direction':list(solid.ExtrudedDirection.DirectionRatios)}
  log=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,log,express_rules=True);r['diagnostics']=log.statements
  if profile.RoundingRadius is not None and profile.RoundingRadius>0 and profile.RoundingRadius<=min(profile.XDim,profile.YDim)/2:
   c=row['case'];unit=.001 if c['unitPrefix']=='MILLI' else 1;factor=solid.Depth*abs(solid.ExtrudedDirection.DirectionRatios[2])/math.hypot(*solid.ExtrudedDirection.DirectionRatios)*abs(math.prod(c['mappingScale']))*unit**3;area=profile.XDim*profile.YDim-(4-math.pi)*profile.RoundingRadius**2;N=72;r['separateMath']={'idealArea':area,'idealVolume':area*factor,'N72PolygonVolume':(profile.XDim*profile.YDim-4*profile.RoundingRadius**2+N*profile.RoundingRadius**2*math.sin(2*math.pi/N)/2)*factor,'copiedManifestIdealVolume':c.get('idealVolume')}
  try:
   shape=ifcopenshell.geom.create_shape(ifcopenshell.geom.settings(),f.by_id(int(row['product'])));a=list(shape.geometry.verts);ix=list(shape.geometry.faces);v=[a[i:i+3] for i in range(0,len(a),3)];volume=math.fsum((v[ix[i]][0]*(v[ix[i+1]][1]*v[ix[i+2]][2]-v[ix[i+1]][2]*v[ix[i+2]][1])+v[ix[i]][1]*(v[ix[i+1]][2]*v[ix[i+2]][0]-v[ix[i+1]][0]*v[ix[i+2]][2])+v[ix[i]][2]*(v[ix[i+1]][0]*v[ix[i+2]][1]-v[ix[i+1]][1]*v[ix[i+2]][0]))/6 for i in range(0,len(ix),3));r['mesh']={'status':'nonempty' if ix else 'empty','vertices':len(v),'triangles':len(ix)//3,'signedVolume':volume,'bounds':[[min(q[k] for q in v) for k in range(3)],[max(q[k] for q in v) for k in range(3)]] if v else None}
  except Exception as e:r['mesh']={'status':'unsupported','error':str(e)}
 except Exception as e:r['parseError']=str(e)
 out.append(r);print('[DEBUG]',r['id'],len(r.get('diagnostics',[])),r.get('profile',{}).get('radiusType'),r.get('mesh',{}).get('status'),flush=True)
(p/'🗑️generated/rounded-rectangle-independent-physical.json').write_text(json.dumps({'ifcopenshell':ifcopenshell.version,'manifestSha256':hashlib.sha256(manifest.read_bytes()).hexdigest(),'records':out},indent=2,default=str)+'\n',encoding='utf8')
