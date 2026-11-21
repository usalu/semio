/** 🧪️ Exercises typed Save qualification through the actual staged host lifecycle and chooser. */
import assert from "node:assert/strict";
import {dirname, join, resolve} from "node:path";
import {pathToFileURL} from "node:url";
import {encodePackValue} from "@semio-tech/framework/packs";

type Row = [string, string | null, string];
const output = join(import.meta.dir, "🗑️generated/classic-graph");
const staged = new Map((await Bun.file(join(output, "registered-save-overlay.json")).json() as Row[])
  .map(([file, , source]) => [file, source]));
const path = [...staged.keys()].find(path => path.includes("🎬️media/📤️file/"))!;
const lifecycle = [...staged.keys()].find(path => path.includes("🎬️media/🚚️lifecycle/"))!;
const lifecycleOut = join(output, "life.ts");
const fileOut = join(output, "save.ts");
function relocate(file: string, text: string, rewrite: (resolved: string) => string = path => path): string {
  return text.replace(/(from ["'])(\.[^"']+)(["'])/g, (_all, prefix, imported, suffix) =>
    prefix + rewrite(resolve(dirname(file), imported)) + suffix);
}
await Bun.write(lifecycleOut, relocate(lifecycle, staged.get(lifecycle)!));
await Bun.write(fileOut, relocate(path, staged.get(path)!, file => file === resolve(lifecycle) ? lifecycleOut : file));
const target = process.argv.includes("--live") ? resolve(path) : fileOut;
const {deliverNaturalMediaFileV1, downloadDeclaredMediaFileV1} = await import(pathToFileURL(target).href);
const handle = {app_instance_id: 7, parent_document_id: "open", base_revision: 11n, operation_id: 14n, generation: 1n};
const declaration = {outputPort: "artifact:native", filename: "test.pdf", formatKind: "s.stdio.pdf@1.7",
  mediaType: "application/pdf", portMediaType: {class: "data", form: "document"}};
const descriptor = [...encodePackValue({portId: declaration.outputPort, mediaType: declaration.portMediaType,
  wire: {kind: "binary", format_kind: declaration.formatKind}})];
type Case = {id: string; status: "failed" | "cancelled" | "complete"; detail?: string; foreign?: boolean;
  moved?: boolean; hasDeclaration?: boolean; sinkFailure?: boolean; outputBeforeRefusal?: boolean; sequence: string[]; error?: string};
const cases: Case[] = [
  {id: "typed reference", status: "failed", detail: "media-export.reference-required", sequence: ["archive", "reference"]},
  {id: "changed archive", status: "failed", detail: "media-export.reference-required", moved: true, sequence: ["archive"], error: "moved"},
  {id: "foreign status", status: "failed", detail: "media-export.reference-required", foreign: true, sequence: ["cancel"], error: "media-export.status-owner"},
  {id: "output before decline", status: "failed", detail: "media-export.reference-required", outputBeforeRefusal: true, sequence: [], error: "media-export.reference-required"},
  {id: "cancelled", status: "cancelled", sequence: [], error: "natural-file.export-owner-retired"},
  {id: "worker failure", status: "failed", detail: "ordinary failure", sequence: [], error: "ordinary failure"},
  {id: "qualified output", status: "complete", sequence: ["sink", "page", "archive", "close"]},
  {id: "sink failure", status: "complete", sinkFailure: true, sequence: ["sink", "page", "abort"], error: "sink failed"},
  {id: "reference-only app", status: "failed", hasDeclaration: false, sequence: ["reference"]},
];
for (const row of cases) {
  let reply = 0;
  const sequence: string[] = [];
  const verify = async () => {
    sequence.push("archive");
    if (row.moved) throw new Error("moved");
  };
  let error: string | undefined;
  try {
    await deliverNaturalMediaFileV1(row.hasDeclaration === false ? null : () => downloadDeclaredMediaFileV1({
      declaration, instanceId: handle.app_instance_id, signal: new AbortController().signal, current: () => true, progress: () => {},
      yieldTurn: async () => {}, beforeDelivery: verify,
      sinkFactory: async () => {
        sequence.push("sink");
        return {
          write: async (bytes: Uint8Array) => {
            sequence.push("page");
            assert.deepEqual([...bytes], [37, 80, 68, 70]);
            if (row.sinkFailure) throw new Error("sink failed");
          },
          close: async () => { sequence.push("close"); },
          abort: async () => { sequence.push("abort"); },
        };
      },
      port: {
        readAppDocumentIdentity: async () => handle,
        submitMediaExport: async (instance: number, port: string, owner: string, revision: bigint) => {
          assert.deepEqual([instance, port, owner, revision], [7, "artifact:native", "open", 11n]);
          return handle;
        },
        pollMediaExport: async () => ({
          handle: row.foreign ? {...handle, base_revision: 12n} : handle, state: row.status,
          checkpoint_available: false, applied_progress: 0n, detail: row.detail ?? "",
          total_bytes: row.status === "complete" || row.outputBeforeRefusal ? 4n : 0n, mime_type: declaration.mediaType,
          descriptor: row.status === "complete" || row.outputBeforeRefusal ? descriptor : [],
        }),
        takeMediaExportChunk: async () => (++reply === 1
          ? {handle, data: Uint8Array.from([37, 80, 68, 70]), terminal: false}
          : {handle, data: new Uint8Array(), terminal: true}),
        cancelMediaExport: async () => { sequence.push("cancel"); },
      },
    }), verify, async () => { sequence.push("reference"); });
  } catch (reason) { error = reason instanceof Error ? reason.message : String(reason); }
  assert.deepEqual({sequence, error, reply}, {sequence: row.sequence, error: row.error, reply: row.status === "complete" ? 2 : 0}, row.id);
  console.log("[DEBUG] " + row.id + " " + sequence.join(" -> "));
}
