#!/usr/bin/env python3
# Applies after act-green/freeze-lifted. Ports raster-model to the work-accounting API (4 errors in mutpolicy-r4-raster.txt).
import os
R='/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/'
def edit(f,pairs):
    p=R+f; s=open(p,encoding='utf8').read()
    for a,b in pairs:
        assert s.count(a)==1,(f,a)
        s=s.replace(a,b)
    open(p,'w',encoding='utf8').write(s)
edit('🗺️owned-map/🦀️.rs',[('            return RetirementStep::Complete;\n','            return RetirementStep::Complete(1);\n')])
edit('🖼️asset-content-and-patch/🦀️.rs',[
 ('admit_owned_retirement(value, birth) {\n        Ok((owner, _))','admit_owned_retirement(value, birth).result {\n        Ok((owner, _))'),
 ('owner.close_step(grant).expect(','owner.close_step(grant).result.expect(')])
