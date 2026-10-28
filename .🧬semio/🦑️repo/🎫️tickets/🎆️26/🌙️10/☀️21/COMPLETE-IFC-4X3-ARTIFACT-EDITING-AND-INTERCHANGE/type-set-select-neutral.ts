import Ajv from'ajv';
import{strict as assert}from'node:assert';
import{writeFileSync}from'node:fs';
import law from'../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/type-set-select-input.json';
import schema from'../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/type-set-select-input.schema.json';
import{ifc4x3Catalog as catalog}from'../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
const validate=new Ajv({strict:true}).compile(schema);assert(validate(law),JSON.stringify(validate.errors));
const declaration=catalog.entities[law.entity]!,set=catalog.types[law.aggregate.type]!.typeExpr!;assert.equal(declaration.attributes.length,law.direct);assert.equal(declaration.allAttributes.length,law.full);assert.equal(declaration.attributes[Number(law.index)]!.name,law.attribute);assert.equal(declaration.attributes[Number(law.index)]!.typeExpr.name,law.select);assert.deepEqual(catalog.types[law.select]!.members,law.members);assert.equal(set.aggregate,law.aggregate.kind);assert.equal(set.lower,law.aggregate.lower);assert.equal(set.upper,law.aggregate.upper);assert.equal(set.unique,law.aggregate.unique);assert.equal(set.item!.name,law.aggregate.item);
assert.equal(new Set(law.cases.map(row=>row.id)).size,law.cases.length);assert.equal(new Set(law.source.entities.map(row=>row.id)).size,law.source.entities.length);
for(const entity of law.source.entities){assert.equal(entity.args.length,entity.complex.length?catalog.entities[entity.name]!.attributes.length:catalog.entities[entity.name]!.allAttributes.length);for(const part of entity.complex)assert.equal(part.args.length,catalog.entities[part.name]!.attributes.length);}
assert.deepEqual(law.pages.flatMap(page=>page.members),law.cases.find(row=>row.id==='set-eighteen')!.input!.value.items![0]!.value.map((row:any)=>row.value));
const output={schema:catalog.schema,source:catalog.source,cases:law.cases.length,sourceEntities:law.source.entities.length,physical:law.cases.filter(row=>row.physical).length,pageMembers:law.pages.map(page=>page.members.length),nativeExecuted:false};writeFileSync(new URL('./🗑️generated/type-set-select-neutral.json',import.meta.url),JSON.stringify(output,null,2)+'\n');console.log('[DEBUG]',JSON.stringify(output));
