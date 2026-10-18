import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
const t = readModuleTreeSource(new URL("file:///Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"));
console.log(t.length, t.includes("fn take_finished_snapshot_owner("), t.includes("fn raw_replay_demands("));
