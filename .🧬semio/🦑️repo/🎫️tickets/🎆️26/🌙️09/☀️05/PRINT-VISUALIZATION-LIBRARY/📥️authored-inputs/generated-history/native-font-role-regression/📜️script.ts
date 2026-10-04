import {readFileSync,writeFileSync,renameSync} from 'node:fs';
const product='C:/git/semio/🧰️framework/🛍️products/📓️print/',temporary=import.meta.dir+'/source.tmp';
function edit(path:string,change:(source:string)=>string){const file=product+path;writeFileSync(temporary,change(readFileSync(file,'utf8')));renameSync(temporary,file);}
edit('🧪️tests/🎬️render-scene/🔣️legend.json',source=>source.replace('"width":100,"height":60,','"width":100,"height":60,\n  "nativeFontFaces":{"titles":["Legende","Key"],"labels":["AV","Äpfel"],"title":"ShareTechMono-Regular","label":"Anta-Regular"},'));
edit('🧪️tests/🧬️native-chart-grammar/🟦️.ts',source=>source.replace('import legendVectors','import {getDocument} from "pdfjs-dist/legacy/build/pdf.mjs";\nimport legendVectors').replace('  console.log("[native-grammar] "+count+" native legend geometry records','  await verifyNativeLegendFonts(join(workDir,"🧪️probe-out","legends.pdf"));\n  console.log("[native-grammar] "+count+" native legend geometry records')+`
/** 🔤️ Checks actual native and emitted PDF text faces against the neutral tracked font roles. */
export async function verifyNativeLegendFonts(path:string):Promise<void>{
  const pdf=await getDocument({data:new Uint8Array(readFileSync(path)),useSystemFonts:true}).promise,expected=legendVectors.nativeFontFaces,failures:string[]=[];let titles=0,labels=0;
  try{for(let page=1;page<=pdf.numPages;page++){
    const proxy=await pdf.getPage(page);await proxy.getOperatorList();
    for(const item of (await proxy.getTextContent()).items){
      if(!("str" in item))continue;
      const title=expected.titles.includes(item.str),label=expected.labels.includes(item.str);if(!title&&!label)continue;
      const face=(proxy.commonObjs.get(item.fontName) as {name?:string}).name??"",wanted=title?expected.title:expected.label;
      if(title)titles++;else labels++;if(!face.endsWith("+"+wanted)&&face!==wanted)failures.push("page "+page+" "+item.str+": "+face+" expected "+wanted);
    }
  }}finally{await pdf.destroy();}
  if(titles<4||labels<4)throw Error("native font role inventory is incomplete");
  if(failures.length)throw Error(failures.length+" native font role mismatches:\\n"+failures.join("\\n"));
  console.log("[native-grammar] "+titles+" title and "+labels+" label PDF text faces matched tracked native font roles");
}
`);
