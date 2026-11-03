import os,sys,json,importlib
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from lib import *
HERE=os.path.dirname(os.path.abspath(__file__))
def apply(aid,build):
    doc=load(aid); a=doc['asset']
    cache=os.path.join(HERE,'orig',aid+'.json')
    if not os.path.exists(cache):
        orig={k:r['decisions'] for k,r in a['representations'].items() if r['status']=='authored'}
        json.dump(orig,open(cache,'w',encoding='utf-8'),ensure_ascii=False,indent=1)
    orig=json.load(open(cache,encoding='utf-8'))
    for sc,rep in a['representations'].items():
        if rep['status']!='authored': continue
        front,plan,notes=build(int(sc))
        geo=rep['geometry']
        rep['geometry']={'elevation':geo['elevation'],'front':front,'plan':plan}
        dec=json.loads(json.dumps(orig[sc]))
        for key,(en,de) in notes.items():
            dec[key]['en']=dec[key]['en'].rstrip()+' '+en
            dec[key]['de']=dec[key]['de'].rstrip()+' '+de
        rep['decisions']=dec
    a['presentation'].pop('viewpoints',None)
    save(aid,doc)
    print('applied',aid,{k:(len(r['geometry']['front']),len(r['geometry']['plan'])) for k,r in a['representations'].items() if r['status']=='authored'})
