/** 🧪️ Physical phase carriers pass through the public exporter and STEP writer. */
import {mkdir,writeFile} from 'node:fs/promises';
import cases from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/bim/phases/cases.json';
import {ifcBimAxisFixture} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/bim-axis-fixtures.ts';
import {bimSpatialToIfc4x3} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/bim.ts';
import {writeIfc4x3Step} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/step-writer.ts';
const root=new URL('./🗑️generated/bim-phases-physical/',import.meta.url);await mkdir(root,{recursive:true});
const observations=[];
for(const row of cases.cases){const model=ifcBimAxisFixture(.3);model.project.phase_names=row.phases;const outcome=await bimSpatialToIfc4x3(model);if(!outcome.artifact)throw Error('Export refused '+row.id);await writeFile(new URL(row.id+'.ifc',root),await writeIfc4x3Step(outcome.artifact));observations.push({id:row.id,phases:row.phases,entities:outcome.artifact.entities.length});}
await writeFile(new URL('input.json',root),JSON.stringify(observations,null,2));console.log('[DEBUG] physical phase inputs '+observations.length);

