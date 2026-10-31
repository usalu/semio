"""🪵 Construct only the missing indoor coal-bay floor over its retained lower plate."""
import copy,hashlib,importlib,json,sys,time
from pathlib import Path
from shapely.geometry import Polygon,box
from shapely.ops import unary_union
from shapely import constrained_delaunay_triangles
sys.path.insert(0,str(Path.cwd()))
A=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
B=Path(__file__).parent


def cells(shape):
    """🔺 Retain every positive constrained triangle without hulls or area filtering."""
    if shape.is_empty:
        return []
    polys=[shape] if shape.geom_type=='Polygon' else list(shape.geoms)
    result=[]
    for poly in polys:
        for triangle in constrained_delaunay_triangles(poly).geoms:
            assert triangle.area>0 and triangle.is_valid
            result.append([[float(x),float(y)] for x,y in list(triangle.exterior.coords)[:-1]])
    assert abs(sum(Polygon(r).area for r in result)-shape.area)<1e-12
    return result


def build(m):
    """🧱 Keep the exterior plate and cover; continue the indoor wearing datum with qualified levelling."""
    source=hashlib.sha256(A._source().encode('utf-8')).hexdigest()
    context=json.loads((B/'coal-bay-wall-context-r94.json').read_text(encoding='utf-8'))
    assert context['source_sha256']==source
    raw=next(p for p in A._find(m,'SL-GROUND-CELLAR')['geometry']['regions'] if p['tag']=='R-SL-GROUND-COAL-CEM-1')
    assert raw['top_offset']['value']==-.13
    plate=Polygon(raw['outline'])
    west=min(q[0] for w in context['walls'] if w['tag']=='W-G-W' for q in w['p']['outline'])
    target=plate.intersection(box(west,plate.bounds[1],plate.bounds[2],plate.bounds[3]))
    walls=unary_union([Polygon(w['p']['outline']) for w in context['walls']]).intersection(target)
    rooms={tag:Polygon(A._expand(m,next(s for s in m['spaces'] if s['tag']==tag)['outline'])).intersection(target).difference(walls) for tag in ['SP-SERVANTS-1','SP-SERVANTS-2']}
    assert unary_union([walls,*rooms.values()]).symmetric_difference(target).area<1e-12
    rows={tag:[] for tag in ['FIN-SERVANTS-1','FIN-SERVANTS-1-WASH','FIN-SERVANTS-2','FIN-SERVANTS-2-WASH']}
    def add(tag,region,z0,z1,material,role):
        for ring in cells(region):
            rows[tag].append({'outline':ring,'z0':z0,'z1':z1,'material':material,'role':role})
    add('FIN-SERVANTS-1',target,-.13,-.04,'MAT-MORTAR-PUTTY','inferred_indoor_coal_bay_levelling')
    add('FIN-SERVANTS-1',walls,-.04,0.,'MAT-MORTAR-PUTTY','inferred_continuous_wall_underlay')
    patches=unary_union([box(3.225,14.88529411764706,3.939705882352941,15.723529411764707),box(3.9676470588235295,15.723529411764707,4.722058823529411,16.533823529411766)])
    for room,region in rooms.items():
        tag='FIN-'+room[3:]
        wet=region.intersection(patches)
        wood=region.difference(wet)
        add(tag+'-WASH',wet,-.015,0.,'MAT-GRES-OCHRE','wearing')
        add(tag+'-WASH',wet,-.04,-.015,'MAT-MORTAR-PUTTY','bedding')
        add(tag,wood,-.025,0.,'MAT-OAK','wearing')
        lo,_,hi,_=wood.bounds
        k=0
        while lo+k*.4<=hi:
            strip=wood.intersection(box(lo+k*.4,plate.bounds[1],lo+k*.4+.04,plate.bounds[3]))
            add(tag,strip,-.04,-.039,'MAT-BITUMEN','inferred_bitumen_batten_bed')
            add(tag,strip,-.039,-.025,'MAT-OAK','inferred_batten')
            k+=1
    return {'source_sha256':source,'target_area':target.area,'wall_area':walls.area,'additions':rows,'qualification':{'en':'Continuous indoor levelling over the lower coal plate is inferred to reconcile the retained −0.13m service datum with indoor finished ±0.00m. Mineral composition and 90mm depth are reconstruction assumptions; wearing continues under the fixed cupboard. The original lower plate, exterior coal cover, openings and their datums are unchanged.','de':'Durchgehender Innenausgleich über der tieferen Kohlenplatte ist zur Verbindung des erhaltenen Installationsniveaus −0,13m mit dem Innenfertigniveau ±0,00m abgeleitet. Mineralische Zusammensetzung und 90mm Höhe sind Rekonstruktionsannahmen; Belag läuft unter dem festen Schrank durch. Tiefere Platte, äußerer Kohlendeckel, Öffnungen und deren Niveaus bleiben erhalten.'}}


def apply(m,candidate):
    """📦 Apply the bounded candidate only to its existing four finish identities."""
    for tag,parts in candidate['additions'].items():
        e=A._find(m,tag)
        e['geometry']['parts'].extend(copy.deepcopy(parts))
        for language,text in candidate['qualification'].items():
            e['historical_evidence']['uncertainty'][language]+=' '+text
    return m


if __name__=='__main__':
    result=build(A.load_model())
    out=B/'🗑️generated/native-r92/coal-bay-floor-candidate-r94.json'
    out.write_text(json.dumps(result,ensure_ascii=True,indent=2),encoding='utf-8')
    print('[DEBUG] '+json.dumps({k:v for k,v in result.items() if k not in ['additions','qualification']}),flush=True)
    print('[DEBUG] additions '+json.dumps({k:len(v) for k,v in result['additions'].items()}),flush=True)
