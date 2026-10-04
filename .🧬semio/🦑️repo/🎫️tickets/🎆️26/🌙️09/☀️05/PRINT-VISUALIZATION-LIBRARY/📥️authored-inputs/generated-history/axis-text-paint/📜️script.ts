import {readFileSync} from 'node:fs';import {join} from 'node:path';import {getDocument,OPS} from 'pdfjs-dist/legacy/build/pdf.mjs';import {rgb} from 'd3-color';
const source=JSON.parse(readFileSync(join(import.meta.dir,'../../🔣️axis-style-proof-2026-10-04.json'),'utf8'));
const role=JSON.parse(readFileSync(join(import.meta.dir,'../../../../../../../../../🧰️framework/🛍️products/📓️print/🧪️tests/🎬️render-scene/🔣️axis.json'),'utf8'));
let count=0;
for(const entry of source.cases){
 const expected=role.cases.find((row:{id:string})=>row.id==='axis-native-theme-'+entry.theme).expectedTextFills[0];
 const pdf=await getDocument({data:new Uint8Array(readFileSync(join(import.meta.dir,'../axis-style-proof',entry.id,'🧪️probe-out',entry.id+'.pdf')))}).promise,actual:{text:string;color:string;opacity:number}[]=[];
 try{for(let page=1;page<=pdf.numPages;page++){
  const ops=await(await pdf.getPage(page)).getOperatorList(),stack:{color:string;opacity:number}[]=[];let state={color:'#000000',opacity:1};
  for(let i=0;i<ops.fnArray.length;i++){
   const fn=ops.fnArray[i],args=ops.argsArray[i];
   if(fn===OPS.save)stack.push({...state});else if(fn===OPS.restore)state=stack.pop()??state;
   else if(fn===OPS.setFillRGBColor)state.color=typeof args[0]==='string'?rgb(args[0]).formatHex():rgb(Number(args[0]),Number(args[1]),Number(args[2])).formatHex();
   else if(fn===OPS.setGState){for(const [key,value] of args[0])if(key==='ca')state.opacity=Number(value);}
   else if(fn===OPS.showText){const text=args[0].map((glyph:{unicode?:string}|number)=>typeof glyph==='number'?'':glyph.unicode??'').join('');if(['0','5','10'].includes(text))actual.push({text,...state});}
  }
 }}finally{await pdf.destroy();}
 console.log('[DEBUG] actual native/canonical axis label paint '+JSON.stringify({id:entry.id,expected,actual}));
 if(actual.length!==6||actual.some(item=>item.color!==expected||Math.abs(item.opacity-(entry.opacity??1))>0.00001))throw Error(entry.id+' actual text paint mismatch');count+=actual.length;
}
console.log('[DEBUG] '+count+' actual axis label colors/opacities matched independent neutral theme roles');
