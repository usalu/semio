// 🧪️ Simulates the store-side tool-job coverage law of the repo library on the current tree (read-only).
import {readFileSync} from "node:fs";
import {policyReadOsStoreSource} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import {toolJobStoreBatchPublicationBounded, toolJobEphemeralOneItemPublicationBounded} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📤️publication/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const plugin = readFileSync(`${root}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, "utf8");
const store = policyReadOsStoreSource(root);
console.log("store bytes", store.length, "storeBatchPublicationBounded", toolJobStoreBatchPublicationBounded(store, plugin), "ephemeralOneItem", toolJobEphemeralOneItemPublicationBounded(store, plugin));
