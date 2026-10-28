import sys
root = sys.argv[1]
path = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts"
s = open(path, encoding="utf8").read()
def sub(old, new):
    global s
    assert s.count(old) == 1, old[:80]
    s = s.replace(old, new)
import re
match = re.search(r'^import \{runBazelRuntimeCommandV1[^\n]*from "\./🌿️environment/🟦️\.ts";\n', s, re.M)
assert match and s.count(match.group(0)) == 1
s = s.replace(match.group(0), match.group(0) + 'import {checkBazelRuntimeRowV1,measureBazelRuntimeRowV1} from "./🌿️environment/🔏️identity/🟦️.ts";\n')
sub(' return witness.allow("runtime",async()=>{const stdout=await run();witness.observe("x",runtime+"\\0"+String(original.SEMIO_BUILD_MODE),hash(JSON.stringify(stdout)));return stdout;});',
    ' return witness.allow("runtime",async()=>{const{stdout,subject}=await measureBazelRuntimeRowV1({root,runtime,environment:original,run});witness.observe("x",subject,hash(JSON.stringify(stdout)));return stdout;});')
sub('''  runtime:async(subject:string,expected:string)=>{
   const[runtime,modeName]=subject.split("\\0"),task=Object.values(planned.tasks)[0];if(!runtime||!task)return false;
   const original=issueBazelTaskEnvironmentV1(root,invocation,task,bazelTaskBaseEnvironmentV1(root,task,process.env));
   if(String(original.SEMIO_BUILD_MODE)!==modeName)return false;
   return hash(JSON.stringify(await runBazelRuntimeCommandV1(root,runtime,original,phaseControl)))===expected;
  },''','''  runtime:async(subject:string,expected:string)=>{
   const task=Object.values(planned.tasks)[0];if(!task)return false;
   const original=issueBazelTaskEnvironmentV1(root,invocation,task,bazelTaskBaseEnvironmentV1(root,task,process.env));
   return checkBazelRuntimeRowV1({root,subject,expected,environment:original,run:runtime=>runBazelRuntimeCommandV1(root,runtime,original,phaseControl),digest:stdout=>hash(JSON.stringify(stdout))});
  },''')
open(path, "w", encoding="utf8").write(s)
print("edited")
