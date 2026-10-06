import { readFileSync, writeFileSync, copyFileSync, renameSync, mkdirSync } from "node:fs";
import { createHash } from "node:crypto";
const ticket=process.env.SEMIO_TICKET_DIR!,root="C:/git/semio/🧰️framework/🛍️products/📓️print",input=ticket+"/📥️authored-inputs/gallery-intrinsic-carrier/sampling-source-contracts.json",rows=JSON.parse(readFileSync(input,"utf8")).keys;
const block=(text:string,name:string,offset:number)=>{const start=text.indexOf('"'+name+'": {',offset),open=text.indexOf('{',start);if(start<0)throw Error(name);let depth=0,quoted=false,escaped=false;for(let index=open;index<text.length;index++){const character=text[index];if(quoted){if(escaped)escaped=false;else if(character==='\\')escaped=true;else if(character==='"')quoted=false;}else if(character==='"')quoted=true;else if(character==='{')depth++;else if(character==='}'&&!--depth)return{start:open,end:index+1,text:text.slice(open,index+1)};}throw Error(name+" unbalanced");};
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const save=(path:string,before:string,after:string,name:string)=>{if(readFileSync(path,"utf8")!==before)throw Error("concurrent "+name);copyFileSync(path,ticket+"/📥️authored-inputs/gallery-intrinsic-carrier/sampling-"+name+"-before"+(name==='api'?'.tex':'.json'));const candidate=ticket+"/🗑️generated/gallery-sampling-contract/"+name+"-candidate";writeFileSync(candidate,after);renameSync(candidate,path);console.log("[DEBUG] "+name+" "+sha(before)+" -> "+sha(after));};
const schemaPath=root+"/🧬️schema/🔣️.json",schemaBefore=readFileSync(schemaPath,"utf8"),familyBlock=block(schemaBefore,"geo-hexbin",schemaBefore.indexOf('"x-semio-family-options"')),family=JSON.parse(familyBlock.text);
if(Object.keys(family.options).length!==13)throw Error("thirteen source controls expected");
for(const row of rows){if(family.options[row.key])throw Error("already declared "+row.key);const {key,setter,...spec}=row;family.options[key]=spec;}
const familyText=JSON.stringify(family,null,2).split('\n').map((line,index)=>index?'    '+line:line).join('\n'),schemaAfter=schemaBefore.slice(0,familyBlock.start)+familyText+schemaBefore.slice(familyBlock.end);
if(Object.keys(family.options).length!==20)throw Error("twenty source controls expected");
save(schemaPath,schemaBefore,schemaAfter,"schema");
const fixturePath=root+"/🎮️commands/🧪️print-pipeline-verification/🧫️fixtures/🔓️api-freshness.json",fixtureBefore=readFileSync(fixturePath,"utf8"),nativeBlock=block(fixtureBefore,"geo-hexbin",fixtureBefore.indexOf('"nativeContracts"')),native=JSON.parse(nativeBlock.text);
for(const row of rows){const {key,type,syntax,minimum,exclusiveMinimum,maximum}=row;native[key]={type,syntax,default:String(row.default),...(minimum!==undefined?{minimum}:{}),...(exclusiveMinimum!==undefined?{exclusiveMinimum}:{}),...(maximum!==undefined?{maximum}:{})};}
const fixtureAfter=fixtureBefore.slice(0,nativeBlock.start)+JSON.stringify(native,null,2).split('\n').map((line,index)=>index?'      '+line:line).join('\n')+fixtureBefore.slice(nativeBlock.end);
save(fixturePath,fixtureBefore,fixtureAfter,"fixture");
const apiPath=root+"/🧾️template/📊️viz-api/🔓️viz-api.tex",apiBefore=readFileSync(apiPath,"utf8"),sectionStart=apiBefore.indexOf('\\subsection{\\Key{geo-hexbin}}'),sectionEnd=apiBefore.indexOf('\\subsection{',sectionStart+15),section=apiBefore.slice(sectionStart,sectionEnd);
if(sectionStart<0||sectionEnd<0||section.includes('\\Key{gridWidth}'))throw Error("API family boundary");
const rowTex=rows.map((row:any)=>'  \\SemioTableRow{\\Key{'+row.key+'} & \\Key{'+row.type+'} & '+(String(row.default)===''?'---':'\\Key{'+row.default+'}')+' & \\ApiText{'+row.description.en+'}{'+row.description.de+'}}').join('\r\n');
const closing='\r\n}\r\n';if(section.split(closing).length!==2)throw Error("API table ending");
const afterSection=section.replace('\\ApiKeyScope{api-registry}','\\ApiKeyScope{api-registry}\r\n\\ApiScopeSource{geo-hexbin}{semio / viz / family / geo-hexbin}').replace(closing,'\r\n'+rowTex+closing);
const apiAfter=apiBefore.slice(0,sectionStart)+afterSection+apiBefore.slice(sectionEnd);save(apiPath,apiBefore,apiAfter,"api");
writeFileSync(ticket+"/📥️authored-inputs/gallery-intrinsic-carrier/sampling-api-row-delta.json",JSON.stringify({before:section,after:afterSection},null,2));
writeFileSync(ticket+"/📥️authored-inputs/gallery-intrinsic-carrier/sampling-family-after.json",JSON.stringify(family,null,2));
console.log("[DEBUG] Seven authored controls admitted; outside family/table/native-contract spans preserved.");
