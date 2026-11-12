import re,sys,json
sys.path.insert(0,sys.argv[1])
import importlib.util
spec=importlib.util.spec_from_file_location('l',sys.argv[1]+'/rootsplit-lanes.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
cat=json.load(open('🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/📖️texts/🔣️.json'))['messages']
for n,l in m.lanes().items():
    for a,e,b in l:
        ks=re.findall(r'(?:refuse|say)VerifyV1\("(?:\w+",\s*")?([^"]+)"',b)
        if ks and n not in('artifact-contract-ownership','artifact-field-parity'): print(n,ks)
k=[x for x in cat if x.startswith('run.window-config-provisioning')][0]; print(k,cat[k])
