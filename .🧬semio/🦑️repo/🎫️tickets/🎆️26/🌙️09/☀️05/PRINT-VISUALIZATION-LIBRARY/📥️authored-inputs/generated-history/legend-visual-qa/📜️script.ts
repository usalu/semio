import {readFileSync} from 'node:fs';
import {getDocument} from 'pdfjs-dist/legacy/build/pdf.mjs';
const pdf=await getDocument({data:new Uint8Array(readFileSync(import.meta.dir+'/'+(process.argv[2]??'legend-proof.pdf'))),useSystemFonts:true}).promise;
for(let page=1;page<=pdf.numPages;page++){const proxy=await pdf.getPage(page);await proxy.getOperatorList();const text=await proxy.getTextContent(),items=text.items.flatMap(item=>'str' in item?[item]:[]);console.log(JSON.stringify({page,text:items.map(item=>item.str).join(' '),fonts:items.filter(item=>['Key','Legende','AV','Äpfel'].includes(item.str)).map(item=>({text:item.str,font:item.fontName,style:text.styles[item.fontName],face:proxy.commonObjs.get(item.fontName)?.name}))}));}
await pdf.destroy();
