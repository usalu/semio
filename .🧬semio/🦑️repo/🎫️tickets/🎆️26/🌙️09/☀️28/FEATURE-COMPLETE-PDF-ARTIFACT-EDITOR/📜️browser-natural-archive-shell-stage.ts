/** 🧷 Stages the Shell's exact selected-source acceptance through durable first-archive publication. */
import assert from "node:assert/strict";
import { mkdir } from "node:fs/promises";
import { join } from "node:path";

const output = join(import.meta.dir, "🗑️generated", "natural-source-browser-mounted");
await mkdir(output, { recursive:true });
const rows = await Bun.file(join(import.meta.dir,"🗑️generated/natural-source-browser/overlay.json")).json() as [string,string|null,string][];
const renderer = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements";
const shell = join(renderer,"🏛️ShellHost");
const replace = (body:string, prior:string, next:string) => {
  assert.equal(body.split(prior).length,2,prior.slice(0,120));
  return body.replace(prior,next);
};
const routePath = join(shell,"📤️document-transfer-tabs/🔶️scope/🟦️.tsx");
const route = rows.find(([name])=>name===routePath);
assert(route);
let body = replace(route[2], 'import { keepArtifactOpeningV1 }', 'import { keepArtifactOpeningV1, keepFirstArtifactOpeningV1, type KeptArtifactReceiptV1 }');
body = replace(body,'import { LOCAL_CATALOG_WRITE_DEADLINE_MS_V1, sameLocalDocumentArchiveV1 }','import { LOCAL_CATALOG_WRITE_DEADLINE_MS_V1, sameLocalDocumentArchiveV1, type LocalCatalogAdmissionV1 }');
body = replace(body,'import { type PluginWasmHandle } from "../../../🔌️PluginRuntime/🟦️.tsx";',[
  'import { type PluginWasmHandle } from "../../../🔌️PluginRuntime/🟦️.tsx";',
  'import { persistNaturalArchive } from "../../../🔌️PluginRuntime/🚚️handle-turn-drive/natural-source-archive.ts";',
  'import type { NaturalSourceAcceptance } from "../../../🔌️PluginRuntime/🚚️handle-turn-drive/natural-source.ts";',
].join("\n"));
body = replace(body, 'held: () => boolean, publish: () => boolean): Promise<boolean> => {', 'held: () => boolean, publish: () => boolean, accepted?: NaturalSourceAcceptance): Promise<boolean> => {');
body = replace(body, '    if (!schema || !owner.handle.readAppDocumentArchive || !current()) throw new Error("natural-file.persistence-owner-unavailable");', '    if (!schema || !current()) throw new Error("natural-file.persistence-owner-unavailable");');
const previous = body.indexOf("    return keepArtifactOpeningV1({");
const done = body.indexOf("\n  };",previous);
assert(previous>=0 && done>previous);
const existing = body.slice(previous,done);
const firstArchive = existing.indexOf("      archive: async () => {");
const firstAttach = existing.indexOf("      attach: (admission) =>");
assert(firstArchive>=0 && firstAttach>firstArchive);
const shared = existing.slice(0,firstArchive)+existing.slice(firstAttach);
let next = replace(shared,"    return keepArtifactOpeningV1({","    const publication = {");
next = replace(next,"      attach: (admission) =>","      attach: (admission: Pick<LocalCatalogAdmissionV1, \"document\" | \"binding\">) =>");
next = replace(next,"      record: async (document) =>","      record: async (document: LocalDocument) =>");
next = replace(next,"      close: async (receipt) =>","      close: async (receipt: KeptArtifactReceiptV1) =>");
next = replace(next,"      unlist: async (document) =>","      unlist: async (document: LocalDocument) =>");
next = replace(next,"      report: (error) =>","      report: (error: unknown) =>");
assert(next.endsWith("    });"));
next = next.slice(0,-"    });".length) + [
  "    };",
  "    if (accepted !== undefined) {",
  "      const capture = core.hostPorts?.firstArchiveCapture;",
  "      const exchange = owner.handle.openAppNaturalArchiveExchange?.(accepted);",
  '      if (!capture || !exchange || accepted.loan.instanceId !== instanceId) throw new Error("natural-file.first-archive-owner-unavailable");',
  "      const actor = core.shellActorIdRef.current;",
  "      const stillOwned = () => current() && actor === core.shellActorIdRef.current;",
  "      return keepFirstArtifactOpeningV1({",
  "        ...publication, current: stillOwned,",
  "        persist: async ({ document, binding }) => {",
  '          if (core.localCatalogRef.current.documents.some((entry) => entry.documentId === document.documentId)) throw new Error("natural-file.document-already-kept");',
  "          const { instanceId: owner, operationId, generation, declaration, extent } = accepted.loan;",
  "          await persistNaturalArchive(accepted, document.documentId, exchange, (frontier, signal) => capture(",
  "            { actor, binding, schema, frontier, source: { instanceId: owner, operationId, generation, sequence: accepted.seq, declaration, extent } }, signal,",
  "          ), task.signal, stillOwned, task.progress);",
  "          return true;",
  "        },",
  "      });",
  "    }",
  '    if (!owner.handle.readAppDocumentArchive) throw new Error("natural-file.persistence-owner-unavailable");',
  "    return keepArtifactOpeningV1({",
  "      ...publication,",
].join("\n")+"\n"+existing.slice(firstArchive,firstAttach)+"    });";
body = replace(body,existing,next);
body = replace(body,'    let pickedName = "";','    let pickedName = "";\n    let openedSource: NaturalSourceAcceptance | undefined;');
body = replace(body,'        if (picked.size > input.maximumBytes && importSource !== undefined && format.binary) {',[
  '        if (picked.size > input.maximumBytes && format.binary && importSource !== undefined) {',
  '          if (core.hostPorts?.firstArchiveCapture === undefined || owner.handle.openAppNaturalArchiveExchange === undefined) throw new Error("natural-file.first-archive-owner-unavailable");',
].join("\n"));
body = replace(body,'importBytes: async (freshInstanceId, signal) => { await importSource(freshInstanceId, format, picked, {','importBytes: async (freshInstanceId, signal) => { openedSource = await importSource(freshInstanceId, format, picked, {');
body = replace(body,'label: program?.label, breadcrumb: program?.breadcrumb }));','label: program?.label, breadcrumb: program?.breadcrumb }), openedSource);');
route[2] = body;

