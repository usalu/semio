import json, os, sys, tempfile

B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
root = sys.argv[1].rstrip("/") + "/"
live = len(sys.argv) > 2 and sys.argv[2] == "live"
src_root = "/Users/ueli/Documents/semio/"

def read(path):
    with open(path, encoding="utf-8") as handle:
        return handle.read()

def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    fd, temporary = tempfile.mkstemp(dir=os.path.dirname(path), prefix=".h29-")
    with os.fdopen(fd, "w", encoding="utf-8") as handle:
        handle.write(text)
    os.replace(temporary, path)

pending = {}

def edit(relative, pairs):
    path = root + B + relative
    text = pending.get(path)
    if text is None:
        text = read(path if live else src_root + B + relative)
    for old, new in pairs:
        count = text.count(old)
        if count != 1:
            raise SystemExit("anchor count %d in %s: %s" % (count, relative, old[:90]))
        text = text.replace(old, new)
    pending[path] = text

RE = "🌐️remote-execution/"
CT = "../🚀️offload/🔬️checkout-test/"

edit(RE + "🛰️sidecar/🚦️ready/🟦️.ts", [
 ('/** 🔎️ The `repo` binary of the account:',
  '/** 🪄️ The environment a stage command of this machine runs in (the tool directories on the path, the checkout as workspace root, the language, no dropped variable): what the checkout test worker gives `repo test-file` so it runs under the same environment as the stages that made the machine Ready. */\nexport const readyStageEnvironmentV1=(environment:Readonly<Record<string,string|undefined>>,root:string,home:string):NodeJS.ProcessEnv=>stagePathV1(environment,root,home);\n\n/** 🔎️ The `repo` binary of the account:'),
])

edit(RE + "👷️worker/🟦️.ts", [
 ('import {OFFLOAD_POLICY_V1,offloadCrateTypesOfV1} from "../🚀️offload/📐️policy/🟦️.ts";\n',
  'import {OFFLOAD_POLICY_V1,offloadCrateTypesOfV1} from "../🚀️offload/📐️policy/🟦️.ts";\n'
  'import {CHECKOUT_TEST_POLICY_V1,checkoutTestBothV1,checkoutTestProgramIsV1,validateCheckoutTestActionV1} from "' + CT + '📐️policy/🟦️.ts";\n'
  'import type {CheckoutTestBeginV1,RemoteExecutionCheckoutTestsV1} from "' + CT + '👷️worker/🟦️.ts";\n'),
 ('commands?:RemoteExecutionToolsV1;confinement?:"required"|"best-effort";',
  'commands?:RemoteExecutionToolsV1;checkoutTests?:RemoteExecutionCheckoutTestsV1;confinement?:"required"|"best-effort";'),
 ('commands?:RemoteExecutionCommandToolsV1;actions?:RemoteExecutionActionPolicyV1;',
  'commands?:RemoteExecutionCommandToolsV1;checkoutTests?:RemoteExecutionCheckoutTestsV1;actions?:RemoteExecutionActionPolicyV1;'),
 ('  if(remoteExecutionCommandProgramV1(command.arguments[0]!)){\n',
  '  if(checkoutTestProgramIsV1(command.arguments[0])){\n'
  '   const refusal=run.actions==="none"?"this worker runs no actions":run.checkoutTests===undefined?checkoutTestBothV1("refusal.disabled"):validateCheckoutTestActionV1(command);\n'
  '   if(refusal!==undefined)return fail(9,`${OFFLOAD_REFUSAL_PREFIX_V1}: ${refusal}`);\n'
  '  }else if(remoteExecutionCommandProgramV1(command.arguments[0]!)){\n'),
 ('  const first=args[0]!,executable=resolveExecutable(first,env,root,cwd),executionStart=Date.now();\n',
  '  let checkoutRun:Extract<CheckoutTestBeginV1,{kind:"run"}>|undefined;\n'
  '  if(checkoutTestProgramIsV1(command.arguments[0])){\n'
  '   const begun=await run.checkoutTests!.begin(await readFile(join(root,...CHECKOUT_TEST_POLICY_V1.request.path.split("/"))).catch(()=>new Uint8Array()),run.signal);\n'
  '   if(begun.kind==="refuse")return {outcome:"refused",status:{code:8,message:begun.message},actionResult:null,refusal:begun.reason};\n'
  '   if(begun.kind==="fail")return fail(begun.code,begun.message);\n'
  '   checkoutRun=begun;\n'
  '  }\n'
  '  const first=args[0]!,executable=checkoutRun===undefined?resolveExecutable(first,env,root,cwd):undefined,executionStart=Date.now();\n'),
 ('  const collected:Collected=executable===undefined?{exitCode:127,',
  '  const collected:Collected=checkoutRun!==undefined?await execute([...checkoutRun.argv],checkoutRun.env,checkoutRun.cwd,run.timeoutSeconds*1000,run.signal,pid=>{taskPid=pid;run.meter?.attach(pid);}).then(async raw=>raw.cancelled||raw.timedOut?raw:{...raw,...await checkoutRun!.finish(raw)}):executable===undefined?{exitCode:127,'),
 ('...(options.commands?{commands:options.commands}:{}),blobs:store}',
  '...(options.commands?{commands:options.commands}:{}),...(options.checkoutTests?{checkoutTests:options.checkoutTests}:{}),blobs:store}'),
])

