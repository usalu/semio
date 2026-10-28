/** 🧪️ Canonical FEM load vectors validate through a test-owned closed schema oracle. */
import Ajv from 'ajv/dist/2020';
import {readFile} from 'node:fs/promises';
import {resolve} from 'node:path';
const root=resolve(import.meta.dir,'../../../../../../..');
const source='✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/loads';
const read=async(path:string)=>JSON.parse(await readFile(resolve(root,path),'utf8'));
const cases=await read(source+'/cases.json'),schema=await read(source+'/schema.json');
const nodal=await read('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/fem/nodal/cases.json');
interface ContractOracle { check(schema:object,input:unknown):boolean; }
class AjvContractOracle implements ContractOracle { check(schema:object,input:unknown){ return Boolean(new Ajv({strict:true}).compile(schema)(input)); } }
const oracle:ContractOracle=new AjvContractOracle();
if(!oracle.check(schema,cases))throw Error('Invalid canonical FEM load contract');
if(JSON.stringify(cases.dofs)!==JSON.stringify(nodal.dofs)||JSON.stringify(cases.bits)!==JSON.stringify(nodal.values.map((value:{bits:string})=>value.bits)))throw Error('Existing eighteen nodal vectors differ');
console.log('[DEBUG] '+JSON.stringify({closedContract:true,selectedVectors:cases.dofs.length*cases.bits.length,scopedIdentityCases:cases.identities.length,nativeMounted:true,nativeExecuted:false}));
