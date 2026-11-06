import fs from "node:fs";import assert from "node:assert/strict";
const file="🧰️framework/🔨️modules/🎒️pack/🔤️json/🎛️controlled-and-borrowed-bridge/🦀️.rs",source=fs.readFileSync(file,"utf8");
assert.match(source,/fn advance<S:JsonWriteSource\+\?Sized>\(&mut self, source:&S, control: &mut semio_framework_value::NativeEncodeControl/);
assert.match(source,/self\.advance\(source,control,remaining\.maximum_copy_bytes\)\?/);
assert.match(source,/normal_step_demands_with_copy\(source,remaining\.maximum_copy_bytes\)/);
assert.match(source,/normal_remaining\(grant,self\.normal_step_progress\)/);
console.log("Controlled JSON quoted turn calls copy-bounded writer and retains grant accounting");
