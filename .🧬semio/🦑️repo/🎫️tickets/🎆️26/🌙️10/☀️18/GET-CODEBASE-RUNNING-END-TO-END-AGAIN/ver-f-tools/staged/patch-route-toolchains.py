"""usage: patch-route-toolchains.py <suites/🔣️.json> — routes 🏗️build/🧰️toolchains tests to the build-facets suite (the module's route-table coverage law reported it unrouted)."""
import sys
p=sys.argv[1];s=open(p).read()
old='        "🏗️build/🗃️state",\n        "🏗️build/🎚️mode"'
new='        "🏗️build/🗃️state",\n        "🏗️build/🎚️mode",\n        "🏗️build/🧰️toolchains"'
assert s.count(old)==1
open(p,'w').write(s.replace(old,new))
