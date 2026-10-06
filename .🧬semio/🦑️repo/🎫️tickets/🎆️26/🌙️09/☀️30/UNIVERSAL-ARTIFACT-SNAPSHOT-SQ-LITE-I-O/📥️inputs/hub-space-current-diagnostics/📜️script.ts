import {readFileSync,writeFileSync} from "node:fs";
import {resolve,join} from "node:path";
const ticket=resolve(import.meta.dir,"../..");
const scope=process.argv[2]??"I5rped";
const source=join(ticket,"🗑️generated/physical-hub-space-census-runtime/exact-cargo-laws-"+scope+"/00/build.stdout");
const errors=readFileSync(source,"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line)).filter(item=>item.reason==="compiler-message"&&item.message.level==="error").map(item=>({code:item.message.code?.code??null,message:item.message.message,spans:item.message.spans.filter(span=>span.is_primary).map(span=>({path:span.file_name,line:span.line_start,label:span.label})),rendered:item.message.rendered}));
writeFileSync(join(ticket,"🗑️generated/physical-hub-space-current-"+scope+"-diagnostics.json"),JSON.stringify(errors,null,2)+"\n");
const groups=new Map<string,{count:number,first:typeof errors[number]}>();for(const error of errors){const key=(error.code??"macro")+" "+error.message;const group=groups.get(key);if(group)group.count++;else groups.set(key,{count:1,first:error});}
const text="# Hub Space Current Compiler Ownership Census\n\nActual registered Native scope `exact-cargo-laws-"+scope+"/00` compiled dependencies then stopped before assertions in `semio-hub-space` lib-test. This report describes actual compiler diagnostics, with no runtime or missing provider inference. Full diagnostic records remain in generated output.\n\n"+[...groups].map(([message,group])=>"- "+group.count+" × `"+message.replaceAll("`","'")+"` — "+group.first.spans.map(span=>"`"+span.path+":"+span.line+"`").join(", ")).join("\n")+"\n";
writeFileSync(join(ticket,scope==="I5rped"?"📓️hub-space-current-compiler-ownership-census.md":"📓️hub-space-current-"+scope+"-compiler-ownership-census.md"),text);
console.log("[DEBUG] actual Hub Space compiler ownership census errors="+errors.length+" groups="+groups.size);
for(const [message,group]of groups)console.log(group.count+" "+message+" "+group.first.spans.map(span=>span.path+":"+span.line).join(","));
