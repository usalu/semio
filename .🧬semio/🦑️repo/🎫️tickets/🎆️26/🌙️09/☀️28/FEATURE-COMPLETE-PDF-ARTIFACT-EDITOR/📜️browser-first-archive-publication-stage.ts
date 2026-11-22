/** 🗂️ Stages catalog publication from a durably stored first archive without materializing its bytes. */
import assert from "node:assert/strict";
import { mkdir } from "node:fs/promises";
import { join } from "node:path";
const root="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗂️local-catalog/📥️artifact";
const generated=join(import.meta.dir,"🗑️generated","browser-first-archive-publication");
await mkdir(generated,{recursive:true});
const artifactPath=join(root,"🟦️.ts");
const openingPath=join(root,"💾️opening/🟦️.ts");
const [artifact,opening]=await Promise.all([Bun.file(artifactPath).text(),Bun.file(openingPath).text()]);
const replace=(s:string,old:string,next:string)=>{
  assert.equal(s.split(old).length,2,old.slice(0,100));
  return s.replace(old,next);
};
const first=artifact.indexOf("/** 📥️ Keeps the imported document");
assert(first>=0);
const common=[
  "type LocalArtifactDestinationV1 = Pick<LocalCatalogAdmissionV1, \"document\" | \"binding\">;",
  "type LocalArtifactIdentityV1 = Readonly<{ documentId: string; schema: string; name: string; surface: LocalDocumentSurface }>;",
  "",
  "/** 📁️ Resolves the existing folder binding for the exact imported document identity. */",
  "export async function localArtifactDestinationV1(input: LocalArtifactIdentityV1, dataDir: string | undefined, nowMs: number): Promise<LocalArtifactDestinationV1 | Readonly<{ refusal: LocalCatalogRefusalV1 }>> {",
  ...artifact.slice(first).split("\n").filter(line=>/^  (if \(!dataDir|if \(!bounded|const digest|const basename|const target|if \(target.length)/u.test(line))
    .map(line=>line.replace(" || input.archive.parent_pack.length === 0","")),
  "  return {",
  '    document: { documentId: input.documentId, schema: input.schema, name: input.name, surface: input.surface, storage: "folder", target, admittedAtMs: nowMs },',
  '    binding: { kind: "folder", dataClass: "persistedLocalOnly", path: target },',
  "  };",
  "}",
  "",
  "/** 📥️ Keeps the imported document's original archive at the validated destination. */",
  "export async function localArtifactAdmissionV1(input: LocalArtifactIdentityV1 & Readonly<{ archive: DocumentArchivePack }>, dataDir: string | undefined, nowMs: number): Promise<LocalCatalogAdmissionV1 | Readonly<{ refusal: LocalCatalogRefusalV1 }>> {",
  '  if (!dataDir) return { refusal: "no-data-folder" };',
  '  if (input.archive.parent_pack.length === 0) return { refusal: "invalid-request" };',
  "  const destination = await localArtifactDestinationV1(input, dataDir, nowMs);",
  '  if ("refusal" in destination) return destination;',
  "  try { return { ...destination, archive: encodeDocumentArchiveBytes(input.archive) }; }",
  '  catch { return { refusal: "invalid-request" }; }',
  "}",
  "",
].join("\n");
assert(common.includes("STORAGE_SUFFIX_CAPACITY") && common.includes("SHA-256"),"Current destination proof drift");
const nextArtifact=artifact.slice(0,first)+common;
let nextOpening=replace(opening,'import { localArtifactAdmissionV1 }','import { localArtifactAdmissionV1, localArtifactDestinationV1 }');
const structStart=nextOpening.indexOf('export async function keepArtifactOpeningV1(port:');
const structEnd=nextOpening.indexOf("}>): Promise<boolean> {",structStart);
assert(structStart>=0&&structEnd>structStart);
const oldPort=nextOpening.slice(nextOpening.lastIndexOf("/** 💾️",structStart),structEnd+"}>): Promise<boolean> {".length);
const openingTypes=[
  'type ArtifactDestination = Pick<LocalCatalogAdmissionV1, "document" | "binding">;',
  "type OpeningPort<A extends ArtifactDestination> = Readonly<{",
  "  name: string; schema: string; surface: LocalDocumentSurface; dataDir: string | undefined; nowMs: number;",
  "  signal: AbortSignal; current: () => boolean; progress: (completed: number, total: number) => void;",
  "  identity: () => Promise<Readonly<{ parent_document_id: string | null }>>;",
  "  attach: (admission: A) => Promise<KeptArtifactReceiptV1 | null>;",
  "  record: (document: LocalDocument) => Promise<boolean>;",
  "  publish: (document: LocalDocument) => boolean;",
  "  close: (receipt: KeptArtifactReceiptV1) => Promise<void>;",
  "  unlist: (document: LocalDocument) => Promise<void>;",
  "  report: (error: unknown) => void;",
  "}>;",
  "",
  "/** 💾️ Publishes an imported document only after its complete archive and catalog entry are durable. */",
  "export function keepArtifactOpeningV1(port: OpeningPort<LocalCatalogAdmissionV1> & Readonly<{",
  "  archive: () => Promise<DocumentArchivePack>;",
  "  write: (admission: LocalCatalogAdmissionV1) => Promise<boolean>;",
  "}>): Promise<boolean> {",
  "  return keepOpening(port, async (documentId, checkpoint) => {",
  "    const archive = await port.archive();",
  "    checkpoint(1);",
  "    return localArtifactAdmissionV1({ documentId, schema: port.schema, name: port.name, surface: port.surface, archive }, port.dataDir, port.nowMs);",
  "  }, admission => port.write(admission));",
  "}",
  "",
  "/** 📂️ Publishes a fresh editor after the folder has sealed and read back its scoped first archive. */",
  "export function keepFirstArtifactOpeningV1(port: OpeningPort<ArtifactDestination> & Readonly<{",
  "  persist: (destination: ArtifactDestination) => Promise<boolean>;",
  "}>): Promise<boolean> {",
  "  return keepOpening(port, documentId => localArtifactDestinationV1(",
  "    { documentId, schema: port.schema, name: port.name, surface: port.surface }, port.dataDir, port.nowMs,",
  "  ), destination => port.persist(destination));",
  "}",
  "",
  "async function keepOpening<A extends ArtifactDestination>(",
  "  port: OpeningPort<A>,",
  "  admit: (documentId: string, checkpoint: (phase: number) => void) => Promise<A | Readonly<{ refusal: string }>>,",
  "  persist: (admission: A) => Promise<boolean>,",
  "): Promise<boolean> {",
].join("\n");
nextOpening=replace(nextOpening,oldPort,openingTypes);
const oldAdmission=[
  "    const archive = await port.archive();",
  "    checkpoint(1);",
  "    const admission = await localArtifactAdmissionV1({ documentId: identity.parent_document_id, schema: port.schema, name: port.name, surface: port.surface, archive }, port.dataDir, port.nowMs);",
].join("\n");
nextOpening=replace(nextOpening,oldAdmission,"    const admission = await admit(identity.parent_document_id, checkpoint);");
nextOpening=replace(nextOpening,"    if (!(await port.write(admission)))","    if (!(await persist(admission)))");
const rows=[[artifactPath,artifact,nextArtifact],[openingPath,opening,nextOpening]];
for(const [, ,body] of rows)new Bun.Transpiler({loader:"ts"}).transformSync(body);
const sha=(body:string)=>new Bun.CryptoHasher("sha256").update(body).digest("hex");
await Bun.write(join(generated,"overlay.json"),JSON.stringify(rows.map(([path,prior,body])=>[path,sha(prior),body])));
await Bun.write(join(generated,"artifact.ts"),nextArtifact);
await Bun.write(join(generated,"opening.ts"),nextOpening.replace('"../🟦️.ts";','"./artifact.ts";'));
console.log("[DEBUG] Staged the shared catalog opening gate and metadata-only folder destination");
