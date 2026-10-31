from pathlib import Path
ROOT=Path(__file__).resolve().parent
exec((ROOT/'scan-reference.py').read_text(encoding='utf-8').split("view=data['bufferViews'][data['images'][0]['bufferView']]")[0])
boxes={'B-N-Slash':(-.178,-.122,.062,.148),'B-S-J':(-.211,-.123,-.115,-.060),'B-E-J':(-.245,-.18,0,.10),'B-E-Slash':(-.28,-.18,-.06,0)}
period=float(np.polyfit([0,1,2,3,6,8],[.24325,.30375,.36375,.42325,.60225,.72075],1)[0])
rows={'period_from_A_six_levels':period,'regions':{}}
for name,(x0,x1,z0,z1) in boxes.items():
    base=(centres[:,0]>x0+.004)&(centres[:,0]<x1-.004)&(centres[:,2]>z0+.004)&(centres[:,2]<z1-.004)&(centres[:,1]>.215)&(centres[:,1]<.755)
    results={}
    for sign,label in ((-1,'down'),(1,'up')):
        mask=base&(unit[:,1]*sign>.85)
        yy,ww=centres[mask,1],areas[mask]
        hist,edges=np.histogram(yy,np.arange(.215,.7551,.0005),weights=ww)
        peaks,_=find_peaks(hist,distance=8,prominence=.000005)
        strong=sorted(peaks,key=lambda i:-hist[i])[:24]
        samples=[]
        anchor=.2230+(.057 if sign>0 else 0)
        for floor in (4,5,6,7,8,9,10,11):
            levels=[]
            for phase in (0,1,2):
                target=anchor+(floor-4)*period+phase*period/3
                ok=np.abs(yy-target)<.007
                levels.append({'phase':phase,'target':round(target,5),'area':float(ww[ok].sum()),'weighted_y':float(np.average(yy[ok],weights=ww[ok])) if ok.any() else None})
            samples.append({'floor':floor,'levels':levels})
        results[label]={'peaks':sorted([(round(float((edges[i]+edges[i+1])/2),5),round(float(hist[i]),7)) for i in strong]),'floors':samples}
    rows['regions'][name]=results
(OUT/'gluon-B-phase-order.json').write_text(json.dumps(rows,indent=2))
for name,row in rows['regions'].items():
    print(name,flush=True)
    for sign in ('down','up'):
        print(sign,'peaks',row[sign]['peaks'],flush=True)
        print('scores',[(f['floor'],[(q['phase'],round(q['area'],6),None if q['weighted_y'] is None else round(q['weighted_y'],5)) for q in f['levels']]) for f in row[sign]['floors']],flush=True)
print('P',period,flush=True)
