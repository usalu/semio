import sys
S="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🧬️schema/🔣️.json"
N="/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R17/new/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🧬️schema/🔣️.json"
steps={"1":(" An edit that is reverted before the restart cancels it."," An edit that is reverted before the restart cancels it. The closure is looked at every watchMilliseconds."),
       "2":(" The closure is looked at every watchMilliseconds."," The closure is looked at every watchMilliseconds. The restart is logged with the changed files and the time it was deferred.")}
old,new=steps[sys.argv[1]]
for p in (S,N):
    t=open(p,encoding="utf8").read();assert t.count(old)==1;open(p,"w",encoding="utf8").write(t.replace(old,new))
print("edited")