const portPath = join(shell,"🪝️host-ports/🟦️.ts");
const priorPort = await Bun.file(portPath).text();
let port = replace(priorPort,'import type { PickedFileFormatCatalogPortV1 }',[
  'import type { NaturalArchiveCapture, NaturalArchiveFrontier } from "../../🔌️PluginRuntime/🚚️handle-turn-drive/natural-source-archive.ts";',
  'import type { NaturalSourceAcceptance } from "../../🔌️PluginRuntime/🚚️handle-turn-drive/natural-source.ts";',
  'import type { LocalCatalogAdmissionV1 } from "../🗂️local-catalog/🟦️.ts";',
  'import type { PickedFileFormatCatalogPortV1 }',
].join("\n"));
port = replace(port,'/** 🪝️ Every port a shell host reads. */',[
  '/** 🎫️ Identifies a first archive capture by the accepted guest source and its exact Store frontier. */',
  'export type FirstArchiveCaptureRequestV1 = Readonly<{',
  '  actor: string;',
  '  binding: LocalCatalogAdmissionV1["binding"];',
  '  schema: string;',
  '  frontier: NaturalArchiveFrontier;',
  '  source: NaturalSourceAcceptance["loan"] & Readonly<{ sequence: bigint }>;',
  '}>;',
  '',
  'export type FirstArchiveCapturePortV1 = (request: FirstArchiveCaptureRequestV1, signal: AbortSignal) => Promise<NaturalArchiveCapture>;',
  '',
  '/** 🪝️ Every port a shell host reads. */',
].join("\n"));
port = replace(port,'  formatCatalog?: PickedFileFormatCatalogPortV1;', '  formatCatalog?: PickedFileFormatCatalogPortV1;\n  firstArchiveCapture?: FirstArchiveCapturePortV1;');
const os = renderer.slice(0,renderer.indexOf("/🔨️modules/"));
const publicPort = join(os,"📦️packages/🟦️typescript/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🪝️host-ports/🟦️.ts");
const publicReact = join(renderer,"../🎯️targets/⚛️react/🗣️interpreter-and-shell/🟦️.ts");
const sha = (s:string)=>new Bun.CryptoHasher("sha256").update(s).digest("hex");
rows.push([portPath,sha(priorPort),port]);
for (const path of [publicPort, publicReact]) {
  const prior = await Bun.file(path).text();
  const published = replace(prior,"export type { CatalogAdmissionPortV1, DocumentAccessPortV1, PluginDistributionPortV1, ShellHostPortsV1 }","export type { CatalogAdmissionPortV1, DocumentAccessPortV1, FirstArchiveCapturePortV1, FirstArchiveCaptureRequestV1, PluginDistributionPortV1, ShellHostPortsV1 }");
  rows.push([path,sha(prior),published]);
}
for (const [path,,source] of rows) new Bun.Transpiler({loader:path.endsWith(".tsx")?"tsx":"ts"}).transformSync(source);
await Bun.write(join(output,"overlay.json"),JSON.stringify(rows));
await Bun.write(join(output,"scope.tsx"),body);
console.log("[DEBUG] Staged the Shell publication route with exact accepted source, Store frontier and capture port");
