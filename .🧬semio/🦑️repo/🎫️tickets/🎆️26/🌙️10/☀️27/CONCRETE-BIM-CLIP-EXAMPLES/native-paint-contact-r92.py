"""🖌️ Inspect actual source coloured wall/panel faces through the real drawing provider."""
import hashlib,importlib,json,math,sys,time
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont,ImageColor
ROOT=Path.cwd();sys.path.insert(0,str(ROOT));OUT=Path(__file__).parent/'🗑️generated/native-r92'
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
prim=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.🧩️primitives')
paths=[Path(api.__file__),Path(api.__file__).with_name('🔣️.json'),Path(api._surface_finishes().__file__),Path(prim.__file__)]
def hashes():
    """🔐 Capture current source/producer identity without copying tracked sources."""
    return {str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
theme=importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")
before=hashes();m=api.load_model();font=ImageFont.truetype(str(theme.font_sources()[theme.FONT_SANS][0]),18);im=Image.new('RGB',(1600,1200),'#EEEEEA');draw=ImageDraw.Draw(im);records=[]
for index,tag in enumerate(('W-KITCHEN','W-SALON','W-E','GARAGE-SLIDING')):
    if (OUT/'paint-contact.cancel').exists():raise KeyboardInterrupt('Paint visual capture cancelled')
    print('[DEBUG] source face '+tag,flush=True);obj=api._find(m,tag)
    if obj['class']=='IfcWall':
        g=api._wall_geometry(m,obj);faces=[f for _,piece in prim._wall_solids(g,{}) for f in piece];a,b=g['start'],g['end'];length=g['length'];base=g['base'];height=g['top']-base
    else:
        g=api._filling_geometry(m,obj);faces=prim._window_faces({}, {'fill_tag':tag},{'fillings':[g]});a,b=g['axis'];length=math.dist(a,b);base=min(p['z0'] for p in g['parts']);height=max(p['z1'] for p in g['parts'])-base
    direction=[(b[k]-a[k])/length for k in range(2)];x0=40+(index%2)*800;y0=50+(index//2)*600;scale=min(700/length,470/height)
    draw.text((x0,y0-30),tag+' — actual source / tatsächliche Quelle',fill='#222222',font=font)
    selected=[f for f in faces if isinstance(f[3],dict)];area=0
    for face in selected:
        rgb=api._surface_finishes().appearance_colour(face[3],'#FFFFFF');points=[(x0+sum((v[k]-a[k])*direction[k] for k in range(2))*scale,y0+height*scale-(v[2]-base)*scale) for v in face[1][0]]
        draw.polygon(points,fill=rgb);area+=api._surface_finishes().face_area(face[1][0])
    assert selected
    draw.text((x0,y0+height*scale+15),'RGB illustrative; original pigment unmeasured / RGB abgeleitet',fill='#333333',font=font)
    records.append({'tag':tag,'coloured_faces':len(selected),'physical_face_area':area,'rgb_inferred':True})
im.save(OUT/'paint-contact.png');after=hashes();payload={'before':before,'after':after,'stable':before==after,'records':records}
(OUT/'paint-contact.json').write_text(json.dumps(payload,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({'stable':payload['stable'],'records':records}),flush=True)
