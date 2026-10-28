import Ajv from 'C:/git/semio/node_modules/ajv/dist/2020.js';
const path='C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/guided-request-copy';
const law=await Bun.file(path+'.json').json();const validate=new Ajv({strict:true}).compile(await Bun.file(path+'.schema.json').json());if(!validate(law))throw new Error(JSON.stringify(validate.errors));
const original=JSON.stringify(law.args);const captured=structuredClone(law.args);if(JSON.stringify(captured)!==original)throw new Error('copy.words');
const text=captured.value.$ifcInputs[0].value.value;if(Buffer.byteLength(text,'utf8')<=64||captured.value.$ifcInputs[1].value.value.bits!=='9223372036854775808')throw new Error('copy.fixture');
console.log(JSON.stringify({oracle:'Ajv2020 and ECMAScript structuredClone',rawUtf8Bytes:Buffer.byteLength(text,'utf8'),exactBits:captured.value.$ifcInputs[1].value.value.bits,nativeExecuted:false}));
