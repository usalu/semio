import re,os
from plan import *
def analyse():
    srcd={c:set() for c in CRATES}; tstd={c:set() for c in CRATES}; sj={c:False for c in CRATES}; kit={c:False for c in CRATES}
    for m,c in OWNER.items():
        for p,store in ((src(m),srcd),(tst(m),tstd)):
            if not os.path.exists(p): continue
            s=open(p).read()
            for d in re.findall(r'\bsemio_framework_pets_(\w+?)::',s):
                if d!=c: store[c].add(d)
            if store is tstd:
                if 'serde_json' in s: sj[c]=True
                if 'testkit' in s: kit[c]=True
    return srcd,tstd,sj,kit
if __name__=='__main__':
    srcd,tstd,sj,kit=analyse()
    for c in CRATES:
        print(c,'src',sorted(srcd[c]),'tests',sorted(tstd[c]-srcd[c]),'serde_json',sj[c],'testkit',kit[c],'planned',CRATES[c]['deps'])
