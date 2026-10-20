#!/usr/bin/env python3
"""Anchored edits of existing files that wire `repo incremental` into the CLI and the Bazel adapter. Every edit is a pure function text->text: it is idempotent (already applied -> unchanged), refuses when its anchor is missing or ambiguous, and never rewrites unrelated text, so it composes with the other staged sets."""
import json,os,re,sys
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names
B=names.B;Z=f"{B}/{names.TOOLS}/{names.BAZEL}";OPS=f"{B}/{names.OPERATIONS}"
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
      "name": "--scenario",
      "value": "required",
      "valueName": "ids|all",
      "operations": ["incremental"],
      "description": { "en": "incremental plan|run: comma list of scenario ids, or all.", "de": "incremental plan|run: Kommaliste von Szenario-IDs oder all." }
    },
    {
      "name": "--implementations",
      "value": "required",
      "valueName": "nx,bazel",
      "operations": ["incremental"],
      "description": { "en": "incremental plan|run: implementations to judge (default: both).", "de": "incremental plan|run: zu prüfende Implementierungen (Standard: beide)." }
    },
    {
      "name": "--root",
      "value": "required",
      "valueName": "path",
      "operations": ["incremental"],
      "description": { "en": "incremental: the disposable repository copy (default: /tmp/sinc, on Windows <drive>:\\\\sinc); it must not lie inside or contain the checkout.", "de": "incremental: die Wegwerf-Kopie des Repositorys (Standard: /tmp/sinc, unter Windows <Laufwerk>:\\\\sinc); sie darf weder im Checkout liegen noch ihn enthalten." }
    },
    {
      "name": "--phases",
      "value": "required",
      "valueName": "warm,edit,revert",
      "operations": ["incremental"],
      "description": { "en": "incremental run: phases to judge (default: all three).", "de": "incremental run: zu prüfende Phasen (Standard: alle drei)." }
    },
    {
      "name": "--fresh",
      "value": "none",
      "operations": ["incremental"],
      "description": { "en": "incremental prepare: discard the copy and clone it again.", "de": "incremental prepare: verwirft die Kopie und klont sie neu." }
    },
'''
OPERATION='''    {
      "name": "incremental",
      "synopsis": "incremental list|plan|prepare|run|discard",
      "summary": { "en": "Prove in a disposable copy that Nx and Bazel rebuild exactly what an edit affects, no more and no less.", "de": "Beweist in einer Wegwerf-Kopie, dass Nx und Bazel nach einer Änderung genau das Betroffene neu bauen, nicht mehr und nicht weniger." },
      "implementations": { "nx": "neutral", "bazel": "neutral" },
      "dryRun": false,
      "progress": true,
      "cancellable": true,
      "subcommands": [
        { "name": "list", "synopsis": "incremental list", "summary": { "en": "List the authored scenarios.", "de": "Listet die verfassten Szenarien auf." } },
        { "name": "plan", "synopsis": "incremental plan [--scenario=<ids|all>]", "summary": { "en": "Print the tasks each scenario must and may re-execute, per implementation.", "de": "Gibt je Implementierung aus, welche Tasks ein Szenario neu ausführen muss und darf." } },
        { "name": "prepare", "synopsis": "incremental prepare [--fresh]", "summary": { "en": "Clone the checkout into the disposable root and provision it.", "de": "Klont den Checkout in das Wegwerf-Verzeichnis und richtet es ein." } },
        { "name": "run", "synopsis": "incremental run [--scenario=<ids|all>]", "summary": { "en": "Run the scenarios warm, edited and reverted with every implementation and judge the executed tasks.", "de": "Führt die Szenarien warm, geändert und zurückgenommen mit jeder Implementierung aus und beurteilt die ausgeführten Tasks." } },
        { "name": "discard", "synopsis": "incremental discard", "summary": { "en": "Remove the disposable root.", "de": "Entfernt das Wegwerf-Verzeichnis." } }
      ],
      "flags": ["--scenario", "--implementations", "--root", "--phases", "--fresh", "--file", "--json"],
      "examples": [
        { "command": "repo incremental list", "description": { "en": "List the incremental build scenarios.", "de": "Listet die Szenarien inkrementeller Builds auf." } },
        { "command": "repo incremental run --scenario=fw-ts-comment", "description": { "en": "Judge one scenario with Nx and Bazel.", "de": "Beurteilt ein Szenario mit Nx und Bazel." } }
      ]
    },
