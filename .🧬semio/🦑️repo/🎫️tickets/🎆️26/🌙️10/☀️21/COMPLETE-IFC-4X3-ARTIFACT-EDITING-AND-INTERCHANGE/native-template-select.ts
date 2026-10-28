import {readFileSync} from 'node:fs';
import {parseIfc4x3Step} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/step.ts';
import {ifcTemplateIdentityKeysAsync} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🏷️templates/✍️apply/🟦️.ts';
import {application} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️tests/template-fixtures.ts';
const source=await parseIfc4x3Step(readFileSync(new URL('./🗑️generated/native-template-oracle/select.ifc',import.meta.url),'utf8'));
try{await ifcTemplateIdentityKeysAsync(source,application());throw new Error('A repeated released template was accepted');}catch(error){if(!(error instanceof Error)||!error.message.includes('existingTemplate'))throw error;}
