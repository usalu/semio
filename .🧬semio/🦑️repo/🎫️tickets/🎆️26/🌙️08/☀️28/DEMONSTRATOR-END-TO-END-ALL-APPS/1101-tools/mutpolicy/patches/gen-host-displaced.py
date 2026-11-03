#!/usr/bin/env python3
# Applies after act-green/freeze-lifted: gen3d host test-gated helper still reads the old close_step Result (mutpolicy-r4-gen3d.txt).
import glob
for f in glob.glob('/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/*/🔨️modules/🏠️host/🦀️.rs'):
    s=open(f,encoding='utf8').read()
    a='owner.close_step(grant).expect("cold displaced replay close")'
    if s.count(a)==1:
        open(f,'w',encoding='utf8').write(s.replace(a,'owner.close_step(grant).result.expect("cold displaced replay close")'));print('patched',f)
