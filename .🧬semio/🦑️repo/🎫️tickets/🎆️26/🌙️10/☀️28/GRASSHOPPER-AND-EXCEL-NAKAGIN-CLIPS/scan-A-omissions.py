from pathlib import Path
ROOT=Path(__file__).resolve().parent
exec((ROOT/'scan-reference.py').read_text(encoding='utf-8').split("view=data['bufferViews'][data['images'][0]['bufferView']]")[0])
regions={'N0-Slash':('z',.15,.06,.12,0),'N1-J':('x',.21,.06,.12,1),'E0-J':('z',.09,0,.06,0),'E1-L':('z',-.09,0,.06,0),'S0-Backslash':('z',-.15,.06,.12,0),'S1-P':('z',-.15,.12,.18,2),'W0-Slash':('x',.27,0,.06,1),'W1-S':('x',.24,-.09,0,2)}
records={}
for floor,base in ((4,.2230),(5,.2827),(13,.7565)):
    row={}
    for name,(axis,plane,lo,hi,phase) in regions.items():
        k,j=(2,0) if axis=='z' else (0,2)
        bottom=base+phase*.0199
        mask=(np.abs(centres[:,k]-plane)<.006)&(centres[:,j]>lo+.004)&(centres[:,j]<hi-.004)&(centres[:,1]>bottom+.008)&(centres[:,1]<bottom+.049)&(np.abs(unit[:,k])>.65)
        row[name]={'vertical_area':float(areas[mask].sum()),'triangles':int(mask.sum()),'height_band':[bottom+.008,bottom+.049]}
    records[str(floor)]=row
(OUT/'gluon-A-omissions.json').write_text(json.dumps(records,indent=2))
print(json.dumps(records,indent=2),flush=True)
