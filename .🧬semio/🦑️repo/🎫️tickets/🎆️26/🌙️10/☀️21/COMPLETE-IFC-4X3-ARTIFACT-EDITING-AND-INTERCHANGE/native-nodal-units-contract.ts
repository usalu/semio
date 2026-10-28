/** 🧪️ Native unit vectors validate behind a test-owned schema interface. */
import Ajv from 'ajv/dist/2020';
import {readFile} from 'node:fs/promises';
import {resolve} from 'node:path';
const root=resolve(import.meta.dir,'../../../../../../..');
const directory='✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/fem/nodal';
interface ContractOracle {check(schema:object,input:unknown):boolean;}
class AjvContractOracle implements ContractOracle {check(schema:object,input:unknown){return Boolean(new Ajv({strict:true}).compile(schema)(input));}}
const read=async(path:string)=>JSON.parse(await readFile(resolve(root,directory,path),'utf8'));
const cases=await read('units-cases.json'),schema=await read('units.schema.json');
if(!new AjvContractOracle().check(schema,cases))throw Error('Invalid native nodal unit contract');
console.log('[DEBUG] '+JSON.stringify({closedContract:true,cases:cases.cases.length,nativeMounted:true,nativeExecuted:false}));
