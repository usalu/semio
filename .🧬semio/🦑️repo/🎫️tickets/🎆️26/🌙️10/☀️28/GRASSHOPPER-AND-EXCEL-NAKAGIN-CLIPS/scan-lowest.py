from pathlib import Path
ROOT=Path(__file__).resolve().parent
source=(ROOT/'scan-reference.py').read_text(encoding='utf-8')
exec(source.split("canvas=Image.new")[0])
selected=np.flatnonzero((centres[:,1]>.17)&(centres[:,1]<.235))
canvas=Image.new('RGB',(1600,1100),'white')
draw=ImageDraw.Draw(canvas)
for k,theta in enumerate((math.pi/4,3*math.pi/4,5*math.pi/4,7*math.pi/4)):
    xoff,yoff=k%2*800,k//2*550
    horizontal=points[:,0]*math.cos(theta)+points[:,2]*math.sin(theta)
    deep=-points[:,0]*math.sin(theta)+points[:,2]*math.cos(theta)
    vertical=(points[:,1]-.17)*3*math.cos(.5)-deep*math.sin(.5)
    depth=(points[:,1]-.17)*3*math.sin(.5)+deep*math.cos(.5)
    for n in selected[np.argsort(depth[indices[selected]].mean(axis=1))]:
        face=indices[n]
        polygon=[(xoff+400+horizontal[i]*1100,yoff+300-vertical[i]*1100) for i in face]
        draw.polygon(polygon,fill=tuple(int(c) for c in colors[n]))
    draw.text((xoff+20,yoff+10),f'GLUON lowest zone, raw Y .17-.235, height exaggerated 3x, quadrant {k}',fill='black')
canvas.save(OUT/'gluon-lowest-axon.png')
print('[DEBUG] Lowest scan axons saved',flush=True)
