"""🗒️ Keeps the English and German structural descriptions consistent with individual open rib courses."""
import json,re
from pathlib import Path
t=Path(__file__).resolve().parent
hum=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans'))
rep={
 'the rib cage as one egg that is narrow at the first rib and widest at the eighth':'the second and seventh rib pairs joined to the sternum',
 'der Brustkorb als ein Ei, schmal an der ersten und am breitesten an der achten Rippe':'das zweite und siebte Rippenpaar mit Anschluss ans Brustbein',
 'Brustkorb als ein Fass':'drei offene Rippenpaare mit Brustbein',
 'Brustkorb als eine Form':'zwei offene Rippenpaare mit Brustbein',
 'Brustkorb als Loft':'drei offene Rippenpaare mit Brustbein',
 "the toddler's spine, rib hoop, girdles":"the toddler's spine, six paired rib courses, girdles",
 'Wirbelsäule, Rippenring, Gürtel, Schädel und Gliedknochen des Kleinkinds':'Wirbelsäule, sechs Rippenpaare, Gürtel, Schädel und Gliedknochen des Kleinkinds',
 'twelve pairs of ribs with a costal margin and two floating pairs onto a curved sternum':'twelve rib pairs, a curved sternum, a costal margin and two floating pairs',
 'sieben nach vorn um 24 Grad abfallende Rippenreifen an einem gebogenen Brustbein':'zwölf Rippenpaare, ein gebogenes Brustbein, der Rippenbogen und zwei freie Paare',
}
for p in sorted(hum.glob('*/*')):
    old=p.read_text();text=old;asset=json.loads(old)['asset']
    for k,v in asset['volumes'].items():
        if not isinstance(v.get('structure'),list):continue
        count=len(v['structure'])
        for dec in v['decisions']:
            for lang in ['en','de']:
                s=dec[lang];after=s
                if not any(x in s.lower() for x in ['structure','skeleton','tragwerk','skelett']):continue
                for x,y in rep.items():after=after.replace(x,y)
                after=re.sub(r'(?i)(structure|tragwerk) \(\d+ (solids|Körper)\)',lambda m:f'{m[1]} ({count} {m[2]})',after)
                if after!=s:text=text.replace(json.dumps(s,ensure_ascii=False),json.dumps(after,ensure_ascii=False),1)
    if text==old:continue
    doc=json.loads(text)['asset'];assert p.read_text()==old;p.write_text(text)
    snap=t/'volumes'/'humans'/(p.parent.name+'.json');s=snap.read_text();draft=json.loads(s)
    for k,v in doc['volumes'].items():draft['volumes'][k]['decisions']=v['decisions']
    assert snap.read_text()==s;snap.write_text(json.dumps(draft,ensure_ascii=False,separators=(',',':'))+'\n')
    print(p.parent.name)
