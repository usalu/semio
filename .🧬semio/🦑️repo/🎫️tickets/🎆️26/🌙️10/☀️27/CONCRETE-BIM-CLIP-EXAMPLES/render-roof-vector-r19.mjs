/** 🧭 Inspect the primary roof linework directly from decoded PDF path operators. */
import {readFile,writeFile} from 'node:fs/promises';
import {createRequire} from 'node:module';
export async function renderRoofVector(root,ticket) {
 const require=createRequire(root+'/package.json'),{createCanvas}=require('@napi-rs/canvas');
 const data=JSON.parse(await readFile(ticket+'/🗑️generated/roof-vector-r19-page2.json','utf8'));
 const canvas=createCanvas(870,930),ctx=canvas.getContext('2d');ctx.fillStyle='white';ctx.fillRect(0,0,870,930);ctx.setTransform(6,0,0,-6,-5400,3300);
 for(let i=0;i<data.fnArray.length;i++){
  const name=data.fnArray[i],a=data.argsArray[i];
  if(name==='save')ctx.save();else if(name==='restore')ctx.restore();else if(name==='transform')ctx.transform(...a);
  else if(name==='setLineWidth')ctx.lineWidth=a[0];else if(name==='setDash')ctx.setLineDash(a[0]);
  else if(name==='setFillRGBColor')ctx.fillStyle=typeof a[0]==='string'?a[0]:'#222';
  else if(name==='setStrokeRGBColor')ctx.strokeStyle=typeof a[0]==='string'?a[0]:'#222';
  else if(name==='constructPath'){
   ctx.beginPath();for(const path of a[1])for(let j=0;j<path.length;){const op=path[j++];if(op===0)ctx.moveTo(path[j++],path[j++]);else if(op===1)ctx.lineTo(path[j++],path[j++]);else if(op===2){ctx.bezierCurveTo(...path.slice(j,j+6));j+=6;}else if(op===3){ctx.quadraticCurveTo(...path.slice(j,j+4));j+=4;}else if(op===4)ctx.closePath();else throw new Error('Unknown PDF path opcode '+op);}
   if([21,26,27].includes(a[0]))ctx.closePath();if([22,24,26].includes(a[0]))ctx.fill();if([23,25,27].includes(a[0]))ctx.fill('evenodd');if([20,21,24,25,26,27].includes(a[0]))ctx.stroke();
  }
 }
 await writeFile(ticket+'/🗑️generated/roof-primary-vector-r19.png',canvas.toBuffer('image/png'));
}
                                                                             