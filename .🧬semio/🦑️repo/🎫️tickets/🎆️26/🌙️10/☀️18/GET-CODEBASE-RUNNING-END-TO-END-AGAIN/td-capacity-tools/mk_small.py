"""🧰 [td-capacity] stages the small test-debt batches timeouts, launcherns, psgate, custodian from the CURRENT live files. usage: python3 -I mk_small.py"""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
CAP = L + "⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/"
BUILD = P + "🏗️build/"
VERIFY_HEAD = '#!/bin/sh\nH=$(cd "$(dirname "$0")" && pwd)\n# usage: verify.sh <root>  (root = a tree holding the batch applied; default: walk up to the directory holding 🧰️framework)\nR=${1:-$(cd "$(dirname "$0")" && while [ ! -d 🧰️framework ] && [ "$PWD" != / ]; do cd ..; done; pwd)}\ncd -P "$R" || exit 1\nstatus=0\n'
SUITE_RUN = 'sh "$H/../../suite-in-root.sh" "$R" {0} || status=1\nexit $status\n'
SANDBOX_RUN = """if [ "$(uname)" = Darwin ]; then
  out=$(sandbox-exec -p '(version 1)(allow default)' bun test --timeout 300000 "./{0}" 2>&1); echo "sandboxed (ps refused): $(echo "$out" | /usr/bin/grep -E "^ *[0-9]+ (pass|fail)$" | tr -s " \\n" " ")"; echo "$out" | /usr/bin/grep -E "^\\(fail\\)" | cut -c1-200
  echo "$out" | /usr/bin/grep -qE "^ *0 fail" || status=1
fi
exit $status
"""
def suite(files, extra=""):
    body = "".join(f'out=$(bun test --timeout 300000 "./{f}" 2>&1); echo "{f[-60:]}: $(echo "$out" | /usr/bin/grep -E "^ *[0-9]+ (pass|fail)$" | tr -s " \\n" " ")"; echo "$out" | /usr/bin/grep -E "^\\(fail\\)" | cut -c1-200; echo "$out" | /usr/bin/grep -qE "^ *0 fail" || status=1\n' for f in files)
    return VERIFY_HEAD + extra + body + "exit $status\n"
