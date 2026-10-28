/** 📚 Export the actual product classification used by TS extraction for independent checking. */
import {mkdir,writeFile} from 'node:fs/promises';
import {resolve,join} from 'node:path';
import {ifc4x3Catalog,ifcIsSubtype} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
const output=resolve(import.meta.dir,'🗑️generated/geometry-products');
await mkdir(output,{recursive:true});
await writeFile(join(output,'owned.json'),JSON.stringify({schema:'IFC4X3_ADD2',entities:Object.keys(ifc4x3Catalog.entities).map(entity=>({entity,isProduct:ifcIsSubtype(entity,'IFCPRODUCT')}))},null,2)+'\n');
console.log(JSON.stringify({entities:Object.keys(ifc4x3Catalog.entities).length}));
