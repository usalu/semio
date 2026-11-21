"""🧵 Independently reviewed hand calculations of the garment exteriors and their underlying skin allocations."""
from pathlib import Path
import sys,json,subprocess
ticket=Path(__file__).resolve().parent
root=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans'))
outfits={
 'parent-with-stroller':{'shoes':.17,'torso-clothing':.703},
 'person-cooking':{'jeans':.6109,'sneakers':.1424,'hoodie':.8207},
 'person-with-rollator':{'beret':.089,'shoes':.166,'coat':1.1569},
 'person-with-umbrella':{'shoes':.19,'coat':1.408},
 'person-with-white-cane':{'boots':.21,'jacket':.913},
 'tour-guide':{'shoes':.198,'torso-clothing':.847},
 'wheelchair-user':{'jeans':.716,'shoes':.174,'hoodie':.9989},
 'pregnant-person':{'leggings':.70,'ankle-boots':.224},
 'seated-reader':{'jeans':.76,'shoes':.154,'sweater':.902,'cup':.038},
 'seated-office-worker':{'trousers':.85,'shoes':.182,'shirt':1.05},
 'person-on-crutches':{'shorts':.456,'boot':.33,'sneaker':.105},
 'parent-carrying-toddler':{'scarf':.135,'coat-torso':1.12,'jeans':.75,'boots':.24,'toddler-boots':.095,'toddler-suit':.5102,'toddler-face':.045},
 'runner-with-blade-prosthesis':{'socket':.06,'arm-skin':.2668}
}
shares={'parent-carrying-toddler':{'toddler-suit':.69477,'toddler-face':.08523,'toddler-boots':.11,'jeans':.285,'boots':.085},'runner-with-blade-prosthesis':{'arm-skin':.15023,'singlet':.27977}}
for slug,c in outfits.items():
    p=next((root/slug).glob('*'));a=json.loads(p.read_text())['asset'];regions={q['id']:q for q in a['thermal']['regions']}
    for subject in set(q['body'].get('subject') for q in regions.values() if q['model']=='body'):
        group=[q for q in regions.values() if q['model']=='body' and q['body'].get('subject')==subject]
        skin=(a['thermal']['body'] if not subject else next(z for z in a['thermal']['passengers'] if z['id']==subject)['physiology'])['skinAreaM2']
        s=sum(shares.get(slug,{}).get(q['id'],q['body']['skinFraction']) for q in group)
        assert abs(s-1)<.001,(slug,subject,s)
        old=sum(q['areaM2'] for q in group);new=sum(c.get(q['id'],q['areaM2']) for q in group)
        print(slug,subject or 'subject',round(old/skin,3),round(new/skin,3))
    args=[]
    for k in dict.fromkeys([*c,*shares.get(slug,{})]):
        fields=[]
        if k in c:fields.append('areaM2='+str(c[k]))
        if k in shares.get(slug,{}):fields.append('skinFraction='+str(shares[slug][k]))
        args.append(k+':'+','.join(fields))
    if '--write' in sys.argv:subprocess.run([sys.executable,str(ticket/'r5-area-apply.py'),'humans/'+slug,*args],check=True,stdout=subprocess.DEVNULL)
