import os,sys,json
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from lib import *
def setdof(bones,spec):
    for b,d in spec.items():
        if b in bones: bones[b].update(d)
def run(aid,spec):
    doc=load(aid); a=doc['asset']; n=0
    for p in a['poses']:
        if p['id'] in spec: setdof(p['bones'],spec[p['id']]); n+=1
    for c in a['clips']:
        for k in c['keys']:
            if k.get('pose') in spec and 'bones' in k:
                setdof(k['bones'],spec[k['pose']]); n+=1
    save(aid,doc); print(aid,'edited',n)
run('person-with-white-cane',{
 'constant-contact-stance':{'arm-r':{'abductionDeg':8},'forearm-r':{'abductionDeg':10},'cane':{'abductionDeg':-4}},
 'diagonal-technique':{'arm-r':{'abductionDeg':25},'forearm-r':{'abductionDeg':20},'cane':{'abductionDeg':14}}})
run('person-cooking',{'standing-by':{'neck':{'twistDeg':-10},'head':{'twistDeg':-25}}})
