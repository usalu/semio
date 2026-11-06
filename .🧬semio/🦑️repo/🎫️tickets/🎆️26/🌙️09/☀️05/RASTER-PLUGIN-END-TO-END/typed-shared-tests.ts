import {readFileSync,writeFileSync} from 'node:fs';
const paths=[
 '🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🎟️context/🧪️tests/🛬️decoder/🦀️.rs',
 '🧰️framework/🔨️modules/🧵️job/🛬️receiving/🧪️tests/🦀️.rs',
 '🧰️framework/🔨️modules/🧵️job/📬️outcome/⏸️continuation/🧪️tests/🎟️source/🦀️.rs',
 '🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🎟️work/🧪️tests/🦀️.rs',
 '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/👷️work/🦀️.rs',
 '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/📨️emission/🛂️metadata/🔗️view/🧪️tests/routing.rs',
 '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/📨️emission/🛂️metadata/🔗️view/🧪️tests/🦀️.rs',
];
for(const path of paths){const original=readFileSync(path,'utf8');const text=original.replaceAll('NativeReceivingEffect::default()','NativeSharedAdmission::default()');if(text!==original){writeFileSync(path,text);console.log(path)}}
