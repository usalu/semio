"""🪪 [td-capacity] stages batch `envelope`: the process-module capacity suites declare the process birth the official suite runner grants (`declareBuildTestEnvelopeV1`, lane td-leases batch `envelope` must be live first), so `bun test ./file` runs them bare; those that write artifacts fall back to a private temporary directory (removed at exit) when the runner published none, and the three that opened their own receiver now use the shared test receiver. usage: python3 -I mk_envelope.py"""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
B = P + "🏗️build/"
T4 = '"../../../../🧪️testing/📥️invocation/🟦️.ts"'
def declare(owner): return f'declareBuildTestEnvelopeV1({{owner:"test:{owner}",maximumElapsedMilliseconds:300000,capabilities:{{}}}});'
FALL = 'const artifactOutput=process.env.SEMIO_TEST_ARTIFACT_DIR??mkdtempSync(join(tmpdir(),"semio-capacity-"));if(process.env.SEMIO_TEST_ARTIFACT_DIR===undefined)process.once("exit",()=>{rmSync(artifactOutput,{recursive:true,force:true});});'
ROUTE = 'import {receiveScriptProcessInvocation,scriptInvocationEnvironment} from "../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";'
edits = {
 B + "🌡️host-capacity/🧪️tests/🧭️liveness/🟦️.ts": [
  (f'import{{withBuildTestInvocationV1}}from{T4};', f'import{{declareBuildTestEnvelopeV1,withBuildTestInvocationV1}}from{T4};\n{declare("host-capacity-liveness")}')],
 B + "🎟️capacity/🧪️tests/🟦️.ts": [
  ('import {withBuildTestInvocationV1} from "../../../🧪️testing/📥️invocation/🟦️.ts";', 'import {declareBuildTestEnvelopeV1,withBuildTestInvocationV1} from "../../../🧪️testing/📥️invocation/🟦️.ts";\n' + declare("compiler-capacity")),
  ('import {availableParallelism} from "node:os";', 'import {availableParallelism,tmpdir} from "node:os";'),
  ('import {mkdtempSync,mkdirSync,writeFileSync,readFileSync,existsSync,readdirSync} from "node:fs";', 'import {mkdtempSync,mkdirSync,writeFileSync,readFileSync,existsSync,readdirSync,rmSync} from "node:fs";'),
  ('const output=process.env.SEMIO_TEST_ARTIFACT_DIR!;', FALL + '\nconst output=artifactOutput;'),
  ('try{while(!existsSync(marker)){if(child.exitCode!==null)', 'try{while(!existsSync(marker)||readFileSync(marker,"utf8")===""){if(child.exitCode!==null)')],
 B + "🎟️capacity/🧮️units/🧪️tests/🟦️.ts": [
  ('import {withBuildTestInvocationV1}from"../../../../🧪️testing/📥️invocation/🟦️.ts";', 'import {declareBuildTestEnvelopeV1,withBuildTestInvocationV1}from"../../../../🧪️testing/📥️invocation/🟦️.ts";\n' + declare("compiler-units")),
  ('import {availableParallelism}from"node:os";', 'import {availableParallelism,tmpdir}from"node:os";'),
  ('import {mkdirSync,mkdtempSync,writeFileSync}from"node:fs";', 'import {mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";'),
  ('const output=process.env.SEMIO_TEST_ARTIFACT_DIR!;', FALL + '\nconst output=artifactOutput;')],
 B + "🎟️capacity/🧪️tests/🔐️inheritance/🟦️.ts": [
  (ROUTE, 'import {scriptInvocationEnvironment} from "../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";\nimport {declareBuildTestEnvelopeV1,withBuildTestInvocationV1} from ' + T4 + ';\n' + declare("compiler-capacity-inheritance")),
  ('import {availableParallelism} from "node:os";', 'import {availableParallelism,tmpdir} from "node:os";'),
  ('receiveScriptProcessInvocation(process.env,async original=>', 'withBuildTestInvocationV1(async original=>'),
  ('output=process.env.SEMIO_TEST_ARTIFACT_DIR!;', 'output=artifactOutput;'),
  ('/** 🗺️ Checks the actual launcher consumes', FALL + '\n/** 🗺️ Checks the actual launcher consumes')],
 B + "🎟️capacity/🧪️tests/🪆️scope/🟦️.ts": [
  ('import{receiveScriptProcessInvocation,withIssuedResourceOperationV1}from"../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";', 'import{withIssuedResourceOperationV1}from"../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";\nimport{declareBuildTestEnvelopeV1,withBuildTestInvocationV1}from' + T4 + ';\n' + declare("compiler-capacity-scope")),
  ('import{availableParallelism}from"node:os";', 'import{availableParallelism,tmpdir}from"node:os";'),
  ('receiveScriptProcessInvocation(process.env,async original=>', 'withBuildTestInvocationV1(async original=>'),
  ('output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Actual ticket output required");', 'output=artifactOutput;'),
  ('/** 🪆️ Admits closed language-neutral subdivisions', FALL + '\n/** 🪆️ Admits closed language-neutral subdivisions')],
 B + "🎟️capacity/🛟️custodian/🧪️tests/🟦️.ts": [
  (ROUTE, 'import {scriptInvocationEnvironment} from "../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";\nimport {declareBuildTestEnvelopeV1,withBuildTestInvocationV1} from ' + T4 + ';\nimport {tmpdir} from "node:os";\n' + declare("compiler-capacity-custodian")),
  ('async()=>receiveScriptProcessInvocation(process.env,async original=>', 'async()=>withBuildTestInvocationV1(async original=>'),
  ('const output=process.env.SEMIO_TEST_ARTIFACT_DIR!;', 'const output=artifactOutput;'),
  ('test("Paid custodian retains', FALL + '\ntest("Paid custodian retains')],
}
verify = '''#!/bin/sh
# usage: verify.sh <root>  runs the six capacity files BARE (no envelope, no artifact directory) and expects the same results as the official suite runner. Needs the td-leases batch `envelope` (declareBuildTestEnvelopeV1) and the td-capacity batches launcherns + custodian applied first.
R=${1:-$(cd "$(dirname "$0")" && while [ ! -d 🧰️framework ] && [ "$PWD" != / ]; do cd ..; done; pwd)}
cd -P "$R" || exit 1
grep -q "export function declareBuildTestEnvelopeV1" 🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts || { echo "MISSING td-leases batch envelope"; exit 2; }
P="🧰️framework/🔨️modules/🏃️process/🏗️build"; status=0
unset SEMIO_SCRIPT_PROCESS_INVOCATION SEMIO_TEST_ARTIFACT_DIR
for f in "$P/🌡️host-capacity/🧪️tests/🧭️liveness" "$P/🎟️capacity/🧪️tests" "$P/🎟️capacity/🧮️units/🧪️tests" "$P/🎟️capacity/🧪️tests/🔐️inheritance" "$P/🎟️capacity/🧪️tests/🪆️scope" "$P/🎟️capacity/🛟️custodian/🧪️tests"; do
  out=$(bun test --timeout 300000 "./$f/🟦️.ts" 2>&1); echo "$(echo "$f" | tail -c 50): $(echo "$out" | /usr/bin/grep -E "^ *[0-9]+ (pass|fail)$" | tr -s " \\n" " ")"; echo "$out" | /usr/bin/grep -E "^\\(fail\\)" | cut -c1-200
  echo "$out" | /usr/bin/grep -qE "^ *0 fail" || status=1
done
exit $status
'''
build("envelope", edits, verify=verify, root=os.environ.get("TD_BASE_ROOT"))
