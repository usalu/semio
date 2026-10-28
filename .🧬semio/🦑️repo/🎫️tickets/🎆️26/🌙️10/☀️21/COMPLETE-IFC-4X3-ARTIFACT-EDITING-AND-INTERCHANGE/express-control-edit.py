from pathlib import Path
p=Path("C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/population.ts")
s=p.read_text(encoding="utf-8")
s=s.replace("import {compileIfc4x3Express} from './compiler.ts';","import {compileIfc4x3Express,type ExpressCompilation} from './compiler.ts';\nimport {captureExpressControlScope} from './control.ts';",1)
head,body=s.split("export async function validateIfc4x3Express(snapshot:IfcSnapshot,control:ExpressControl={},program?:ExpressProgram):Promise<IfcExpressValidation>{",1)
assert body.count("catch(error){")==8,body.count("catch(error){")
body=body.replace("catch(error){","catch(error){scope.rethrowCallerFailure();")
assert body.count("return{complete:false,valid:false,")==10,body.count("return{complete:false,valid:false,")
body=body.replace("return{complete:false,valid:false,","return outcome({complete:false,valid:false,")
assert body.count(",results,work:0};")==10,body.count(",results,work:0};")
body=body.replace(",results,work:0};",",results,work:0});")
old=" const compilation=program?{program,complete:true,diagnostics:[]} :await compileIfc4x3Express(control);const diagnostics:"
new=""" const scope=captureExpressControlScope(control);control=scope.control;
 const cancelled=(report:IfcExpressValidation):IfcExpressValidation=>({...report,complete:false,valid:false,diagnostics:report.diagnostics.some(row=>row.code==='express.cancelled')?report.diagnostics:[...report.diagnostics,{code:'express.cancelled',message:'EXPRESS validation cancelled'}]});
 const outcome=async(report:IfcExpressValidation):Promise<IfcExpressValidation>=>{scope.rethrowCallerFailure();if(control.signal?.aborted)return cancelled(report);try{await scope.checkpoint({phase:'validation-result',completed:1,total:1});}catch(error){scope.rethrowCallerFailure();if(!control.signal?.aborted)throw error;return cancelled(report);}return report;};
 let compilation:ExpressCompilation;try{compilation=program?{program,complete:true,diagnostics:[]}:await compileIfc4x3Express(control);}catch(error){scope.rethrowCallerFailure();return outcome({complete:false,valid:false,diagnostics:[{code:'express.compile.'+(error instanceof ExpressEvaluationError?error.code:'admission'),message:String(error)}],results:[],work:0});}
 const diagnostics:"""
assert old in body
body=body.replace(old,new,1)
old="return{complete,valid:complete&&!diagnostics.some(item=>['express.rule','express.unique','express.inverse','express.bound'].includes(item.code)),diagnostics,results,work:evaluator.work};"
assert old in body
body=body.replace(old,"return outcome({complete,valid:complete&&!diagnostics.some(item=>['express.rule','express.unique','express.inverse','express.bound'].includes(item.code)),diagnostics,results,work:evaluator.work});",1)
body=body.replace("catch(error){scope.rethrowCallerFailure();complete=false;","catch(error){scope.check();complete=false;",4)
body=body.replace("}catch(error){scope.check();complete=false;diagnostics.push({code:`express.","}catch(error){scope.rethrowCallerFailure();complete=false;diagnostics.push({code:`express.")
body=body.replace("const scope=captureExpressControlScope(control);control=scope.control;","const controlScope=captureExpressControlScope(control);control=controlScope.control;").replace("scope.rethrowCallerFailure()","controlScope.rethrowCallerFailure()").replace("scope.checkpoint(","controlScope.checkpoint(").replace("scope.check()","controlScope.check()")
p.write_text(head+"export async function validateIfc4x3Express(snapshot:IfcSnapshot,control:ExpressControl={},program?:ExpressProgram):Promise<IfcExpressValidation>{"+body,encoding="utf-8")
