#!/usr/bin/env python3
"""R100: puts the backed up script-inputs module back (only while the live file is still the landed one) and removes the tool-boundary test."""
import os,shutil,hashlib
R="/Users/ueli/Documents/semio"
SI=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"
TB=os.path.dirname(SI)+"/🔌️provider-closures/🧪️tests/🛑️tool-boundary"
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R100"
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
if sha(SI)==open(SC+"/backup/after.sha").read():
    tmp=SI+".r100.tmp";shutil.copy(SC+"/backup/si.mjs",tmp);os.replace(tmp,SI);print("restored",sha(SI))
else:print("live file changed since the landing, NOT restored")
shutil.rmtree(TB,ignore_errors=True)
