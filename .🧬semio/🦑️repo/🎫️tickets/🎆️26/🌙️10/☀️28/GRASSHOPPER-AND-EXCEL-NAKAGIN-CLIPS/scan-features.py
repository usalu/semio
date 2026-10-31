from pathlib import Path
from scipy.sparse import coo_matrix
from scipy.sparse.csgraph import connected_components

ROOT=Path(__file__).resolve().parent
exec((ROOT/'scan-reference.py').read_text(encoding='utf-8').split("view=data['bufferViews'][data['images'][0]['bufferView']]")[0])
_,weld=np.unique(points,axis=0,return_inverse=True)
selected=np.flatnonzero(np.abs(unit[:,1])>.85)
faces=weld[indices[selected]]
edges=np.sort(np.concatenate((faces[:,[0,1]],faces[:,[1,2]],faces[:,[2,0]])),axis=1)
owners=np.tile(np.arange(len(selected)),3)
order=np.lexsort((edges[:,1],edges[:,0]))
edges,owners=edges[order],owners[order]
same=np.all(edges[1:]==edges[:-1],axis=1)
a,b=owners[:-1][same],owners[1:][same]
matrix=coo_matrix((np.ones(len(a)),(a,b)),shape=(len(selected),len(selected)))
count,labels=connected_components(matrix,directed=False)
weights=np.bincount(labels,weights=areas[selected])
rows=[]
for label in np.flatnonzero(weights>.00015):
    fi=selected[labels==label]
    p=points[np.unique(indices[fi])]
    lo,hi=p.min(axis=0),p.max(axis=0)
    rows.append({'normal_y':float(np.average(unit[fi,1],weights=areas[fi])),'component':int(label),'area':float(weights[label]),'bounds':[lo.tolist(),hi.tolist()],'extent':(hi-lo).tolist(),'triangles':len(fi)})
rows.sort(key=lambda row:-row['area'])
(OUT/'gluon-horizontal-components.json').write_text(json.dumps(rows,indent=2))
print('[DEBUG] Signed surface components saved',flush=True)
