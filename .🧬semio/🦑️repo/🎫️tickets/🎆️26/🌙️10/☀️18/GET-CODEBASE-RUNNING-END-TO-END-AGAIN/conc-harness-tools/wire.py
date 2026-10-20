#!/usr/bin/env python3
"""Anchored edits of existing files that wire `repo concurrency` into the CLI. Every edit is a pure function text->text: it is idempotent (already applied -> unchanged), refuses when its anchor is missing or ambiguous, and never rewrites unrelated text, so it composes with the other staged sets. The group `trace` adds the optional lease trace emitter to the lease module."""
import json,os,re,sys
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names
B=names.B;OPS=f"{B}/{names.OPERATIONS}"
class Drift(Exception):pass
def once(text,anchor,label):
    n=text.count(anchor)
    if n!=1:raise Drift(f"{label}: anchor found {n} times: {anchor[:90]!r}")
def after(text,anchor,add,label,mark=None):
    if (mark or add) in text:return text
    once(text,anchor,label);return text.replace(anchor,anchor+add,1)
def before(text,anchor,add,label,mark=None):
    if (mark or add) in text:return text
    once(text,anchor,label);return text.replace(anchor,add+anchor,1)
def sub(text,old,new,label):
    if new in text:return text
    once(text,old,label);return text.replace(old,new,1)

FLAGS='''    {
      "name": "--workloads",
      "value": "required",
      "valueName": "ids|all",
      "operations": ["concurrency"],
      "description": { "en": "concurrency plan|run: comma list of workload ids, or all.", "de": "concurrency plan|run: Kommaliste von Arbeitslast-IDs oder all." }
    },
    {
      "name": "--sessions",
      "value": "required",
      "valueName": "n",
      "operations": ["concurrency"],
      "description": { "en": "concurrency plan|run: simulated agents per workload, the first n of its sessions (default: 3, at most 8).", "de": "concurrency plan|run: simulierte Agenten je Arbeitslast, die ersten n ihrer Sitzungen (Standard: 3, höchstens 8)." }
    },
    {
      "name": "--criteria",
      "value": "required",
      "valueName": "id",
      "operations": ["concurrency"],
      "description": { "en": "concurrency run: judge every workload with this set of pass criteria instead of its own.", "de": "concurrency run: beurteilt jede Arbeitslast mit diesen Abnahmekriterien statt mit ihren eigenen." }
    },
'''
OPERATION='''    {
      "name": "concurrency",
      "requires": "none",
      "synopsis": "concurrency list|plan|run",
      "summary": { "en": "Prove that simulated agents working in one checkout copy at the same time never wait for unrelated projects, share cached work and get the results of a serial run, with Nx and Bazel.", "de": "Beweist, dass simulierte Agenten, die gleichzeitig in einer Checkout-Kopie arbeiten, nie auf fremde Projekte warten, Zwischengespeichertes gemeinsam nutzen und die Ergebnisse eines seriellen Laufs erhalten, mit Nx und Bazel." },
      "implementations": { "nx": "neutral", "bazel": "neutral" },
      "dryRun": false,
      "progress": true,
      "cancellable": true,
      "notes": [
        "Prepare the disposable copy first with `repo incremental prepare`.",
        "Waits are measured from the lease trace named by SEMIO_LEASE_TRACE; without it the progress rows only give lower bounds and the criteria stay unproven."
      ],
      "subcommands": [
        { "name": "list", "synopsis": "concurrency list", "summary": { "en": "List the authored workloads with their simulated agents.", "de": "Listet die verfassten Arbeitslasten mit ihren simulierten Agenten auf." } },
        { "name": "plan", "synopsis": "concurrency plan [--workloads=<ids|all>] [--sessions=<n>]", "summary": { "en": "Print the closure of every target, the tasks that always execute and whether the agents are independent.", "de": "Gibt die Hülle jedes Ziels, die immer ausgeführten Tasks und die Unabhängigkeit der Agenten aus." } },
        { "name": "run", "synopsis": "concurrency run [--workloads=<ids|all>] [--sessions=<n>] [--criteria=<id>]", "summary": { "en": "Run the agents at the same time and once more in a row, measure waits per lock, cache hits and correctness, and judge the pass criteria.", "de": "Führt die Agenten gleichzeitig und danach nacheinander aus, misst Wartezeiten je Sperre, Cache-Treffer und Korrektheit und beurteilt die Abnahmekriterien." } }
      ],
      "flags": ["--workloads", "--sessions", "--criteria", "--implementations", "--root", "--file", "--json"],
      "examples": [
        { "command": "repo concurrency list", "description": { "en": "List the concurrent workloads.", "de": "Listet die gleichzeitigen Arbeitslasten auf." } },
        { "command": "repo concurrency run --workloads=disjoint-projects --sessions=3", "description": { "en": "Let three agents edit and build disjoint projects at once with Nx and Bazel.", "de": "Lässt drei Agenten gleichzeitig getrennte Projekte ändern und bauen, mit Nx und Bazel." } }
      ]
    },
'''
def widen(text,flag,old,new,label):
    i=text.find(f'"name": "{flag}",')
    if i<0:raise Drift(f"catalog {flag}: flag missing")
    j=text.index('"operations": [',i);k=text.index("]",j)+1
    row=text[j:k]
    if new in row:return text
    if old not in row:raise Drift(f"catalog {label}: {row!r}")
    return text[:j]+row.replace(old,new,1)+text[k:]
