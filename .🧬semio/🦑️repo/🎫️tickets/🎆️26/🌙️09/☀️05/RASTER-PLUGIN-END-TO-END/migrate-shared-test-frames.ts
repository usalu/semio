import {readFileSync,writeFileSync} from 'node:fs';
const paths=[
 '🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🎟️context/🧪️tests/🛬️decoder/🦀️.rs',
 '🧰️framework/🔨️modules/🧵️job/🛬️receiving/🧪️tests/🦀️.rs',
 '🧰️framework/🔨️modules/🧵️job/📬️outcome/⏸️continuation/🧪️tests/🎟️source/🦀️.rs',
 '🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🧪️tests/🦀️.rs',
 '🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🎟️work/🧪️tests/🦀️.rs',
];
for(const path of paths){
 const original=readFileSync(path,'utf8');let text=original;
 const functions=[...original.matchAll(/\bfn\s+[A-Za-z0-9_]+\s*\([^{}]*\)\s*(?:->\s*[^{}]+)?\{/g)];
 for(const f of functions.reverse()){
  const start=f.index!+f[0].length;let depth=1,end=start;while(depth&&end<text.length){const c=text[end++];if(c==='{')depth++;else if(c==='}')depth--;}
  let body=text.slice(start,end-1);const before=body;
  body=body.replace(/(\.receive_shared\([^;\n]*?SharedIssuer::owned\(\))(\))/g,'$1,&mut shared_allocation_frame$2');
  if(body===before)continue;
  const prefix=path.includes('🌱️value/')?'crate::native_decoding':'semio_framework_value::native_decoding';
  body=`let mut shared_allocation_frame=${prefix}::NativeReceivingEffect::default();`+body;
  text=text.slice(0,start)+body+text.slice(end-1);
 }
 if(text!==original){if(!process.argv.includes('--dry'))writeFileSync(path,text);console.log(path)}
}
