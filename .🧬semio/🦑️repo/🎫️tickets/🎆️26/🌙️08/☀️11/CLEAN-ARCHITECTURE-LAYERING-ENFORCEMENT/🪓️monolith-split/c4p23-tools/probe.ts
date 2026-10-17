// 🔬️ c4p23 reader probe: runs the source-text readers that can see the Package 23 files and prints deterministic JSON, so a run before and a run after the split can be compared.
// usage: bun probe.ts <label> [lawonly] > <out.json>   (label "post*" reads the raster schema root plus its four children, as the re-pointed law reader does; any other label reads the unsplit root; "lawonly" skips the slow tool-job scan)
import {readFileSync} from "node:fs";
import {toolJobPluginProductionSources, toolJobProofCatalogFailures} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
import {toolJobAppOwnedRows, toolJobDispositions, toolJobProofs, toolJobStaticRows} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
import {toolJobRasterEnvelopeCallerRetainedExact} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🗄️artifact-store/🟦️.ts";
import {policyReadFileSafe, policyReadModuleTreeSource} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts";
import {policyReadOsStoreSource} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
const out: Record<string, unknown> = {};
const mine = (text: string) => /🖨️raster|stdio\/📇️registry\/🧬️contract\/✏️editing/u.test(text);
const label = process.argv[2] ?? "pre";
const lawOnly = process.argv[3] === "lawonly";
if (!lawOnly) {
  const files = toolJobPluginProductionSources(root);
  const proofs = toolJobProofs(files);
  const rows = toolJobStaticRows(files);
  const dispositions = toolJobDispositions(files);
  const failures = toolJobProofCatalogFailures(files, rows, dispositions, proofs);
  const norm = (s: string) => s.replace(/(?:[^\s"'`(),]*\/)?(?:🌳️layer-tree-helpers|🗺️owned-map|📐️layer-model|🖼️asset-content-and-patch|🌲️layer-tree-apply|🧮️inverse-and-absorb|🖌️repaint-canvas|🌉️semio-bridge|🧩️composite-and-media|🎹️derived-composition-and-registry|🏗️derived-construction-and-analysis|📜️edit-contract|🪡️value-operations|📨️action-bridge|🧰️editor-traits-and-retained-copies|🛠️tool-job-factories|📜️patch-wire|🪜️patch-steps|🧭️patch-edits|🧩️inverse-parts|🧬️input-schema|📐️detail-model|🧬️schema-resolution|🧩️template-synthesis|🗂️dsl-provider|🏷️labels-and-controls|🌲️detail-tree-and-window)\/🦀️\.rs/gu, "<child>");
  out.toolJob = {
    proofs: proofs.length,
    proofsMine: proofs.filter((p) => mine(p.ownerFile)).map((p) => [p.ownerFile.slice(-60), p.ownerTypeName, p.controllerId, p.factory, p.toolId]),
    rows: rows.length,
    rowsMine: rows.filter((r) => mine(r.file)).map((r) => [r.file.slice(-60), r.id]),
    appOwnedMine: toolJobAppOwnedRows(files).filter((r) => mine(r.ownerFile)).map((r) => [r.toolId, r.publicationReady]),
    failuresTotal: failures.length,
    failuresMine: failures.filter(mine).map(norm).sort(),
    failuresHash: Bun.hash(failures.slice().map(norm).sort().join("\n")).toString(),
    sourcesMine: [...files.keys()].filter(mine).length,
  };
}
const rasterFile = "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs";
const rest = [
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs",
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs",
];
const serializerKinds = ["🎞️gif/🔖️87a/✳️any", "🖼️tiff/🔖️6.0/✳️any", "🎨️svg/🔖️1.1/✳️any", "🪟️bmp/🔖️v3/✳️any", "📖️pdf/🔖️1.4/✳️any", "📸️jpg/🔖️jfif-1.01/♾️any", "📷️png/🔖️1.2/✳️any", "🖊️dwg/🔖️ac1018/✳️any"];
const base = "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any";
const serializers = serializerKinds.map((k) => policyReadFileSafe(root, `${base}/🚪️io/📤️export/🧵️serializers/🗿️artifacts/${k}/🦀️.rs`));
const snapshot = policyReadFileSafe(root, `${base}/🧬️schema/📸️snapshot/🦀️.rs`);
const store = policyReadOsStoreSource(root);
const plugin = policyReadFileSafe(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs");
const editor = policyReadFileSafe(root, `${base}/✏️editor/🦀️.rs`);
const lawText = (schema: string) => [policyReadFileSafe(root, rest[0]!), policyReadFileSafe(root, rest[1]!), schema, snapshot, ...serializers].join("\n");
const fileText = lawText(policyReadFileSafe(root, rasterFile));
const kids = ["🌳️layer-tree-helpers", "🗺️owned-map", "📐️layer-model", "🖼️asset-content-and-patch"];
const kidsText = label.startsWith("post") ? lawText([policyReadFileSafe(root, rasterFile), ...kids.map((k) => policyReadFileSafe(root, rasterFile.replace(/🦀️\.rs$/u, `${k}/🦀️.rs`)))].join("\n")) : fileText;
const treeText = kidsText;
const lawSrc = readFileSync(`${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🗄️artifact-store/🟦️.ts`, "utf8");
const start = lawSrc.indexOf("export function toolJobRasterEnvelopeCallerRetainedExact");
const body = lawSrc.slice(start, lawSrc.indexOf("\nexport function", start + 10));
const strings = [...body.matchAll(/(?<![\w$])(?:raster|editor|store)\.(?:includes|indexOf)\(\s*("(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*')/g)].map((m) => JSON.parse(m[1]!.startsWith("'") ? JSON.stringify(m[1]!.slice(1, -1)) : m[1]!));
const regexes = [...body.matchAll(/\/((?:[^\/\\\n]|\\.)+)\/([gimsuy]*)(?=\s*[.)]|\.test|\.exec)/g)].map((m) => [m[1]!, m[2]!] as const).filter(([src]) => src.length > 12);
const count = (text: string, needle: string) => text.split(needle).length - 1;
out.rasterLaw = {
  fileMode: toolJobRasterEnvelopeCallerRetainedExact(store, fileText, editor, plugin),
  treeMode: toolJobRasterEnvelopeCallerRetainedExact(store, treeText, editor, plugin),
  strings: strings.length,
  stringDiffs: strings.map((s) => [s.slice(0, 90), count(fileText, s), count(treeText, s)]).filter(([, a, b]) => a !== b),
  regexes: regexes.length,
  regexDiffs: regexes.map(([src, flags]) => {
    const re = new RegExp(src, flags.includes("g") ? flags : flags + "g");
    return [src.slice(0, 90), (fileText.match(re) ?? []).length, (treeText.match(re) ?? []).length] as const;
  }).filter(([, a, b]) => a !== b),
  textBytes: [fileText.length, treeText.length],
  activeLaw: toolJobRasterEnvelopeCallerRetainedExact(store, treeText, editor, plugin),
  activeStringCounts: strings.map((s) => count(treeText, s)),
  activeRegexCounts: regexes.map(([src, flags]) => (treeText.match(new RegExp(src, flags.includes("g") ? flags : flags + "g")) ?? []).length),
};
console.log(JSON.stringify(out, null, 1));
