#!/usr/bin/env python3
"""F68: lands the staged typed fixture member `🧪️testing/🧾️typed-fixture` as one directory rename (compile-atomic)."""
import sys,os,shutil;sys.path.insert(0,os.path.dirname(__file__))
from f68lib import *
STG="/private/tmp/claude-501/-Users-ueli--claude-spares-spare-1dOdZf/35823786-c219-418d-81dc-737e033e7539/scratchpad/F68/stage"
base=LIB+"/🧪️testing";final=base+"/🧾️typed-fixture";tmp=base+"/.f68-typed-fixture"
got,held=take({"testing"},8)
print("held",held)
try:
    if not held:
        assert not os.path.exists(final)
        shutil.rmtree(tmp,ignore_errors=True);shutil.copytree(STG,tmp);os.rename(tmp,final);print("landed")
finally:release(got)
