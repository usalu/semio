#!/bin/zsh
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad
cd "${0:A:h}"
./run_sandbox.sh >/dev/null && python3 census.py $S/sb/work $S/census-work.json | /usr/bin/grep -E "ERROR|OVER|>target" ; python3 - <<PY
import sys
sys.path.insert(0,'.')
from asm import *
S="$S/sb/"
root=build_asm(S+'work',S+'../census-work.json',S+'asm')
for flags in ([], ['--test']):
    errs=run_rustc(root,flags)
    ext,non=classify(errs)
    print(flags,len(errs),'non-extern',len(non))
    seen=set()
    for e in non:
        k=(e['code'],e['msg'][:100],e['file'][-60:],e['line'])
        if k in seen: continue
        seen.add(k)
        print(' ',e['code'],e['msg'][:110],'|',e['file'].replace('🧰️framework/🔨️modules/','').split('asm-root/../')[-1][-60:],e['line'],e['text'][:80])
PY
