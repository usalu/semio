// 🔬 c4p37 probe: the microsecond worker law on the baseline trace text (HEAD blob or the orig snapshot) against the live module-tree text.
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {toolJobMicrosecondWorkerExact} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
import {readModuleTreeSource} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const base = join(root, "🧰️framework/🔨️modules/🧵️job/⏱️budget");
const plugin = readFileSync(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"), "utf8");
const job = readFileSync(join(base, "../🦀️.rs"), "utf8");
const loan = readFileSync(join(base, "../📬️outcome/🤝️loan/🦀️.rs"), "utf8");
const orig = readFileSync(join(import.meta.dir, "orig/trc.rs"), "utf8");
const tree = readModuleTreeSource(join(root, "🧰️framework/🔨️modules/⏱️trace/🦀️.rs"));
console.log("baseline trace text:", toolJobMicrosecondWorkerExact(plugin, job, orig, loan));
console.log("module tree trace text:", toolJobMicrosecondWorkerExact(plugin, job, tree, loan));
console.log("tree has default_clock_us None:", /fn default_clock_us\(\) -> Option<u64> \{\s*None\s*\}/.test(tree), "elapsed:", tree.includes("elapsed().as_micros()).ok()"));
console.log("orig has:", /fn default_clock_us\(\) -> Option<u64> \{\s*None\s*\}/.test(orig), orig.includes("elapsed().as_micros()).ok()"));
