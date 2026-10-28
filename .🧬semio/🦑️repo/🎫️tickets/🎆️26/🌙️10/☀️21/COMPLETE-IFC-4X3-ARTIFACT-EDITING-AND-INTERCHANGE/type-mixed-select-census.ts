import{writeFileSync}from'node:fs';
import{ifc4x3Catalog as catalog}from'../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
const entities=['IFCFILLAREASTYLEHATCHING','IFCVECTOR','IFCDIRECTION','IFCCURVESTYLE','IFCCARTESIANPOINT','IFCGEOMETRICREPRESENTATIONITEM','IFCREPRESENTATIONITEM','IFCPRESENTATIONITEM'].map(name=>({name,declaration:catalog.entities[name]}));
const types=['IFCHATCHLINEDISTANCESELECT','IFCPOSITIVELENGTHMEASURE','IFCLENGTHMEASURE'].map(name=>({name,declaration:catalog.types[name]}));
writeFileSync(new URL('./🗑️generated/type-mixed-select-census.json',import.meta.url),JSON.stringify({schema:catalog.schema,source:catalog.source,entities,types},null,2)+'\n');
console.log('[DEBUG]',JSON.stringify({entities:entities.map(row=>({name:row.name,direct:row.declaration?.attributes.length,full:row.declaration?.allAttributes.length})),types:types.map(row=>({name:row.name,kind:row.declaration?.kind}))}));
