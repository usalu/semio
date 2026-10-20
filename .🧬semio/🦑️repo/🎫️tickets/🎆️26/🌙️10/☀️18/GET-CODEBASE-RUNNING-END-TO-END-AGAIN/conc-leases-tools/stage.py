import hashlib,json,os,sys
HERE=os.path.dirname(os.path.abspath(__file__))
def find_root():
    path=HERE
    while path!=os.path.dirname(path):
        if os.path.isfile(os.path.join(path,"bunfig.toml")) and os.path.isfile(os.path.join(path,"bun.lock")):return path
        path=os.path.dirname(path)
    return "/Users/ueli/Documents/semio"
ROOT=find_root()
STAGED=os.path.join(HERE,"staged")
SOURCE=os.environ.get("STAGE_SOURCE",ROOT)
P="🧰️framework/🔨️modules/🏃️process/"
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
class Drift(Exception):pass
def once(text,old,new,label):
    n=text.count(old)
    if n!=1:raise Drift("%s: anchor found %d times"%(label,n))
    return text.replace(old,new)
def sha(text):return hashlib.sha256(text.encode("utf8")).hexdigest()
def read(rel):return open(os.path.join(SOURCE,rel),encoding="utf8").read()
bases={}
optional_bases={}
OPTIONAL=os.path.join(HERE,"staged-optional")
def write(rel,text,base,optional=False):
    path=os.path.join(OPTIONAL if optional else STAGED,rel);os.makedirs(os.path.dirname(path),exist_ok=True)
    open(path,"w",encoding="utf8").write(text);(optional_bases if optional else bases)[rel]=sha(base)
def patch(rel,edits,optional=False):
    base=read(rel);text=base
    for i,(old,new) in enumerate(edits):text=once(text,old,new,"%s edit %d"%(rel,i))
    write(rel,text,base,optional)
def load(name):
    return open(os.path.join(HERE,"patches",name),encoding="utf8").read()

