import {readFileSync,writeFileSync} from "node:fs";
const path=process.argv[2]!,source=readFileSync(path,"utf8");
let changed=source;
const signatures=[
 ["pump_renderer_asset_decode_step","RendererAssetDecodeTurnKind","RendererAssetDecodeTurnKind::Pending"],
 ["close_renderer_asset_probe_step","bool","false"],
 ["close_component_asset_step","bool","infinite_world::world::close_world3d_asset_step(&mut state.assets)"],
];
for(const[name,result,tail]of signatures){
 const marker="fn "+name+"(",at=changed.indexOf(marker);if(at<0)throw Error("missing function "+name);
 const open=changed.indexOf("{",at),close=changed.indexOf("\n    }\n",open);if(close<0)throw Error("missing body end "+name);
 const signature=changed.slice(at,open),body=changed.slice(open+1,close);
 if(signature.includes("StepContext")){console.log("already migrated "+name);continue;}
 let replacement=body,edits:{start:number;end:number;value:string}[]=[];
 let quoted=false,escaped=false,comment=false;
 for(let i=0;i<body.length;i++){
  const char=body[i]!;
  if(comment){if(char==="\n")comment=false;continue;}
  if(quoted){if(escaped)escaped=false;else if(char==="\\")escaped=true;else if(char==='"')quoted=false;continue;}
  if(char==='"'){quoted=true;continue;}
  if(body.slice(i,i+2)==="//"){comment=true;continue;}
  if(!body.startsWith("return ",i))continue;
  const start=i+7;let j=start,depth=0,string=false,escape=false;
  for(;j<body.length;j++){
   const token=body[j]!;
   if(string){if(escape)escape=false;else if(token==="\\")escape=true;else if(token==='"')string=false;continue;}
   if(token==='"'){string=true;continue;}
   if(depth===0&&(token===";"||token==="}"||token===","))break;
   if("([{".includes(token))depth++;else if(")]}".includes(token))depth--;
  }
  if(j===body.length)throw Error("unterminated return "+name);
  const expression=body.slice(start,j).trim();edits.push({start,end:j,value:"Ok("+expression+")"});i=j-1;
 }
 for(const edit of edits.reverse())replacement=replacement.slice(0,edit.start)+edit.value+replacement.slice(edit.end);
 const offset=replacement.lastIndexOf(tail);if(offset<0||replacement.slice(offset+tail.length).trim())throw Error("unexpected tail "+name);
 replacement=replacement.slice(0,offset)+"Ok("+tail+")"+replacement.slice(offset+tail.length);
 replacement=replacement.replaceAll("probe.close_step()","probe.close_step(cx)?");
 const args=signature.slice(signature.indexOf("(")+1,signature.indexOf(")"));
 const updated=signature.replace("("+args+")","("+args+", cx: &mut semio_framework_job::StepContext<'_>)").replace("-> "+result,"-> Result<"+result+", semio_framework_job::OriginalTurnFailure>");
 changed=changed.slice(0,at)+updated+"{"+replacement+changed.slice(close);console.log(name+" returns="+edits.length);
}
if(!process.argv.includes("--write"))throw Error("supply --write after reviewing the exact bounded function transform");
writeFileSync(path,changed);
