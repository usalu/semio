import {readFileSync,writeFileSync} from 'node:fs';
const rows:[string,string][]=[
['🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🔗️sealed-shared-receiver/🦀️.rs','crate'],
['🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🪡️field/🦀️.rs','crate'],
['🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📋️list/🦀️.rs','crate'],
['🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/📥️owned/📋️persistent/🦀️.rs','semio_framework_value'],
['🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/🛬️lifted/♻️retirement/🌱️birth/🦀️.rs','semio_framework_value']];
for(const[path,root]of rows){const body=readFileSync(path,'utf8');const old=`self.allocation.phase()!=${root}::native_decoding::NativeReceivingEffectPhase::Empty`;if(!body.includes(old))throw new Error(`missing admission guard: ${path}`);writeFileSync(path,body.replace(old,'!self.allocation.can_advance()'))}
const path='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/📨️emission/🛂️metadata/🔗️view/🦀️.rs';const body=readFileSync(path,'utf8');const old='frame.phase()!=semio_framework_value::native_decoding::NativeReceivingEffectPhase::Empty';if(!body.includes(old))throw new Error('missing view admission guard');writeFileSync(path,body.replace(old,'!frame.can_advance()'));