LEASES=P+"🔒️leases/🟦️.ts"
patch(LEASES,[
 ('import { createHash } from "node:crypto";','import { createHash, randomBytes } from "node:crypto";\nimport { AsyncLocalStorage } from "node:async_hooks";\nimport {beginLeaseOrderV1,classifyLeaseResourceV1,forgetLeaseV1,heldLeasesV1,leaseOrderPolicyV1,leaseTaskIdV1,type LeaseOrderGuardV1} from "./🧭️order/🟦️.ts";\nexport {LeaseOrderErrorV1,formatLeaseViolationV1,leaseInheritanceEnvironmentV1} from "./🧭️order/🟦️.ts";'),
 ('import {requireIssuedResourceControlV1,requireAcceptedPhysicalCleanupDescendantV1,','import {readIssuedResourceControlCapabilitiesV1,requireIssuedResourceControlV1,requireAcceptedPhysicalCleanupDescendantV1,'),
 ('export function acquireResourceLease(options:LeaseOptions):Promise<ResourceLease>{return acquireResourceLeaseBody(options);}',load("acquire-wrapper.ts.txt").rstrip("\n")),
 ('const started=Date.now();remaining(options,started,cleanup);let gate:ResourceLease|undefined,lease:ResourceLease|undefined,failed=false,failure:unknown;\n try{gate=await directoryAdmission(options,"shared",started,cleanup);',
  'const started=Date.now();remaining(options,started,cleanup);\n if(!cleanup&&options.timeoutMs!==0&&leaseOrderPolicyV1().gate.releaseWhileWaiting)return acquireWaitingOutsideGate(options,started);\n return acquireHoldingGate(options,started,cleanup);\n}\n'+load("outside-gate.ts.txt").rstrip("\n")+'\nasync function acquireHoldingGate(options:LeaseOptions,started:number,cleanup?:CleanupLeaseScope):Promise<ResourceLease>{\n let gate:ResourceLease|undefined,lease:ResourceLease|undefined,failed=false,failure:unknown;\n try{gate=await directoryAdmission(options,"shared",started,cleanup);'),
 ('export async function acquireQueuedResourceLease(options: QueuedLeaseOptions): Promise<ResourceLease> {\n  options=Object.freeze({...options});',load("queued-wrapper.ts.txt").rstrip("\n")+'\nasync function acquireQueuedResourceLeaseBody(options: QueuedLeaseOptions,guard?:LeaseOrderGuardV1): Promise<ResourceLease> {\n  options=Object.freeze({...options});'),
 ('  return awaitQueuedTurn(options,started,queue,ticket,claim);\n}','  return awaitQueuedTurn(options,started,queue,ticket,claim,guard);\n}'),
 ('async function awaitQueuedTurn(options:QueuedLeaseOptions,started:number,queue:string,ticket:string,claim:ResourceLease):Promise<ResourceLease>{','async function awaitQueuedTurn(options:QueuedLeaseOptions,started:number,queue:string,ticket:string,claim:ResourceLease,guard?:LeaseOrderGuardV1):Promise<ResourceLease>{'),
 ('const lease = await acquireResourceLease({ ...options, timeoutMs: options.timeoutMs === undefined ? undefined : Math.max(0, options.timeoutMs - (Date.now() - started)) });','const lease = await acquireResourceLeaseBody({ ...options, timeoutMs: options.timeoutMs === undefined ? undefined : Math.max(0, options.timeoutMs - (Date.now() - started)) });'),
 ('      if (elapsedMs >= nextProgress) {\n        const progress = { resource: options.resource, mode: options.mode, elapsedMs },info=','      await guard?.contention(elapsedMs);\n      if (elapsedMs >= nextProgress) {\n        const progress = { resource: options.resource, mode: options.mode, elapsedMs },info='),
 ('  const started=Date.now();remaining(options,started);\n  const name=join(turn.queue,turn.ticket),temporary=','  const started=Date.now();remaining(options,started);forgetLeaseV1(lease);const guard=orderGuard(options,true);\n  const name=join(turn.queue,turn.ticket),temporary='),
 ('  return awaitQueuedTurn(options,started,turn.queue,turn.ticket,turn.claim);\n}','  let upgraded:ResourceLease;try{upgraded=await awaitQueuedTurn(options,started,turn.queue,turn.ticket,turn.claim,guard);}catch(error){guard?.finish();throw error;}\n  guard?.hold(upgraded);return upgraded;\n}'),
 ('    for (const resource of [...resources.keys()].sort()) leases.push(await acquireResourceLease({ ...options, resource, mode: resources.get(resource)! }));','    for (const resource of orderedLeaseResources([...resources.keys()])) leases.push(await acquireOrdered({ ...options, resource, mode: resources.get(resource)! }));'),
])
text=open(os.path.join(STAGED,LEASES),encoding="utf8").read()
text=once(text,'/** 🪢️ Orders a resource set consistently and releases it in reverse order on every exit. */',load("ordered.ts.txt").rstrip("\n")+'\n/** 🪢️ Orders a resource set by declared lock rank then name, takes shared and exclusive access through the fair arrival queue (readers join cohorts, writers are not starved, a task re-entering a lease it already holds bypasses the queue) and releases in reverse order on every exit. */',"ordered")
open(os.path.join(STAGED,LEASES),"w",encoding="utf8").write(text)

PKGJSON=P+"📦️packages/🟦️typescript/package.json"
patch(PKGJSON,[('    "./leases/physical": "./🔒️leases/📁️physical/🟦️.ts",\n','    "./leases/physical": "./🔒️leases/📁️physical/🟦️.ts",\n    "./leases/order": "./🔒️leases/🧭️order/🟦️.ts",\n    "./leases/status": "./🔒️leases/📋️status/🟦️.ts",\n')])

