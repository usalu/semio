/** 🧭 Validate the approved proposed-study wording against its portable schema and both transcripts. */
const fs=require('node:fs');
const path=require('node:path');
const assert=require('node:assert/strict');
const crypto=require('node:crypto');
const Ajv=require('ajv/dist/2020').default;
let root=__dirname;
while(!fs.existsSync(path.join(root,'🧰️framework'))){const parent=path.dirname(root);if(parent===root)throw Error('Repository root missing');root=parent;}
const clip=path.join(root,'🎓️teaching/🏛️architecture/📊️excel/🎬️clip');
const source=path.join(clip,'📋️program/🐍️.py');
const fixturePath=path.join(clip,'🧪️tests/🎭️presentation/🔣️.cases.json');
const testPath=path.join(clip,'🧪️tests/🎭️presentation/🐍️.py');
const files=[source,fixturePath,testPath,...['en','de'].map(language=>path.join(clip,'🗣️transcript.'+language+'.md')),__filename];
const hashes=()=>Object.fromEntries(files.map(file=>[path.relative(root,file),crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex')]));
const before=hashes();
const fixture=JSON.parse(fs.readFileSync(fixturePath,'utf8')).program_study_hook;
const match=fs.readFileSync(source,'utf8').match(/\["hook",\s*("(?:[^"\\]|\\.)*"),\s*("(?:[^"\\]|\\.)*")\]/s);
assert.ok(match);
const actual={en:JSON.parse(match[1]),de:JSON.parse(match[2])};
assert.deepEqual(actual,fixture.localized);
const validate=new Ajv({strict:true}).compile(fixture.schema);
assert.equal(validate(actual),true,JSON.stringify(validate.errors));
for(const language of ['en','de']){
 assert.equal(fixture.required_phrases[language].length>=3,true);
 for(const phrase of fixture.required_phrases[language])assert.equal(actual[language].includes(phrase),true);
 assert.equal(fs.readFileSync(path.join(clip,'🗣️transcript.'+language+'.md'),'utf8').includes(actual[language]),true);
 const invalid=structuredClone(actual);invalid[language]=invalid[language].replace(language==='en'?'proposed':'vorgeschlagene','historical');assert.equal(validate(invalid),false);
}
const after=hashes();
assert.deepEqual(after,before);
const output=path.join(__dirname,'🗑️generated/excel-hook-static');fs.mkdirSync(output,{recursive:true});
fs.writeFileSync(path.join(output,'result.json'),JSON.stringify({tests:1,invalid_localizations:2,success:true,before,after,unchanged:true},null,2));
console.log('[DEBUG] Bilingual proposed-study hook, exact source and both transcripts match; two invalid claims rejected.');
