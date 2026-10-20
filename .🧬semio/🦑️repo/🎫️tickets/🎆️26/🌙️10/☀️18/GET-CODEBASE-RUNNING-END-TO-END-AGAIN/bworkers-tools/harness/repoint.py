# 🧩 Builds the live-root harness: copies of the staged runtime files below the ticket, every relative specifier re-pointed to the sibling staged copy where one exists and to the checkout file otherwise, plus the scratch host appended to the adapter. Nothing is written outside the ticket.
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import lib
import importlib.util

spec = importlib.util.spec_from_file_location("apply_targets", os.path.join(lib.TOOLS, "apply.py"))
source = open(os.path.join(lib.TOOLS, "apply.py"), encoding="utf8").read().split("def main():")[0]
namespace = {"__name__": "apply_targets"}
sys.argv = [sys.argv[0]]
exec(compile(source, "apply.py", "exec"), namespace)
TARGETS = {name: target for name, target in namespace["TARGETS"]}
by_target = {target: name for name, target in TARGETS.items()}

OUT = os.path.join(lib.TOOLS, "harness", "live")
RUNTIME = ["workers-schema.json", "workers.ts", "workers-protocol.ts", "bazel-workers.ts", "policy.json", "script.ts"]
SPEC = re.compile(r'''(from\s*|import\()(["'])(\.{1,2}/[^"']+)\2''')


def repoint(name: str) -> str:
    target = TARGETS[name]
    text = lib.staged(name)
    here = os.path.dirname(os.path.join(lib.ROOT, target))

    def replace(match):
        head, quote, spec_path = match.groups()
        resolved = os.path.normpath(os.path.join(here, spec_path))
        relative = os.path.relpath(resolved, lib.ROOT)
        if relative in by_target and by_target[relative] in RUNTIME:
            destination = os.path.join(OUT, by_target[relative])
        else:
            destination = resolved
        rel = os.path.relpath(destination, OUT)
        if not rel.startswith("."):
            rel = "./" + rel
        return head + quote + rel + quote

    return SPEC.sub(replace, text)


os.makedirs(OUT, exist_ok=True)
for name in RUNTIME:
    text = repoint(name) if name.endswith(".ts") else lib.staged(name)
    if name == "script.ts":
        e = lib.Edit(text, "scratch")
        e.once("async function writeBazelRuntimeV1(", "export async function writeBazelRuntimeV1(")
        e.once("async function bazelCancellationControl(", "export async function bazelCancellationControl(")
        e.text += open(os.path.join(lib.TOOLS, "harness", "scratch.part.ts"), encoding="utf8").read()
        if os.environ.get("BW_INSTRUMENT") == "1":
            e.once("try{recheck?.();await runBazelWorkRequestV1(request.arguments,implementation,signal);return 0;}", "try{const started=performance.now();recheck?.();const checked=performance.now();await runBazelWorkRequestV1(request.arguments,implementation,signal);process.stderr.write(`[DEBUG] bw request recheck=${Math.round(checked-started)}ms total=${Math.round(performance.now()-started)}ms\\n`);return 0;}")
            e.once('return runOwnedCommand(process.execPath,["exec",source],cwd,task.id,compilerAdmissionBudgetV1(actual),{env:environment,signal,...(workerChildInputV1===undefined?{}:{input:workerChildInputV1})});', 'const leafStarted=performance.now();return runOwnedCommand(process.execPath,["exec",source],cwd,task.id,compilerAdmissionBudgetV1(actual),{env:environment,signal,...(workerChildInputV1===undefined?{}:{input:workerChildInputV1})}).finally(()=>process.stderr.write(`[DEBUG] bw leaf=${Math.round(performance.now()-leafStarted)}ms\\n`));')
        if os.environ.get("BW_INSTRUMENT") == "1":
            e.once("async function runBazelActionV1(argv:string[],implementation:BazelImplementationV1,cancel?:AbortSignal):Promise<void>{\n", "async function runBazelActionV1(argv:string[],implementation:BazelImplementationV1,cancel?:AbortSignal):Promise<void>{\n const mark0=performance.now(),mark=(label:string)=>process.stderr.write(`[DEBUG] bw mark ${label}=${Math.round(performance.now()-mark0)}\\n`);\n")
            e.once("const runtime=await readBazelRuntimeV1(manifestPath,implementation,checkBirth),environment=runtime.environment;\n await receiveScriptProcessInvocation(environment,async original=>{if(command===\"launcher\")", "const runtime=await readBazelRuntimeV1(manifestPath,implementation,checkBirth),environment=runtime.environment;mark(\"runtime\");\n await receiveScriptProcessInvocation(environment,async original=>{mark(\"received\");if(command===\"launcher\")")
            e.once("await withBazelCancellation(environment,original,async invocation=>{const actionPhase=", "await withBazelCancellation(environment,original,async invocation=>{mark(\"joined\");const actionPhase=")
            e.once("async(_graph,graphPhase)=>{for(const dependency of dependencies)", "async(_graph,graphPhase)=>{mark(\"store\");for(const dependency of dependencies)")
            e.once("await executeTask(manifest,runtime.workspace,invocation,graphPhase);if(command===\"action\")await captureBazelSnapshotV1({workspace:runtime.workspace,task:manifest.task},resolve(output!),invocation);}finally", "await executeTask(manifest,runtime.workspace,invocation,graphPhase);mark(\"executed\");if(command===\"action\")await captureBazelSnapshotV1({workspace:runtime.workspace,task:manifest.task},resolve(output!),invocation);mark(\"captured\");}finally")
            e.once(" let primaryFailed=false;try{const wire=await joined;", " let primaryFailed=false;const birthStarted=performance.now();try{const wire=await joined;process.stderr.write(`[DEBUG] bw birth joined=${Math.round(performance.now()-birthStarted)}\\n`);")
        text = e.text
    with open(os.path.join(OUT, name), "w", encoding="utf8") as handle:
        handle.write(text)
print("harness runtime files:", ", ".join(RUNTIME), "->", os.path.relpath(OUT, lib.ROOT)[-50:])
