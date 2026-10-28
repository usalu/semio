/** 🧪️ Closed import status laws validate behind an owned schema oracle without native execution claims. */
import Ajv from 'ajv/dist/2020';
import {readFile} from 'node:fs/promises';
import {resolve} from 'node:path';
const root=resolve(import.meta.dir,'../../../../../../..');
const directory='✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/fem/nodal';
interface ContractOracle {check(schema:object,input:unknown):boolean;}
class AjvContractOracle implements ContractOracle {check(schema:object,input:unknown){return Boolean(new Ajv({strict:true}).compile(schema)(input));}}
const read=async(path:string)=>JSON.parse(await readFile(resolve(root,directory,path),'utf8'));
const cases=await read('import-cases.json'),schema=await read('import.schema.json');
if(!new AjvContractOracle().check(schema,cases))throw Error('Invalid native import contract');
console.log('[DEBUG] '+JSON.stringify({contractOnly:true,states:cases.queryStates.length,components:cases.components.length,laws:cases.laws.length,nativeExecuted:false}));
