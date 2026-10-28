import {applyIfcDiff,composeIfcDiff,inverseIfcDiff,equalIfcValue} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts';
const integer=(value:number)=>({kind:'integer',value:BigInt(value)});
const graph=()=>({schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[{kind:'aggregate',value:[{kind:'string',value:'IFC4X3_ADD2'}]}]},entities:[{id:1n,name:'IFCTEST',args:[0,1,2,3].map(integer),complex:[]}]});
const base=graph();
try {const result=applyIfcDiff(base as any,{entities:{modified:[{id:1n,diff:{args:{modified:[{index:0,value:{kind:'binary',value:0} as any}]}}}]}});console.log('[DEBUG] numeric binary admitted',result.entities[0].args[0]);} catch(error){console.log('[DEBUG] numeric binary refused',String(error));}
let cases=0;
for(let removed=0;removed<16;removed++)for(let addMask=0;addMask<16;addMask++) {
  const rem=[0,1,2,3].filter(i=>removed&(1<<i));const len=4-rem.length;const adds=[0,1,2,3].filter(i=>addMask&(1<<i));
  if(adds.some((i,j)=>i>len+j))continue;
  const a={entities:{modified:[{id:1n,diff:{args:{removed:rem,added:adds.map(index=>({index,value:integer(10+index)}))}}}]}};
  const intermediate=applyIfcDiff(graph() as any,a);const n=intermediate.entities[0].args.length;
  for(let target=0;target<n;target++)for(const mode of ['remove','modify'] as const){
    const args=mode==='remove'?{removed:[target]}:{modified:[{index:target,value:integer(99)}]};const b={entities:{modified:[{id:1n,diff:{args}}]}};
    const expected=applyIfcDiff(intermediate,b);const c=composeIfcDiff(graph() as any,a,b);const actual=applyIfcDiff(graph() as any,c);cases++;
    if(!equalIfcValue(actual,expected)){console.log('[DEBUG] composition mismatch', {a,b,c,expected,actual});process.exit(1);}
    const restored=applyIfcDiff(intermediate,inverseIfcDiff(graph() as any,a));if(!equalIfcValue(restored,graph()))throw Error('inverse mismatch');
  }
}
console.log('[DEBUG] composition cases passed',cases);
let reads=0;
const children=new Proxy(Array.from({length:100000},()=>integer(1)),{get(target,key,receiver){if(typeof key==='string'&&/^\d+$/.test(key))reads++;return Reflect.get(target,key,receiver);}});
const wide=graph();wide.entities[0].args=[{kind:'aggregate',value:children}] as any;
try{applyIfcDiff(wide as any,{}, {maxValues:3});}catch(error){console.log('[DEBUG] tiny value budget still reads every child before refusal',{reads,error:String(error)});}
const sharedBase=graph();const sharedResult=applyIfcDiff(sharedBase as any,{});(sharedResult.entities[0].args[0] as any).value=99n;console.log('[DEBUG] result mutation changes original',sharedBase.entities[0].args[0].value===99n);
