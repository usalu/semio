// Appends the removed layout-engine drawing tests as oracle-witnessed corpus cases (idempotent by name).
import {readFileSync,writeFileSync} from 'node:fs';
import {plan} from '/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🧵️drawing/🧪️tests/oracle.mjs';
const path='/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🧵️drawing/🧪️tests/🧫️fixtures/🔣️.json';
const corpus=JSON.parse(readFileSync(path,'utf8'));
const T0={x:0,y:0,scaleX:1,scaleY:1,shear:0,rotation:0};
const base=(id,kind,extra)=>({id,kind,name:id,visible:true,locked:false,opacity:1,blendMode:'normal',transform:{...T0},attributes:{fillRule:'nonzero'},...extra});
const rect=(id,x0,y0,x1,y1)=>base(id,'path',{attributes:{fillRule:'nonzero',stroke:{color:[0.15,0.2,0.28,1],width:1,cap:'round',join:'round'}},segments:[{kind:'move',to:[x0,y0]},{kind:'line',to:[x1,y0]},{kind:'line',to:[x1,y1]},{kind:'line',to:[x0,y1]},{kind:'close'}]});
const doc=(name,layers,assets={})=>({schema:'drawing.document',id:name,title:name,assets,layers});
const cases=[
 {name:'removed: background drawing fits an offset plan onto the page',content:doc('offset',[rect('p',10,20,110,70)]),target:{x:0,y:0,width:100,height:50,rotation:0}},
 {name:'removed: hidden background layer contributes no geometry',content:doc('hidden',[{...rect('p',10,20,110,70),visible:false}]),target:{x:0,y:0,width:100,height:50,rotation:0}},
 {name:'removed: placed drawing frame fits the plan inside its bounds',content:doc('unit',[rect('p',0,0,1,1)]),target:{x:146,y:435,width:40,height:40,rotation:0}},
 {name:'removed: drawing text lands inside the placed frame',content:doc('text',[rect('p',0,0,100,50),base('t','text',{x:10,y:30,content:'Plan',size:11,fontFamily:'shareTechMono',attributes:{fillRule:'nonzero',fill:{kind:'solid',color:[0,0,0,1]}}})]),target:{x:20,y:30,width:200,height:100,rotation:0}},
 {name:'removed: embedded png fits the placed frame',content:doc('png',[base('i','image',{imageKey:'i',width:10,height:10,transform:{...T0,x:5,y:5}})],{i:{width:1,height:1,samples:[[255,0,0,255]]}}),target:{x:10,y:10,width:50,height:50,rotation:0}},
 {name:'removed: imported arc bulges off its chord',content:doc('arc',[base('a','path',{attributes:{fillRule:'nonzero',stroke:{color:[0,0,0,1],width:1,cap:'round',join:'round'}},segments:[{kind:'move',to:[0,0]},{kind:'arc',rx:10,ry:10,rotation:0,largeArc:false,sweep:true,to:[20,0]}]})]),target:{x:0,y:0,width:100,height:50,rotation:0}},
 {name:'removed: rotated drawing frame turns the plan and keeps its png',content:doc('rot',[rect('p',0,0,100,50),base('i','image',{imageKey:'i',width:10,height:10,transform:{...T0,x:5,y:5}})],{i:{width:1,height:1,samples:[[255,0,0,255]]}}),target:{x:30,y:40,width:120,height:80,rotation:0.7853981633974483}},
];
const six=(v)=>JSON.parse(JSON.stringify(v));
let added=0;
for(const c of cases){
 if(corpus.cases.some(x=>x.name===c.name))continue;
 const out=six(plan(c));
 c.expected={bounds:out.bounds,affine:out.affine,textExtents:out.texts.map(t=>({index:t.index,content:t.content,bounds:t.bounds})),primitiveKinds:out.primitives.map(p=>p.kind)};
 corpus.cases.push(c);added++;
}
writeFileSync(path,JSON.stringify(corpus,null,2)+'\n');
console.log('added',added);
