import { join } from "node:path";
import { readFileSync } from "node:fs";
import { readStoreSourceText } from "@semio-tech/framework/test/source-text";
import { toolJobRustBlock } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
const W="/Users/ueli/Documents/semio/", storePath="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store";
const base=join(W,storePath,"🧵️canonical-edit");
const store=readFileSync(join(W,storePath,"🎮️one-item/📬️publication/🦀️.rs"),"utf8")+"\n"+readStoreSourceText();
const source=readFileSync(join(W,storePath,"🎮️one-item/📬️publication/📥️lifted/🧵️canonical-edit/🦀️.rs"),"utf8");
const method=(text:string,pattern:RegExp)=>{const start=text.search(pattern);return start<0?"":toolJobRustBlock(text,text.indexOf("{",start))?.body??"";};
const validation=method(store,/fn validate_prepared</),mint=method(store,/fn seal_prepared_owned</),advance=method(source,/pub fn advance\(&mut self, grant: ArtifactStoreOneItemGrant/);
const commit=[...store.matchAll(/fn fold_batch_item\(/g)].map(m=>toolJobRustBlock(store,store.indexOf("{",m.index))?.body??"").reduce((l,b)=>b.length>l.length?b:l,"");
const c:Record<string,boolean>={
v1:validation.includes("Arc::ptr_eq(self, &prepared.seal.authority)"),v2:validation.includes("prepared.seal.edit_address != prepared.edit.as_ref() as *const"),v3:validation.includes("prepared.seal.post_address != Arc::as_ptr(&prepared.post_snapshot)"),v4:validation.includes("prepared.seal.digest != prepared.edit_digest"),
m1:mint.includes("edit_address: edit.as_ref() as *const"),m2:mint.includes("digest: edit_digest"),nopub:!/pub\s+fn\s+seal_prepared\s*[<(]/.test(store),
a1:advance.includes("self.encoder.encode_chunk(edit.as_ref(), &mut self.last_chunk[..maximum])"),a2:advance.includes("self.hash.update(&self.last_chunk[..self.last_length])"),a3:advance.includes("authority.seal_prepared_owned(edit, post, self.hash.clone().finalize(), identities)"),a4:!/serde_json::to_(?:vec|value|string)|prepared_edit_digest\(/.test(advance),
c1:commit.includes("authority.validate_prepared(candidate)"),c2:!commit.includes("prepared_edit_digest("),
lens:validation.length>0&&mint.length>0&&advance.length>0&&commit.length>0};
console.log(c,validation.length,mint.length,advance.length,commit.length);
