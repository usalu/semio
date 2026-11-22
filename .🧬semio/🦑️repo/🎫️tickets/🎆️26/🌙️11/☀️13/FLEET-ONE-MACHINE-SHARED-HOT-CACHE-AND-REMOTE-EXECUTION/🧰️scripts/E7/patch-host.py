import glob,json,os,re,sys,hashlib
ROOT="/Users/ueli/Documents/semio/🧰️framework/"
HC=ROOT+"🔨️modules/🏃️process/🏗️build/🌡️host-capacity/"
B=ROOT+"🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
def sha(p):return hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
def sub(path,old,new,count=1):
    t=open(path,encoding="utf8").read()
    n=t.count(old)
    if n!=count:raise SystemExit(f"anchor count {n}!={count} in {path}: {old[:80]}")
    open(path,"w",encoding="utf8").write(t.replace(old,new))
before={}
files=[HC+"📏️decision/🟦️.ts",HC+"🧬️schema/🔣️.json",B+"🔣️policy.json",HC+"🧪️tests/🧫️fixtures/🔣️.json",HC+"🌤️followup/x" ]
D=HC+"📏️decision/🟦️.ts"
sub(D,"swap:Readonly<{ceilingBasisPoints:number;activeMegabytesPerSecond:number;outMegabytesPerSecond:number}>","swap:Readonly<{ceilingBasisPoints:number;activeBasisPointsPerSecond:number;outBasisPointsPerSecond:number;shortBasisPoints:number}>")
sub(D," const swapCeiling=Math.floor(host.totalMegabytes*policy.swap.ceilingBasisPoints/10000);\n const swapping=host.swapOutMegabytesPerSecond!==null&&host.swapOutMegabytesPerSecond>=policy.swap.activeMegabytesPerSecond;\n","")
sub(D,"const hard:HostReasonV1|undefined=host.pressure===\"critical\"?\"pressure-critical\":host.swapUsedMegabytes!==null&&host.swapUsedMegabytes>swapCeiling&&swapping?\"swap\":host.swapOutMegabytesPerSecond!==null&&host.swapOutMegabytesPerSecond>policy.swap.outMegabytesPerSecond?\"swap-activity\":undefined;","const hard:HostReasonV1|undefined=host.pressure===\"critical\"?\"pressure-critical\":hostSwapStopV1(policy,host);")
sub(D,"/** 🧮️ The decision proper on the view `evaluateHostCapacityV1` hands it","""/** 🌊️ Whether the host's swap is a reason to wait: swap that is merely occupied never is (a long-running host parks cold pages there for days and a gate that refused on occupancy would never admit again), only swap that is worked while memory is short: the pages swapped in and out per second reach `activeBasisPointsPerSecond` of the host's memory while the occupied swap is above `ceilingBasisPoints` of it and the memory the host can still give out is below `shortBasisPoints` of it (`swap`), or the traffic alone exceeds `outBasisPointsPerSecond` of the memory per second (`swap-activity`: the disk is saturated whatever the free memory says). Every threshold is a share of the host's memory, so a 16 GB and a 64 GB host judge the same state alike.
 * @see decideHostCapacityV1 */
export function hostSwapStopV1(policy:HostCapacityPolicyV1,host:Pick<HostSampleV1,"totalMegabytes"|"availableMegabytes"|"swapUsedMegabytes"|"swapOutMegabytesPerSecond">):"swap"|"swap-activity"|undefined{
 const rate=host.swapOutMegabytesPerSecond;
 if(rate===null)return undefined;
 const rules=policy.swap,total=host.totalMegabytes,swapping=rate*10000>=total*rules.activeBasisPointsPerSecond,short=host.availableMegabytes*10000<total*rules.shortBasisPoints,crowded=host.swapUsedMegabytes!==null&&host.swapUsedMegabytes*10000>total*rules.ceilingBasisPoints;
 return crowded&&swapping&&short?"swap":rate*10000>total*rules.outBasisPointsPerSecond?"swap-activity":undefined;
}

/** 🧮️ The decision proper on the view `evaluateHostCapacityV1` hands it""")
sub(D,"export function sizeHostRequestV1(policy:HostCapacityPolicyV1,input:Readonly<{id:string;operation:string;preferredJobs?:number;kinds?:readonly HostActionKindV1[];targets?:readonly string[]}>):HostRequestV1|undefined{\n const operation=policy.operations.find(row=>row.name===input.operation);\n if(!operation)return undefined;\n if(operation.shape===\"fixed\")return requireHostCapacityV1<HostRequestV1>(\"Request\",Object.freeze({id:input.id,operation:operation.name,priority:operation.priority,minimumJobs:1,preferredJobs:1,fixedMegabytes:operation.fixedMegabytes,",
"export function sizeHostRequestV1(policy:HostCapacityPolicyV1,input:Readonly<{id:string;operation:string;preferredJobs?:number;kinds?:readonly HostActionKindV1[];targets?:readonly string[];host?:Pick<HostSampleV1,\"totalMegabytes\">}>):HostRequestV1|undefined{\n const operation=policy.operations.find(row=>row.name===input.operation);\n if(!operation)return undefined;\n const reach=input.host===undefined?Infinity:Math.max(operation.minimumMegabytes??0,Math.floor(Math.max(0,input.host.totalMegabytes-hostFloorMegabytesV1(policy,input.host))*policy.estimates.maximumShareBasisPoints/10000)),fixed=Math.min(operation.fixedMegabytes,reach);\n if(operation.shape===\"fixed\")return requireHostCapacityV1<HostRequestV1>(\"Request\",Object.freeze({id:input.id,operation:operation.name,priority:operation.priority,minimumJobs:1,preferredJobs:1,fixedMegabytes:fixed,")
sub(D," return requireHostCapacityV1<HostRequestV1>(\"Request\",Object.freeze({id:input.id,operation:operation.name,priority:operation.priority,minimumJobs:1,preferredJobs:Math.max(1,Math.min(input.preferredJobs??1,protocol.maximumJobs)),fixedMegabytes:operation.fixedMegabytes,perJobMegabytes:Math.max(...rows.map(row=>row.megabytes)),cpuPerJob:Math.max(...rows.map(row=>row.cpu))}));",
" const declared=Math.max(...rows.map(row=>row.megabytes)),perJob=Math.min(declared,Math.max(policy.estimates.minimumMegabytes,reach-fixed));\n return requireHostCapacityV1<HostRequestV1>(\"Request\",Object.freeze({id:input.id,operation:operation.name,priority:operation.priority,minimumJobs:1,preferredJobs:Math.max(1,Math.min(input.preferredJobs??1,protocol.maximumJobs)),fixedMegabytes:fixed,perJobMegabytes:perJob,...perJob<declared?{envelopePerJobMegabytes:declared}:{},cpuPerJob:Math.max(...rows.map(row=>row.cpu))}));")
S=HC+"🧬️schema/🔣️.json"
sub(S,'''          "required": [
            "ceilingBasisPoints",
            "activeMegabytesPerSecond",
            "outMegabytesPerSecond"
          ],
          "properties": {
            "ceilingBasisPoints": {
              "type": "integer",
              "minimum": 0,
              "maximum": 10000
            },
            "activeMegabytesPerSecond": {
              "type": "number",
              "minimum": 0
            },
            "outMegabytesPerSecond": {
              "type": "number",
              "minimum": 0
            }
          }''','''          "required": [
            "ceilingBasisPoints",
            "activeBasisPointsPerSecond",
            "outBasisPointsPerSecond",
            "shortBasisPoints"
          ],
          "properties": {
            "ceilingBasisPoints": {
              "type": "integer",
              "minimum": 0,
              "maximum": 10000,
              "description": "Occupied swap above this share of the host memory is crowded. Occupancy alone never stops admission: a host that has run for days keeps cold pages in swap."
            },
            "activeBasisPointsPerSecond": {
              "type": "number",
              "minimum": 0,
              "description": "Swap traffic (pages swapped in plus out) per second, as a share of the host memory in basis points, from which swap counts as worked. 0.6 is 1 MB/s on a 16 GB host."
            },
            "outBasisPointsPerSecond": {
              "type": "number",
              "minimum": 0,
              "description": "Swap traffic per second, as a share of the host memory in basis points, above which the host waits whatever its free memory is. 40 is 64 MB/s on a 16 GB host."
            },
            "shortBasisPoints": {
              "type": "integer",
              "minimum": 0,
              "maximum": 10000,
              "description": "Crowded and worked swap only stops admission while the memory the host can still give out is below this share of its memory: swap traffic with plenty of reclaimable memory is the host tidying up, not a shortage."
            }
          }''')
