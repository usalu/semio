import Ajv from 'C:/git/semio/node_modules/ajv/dist/2020.js';
const root='C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/👷️jobs/📥️staging/🖼️render/';
const contract=await Bun.file(root+'source-loan-cases.json').json();
const validate=new Ajv({strict:true}).compile(await Bun.file(root+'source-loan-cases.schema.json').json());
if(!validate(contract))throw new Error(JSON.stringify(validate.errors));
for(const row of contract.cases)if(row.admitted!==(row.caller==='original'&&row.source==='original'&&!row.cancelled))throw new Error(row.id);
console.log(JSON.stringify({oracle:'Ajv2020',neutralCases:contract.cases.length,nativeExecuted:false}));