'''
def catalog(text):
    text=before(text,'    {\n      "name": "--scope",\n',FLAGS,"catalog flags",mark='"name": "--scenario"')
    text=before(text,'    {\n      "name": "dependencies",\n',OPERATION,"catalog operation",mark='"name": "incremental"')
    for flag in ("--file","--json"):
        old=f'"name": "{flag}",'
        i=text.index(old);j=text.index('"operations": [',i);k=text.index("]",j)
        row=text[j:k]
        if '"incremental"' in row:continue
        m=re.search(r'"show"',row)
        if not m:raise Drift(f"catalog {flag}: no show operation")
        text=text[:j]+row[:m.end()]+', "incremental"'+row[m.end():]+text[k:]
    return text
def schema_enum(text):return sub(text,'"preflight", "dependencies"','"preflight", "incremental", "dependencies"',"operations schema enum")
def script(text):
    anchor='await runBuildPreflightV1(root,selected.args,parent);return;}\n'
    add=' if(selected.args[0]==="incremental"){const{runIncrementalV1}=await import("./%s/%s/%s");await runIncrementalV1(root,selected.args,parent);return;}\n'%(names.INCR,names.COMMAND,names.TS)
    return after(text,anchor,add,"build script dispatch",mark='"incremental"){const{runIncrementalV1}')
def child(text):
    text=sub(text,'["dependencies","cargo","watch","preflight"].includes(command??"")','["dependencies","cargo","watch","preflight","incremental"].includes(command??"")',"child nx exclusion")
    anchor='readBuildPreflightArgumentsV1(resolved.args);}}\n'
    add='  if(resolved.args[0]==="incremental"){const {buildInvocationRequestsHelpV1}=await import("../../%s/%s");if(!buildInvocationRequestsHelpV1(resolved.args)){const {incrementalArgumentsV1}=await import("../../%s/%s/%s");incrementalArgumentsV1(resolved.args.slice(1));}}\n'%(names.INVOCATION,names.TS,names.INCR,names.COMMAND,names.TS)
    return after(text,anchor,add,"child bootstrap arguments",mark='resolved.args[0]==="incremental"')
REPORT_FN='''
/** 🧾️ Names the compact execution log a caller asks for through the environment: one absolute path, so a conformance harness can read which task of the run executed and which was restored. */
export function bazelTaskReportOptionsV1(environment:Readonly<Record<string,string|undefined>>):string[]{
 const path=environment.SEMIO_BUILD_TASK_REPORT;
 if(path===undefined||path==="")return [];
 if(!isAbsolute(path)||/[\\r\\n]/u.test(path))throw Error("Absolute Bazel task report path required");
 return [`--execution_log_compact_file=${path}`];
}
'''
def commands(text):
    if "bazelTaskReportOptionsV1" in text:return text
    if 'from "node:path"' not in text:
        text='import {isAbsolute} from "node:path";\n'+text
    elif "isAbsolute" not in text:raise Drift("commands: node:path import without isAbsolute")
    once(text,"/** 🗺️ Maps a selected batch","commands anchor")
    return text.replace("/** 🗺️ Maps a selected batch",REPORT_FN.lstrip("\n")+"\n/** 🗺️ Maps a selected batch",1)
def adapter(text):
    text=sub(text,'import {bazelCommandPlanV1,bazelTestTimeoutV1,bazelTestWithinLimitV1} from "./%s/%s";'%(names.n(0x1F9ED,"commands"),names.TS),'import {bazelCommandPlanV1,bazelTaskReportOptionsV1,bazelTestTimeoutV1,bazelTestWithinLimitV1} from "./%s/%s";'%(names.n(0x1F9ED,"commands"),names.TS),"adapter import")
    return sub(text,'base:options.filter(option=>!option.startsWith("--jobs=")&&option!=="--keep_going"&&option!=="--subcommands"),jobs,','base:[...options.filter(option=>!option.startsWith("--jobs=")&&option!=="--keep_going"&&option!=="--subcommands"),...bazelTaskReportOptionsV1(process.env)],jobs,',"adapter base")

HELP=json.load(open(os.path.join(here,"help-snapshots.json"),encoding="utf8")) if os.path.exists(os.path.join(here,"help-snapshots.json")) else None
def helpfixture(text):
    if HELP is None:raise Drift("help-snapshots.json missing: run render-help.py")
    d=json.loads(text)
    for lang,add in HELP.items():
        s=d["snapshots"][lang]
        if "incremental" in s:continue
        root=s["root"]
        def at(prefix,label):
            hits=[i for i,line in enumerate(root) if line.startswith(prefix)]
            if len(hits)!=1:raise Drift(f"help fixture {lang} root {label}: {len(hits)} rows start with {prefix!r}")
            return hits[0]
        i=at(add["commandAfter"],"commands");root.insert(i+1,add["commandRow"])
        i=at(add["optionBefore"],"options");root[i:i]=add["optionRows"]
        for prefix,old,new in add["used"]:
            i=at(prefix,"used-by");
            if old not in root[i]:raise Drift(f"help fixture {lang}: {old!r} missing in {root[i][:60]!r}")
            root[i]=root[i].replace(old,new,1)
        i=at(add["exampleAfter"],"example");root[i+2:i+2]=add["exampleRows"]
        rebuilt={}
        for key,value in s.items():
            rebuilt[key]=value
            if key=="preflight":rebuilt.update(add["pages"])
        d["snapshots"][lang]=rebuilt
    return json.dumps(d,ensure_ascii=False,indent=2)+"\n"
EDITS=[(f"{OPS}/{names.CATALOG}",catalog),(f"{OPS}/{names.SCHEMA}/{names.JSON}",schema_enum),(f"{OPS}/{names.TESTS}/{names.FIXTURES}/{names.n(0x1F4D6,'help')}/{names.JSON}",helpfixture),
 (f"{B}/{names.SCRIPT}",script),(f"{B}/{names.CALLER}/{names.CHILD}/{names.TS}",child),(f"{Z}/{names.n(0x1F9ED,'commands')}/{names.TS}",commands),(f"{Z}/{names.SCRIPT}",adapter)]
