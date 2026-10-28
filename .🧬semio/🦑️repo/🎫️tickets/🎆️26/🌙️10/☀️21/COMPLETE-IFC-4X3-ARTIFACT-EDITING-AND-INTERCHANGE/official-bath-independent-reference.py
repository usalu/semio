"""🛁 Independently recompute owned external bath geometry evidence and immutable source custody."""
from pathlib import Path
import json,hashlib,math,collections
p=Path(__file__).parent;data=json.loads((p/'🗑️generated/official-bath-geometry-reference.json').read_text(encoding='utf8'));mesh=data['mesh'];points=mesh['positions'];indices=mesh['indices'];edges=collections.defaultdict(lambda:[0,0]);terms=[]
for at in range(0,len(indices),3):
    ids=indices[at:at+3];a,b,c=[points[i] for i in ids];terms.append((a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]))/6)
    for j in range(3):
        x,y=ids[j],ids[(j+1)%3];key=tuple(sorted((x,y)));edges[key][0]+=1;edges[key][1]+=1 if x<y else -1
source=Path(data['source']);digest=hashlib.sha256(source.read_bytes()).hexdigest();volume=math.fsum(terms);ideal=1.28-.7*(1.8*.6-(4-math.pi)*.2**2);result={'sourceSha256':digest,'sourceMatch':digest==data['sourceSha256'],'library':data['library'],'diagnostics':len(data['diagnostics']),'vertices':len(points),'triangles':len(indices)//3,'edges':len(edges),'edgeOwnersValid':all(count==2 and direction==0 for count,direction in edges.values()),'signedVolume':volume,'reportedVolume':mesh['signedVolume'],'ideal':ideal,'externalMinusIdeal':volume-ideal,'ourProducerExecuted':False};assert result['sourceMatch'] and result['edgeOwnersValid'] and abs(volume-mesh['signedVolume'])<1e-12;(p/'🗑️generated/official-bath-independent-reference.json').write_text(json.dumps(result,indent=2),encoding='utf8');print('[DEBUG]',result)
