import {readFileSync} from 'node:fs';import {join} from 'node:path';import {getDocument,OPS} from 'pdfjs-dist/legacy/build/pdf.mjs';import {rgb} from 'd3-color';
import {compileVizProbeDocument} from '../../../../../../../../../🧰️framework/🛍️products/📓️print/🔨️modules/🧪️viz-probe/🟦️.ts';
import {inferVizChart} from '../../../../../../../../../🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🟦️.ts';
import {legendSpecifications} from '../../../../../../../../../🧰️framework/🛍️products/📓️print/🧪️tests/🎬️render-scene/🧭️legend/🟦️.ts';
const fixture=JSON.parse(readFileSync(join(import.meta.dir,'../../../../../../../../../🧰️framework/🛍️products/📓️print/🧪️tests/🎬️render-scene/🔣️legend.json'),'utf8'));
let count=0;
for(const appearance of ['light','dark']){
 const entries=legendSpecifications().filter(entry=>entry.id.startsWith('native-foreground')&&entry.spec.theme?.appearance===appearance),body:{raw:string}[]=[],expected:string[]=[];
 for(const entry of entries){const guide=entry.spec.guides![0]!,o={scale:guide.scale,...guide.options,title:guide.title!.en},result=await inferVizChart({chart:entry.spec});if(!result.complete)throw Error(JSON.stringify(result.diagnostics));
  const scales=fixture.scales.map((scale:any)=>'\\SemioVizScale{'+scale.name+'}{'+scale.kind+'}{'+scale.domain.join(',')+'}{'+scale.range.join(',')+'}').join(''),keys=Object.entries(o).map(([key,value])=>key+'={'+value+'}').join(',');
  body.push({raw:'Native '+entry.id+'\\par\\begin{VizFigure}[width=100,height=60]'+scales+'\\SemioVizLegend['+keys+']\\end{VizFigure}\\par Canonical '+entry.id+'\\par'+result.tikz});
  expected.push(...Array(6).fill(fixture.cases.find((row:any)=>row.id===entry.id).expectedTextFill));
 }
 const id='legend-foreground-'+appearance,workDir=join(import.meta.dir,"../../🗑️generated/legend-text-paint",appearance);
 await compileVizProbeDocument({case:'legend-text-paint',scenario:id,documentClass:'semio',documentClassOptions:'type=paper,language=en,theme='+appearance,packages:['semio-viz'],geometry:true,preamble:['\\title{Legend Paint Proof}','\\author{Semio}','\\date{}'],body},{workDir,keepWorkDir:true});
 const pdf=await getDocument({data:new Uint8Array(readFileSync(join(workDir,'🧪️probe-out',id+'.pdf')))}).promise,actual:{text:string;color:string;face:string}[]=[];
 try{for(let page=1;page<=pdf.numPages;page++){
  const proxy=await pdf.getPage(page),ops=await proxy.getOperatorList(),stack:{color:string;font:string}[]=[];let state={color:'#000000',font:''};
  for(let i=0;i<ops.fnArray.length;i++){const fn=ops.fnArray[i],args=ops.argsArray[i];
   if(fn===OPS.save)stack.push({...state});else if(fn===OPS.restore)state=stack.pop()??state;
   else if(fn===OPS.setFillRGBColor)state.color=typeof args[0]==='string'?rgb(args[0]).formatHex():rgb(Number(args[0]),Number(args[1]),Number(args[2])).formatHex();
   else if(fn===OPS.setFont)state.font=String(args[0]);
   else if(fn===OPS.showText){const text=args[0].map((glyph:any)=>typeof glyph==='number'?'':glyph.unicode??'').join('');if(['Palette','bb','ccc'].includes(text))actual.push({text,color:state.color,face:(proxy.commonObjs.get(state.font) as {name:string}).name});}
  }
 }}finally{await pdf.destroy();}
 console.log('[DEBUG] actual native/canonical legend label/title paints '+JSON.stringify({appearance,expected,actual}));
 if(actual.length!==expected.length||actual.some((item,index)=>item.color!==expected[index]||!item.face.endsWith('+'+(item.text==='Palette'?'ShareTechMono-Regular':'Anta-Regular'))))throw Error(appearance+' actual legend text paint/font mismatch');count+=actual.length;
}
console.log('[DEBUG] '+count+' actual legend label/title paints and font roles matched neutral light/dark/override controls');
