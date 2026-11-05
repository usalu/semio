import Ajv from "ajv";
import {readFileSync} from "node:fs";
const base="✏️s/🔌️plugins/🀄️wfc/🧪️tests/🧫️bitmap-fill-publication/";
const fixture=JSON.parse(readFileSync(base+"🔣️.json","utf8"));
const ajv=new Ajv({strict:false,allErrors:true});
if(!ajv.validate(JSON.parse(readFileSync(base+"🧬️schema.json","utf8")),fixture))throw Error(JSON.stringify(ajv.errors));
for(const row of fixture.cases){
 const cells=row.width*row.height;
 const pixels=Buffer.alloc(cells),decided=Buffer.alloc(cells);decided[0]=1;
 const payload={pixels:pixels.toString("base64"),decided:decided.toString("base64"),width:row.width,height:row.height,contradiction:row.contradiction,done:true,trace:[]};
 const admitted=!row.cancelled&&Buffer.byteLength(JSON.stringify(payload))+1024<262144;
 if(admitted!==row.admitted||(admitted&&!row.contradiction)!==row.commit)throw Error(row.id);
}
console.log("[DEBUG] Ajv and Node base64/JSON independent bitmap publication oracle passed "+fixture.cases.length+" cases");
