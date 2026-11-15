#!/usr/bin/env python3
"""R7b re-application after a concurrent writer replaced three of the landed files with an older copy (last writer wins): re-reads each live file under T/landing.lock and re-applies the R7b edits by anchored replacement; refuses when the live text no longer has the anchors."""
import os,sys,hashlib
HERE=os.path.dirname(os.path.abspath(__file__));R="/Users/Shared/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
B=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident"
lock=T+"/landing.lock";os.mkdir(lock);open(lock+"/owner","w").write("R7b-reapply-%d\n"%os.getpid())
try:
    p=B+"/🟨️.mjs";s=open(p,encoding="utf8").read()
    if "lastAccessDenialV1" not in s:
        a="retireHostServiceV1,withRestartOwnerV1,";assert s.count(a)==1;s=s.replace(a,"retireHostServiceV1,lastAccessDenialV1,withRestartOwnerV1,")
    if "rememberLaunchdRefusalV1" not in s:
        a="probeConsentV1,sampleAccessV1,";assert s.count(a)==1;s=s.replace(a,"probeConsentV1,sampleAccessV1,rememberLaunchdRefusalV1,launchdRefusalV1,forgetLaunchdRefusalV1,RESET_MARKER_V1,")
    open(p+".r7btmp","w",encoding="utf8").write(s);os.replace(p+".r7btmp",p)
    p=B+"/🏗️composition/🌐️host/🟨️.mjs";s=open(p,encoding="utf8").read()
    if "launchd route was refused" not in s:
        def rep(a,b):
            global s
            assert s.count(a)==1,a[:60];s=s.replace(a,b)
        rep("import {accessDirectoryV1,consentIdentityV1,launcherDigestV1,probeConsentV1,","import {accessDirectoryV1,consentIdentityV1,launchdRefusalV1,launcherDigestV1,probeConsentV1,")
        rep("probeConsentV1(path,{platform:accessPlatform,home,timeoutMilliseconds:access.timeoutMilliseconds})","probeConsentV1(path,{platform:accessPlatform,home,timeoutMilliseconds:access.timeoutMilliseconds,...access.list?{list:access.list}:{},...access.stat?{stat:access.stat}:{}})")
        rep("createHostServiceV1({runtime:{...runtime},implementation,","createHostServiceV1({runtime:{...runtime},implementation,...access.root?{access:{root:access.root,...access.platform?{platform:access.platform}:{},...access.home?{home:access.home}:{}}}:{},")
        rep("  record({reason:String(reason),code});\n","  record({reason:String(reason),code,tag:runtime.tag,route:serviceRouteV1(environment),...reason===\"access-denied\"?{root:access.root??process.cwd(),folder:protectedFolderOfV1(access.root??process.cwd(),{platform:access.platform??platform,home:access.home??homedir()})??\"\"}:{}});\n")
        merged=open(HERE+"/merged-composition.mjs",encoding="utf8").read()
        i=merged.index("  const eligible=launchdRouteV1(");j=merged.index("  if(route){",i)
        old="  const route=launchdRouteV1({command,environment,cwd,runtimeDirectory:runtime.directory,tag:runtime.tag,...launchd.route});\n  if(route){"
        rep(old,merged[i:j]+"  if(route){")
    open(p+".r7btmp","w",encoding="utf8").write(s);os.replace(p+".r7btmp",p)
    print("reapplied")
finally:
    os.remove(lock+"/owner");os.rmdir(lock)
