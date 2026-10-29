import jsonpatch from "fast-json-patch";
import Ajv2020 from "ajv/dist/2020";
import fixture from "../../../../../../../✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎥️media-export-job-and-factories/🧪️fixture.json";
import schema from "../../../../../../../✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎥️media-export-job-and-factories/🧬️vector.schema.json";
const validate=new Ajv2020({strict:true,allErrors:true}).compile(schema);
for(const row of fixture.projectionCases){
 const svg=row.svg.repeat("repeat" in row?row.repeat:1);
 const projection=jsonpatch.applyPatch({},[{op:"add",path:"/schema",value:fixture.projectionSchema},{op:"add",path:"/source",value:{documentId:row.documentId,frontierRevision:row.frontierRevision}},{op:"add",path:"/svg",value:svg}],true,false).newDocument;
 const actual=JSON.parse(JSON.stringify(projection));
 if(!validate(actual)||JSON.stringify(actual)!==JSON.stringify({...row.expected,svg}))throw Error(row.name);
}
console.log(JSON.stringify({projectionCases:fixture.projectionCases.length,validatedSchema:schema.$id,sourceFields:Object.keys(schema.properties.source.properties)}));
