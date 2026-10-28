/** 🔬 Observe the declared BIM phase-name list through the actual exporter/importer. */
import {writeFile} from 'node:fs/promises';
import {ifcBimAxisFixture} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/bim-axis-fixtures.ts';
import {bimSpatialToIfc4x3,ifc4x3ToBimSpatial} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/bim.ts';
const rows=[];
for(const phases of [[],[''],['Design;Review'],['Design','Review'],['Design;Review','','Design;Review']]){const model=ifcBimAxisFixture(.3);model.project.phase_names=phases as never[];const exported=await bimSpatialToIfc4x3(model);if(!exported.artifact)throw Error('Export refused');const imported=await ifc4x3ToBimSpatial(exported.artifact);rows.push({source:phases,actual:imported.artifact?.project.phase_names,equal:JSON.stringify(phases)===JSON.stringify(imported.artifact?.project.phase_names)});}
await writeFile(new URL('./🗑️generated/bim-phase-roundtrip-before.json',import.meta.url),JSON.stringify(rows,null,2),{flag:'wx'});console.log('[DEBUG] '+JSON.stringify(rows));
