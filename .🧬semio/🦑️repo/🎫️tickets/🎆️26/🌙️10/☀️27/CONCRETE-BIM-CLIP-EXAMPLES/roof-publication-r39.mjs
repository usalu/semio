/** 📰 Extract direct scanned publication image streams without invoking PDF actions. */
import { readFile, writeFile } from 'node:fs/promises';
export async function extractJpeg({source,target,id}) {
 const pdf=(await readFile(source)).toString('latin1');
 const objects=[...pdf.matchAll(/(\d+) (\d+) obj\s*([^]*?)endobj/g)];
 const object=objects.filter(value=>+value[1]===id).at(-1)?.[3];
 if(!object||!/\/Filter \/DCTDecode/.test(object))throw Error('Expected direct JPEG');
 const marker=/stream\r?\n/.exec(object),length=+object.match(/\/Length (\d+)/)[1];
 await writeFile(target,Buffer.from(object.slice(marker.index+marker[0].length,marker.index+marker[0].length+length),'latin1'));
}