def catalog(text):
    text=before(text,'    {\n      "name": "--scope",\n',FLAGS,"catalog flags",mark='"name": "--workloads"')
    text=before(text,'    {\n      "name": "dependencies",\n      "requires": "none",\n',OPERATION,"catalog operation",mark='"name": "concurrency"')
    text=widen(text,"--file",'"incremental"]','"incremental", "concurrency"]',"--file")
    text=widen(text,"--json",'"incremental", "capacity"]','"incremental", "concurrency", "capacity"]',"--json")
    text=widen(text,"--implementations",'["incremental"]','["incremental", "concurrency"]',"--implementations")
    text=widen(text,"--root",'["incremental"]','["incremental", "concurrency"]',"--root")
    text=sub(text,'"incremental plan|run: implementations to judge (default: both).", "de": "incremental plan|run: zu prüfende Implementierungen (Standard: beide)."','"incremental and concurrency plan|run: implementations to judge (default: both).", "de": "incremental und concurrency plan|run: zu prüfende Implementierungen (Standard: beide)."',"--implementations text")
    text=sub(text,'"incremental: the disposable repository copy (default: /tmp/sinc','"incremental, concurrency: the disposable repository copy (default: /tmp/sinc',"--root en")
    text=sub(text,'"incremental: die Wegwerf-Kopie des Repositorys','"incremental, concurrency: die Wegwerf-Kopie des Repositorys',"--root de")
    return text
def schema_enum(text):return sub(text,'"preflight", "incremental", "dependencies"','"preflight", "incremental", "concurrency", "dependencies"',"operations schema enum")
def requires(text):return sub(text,'"preflight", "incremental", "dependencies"','"preflight", "incremental", "concurrency", "dependencies"',"requires fixture")
def script(text):
    anchor='await runIncrementalV1(root,selected.args,parent);return;}\n'
    add=' if(selected.args[0]==="concurrency"){const{runConcurrencyV1}=await import("./%s/%s/%s/%s");await runConcurrencyV1(root,selected.args,parent);return;}\n'%(names.INCR,names.CONC,names.COMMAND,names.TS)
    return after(text,anchor,add,"build script dispatch",mark='"concurrency"){const{runConcurrencyV1}')
def child(text):
    text=sub(text,'["dependencies","cargo","watch","preflight","incremental"].includes(command??"")','["dependencies","cargo","watch","preflight","incremental","concurrency"].includes(command??"")',"child nx exclusion")
    anchor='incrementalArgumentsV1(resolved.args.slice(1));}}\n'
    add='  if(resolved.args[0]==="concurrency"){const {buildInvocationRequestsHelpV1}=await import("../../%s/%s");if(!buildInvocationRequestsHelpV1(resolved.args)){const {concurrencyArgumentsV1}=await import("../../%s/%s/%s/%s");concurrencyArgumentsV1(resolved.args.slice(1));}}\n'%(names.INVOCATION,names.TS,names.INCR,names.CONC,names.COMMAND,names.TS)
    return after(text,anchor,add,"child bootstrap arguments",mark='resolved.args[0]==="concurrency"')

