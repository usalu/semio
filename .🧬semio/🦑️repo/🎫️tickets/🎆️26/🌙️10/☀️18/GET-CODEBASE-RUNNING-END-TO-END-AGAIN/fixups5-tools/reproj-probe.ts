import { join } from "node:path";
import { readModuleTreeSource } from "@semio-tech/framework/compiler/syntax/module-tree";
const E = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements";
const host = readModuleTreeSource(join(E, "🛠️ShellHelpers", "..", "🏛️ShellHost", "⚡️host-effects", "📥️document-load-effects", "🟦️.ts"));
const start = host.indexOf("export async function applyLoadDocumentEffect(");
const load = host.slice(start, host.indexOf("\nexport ", start + 1));
console.log("probe", start, load.length, [load.split("programHistoryProjectionsWithLoadV1(").length - 1, /progress: \(status\) => \{[^}]*showLoad\(status\);/u.test(load), /\} finally \{\s*showLoad\(null\);/u.test(load)]);
