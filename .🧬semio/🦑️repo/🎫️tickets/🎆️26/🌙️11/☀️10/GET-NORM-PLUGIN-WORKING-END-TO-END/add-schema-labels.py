import json,glob,sys
sys.path.insert(0,'.')
import audit as A
L=lambda en,de:{"label":{"en":en,"de":de}}
def pa(en,de,step): return {**L(en,de),"widget":"stepper","unit":"Pa","displayUnit":"N/mm²","displayFactor":1e-06,"precision":0,"step":step}
SPEC={
'din18599':{'DhwSystem':{'dwellingCount':{**L('Number of dwellings','Anzahl der Wohneinheiten'),'widget':'stepper','precision':0,'step':1}}},
'din16798':{'UnoccupiedVentilation':{
 'operation':{**L('Operation outside occupancy','Betrieb außerhalb der Nutzungszeit'),'widget':'select','options':{
   'nonResidentialPurge':{'en':'Non-residential: purge before occupancy','de':'Nichtwohngebäude: Spülung vor Nutzungsbeginn'},
   'nonResidentialReduced':{'en':'Non-residential: reduced ventilation','de':'Nichtwohngebäude: reduzierte Lüftung'},
   'residentialReduced':{'en':'Residential: reduced ventilation','de':'Wohngebäude: reduzierte Lüftung'},
   'residentialShutdown':{'en':'Residential: ventilation switched off','de':'Wohngebäude: Lüftung abgeschaltet'}}},
 'assessedAreaM2':{**L('Assessed floor area','Bewertete Grundfläche'),'widget':'stepper','unit':'m²','precision':1,'step':1},
 'zoneVolumeM3':{**L('Zone volume','Zonenvolumen'),'widget':'stepper','unit':'m³','precision':1,'step':1},
 'minutesBeforeOccupancy':{**L('Purge duration before occupancy','Spüldauer vor Nutzungsbeginn'),'widget':'stepper','unit':'min','precision':0,'step':5},
 'residentialDesignRateLPerSM2':{**L('Residential design air flow rate per floor area','Auslegungs-Luftvolumenstrom je Grundfläche (Wohngebäude)'),'widget':'stepper','unit':'L/(s·m²)','precision':2,'step':0.05},
 'outdoorAirM3H':{**L('Outdoor air flow','Außenluftvolumenstrom'),'widget':'stepper','unit':'m³/h','precision':1,'step':10}}},
'en1991':{
 'ConstructionLoad':{'stage':L('Construction stage id','Bauzustand-Id')},
 'ConstructionStage':{'activity':L('Activity','Tätigkeit')},
 'CraneHoist':{'releaseDevice':L('Release device (grab or magnet)','Abwurfeinrichtung (Greifer oder Magnet)')},
 'RoofSurface':{'face':L('Building face','Gebäudeseite'),'shape':L('Roof shape','Dachform')},
 'SpecialVehicle':{'designation':L('Designation total/axle [kN]','Bezeichnung Gesamt/Achse [kN]')},
 'RailDynamic':{'maintenance':L('Track maintenance (careful or standard)','Gleisunterhaltung (sorgfältig oder normal)')}},
'en1992':{
 'BarPoint':{'bend':L('Bend shape','Biegeform')},
 'ReinforcementGrade':{'fYk':pa('Characteristic yield strength f_yk','Charakteristische Streckgrenze f_yk',5000000),'eS':L('Modulus of elasticity E_s','Elastizitätsmodul E_s'),'ductility':L('Ductility class','Duktilitätsklasse'),'k':L('Ratio k = (f_t/f_y)_k','Verhältniswert k = (f_t/f_y)_k'),'epsUk':L('Characteristic strain at maximum force ε_uk','Charakteristische Dehnung unter Höchstkraft ε_uk')},
 'ConcreteGrade':{'fCk':pa('Characteristic compressive cylinder strength f_ck','Charakteristische Zylinderdruckfestigkeit f_ck',1000000)},
 'PrestressSteel':{'fPk':pa('Characteristic tensile strength f_pk','Charakteristische Zugfestigkeit f_pk',10000000),'fP01k':pa('Characteristic 0,1 % proof strength f_p0,1k','Charakteristische 0,1 %-Dehngrenze f_p0,1k',10000000)}},
}
def fam_dir(fam): return [x for x in glob.glob(A.ROOT+'*') if x.endswith(fam)][0].split('/')[-1]
def en1998_spec():
    sd=glob.glob(A.ROOT+'*en1998/🏅️standards/*/🪆️subsets/*/🧬️schema')[0]
    lab={}
    for p in glob.glob(sd+'/🧬️mutations/*/🧬️schema/🔣️.json'):
        for k,v in json.load(open(p))['properties'].items():
            if k.startswith('new') and 'label' in v.get('x-semio-ui',{}): lab.setdefault(k[3].lower()+k[4:],v['x-semio-ui']['label'])
    spec={}
    for o in A.audit('🫨️en1998'):
        if o[2]=='LabelMissing':
            spec.setdefault(o[3],{})[o[4]]={'label':lab[o[4]]}
    return spec
SPEC['en1998']=en1998_spec()
def apply(fam,spec):
    sd=glob.glob(A.ROOT+fam_dir(fam)+'/🏅️standards/*/🪆️subsets/*/🧬️schema')[0]
    sp=sd+'/📸️snapshot/🔣️.json'
    s=open(sp).read(); d=json.loads(s)
    pretty=json.dumps(d,indent=2,ensure_ascii=False)+"\n"==s
    def upd(prop,ui):
        assert 'x-semio-ui' not in prop,prop
        new={}
        for k,v in prop.items():
            new[k]=v
            if k in('type','$ref'): new['x-semio-ui']=ui
        assert 'x-semio-ui' in new
        return new
    if pretty:
        for dn,props in spec.items():
            for pn,ui in props.items(): d['$defs'][dn]['properties'][pn]=upd(d['$defs'][dn]['properties'][pn],ui)
        open(sp,'w').write(json.dumps(d,indent=2,ensure_ascii=False)+"\n"); return
    lines=s.split('\n'); idx={}
    for i,l in enumerate(lines):
        if l.startswith('    "') and not l.startswith('     '):
            t=l.strip().rstrip(','); k,v=t.split(':',1)
            try: idx[json.loads(k)]=i
            except Exception: pass
    for dn,props in spec.items():
        l=lines[idx[dn]]; comma=',' if l.rstrip().endswith(',') else ''
        obj=json.loads(l.strip().rstrip(',').split(':',1)[1])
        for pn,ui in props.items(): obj['properties'][pn]=upd(obj['properties'][pn],ui)
        lines[idx[dn]]=l[:len(l)-len(l.lstrip())]+json.dumps(dn)+':'+json.dumps(obj,separators=(',',':'),ensure_ascii=False)+comma
    open(sp,'w').write('\n'.join(lines))
for fam,spec in SPEC.items(): apply(fam,spec); print('applied',fam)
