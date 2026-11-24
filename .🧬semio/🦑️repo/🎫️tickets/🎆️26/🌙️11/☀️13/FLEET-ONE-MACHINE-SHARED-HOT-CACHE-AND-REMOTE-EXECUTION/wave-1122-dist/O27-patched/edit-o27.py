import sys,re,io
def sub(path,pairs):
    s=open(path,encoding="utf8").read()
    for a,b in pairs:
        assert s.count(a)==1,(path,a,s.count(a))
        s=s.replace(a,b)
    open(path,"w",encoding="utf8").write(s)
root=sys.argv[1]; JS=root+"/📦️artifacts/🏗️native-build/🧩️component-pass/🧵️job-slots"; U=root+"/🏗️build/🎟️capacity/🧮️units"
sub(JS+"/🧬️schema/🔣️.json",[('        "pressureWarnDivisor": 2,\n','') if '"pressureWarnDivisor": 2' in open(JS+"/🧬️schema/🔣️.json",encoding="utf8").read() else ('        "pressureWarnDivisor": 1,\n',''),
 ('"jobs-pressure": "memory pressure is raised, so fewer remote jobs wait here"','"jobs-pressure": "memory pressure is critical, so no remote job waits here"'),
 ('"jobs-memory": "the free memory here carries fewer remote jobs"','"jobs-memory": "the free memory here carries fewer remote jobs, so fewer remote jobs wait here (memory pressure is raised)"') if False else ('"jobs-memory": "the free memory here carries fewer remote jobs"','"jobs-memory": "the free memory here carries fewer remote jobs"'),
 ('"jobs-pressure": "der Speicherdruck ist erhöht, daher warten weniger entfernte Jobs hier"','"jobs-pressure": "der Speicherdruck ist kritisch, daher wartet hier kein entfernter Job"')])
sub(JS+"/🟦️.ts",[(' if(host.pressure==="critical")limit(0,"jobs-pressure");\n else if(host.pressure==="warn")limit(Math.floor(remote/policy.pressureWarnDivisor),"jobs-pressure");\n',' if(host.pressure==="critical")limit(0,"jobs-pressure");\n'),
 ('none under critical and half under raised memory pressure. Without','none under critical memory pressure. Raised pressure takes nothing off the fleet share: a remote job holds only its small known footprint here, so the memory term alone bounds it. Without')])
sub(U+"/🧬️schema/🔣️.json",[('        "pressureWarnDivisor": 2,\n','        "localPressureWarnDivisor": 2,\n'),
 ('        "lightUnitMegabytes": 96\n','        "lightUnitMegabytes": 96,\n        "lightUnitMeasurement": {\n          "samples": 19,\n          "minimumMegabytes": 40.7,\n          "medianMegabytes": 48,\n          "maximumMegabytes": 51.4,\n          "source": "ps rss of the waiting unit-cache wrapper processes (script.ts and buildscript.ts) of the running play build, 2026-11-24 15:07; 96 is about twice the maximum"\n        }\n'),
 ('"fleet-pressure": "memory pressure is raised, so fewer remote jobs wait here"','"fleet-pressure": "memory pressure is critical, so no remote job waits here"'),
 ('"fleet-pressure": "der Speicherdruck ist erhöht, daher warten weniger entfernte Jobs hier"','"fleet-pressure": "der Speicherdruck ist kritisch, daher wartet hier kein entfernter Job"')])
sub(U+"/🟦️.ts",[('limit(Math.ceil(units/policy.pressureWarnDivisor),"pressure-warn")','limit(Math.ceil(units/policy.localPressureWarnDivisor),"pressure-warn")'),
 (' if(host.pressure==="critical")limit(0,"fleet-pressure");\n else if(host.pressure==="warn")limit(Math.floor(remote/policy.pressureWarnDivisor),"fleet-pressure");\n',' if(host.pressure==="critical")limit(0,"fleet-pressure");\n'),
 ('none under critical memory pressure and half under raised pressure. Chosen','none under critical memory pressure. Raised pressure takes nothing off the remote share: a waiting remote job is one idle wrapper process (`lightUnitMegabytes`, measured in the policy), so the memory term alone bounds it. Chosen')])
