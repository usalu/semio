import os,sys,datetime,subprocess
R="/Users/Shared/semio";T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION";N=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES"
F=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🟦️.ts"
old="whose results are keyed by the implementation that produced them, so while"
new="whose results are keyed by the implementation that produced them (the read itself runs under a detached runner and survives any restart; the wait only keeps the server from claiming new work meanwhile), so while"
os.mkdir(T+"/landing.lock");open(T+"/landing.lock/owner","w").write("R42-doc\n")
open(N+"/LAND-OK","w").write("R42 "+datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")+"\n")
try:
    t=open(F,encoding="utf8").read();assert t.count(old)==1
    open(F+".r42tmp","w",encoding="utf8").write(t.replace(old,new));os.replace(F+".r42tmp",F)
    open(T+"/landings.md","a",encoding="utf8").write("\n## Package R42 second landing (%s): docstring of createSidecarRestartV1 (graph read survives restarts); no behaviour change\n- edit `🏗️build/🌐️remote-execution/🛰️sidecar/🟦️.ts` docstring only\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
finally:
    os.remove(N+"/LAND-OK");os.remove(T+"/landing.lock/owner");os.rmdir(T+"/landing.lock")