edit(RE + "🛰️sidecar/👷️agent/🟦️.ts", [
 ('import {detectWorkerCommandsV1} from "../../🚀️offload/🧰️command/🛠️worker-tools/🟦️.ts";\n',
  'import {detectWorkerCommandsV1} from "../../🚀️offload/🧰️command/🛠️worker-tools/🟦️.ts";\nimport {detectWorkerCheckoutTestsV1} from "../../🚀️offload/🔬️checkout-test/👷️worker/🟦️.ts";\n'),
 ('commands=await detectWorkerCommandsV1(context.root,environment);',
  'commands=await detectWorkerCommandsV1(context.root,environment),checkoutTests=await detectWorkerCheckoutTestsV1(context.root,environment);'),
 ('extraProperties:[...rustc?.properties??[],...commands.properties]',
  'extraProperties:[...rustc?.properties??[],...commands.properties,...checkoutTests?.properties??[]]'),
 ('commands:commands.tools,', 'commands:commands.tools,...checkoutTests?{checkoutTests}:{},'),
])

edit(RE + "⌨️command/🟦️.ts", [
 ('import {detectWorkerCommandsV1} from "../🚀️offload/🧰️command/🛠️worker-tools/🟦️.ts";\n',
  'import {detectWorkerCommandsV1} from "../🚀️offload/🧰️command/🛠️worker-tools/🟦️.ts";\nimport {detectWorkerCheckoutTestsV1} from "../🚀️offload/🔬️checkout-test/👷️worker/🟦️.ts";\n'),
 ('commands=await detectWorkerCommandsV1(context.root,context.environment);',
  'commands=await detectWorkerCommandsV1(context.root,context.environment),checkoutTests=await detectWorkerCheckoutTestsV1(context.root,context.environment);'),
 ('...commands.properties]', '...commands.properties,...checkoutTests?.properties??[]]'),
 ('commands:commands.tools,', 'commands:commands.tools,...checkoutTests?{checkoutTests}:{},'),
])

TF = "🔬️test-file/"
edit(TF + "🟦️.ts", [
 ('pass?:number;fail?:number;error?:string;tail?:string}>;', 'pass?:number;fail?:number;error?:string;tail?:string;ranOn?:string}>;'),
 ('const counts=row.pass===undefined?"":testFileTextV1(language,"row.counts",{pass:row.pass,fail:row.fail??0}),ran=',
  'const counts=(row.pass===undefined?"":testFileTextV1(language,"row.counts",{pass:row.pass,fail:row.fail??0}))+(row.ranOn===undefined?"":testFileTextV1(language,"row.ran-on",{origin:row.ranOn})),ran='),
])

edit(TF + "🧬️schema/🔣️.json", [
 ('        "tail": {\n          "type": "string"\n        }\n      }\n    },\n    "Counts": {',
  '        "tail": {\n          "type": "string"\n        },\n        "ranOn": {\n          "type": "string",\n          "minLength": 1\n        }\n      }\n    },\n    "Counts": {'),
])

edit(TF + "📖️texts/🔣️.json", [
 ('    "run.oracle-environment": {',
  '    "run.remote-done": {\n      "en": "[test-file] {file} ran on {origin} ({seconds} s)",\n      "de": "[test-file] {file} lief auf {origin} ({seconds} s)"\n    },\n'
  '    "run.remote-local": {\n      "en": "[test-file] {file} runs here, the remote run was not used: {reason}",\n      "de": "[test-file] {file} läuft hier, der entfernte Lauf wurde nicht genutzt: {reason}"\n    },\n'
  '    "run.cancelled": {\n      "en": "The test run was cancelled",\n      "de": "Der Testlauf wurde abgebrochen"\n    },\n'
  '    "run.oracle-environment": {'),
 ('    "row.error": {',
  '    "row.ran-on": {\n      "en": ", ran on {origin}",\n      "de": ", lief auf {origin}"\n    },\n    "row.error": {'),
])

