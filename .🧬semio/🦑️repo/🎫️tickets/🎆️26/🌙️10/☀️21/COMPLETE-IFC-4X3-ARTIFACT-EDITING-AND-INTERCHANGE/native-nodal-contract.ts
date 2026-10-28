/** 🧪️ Native nodal lifecycle metadata validates behind a test-owned schema interface. */
import Ajv from 'ajv/dist/2020';
import contract from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/fem/nodal/native-cases.json';
import schema from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/fem/nodal/native.schema.json';
import vectors from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/fem/nodal/cases.json';
interface ContractOracle {check(schema:object,input:unknown):boolean;}
class AjvContractOracle implements ContractOracle {check(schema:object,input:unknown){return Boolean(new Ajv({strict:true}).compile(schema)(input));}}
const oracle:ContractOracle=new AjvContractOracle();if(!oracle.check(schema,contract))throw Error('Invalid native nodal contract');if(vectors.dofs.length*vectors.values.length!==contract.selectedCases)throw Error('Neutral vector identity mismatch');console.log('[DEBUG] '+JSON.stringify({closedContract:true,sharedSelectedCases:contract.selectedCases,lifecycleLaws:contract.laws.length,nativeMounted:false,nativeExecuted:false}));

