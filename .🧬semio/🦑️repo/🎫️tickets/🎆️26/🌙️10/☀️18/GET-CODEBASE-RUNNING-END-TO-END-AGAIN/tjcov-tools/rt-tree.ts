import { readModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const P = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const t = readModuleTreeSource(`${P}/🧵️instance/🦀️.rs`);
console.log(t.length, t.includes("async fn dispatch_typed_command_inner"), t.includes("fn publish_mounted_typed_operation_unit"));
