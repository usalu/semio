"""🧭 Checks the authored rotor, shaft diameters, risers, shell lengths and flow passages at all four scales."""
from pathlib import Path
import json,sys,math
t=Path(__file__).resolve().parent;r=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/🔌️appliances'))
for slug in (sys.argv[1:] or ['duct-fan','decentralised-hrv-fan','circulation-pump']):
    a=json.loads(next((r/slug).iterdir()).read_bytes())['asset']
    for d,v in a['volumes'].items():
        by={q['id']:q for q in v['body']};real=lambda q:q.get('repeat',{}).get('count',1)
        if slug=='circulation-pump':
            assert by['terminal-box']['size']==[.02,.06,.066],(slug,d,'enclosure')
            for key in ['terminal-neck','terminal-lid','window','pointer','neck-l','neck-r','stub-l','stub-r']:assert key in by,(slug,d,key)
            assert by['pointer']['bone']=='rotor' and real(by['impeller-vane'])==6
            assert by['drive-shaft']['material']=='shaft-ceramic' and by['thrust-washer']['material']=='thrust-carbon'
            assert all(by[key]['bone']=='motor' and by[key]['material']=='shaft-ceramic' for key in ['radial-front','radial-rear'])
            assert by['wet-rotor-can']['material']=='rotor-stainless'
            for id in ['neck-l','neck-r','stub-l','stub-r']:assert abs(min(p[1] for p in by[id]['profile'])-.0125)<1e-8
            vane=by['impeller-vane']['profile'];inner=min(vane,key=lambda p:math.hypot(*p));outer=max(vane,key=lambda p:math.hypot(*p))
            assert math.atan2(outer[1],outer[0])-math.atan2(inner[1],inner[0])<-.5
            assert math.isclose(max(p[1] for p in by['impeller-eye']['profile']),.031)
            assert len(by['suction-divider']['triangles'])>=384
            for key in ['stub-l','stub-r']:
                q=by[key];assert abs(min(x[1] for x in q['profile'])-.0125)<1e-6
        else:
            hrv=slug.startswith('decent')
            assert by['bearing']['bone']=='ring' and by['shaft']['bone']=='rotor'
            rshaft=max(p[1] for p in by['shaft']['profile']);rbore=min(p[1] for p in by['bearing']['profile'])
            assert math.isclose(rbore-rshaft,.00005)
            blade=[q for q in v['body'] if q['id'].startswith('blade')]
            assert sum(real(q) for q in blade)==5,(slug,d,'blade count')
            assert all(q['kind']=='loft' and q['bone']=='rotor' for q in blade)
            ring=by['ring']['profile'];assert abs(max(q[0] for q in ring)-min(q[0] for q in ring)-(.075 if hrv else .03))<1e-6,(slug,d,'ring length')
            strut=by['strut'];assert abs(min(strut['size'])-.008)<1e-7 and real(strut)==(3 if hrv else 4),(slug,d,'mount')
            assert by['hub']['bone']=='rotor' and abs(max(q[1] for q in by['hub']['profile'])-(.028 if hrv else .034))<1e-6
            if hrv:
                sleeve=by['sleeve'];assert abs(sleeve['to']-sleeve['from']-.22)<1e-6
                core=by['core'];assert len(core['holes'])==37 and abs(core['to']-core['from']-.1)<1e-6
            else:assert abs(max(q[0] for q in by['spigot']['profile'])-min(q[0] for q in by['spigot']['profile'])-.06)<1e-6
        print(slug,d,'pass')
