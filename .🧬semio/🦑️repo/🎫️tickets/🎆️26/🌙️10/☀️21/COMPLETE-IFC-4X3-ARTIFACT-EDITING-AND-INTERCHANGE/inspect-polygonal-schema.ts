/** 🔎️ Inspect actual ADD2 owned declaration fields before polygonal implementation. */
import {ifc4x3Catalog} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
const names=['IFCPOLYGONALFACESET','IFCINDEXEDPOLYGONALFACE','IFCINDEXEDPOLYGONALFACEWITHVOIDS','IFCCARTESIANPOINTLIST3D'];
console.log(JSON.stringify(names.map(name=>({name,attributes:ifc4x3Catalog.entities[name]?.allAttributes.map(({name,declaredBy,typeExpr,optional})=>({name,declaredBy,typeExpr,optional}))})),null,2));
