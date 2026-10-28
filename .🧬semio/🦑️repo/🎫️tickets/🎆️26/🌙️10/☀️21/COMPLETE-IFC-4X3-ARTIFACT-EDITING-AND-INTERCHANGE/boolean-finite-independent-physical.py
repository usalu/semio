"""🪚 Observe exact STEP bytes through installed external parse, full EXPRESS and mesh APIs."""
from pathlib import Path
import json, hashlib, math
import ifcopenshell, ifcopenshell.validate, ifcopenshell.geom
folder=Path(__file__).parent/'🗑️generated/boolean-finite-independent'
class PhysicalOracle:
    """🔎 Return owned JSON observations without source reconstruction/coercion."""
    def observe(self,row):
        path=Path(row['path']); raw=path.read_bytes(); result={'id':row['input']['id'],'sha256':hashlib.sha256(raw).hexdigest()}
        try:
            source=ifcopenshell.open(str(path)); result['entities']=len(list(source)); logger=ifcopenshell.validate.json_logger(); ifcopenshell.validate.validate(source,logger,express_rules=True); result['diagnostics']=logger.statements
            try:
                owner=ifcopenshell.geom.create_shape(ifcopenshell.geom.settings(),source.by_id(int(row['product']))); flat=list(owner.geometry.verts); indices=list(owner.geometry.faces); p=[flat[i:i+3] for i in range(0,len(flat),3)]; volume=math.fsum((p[indices[i]][0]*(p[indices[i+1]][1]*p[indices[i+2]][2]-p[indices[i+1]][2]*p[indices[i+2]][1])+p[indices[i]][1]*(p[indices[i+1]][2]*p[indices[i+2]][0]-p[indices[i+1]][0]*p[indices[i+2]][2])+p[indices[i]][2]*(p[indices[i+1]][0]*p[indices[i+2]][1]-p[indices[i+1]][1]*p[indices[i+2]][0]))/6 for i in range(0,len(indices),3)); result['mesh']={'status':'nonempty' if indices else 'empty','vertices':len(p),'triangles':len(indices)//3,'volume':volume,'bounds':[[min(v[a] for v in p) for a in range(3)],[max(v[a] for v in p) for a in range(3)]] if p else None}
            except Exception as error: result['mesh']={'status':'unsupported','error':str(error)}
        except Exception as error: result['parseError']=str(error)
        return result
rows=json.loads((folder/'emitted.json').read_text(encoding='utf8')); oracle=PhysicalOracle(); output=[]
for row in rows:
    result=oracle.observe(row); output.append(result); print('[DEBUG]',result['id'],len(result.get('diagnostics',[])),result.get('mesh',{}).get('status'),flush=True)
(folder/'physical.json').write_text(json.dumps({'version':ifcopenshell.version,'rows':output},indent=2,default=str),encoding='utf8')

