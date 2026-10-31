"""🛁 Author inferred native shared-sanitary assets from registered original programme without modifying model source."""
import importlib,json,math,sys
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');model=api.load_model();ticket=Path(__file__).parent
case=json.loads((root/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🛁️bathrooms.json').read_text(encoding='utf8'))
def ellipse(cx,cy,rx,ry,n=48):
    """🫧 Publish a source-qualified inferred ellipse as builtin metre coordinates."""
    return [[round(cx+rx*math.cos(2*math.pi*i/n),12),round(cy+ry*math.sin(2*math.pi*i/n),12)] for i in range(n)]
def emit(parts,outline,z0,z1,material,role,top=None):
    """🧱 Emit one explicit native occupied material piece."""
    parts.append({'outline':outline,'z0':z0,'z1':z1,'material':material,'role':role,**({'top_outline':top} if top else {})})
def ring(parts,outer,inner,z0,z1,material,role,top_outer=None,top_inner=None):
    """⭕ Preserve an open material ring through disjoint convex pieces."""
    for i in range(len(outer)):
        j=(i+1)%len(outer);top=[top_outer[i],top_outer[j],top_inner[j],top_inner[i]] if top_outer else None
        emit(parts,[outer[i],outer[j],inner[j],inner[i]],z0,z1,material,role,top)
def taps(parts,cx,cy,offsets,back,base):
    """🚰 Add inferred hollow tap stems and complementary mitered spouts over the actual cavity."""
    for offset in offsets:
        joint=[cx+offset,cy+back,.99 if base>.6 else .55];plane={'normal':[0.,1.,1.],'point':joint}
        for kind,points,clip in [('tap_stem',[[cx+offset,cy+back,base],joint],{'clip_end':plane}),('tap_spout',[joint,[cx+offset,cy-.02,joint[2]]],{'clip_start':plane})]:
            obj={'guid':api.global_id('AUTHORING-'+kind),'tag':'AUTHORING-'+kind,'class':'IfcPipeSegment','storey':'first','system':'water','material':'MAT-METAL','geometry':{'points':points,'tube':{'wall_thickness':{'value':.002},'segments':32,**clip}},'attributes':{'diameter':{'value':.025}}}
            for p in api._service(model,obj)['parts']:
                body={k:p[k] for k in ('mesh','section','outline','top_outline','z0','z1') if k in p}
                if 'mesh' in body:body={'mesh':body['mesh']}
                elif 'section' in body:body={'section':body['section']}
                parts.append({**body,'material':'MAT-METAL','role':kind})
def basin(cx,cy):
    """🏺 Construct an inferred elliptical pedestal basin with real bowl and stem voids."""
    parts=[];ratio=.235/.31;outer=ellipse(cx,cy,.31,.235);inner=ellipse(cx,cy,.295,.295*ratio);base=ellipse(cx,cy,.13,.13*ratio);hole=ellipse(cx,cy,.025,.025)
    ring(parts,base,hole,0,.04,'MAT-SANITARY-CERAMIC','pedestal_foot')
    ring(parts,ellipse(cx,cy,.065,.065*ratio),hole,.04,.65,'MAT-SANITARY-CERAMIC','pedestal_stem')
    ring(parts,base,hole,.65,.665,'MAT-SANITARY-CERAMIC','bowl_bottom')
    ring(parts,base,ellipse(cx,cy,.105,.105*ratio),.665,.82,'MAT-SANITARY-CERAMIC','bowl_shell',outer,inner)
    deck=api._analysis._clip(outer,[[cx-.4,cy-.3],[cx+.4,cy-.3],[cx+.4,cy-.16],[cx-.4,cy-.16]],digits=None)
    emit(parts,deck,.82,.835,'MAT-SANITARY-CERAMIC','tap_deck')
    taps(parts,cx,cy,[-.1,.1],-.195,.835)
    return parts
def bidet(cx,cy):
    """🚿 Construct an inferred hollow original bidet with a distinct open wash bowl."""
    parts=[];ratio=1.4;bottom=ellipse(cx,cy,.11,.154);hole=ellipse(cx,cy,.025,.025)
    ring(parts,bottom,hole,0,.16,'MAT-SANITARY-CERAMIC','bidet_base')
    ring(parts,bottom,ellipse(cx,cy,.08,.112),.16,.38,'MAT-SANITARY-CERAMIC','bowl_shell',ellipse(cx,cy,.18,.252),ellipse(cx,cy,.145,.203))
    taps(parts,cx,cy,[0.],-.23,.38)
    return parts
def capsule(cx,cy,left,right,radius,n=24):
    """🛀 Describe one capsule boundary with corresponding circular cap stations."""
    return [[round(c+r*math.cos(a),12),round(cy+r*math.sin(a),12)] for c,r,angles in [(right,radius,[-math.pi/2+math.pi*i/n for i in range(n+1)]),(left,radius,[math.pi/2+math.pi*i/n for i in range(n+1)])] for a in angles]
def tub():
    """🪠 Construct an inferred hollow metal/enamel tub with a real bottom waste aperture."""
    x0,y0,x1,y1=case['tub_bounds'];cx,cy=(x0+x1)/2,(y0+y1)/2;r=(y1-y0)/2;left,right=x0+r,x1-r
    outer=capsule(cx,cy,left,right,r);metal_inner=capsule(cx,cy,left,right,r-.008);inner=capsule(cx,cy,left,right,r-.009);drain=ellipse(right,cy,.02,.02,24);parts=[]
    for boundary,z0,z1,mat,role in [(outer,.04,.048,'MAT-METAL','tub_bottom'),(metal_inner,.048,.049,'MAT-SANITARY-CERAMIC','enamel_bottom')]:
        mesh=api.profile_mesh([boundary,drain])
        for triangle in mesh['triangles']:emit(parts,[mesh['vertices'][i] for i in triangle],z0,z1,mat,role)
    ring(parts,outer,metal_inner,.048,.55,'MAT-METAL','tub_shell')
    ring(parts,metal_inner,inner,.049,.55,'MAT-SANITARY-CERAMIC','enamel_lining')
    for x in [left,right]:
        for y in [cy-.18,cy+.18]:emit(parts,[[x-.04,y-.05],[x+.04,y-.05],[x+.04,y+.05],[x-.04,y+.05]],0,.04,'MAT-BRICK','concealed_support')
    return parts,[cx,cy]
assets={'BIDET-SHARED':{'axis':case['fixtures'][0]['axis'],'parts':bidet(*case['fixtures'][0]['axis'])},'FIX-SHARED-BASIN':{'axis':case['fixtures'][1]['axis'],'parts':basin(*case['fixtures'][1]['axis'])}}
parts,axis=tub();assets['FIX-SHARED-BATH']={'axis':axis,'parts':parts};(ticket/'🗑️generated/shared-sanitary-assets-r92.json').write_text(json.dumps(assets,ensure_ascii=False),encoding='utf8');print(json.dumps({k:len(v['parts']) for k,v in assets.items()}),flush=True)
