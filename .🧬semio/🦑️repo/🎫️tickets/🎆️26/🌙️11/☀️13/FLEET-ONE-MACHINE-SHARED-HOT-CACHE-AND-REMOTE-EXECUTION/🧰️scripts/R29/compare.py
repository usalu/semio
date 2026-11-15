import sys
sys.path.insert(0,'/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R29')
from roots2 import *
before,after=sys.argv[1],sys.argv[2]
fo,U,rows=roots(after,21)
for f in sorted(U,key=lambda f:-len(fo[f])):print(len(fo[f]),f.replace('🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/','LIB/'))
b=load(after);a=load(before)
print('errors',a['err'],b['err'],'slots',sum(len(s) for s in a['own'].values()),'->',sum(len(s) for s in b['own'].values()))
print('owners losing files from key set',sum(1 for r in a['sets'] if not a['sets'][r]<=b['sets'][r]),'strictly larger',sum(1 for r in a['sets'] if len(b['sets'][r])>len(a['sets'][r])),'losing packages',sum(1 for r in a['pk'] if not a['pk'][r]<=b['pk'][r]))