HELP=json.load(open(os.path.join(here,"help-snapshots.json"),encoding="utf8")) if os.path.exists(os.path.join(here,"help-snapshots.json")) else None
def helpfixture(text):
    if HELP is None:raise Drift("help-snapshots.json missing: run render-help.py")
    d=json.loads(text)
    for lang,rows in HELP.items():
        s=d["snapshots"][lang]
        for key,row in rows["changed"].items():
            if s[key]==row["new"]:continue
            if s[key]!=row["old"]:raise Drift(f"help fixture {lang}/{key}: neither the old nor the new rendering")
            s[key]=row["new"]
        if "concurrency" in s:continue
        rebuilt={}
        for key,value in s.items():
            rebuilt[key]=value
            if key=="incremental discard":rebuilt.update(rows["pages"])
        d["snapshots"][lang]=rebuilt
    return json.dumps(d,ensure_ascii=False,indent=2)+"\n"
EDITS=[(f"{OPS}/{names.CATALOG}",catalog),(f"{OPS}/{names.SCHEMA}/{names.JSON}",schema_enum),(f"{OPS}/{names.TESTS}/{names.REQUIRES}/{names.JSON}",requires),
 (f"{OPS}/{names.TESTS}/{names.FIXTURES}/{names.HELP}/{names.JSON}",helpfixture),(f"{B}/{names.SCRIPT}",script),(f"{B}/{names.CALLER}/{names.CHILD}/{names.TS}",child)]

LEASE=f"{names.FRAMEWORK}/{names.MODULES}/{names.PROCESS}/{names.LEASES}/{names.TS}"
TRACE_HELPER='''
/** 📼 Appends one row `{version,resource,mode,startedMs,acquiredMs,pid}` to the file named by `SEMIO_LEASE_TRACE` for every acquisition that waited at least 100 ms, so a harness can measure lock waits exactly instead of from the 15 s progress rows; never fails an acquisition. */
async function traceLeaseV1(options:LeaseOptions,acquire:()=>Promise<ResourceLease>):Promise<ResourceLease>{
  const path=process.env.SEMIO_LEASE_TRACE;
  if(!path)return acquire();
  const startedMs=Date.now(),lease=await acquire(),acquiredMs=Date.now();
  if(acquiredMs-startedMs>=100)try{appendFileSync(path,JSON.stringify({version:1,resource:options.resource,mode:options.mode,startedMs,acquiredMs,pid:process.pid})+"\\n");}catch{}
  return lease;
}
'''
def trace_import(text):
    return sub(text,'import { lstatSync, mkdirSync, readdirSync,','import { appendFileSync, lstatSync, mkdirSync, readdirSync,',"lease fs import")
def trace_public(text):
    old='export function acquireResourceLease(options:LeaseOptions):Promise<ResourceLease>{return acquireResourceLeaseBody(options);}\n'
    new='export function acquireResourceLease(options:LeaseOptions):Promise<ResourceLease>{return traceLeaseV1(options,()=>acquireResourceLeaseBody(options));}\n'
    text=sub(text,old,new,"lease public acquire")
    return after(text,new,TRACE_HELPER.lstrip("\n"),"lease trace helper",mark="async function traceLeaseV1")
def trace_queued(text):
    old='export async function acquireQueuedResourceLease(options: QueuedLeaseOptions): Promise<ResourceLease> {\n'
    new='export function acquireQueuedResourceLease(options: QueuedLeaseOptions): Promise<ResourceLease> {return traceLeaseV1(options,()=>acquireQueuedResourceLeaseBody(options));}\nasync function acquireQueuedResourceLeaseBody(options: QueuedLeaseOptions): Promise<ResourceLease> {\n'
    return sub(text,old,new,"lease queued acquire")
def trace(text):return trace_queued(trace_public(trace_import(text)))
TRACE_EDITS=[(LEASE,trace)]
