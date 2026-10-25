"""One-shot transform: moves the executor half of bazel/📜️script.ts into bazel/🎬️action/🟦️.ts.

python3 make-action.py <dry|apply>
dry   writes the new module and 📜️script.next.ts beside script.ts (type-check both, then apply)
apply renames 📜️script.next.ts over 📜️script.ts when script.ts still has the base hash
"""
import hashlib, os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from blocks import blocks

ROOT = "/Users/ueli/Documents/semio"
BZ = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel"
SCRIPT = BZ + "/📜️script.ts"
NEXT = BZ + "/📜️script.next.ts"
ACTION_DIR = BZ + "/🎬️action"
ACTION = ACTION_DIR + "/🟦️.ts"

MOVE = """BazelImplementationV1 BazelManifestImplementationV1 BazelTaskManifestV1 BazelRuntimeV1 BazelSnapshotContextV1 SnapshotEntryV1 SnapshotV1
workspacePath outputAncestry fileHash interpolate requireBazelManifestV1 readBazelRuntimeV1 cargoDirectoryKeys bazelTaskCargoDirectoriesV1
issueBazelTaskEnvironmentV1 bazelTaskBaseEnvironmentV1 requireBazelImplementationV1 bazelMissingOutputRefusedV1 bazelOtherModeOutputV1
captureBazelSnapshotV1 chmodSnapshotDirectory restoreBazelSnapshotV1 workerChildInputV1 executeTask initializeBazelControlTextV1
withBazelTaskCleanupV1 bazelBirthParametersAllowedV1 withBazelCancellation readBazelBirthFileV1 launchBazelActionBirthV1 runBazelActionV1
runBazelActionInProcessV1 workerPlanV1 inProcessLockV1 bazelWorkerInProcessV1 runBazelWorkRequestV1 serveBazelWorkV1""".split()

raw = open(SCRIPT, encoding="utf-8").read()
base_hash = hashlib.sha256(raw.encode()).hexdigest()
lines = raw.split("\n")
bl = blocks(lines)
byname = {}
for name, s, e in bl:
    byname.setdefault(name, []).append((s, e))
for name in MOVE:
    assert name in byname and len(byname[name]) == 1, name

moved_ranges = sorted((byname[n][0][0], byname[n][0][1], n) for n in MOVE)
moved_lines = set()
for s, e, n in moved_ranges:
    moved_lines.update(range(s, e + 1))

import_re = re.compile(r'^\s*import\s*(type\s+)?(.*?)\s*from\s*"([^"]+)"(.*)$')
imports = []
for i, l in enumerate(lines[:80], 1):
    m = import_re.match(l)
    if m and not l.lstrip().startswith("export"):
        spec = m.group(2).strip()
        names = []
        default = None
        if spec.startswith("{"):
            inner = spec[1:spec.rindex("}")]
            for part in [p.strip() for p in inner.split(",") if p.strip()]:
                is_type = part.startswith("type ")
                part = part[5:].strip() if is_type else part
                if " as " in part:
                    orig, local = [x.strip() for x in part.split(" as ")]
                else:
                    orig = local = part
                names.append((orig, local, is_type))
        else:
            default = spec
        imports.append(dict(line=i, typeonly=bool(m.group(1)), names=names, default=default, path=m.group(3), rest=m.group(4)))

tok = re.compile(r"[A-Za-z_$][\w$]*")
def used(text):
    return set(tok.findall(text))

import_lines = {im["line"] for im in imports}
header_idx = {i for i, l in enumerate(lines, 1) if l.startswith(('import componentSchema', 'const policy=', 'const workerPolicy=', 'declare global', 'const hash=', 'if(isMainModuleV1'))}
assert len(header_idx) == 6, header_idx
moved_body = "\n".join(lines[i - 1] for i in sorted(moved_lines))
kept_body = "\n".join(l for i, l in enumerate(lines, 1) if i not in moved_lines and i not in import_lines and i not in header_idx)

def adjust(path):
    if path.startswith("./"):
        return "../" + path[2:]
    if path.startswith("../"):
        return "../" + path
    return path

def render_import(im, names, default):
    kw = "import type " if im["typeonly"] else "import "
    parts = []
    if default:
        parts.append(default)
    if names:
        parts.append("{" + ",".join((("type " if t and not im["typeonly"] else "") + (o if o == l else o + " as " + l)) for o, l, t in names) + "}")
    return kw + ",".join(parts) + ' from "' + (im["path"]) + '"'

