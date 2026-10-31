from pathlib import Path
ROOT=Path(__file__).resolve().parent
exec((ROOT/'scan-reference.py').read_text(encoding='utf-8').split("view=data['bufferViews'][data['images'][0]['bufferView']]")[0])
canvas=Image.new('RGB',(1600,1200),'white')
draw=ImageDraw.Draw(canvas)
record={}
for k,height in enumerate((.18,.195,.215,.235)):
    xoff,yoff=k%2*800,k//2*600
    segments=[]
    mask=(tri[:,:,1].min(axis=1)<=height)&(tri[:,:,1].max(axis=1)>=height)
    for face in tri[mask]:
        hits=[]
        for a,b in ((face[0],face[1]),(face[1],face[2]),(face[2],face[0])):
            if (a[1]<height)!=(b[1]<height):
                point=a+(b-a)*(height-a[1])/(b[1]-a[1])
                hits.append((float(point[0]),float(point[2])))
        if len(hits)==2:
            segments.append(hits)
            p=[(xoff+400+x*1100,yoff+300-z*1100) for x,z in hits]
            draw.line(p,fill=(25,25,25),width=1)
    for x in np.arange(-.3,.31,.06):
        px=xoff+400+x*1100
        draw.line((px,yoff+60,px,yoff+560),fill=(220,220,240))
        draw.text((px,yoff+570),f'{x:.2f}',fill='blue')
    for z in np.arange(-.18,.181,.06):
        py=yoff+300-z*1100
        draw.line((xoff+30,py,xoff+770,py),fill=(240,220,220))
        draw.text((xoff+10,py),f'{z:.2f}',fill='red')
    draw.text((xoff+20,yoff+10),f'Scan section Y={height:.3f} (raw normalized)',fill='black')
    record[str(height)]=segments
canvas.save(OUT/'gluon-lowest-sections.png')
(OUT/'gluon-lowest-segments.json').write_text(json.dumps(record))
print('[DEBUG] Lowest scan sections saved',flush=True)
