import { readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
const workspace="C:/git/semio",ticket=workspace+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/PRINT-VISUALIZATION-LIBRARY";
const canvas=createRequire(workspace+"/node_modules/pdfjs-dist/legacy/build/pdf.mjs")("@napi-rs/canvas");
(globalThis as any).DOMMatrix??=canvas.DOMMatrix;
const {getDocument,OPS,Util}=await import(workspace+"/node_modules/pdfjs-dist/legacy/build/pdf.mjs");
const path=ticket+"/🗑️generated/print-pipeline/.gallery-carrier-qGHM6B/en-light/out/carrier.pdf";
const pdf=await getDocument({data:new Uint8Array(readFileSync(path))}).promise,rows=[];
for(let number=1;number<=pdf.numPages;number++){
 const page=await pdf.getPage(number),ops=await page.getOperatorList(),text=await page.getTextContent({includeMarkedContent:true});
 let matrix=[1,0,0,1,0,0];const stack:number[][]=[],wide=[];
 for(const[index,id]of ops.fnArray.entries()){const args=ops.argsArray[index];if(id===OPS.save)stack.push([...matrix]);else if(id===OPS.restore)matrix=stack.pop()!;else if(id===OPS.transform)matrix=Util.transform(matrix,args);else if(id===OPS.constructPath&&args[2]){const b=args[2],points=[[b[0],b[1]],[b[2],b[1]],[b[2],b[3]],[b[0],b[3]]];points.forEach(point=>Util.applyTransform(point,matrix));const bounds=[Math.min(...points.map(p=>p[0])),Math.max(...points.map(p=>p[0])),Math.min(...points.map(p=>p[1])),Math.max(...points.map(p=>p[1]))];if(bounds[1]-bounds[0]>300)wide.push({paint:args[0],bounds});}}
 rows.push({page:number,wide,markers:ops.fnArray.flatMap((id:number,index:number)=>[OPS.beginMarkedContent,OPS.beginMarkedContentProps,OPS.endMarkedContent].includes(id)?[{id,args:ops.argsArray[index]}]:[]),textMarkers:text.items.filter((item:any)=>!('str'in item)),paths:ops.fnArray.flatMap((id:number,index:number)=>id===OPS.constructPath?[{paint:ops.argsArray[index][0],bounds:ops.argsArray[index][2]}]:[])});
}
writeFileSync(ticket+"/🗑️generated/gallery-carrier-inspection/operators.json",JSON.stringify(rows,null,2));
console.log(JSON.stringify(rows.map(row=>({page:row.page,wide:row.wide})),null,2));
await pdf.destroy();
