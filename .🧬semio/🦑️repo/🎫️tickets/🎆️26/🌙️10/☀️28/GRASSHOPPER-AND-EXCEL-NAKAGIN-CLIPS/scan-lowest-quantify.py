from pathlib import Path
ROOT=Path(__file__).resolve().parent
exec((ROOT/'scan-reference.py').read_text(encoding='utf-8').split("view=data['bufferViews'][data['images'][0]['bufferView']]")[0])
boxes={'A-east':(.18,.28,0,.06),'A-north':(.12,.215,.06,.12),'B-west-north':(-.245,-.18,0,.10),'B-west-south':(-.28,-.18,-.06,0)}
records={}
for name,(x0,x1,z0,z1) in boxes.items():
    mask=(centres[:,0]>x0+.003)&(centres[:,0]<x1-.003)&(centres[:,2]>z0+.003)&(centres[:,2]<z1-.003)&(centres[:,1]>.17)&(centres[:,1]<.285)
    row={}
    for sign in (1,-1):
        chosen=mask&(unit[:,1]*sign>.85)
        bins=np.arange(.17,.2851,.0005)
        hist,edges=np.histogram(centres[chosen,1],bins,weights=areas[chosen])
        peaks,_=find_peaks(hist,distance=8,prominence=.00001)
        ranked=sorted(peaks,key=lambda i:-hist[i])[:6]
        row['up' if sign>0 else 'down']=[(round(float((edges[i]+edges[i+1])/2),5),round(float(hist[i]),7)) for i in ranked]
    records[name]=row
ground=0.0025
a_top=float(points[(points[:,0]>.02)&(points[:,0]<.21),1].max())
b_top=float(points[points[:,0]<-.05,1].max())
records['height_ratio']={'ground_raw':ground,'A_top_raw':a_top,'B_top_raw':b_top,'B_over_A':(b_top-ground)/(a_top-ground),'B_if_A53_5':53.5*(b_top-ground)/(a_top-ground)}
(OUT/'gluon-lowest-quantification.json').write_text(json.dumps(records,indent=2))
print(json.dumps(records,indent=2),flush=True)
