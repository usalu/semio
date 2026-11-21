"""🛠️ Authors the circulation pump's open pipe tails, split volute, wet rotor, impeller, viewing window and terminal enclosure at physical size."""
import json,math,copy
from pathlib import Path
t=Path(__file__).resolve().parent;r=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/🔌️appliances/circulation-pump/*'))
out=t/'🗑️generated'/'rotor';out.mkdir(parents=True,exist_ok=True)
def pt(x,y):return [round(x,6),round(y,6)]
def circle(radius,n,reverse=False):return [pt(radius*math.cos(i*math.tau/n),radius*math.sin(i*math.tau/n)) for i in (range(n-1,-1,-1) if reverse else range(n))]
def revolve(id,bone,mat,profile,origin=[0,0,0],tone=.45,role='body'):
    return {'id':id,'kind':'revolve','bone':bone,'material':mat,'role':role,'tone':tone,'profile':profile,'axis':[0,0,1],'origin':origin}
def extrude(id,bone,mat,profile,lo,hi,tone=.45):
    return {'id':id,'kind':'extrude','bone':bone,'material':mat,'role':'body','tone':tone,'plane':'xy','profile':profile,'from':lo,'to':hi}
def author(a):
    fine=copy.deepcopy(a['volumes']['50']);b={q['id']:q for q in fine['body']};terms={q.get('of'):q for q in fine['thermal'] if 'of'in q}
    for d,n,arc in [(50,48,12),(100,36,9),(200,24,6),(500,16,4)]:
        v=a['volumes'][str(d)];body=[];thermal=[];local={};struct=[]
        def put(q):
            body.append(q);local[q['id']]=q
        def sections(q,stations):return {**q,'sections':[q['sections'][i] for i in stations]}
        def reduced(q,stations):return {**q,'profile':[q['profile'][i] for i in stations]}
        origin=[.1055,-.003,0]
        outer=[[.0475,.048],[.057,.048],[.061,.0468],[.064,.044],[.066,.0395],[.067,.033]]
        if d==100:outer.pop(3)
        elif d==200:outer=[outer[i] for i in [0,1,3,5]]
        elif d==500:outer=[outer[i] for i in [0,1,5]]
        back=outer+[[.067,0],[.062,0],[.062,.032],[.057,.043],[.0475,.043],outer[0]]
        put(revolve('casing','housing','cast-iron',back,origin,.34))
        front=[[.012,.0412],[.012,.0462],[.014,.048],[.0225,.048],[.0225,.043],[.014,.043],[.012,.0412]]
        if d>=200:front=[front[i] for i in [0,1,2,3,4,6]]
        put(revolve('casing-front','housing','cast-iron',front,origin,.34))
        for id,lo,hi in [('crown',math.asin(.016/.043),math.pi-math.asin(.016/.043)),('sump',math.pi+math.asin(.01/.043),math.tau-math.asin(.01/.043))]:
            def arcpts(rad):
                return [pt(origin[0]+rad*math.cos(lo+i*(hi-lo)/arc),origin[1]+rad*math.sin(lo+i*(hi-lo)/arc)) for i in range(arc+1)]
            put(extrude('casing-'+id,'housing','cast-iron',arcpts(.048)+arcpts(.043)[::-1],.0225,.0475,.34))
        for id in ['flange','motor-flange']:
            q=b[id];hole=.0185 if id=='motor-flange' else .0412
            if id=='flange':hole=.035
            cx=.104 if id=='flange' else 0
            outer=q['profile'];step=1 if d==50 else 2 if d<=200 else 3
            put({**q,'profile':outer[::step],'holes':[[[round(cx+x,6),y] for x,y in circle(hole,n,True)]]})
        for side in 'lr':
            q=b['neck-'+side];old=q['profile'];outer=old[1:-1];outer[0]=[.034,outer[0][1]]
            if d>=200:outer=[outer[i] for i in [0,1,4,5,6]]
            if d==500:outer=[outer[i] for i in [0,1,2,4]]
            put({**q,'profile':[[.034,.0125],*outer,[.09,.0125],[.034,.0125]]})
            q=b['union-'+side];put({**q,'holes':[circle(.0228,n,True)]})
            put(b['stub-'+side])
        q=b['motor-head'];outer=q['profile'][1:-1]
        if d==100:outer=[outer[i] for i in [0,2,4,5]]
        elif d>=200:outer=[outer[i] for i in [0,4,5]]
        put({**q,'profile':[[outer[0][0],.0185],*outer,[.004,.0185],[outer[0][0],.0185]]})
        if d==50:put(b['screw'])
        q=b['bezel'];put(q if d<200 else reduced(q,[0,1,3,4,5]))
        put(b['window'])
        if d<=100:put(b['dial'])
        put(b['hub']);put(b['pointer'])
        if d<=100:put(b['tail'])
        for id in ['terminal-neck','terminal-box','terminal-lid']:
            q={**b[id]}
            if d==100:q['cornerRadius']=q['cornerRadius']*.75
            elif d==200:q['cornerRadius']=q['cornerRadius']*.4
            elif d==500:q.pop('cornerRadius',None)
            put(q)
        q=b['gland'];put(q if d==50 else reduced(q,[0,1,2,3,4,len(q['profile'])-2,len(q['profile'])-1]) if d==100 else reduced(q,[0,1,3,len(q['profile'])-2,len(q['profile'])-1]))
        if d==50:put(b['led'])
        put(revolve('drive-shaft','rotor','steel',[[-.06,0],[-.06,.004],[.024,.004],[.024,0]],tone=.62))
        barrel=[[-.050,0],[-.050,.016],[-.049,.017],[-.009,.017],[-.008,.016],[-.008,0]]
        put(revolve('rotor-drum','rotor','steel',barrel if d<200 else [[-.05,0],[-.05,.017],[-.008,.017],[-.008,0]],tone=.43))
        put(revolve('impeller-back','rotor','polyamide',[[.020,0],[.020,.031],[.0225,.031],[.0225,0]],tone=.42))
        vane=[]
        stations={50:8,100:6,200:4,500:3}[d]
        for side in [1,-1]:
            for i in (range(stations+1) if side==1 else range(stations,-1,-1)):
                u=i/stations;radius=.010+.020*u
                angle=math.radians(10+45*u**1.35)+side*.001/radius
                vane.append(pt(radius*math.cos(angle),radius*math.sin(angle)))
        q=extrude('impeller-vane','rotor','polyamide',vane,.0225,.030,.47);q['repeat']={'count':6,'rotationAxis':[0,0,1],'rotationRad':round(math.tau/6,8)};put(q)
        cover=[[.03,.010],[.03,.031],[.032,.031],[.032,.014],[.034,.012],[.034,.010],[.03,.010]]
        put(revolve('impeller-eye','rotor','polyamide',cover,tone=.48))
        for ref in fine['thermal']:
            key=ref.get('of')
            if ref.get('tags'):thermal.append(ref);continue
            if key in ['casing','flange','motor-head','motor-flange','neck-l','neck-r','union-l','union-r']:
                metal=b[key]
                if key=='casing':metal=sections(metal,{50:[0,1,2,3,4,5,6],100:[0,1,2,4,6],200:[0,1,2,6],500:[0,1,2,6]}[d])
                thermal.append({**metal,**{k:v for k,v in ref.items() if k!='of'}})
            elif key in local:thermal.append(ref)
            elif key is None and ref['id'].startswith('th-stub-'):thermal.append(ref)
        for q in v['structure']:
            if q['id']=='member-terminal':q={'id':'member-terminal','of':'terminal-box','member':'terminal','part':'member'}
            if q['id']=='member-rotor':q={'id':'member-rotor','of':'drive-shaft','member':'rotor','part':'member'}
            if q.get('of') and q['of'] not in local:continue
            struct.append(q)
        for id in ['impeller-back','impeller-vane','impeller-eye']:
            struct.append({'id':'member-'+id,'of':id,'member':'rotor','part':'member'})
        v.update(body=body,thermal=thermal,structure=struct)
        v['decisions']=[{'en':'The water path enters through 25 mm open pipe tails, hollow union nuts and cast necks. The pierced casing flanks meet the ports. Six backward-curved impeller vanes lie between a back plate and a shroud with an open eye. The shaft, impeller and indicator turn together.','de':'Der Wasserweg verläuft durch offene 25-mm-Rohrenden, hohle Überwurfmuttern und Gussstutzen. Die durchbrochenen Gehäuseflanken schließen an die Stutzen an. Sechs rückwärts gekrümmte Laufradschaufeln liegen zwischen Rückplatte und Deckscheibe mit offenem Saugauge. Welle, Laufrad und Anzeige drehen gemeinsam.'},{'en':f'At 1:{d} the motor can, shaft, window, 20 mm terminal housing, 16 mm saddle and 14 mm lid keep their measured sections. The external heat field follows the room-side metal surfaces.','de':f'Bei 1:{d} behalten Motorgehäuse, Welle, Sichtfenster, 20-mm-Klemmenkasten, 16-mm-Sattel und 14-mm-Deckel ihre Querschnitte. Die äußeren Wärmeflächen folgen den raumseitigen Metallflächen.'}]
doc=json.loads(r.read_bytes());author(doc['asset']);(out/'circulation-pump.json').write_text(json.dumps(doc,ensure_ascii=False))
