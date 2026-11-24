import sys
root=sys.argv[1]; JS=root+"/📦️artifacts/🏗️native-build/🧩️component-pass/🧵️job-slots"; U=root+"/🏗️build/🎟️capacity/🧮️units"
def sub(path,pairs):
    import os
    if not os.path.exists(path):return
    s=open(path,encoding="utf8").read()
    for a,b in pairs:
        assert s.count(a)==1,(path,a,s.count(a)); s=s.replace(a,b)
    open(path,"w",encoding="utf8").write(s)
orc_old='''    if host["pressure"] == "critical":
        halved = 0
    elif host["pressure"] == "warn":
        halved = remote // policy["pressureWarnDivisor"]
    else:
        halved = remote
    if halved < remote:
        remote, reason = halved, "%s"
'''
orc_new='''    if host["pressure"] == "critical" and remote > 0:
        remote, reason = 0, "%s"
'''
sub(JS+"/🧪️tests/🧵️plan/🐍️oracle/🐍️.py",[(orc_old%"jobs-pressure",orc_new%"jobs-pressure")])
sub(U+"/🧪️tests/🛰️fleet-jobs/🐍️oracle/🐍️.py",[(orc_old%"fleet-pressure",orc_new%"fleet-pressure")])
sub(U+"/🧪️tests/🐍️oracle/🐍️.py",[('policy["pressureWarnDivisor"]','policy["localPressureWarnDivisor"]')])
sub(JS+"/🧪️tests/🧵️plan/🧫️fixtures/🔣️.json",[
('{"name": "raised memory pressure halves the remote jobs", "host": {"local": 1, "fleetSlots": 11, "availableMegabytes": 64000, "pressure": "warn", "unitMegabytes": 512}, "expect": {"jobs": 6, "local": 1, "remote": 5, "reason": "jobs-pressure"}}',
 '{"name": "raised memory pressure with ample memory takes nothing off the fleet share", "host": {"local": 1, "fleetSlots": 11, "availableMegabytes": 64000, "pressure": "warn", "unitMegabytes": 512}, "expect": {"jobs": 12, "local": 1, "remote": 11, "reason": "jobs-all"}},\n    {"name": "raised memory pressure with scarce memory is bound by the memory term alone", "host": {"local": 1, "fleetSlots": 11, "availableMegabytes": 3072, "pressure": "warn", "unitMegabytes": 512}, "expect": {"jobs": 3, "local": 1, "remote": 2, "reason": "jobs-memory"}}'),
('{"name": "one fitting slot under raised pressure is none", "host": {"local": 1, "fleetSlots": 1, "availableMegabytes": 64000, "pressure": "warn", "unitMegabytes": 512}, "expect": {"jobs": 1, "local": 1, "remote": 0, "reason": "jobs-pressure"}}',
 '{"name": "one fitting slot under raised pressure is still taken", "host": {"local": 1, "fleetSlots": 1, "availableMegabytes": 64000, "pressure": "warn", "unitMegabytes": 512}, "expect": {"jobs": 2, "local": 1, "remote": 1, "reason": "jobs-all"}}')])
sub(U+"/🧪️tests/🛰️fleet-jobs/🧫️fixtures/🔣️.json",[
('{"name": "raised memory pressure halves the remote jobs", "host": {"local": 4, "fleetIdle": 7, "availableMegabytes": 64000, "pressure": "warn", "unitMegabytes": 512}, "expect": {"jobs": 7, "local": 4, "remote": 3, "reason": "fleet-pressure"}}',
 '{"name": "raised memory pressure with ample memory takes nothing off the remote share", "host": {"local": 4, "fleetIdle": 7, "availableMegabytes": 64000, "pressure": "warn", "unitMegabytes": 512}, "expect": {"jobs": 11, "local": 4, "remote": 7, "reason": "fleet-all"}},\n    {"name": "raised memory pressure with scarce memory is bound by the memory term alone", "host": {"local": 2, "fleetIdle": 30, "availableMegabytes": 4000, "pressure": "warn", "unitMegabytes": 512}, "expect": {"jobs": 11, "local": 2, "remote": 9, "reason": "fleet-memory"}},\n    {"name": "the 92 idle slot release build under raised pressure: 44 remote jobs, not 14", "host": {"local": 4, "fleetIdle": 92, "availableMegabytes": 9000, "pressure": "warn", "unitMegabytes": 512}, "expect": {"jobs": 48, "local": 4, "remote": 44, "reason": "fleet-cap"}}'),
('{"name": "one idle slot under raised pressure is none", "host": {"local": 4, "fleetIdle": 1, "availableMegabytes": 64000, "pressure": "warn", "unitMegabytes": 512}, "expect": {"jobs": 4, "local": 4, "remote": 0, "reason": "fleet-pressure"}}',
 '{"name": "one idle slot under raised pressure is still taken", "host": {"local": 4, "fleetIdle": 1, "availableMegabytes": 64000, "pressure": "warn", "unitMegabytes": 512}, "expect": {"jobs": 5, "local": 4, "remote": 1, "reason": "fleet-all"}}')])
