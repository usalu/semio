"""🛠️ Author unpublished fixture stocks from explicit source-qualified fitting controls."""
import importlib,json
from copy import deepcopy
from fractions import Fraction
from pathlib import Path
ROOT=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file())
PREFIX='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.'
fits=importlib.import_module(PREFIX+'🧱️model.🧪️tests.🔧️fixture-fits');profile=importlib.import_module(PREFIX+'🧱️model.🧊️profiles.🐍️')

def qualify(obj,en,de):
    """📝 Keep inferred metric fitting explicit in both historical evidence views."""
    uncertainty=obj['historical_evidence']['uncertainty']
    for language,text in(('en',en),('de',de)):
        if text not in uncertainty[language]:uncertainty[language]+=' '+text
    property=obj.get('psets',{}).get('Pset_HistoricalEvidence',{}).get('ReconstructionUncertainty')
    if property is not None:property['value']=uncertainty['en']+' / '+uncertainty['de']

def aligned_pipe(source,row):
    """🪈 Transport the exact existing sleeve boundary through the canonical miter construction."""
    q=lambda x:Fraction(str(x));items={e['tag']:e for e in source['elements']};sleeve=[p for p in items[row['collector']]['geometry']['parts']if p['role']=='inlet_sleeve'];assert len(sleeve)==row['sectors'];assert all(p['outline'][2]==sleeve[(i+1)%len(sleeve)]['outline'][3]for i,p in enumerate(sleeve));points=[[q(v)for v in p]for p in row['spine']];reference=profile.hollow_polyline(points,row['diameter'],row['gauge'],row['sectors'],exact=True);ring=[[q(p['outline'][3][0]),q(p['outline'][3][1]),points[0][2]]for p in sleeve]
    area=lambda r:sum(a[0]*b[1]-b[0]*a[1]for a,b in zip(r,r[1:]+r[:1]));ring=ring if area(ring)*area(reference['rings'][0])>0 else list(reversed(ring));u=[reference['rings'][0][0][k]-points[0][k]for k in range(3)];v=[reference['rings'][0][row['sectors']//4][k]-points[0][k]for k in range(3)];a,b=max(((i,j)for i in range(3)for j in range(i+1,3)),key=lambda pair:abs(u[pair[0]]*v[pair[1]]-u[pair[1]]*v[pair[0]]));det=u[a]*v[b]-u[b]*v[a];coefficients=[]
    for p in ring:
        d=[p[k]-points[0][k]for k in range(3)];coefficients.append(((d[a]*v[b]-d[b]*v[a])/det,(u[a]*d[b]-u[b]*d[a])/det))
    outer=[]
    for center,stock in zip(points,reference['rings']):
        left=[stock[0][k]-center[k]for k in range(3)];right=[stock[row['sectors']//4][k]-center[k]for k in range(3)];outer.append([[center[k]+alpha*left[k]+beta*right[k]for k in range(3)]for alpha,beta in coefficients])
    ratio=(q(row['diameter'])/2-q(row['gauge']))/(q(row['diameter'])/2);inner=[[[center[k]+ratio*(p[k]-center[k])for k in range(3)]for p in ring]for center,ring in zip(points,outer)];metal=deepcopy(reference['metal'])
    for segment in range(len(points)-1):
        for i in range(row['sectors']):
            j=(i+1)%row['sectors'];metal[segment*row['sectors']+i]['mesh']['vertices']=[outer[segment][i],outer[segment][j],inner[segment][j],inner[segment][i],outer[segment+1][i],outer[segment+1][j],inner[segment+1][j],inner[segment+1][i]]
    def publish(value):
        """📤 Publish the shared exact vertices once at the native binary64 boundary."""
        if isinstance(value,Fraction):return float(value)
        if isinstance(value,list):return[publish(v)for v in value]
        if isinstance(value,dict):return{k:publish(v)for k,v in value.items()}
        return value
    return publish(metal)

def apply(model):
    """🔩 Preserve identities and sections while authoring the five coordinated visible assets."""
    source=deepcopy(model);c=fits.case();items={e['tag']:e for e in source['elements']};r=c['rain'];obj=items[r['tag']];assert obj['geometry']['axis']==r['spine'][-1][:2];assert obj['attributes']['diameter']['value']==r['diameter']and obj['attributes']['wall_thickness']['value']==r['gauge'];old=obj['geometry']['parts'];assert len(old)==3*r['sectors'];rows=aligned_pipe(source,r);obj['geometry']['parts']=[{**p,'material':old[i]['material'],'role':'pipe_wall_'+str(i//r['sectors'])}for i,p in enumerate(rows)];qualify(obj,'The visible upper pipe station follows the actual patio inlet at (2.80, 8.60) m; the lower No9 station is retained. The 80 mm diameter, 4 mm gauge and exposed tie route are inferred; the original inlet seal and concealed continuation remain unregistered.','Die sichtbare obere Rohrstation folgt dem tatsächlichen Patioeinlauf bei (2,80; 8,60) m; die untere Station an Nr.9 bleibt erhalten. Durchmesser 80 mm, Wandung 4 mm und sichtbare Verbindung sind abgeleitet; ursprüngliche Einlaufabdichtung und verdeckte Fortsetzung bleiben unregistriert.')
    l=c['lamp'];obj=items[l['tag']];delta=[0,0,l['mount_height']-obj['geometry']['mounting']['point'][2]];obj['geometry']['parts']=[fits.shifted(p,delta)for p in obj['geometry']['parts']];obj['geometry']['mounting']['point'][2]=l['mount_height'];qualify(obj,'The current 2.00 m mounting height is an inferred fit below the reconstructed ramp and guard. Column19, the mounting side, complete reflector family and aim are retained; the original metric mounting height remains unregistered.','Die aktuelle Montagehöhe 2,00 m ist eine abgeleitete Einpassung unter rekonstruiertem Rampenlauf und Brüstung. Stütze19, Befestigungsseite, vollständige Reflektorfamilie und Ausrichtung bleiben erhalten; die ursprüngliche metrische Montagehöhe bleibt unregistriert.')
    for row in c['taps']:
        obj=items[row['tag']];delta=[row['axis'][k]-obj['geometry']['axis'][k]for k in range(2)]+[0];obj['geometry']['parts']=[fits.shifted(p,delta)for p in obj['geometry']['parts']];obj['geometry']['axis']=deepcopy(row['axis']);qualify(obj,'The complete tap is fitted 1.5 mm rearward on its existing counter deck to clear the raised ceramic rim; flange height, section and all hollow/control parts are retained. This local fit is inferred within the unmeasured mounting registration.','Der vollständige Hahn ist auf seiner vorhandenen Arbeitsplatte um 1,5 mm nach hinten eingepasst und bleibt vom erhöhten Keramikrand frei; Flanschhöhe, Querschnitt sowie alle Hohl- und Bedienteile bleiben erhalten. Diese örtliche Einpassung ist innerhalb der nicht vermessenen Montageposition abgeleitet.')
    s=c['storage'];obj=items[s['tag']];q=lambda x:Fraction(str(x));delta=q(s['rear_x'])-min(q(v[0])for p in obj['geometry']['parts']for v in p['outline'])
    for part in obj['geometry']['parts']:
        for key in('outline','top_outline'):
            if key in part:part[key]=[[float(q(v[0])+delta),v[1]]for v in part[key]]
        if 'holes'in part:part['holes']=[[[float(q(v[0])+delta),v[1]]for v in ring]for ring in part['holes']]
    qualify(obj,'The rear cabinet plane is registered exactly to the declared pantry divider finish at X=3.04 m, removing accumulated coordinate arithmetic intrusion. All 38 cabinet parts and the source-qualified inferred dimensions are retained.','Die hintere Schrankebene ist exakt an der deklarierten Pantrytrennwandoberfläche bei X=3,04 m registriert; die durch aufgelaufene Koordinatenarithmetik entstandene Durchdringung entfällt. Alle 38 Schrankteile und die quellenqualifizierten abgeleiteten Maße bleiben erhalten.')
    return source
