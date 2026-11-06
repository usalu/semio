import {readFileSync,writeFileSync} from 'node:fs';
for(const [path,source,output] of [
 ['🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/📨️emission/🛂️metadata/🔗️view/🧪️tests/routing.rs','original','root'],
 ['🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/📨️emission/🛂️metadata/🔗️view/🧪️tests/🦀️.rs','source','root'],
] as const){
 const original=readFileSync(path,'utf8');let text=original;
 text=text.replaceAll(`let mut ${source}=Some(view);let mut ${output}=None;`,`let mut ${source}=Some(view);let mut ${output}=None;let mut frame=semio_framework_value::native_decoding::NativeReceivingEffect::default();`);
 text=text.replaceAll(`ActionMeta::admit_view_root(&mut ${source},&mut ${output},cx)`,`ActionMeta::admit_view_root(&mut ${source},&mut ${output},&mut frame,cx)`);
 if(text===original)throw new Error(`view law changed: ${path}`);
 writeFileSync(path,text);
}
