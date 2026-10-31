from pathlib import Path
ROOT=Path(__file__).resolve().parent
exec((ROOT/'scan-reference.py').read_text(encoding='utf-8').split("view=data['bufferViews'][data['images'][0]['bufferView']]")[0])
boxes={'B-N0':(-.118,-.064,.062,.148),'B-N-Slash':(-.178,-.122,.062,.148),'B-S0':(-.118,-.064,-.148,-.062),'B-S-J':(-.211,-.123,-.115,-.060),'B-E-J':(-.245,-.18,0,.10),'B-E-Slash':(-.28,-.18,-.06,0),'A-N-J':(.12,.215,.06,.12),'A-W-Slash':(.18,.28,0,.06)}
samples={}
for name,(x0,x1,z0,z1) in boxes.items():
    mask=(centres[:,0]>x0+.004)&(centres[:,0]<x1-.004)&(centres[:,2]>z0+.004)&(centres[:,2]<z1-.004)&(centres[:,1]>.215)&(centres[:,1]<.735)&(unit[:,1]<-.85)
    samples[name]=(centres[mask,1],areas[mask])
anchor_y,anchor_w=samples['B-N0']
periods=np.linspace(.055,.065,1001)
scores=np.array([abs(np.sum(anchor_w*np.exp(2j*np.pi*anchor_y/p)))/anchor_w.sum() for p in periods])
period=float(periods[scores.argmax()])
anchor=float(np.angle(np.sum(anchor_w*np.exp(2j*np.pi*anchor_y/period)))/(2*np.pi)*period%period)
rows={'period':period,'anchor_modulo':anchor,'anchor_coherence':float(scores.max()),'regions':{}}
for name,(yy,ww) in samples.items():
    value=np.sum(ww*np.exp(2j*np.pi*yy/period))
    phase=float(np.angle(value)/(2*np.pi)*period%period)
    hist,edges=np.histogram(yy,np.arange(.215,.7351,.0005),weights=ww)
    peaks,_=find_peaks(hist,distance=10,prominence=.000005)
    strong=sorted(peaks,key=lambda i:-hist[i])[:18]
    rows['regions'][name]={'area':float(ww.sum()),'coherence':float(abs(value)/ww.sum()),'relative_phase':float(((phase-anchor)%period)/period),'peaks':sorted([(round(float((edges[i]+edges[i+1])/2),5),round(float(hist[i]),7)) for i in strong])}
(OUT/'gluon-regular-phases.json').write_text(json.dumps(rows,indent=2))
print(json.dumps(rows,indent=2),flush=True)
