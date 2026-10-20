import json, os

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged")


def staged(name):
    with open(os.path.join(STAGED, name), encoding="utf-8") as handle:
        return handle.read()


class Drift(Exception):
    pass


def replace_once(text, old, new, label):
    if new in text:
        return text
    count = text.count(old)
    if count != 1:
        raise Drift("%s: anchor found %d times" % (label, count))
    return text.replace(old, new)


def policy(text):
    data = json.loads(text)
    section = json.loads(staged("policy-host-capacity.json"))
    if data.get("hostCapacity") == section:
        return text
    out = {}
    for key, value in data.items():
        if key == "hostCapacity":
            continue
        if key == "workers":
            out["hostCapacity"] = section
        out[key] = value
    if "hostCapacity" not in out:
        raise Drift("policy: workers section not found")
    return json.dumps(out, indent=2, ensure_ascii=False) + "\n"


BUILD_SCRIPT = [
    (
        'export async function runBuildCliV1(root:string,args:readonly string[],environment:NodeJS.ProcessEnv=process.env):Promise<void> {\n  issueRepositoryScriptProvidersV1(environment);',
        'export async function runBuildCliV1(root:string,args:readonly string[],environment:NodeJS.ProcessEnv=process.env):Promise<void> {\n'
        '  const prefix=args[0]==="repo"||args[0]==="build"?1:0;\n'
        '  if(args[prefix]==="capacity"){const{runCapacityCommandV1}=await import("./🌡️host-capacity/⌨️command/🟦️.ts");process.exitCode=await runCapacityCommandV1(args.slice(prefix+1),{environment,write:line=>console.log(line)});return;}\n'
        '  const{withBuildCapacityV1}=await import("./🌡️host-capacity/🟦️.ts");\n'
        '  await withBuildCapacityV1(args.slice(prefix),environment,granted=>runAdmittedBuildCliV1(root,[...args.slice(0,prefix),...granted.args],environment));\n'
        '}\n\n'
        '/** 🌡️ Creates the original script envelope and runs the CLI once the machine has admitted the invocation, so its budget starts with the grant and not with the queue. */\n'
        'async function runAdmittedBuildCliV1(root:string,args:readonly string[],environment:NodeJS.ProcessEnv):Promise<void> {\n  issueRepositoryScriptProvidersV1(environment);',
    ),
    (
        'export async function runOwnedBuildInvocationV1(root:string,args:readonly string[],parent:import("../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<void> {\n requireIssuedResourceControlInvocationV1(phaseControl,parent);',
        'export async function runOwnedBuildInvocationV1(root:string,args:readonly string[],parent:import("../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<void> {\n'
        ' const{withBuildCapacityV1}=await import("./🌡️host-capacity/🟦️.ts");\n'
        ' await withBuildCapacityV1(args,process.env,granted=>runAdmittedBuildInvocationV1(root,granted.args,parent,phaseControl),{control:phaseControl});\n'
        '}\n\n'
        '/** 🌡️ Runs an invocation whose machine capacity is already granted, inherited from the CLI door or taken here for scripts routed around it. */\n'
        'async function runAdmittedBuildInvocationV1(root:string,args:readonly string[],parent:import("../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<void> {\n requireIssuedResourceControlInvocationV1(phaseControl,parent);',
    ),
]

BAZEL_SCRIPT = [
    (
        'import {bazelWorkerMnemonicV1,bazelWorkerSettingsV1} from "./👷️workers/🟦️.ts";',
        'import {bazelWorkerMnemonicV1,bazelWorkerSettingsV1} from "./👷️workers/🟦️.ts";\nimport {bazelGrantedWorkerHostV1,bazelHostPlanV1,bazelHostResourceSettingsV1,bazelResourceSetsV1} from "./🌡️host-capacity/🟦️.ts";\nimport {BUILD_HOST_CAPACITY_POLICY_V1} from "../../🌡️host-capacity/🟦️.ts";',
    ),
    (
        "const STARLARK_RULES=`def _requirements(ctx, worker = False):",
        "const STARLARK_RULES=bazelResourceSetsV1(BUILD_HOST_CAPACITY_POLICY_V1.kinds)+`def _requirements(ctx, worker = False):",
    ),
    (
        "resource_set = cargo_resources if ctx.attr.exclusive else None)",
        "resource_set = cargo_resources if ctx.attr.exclusive else _RESOURCE_SETS.get(ctx.attr.mnemonic))",
    ),
    (
        "workers=planWorkersV1(workerPolicy,currentWorkerHostV1(),",
        "hostPlan=bazelHostPlanV1(process.env),workers=planWorkersV1(workerPolicy,bazelGrantedWorkerHostV1(hostPlan,currentWorkerHostV1()),",
    ),
    (
        'options=["--lockfile_mode=off","--incompatible_strict_action_env",...cache.command,...settings.flags];',
        'options=["--lockfile_mode=off","--incompatible_strict_action_env",...cache.command,...settings.flags,...(hostPlan?bazelHostResourceSettingsV1(hostPlan).flags:[])];',
    ),
]

