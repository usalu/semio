import {readFileSync,writeFileSync} from 'node:fs';
const path='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/🛬️lifted/♻️retirement/🌱️birth/🦀️.rs';
const original=readFileSync(path,'utf8');
const next=original.replaceAll('semio_framework_value::NativeReceivingEffect','semio_framework_value::native_decoding::NativeReceivingEffect');
if(next===original)throw new Error('birth receiving type changed');
writeFileSync(path,next);
