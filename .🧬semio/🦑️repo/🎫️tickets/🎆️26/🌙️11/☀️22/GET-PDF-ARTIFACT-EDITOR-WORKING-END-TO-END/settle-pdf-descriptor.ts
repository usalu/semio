import { readFileSync } from "node:fs";
import { join } from "node:path";

/** 🧾️ Settles the `stdio-pdf` descriptor pair the native `committed_descriptor` test described (the runtime path of the component's `describe` export, written to its `CARGO_TARGET_TMPDIR/stdio-pdf-described`) into the committed owner in `write` mode, through the same verification and settle step `repo run workspace:components -- describe stdio-pdf` runs after its wasm build, which this ticket's memory gate forbids.
 * Usage: `bun settle-pdf-descriptor.ts <described directory>` from the workspace root.
 * @see ./🧱️pdf-descriptor-and-retirement.md */

const root = process.cwd();
const [directory] = process.argv.slice(2) as [string];
const describe = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe");
const { componentStageTargetV1, settleComponentDescriptorV1 } = await import(join(describe, "🧩️component-stage/🟦️.ts"));
const { verifySemanticDescriptorPairBytesV1 } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts"));
const target = componentStageTargetV1(root, "🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/📘️pdf/🧩️component/📦️packages/🦀️rust/Cargo.toml");
const jsonBytes = readFileSync(join(directory, "🔣️.json")), packBytes = readFileSync(join(directory, "🛂️.descriptor.semio"));
const pair = verifySemanticDescriptorPairBytesV1(jsonBytes, packBytes);
if (pair.pluginId !== target.pluginId) throw new Error(`the described pair is ${pair.pluginId}, not ${target.pluginId}`);
const drift = settleComponentDescriptorV1(root, target, { profile: "prototype", directory }, { jsonBytes, packBytes }, "write");
console.log(`${jsonBytes.byteLength} JSON bytes, ${packBytes.byteLength} pack bytes; ${drift.length ? `rewrote ${target.ownerRoot}:\n  ${drift.slice(0, 40).join("\n  ")}` : "committed descriptor already current"}`);
