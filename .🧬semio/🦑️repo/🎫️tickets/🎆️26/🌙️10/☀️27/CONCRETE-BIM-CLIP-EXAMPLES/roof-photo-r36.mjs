/** 🖼️ Extract inspected archival photo objects through an authoring-only image codec interface. */
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {join} from 'node:path';
export async function extractPhotoImages({source,targetDirectory,ids,decodeImage,encodeImage,invert=false}) {
 const pdf=(await readFile(source)).toString('latin1');
 const objects=[...pdf.matchAll(/(\d+) 0 obj([^]*?)endobj/g)].map(match=>({id:+match[1],raw:match[2]}));
 await mkdir(targetDirectory,{recursive:true});
 const results=[];
 for(const id of ids){
  const object=objects.find(value=>value.id===id);
  if(!object||!/\/Subtype\s*\/Image/.test(object.raw)||!/\/Filter \/JPXDecode/.test(object.raw))throw Error('Expected direct JPX image object '+id);
  const width=+object.raw.match(/\/Width (\d+)/)[1],height=+object.raw.match(/\/Height (\d+)/)[1],length=+object.raw.match(/\/Length (\d+)/)[1];
  const start=object.raw.indexOf('stream\n')+7;
  const pixels=await decodeImage(Buffer.from(object.raw.slice(start,start+length),'latin1'));
  const channels=pixels.length/(width*height);
  if(![1,3,4].includes(channels))throw Error('Unexpected decoded image dimensions');
  const target=join(targetDirectory,'rita-r36-image'+id+'.png');
  await writeFile(target,await encodeImage({pixels,width,height,channels,invert}));
  results.push({id,width,height,channels,target});
 }
 return results;
}
