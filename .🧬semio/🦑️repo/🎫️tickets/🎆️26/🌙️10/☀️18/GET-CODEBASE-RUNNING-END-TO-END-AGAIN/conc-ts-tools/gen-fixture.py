#!/usr/bin/env python3
"""Generates staged/🧩️units/🧪️tests/🔣️.json: hand-authored planner cases with hand-written expectations (language-agnostic). The oracle and the engine must both reproduce them."""
import json,os
S=os.path.join(os.path.dirname(os.path.abspath(__file__)),'staged','🧩️units')
doc=json.load(open(os.path.join(S,'🔣️.json')))
PK=doc['seeds']['markers'][0]; TESTS=doc['seeds']['isolate'][0]; AMB='🌿️ambient'
doc['cycles']={'collapseFiles':10,'refineRounds':2}; doc['coarsen']={'minimumFiles':0}
def case(name,why,files,edges,expect,manifests=(),json_=(),override=None):
    c={'name':name,'description':why,'files':files,'manifests':list(manifests),'edges':edges,'json':[list(j) for j in json_],'expect':expect}
    if override: c['override']=override
    return c
pk=lambda m:m+'/'+PK+'/ts/index.ts'
cases=[
 case('acyclic-modules','Two modules joined by one import: two units, the importer depends on the importer\'s target.',
  [pk('app'),'app/src/a.ts',pk('lib'),'lib/core/c.ts'],[['app/src/a.ts','lib/core/c.ts']],
  {'units':{'dir:app':{'files':[pk('app'),'app/src/a.ts'],'dependencies':['dir:lib'],'json':[]},'dir:lib':{'files':[pk('lib'),'lib/core/c.ts'],'dependencies':[],'json':[]}},'order':['dir:lib','dir:app'],'ambient':[]}),
 case('isolated-tests','Files below an isolated directory form their own unit per root; production never depends on it.',
  [pk('lib'),'lib/core/c.ts','lib/'+TESTS+'/c.ts'],[['lib/'+TESTS+'/c.ts','lib/core/c.ts']],
  {'units':{'dir:lib':{'files':[pk('lib'),'lib/core/c.ts'],'dependencies':[],'json':[]},'dir:lib#'+TESTS:{'files':['lib/'+TESTS+'/c.ts'],'dependencies':['dir:lib'],'json':[]}},'order':['dir:lib','dir:lib#'+TESTS],'ambient':[]}),
 case('small-cycle-collapses','Two modules whose files form no file cycle but whose modules import each other: one collapsed unit.',
  [pk('a'),'a/x.ts','a/w.ts',pk('b'),'b/y.ts','b/z.ts'],[['a/x.ts','b/y.ts'],['b/z.ts','a/w.ts']],
  {'units':{'group:a':{'files':['a/w.ts','a/x.ts',pk('a'),'b/y.ts','b/z.ts',pk('b')],'dependencies':[],'json':[]}},'order':['group:a'],'ambient':[]}),
 case('large-cycle-refines','A cyclic core above the collapse limit is split along its directories until the unit graph is acyclic.',
  [pk('a'),'a/p/1.ts','a/q/2.ts',pk('b'),'b/r/3.ts','b/s/4.ts'],[['a/p/1.ts','b/r/3.ts'],['b/s/4.ts','a/q/2.ts']],
  {'units':{
    'dir:a/'+PK:{'files':[pk('a')],'dependencies':[],'json':[]},'dir:a/p':{'files':['a/p/1.ts'],'dependencies':['dir:b/r'],'json':[]},'dir:a/q':{'files':['a/q/2.ts'],'dependencies':[],'json':[]},
    'dir:b/'+PK:{'files':[pk('b')],'dependencies':[],'json':[]},'dir:b/r':{'files':['b/r/3.ts'],'dependencies':[],'json':[]},'dir:b/s':{'files':['b/s/4.ts'],'dependencies':['dir:a/q'],'json':[]}},
   'order':['dir:a/q','dir:a/'+PK,'dir:b/r','dir:a/p','dir:b/s','dir:b/'+PK],'ambient':[]},
  override={'cycles':{'collapseFiles':2,'refineRounds':2}}),
 case('file-cycle-collapses','Two single-file units that import each other cannot be refined: collapsed even above the limit.',
  ['a/x.ts','b/y.ts'],[['a/x.ts','b/y.ts'],['b/y.ts','a/x.ts']],
  {'units':{'group:a':{'files':['a/x.ts','b/y.ts'],'dependencies':[],'json':[]}},'order':['group:a'],'ambient':[]},
  manifests=['a','b'],override={'cycles':{'collapseFiles':1,'refineRounds':2}}),
 case('ambient-and-json','Global declaration files belong to no unit; JSON imported across units is an input of the importer.',
  [pk('lib'),'lib/'+AMB+'/g.d.ts','lib/core/c.ts','app/a.ts'],[['app/a.ts','lib/core/c.ts']],
  {'units':{'dir:app':{'files':['app/a.ts'],'dependencies':['dir:lib'],'json':['lib/core/data.json']},'dir:lib':{'files':[pk('lib'),'lib/core/c.ts'],'dependencies':[],'json':[]}},'order':['dir:lib','dir:app'],'ambient':['lib/'+AMB+'/g.d.ts']},
  manifests=['app'],json_=[('app/a.ts','lib/core/data.json')]),
 case('coarsen-merges-siblings','Tiny sibling units merge into one cluster when the unit graph stays acyclic.',
  ['pk/a/1.ts','pk/b/2.ts','pk/c/3.ts','pk/d/4.ts'],[['pk/a/1.ts','pk/c/3.ts'],['pk/b/2.ts','pk/c/3.ts'],['pk/d/4.ts','pk/a/1.ts']],
  {'units':{'group:cluster/pk/a':{'files':['pk/a/1.ts','pk/b/2.ts','pk/c/3.ts'],'dependencies':[],'json':[]},'dir:pk/d':{'files':['pk/d/4.ts'],'dependencies':['group:cluster/pk/a'],'json':[]}},'order':['group:cluster/pk/a','dir:pk/d'],'ambient':[]},
  manifests=['pk/a','pk/b','pk/c','pk/d'],override={'coarsen':{'minimumFiles':3}}),
 case('coarsen-refuses-cycle','A merge that would close a cycle through a third unit is refused.',
  ['pk/a/1.ts','pk/b/2.ts','other/x/1.ts','other/x/2.ts','other/x/3.ts'],[['pk/a/1.ts','other/x/1.ts'],['other/x/2.ts','pk/b/2.ts']],
  {'units':{'dir:pk/a':{'files':['pk/a/1.ts'],'dependencies':['dir:other/x'],'json':[]},'dir:pk/b':{'files':['pk/b/2.ts'],'dependencies':[],'json':[]},'dir:other/x':{'files':['other/x/1.ts','other/x/2.ts','other/x/3.ts'],'dependencies':['dir:pk/b'],'json':[]}},'order':['dir:pk/b','dir:other/x','dir:pk/a'],'ambient':[]},
  manifests=['pk/a','pk/b','other/x'],override={'coarsen':{'minimumFiles':3}}),
 case('same-stem-sources-split','`x.ts` and `x.tsx` would write one declaration path: the `.tsx` becomes its own unit.',
  [pk('lib'),'lib/e/x.ts','lib/e/x.tsx','lib/e/y.ts'],[['lib/e/y.ts','lib/e/x.ts']],
  {'units':{'dir:lib':{'files':[pk('lib'),'lib/e/x.ts','lib/e/y.ts'],'dependencies':[],'json':[]},'file:lib/e/x.tsx':{'files':['lib/e/x.tsx'],'dependencies':[],'json':[]}},'order':['dir:lib','file:lib/e/x.tsx'],'ambient':[]}),
 case('collision-inside-cycle-refines','A cyclic core that would hold `x.ts` and `x.tsx` together is refined instead of collapsed, although it is below the collapse limit; the part that stays cyclic collapses without the clash.',
  [pk('a'),'a/p/x.ts','a/p/x.tsx','a/q/w.ts','a/q/y.ts',pk('b'),'b/r/z.ts'],[['a/q/w.ts','a/p/x.tsx'],['a/p/x.tsx','b/r/z.ts'],['b/r/z.ts','a/q/y.ts']],
  {'units':{
    'dir:a/'+PK:{'files':[pk('a')],'dependencies':[],'json':[]},'dir:a/p':{'files':['a/p/x.ts'],'dependencies':[],'json':[]},'dir:b/'+PK:{'files':[pk('b')],'dependencies':[],'json':[]},
    'group:a/q':{'files':['a/p/x.tsx','a/q/w.ts','a/q/y.ts','b/r/z.ts'],'dependencies':[],'json':[]}},
   'order':['dir:a/p','dir:a/'+PK,'dir:b/'+PK,'group:a/q'],'ambient':[]}),
 case('glob-root','A directory matching a root glob is a unit root without a manifest.',
  ['✏️s/🔌️plugins/p1/a.ts','✏️s/🔌️plugins/p2/b.ts','✏️s/other.ts'],[['✏️s/🔌️plugins/p1/a.ts','✏️s/🔌️plugins/p2/b.ts']],
  {'units':{'dir:✏️s':{'files':['✏️s/other.ts'],'dependencies':[],'json':[]},'dir:✏️s/🔌️plugins/p1':{'files':['✏️s/🔌️plugins/p1/a.ts'],'dependencies':['dir:✏️s/🔌️plugins/p2'],'json':[]},'dir:✏️s/🔌️plugins/p2':{'files':['✏️s/🔌️plugins/p2/b.ts'],'dependencies':[],'json':[]}},'order':['dir:✏️s','dir:✏️s/🔌️plugins/p2','dir:✏️s/🔌️plugins/p1'],'ambient':[]},
  manifests=['✏️s']),
]
json.dump({'version':1,'description':'Language-agnostic planner fixture: engine and oracle must produce exactly the expected units, dependencies, JSON inputs, ambient files and order.','document':doc,'cases':cases},open(os.path.join(S,'🧪️tests','🔣️.json'),'w'),ensure_ascii=False,indent=1)
print(len(cases),'cases')