SCRIPT=LIB+"🏗️build/📜️script.ts"
patch(SCRIPT,[('  if(args[prefix]==="capacity"){const{buildInvocationRequestsHelpV1}','  if(args[prefix]==="leases"){const{buildInvocationRequestsHelpV1}=await import("./🧭️invocation/🟦️.ts");if(buildInvocationRequestsHelpV1(args.slice(prefix))){const{buildHelpForArgumentsV1}=await import("./⚙️operations/📖️help/🟦️.ts");console.log(buildHelpForArgumentsV1(args.slice(prefix),environment));return;}const{runLeasesCliV1}=await import("./🔒️leases/⌨️command/🟦️.ts");process.exitCode=await runLeasesCliV1(root,args.slice(prefix+1),environment);return;}\n  if(args[prefix]==="capacity"){const{buildInvocationRequestsHelpV1}')])

OPS=LIB+"🏗️build/⚙️operations/"
patch(OPS+"🧬️schema/🔣️.json",[('"cache", "capacity", "remote-execution"]','"cache", "capacity", "leases", "remote-execution"]')])
FLAGS_TAIL='''      "description": { "en": "capacity run: jobs of a task operation to reserve (default: 1).", "de": "capacity run: Jobs einer Task-Operation, die reserviert werden (Standard: 1)." }
    }
  ],
  "operations": ['''
FLAGS_NEW='''      "description": { "en": "capacity run: jobs of a task operation to reserve (default: 1).", "de": "capacity run: Jobs einer Task-Operation, die reserviert werden (Standard: 1)." }
    },
    {
      "name": "--lease-directory",
      "value": "required",
      "valueName": "path",
      "operations": ["leases"],
      "description": { "en": "leases status: the lease directory to inspect (default: the checkout's lease store).", "de": "leases status: das zu untersuchende Lease-Verzeichnis (Standard: der Lease-Speicher dieses Checkouts)." }
    },
    {
      "name": "--violations",
      "value": "required",
      "valueName": "n",
      "operations": ["leases"],
      "description": { "en": "leases status: how many recent lock order violations to list (default: 10).", "de": "leases status: wie viele jüngste Verletzungen der Sperrreihenfolge aufgelistet werden (Standard: 10)." }
    }
  ],
  "operations": ['''
LEASES_OP='''    {
      "name": "leases",
      "requires": "none",
      "synopsis": "leases status",
      "summary": {
        "en": "Show which processes hold and wait for resource leases, how long, who blocks whom, deadlock cycles, lock order violations and the machine's admission queue.",
        "de": "Zeigt, welche Prozesse Ressourcen-Leases halten und erwarten, wie lange, wer wen blockiert, Verklemmungszyklen, Verletzungen der Sperrreihenfolge und die Zulassungswarteschlange des Rechners."
      },
      "implementations": {
        "nx": "neutral",
        "bazel": "neutral"
      },
      "dryRun": false,
      "progress": false,
      "cancellable": false,
      "subcommands": [
        {
          "name": "status",
          "synopsis": "leases status [--json] [--all] [--lease-directory=<path>] [--violations=<n>]",
          "summary": {
            "en": "Print every held and awaited lease with mode, class, holder, hold duration, queue position and blockers, the wait-for graph, deadlock cycles, recent order violations and the admission queue.",
            "de": "Gibt jedes gehaltene und erwartete Lease mit Modus, Klasse, Halter, Haltedauer, Warteschlangenposition und Blockierern aus, dazu den Wartegraphen, Verklemmungszyklen, jüngste Reihenfolgeverletzungen und die Zulassungswarteschlange."
          }
        }
      ],
      "flags": [
        "--json",
        "--all",
        "--lease-directory",
        "--violations"
      ],
      "examples": [
        {
          "command": "repo leases status",
          "description": { "en": "See who holds the repository's leases, who waits behind whom and for how long.", "de": "Zeigt, wer die Leases des Repositorys hält, wer hinter wem wartet und wie lange." }
        },
        {
          "command": "repo leases status --all --json",
          "description": { "en": "Machine-readable state of every lease directory that registered a process.", "de": "Maschinenlesbarer Zustand jedes Lease-Verzeichnisses, in dem sich ein Prozess registriert hat." }
        }
      ],
      "notes": [
        "Leases are declared in lock classes with a rank (SEMIO_LEASE_ORDER=off|observe|enforce overrides the declared rules). A request that breaks the order, closes a deadlock cycle, or waits beyond the grace period while its task holds another lease exclusively fails fast with a diagnostic naming the blocking process.",
        "Processes register what they hold and await in the per-user lease registry (SEMIO_LEASE_REGISTRY_DIRECTORY overrides its location); entries of dead processes are ignored and removed."
      ]
    },
'''
patch(OPS+"🔣️catalog.json",[
 ('"operations": ["graph", "show", "incremental", "capacity"],','"operations": ["graph", "show", "incremental", "capacity", "leases"],'),
 ('"operations": ["affected", "watch", "dependencies"],','"operations": ["affected", "watch", "dependencies", "leases"],'),
 ('"description": { "en": "Select every project (affected, watch) or every dependency owner (dependencies).", "de": "Wählt jedes Projekt (affected, watch) oder jeden Abhängigkeits-Eigentümer (dependencies)." }','"description": { "en": "Select every project (affected, watch), every dependency owner (dependencies) or every lease directory with registered processes (leases).", "de": "Wählt jedes Projekt (affected, watch), jeden Abhängigkeits-Eigentümer (dependencies) oder jedes Lease-Verzeichnis mit registrierten Prozessen (leases)." }'),
 (FLAGS_TAIL,FLAGS_NEW),
 ('    {\n      "name": "remote-execution",',LEASES_OP+'    {\n      "name": "remote-execution",'),
])