helper = '''
/** 🗺️ The planner of orchestrated tasks: only the adapter implementation carries the planning closure, so the narrow action implementation refuses them. */
export type BazelOrchestrationV1=(invocation:ScriptInvocation)=>Parameters<typeof withBuildOrchestrationTaskEnvironmentV1>[7];

let bazelOrchestrationV1:BazelOrchestrationV1|undefined;

/** 🔌 Installs the planner an adapter process lends to the orchestrated tasks it executes. */
export function installBazelOrchestrationV1(orchestration:BazelOrchestrationV1):void{bazelOrchestrationV1=orchestration;}

/** 🗺️ Plans the children of an orchestrated task or refuses when this implementation has no planner. */
function bazelOrchestrationPlanV1(invocation:ScriptInvocation):Parameters<typeof withBuildOrchestrationTaskEnvironmentV1>[7]{
 if(!bazelOrchestrationV1)throw new Error("Orchestrated Bazel tasks run only in the adapter implementation\\nOrchestrierte Bazel-Tasks laufen nur in der Adapter-Implementierung");
 return bazelOrchestrationV1(invocation);
}

/** 🧊️ Binds the captured proof of this process to the exact snapshot, loader and runner it was started from. */
export function bazelCapturedImplementationV1(proof?:{version:1;snapshot:string;sha256:string;loaderSha256?:string;recheck?:()=>void}):BazelImplementationV1{
 if(!proof||proof.version!==1||import.meta.semioCapturedImplementationIdentity!==proof.sha256||import.meta.semioCapturedLoaderIdentity!==proof.loaderSha256||proof.loaderSha256!==hash(buildSourceClosureLoaderV1))throw new Error("Captured Bazel loader identity required");
 const implementation:BazelImplementationV1={version:1,snapshot:proof.snapshot,runner:join(dirname(proof.snapshot),"📜️script.ts"),sha256:proof.sha256,loaderSha256:proof.loaderSha256};requireBazelImplementationV1(implementation,implementation);return implementation;
}

/** 🏃️ Executes one action-side entry of a captured implementation: a persistent worker, a birth or a direct action. */
export async function runBazelActionEntryV1(argv:string[],implementation:BazelImplementationV1,recheck?:()=>void):Promise<void>{
 if(argv[0]==="work-action"){await serveBazelWorkV1(argv.slice(1),implementation,recheck);return;}
 if(argv[0]?.startsWith("birth-")){await launchBazelActionBirthV1(argv[0].slice(6),argv[1]!,argv.slice(2),implementation);}else await runBazelActionV1(argv,implementation);
}

/** 🏃️ Executes only the checksum-bound captured action implementation inside the actual child birth. */
export async function runCapturedBuildSourceV1(argv:string[],proof?:{version:1;snapshot:string;sha256:string;loaderSha256?:string;recheck?:()=>void}):Promise<void>{
 await runBazelActionEntryV1(argv,bazelCapturedImplementationV1(proof),proof?.recheck);
}
'''

moved_used = used(moved_body+helper+' readWorkerPolicyV1 createHash sep isMainModuleV1')
new_imports = []
for im in imports:
    if im["path"].endswith("🧬️schema/🔣️.json"):
        continue
    names = [n for n in im["names"] if n[1] in moved_used]
    default = im["default"] if im["default"] and im["default"] in moved_used else None
    if not names and not default:
        continue
    j = dict(im)
    j["path"] = adjust(im["path"])
    suffix = ' with { type: "json" };' if "with { type" in im["rest"] else ";"
    new_imports.append(render_import(j, names, default) + suffix)

exported_by_action = set()
action_blocks = []
for s, e, n in moved_ranges:
    chunk = lines[s - 1:e]
    action_blocks.append((n, chunk))

kept_used = used(kept_body)
needed = set()
for s, e, n in moved_ranges:
    if n in kept_used:
        needed.add(n)
needed -= {'launchBazelActionBirthV1', 'runBazelActionV1', 'serveBazelWorkV1'}

def exportify(chunk, name):
    for k, l in enumerate(chunk):
        if re.match(r"^(?:async )?function %s\b" % name, l) or re.match(r"^(?:const|let) %s\b" % name, l) or re.match(r"^type %s\b" % name, l) or re.match(r"^interface %s\b" % name, l):
            chunk[k] = "export " + l
            return chunk
    raise AssertionError(name)

out_blocks = []
for n, chunk in action_blocks:
    chunk = list(chunk)
    text = "\n".join(chunk)
    if n in needed and not any(re.match(r"^export ", l) for l in chunk if re.match(r"^export (?:async )?(?:function|const|let|type|interface) %s\b" % n, l)):
        chunk = exportify(chunk, n)
    out_blocks.append("\n".join(chunk))

action_text = "\n\n".join(out_blocks)

old_plan = "(graph,request)=>planBazelTasksV1(graph,bazelAuthorizedRequestV1(request,invocation))"
assert action_text.count(old_plan) == 1
action_text = action_text.replace(old_plan, "bazelOrchestrationPlanV1(invocation)")
anchor = "if(manifest.orchestration)return settled("
assert action_text.count(anchor) == 1
assert "const base=bazelTaskBaseEnvironmentV1(workspace,task,process.env);" in action_text


