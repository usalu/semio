import sys,hashlib
root=sys.argv[1]
H=root+"/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity"
def edit(path,old,new):
    t=open(path,encoding="utf8").read()
    assert t.count(old)==1,(path,old[:60],t.count(old))
    open(path,"w",encoding="utf8").write(t.replace(old,new))
edit(H+"/📏️decision/🟦️.ts","rampIdleKeepBasisPoints?:number;childStartMegabytes?:number;","rampIdleKeepBasisPoints?:number;settledAfterMilliseconds?:number;settledKeepBasisPoints?:number;childStartMegabytes?:number;")
edit(H+"/📏️decision/🟦️.ts",
" const idle=rules.rampIdleAfterMilliseconds!==undefined&&rules.rampIdleObservedBasisPoints",
" const settled=rules.settledAfterMilliseconds!==undefined&&rules.settledKeepBasisPoints!==undefined&&nowMs-(grant.resumedAtMs??grant.admittedAtMs)>=rules.settledAfterMilliseconds&&typeof grant.observedMegabytes===\"number\"&&grant.observedMegabytes>0;\n if(settled)return Math.min(grant.reservedMegabytes,Math.max(rules.rampIdleKeepMegabytes??0,Math.ceil(grant.observedMegabytes!*rules.settledKeepBasisPoints!/10000)));\n const idle=rules.rampIdleAfterMilliseconds!==undefined&&rules.rampIdleObservedBasisPoints")
edit(H+"/📏️decision/🟦️.ts","a reservation is what a job needs, so the unused rest of a worst-case estimate is released to the others; without those two numbers the whole reservation stays charged.","a reservation is what a job needs, so the unused rest of a worst-case estimate is released to the others; without those two numbers the whole reservation stays charged. A grant that has run `settledAfterMilliseconds` since it was admitted or resumed and has been measured is settled: its estimate is a guess and its measurement is a fact, so it is charged `settledKeepBasisPoints` of what it holds (never under `rampIdleKeepMegabytes`, never more than it reserved), re-read at every heartbeat, so growth is followed within one beat and the unused rest of a worst-case estimate (a read of an eight-hour refresh booked 14.3 GB for 4.3 GB held) is spare for the farm and the pools at once.")
edit(H+"/🧬️schema/🔣️.json",'''            "childStartMegabytes": {
              "type": "integer",
              "minimum": 0,''','''            "settledAfterMilliseconds": {
              "type": "integer",
              "minimum": 1000,
              "description": "With settledKeepBasisPoints: how long a grant has run since it was admitted or resumed before its measurement replaces its estimate. A settled grant is charged settledKeepBasisPoints of what it measurably holds (never under rampIdleKeepMegabytes, never more than it reserved)."
            },
            "settledKeepBasisPoints": {
              "type": "integer",
              "minimum": 10000,
              "maximum": 100000
            },
            "childStartMegabytes": {
              "type": "integer",
              "minimum": 0,''')
edit(root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json",'      "rampIdleKeepBasisPoints": 15000,\n      "childStartMegabytes": 512','      "rampIdleKeepBasisPoints": 15000,\n      "settledAfterMilliseconds": 900000,\n      "settledKeepBasisPoints": 15000,\n      "childStartMegabytes": 512')