NX_SCRIPT = [
    (
        'import buildPolicy from "../../🏗️build/🔣️policy.json" with { type: "json" };',
        'import buildPolicy from "../../🏗️build/🔣️policy.json" with { type: "json" };\nimport {readHostCapacityPlanV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";\nimport {nxHostResourceSettingsV1} from "./🛠️tools/🌡️host-capacity/🟦️.ts";',
    ),
    (
        "env.NX_PARALLEL=environment.SEMIO_BUILD_PARALLEL??environment.NX_PARALLEL??workers.environment.NX_PARALLEL;",
        "const hostPlan=readHostCapacityPlanV1(environment);\n env.NX_PARALLEL=hostPlan?nxHostResourceSettingsV1(hostPlan).environment.NX_PARALLEL:environment.SEMIO_BUILD_PARALLEL??environment.NX_PARALLEL??workers.environment.NX_PARALLEL;",
    ),
]

OPERATIONS_SCHEMA = [
    ('"enum": ["run", "run-many", "affected", "exec", "watch", "graph", "show", "reset", "tools", "preflight", "dependencies", "cargo", "cache"]', '"enum": ["run", "run-many", "affected", "exec", "watch", "graph", "show", "reset", "tools", "preflight", "dependencies", "cargo", "cache", "capacity"]'),
]

FLAGS = '''    },
    {
      "name": "--operation",
      "value": "required",
      "valueName": "name",
      "operations": ["capacity"],
      "description": { "en": "capacity run: the catalogue operation whose estimate is reserved (default: command).", "de": "capacity run: die Operation des Katalogs, deren Schätzung reserviert wird (Standard: command)." }
    },
    {
      "name": "--memory",
      "value": "required",
      "valueName": "megabytes",
      "operations": ["capacity"],
      "description": { "en": "capacity run: reserve exactly this many megabytes instead of an operation's estimate.", "de": "capacity run: reserviert genau so viele Megabyte statt der Schätzung einer Operation." }
    },
    {
      "name": "--cpu",
      "value": "required",
      "valueName": "cores",
      "operations": ["capacity"],
      "description": { "en": "capacity run: cores the command keeps busy, counted against the load ceiling (default: 1).", "de": "capacity run: Kerne, die der Befehl auslastet und die gegen die Lastobergrenze zählen (Standard: 1)." }
    },
    {
      "name": "--jobs",
      "value": "required",
      "valueName": "n",
      "operations": ["capacity"],
      "description": { "en": "capacity run: jobs of a task operation to reserve (default: 1).", "de": "capacity run: Jobs einer Task-Operation, die reserviert werden (Standard: 1)." }
    }
  ],
  "operations": [
    {
      "name": "run",'''

CAPACITY = '''      ]
    },
    {
      "name": "capacity",
      "synopsis": "capacity status|run",
      "summary": {
        "en": "Show the machine's free memory, floor, load and the running and waiting grants, or run any command under a machine-wide capacity grant.",
        "de": "Zeigt freien Speicher, Untergrenze, Last sowie laufende und wartende Zusagen des Rechners oder führt einen beliebigen Befehl unter einer rechnerweiten Kapazitätszusage aus."
      },
      "implementations": {
        "nx": "neutral",
        "bazel": "neutral"
      },
      "dryRun": false,
      "progress": true,
      "cancellable": true,
      "subcommands": [
        {
          "name": "status",
          "synopsis": "capacity status [--json]",
          "summary": {
            "en": "Print the host sample, the free-memory floor, every running grant with what it reserved and what it holds, and every waiting request.",
            "de": "Gibt die Messung des Rechners, die Untergrenze des freien Speichers, jede laufende Zusage mit Reservierung und Verbrauch und jede wartende Anfrage aus."
          }
        },
        {
          "name": "run",
          "synopsis": "capacity run [--operation=<name>|--memory=<megabytes> [--cpu=<cores>]] [--jobs=<n>] -- <command> <args>",
          "summary": {
            "en": "Wait in the machine-wide queue until the host can carry the command, run it with the grant in its environment and give the grant back when it ends.",
            "de": "Wartet in der rechnerweiten Warteschlange, bis der Rechner den Befehl tragen kann, führt ihn mit der Zusage in seiner Umgebung aus und gibt sie am Ende zurück."
          }
        }
      ],
      "flags": [
        "--json",
        "--operation",
        "--memory",
        "--cpu",
        "--jobs"
      ],
      "examples": [
        {
          "command": "repo capacity status",
          "description": { "en": "See how much memory the machine has left above its floor and who holds it.", "de": "Zeigt, wie viel Speicher dem Rechner oberhalb der Untergrenze bleibt und wer ihn hält." }
        },
        {
          "command": "repo capacity run --memory=6144 -- bun test ./some.test.ts",
          "description": { "en": "Run an ad-hoc test runner only when 6 GB can be reserved.", "de": "Führt einen Test-Runner nur aus, wenn 6 GB reserviert werden können." }
        },
        {
          "command": "repo capacity run --operation=dependencies -- bun install",
          "description": { "en": "Reserve what a dependency sync is estimated to need.", "de": "Reserviert, was ein Abhängigkeitsabgleich voraussichtlich braucht." }
        }
      ],
      "notes": [
        "Every heavy repo command (run, run-many, affected, exec, watch, cargo, show, graph, dependencies) is admitted the same way before it starts; capacity run does it for everything else.",
        "A request is admitted while the free memory stays above the floor (a share of the host memory between 4 and 8 GB), the swap stays below its ceiling and the load below its ceiling per core, the machine-wide open file table stays below its ceiling share and fewer disk images are attached than the maximum (the last three are waited out and overridden only when nothing of ours runs); memory pressure at the critical level admits nobody. Requests are served first come first served, interactive before batch.",
        "The grant sets the parallelism of the selected tool: Nx runs with NX_PARALLEL, Bazel with --jobs and --local_resources=cpu=/memory=, Cargo with --parallelism. SEMIO_HOST_CAPACITY=off switches the governor off explicitly.",
        "Waiting prints a progress line in the chosen language and stops at the next checkpoint when the caller cancels; the grant of a process that dies is reclaimed automatically."
      ]
    }
  ]
}
'''


