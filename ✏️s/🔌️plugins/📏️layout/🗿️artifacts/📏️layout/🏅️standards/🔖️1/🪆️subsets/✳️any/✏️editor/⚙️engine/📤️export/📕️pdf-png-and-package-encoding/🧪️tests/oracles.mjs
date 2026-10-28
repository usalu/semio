import {createRequire} from 'node:module';
import {pathToFileURL} from 'node:url';
import {Blob} from 'node:buffer';
import * as streams from 'node:stream/web';
const require=createRequire(import.meta.url);
let pdfJs;
async function reader(){
 if(pdfJs)return pdfJs;
 const canvas=require('@napi-rs/canvas');
 for(const key of ['DOMMatrix','ImageData','Path2D'])if(typeof globalThis[key]==='undefined')globalThis[key]=canvas[key];
 if(typeof globalThis.navigator==='undefined')Object.defineProperty(globalThis,'navigator',{value:{userAgent:'Node',platform:'Node',language:'en-US'},configurable:true});
 if(typeof globalThis.Blob==='undefined')globalThis.Blob=Blob;
 for(const [key,value]of Object.entries(streams))if(typeof globalThis[key]==='undefined')globalThis[key]=value;
 pdfJs=await import('pdfjs-dist/legacy/build/pdf.mjs');pdfJs.GlobalWorkerOptions.workerSrc=pathToFileURL(require.resolve('pdfjs-dist/legacy/build/pdf.worker.mjs')).href;return pdfJs;
}
/** 🔬️ Independent image library stays behind an owned pixel-plane interface. */
export async function rasterPlane(source,plane){const image=require('sharp')(source.rgba,{raw:{width:source.width,height:source.height,channels:4}});return await(plane==='rgb'?image.removeAlpha():image.extractChannel('alpha')).raw().toBuffer();}
/** 📖️ Independent PDF viewer observes emitted Unicode CMaps through a minimal standard witness document. */
async function unicodeObservation(unicode,encoding,encoded,scope=''){const pdfJs=await reader();
const ops=scope+'BT /F1 12 Tf 3 Tr 1 0 0 1 10 50 Tm <'+encoded+'> Tj ET'+(scope?' EMC':'');const objects=['<< /Type /Catalog /Pages 2 0 R >>','<< /Type /Pages /Kids [3 0 R] /Count 1 >>','<< /Type /Page /Parent 2 0 R /MediaBox [0 0 800 100] /Resources << /Font << /F1 4 0 R >> >> /Contents 8 0 R >>','<< /Type /Font /Subtype /Type0 /BaseFont /LayoutSans /Encoding 7 0 R /DescendantFonts [5 0 R] /ToUnicode 6 0 R >>','<< /Type /Font /Subtype /CIDFontType2 /BaseFont /LayoutSans /CIDSystemInfo << /Registry (Adobe) /Ordering (Identity) /Supplement 0 >> /FontDescriptor 9 0 R /DW 1000 /CIDToGIDMap /Identity >>','<< /Length '+unicode.length+' >>\nstream\n'+unicode+'\nendstream','<< /Length '+encoding.length+' >>\nstream\n'+encoding+'\nendstream','<< /Length '+ops.length+' >>\nstream\n'+ops+'\nendstream','<< /Type /FontDescriptor /FontName /LayoutSans /Flags 32 /FontBBox [0 -200 1000 1000] /ItalicAngle 0 /Ascent 900 /Descent -200 /CapHeight 700 /StemV 80 >>'];let pdf='%PDF-1.4\n';const offsets=[0];objects.forEach((object,index)=>{offsets.push(pdf.length);pdf+=(index+1)+' 0 obj\n'+object+'\nendobj\n';});const xref=pdf.length;pdf+='xref\n0 '+offsets.length+'\n0000000000 65535 f \n'+offsets.slice(1).map(offset=>offset.toString().padStart(10,'0')+' 00000 n \n').join('')+'trailer\n<< /Root 1 0 R /Size '+offsets.length+' >>\nstartxref\n'+xref+'\n%%EOF\n';const document=await pdfJs.getDocument({data:new Uint8Array(Buffer.from(pdf)),disableFontFace:true,isEvalSupported:false}).promise;try{const content=await(await document.getPage(1)).getTextContent({includeMarkedContent:true});return{text:content.items.filter(item=>'str'in item).map(item=>item.str).join(''),tags:content.items.filter(item=>item.type==='beginMarkedContentProps').map(item=>item.tag)};}finally{await document.destroy();}
}

/** 📖️ Independent viewer observes actual Unicode text. */
export async function unicodeText(unicode,encoding,encoded){return(await unicodeObservation(unicode,encoding,encoded)).text;}
/** 🏷️ Independent viewer observes valid marked-content scopes and unchanged text. */
export async function markedText(unicode,encoding,encoded,scope){return await unicodeObservation(unicode,encoding,encoded,scope);}
/** 🔠️ Independent encoding library observes authored font-family PDF string bytes. */
export function fontFamilyHex(family){return require('iconv-lite').encode(family,'utf16-be').toString('hex').toUpperCase();}
