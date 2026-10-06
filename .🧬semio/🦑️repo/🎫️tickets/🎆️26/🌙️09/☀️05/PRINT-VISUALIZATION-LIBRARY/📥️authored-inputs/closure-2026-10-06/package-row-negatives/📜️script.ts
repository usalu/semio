import {readFileSync,writeFileSync} from 'node:fs';import {join} from 'node:path';
const p=join(process.cwd(),'🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧫️fixtures/🔓️api-freshness.json'),x=readFileSync(p,'utf8'),d=JSON.parse(x),base=d.printed.packageRows[0],row=base.row;
d.printed.packageRowCases=[
{id:'unrelated-owner',scope:'semio / viz / family / other-owner',rows:[row],valid:false},
{id:'missing-german',scope:base.scope,rows:[{...row,meaning:{en:row.meaning.en,de:''}}],valid:false},
{id:'missing-english',scope:base.scope,rows:[{...row,meaning:{en:'',de:row.meaning.de}}],valid:false},
{id:'conflicting-default',scope:base.scope,rows:[row,{...row,default:'false'}],valid:false},
{id:'conflicting-type',scope:base.scope,rows:[row,{...row,type:'string'}],valid:false},
{id:'conflicting-meaning',scope:base.scope,rows:[row,{...row,meaning:{...row.meaning,en:'Regression model.'}}],valid:false},
{id:'identical-row-contract',scope:base.scope,rows:[row,row],valid:true}
];
writeFileSync(join(import.meta.dir,'fixture-before.json'),x);writeFileSync(join(import.meta.dir,'fixture-candidate.json'),JSON.stringify(d,null,2)+'\n');
