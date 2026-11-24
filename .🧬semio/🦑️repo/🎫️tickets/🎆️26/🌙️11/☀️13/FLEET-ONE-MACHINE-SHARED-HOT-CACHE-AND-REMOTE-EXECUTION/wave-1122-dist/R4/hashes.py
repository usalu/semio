# -*- coding: utf-8 -*-
import hashlib,subprocess
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
files=["🪢️slots/🧬️schema/🔣️.json","🪢️slots/🟦️.ts","🌐️remote-cache/🏷️retention/🍂️staleness/🟦️.ts","🌐️remote-cache/🏷️retention/🍂️staleness/🧪️tests/🐍️oracle/🐍️.py","🌐️remote-cache/🏷️retention/🍂️staleness/🧪️tests/🧫️fixtures/🔣️.json","🏃️execution/🧬️schema/🌐️remote-cache.json","🔣️policy.json","🌐️remote-cache/🧹️collector/🟦️.ts","🌐️remote-cache/💾️store/🟦️.ts","🌐️remote-cache/🖥️server/🟦️.ts","🌐️remote-cache/🏠️hosted/🟦️.ts","🌐️remote-cache/⌨️command/🟦️.ts","🌐️remote-execution/🛰️sidecar/🗄️cache/🟦️.ts","🪪️graph-state/🧬️schema/🔣️.json","🪪️graph-state/🛰️exchange/🟦️.ts","🕸️graph/📥️load/🤝️selected-exchange/🟦️.ts","🕸️graph/📥️load/🤝️selected-exchange/🧬️schema/🔣️.json","🕸️graph/📥️load/🤝️selected-exchange/🧪️tests/🟦️.ts","🌐️remote-cache/📐️blob-bound/🛡️graph-class/🧪️tests/🟦️.ts","🧪️tests/🕸️graph/🩹️repair-rounds/🟦️.ts","🧪️tests/🕸️graph/🌊️seal-under-churn/🟦️.ts","🧪️tests/🕸️graph/🧊️stale-facts/🟦️.ts","🧪️tests/🕸️graph/🔄️candidate/🟦️.ts","🕸️graph/📥️load/🧪️tests/🌊️vanished-directory/🟦️.ts"]
for f in files:
    after=hashlib.sha256(open(B+f,"rb").read()).hexdigest()[:12]
    r=subprocess.run(["git","show","HEAD:"+B+f],capture_output=True)
    before=hashlib.sha256(r.stdout).hexdigest()[:12] if r.returncode==0 else "new"
    print("|",f,"|",before,"|",after,"|")
