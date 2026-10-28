import Ajv from'ajv';
import{strict as assert}from'node:assert';
import{writeFileSync}from'node:fs';
import law from'../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/type-mixed-select-input.json';
import schema from'../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/type-mixed-select-input.schema.json';
import{ifc4x3Catalog as catalog}from'../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
const validate=new Ajv({strict:true}).compile(schema);assert(validate(law),JSON.stringify(validate.errors));
const declaration=catalog.entities[law.entity]!;assert.equal(declaration.attributes.length,law.direct);assert.equal(declaration.allAttributes.length,law.full);assert.equal(declaration.attributes[Number(law.index)]!.name,law.attribute);assert.equal(declaration.attributes[Number(law.index)]!.typeExpr.name,law.select);assert.deepEqual(catalog.types[law.select]!.members,law.members);
assert.equal(new Set(law.cases.map(row=>row.id)).size,law.cases.length);
for(const entity of law.source.entities){assert.equal(entity.args.length,entity.complex.length?catalog.entities[entity.name]!.attributes.length:catalog.entities[entity.name]!.allAttributes.length);for(const part of entity.complex)assert.equal(part.args.length,catalog.entities[part.name]!.attributes.length);}
const output={schema:catalog.schema,source:catalog.source,cases:law.cases.length,physical:law.cases.filter(row=>row.physical).length,initialSelection:false,nativeExecuted:false};
writeFileSync(new URL('./🗑️generated/type-mixed-select-neutral.json',import.meta.url),JSON.stringify(output,null,2)+'\n');console.log('[DEBUG]',JSON.stringify(output));
