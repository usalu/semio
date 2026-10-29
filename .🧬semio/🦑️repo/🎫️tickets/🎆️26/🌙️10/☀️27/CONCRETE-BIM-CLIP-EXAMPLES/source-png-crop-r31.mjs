import {readFile,writeFile} from 'node:fs/promises';
import {inflateSync,deflateSync} from 'node:zlib';
export async function cropPng(input,output,box,scale=1){
 const file=await readFile(input),w=file.readUInt32BE(16),h=file.readUInt32BE(20);
 if(file[24]!==8||file[25]!==2||file[28]!==0)throw Error('RGB8 noninterlaced PNG required');
 const idat=[];for(let p=8;p<file.length;){let n=file.readUInt32BE(p);if(file.toString('ascii',p+4,p+8)==='IDAT')idat.push(file.subarray(p+8,p+8+n));p+=n+12;}
 const raw=inflateSync(Buffer.concat(idat)),pixels=Buffer.alloc(w*h*3),stride=w*3;
 const paeth=(a,b,c)=>{let p=a+b-c,pa=Math.abs(p-a),pb=Math.abs(p-b),pc=Math.abs(p-c);return pa<=pb&&pa<=pc?a:pb<=pc?b:c;};
 for(let y=0;y<h;y++){let filter=raw[y*(stride+1)];for(let x=0;x<stride;x++){let a=x>=3?pixels[y*stride+x-3]:0,b=y?pixels[(y-1)*stride+x]:0,c=y&&x>=3?pixels[(y-1)*stride+x-3]:0;pixels[y*stride+x]=(raw[y*(stride+1)+x+1]+[0,a,b,Math.floor((a+b)/2),paeth(a,b,c)][filter])&255;}}
 const [x0,y0,sw,sh]=box,cw=sw*scale,ch=sh*scale,out=Buffer.alloc(ch*(cw*3+1));for(let y=0;y<ch;y++)for(let x=0;x<cw;x++)pixels.copy(out,y*(cw*3+1)+1+x*3,((Math.floor(y/scale)+y0)*w+x0+Math.floor(x/scale))*3,((Math.floor(y/scale)+y0)*w+x0+Math.floor(x/scale))*3+3);
 const table=Array.from({length:256},(_,i)=>{for(let k=0;k<8;k++)i=i&1?0xedb88320^(i>>>1):i>>>1;return i>>>0;});
 const chunk=(name,data)=>{let n=Buffer.from(name),crc=0xffffffff;for(let b of Buffer.concat([n,data]))crc=table[(crc^b)&255]^(crc>>>8);let result=Buffer.alloc(data.length+12);result.writeUInt32BE(data.length);n.copy(result,4);data.copy(result,8);result.writeUInt32BE((crc^0xffffffff)>>>0,result.length-4);return result;};
 const head=Buffer.alloc(13);head.writeUInt32BE(cw);head.writeUInt32BE(ch,4);head[8]=8;head[9]=2;
 await writeFile(output,Buffer.concat([file.subarray(0,8),chunk('IHDR',head),chunk('IDAT',deflateSync(out)),chunk('IEND',Buffer.alloc(0))]));
}