build("timeouts", {
 CAP + "⛓️cpu-last/🟦️.ts": [
  (" expect(actual).toEqual([]);\n});\n\nconst leases=[", " expect(actual).toEqual([]);\n},180000);\n\nconst leases=["),
  (" expect(actual).toEqual([]);\n});\n\nconst inputScopes=", " expect(actual).toEqual([]);\n},180000);\n\nconst inputScopes="),
  (" expect(actual).toEqual([]);\n});\n\ntest(\"the writer detector flags", " expect(actual).toEqual([]);\n},180000);\n\ntest(\"the writer detector flags"),
 ],
 CAP + "🔒️exclusive-writer/🟦️.ts": [("expect(actual).toEqual([]);\n});", "expect(actual).toEqual([]);\n},180000);")],
}, verify=suite([CAP + "⛓️cpu-last/🟦️.ts", CAP + "🔒️exclusive-writer/🟦️.ts"], "# the two files scan every framework source through `git ls-files`: run inside a git work tree (a mirror needs `git init`); the Bun default of 5 s per test is what failed.\n"))
build("launcherns", {
 BUILD + "🎟️capacity/🧪️tests/🔐️inheritance/🟦️.ts": [
  ('actualGuardControl=actualGuardControl&&node.arguments.map(argument=>argument.getText(source)).join(",")==="store.storage,store.control";',
   'actualGuardControl=actualGuardControl&&/^(store|options)\\.storage,\\1\\.control$/u.test(node.arguments.map(argument=>argument.getText(source)).join(","));')],
}, verify=VERIFY_HEAD + SUITE_RUN.format("compiler-capacity-inheritance"))
GATE = BUILD + "🌡️host-capacity/🧪️tests/🚪️gate/🟦️.ts"
build("psgate", {
 GATE: [
  ('import {setTimeout as sleep} from "node:timers/promises";\nimport {attachHostCapacityProcessV1,', 'import {setTimeout as sleep} from "node:timers/promises";\nimport {liveChildPidsV1} from "../../../../🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts";\nimport {attachHostCapacityProcessV1,'),
  ('expect(await run("waiting"),"a process that only waits is lent after the authored time").toBe(true);',
   'const readable=liveChildPidsV1()!==undefined;\n expect(await run("waiting"),readable?"a process that only waits is lent after the authored time":"a process whose process table cannot be read is never lent").toBe(readable);'),
  ('/** 🧪️ A child process that takes a grant and then either waits',
   '/** 🛡️ An unreadable process table counts as a live child, so a grant is never lent on a guess (the host cannot run `ps`, as inside the macOS `sandbox-exec` seal of the mirror tool). */\ntest("A waiting process whose process table cannot be read is never lent",async()=>{\n const directory=fresh();let skew=0;\n await withHostCapacityV1(options(directory,{observe:async()=>100,waited:()=>40000,children:()=>undefined,now:()=>Date.now()+skew}),async handle=>{skew=40000;\n  const read=()=>JSON.parse(readFileSync(join(directory,"grants",`${handle.id}.json`),"utf8"));\n  await sleep(400);expect(read().lentSinceMs).toBeUndefined();expect(read().resumedAtMs).toBeUndefined();\n });\n},30000);\n\n/** 🧪️ A child process that takes a grant and then either waits'),
 ],
}, verify=suite([GATE]).replace("exit $status\n", SANDBOX_RUN.format(GATE)))
CUST = BUILD + "🎟️capacity/🛟️custodian/🧪️tests/"
build("custodian", {
 CUST + "🟦️.ts": [
  ('const wait=async(path:string)=>{while(!existsSync(path)){control.signal.throwIfAborted();if((control.remainingMilliseconds()??1)<1)throw Error("Custodian test gate exhausted");await Bun.sleep(10);}};',
   'const wait=async(path:string,child?:Promise<unknown>)=>{let ended=false;void child?.then(()=>{ended=true;},()=>{ended=true;});while(!existsSync(path)){control.signal.throwIfAborted();if(ended&&!existsSync(path))throw Error("Custodian test child ended before its marker "+path);if((control.remainingMilliseconds()??1)<1)throw Error("Custodian test gate exhausted");await Bun.sleep(10);}};\n const independent=(args:string[])=>{const work=new Promise<{status:number|null;stdout:string;stderr:string}>((resolve,reject)=>{const env={...process.env};delete env.SEMIO_SCRIPT_PROCESS_INVOCATION;const child=spawn(process.execPath,args,{cwd:process.cwd(),env,stdio:["ignore","pipe","pipe"]});let stdout="",stderr="";child.stdout.on("data",chunk=>{stdout+=chunk;});child.stderr.on("data",chunk=>{stderr+=chunk;});control.signal.addEventListener("abort",()=>child.kill(),{once:true});child.on("error",reject);child.on("close",status=>resolve({status,stdout,stderr}));});operations.push(work);void work.catch(()=>{});return work;};'),
  ('import {join} from "node:path";','import {spawn} from "node:child_process";\nimport {join} from "node:path";'),
  ('const idle=captureOwnedCommandResultV1(process.execPath,[fileURLToPath(new URL("./⏱️deadline/📜️script.ts",import.meta.url)),directory,serverReady],process.cwd(),{control,output:fixture.output,env:{...process.env,SEMIO_SCRIPT_PROCESS_INVOCATION:undefined}});','const idle=independent([fileURLToPath(new URL("./⏱️deadline/📜️script.ts",import.meta.url)),directory,serverReady]);'),
  ('const waiting=await captureOwnedCommandResultV1(process.execPath,[fileURLToPath(new URL("./⏱️deadline/📜️script.ts",import.meta.url)),directory,serverReady],process.cwd(),{control,output:fixture.output,env:{...process.env,SEMIO_SCRIPT_PROCESS_INVOCATION:undefined}});','const waiting=await independent([fileURLToPath(new URL("./⏱️deadline/📜️script.ts",import.meta.url)),directory,serverReady]);'),
  ('await wait(serverReady);expect(await probe()).toBe("paid-busy");const deadline=await idle;', 'await wait(serverReady,idle);expect(await probe()).toBe("paid-busy");const deadline=await idle;'),
  ('expect(idleFacts.deadline-idleFacts.started).toBe(1000);', 'expect(idleFacts.deadline-idleFacts.started).toBe(fixture.idleDeadlineMilliseconds);'),
  ('expect(waitingFacts.deadline-waitingFacts.started).toBe(1000);', 'expect(waitingFacts.deadline-waitingFacts.started).toBe(fixture.idleDeadlineMilliseconds);'),
  ('"test:paid-direct-child-custodian",30000)', '"test:paid-direct-child-custodian",120000)'),
  ('}),30000);', '}),120000);'),
 ],
 CUST + "⏱️deadline/🟦️.ts": [("maximumElapsedMilliseconds:1000}", "maximumElapsedMilliseconds:fixture.idleDeadlineMilliseconds}")],
 CUST + "🧫️fixtures/🔣️.json": [('  "parallelism": 1,\n  "output"', '  "parallelism": 1,\n  "idleDeadlineMilliseconds": 3000,\n  "output"')],
}, verify=VERIFY_HEAD + SUITE_RUN.format("compiler-capacity-custodian"))
