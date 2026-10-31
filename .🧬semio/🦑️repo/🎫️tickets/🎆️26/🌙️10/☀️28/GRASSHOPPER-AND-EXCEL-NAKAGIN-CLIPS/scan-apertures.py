from pathlib import Path
import json,io,math
ROOT=Path(__file__).resolve().parent
exec((ROOT/'scan-reference.py').read_text(encoding='utf8').split("view=data['bufferViews'][data['images'][0]['bufferView']]")[0])
view=data['bufferViews'][data['images'][0]['bufferView']]
tex=np.asarray(Image.open(io.BytesIO(blob[view['byteOffset']:view['byteOffset']+view['byteLength']])).convert('RGB'))
coords=uv[indices].mean(axis=1)
colors=tex[np.minimum((np.mod(coords[:,1],1)*tex.shape[0]).astype(int),tex.shape[0]-1),np.minimum((np.mod(coords[:,0],1)*tex.shape[1]).astype(int),tex.shape[1]-1)]
dark=colors.mean(axis=1)<85
coverage=json.loads((OUT/'gluon-all-capsule-coverage.json').read_text())
for row in coverage['rows']:
 x,y,z=row['center'];ang=math.radians(row['rotation']);c,s=math.cos(ang),math.sin(ang)
 mx=-46.25*(centres[:,0]+.12);my=46.25*centres[:,2]
 lx=(mx-x)*c+(my-y)*s;ly=-(mx-x)*s+(my-y)*c
 roof=row['phase_scores'][row['phase']]['roof_raw']
 h=(centres[:,1]-roof)*46.81675+2.5
 if row['window_wall']=='front':
  plane=abs(ly-2)<.28;u=lx;normal=unit[:,0]*s+unit[:,2]*c
 else:
  plane=abs(lx-1.25)<.28;u=ly-.95;normal=-unit[:,0]*c+unit[:,2]*s
 valid=plane&(normal>.65)
 rad=np.sqrt(u*u+(h-1.25)**2)
 inner=valid&(rad<.46);ann=valid&(rad>.73)&(rad<.93)
 ai=float(areas[inner].sum());aa=float(areas[ann].sum())
 di=float(areas[inner&dark].sum()/ai) if ai else None
 da=float(areas[ann&dark].sum()/aa) if aa else None
 positive=ai>.00008 and aa>.00008 and di>.55 and da<.4 and di-da>.25
 row['window_photometry']={'method':'area-weighted triangle UV-centroid luminance; not a fitted aperture contour','inner_area':ai,'annulus_area':aa,'inner_dark_fraction':di,'annulus_dark_fraction':da,'status':'positive_dark_disc_light_surround' if positive else 'unresolved_capture_finish_or_registration'}
(OUT/'gluon-all-capsule-coverage.json').write_text(json.dumps(coverage,indent=2))
print('[DEBUG] Positive aperture-facing contrast',sum(r['window_photometry']['status'].startswith('positive')for r in coverage['rows']))
im=Image.new('RGB',(1500,700),'white');draw=ImageDraw.Draw(im)
for number,yy in enumerate((.3495,.5272,.7058)):
 sel=(abs(centres[:,0])<.085)&(centres[:,2]>-.115)&(centres[:,2]<.045)&(abs(centres[:,1]-yy)<.005)&(unit[:,1]>.8)
 for k in np.flatnonzero(sel):
  poly=[(250+number*500+float(q[0])*2600,230-float(q[2])*2600) for q in tri[k]]
  draw.polygon(poly,fill=tuple(int(v)for v in colors[k]))
 draw.text((number*500+20,20),f'Bridge roof-plane rawY{yy}',fill='black')
im.save(OUT/'gluon-bridge-top-planes.png')