sub(S,'"swap": "swap use is above its ceiling"','"swap": "swap is crowded and worked while memory is short"')
sub(S,'"swap": "die Auslagerung liegt über ihrer Obergrenze"','"swap": "die Auslagerung ist voll und wird bewegt, während der Speicher knapp ist"')
P=B+"🔣️policy.json"
sub(P,'''      "ceilingBasisPoints": 2500,
      "activeMegabytesPerSecond": 1,
      "outMegabytesPerSecond": 64
''','''      "ceilingBasisPoints": 2500,
      "activeBasisPointsPerSecond": 0.6,
      "outBasisPointsPerSecond": 40,
      "shortBasisPoints": 2500
''')
F=HC+"🧪️tests/🧫️fixtures/🔣️.json"
sub(F,'"swap":{"ceilingBasisPoints":2500,"activeMegabytesPerSecond":1,"outMegabytesPerSecond":64}','"swap":{"ceilingBasisPoints":2500,"activeBasisPointsPerSecond":0.3052,"outBasisPointsPerSecond":19.53125,"shortBasisPoints":10000}')
R=HC+"🧪️tests/🌤️followup/🧫️fixtures/📼️recorded.json"
sub(R,'''   "ceilingBasisPoints": 2500,
   "activeMegabytesPerSecond": 1,
   "outMegabytesPerSecond": 64
''','''   "ceilingBasisPoints": 2500,
   "activeBasisPointsPerSecond": 0.3052,
   "outBasisPointsPerSecond": 19.53125,
   "shortBasisPoints": 10000
''')
A=ROOT+"🔨️modules/🏃️process/🪪️agent-sessions/🧪️tests/🔗️adapters/🟦️.ts"
sub(A,"swap:{ceilingBasisPoints:10000,activeMegabytesPerSecond:1,outMegabytesPerSecond:99999}","swap:{ceilingBasisPoints:10000,activeBasisPointsPerSecond:10,outBasisPointsPerSecond:99999,shortBasisPoints:10000}")
Q=HC+"🧪️tests/🔮️sqlite/🟦️.ts"
sub(Q,"const swapCeiling=Math.floor(host.totalMegabytes*policy.swap.ceilingBasisPoints/10000),rate=host.swapOutMegabytesPerSecond;","const swapCeiling=host.totalMegabytes*policy.swap.ceilingBasisPoints/10000,rate=host.swapOutMegabytesPerSecond,activeRate=host.totalMegabytes*policy.swap.activeBasisPointsPerSecond/10000,outRate=host.totalMegabytes*policy.swap.outBasisPointsPerSecond/10000,short=host.availableMegabytes<host.totalMegabytes*policy.swap.shortBasisPoints/10000;")
sub(Q,"host.swapUsedMegabytes>swapCeiling&&rate!==null&&rate>=policy.swap.activeMegabytesPerSecond?\"swap\":rate!==null&&rate>policy.swap.outMegabytesPerSecond?\"swap-activity\":null;","host.swapUsedMegabytes>swapCeiling&&rate!==null&&rate>=activeRate&&short?\"swap\":rate!==null&&rate>outRate?\"swap-activity\":null;")
O=HC+"🧪️tests/🔼️upgrade/🐍️oracle/🐍️.py"
sub(O,'''    swap_ceiling=host["totalMegabytes"]*policy["swap"]["ceilingBasisPoints"]//10000;rate=host["swapOutMegabytesPerSecond"];swapping=rate is not None and rate>=policy["swap"]["activeMegabytesPerSecond"]''','''    swap_total=host["totalMegabytes"];swap_rules=policy["swap"];swap_ceiling=swap_total*swap_rules["ceilingBasisPoints"]/10000;rate=host["swapOutMegabytesPerSecond"];swapping=rate is not None and rate>=swap_total*swap_rules["activeBasisPointsPerSecond"]/10000;short=host["availableMegabytes"]<swap_total*swap_rules["shortBasisPoints"]/10000''')
sub(O,'''host["swapUsedMegabytes"]>swap_ceiling and swapping:hard="swap"''','''host["swapUsedMegabytes"]>swap_ceiling and swapping and short:hard="swap"''')
sub(O,'''rate>policy["swap"]["outMegabytesPerSecond"]:hard="swap-activity"''','''rate>swap_total*swap_rules["outBasisPointsPerSecond"]/10000:hard="swap-activity"''')
print("ok")