header_lines = [
    "/** 🎬️ The executor half of the Bazel adapter: everything an action runs inside its birth, with its own narrow source closure, so that editing planning, graph or launcher code never changes an action key. */",
]
header_lines += new_imports
header_lines.append('import componentSchema from"../🧬️schema/🔣️.json" with { type: "json" };const {ActionBirth:birthSchema,OutputSnapshot:snapshotSchema,Implementation:implementationSchema,Manifest:manifestSchema,Runtime:runtimeSchema}=componentSchema.definitions;')
header_lines.append('import buildPolicy from "../../../🔣️policy.json" with { type: "json" };')
header_lines.append("")
header_lines.append("const policy=componentSchema.definitions.Policy.const,workerPolicy=readWorkerPolicyV1(buildPolicy.workers);")
header_lines.append("")
header_lines.append("declare global{interface ImportMeta{readonly semioCapturedImplementationIdentity?:string;readonly semioCapturedLoaderIdentity?:string}}")
header_lines.append("")
header_lines.append('const hash=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex"),slash=(value:string)=>value.split(sep).join("/");')
header_lines.append("")

tail = '''
if(isMainModuleV1(import.meta.url))throw new Error("Bazel task entry requires its captured implementation loader");
'''

action_full = "\n".join(header_lines) + "\n" + action_text + "\n" + helper + tail

# ---- script.next.ts ----
runcap = byname["runCapturedBuildSourceV1"][0]
rs, re_ = runcap
new_lines = []
for i, l in enumerate(lines, 1):
    if i in moved_lines or lines[i - 1].startswith('declare global'):
        continue
    new_lines.append(l)
script_text = "\n".join(new_lines)

# wide runCapturedBuildSourceV1
m = re.search(r'/\*\*[^\n]*\*/\nexport async function runCapturedBuildSourceV1.*?\n\}\n', script_text, re.S)
assert m
wide = m.group(0)
head_end = wide.index('if(argv[0]==="planner")')
sig_end = wide.index("\n", wide.index("export async function runCapturedBuildSourceV1")) + 1
doc_and_sig = wide[:sig_end]
body = wide[sig_end:]
body_head_end = body.index('if(argv[0]==="planner")')
body_planner_start = body_head_end
planner_end = body.index('if(argv[0]==="work-action")')
planner = body[body_planner_start:planner_end]
planner = planner.replace("const recheck=proof.recheck;", "").replace("proof.recheck", "recheck")
new_body = ' const implementation=bazelCapturedImplementationV1(proof),recheck=proof!.recheck;' + planner + 'installBazelOrchestrationV1(invocation=>(graph,request)=>planBazelTasksV1(graph,bazelAuthorizedRequestV1(request,invocation)));await runBazelActionEntryV1(argv,implementation,recheck);\n}\n'
script_text = script_text.replace(wide, doc_and_sig + new_body)

# imports of script.next.ts: prune unused, add action import
final_lines = script_text.split("\n")
remaining_text = "\n".join(l for i, l in enumerate(final_lines) if not import_re.match(l) or l.lstrip().startswith("export"))
rem_used = used(remaining_text)
action_exports = set(needed) | {"installBazelOrchestrationV1", "bazelCapturedImplementationV1", "runBazelActionEntryV1"}
out = []
for l in final_lines:
    m = import_re.match(l)
    if m and not l.lstrip().startswith("export"):
        spec = m.group(2).strip()
        if spec.startswith("{"):
            inner = spec[1:spec.rindex("}")]
            keep = []
            for part in [p.strip() for p in inner.split(",") if p.strip()]:
                p2 = part[5:].strip() if part.startswith("type ") else part
                local = p2.split(" as ")[-1].strip()
                if local in rem_used:
                    keep.append(part)
            if not keep:
                continue
            l = l.replace("{" + inner + "}", "{" + ",".join(keep) + "}")
        else:
            if spec not in rem_used and not spec.endswith("Schema"):
                continue
    out.append(l)
script_text = "\n".join(out)

type_names = {"BazelImplementationV1", "BazelManifestImplementationV1", "BazelTaskManifestV1", "BazelRuntimeV1", "BazelSnapshotContextV1", "SnapshotEntryV1", "SnapshotV1"}
imp_names = sorted(n for n in action_exports if n in rem_used or n in {"installBazelOrchestrationV1", "bazelCapturedImplementationV1", "runBazelActionEntryV1"})
imp_line = 'import {' + ",".join(("type " + n if n in type_names else n) for n in imp_names) + '} from "./🎬️action/🟦️.ts";'
ins = script_text.index("\n", script_text.index("import {withRepositoryCacheStoreV1}") if "import {withRepositoryCacheStoreV1}" in script_text else 0)
script_text = imp_line + "\n" + script_text

os.makedirs(ACTION_DIR, exist_ok=True)
mode = sys.argv[1]
if mode == "dry":
    open(ACTION, "w", encoding="utf-8").write(action_full)
    open(NEXT, "w", encoding="utf-8").write(script_text)
    open(os.path.dirname(__file__) + "/base.sha", "w").write(base_hash)
    print("dry ok; moved", len(MOVE), "needed exports:", sorted(needed))
elif mode == "apply":
    assert hashlib.sha256(open(SCRIPT, encoding="utf-8").read().encode()).hexdigest() == open(os.path.dirname(__file__) + "/base.sha").read(), "script.ts changed since the dry run"
    os.replace(NEXT, SCRIPT)
    print("applied")