edit(TF + "🏃️run/🟦️.ts", [
 ('import {refuseTestFileV1,testFileTextV1,type TestFileLanguageV1} from "../📖️texts/🟦️.ts";\n',
  'import {refuseTestFileV1,testFileTextV1,type TestFileLanguageV1} from "../📖️texts/🟦️.ts";\nimport type {TestFileRemoteV1} from "../🌐️remote/🟦️.ts";\n'),
 ('tables?:ReadonlyMap<string,ProcessTestTableV1>;ensureWarden?:()=>void}>;',
  'tables?:ReadonlyMap<string,ProcessTestTableV1>;ensureWarden?:()=>void;remote?:TestFileRemoteV1}>;'),
 ('  let stopped=false;\n  for(const [index,route] of routes.entries()){\n   const identity=',
  '  let stopped=false;\n  const remoted=new Map<string,"ran"|"local">();\n  for(const [index,route] of routes.entries()){\n   if(remoted.get(route.file)==="ran")continue;\n   const identity='),
 ('   if(!route.runsOnHost){rows.push({...identity,status:"skipped",seconds:0});continue;}\n',
  '   if(!route.runsOnHost){rows.push({...identity,status:"skipped",seconds:0});continue;}\n'
  '   if(ports.remote!==undefined&&!remoted.has(route.file)){\n'
  '    const same=routes.filter(other=>other.file===route.file&&other.runsOnHost),budget=same.reduce((sum,other)=>sum+stretchedBudgetV1(other.budgetMs).milliseconds,0)+rules.elapsedSlackMs,outcome=await ports.remote(route.file,budget,{language:options.language,notice});\n'
  '    if(outcome.kind==="cancelled")throw refuseTestFileV1("failed","run.cancelled");\n'
  '    remoted.set(route.file,outcome.kind==="ran"?"ran":"local");\n'
  '    if(outcome.kind==="ran"){\n'
  '     for(let at=rows.length-1;at>=0;at--)if(rows[at]!.file===route.file)rows.splice(at,1);\n'
  '     if(outcome.output!=="")process.stderr.write(outcome.output.endsWith("\\n")?outcome.output:outcome.output+"\\n");\n'
  '     notice(testFileTextV1(options.language,"run.remote-done",{file:route.file,origin:outcome.origin,seconds:outcome.seconds}));\n'
  '     for(const row of outcome.result.files)rows.push({...row,ranOn:outcome.origin});\n'
  '     if(outcome.result.status==="failed"&&options.bail)stopped=true;\n'
  '     continue;\n'
  '    }\n'
  '    if(outcome.attempted)notice(testFileTextV1(options.language,"run.remote-local",{file:route.file,reason:outcome.reason+(outcome.detail===""?"":": "+outcome.detail.slice(0,rules.limits.errorCharacters))}));\n'
  '   }\n'),
])

edit(TF + "⌨️command/🟦️.ts", [
 ('const {runTestFileRoutesV1}=await import("../🏃️run/🟦️.ts"),result=await runTestFileRoutesV1(root,routes,{bail:options.bail,json:options.json,language:shown},{...ports.run,tables,notice}),problems=',
  'const {runTestFileRoutesV1}=await import("../🏃️run/🟦️.ts"),{createTestFileRemoteV1}=await import("../🌐️remote/🟦️.ts"),remote=ports.run?.remote??await createTestFileRemoteV1(root,environment),result=await runTestFileRoutesV1(root,routes,{bail:options.bail,json:options.json,language:shown},{...ports.run,tables,notice,...remote===undefined?{}:{remote}}),problems='),
])

edit(TF + "🧪️tests/🟦️.ts", [
 ('const run=async(args:readonly string[])=>{const lines:string[]=[],code=await runTestFileCliV1(',
  'process.env.SEMIO_TEST_FILE_OFFLOAD="never";\nconst run=async(args:readonly string[])=>{const lines:string[]=[],code=await runTestFileCliV1('),
])

for path, text in pending.items():
    write(path, text)
    print("patched", path[len(root + B):])
