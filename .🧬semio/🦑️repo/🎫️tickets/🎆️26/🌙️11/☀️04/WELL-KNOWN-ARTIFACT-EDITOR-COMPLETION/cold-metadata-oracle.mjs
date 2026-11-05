import Ajv from 'ajv/dist/2020.js';
import {Database} from 'bun:sqlite';
const root=process.cwd()+'/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/❄️cold-apply/🧪️tests/📊️metadata/';
const data=await Bun.file(root+'🔣️.json').json();
const schema=await Bun.file(root+'🧬️schema.json').json();
const valid=new Ajv({strict:true}).compile(schema);
if(!valid(data))throw Error(JSON.stringify(valid.errors));
const sql=new Database(':memory:');
for(const row of data.cases){
 const funded=sql.query('SELECT (? AND ?>=1 AND ?>=1 AND ?>=1) AS paid').get(+row.cloned,row.items,row.copyBytes,row.depth).paid;
 const expected={items:funded,copyBytes:funded,work:funded,terminal:!row.cloned||!!funded};
 if(JSON.stringify(expected)!==JSON.stringify(row.expected))throw Error(JSON.stringify({row,expected}));
}
console.log('[DEBUG] cold metadata AJV + SQLite '+data.cases.length+' cases passed');
