"""🦴 Trace distinct equine, canine and ungulate limb volumes back to their own joint stations."""
from pathlib import Path
import json
import sys
from copy import deepcopy
t=Path(__file__).parent
def cropped(s,stations,id):
    r=deepcopy(s);r['id']=id
    if r['kind']=='loft':
        sec=s['sections'];r['sections']=[sec[j] for j in sorted(set(round(f*(len(sec)-1)) for f in stations))]
    return r
stations={
 'sheep':((0,.24,.59,1),(0,.2,.77,1),(0,.4,1)),
 'pig':((0,.28,.66,1),(0,.24,.71,1),(0,.39,.73,1)),
 'roe-deer':((0,.21,.53,.8,1),(0,.16,.72,1),(0,.28,.67,1)),
 'labrador-dog':((0,.18,.47,.74,1),(0,.2,.66,.82,1),(0,.29,.71,1)),
 'horse':((0,.2,.52,.73,1),(0,.25,.71,1),(0,.34,.68,1))
}
for slug in sys.argv[1:] or stations:
    cat=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog/*animals/'+slug+'/*.json'))
    for path in (cat,t/'volumes/animals'/f'{slug}.json'):
        raw=path.read_text();v=raw.index('{',raw.index('"volumes":'));d=json.JSONDecoder();vol,_=d.raw_decode(raw,v)
        fine=vol['50'];parts={s['id']:s for s in fine['body']};ske={s['id']:s for s in fine['structure']}
        heat={s['of']:s['region'] for s in fine['thermal'] if 'of' in s}
        edits=[];muscle,shaft,tip=stations[slug]
        for k in ('100','200','500'):
            start=raw.index('{',raw.index('"'+k+'"',v));scale,end=d.raw_decode(raw,start)
            body={s['id']:s for s in scale['body']};therm={s['id']:s for s in scale['thermal']};bones={s['id']:s for s in scale['structure']}
            def bodyPart(id,src,shape,add=False):
                if not add and id not in body:return
                body[id]=cropped(parts[src],shape,id)
                if src in heat:therm['th-'+id]=dict(id='th-'+id,of=id,region=heat[src])
            for side,letter in [('near','n'),('far','f')]:
                def sided(id):return side+'-'+id
                if slug in ('sheep','pig'):
                    bodyPart(sided('ham'),sided('ham'),muscle,True)
                    bodyPart(sided('shoulder'),sided('shoulder'),muscle,True)
                    if k!='500' and slug=='pig':
                        profile=(0,.15,.32,.53,.72,.87,1) if k=='100' else (0,.21,.44,.74,1)
                        for s in ('gaskin','forearm','hind-cannon','fore-cannon'):bodyPart(sided(s),sided(s),profile)
                if slug=='horse' and k!='500':
                    for limb in ('fore','hind'):
                        for s in ('cannon','pastern','hoof'):
                            src=sided(limb+'-'+s)
                            bodyPart(src,src,(0,.17,.33,.51,.68,.84,1) if k=='100' else (0,.21,.49,.77,1))
                if slug in ('sheep','pig'):
                    if k=='500':
                        bodyPart(sided('ear'),sided('ear'),tip)
                        for j,src in [('hind-upper','gaskin'),('fore-upper','forearm'),('hind-lower','hind-cannon'),('fore-lower','fore-cannon')]:
                            bodyPart(sided(j),sided(src),muscle if 'upper' in j else shaft)
                if k!='500':continue
                if slug=='roe-deer':
                    bodyPart(sided('ear'),sided('ear'),tip)
                    bodyPart(sided('antler'),sided('antler-beam'),(0,.25,.62,1))
                    for s in ('antler-brow','antler-back'):bodyPart(sided(s),sided(s),(0,.5,1),True)
                    for old,src in [('thigh','thigh'),('hind-leg','shank'),('fore-upper','humerus'),('fore-leg','forearm'),('hind-lower','hind-cannon'),('fore-lower','fore-cannon'),('hock','hock-point')]:
                        bodyPart(sided(old),sided(src),shaft if 'lower' in old else muscle)
                    if sided('carpus') in body:
                        del body[sided('carpus')];therm.pop('th-'+sided('carpus'),None)
                if slug=='labrador-dog':
                    for old,src in [('hind-thigh','thigh'),('hind-leg','shank'),('fore-upper','upper-arm'),('fore-leg','forearm'),('hind-foot','metatarsus'),('fore-foot','pastern')]:
                        bodyPart(old+'-'+letter,src+'-'+letter,shaft if 'foot' in old else muscle)
                    bodyPart('ear-'+letter,'ear-'+letter,tip,True)
                    for j in ('h','f'):bodyPart('paw-'+j+letter,'foot-'+j+letter,(0,.17,.48,.73,1),True)
                if slug=='horse':
                    for old,src in [('hind-upper','thigh'),('gaskin','gaskin'),('fore-upper','arm'),('forearm','forearm'),('fore-lower','fore-cannon'),('hind-lower','hind-cannon')]:
                        bodyPart(sided(old),sided(src),shaft if 'lower' in old else muscle)
                    for j in ('fore','hind'):
                        for anatomical in ('pastern','hoof'):
                            src=sided(j+'-'+anatomical);bodyPart(src,src,tip,True)
                boneNames={}
                if slug in ('sheep','pig'):
                    boneNames={'hind-thigh':'femur','hind-shank':'tibia','hind-metatarsus':'metatarsal' if slug=='sheep' else 'hind-metapodial-1','fore-scapula':'scapula','fore-humerus':'humerus','fore-forearm':'radius','fore-metacarpus':'metacarpal' if slug=='sheep' else 'fore-metapodial-1','hind-digits':'hind-digit-1','fore-digits':'fore-digit-1'}
                elif slug=='roe-deer':boneNames={'hind-digit':'hind-digit-1','fore-digit':'fore-digit-1'}
                elif slug=='horse':boneNames={j+'-'+q:j+'-'+w for j in ('fore','hind') for q,w in [('cannon','cannon-bone'),('pastern','long-pastern')]}
                for old,s in list(bones.items()):
                    if s.get('part')!='member' or not s.get('bone') or not (s['bone'].endswith(('h'+letter,'f'+letter)) or s['bone']=='ear-'+letter):continue
                    src=old
                    if old.startswith(side+'-'):src=sided(boneNames.get(old[len(side)+1:],old[len(side)+1:]))
                    if slug=='labrador-dog' and old.startswith('phalanges-'):src='phalanx-1-'+old.split('-')[1]+'-p'
                    if src in ske and 'kind' in ske[src]:bones[old]=cropped(ske[src],(0,.24,.7,1) if 'tibia' in src or 'radius' in src else (0,.45,1),old)
            scale.update(body=list(body.values()),thermal=list(therm.values()),structure=list(bones.values()))
            text={
              'sheep':('Wool at the two hams and shoulders follows the moving limbs. The short-haired cannons taper to the fetlocks and the phalanges sit inside the cloven feet.','Die Wolle an Keulen und Schultern folgt den Gliedmaßen. Die kurzhaarigen Röhrenbeine verjüngen sich zum Fesselgelenk. Die Zehenglieder liegen in den Klauen.'),
              'pig':('The hams follow each femur and the shoulders follow the scapulae. The short sloping cannon ends above each articulated pair of claws.','Die Schinken folgen den Oberschenkeln und die Schultern den Schulterblättern. Jedes kurze schräge Röhrenbein endet über seinem beweglichen Klauenpaar.'),
              'roe-deer':('The buck retains both slender antler beams and four distinct branch tines. The long tapered cannons, sloping pasterns and pointed claws follow their separate joints.','Der Bock behält beide schlanken Geweihstangen und vier getrennte Sprossen. Die langen Röhrenbeine, schrägen Fesseln und spitzen Schalen folgen ihren eigenen Gelenken.'),
              'labrador-dog':('The two folded ear flaps follow the pinnae. The long hind metatarsals and short front pasterns meet separate paws that follow the toes and their pads.','Beide Ohrlappen folgen den Ohrmuscheln. Die langen hinteren Mittelfüße und die kurzen vorderen Vordermittelfüße treffen auf eigenständig bewegliche Pfoten mit ihren Ballen.'),
              'horse':('The cannon, sloping pastern and hoof remain distinct on every leg. The heavy coronet and toe follow the terminal joint during loading and lift-off.','Röhrbein, schräge Fessel und Huf bleiben an jedem Bein unterscheidbar. Kronrand und Zehe folgen dem Endgelenk beim Auffußen und Abheben.')
            }[slug]
            if k=='500':
                scale['decisions'][0]={'en':text[0],'de':text[1]}
            elif slug in ('pig','sheep') and {'en':text[0],'de':text[1]} not in scale['decisions']:scale['decisions'].append({'en':text[0],'de':text[1]})
            if scale!=vol[k]:edits.append((start,end,json.dumps(scale,ensure_ascii=False,separators=(',',':'))))
        new=raw
        for i,j,s in reversed(edits):new=new[:i]+s+new[j:]
        if raw!=path.read_text():raise RuntimeError(path)
        path.write_text(new)

