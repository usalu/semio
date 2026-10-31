"""🪵 Author continuous visible cupboard-front timber plinths from the retained source register."""
from copy import deepcopy
from pathlib import Path
import importlib,json,math,sys
BASE=Path(__file__).parent
def build(model,api,finish):
    """🪵 Preserve hollow cupboard construction while fitting new continuous15×90 mm front boards."""
    register=json.loads((BASE/'cupboard-skirting-register-r92.json').read_text(encoding='utf8'));result=[];checks=[]
    for row in register['runs']:
        obj=api._find(model,row['host']);path=[[*p,0.] for p in row['path']];parts=[]
        joints={i:finish.skirting_joint(path[i-1],path[i],path[i+1],register['section']['thickness']) for i in range(1,len(path)-1)}
        for index,(a,b) in enumerate(zip(path,path[1:])):
            delta=[b[k]-a[k] for k in range(3)];length=math.hypot(*delta);axis=[v/length for v in delta];normal=[axis[1],-axis[0],0.]
            start=joints.get(index,{'plane':{'normal':axis,'offset':sum(x*y for x,y in zip(axis,a)),'point':a},'extension':0.})
            end=joints.get(index+1,{'plane':{'normal':axis,'offset':sum(x*y for x,y in zip(axis,b)),'point':b},'extension':0.})
            stock=finish.skirting_stock(a,b,normal,register['section'],'MAT-SKIRTING-GRAY',extensions=(start['extension'],end['extension']),back_clearance=register['fitting_clearance'])
            cuts=[{**start['plane'],'normal':[-v for v in start['plane']['normal']],'offset':-start['plane']['offset']},end['plane']]
            part=finish.publish_stock(finish.clip_stock(stock,cuts,exact=True));api._polyhedron_part(part['mesh']['vertices'],part['mesh']['faces']);parts.append(part)
        tag='SKIRT-'+obj['tag']+'-front'
        product={'tag':tag,'guid':api.global_id(tag),'class':'IfcCovering','predefined_type':'SKIRTINGBOARD','name':{'en':'Continuous original gray timber cupboard-front plinth at '+obj['tag'],'de':'Durchgehende ursprüngliche graue Holzsockelleiste vor '+obj['tag']},'storey':obj['storey'],'material':'MAT-SKIRTING-GRAY','geometry':{'kind':'skirting','base_level':obj['storey'],'parts':parts},'attributes':{},'psets':{},'metadata':{'author':'Atelier (architecture)','revision':1,'domains':['architecture']},'historical_evidence':{'state':'original-completed-1930-1931','status':'inferred_reconstruction','source_kind':'mixed','sources':['SAB2024-p190; FLC H1(13)01610 estimate'],'uncertainty':{'en':'Continuous visible cupboard-front skirting and 15×90 mm painted timber section are sourced. Exact front endpoints, gray display RGB, 2 mm rounded top radius, 0.1 mm installation fitting clearance and mitre joints are inferred. Existing reconstructed hollow carcasses and raised cupboard floor members remain; no universal cement infill or measured historical gap is asserted.','de':'Durchgehender sichtbarer Schranksockel und gestrichenes15×90 mm Holzprofil sind belegt. Endpunkte, graues Anzeige-RGB, 2 mm Rundungsradius, 0.1 mm Einpassluft und Fugen sind abgeleitet. Vorhandene hohle Korpusse und erhöhte Schrankbodenbauteile bleiben erhalten; keine allgemeine Zementfüllung oder historisch gemessene Fuge wird behauptet.'}}}
        result.append(product)
    return {'elements':result,'sliding_clearance':register['sliding_clearance'],'checks':checks}
if __name__=='__main__':
    sys.path.insert(0,str(Path.cwd()));api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');model=api.load_model();report=build(model,api,api._surface_finishes())
    from shapely.geometry import Polygon
    from scipy.spatial import ConvexHull
    import numpy as np
    for obj in report['elements']:
        host=api._find(model,obj['tag'][6:-6])
        for part in obj['geometry']['parts']:
            points=np.asarray(part['mesh']['vertices']);assert abs(ConvexHull(points-points[0]).volume-api._polyhedron_volume(part))<1e-12
            for physical in host['geometry']['parts']:
                overlap=max(0,min(part['z1'],physical['z1'])-max(part['z0'],physical['z0']))
                assert Polygon(part['outline']).intersection(Polygon(physical['outline'])).area*overlap<1e-12,(host['tag'],physical['role'])
    sliding=report['sliding_clearance'];host=api._find(model,sliding['host']);board=next(x for x in report['elements'] if x['tag']=='SKIRT-'+host['tag']+'-front')
    from shapely.affinity import translate
    for leaf in host['geometry']['parts']:
        if leaf['role']!=sliding['role']:continue
        for displacement in (-sliding['travel'],0,sliding['travel']):
            polygon=translate(Polygon(leaf['outline']),xoff=displacement*sliding['axis'][0],yoff=displacement*sliding['axis'][1])
            assert all(polygon.intersection(Polygon(p['outline'])).area==0 for p in board['geometry']['parts'])
    (BASE/'🗑️generated/native-r92/cupboard-skirting-candidate.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] cupboard fronts independent native/SciPy/GEOS PASS',len(report['elements']),sum(len(x['geometry']['parts']) for x in report['elements']))
