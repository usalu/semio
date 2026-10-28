import Ajv from'ajv';
import{strict as assert}from'node:assert';
import{readFileSync,writeFileSync}from'node:fs';
interface CollectionOracle{observe(values:readonly string[]):boolean}
class AjvReleasedSet implements CollectionOracle{
 private readonly validate;
 constructor(metadata:{kind:string;lower:number;upper:number}){assert.equal(metadata.kind,'set');assert.equal(metadata.upper,-1);this.validate=new Ajv({strict:true}).compile({type:'array',minItems:metadata.lower,uniqueItems:true,items:{type:'string',pattern:'^[1-9][0-9]*$'}});}
 observe(values:readonly string[]):boolean{return this.validate(values)===true;}
}
const input=JSON.parse(readFileSync(new URL('./🗑️generated/type-set-select-oracle.json',import.meta.url),'utf8')),metadata=JSON.parse(readFileSync(new URL('./🗑️generated/type-set-select-library-metadata.json',import.meta.url),'utf8')),oracle=new AjvReleasedSet(metadata);
const rows=input.rows.filter((row:any)=>row.physical).map((row:any)=>{const collection=row.carrier==='typedValue'?oracle.observe(row.references):true,accepted=row.admitted&&collection;assert(row.exact);assert.equal(accepted,row.expectedAdmitted);return{id:row.id,physicalCarrierExact:row.exact,libraryExpressAccepted:row.admitted,collectionAccepted:collection,accepted,expectedAccepted:row.expectedAdmitted,libraryUniquenessGap:row.admitted&&!collection};});
const output={physicalOracle:{name:input.oracle,version:input.version},collectionOracle:'Ajv',metadata,rows,nativeExecuted:false};writeFileSync(new URL('./🗑️generated/type-set-select-collection-oracle.json',import.meta.url),JSON.stringify(output,null,2)+'\n');console.log('[DEBUG]',JSON.stringify({physical:rows.length,compositeMatched:rows.length,rawExpressMatched:rows.filter((row:any)=>row.libraryExpressAccepted===row.expectedAccepted).length,uniquenessGap:rows.filter((row:any)=>row.libraryUniquenessGap).map((row:any)=>row.id)}));
