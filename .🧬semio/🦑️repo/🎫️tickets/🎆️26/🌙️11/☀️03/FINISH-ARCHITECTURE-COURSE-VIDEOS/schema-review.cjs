const fs=require('fs'),Ajv=require('ajv/dist/2020').default;
const base=JSON.parse(fs.readFileSync(0,'utf8')),path='🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/',cur=JSON.parse(fs.readFileSync(path+'🧬️schema/🔣️.schema.json','utf8')),model=JSON.parse(fs.readFileSync(path+'🔣️.json','utf8'));
const old=new Ajv({strict:false}).compile(base),now=new Ajv({strict:true}).compile(cur);
function check(name,m){let a=old(m),b=now(m);console.log(name,a,b,b?'':JSON.stringify(now.errors[0]));}
check('authored',model);
let ix=model.elements.findIndex(x=>x.tag==='FIN-RAMP-INTERIOR');
for(const kind of ['one','two','zero','three','reversed','bad_first','bad_second']){let d=structuredClone(model),a=d.elements[ix].geometry.ramp_finish.layers;if(kind==='one')a.splice(1);if(kind==='zero')a.splice(0);if(kind==='three')a.push(structuredClone(a[0]));if(kind==='reversed')a.reverse();if(kind==='bad_first')a[0].role='bedding';if(kind==='bad_second')a[1].role='wearing';check('ramp_'+kind,d)}
let diff=0,total=0;
for(let i=0;i<model.elements.length;i+=100){for(let field of ['class','tag','geometry','attributes','metadata']){let d=structuredClone(model);delete d.elements[i][field];let a=old(d),b=now(d);total++;if(a!==b){diff++;console.log('DIFF',model.elements[i].tag,field,a,b)}}}
console.log('MUTATIONS',total,'DIFFERENCES',diff);
