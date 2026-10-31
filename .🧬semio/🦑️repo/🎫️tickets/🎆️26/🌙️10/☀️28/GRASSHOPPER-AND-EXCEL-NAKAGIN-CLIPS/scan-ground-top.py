from pathlib import Path
ROOT=Path(__file__).resolve().parent
exec((ROOT/'scan-reference.py').read_text(encoding='utf-8').split("view=data['bufferViews'][data['images'][0]['bufferView']]")[0])
mask=(np.abs(unit[:,1])>.95)&(centres[:,1]>-.01)&(centres[:,1]<.015)
xyz=centres[mask]
weights=areas[mask].astype(float)
design=np.column_stack((xyz[:,0],xyz[:,2],np.ones(len(xyz))))
coef=np.linalg.lstsq(design*np.sqrt(weights[:,None]),xyz[:,1]*np.sqrt(weights),rcond=None)[0]
for _ in range(12):
    residual=xyz[:,1]-design@coef
    ww=weights/(1+(residual/.001)**2)
    coef=np.linalg.lstsq(design*np.sqrt(ww[:,None]),xyz[:,1]*np.sqrt(ww),rcond=None)[0]
rows={'ground_plane_Y_aX_bZ_c':coef.tolist(),'ground_triangles':len(xyz),'ground_weighted_rms':float(np.sqrt(np.average((xyz[:,1]-design@coef)**2,weights=ww)))}
tops={}
for tower,x0,x1 in (('A',.055,.185),('B',-.19,-.055)):
    subset=points[(points[:,0]>x0)&(points[:,0]<x1)&(np.abs(points[:,2])<.067)&(points[:,1]>.85)]
    high=float(subset[:,1].max())
    tip=subset[subset[:,1]>high-.003]
    ground=float(np.array(((x0+x1)/2,0,1))@coef)
    tops[tower]={'top_Y':high,'near_tip_points':len(tip),'tip_bounds':[tip.min(axis=0).tolist(),tip.max(axis=0).tolist()],'ground_Y_at_core':ground,'span':high-ground}
scale=53.5/tops['A']['span']
rows.update({'tops':tops,'scale_from_A53_50':scale,'B_total_m':tops['B']['span']*scale,'pitch_m_from_medians':[.0595*scale,.0596666667*scale],'pitch_m_from_A_regression':.0596584507*scale})
(OUT/'gluon-ground-top-registration.json').write_text(json.dumps(rows,indent=2))
print(json.dumps(rows,indent=2),flush=True)
