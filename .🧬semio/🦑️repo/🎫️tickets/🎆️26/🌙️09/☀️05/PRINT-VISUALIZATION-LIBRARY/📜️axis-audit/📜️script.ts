import {readFileSync,mkdirSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {createHash} from "node:crypto";
import {getDocument} from "pdfjs-dist/legacy/build/pdf.mjs";
import {line} from "d3-shape";
import {compileVizProbeDocument} from "./../../../../../../../../🧰️framework/🛍️products/📓️print/🔨️modules/🧪️viz-probe/🟦️.ts";
import {inferVizChart} from "./../../../../../../../../🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🟦️.ts";
import type {VizChartSpecification} from "./../../../../../../../../🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/📊️chart/🟦️.ts";
import fixture from "./../../../../../../../../🧰️framework/🛍️products/📓️print/🧪️tests/🎬️render-scene/🔣️axis.json";
const work=resolve(process.argv[2]!);mkdirSync(work,{recursive:true});
const ids=["axis-native-polar-default","axis-native-rotation-positive","axis-native-rotation-negative","axis-native-rotation-alias"],cases=fixture.cases.filter(entry=>ids.includes(entry.id));
const before=Object.fromEntries(["C:/git/semio/🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🖼️render/🟦️.ts","C:/git/semio/🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-guide.sty"].map(name=>[name,createHash("sha256").update(readFileSync(name)).digest("hex")]));
for(const theme of ["light","dark"]){
 const body:{raw:string}[]=[],results=new Map<string,Awaited<ReturnType<typeof inferVizChart>>>();
 for(const entry of cases){
  const spec={...entry.spec,theme:{appearance:theme}} as VizChartSpecification,result=await inferVizChart({chart:spec});if(!result.complete)throw Error(JSON.stringify(result.diagnostics));results.set(entry.id,result);
  const guide=spec.guides![0]!,scale=spec.scales![0]!,options={...guide.options},orient=String(options.orient??guide.orient??"bottom");delete options.orient;
  const title=guide.title?.[spec.language!],nativeOptions=Object.entries(options).map(([key,value])=>key+"={"+String(value)+"}");if(title!==undefined)nativeOptions.push("title={"+title+"}");
  const native="\\begin{VizFigure}[title={Native "+entry.id+"},width="+spec.width+",height="+spec.height+"]\\SemioVizScale{paired-axis}{"+scale.kind+"}{"+scale.domain.join(",")+"}{"+scale.range.join(",")+"}\\SemioVizProbeBegin{paired-axis}{"+entry.id+"}\\SemioVizAxis[scale=paired-axis,orient="+orient+",tickValues={"+guide.tickValues!.join(",")+"},"+nativeOptions.join(",")+"]\\end{VizFigure}";
  body.push({raw:"\\clearpage"+native+"\\par Canonical "+entry.id+"\\par "+result.tikz});
 }
 const root=join(work,theme),records=await compileVizProbeDocument({case:"paired-axis",scenario:"axes",documentClass:"semio",documentClassOptions:"type=paper,language=en,theme="+theme,packages:["semio-viz"],geometry:true,preamble:["\\title{Paired Native Axis Proof}","\\author{Semio}","\\date{}"],body},{workDir:root,scenario:undefined,keepWorkDir:true});
 let ticks=0,titles=0;
 for(const entry of cases){
  const actual=records.filter(record=>record.scenario===entry.id&&record.key==="geometry/axis-tick"),expected=entry.expectedLines;
  if(actual.length!==expected.length)throw Error(theme+"/"+entry.id+": native tick count differs");
  for(const [index,record]of actual.entries()){
   const values=record.values.map(Number),projected=[values[0]!,entry.spec.height-values[1]!,values[2]!,entry.spec.height-values[3]!];
   const oracle:number[]=[];line().context({moveTo:(x:number,y:number)=>oracle.push(x,y),lineTo:(x:number,y:number)=>oracle.push(x,y)} as never)([[expected[index]![0]!,expected[index]![1]!],[expected[index]![2]!,expected[index]![3]!]]);
   if(projected.some((value,dimension)=>Math.abs(value-oracle[dimension]!)>1e-4))throw Error(theme+"/"+entry.id+": compiled native/D3 tick discrepancy "+projected);ticks++;
  }
  const title=records.find(record=>record.scenario===entry.id&&record.key==="geometry/axis-title");if(title){const guide=(entry.spec as VizChartSpecification).guides![0]!,content=String(guide.options?.title??guide.title?.[entry.spec.language as "en"|"de"]),plan=results.get(entry.id)!.plan!.items.find(item=>item.kind==="text"&&item.content===content);if(!plan||plan.kind!=="text"||Math.abs(plan.x-Number(title.values[0]))>1e-4||Math.abs(plan.y-(entry.spec.height-Number(title.values[1])))>1e-4)throw Error("actual native/inferred title mismatch");titles++;}
 }
 const pdf=await getDocument({data:new Uint8Array(readFileSync(join(root,"\u{1f9ea}\uFE0Fprobe-out","axes.pdf"))),useSystemFonts:true}).promise;let text="",angles=0;for(let page=1;page<=pdf.numPages;page++){const content=(await(await pdf.getPage(page)).getTextContent()).items.filter(item=>"str"in item);text+=content.map(item=>"str"in item?item.str:"").join(" ");const entry=cases.find(entry=>entry.id.includes("rotation")&&content.some(item=>"str"in item&&item.str.includes(entry.id)));if(entry){const options=(entry.spec as VizChartSpecification).guides![0]!.options!,expected=Number(options.labelRotation??options.labelRotate),labels=content.filter(item=>"str"in item&&["0","10"].includes(item.str));if(labels.length!==4)throw Error(theme+"/"+entry.id+": expected four native/canonical PDF label transforms");for(const item of labels){if(!("transform"in item))throw Error("PDF label has no transform");const rotation=Math.atan2(item.transform[1]!,item.transform[0]!)*180/Math.PI;if(Math.abs(rotation-expected)>.01)throw Error(theme+"/"+entry.id+": compiled PDF angle "+rotation+" differs from native "+expected);angles++;}}}const pages=pdf.numPages;await pdf.destroy();for(const entry of cases)if(!text.includes(entry.id))throw Error("paired PDF omittedcase "+entry.id);
 console.log("[DEBUG] axis-proof "+theme+": "+ticks+" compiled native/D3 tick segments, "+titles+" native/inferred titles, "+pages+" PDF.js pages and "+angles+" actual text angles PASS");
}
const after=Object.fromEntries(Object.keys(before).map(name=>[name,createHash("sha256").update(readFileSync(name)).digest("hex")]));if(JSON.stringify(before)!==JSON.stringify(after))throw Error("Axis owner source changed during paired proof");writeFileSync(join(work,"proof.json"),JSON.stringify({before,after,cases:ids},null,2)+"\n");