def catalog(text):
    text = replace_once(text, '"operations": ["graph", "show"],\n      "description": { "en": "Print lists as JSON', '"operations": ["graph", "show", "capacity"],\n      "description": { "en": "Print lists as JSON', "catalog json flag")
    text = replace_once(text, '    }\n  ],\n  "operations": [\n    {\n      "name": "run",', FLAGS, "catalog flags")
    if '"name": "capacity",' not in text:
        tail = '      ]\n    }\n  ]\n}\n'
        if not text.endswith(tail):
            raise Drift("catalog: unexpected end of file")
        text = text[: -len(tail)] + CAPACITY
    return text


def process_package(text):
    anchor = '    "./build/capacity/schema": "./🏗️build/🎟️capacity/🧬️schema/🟦️.ts",\n'
    new = anchor + '    "./build/host-capacity": "./🏗️build/🌡️host-capacity/🟦️.ts",\n    "./build/host-capacity/schema": "./🏗️build/🌡️host-capacity/🧬️schema/🟦️.ts",\n    "./build/host-capacity/schema.json": "./🏗️build/🌡️host-capacity/🧬️schema/🔣️json/🟦️.ts",\n'
    return replace_once(text, anchor, new, "process package exports")


TELEMETRY_START = "/** 🧠️ Parses fixed platform memory counter DATA without executing platform commands. */"
TELEMETRY_END = "/** 🧠️ Reads finite platform counters beneath the actual carried sample phase. */"
TELEMETRY_IMPORT_ANCHOR = 'import schema from "./🧬️schema/🔣️.json" with { type: "json" };'
TELEMETRY_IMPORT = TELEMETRY_IMPORT_ANCHOR + '\nimport {parseDarwinCountersV1,parseLinuxCountersV1} from "../../../🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts";'
TELEMETRY_BODY = """/** 🧠️ Parses fixed platform memory counter DATA without executing platform commands; the host capacity sampler owns the parsing. */
export function parseMemoryCountersV1(platform:"darwin"|"linux",first:string,second:string):Readonly<{availableBytes:number|null;swapUsedBytes:number|null;swappedOutBytes:number|null}>{
 if(Buffer.byteLength(first)>TELEMETRY_POLICY_V1.maximumOutputBytes||Buffer.byteLength(second)>TELEMETRY_POLICY_V1.maximumOutputBytes)throw Error("Bounded telemetry counter bytes required");
 const counters=platform==="darwin"?parseDarwinCountersV1(first,second,""):parseLinuxCountersV1(first,second,null,null,null);
 return Object.freeze({availableBytes:counters.availableBytes,swapUsedBytes:counters.swapUsedBytes,swappedOutBytes:counters.swappedOutBytes});
}
"""


def telemetry(text):
    if TELEMETRY_IMPORT in text and "parseDarwinCountersV1(first,second" in text:
        return text
    start, end = text.find(TELEMETRY_START), text.find(TELEMETRY_END)
    if start < 0 or end < start:
        raise Drift("telemetry: parse region not found")
    region = text[start:end]
    if "Pages free" not in region or "MemAvailable" not in region or region.count("export function") != 1:
        raise Drift("telemetry: parse region changed")
    text = text[:start] + TELEMETRY_BODY + text[end:]
    return replace_once(text, TELEMETRY_IMPORT_ANCHOR, TELEMETRY_IMPORT, "telemetry import")


def edits(items):
    def apply(text):
        for index, (old, new) in enumerate(items):
            text = replace_once(text, old, new, "edit %d" % index)
        return text
    return apply


EDITORS = {
    "policy": policy,
    "build-script": edits(BUILD_SCRIPT),
    "bazel-script": edits(BAZEL_SCRIPT),
    "nx-script": edits(NX_SCRIPT),
    "operations-schema": edits(OPERATIONS_SCHEMA),
    "catalog": catalog,
    "process-package": process_package,
    "telemetry": telemetry,
}
