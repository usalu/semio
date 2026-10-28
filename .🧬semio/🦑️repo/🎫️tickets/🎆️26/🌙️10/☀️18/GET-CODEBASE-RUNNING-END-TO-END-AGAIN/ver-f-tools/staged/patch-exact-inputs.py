"""usage: patch-exact-inputs.py <exact inputs test file> — adapts the consumed-input law to the current exact Cargo driver: `execute` reads cargoPlanningOutputBytesV1 and compileUnitsForHostV1, and the build lease now wraps the whole `execute` (lockc-d hoist), so the harness extracts `leased` + `execute`, binds the outer args/env, expects the lease on the phase control and closes it after the receipt."""
import glob,os,sys
p=os.path.abspath(sys.argv[1]);s=open(p).read()
root=os.path.abspath("🧰️framework/🔨️modules/🏃️process")
capture=[c for c in glob.glob(root+"/*/*native-build/*units/*capture/*.ts") if "tests" not in c]
assert len(capture)==1,capture
spec=os.path.relpath(capture[0],os.path.dirname(p)) if p.startswith(root+"/") else capture[0]
anchor='import{compilerAdmissionBudgetV1}from'
i=s.index(anchor);j=s.index("\n",i)
s=s[:j]+'\nimport{cargoPlanningOutputBytesV1}from"'+spec+'";'+s[j:]

def swap(old,new):
    global s
    assert s.count(old)==1,(old,s.count(old))
    s=s.replace(old,new)

swap('compilerAdmissionBudgetV1,next:row.stage,','compilerAdmissionBudgetV1,cargoPlanningOutputBytesV1,compileUnitsForHostV1:async()=>({request:{}}),args:corpus.arguments,env:corpus.environment,next:row.stage,')
swap('bodies:ts.ArrowFunction[]=[]','bodies:{name:string;fn:ts.ArrowFunction}[]=[]')
swap('if(ts.isVariableDeclaration(node)&&node.name.getText(tree)==="execute"&&node.initializer&&ts.isArrowFunction(node.initializer))bodies.push(node.initializer);','if(ts.isVariableDeclaration(node)&&(node.name.getText(tree)==="execute"||node.name.getText(tree)==="leased")&&node.initializer&&ts.isArrowFunction(node.initializer))bodies.push({name:node.name.getText(tree),fn:node.initializer});')
swap('expect(bodies).toHaveLength(1);','expect(bodies.map(row=>row.name)).toEqual(["leased","execute"]);')
swap('ts.transpileModule("const execute="+bodies[0]!.getText(tree),','ts.transpileModule(bodies.map(row=>"const "+row.name+"="+row.fn.getText(tree)+";").join("\\n"),')
swap('const execute=Function(...Object.keys(bindings),code+";return execute;")(...Object.values(bindings));return execute({arguments:corpus.arguments,environment:corpus.environment,control:actualControl});','const{execute,leased}=Function(...Object.keys(bindings),code+";return {execute,leased};")(...Object.values(bindings)),consumed={arguments:corpus.arguments,environment:corpus.environment,control:actualControl};return row.stage==="build"?leased(()=>execute(consumed)):execute(consumed);')
swap('expect(request.control).toBe(actualControl);expect(request.signal).toBe(actualControl.signal);return{release:()=>{events.push("lease-close")','expect(request.control).toBe(phaseControl);expect(request.signal).toBe(phaseControl.signal);return{release:()=>{events.push("lease-close")')
swap('["plan","source","probe","capacity-close","lease-close","receipt"]','["plan","source","probe","capacity-close","receipt","lease-close"]')
open(p,'w').write(s)
