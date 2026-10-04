import {readFileSync} from 'node:fs';
const fixture=JSON.parse(readFileSync('C:/git/semio/🧰️framework/🛍️products/📓️print/🧪️tests/🎬️render-scene/🔣️legend.json','utf8'));
const records=readFileSync(import.meta.dir+'/../legend-native-after/🧪️probe-out/legends.probe.jsonl','utf8').trim().split('\n').map(line=>JSON.parse(line));
const failures:string[]=[];
for(const entry of fixture.cases.filter((entry:any)=>entry.title)){
 const record=records.find((record:any)=>record.scenario===entry.id&&record.key==='geometry/legend-item');
 const expected=Number(entry.options.at.split(',')[1])-Math.max(Number(entry.options.swatchSize??2.6),Number(entry.options.titleSize??7.2)*25.4/72.27)-Number(entry.options.itemGap??1.2)-Number(entry.options.swatchSize??2.6);
 if(Math.abs(Number(record.values[2])-expected)>1e-4)failures.push(entry.id+': '+record.values[2]+' expected '+expected);
}
console.log('[DEBUG] native authored title-line-box counterexample '+JSON.stringify(failures));if(failures.length)throw Error(failures.join('\n'));