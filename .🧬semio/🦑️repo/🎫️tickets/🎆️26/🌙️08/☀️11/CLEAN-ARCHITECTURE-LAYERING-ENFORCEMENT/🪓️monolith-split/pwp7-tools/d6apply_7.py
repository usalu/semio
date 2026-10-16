import sys, json, os, shutil, re
sys.path.insert(0,'.')
from apply7 import *
plan=json.load(open(GEN+'/pwp7-d6.json'))
SNAPR=GEN+'/pwp7-snap-repo'
touched=[]
for f,n in plan.items():
    p=ROOT+f
    t=open(p,encoding='utf-8').read()
    if f.startswith('🌎️hub'):
        m=re.search(r'(?m)^use semio_framework_plugin::\{([^}]*)\};',n)
        assert m,f
        names=[x.strip() for x in m.group(1).split(',')]
        for add in ('NeedsLabel','PluginBuilder'):
            if add not in names: names.append(add)
        names=sorted(set(names))
        n=n[:m.start()]+'use semio_framework_plugin::{%s};'%', '.join(names)+n[m.end():]
    sp=SNAPR+'/'+f
    if not os.path.exists(sp):
        os.makedirs(os.path.dirname(sp),exist_ok=True); shutil.copy2(p,sp)
    assert open(p,encoding='utf-8').read()==t
    open(p,'w',encoding='utf-8').write(n); touched.append(f)
# remove the inherent impl
rel='🏗️builder/🦀️.rs'
t=open(PLUG+rel,encoding='utf-8').read()
old='''impl<PA: PluginApp> Plugin<PA> {
    /// 🏗️ Starts a typestate plugin builder from a stable plugin id.
    pub fn builder(plugin_id: impl Into<String>) -> PluginBuilder<NeedsLabel, PA> {
        PluginBuilder::new(plugin_id)
    }
}

'''
assert t.count(old)==1
sp=SNAP+'/'+rel
if not os.path.exists(sp):
    os.makedirs(os.path.dirname(sp),exist_ok=True); shutil.copy2(PLUG+rel,sp)
open(PLUG+rel,'w',encoding='utf-8').write(t.replace(old,''))
json.dump(touched,open(GEN+'/pwp7-repo-touched.json','w'),ensure_ascii=False,indent=1)
print(len(touched),'consumer files; builder impl removed')
