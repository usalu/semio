import{writeFileSync}from'node:fs';
import{ifc4x3Catalog as catalog}from'../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
const entityNames=['IFCRIGIDOPERATION','IFCCOORDINATEOPERATION','IFCPROJECTEDCRS','IFCCOORDINATEREFERENCESYSTEM','IFCTEXTSTYLEFONTMODEL'];
const typeNames=['IFCMEASUREVALUE','IFCCOMPLEXNUMBER','IFCCOUNTMEASURE','IFCNUMERICMEASURE','IFCSIZESELECT','IFCCOORDINATEREFERENCESYSTEMSELECT'];
const result={schema:catalog.schema,source:catalog.source,entities:entityNames.map(name=>({name,declaration:catalog.entities[name]})),types:typeNames.map(name=>({name,declaration:catalog.types[name]}))};
writeFileSync(new URL('./🗑️generated/type-measure-select-census.json',import.meta.url),JSON.stringify(result,null,2)+'\n');
console.log('[DEBUG]',JSON.stringify({entities:result.entities,types:result.types}));
