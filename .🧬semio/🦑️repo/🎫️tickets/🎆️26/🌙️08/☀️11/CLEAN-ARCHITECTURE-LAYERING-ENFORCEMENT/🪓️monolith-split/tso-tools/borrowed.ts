import { join } from "node:path";
import { readFileSync } from "node:fs";
import { toolJobRustBlock } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
const base="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit";
const parent=readFileSync(join(base,"🦀️.rs"),"utf8"),child=readFileSync(join(base,"🧵️borrowed/🦀️.rs"),"utf8");
const method=(text:string,pattern:RegExp)=>{const start=text.search(pattern);if(start<0)return "";const arrow=text.indexOf(") ->",start);return toolJobRustBlock(text,text.indexOf("{",arrow<0?start:arrow))?.body??"";};
const close=method(parent,/pub fn close_step\(&mut self, grant: ArtifactStoreOneItemGrant/),bind=method(child,/fn bind</);
console.log({
p1:parent.includes("pub trait ArtifactCanonicalJson: Sync"),
p2:parent.indexOf("encoder: ArtifactCanonicalEditEncoder")<parent.indexOf("edit: Option<Box<Edit<M>>>"),idx:[parent.indexOf("encoder: ArtifactCanonicalEditEncoder"),parent.indexOf("edit: Option<Box<Edit<M>>>")],
c1:close.indexOf("self.encoder.close_step()")>=0,closeLen:close.length,c2:close.indexOf("self.encoder.close_step()")<close.indexOf("self.edit.take()"),
b1:bind.includes("self.root_address != address"),b2:bind.includes("root.canonical_json_borrowed_root()"),bindLen:bind.length,
ch1:child.includes("pub(super) struct ArtifactCanonicalEditEncoder"),ch2:child.includes("Iterator<Item = (&'a str, ArtifactCanonicalJsonValue<'a>)> + Send + 'a"),ch3:child.includes("values.values.next()"),ch4:child.includes("self.frames[self.depth] = None"),
p3:parent.includes("self.depth >= self.maximum_depth"),ch5:child.includes("maximum_depth: ARTIFACT_CANONICAL_JSON_DEPTH - top"),no:!/\.nth\(|\.range\(|serde_json::to_(?:vec|value|string)/.test(child)});
