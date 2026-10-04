import {readFileSync} from 'node:fs';import {getDocument,OPS} from 'pdfjs-dist/legacy/build/pdf.mjs';
const pdf=await getDocument({data:new Uint8Array(readFileSync(import.meta.dir+'/../axis-visual-after/🧪️probe-out/after.pdf'))}).promise;const page=await pdf.getPage(2),ops=await page.getOperatorList();const names=new Map(Object.entries(OPS).map(([name,id])=>[id,name]));
for(let i=0;i<ops.fnArray.length;i++){const name=names.get(ops.fnArray[i]!);if(['setLineWidth','setStrokeRGBColor','setGState','constructPath','stroke','fillStroke'].includes(name!))console.log('[DEBUG] '+name+' '+JSON.stringify(ops.argsArray[i]));}
await pdf.destroy();
console.log('[DEBUG] OPS stroke '+OPS.stroke+' paths '+OPS.constructPath);