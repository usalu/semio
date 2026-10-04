import {readFileSync} from 'node:fs';import {join} from 'node:path';import {getDocument,OPS} from 'pdfjs-dist/legacy/build/pdf.mjs';import {rgb} from 'd3-color';
import {compileVizProbeDocument} from '../../../../../../../../../🧰️framework/🛍️products/📓️print/🔨️modules/🧪️viz-probe/🟦️.ts';
import {inferVizChart} from '../../../../../../../../../🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🟦️.ts';
const fixture=JSON.parse(readFileSync(join(import.meta.dir,'../../🔣️axis-style-proof-2026-10-04.json'),'utf8'));
type Paint={width:number;color:string;opacity:number;length:number};
async function paints(path:string):Promise<Paint[]>{
 const pdf=await getDocument({data:new Uint8Array(readFileSync(path))}).promise,paths:Paint[]=[];
 try{for(let page=1;page<=pdf.numPages;page++){
  const ops=await(await pdf.getPage(page)).getOperatorList(),stack:Omit<Paint,'length'>[]=[];let state={width:1,color:'#000000',opacity:1};
  for(let index=0;index<ops.fnArray.length;index++){
   const fn=ops.fnArray[index],args=ops.argsArray[index];
   if(fn===OPS.save)stack.push({...state});else if(fn===OPS.restore)state=stack.pop()??state;
   else if(fn===OPS.setLineWidth)state.width=Number(args[0]);
   else if(fn===OPS.setStrokeRGBColor)state.color=typeof args[0]==='string'?rgb(args[0]).formatHex():rgb(Number(args[0]),Number(args[1]),Number(args[2])).formatHex();
   else if(fn===OPS.setGState){for(const [key,value] of args[0])if(key==='CA')state.opacity=Number(value);}
   else if(fn===OPS.constructPath&&args[0]===OPS.stroke){for(const raw of args[1]??[]){if(!raw)continue;const data=Array.from(raw) as number[];if(data.length===6&&data[0]===0&&data[3]===1)paths.push({...state,length:Math.hypot(data[4]!-data[1]!,data[5]!-data[2]!)*25.4/72});}}
  }
 }}finally{await pdf.destroy();}return paths;
}
if(process.argv[2]==='baseline'){
 const before=await paints(join(import.meta.dir,'../../🗑️generated/axis-visual-after/🧪️probe-out/after.pdf'));const actual=before.filter(path=>Math.abs(path.length-fixture.majorLengthMm)<0.001);
 const failures=actual.filter(path=>path.color!=='#001117'||Math.abs(path.width-.75*72/72.27)>0.0001);console.log('[DEBUG] actual PDF axis stroke baseline '+JSON.stringify({count:actual.length,failures}));if(failures.length||actual.length!==18)throw Error('actual PDF default-axis paint parity failed');
}else for(const entry of fixture.cases.filter((entry:{id:string})=>!process.argv[2]||entry.id===process.argv[2])){
 const o={...entry.options,tickSize:fixture.majorLengthMm},spec={width:fixture.frame[0],height:fixture.frame[1],language:'en',theme:{appearance:entry.theme},margin:{left:8,right:2,top:2,bottom:8},tables:[],layers:[],scales:[{name:'x',kind:'linear',domain:[0,10],range:[10,90]}],guides:[{kind:'axis',scale:'x',orient:'bottom',tickValues:[0,5,10],options:o}]};
 const result=await inferVizChart({chart:spec as never});if(!result.complete)throw Error(JSON.stringify(result.diagnostics));
 const keys=Object.entries(o).map(([key,value])=>key+'={'+value+'}').join(','),native='\\begin{VizFigure}[width=100,height=80]\\SemioVizScale{x}{linear}{0,10}{10,90}\\SemioVizAxis[scale=x,orient=bottom,tickValues={0,5,10},'+keys+']\\end{VizFigure}';
 const workDir=join(import.meta.dir,"../../🗑️generated/axis-style-proof",entry.id);await compileVizProbeDocument({case:'axis-style',scenario:entry.id,documentClass:'semio',documentClassOptions:'type=paper,language=en,theme='+entry.theme,packages:['semio-viz'],geometry:true,preamble:['\\title{Axis Style Proof}','\\author{Semio}','\\date{}'],body:[{raw:'Native '+entry.id+'\\par'+native+'\\par Canonical '+entry.id+'\\par'+result.tikz}]},{workDir,keepWorkDir:true});
 const actual=await paints(join(workDir,'🧪️probe-out',entry.id+'.pdf')),major=actual.filter(path=>Math.abs(path.length-fixture.majorLengthMm)<0.001),width=entry.widthMm===undefined?entry.widthTeXPoints*72/72.27:entry.widthMm*72/25.4;
 if(major.length!==6||major.some(path=>path.color!==entry.color||Math.abs(path.width-width)>0.0001||Math.abs(path.opacity-(entry.opacity??1))>0.00001))throw Error(entry.id+' actual major paint mismatch '+JSON.stringify(major));
 if(entry.options.minor){const minor=actual.filter(path=>Math.abs(path.length-fixture.minorLengthMm)<0.001),grid=actual.filter(path=>Math.abs(path.length-fixture.gridLengthMm)<0.001);if(minor.length!==12||minor.some(path=>path.color!==entry.color||Math.abs(path.width-width)>0.0001||Math.abs(path.opacity-.6)>0.00001))throw Error(entry.id+' actual minor paints '+JSON.stringify(minor));if(grid.length!==6||grid.some(path=>path.color!=='#0000ff'||Math.abs(path.width-width)>0.0001||Math.abs(path.opacity-.45)>0.00001))throw Error(entry.id+' actual grid paints '+JSON.stringify(grid));}
 console.log('[DEBUG] actual native/canonical PDF axis styles '+JSON.stringify({id:entry.id,major}));
}