PROC=P+"📜️script.ts"
patch(PROC,[('      "resource-lease-fairness": {source:"🔒️leases/⚖️fairness/🧪️tests/🟦️.ts",budgetMs:TEST_LEVEL_BUDGET_MS.quick},\n','      "resource-lease-fairness": {source:"🔒️leases/⚖️fairness/🧪️tests/🟦️.ts",budgetMs:TEST_LEVEL_BUDGET_MS.quick},\n      "resource-lease-order": {source:"🔒️leases/🧭️order/🧪️tests/🟦️.ts",budgetMs:TEST_LEVEL_BUDGET_MS.quick},\n      "resource-lease-order-processes": {source:"🔒️leases/🧭️order/🧪️tests/📥️processes/🟦️.ts",budgetMs:TEST_LEVEL_BUDGET_MS.quick},\n')])
def target(name):
    return '''    "test-%s": {
      "executor": "repo:script",
      "cache": false,
      "dependsOn": [],
      "inputs": [
        "{projectRoot}/**/*"
      ],
      "outputs": [],
      "options": {
        "cwd": "🧰️framework/🔨️modules/🏃️process",
        "command": "bun ./📜️script.ts test %s"
      },
      "metadata": {
        "semio": {
          "execution": {
            "version": 1,
            "kind": "source"
          },
          "cache": {
            "version": 1,
            "class": "live",
            "reason": "exercises-host-resources"
          }
        }
      }
    },
'''%(name,name)
patch(P+"📋️project.json",[('    "test-windows-conditional-retirement": {',target("resource-lease-order")+target("resource-lease-order-processes")+'    "test-windows-conditional-retirement": {')])

ROUTING=P+"🧭️routing/📥️invocation/🏃️process/🟦️.ts"
patch(ROUTING,[('import{AsyncLocalStorage}from"node:async_hooks";','import{AsyncLocalStorage}from"node:async_hooks";\nimport{leaseInheritanceEnvironmentV1}from"../../../🔒️leases/🧭️order/🟦️.ts";'),
 ('  return { ...environment, [SCRIPT_PROCESS_INVOCATION_ENV]: JSON.stringify(readScriptProcessEnvelope(envelope, Date.now())) };','  return leaseInheritanceEnvironmentV1({ ...environment, [SCRIPT_PROCESS_INVOCATION_ENV]: JSON.stringify(readScriptProcessEnvelope(envelope, Date.now())) });')],optional=True)
json.dump(optional_bases,open(os.path.join(HERE,"bases-optional.json"),"w",encoding="utf8"),indent=1,ensure_ascii=False)
json.dump(bases,open(os.path.join(HERE,"bases.json"),"w",encoding="utf8"),indent=1,ensure_ascii=False)
print("staged",len(bases),"patched files")
