"""⚙️ Seats both radial bearings, the carbon thrust washer and the canned wet rotor in the circulation pump."""
from pathlib import Path
import json
t=Path(__file__).resolve().parent;p=t/'🗑️generated'/'rotor'/'circulation-pump.json';doc=json.loads(p.read_bytes());a=doc['asset']
for id,en,de,k,rho,cp,em,absorb in [
    ('shaft-ceramic','Technical ceramic shaft and radial bushes','Technische Keramik für Welle und Radiallager',25,3850,800,.85,.42),
    ('rotor-stainless','Stainless wet rotor can and bearing plates','Edelstahl für Spaltrohr und Lagerplatten',16,7900,500,.26,.44),
    ('thrust-carbon','Carbon thrust washer','Kohle-Axiallager',14,1800,710,.86,.88)]:
    a['materials'].append({'id':id,'title':{'en':en,'de':de},'emissivity':em,'solarAbsorptivity':absorb,'densityKgM3':rho,'conductivityWmK':k,'specificHeatJKgK':cp,'source':'authored-estimate','reference':'incropera-2018'})
for v in a['volumes'].values():
    b={q['id']:q for q in v['body']}
    shaft=b['drive-shaft'];shaft['material']='shaft-ceramic';shaft['profile'][0][0]=shaft['profile'][1][0]=-.062
    drum=b['rotor-drum'];drum['material']='rotor-stainless';prof=drum['profile'];prof[0][1]=prof[-1][1]=.004;prof.append(prof[0])
    if 'dial' in b:
        p0=b['dial']['profile'];p0[0][1]=p0[-1][1]=.0042;p0.append(p0[0])
    def add(k,m,lo,hi,ri,ro):
        v['body'].append({'id':k,'kind':'revolve','bone':'motor','material':m,'role':'detail','tone':.48,'profile':[[lo,ri],[lo,ro],[hi,ro],[hi,ri],[lo,ri]],'axis':[0,0,1],'origin':[0,0,0]})
    add('wet-rotor-can','rotor-stainless',-.052,.003,.0178,.0185)
    for side,lo,hi in [('front',-.0595,-.052),('rear',.003,.0105)]:
        add('radial-'+side,'shaft-ceramic',lo,hi,.00405,.009)
        add('seat-'+side,'rotor-stainless',lo,hi,.009,.0185)
    add('thrust-washer','thrust-carbon',-.00795,-.006,.00415,.010)
    add('thrust-retainer','rotor-stainless',-.006,.003,.0042,.010)
    for q in v['structure']:
        if q['id'] in ['joint-rotor-front','joint-rotor-rear']:
            side=q['id'].split('-')[-1];q.clear();q.update(id='joint-rotor-'+side,of='radial-'+side,member='rotor',part='joint')
    v['structure'].append({'id':'joint-thrust','of':'thrust-washer','member':'rotor','part':'joint'})
for d in ['50','100','200']:
    a['representations'][d]['geometry']['front'].append({'id':'thrust-washer-section','bone':'motor','kind':'rect','role':'detail','x':-.00795,'y':.05515,'w':.00195,'h':.00585,'z':4.7,'tone':.24,'material':'thrust-carbon','filled':True,'penMm':.2})
p.write_text(json.dumps(doc,ensure_ascii=False))
