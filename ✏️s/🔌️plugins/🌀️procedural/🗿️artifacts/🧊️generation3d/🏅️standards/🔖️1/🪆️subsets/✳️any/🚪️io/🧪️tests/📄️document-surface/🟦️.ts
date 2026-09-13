import assert from "node:assert/strict";
import { readFileSync, readdirSync } from "node:fs";
import { fileURLToPath } from "node:url";

/** 📄️ Third-party twin of the DOCUMENT-IO SURFACE (`🧫️fixtures/🚪️io/📄️document-surface.json`) — an
 * independent TypeScript model of the import/export surface the generation3d editor and viewer
 * publish, driven from the SAME fixture the Rust laws drive and importing none of their code.
 *
 * 🔍️ Independent in the way that matters: where Rust asks `document_io` what a format's extension,
 * MIME and binary-ness are, this file goes to each `s.stdio.<format>` artifact's OWN
 * `📜️artifact-definition.json` on disk and reads them there. A roster that drifted would have to
 * drift identically in the Rust module, in the fixture and in every owning artifact's definition
 * before both halves agreed again.
 *
 * 🧩️ The chunk ledger is re-derived from the fixture's own prose rather than called: an import
 * arrives one chunk at a time, so a run that staged, retransmitted or gapped differently in one
 * implementation would answer a different `nextChunk` here.
 *
 * @see ./🦀️.rs — the Rust half, which additionally drives `parry3d` over every geometry format.
 * @see ../../🦀️.rs — `document_io`, the module under test.
 */

//#region 🧫️Fixture
interface FormatRow {
  readonly id: string;
  readonly labelEn?: string;
  readonly labelDe?: string;
  readonly kindId?: string;
  readonly extension: string;
  readonly mime?: string;
  readonly binary?: boolean;
  readonly filename?: string;
  readonly geometry?: boolean;
}

interface ActionRow {
  readonly id: string;
  readonly kind: string;
  readonly inPalette: boolean;
  readonly labelEn: string;
  readonly labelDe: string;
  readonly chord?: string;
}

interface ChunkCase {
  readonly id: string;
  readonly payloadBytes: number;
  readonly chunks: number;
}

interface StagingCase {
  readonly id: string;
  readonly events: readonly { readonly chunk: number; readonly chunkCount: number }[];
  readonly outcome: string;
  readonly nextChunk?: number;
}

interface DocumentSurfaceFixture {
  readonly schema: string;
  readonly artifactKind: string;
  readonly exportFormats: readonly FormatRow[];
  readonly importFormats: readonly FormatRow[];
  readonly importWithheld: readonly { readonly id: string; readonly reason: string }[];
  readonly acceptFilter: string;
  readonly editorActions: readonly ActionRow[];
  readonly viewerActions: readonly ActionRow[];
  readonly publicationLanes: Record<string, Record<string, readonly string[]>>;
  readonly chunking: { readonly chunkBytes: number; readonly cases: readonly ChunkCase[]; readonly staging: readonly StagingCase[] };
  readonly faultCodes: Record<string, string>;
  readonly dataUrl: { readonly cases: readonly { readonly id: string; readonly payload: string; readonly bytes: string }[] };
}
//#endregion 🧫️Fixture

//#region 🗄️OwningArtifacts
interface StdioRepresentation {
  readonly id: string;
  readonly mimes: readonly string[];
  readonly extensions: readonly string[];
  readonly is_binary: boolean;
}

/** 🗄️ Every `s.stdio.*` representation on disk, indexed by its representation id — read from the
 * artifacts themselves, so nothing in this file restates a format's file facts either. */
function stdioRepresentations(stdioRoot: string): Map<string, StdioRepresentation> {
  const found = new Map<string, StdioRepresentation>();
  for (const entry of readdirSync(stdioRoot, { withFileTypes: true })) {
    if (!entry.isDirectory()) continue;
    let raw: string;
    try {
      raw = readFileSync(`${stdioRoot}/${entry.name}/📜️artifact-definition.json`, "utf8");
    } catch {
      continue;
    }
    const definition = JSON.parse(raw) as { representations?: readonly StdioRepresentation[] };
    for (const representation of definition.representations ?? []) found.set(representation.id, representation);
  }
  return found;
}
//#endregion 🗄️OwningArtifacts

//#region 📥️ChunkLedger
/** 📥️ The host's slicing, re-derived: a payload is cut by UTF-8 EXTENT so no slice ever splits a
 * code point, and an empty payload is still one chunk (there is always something to deliver). */
function chunksOf(payload: string, chunkBytes: number): string[] {
  const pages: string[] = [];
  let page = "";
  let pageBytes = 0;
  for (const character of payload) {
    const code = character.codePointAt(0) ?? 0;
    const characterBytes = code < 0x80 ? 1 : code < 0x800 ? 2 : code < 0x10000 ? 3 : 4;
    if (pageBytes + characterBytes > chunkBytes) {
      pages.push(page);
      page = "";
      pageBytes = 0;
    }
    page += character;
    pageBytes += characterBytes;
  }
  if (page.length > 0 || pages.length === 0) pages.push(page);
  return pages;
}

type LedgerStep = { readonly kind: "complete"; readonly pages: number } | { readonly kind: "staged"; readonly nextChunk: number; readonly chunkCount: number } | { readonly kind: "fault"; readonly fault: string };

/** 🧵️ The staging ledger, re-derived from the fixture's prose: a run keyed by `(name, chunkCount)`
 * admits chunks in order, acknowledges a retransmission at the cursor it stands on, and drops itself
 * on a gap rather than resuming into bytes nobody can account for. */
class ImportLedger {
  private run: { name: string; chunkCount: number; nextChunk: number; pages: number } | undefined;

  admit(name: string, chunk: number, chunkCount: number, maximumChunks: number): LedgerStep {
    if (chunkCount === 0 || chunkCount > maximumChunks || chunk >= chunkCount) return { kind: "fault", fault: "envelope" };
    if (chunkCount === 1) return { kind: "complete", pages: 1 };
    const held = this.run && this.run.name === name && this.run.chunkCount === chunkCount ? this.run : undefined;
    if (!held) {
      if (chunk !== 0) return { kind: "fault", fault: "gap" };
      this.run = { name, chunkCount, nextChunk: 0, pages: 0 };
    } else if (held.nextChunk !== chunk) {
      if (chunk !== 0 && chunk < held.nextChunk) return { kind: "staged", nextChunk: held.nextChunk, chunkCount: held.chunkCount };
      if (chunk !== 0) {
        this.run = undefined;
        return { kind: "fault", fault: "gap" };
      }
      this.run = { name, chunkCount, nextChunk: 0, pages: 0 };
    }
    const run = this.run!;
    run.pages += 1;
    run.nextChunk = chunk + 1;
    if (run.nextChunk < run.chunkCount) return { kind: "staged", nextChunk: run.nextChunk, chunkCount: run.chunkCount };
    const pages = run.pages;
    this.run = undefined;
    return { kind: "complete", pages };
  }

  get open(): number {
    return this.run ? 1 : 0;
  }
}
//#endregion 📥️ChunkLedger

/** 📦️ The payload shapes a shell can answer a file pick with, decoded independently. */
function decodePayload(payload: string): string {
  const comma = payload.indexOf(",");
  if (comma >= 0) {
    const header = payload.slice(0, comma);
    if (header.startsWith("data:")) {
      const body = payload.slice(comma + 1);
      return header.endsWith(";base64") ? Buffer.from(body, "base64").toString("utf8") : body;
    }
  }
  return payload;
}

export function testGeneration3dDocumentIoSurface(): void {
  const here = fileURLToPath(new URL(".", import.meta.url));
  const fixture = JSON.parse(readFileSync(`${here}/../../../🧫️fixtures/🚪️io/📄️document-surface.json`, "utf8")) as DocumentSurfaceFixture;
  assert.equal(fixture.schema, "generation3d.document-io-surface.v1");
  assert.equal(fixture.artifactKind, "s.procedural.generation3d");

  const representations = stdioRepresentations(`${here}/../../../../../../../../../../🗄️stdio/🗿️artifacts`);
  assert.ok(representations.size > 0, "the stdio artifacts' own definitions were found on disk");

  // 📤️ Every export row's file facts come from its OWNING artifact, and its download envelope
  // follows from them — the filename is the artifact kind plus that artifact's own extension, and a
  // format is base64'd exactly when its owner calls itself binary.
  for (const row of fixture.exportFormats) {
    const representation = representations.get(row.kindId ?? "");
    assert.ok(representation, `${row.id}: its owning stdio artifact publishes ${row.kindId ?? "<no kind id>"}`);
    assert.equal(representation.extensions[0], row.extension, `${row.id}: extension`);
    assert.equal(representation.mimes[0], row.mime, `${row.id}: MIME`);
    assert.equal(representation.is_binary, row.binary, `${row.id}: binary claim`);
    assert.equal(row.filename, `generation3d${row.extension}`, `${row.id}: download filename`);
    // 🗣️ English first, German second, and no row may exist in one language only.
    assert.ok(row.labelEn && row.labelEn.length > 0, `${row.id}: English label`);
    assert.ok(row.labelDe && row.labelDe.length > 0, `${row.id}: German label`);
  }
  assert.equal(new Set(fixture.exportFormats.map((row) => row.id)).size, fixture.exportFormats.length, "no export id is offered twice");

  // 📥️ Offered plus withheld is the whole registry, and the two withheld rows carry a stated reason
  // rather than being silently absent.
  const offered = fixture.importFormats.map((row) => row.id);
  const withheld = fixture.importWithheld.map((row) => row.id);
  assert.equal(new Set([...offered, ...withheld]).size, offered.length + withheld.length, "a format is either offered or withheld, never both");
  for (const row of fixture.importWithheld) assert.ok(row.reason.length > 20, `${row.id}: the withheld row states why`);
  assert.equal(fixture.acceptFilter, offered.map((id) => fixture.importFormats.find((row) => row.id === id)!.extension).join(","), "the accept filter is every importable extension in roster order");
  for (const row of fixture.importFormats) assert.ok(row.extension.startsWith("."), `${row.id}: an accept entry is an extension`);

  // 🎬️ Both surfaces really OFFER the verbs — the finding this lane closed was that neither did.
  const editorIds = fixture.editorActions.map((row) => row.id);
  assert.deepEqual(editorIds, ["importDocumentRequest", "exportDocument", "importDocument"], "the editor offers the picker, the export and the chunk sink");
  assert.deepEqual(fixture.viewerActions.map((row) => row.id), ["exportDocument"], "a viewer exports and never imports");
  for (const row of [...fixture.editorActions, ...fixture.viewerActions]) {
    assert.ok(row.labelEn.length > 0 && row.labelDe.length > 0, `${row.id}: both languages`);
    assert.notEqual(row.labelEn, row.labelDe, `${row.id}: a German label that equals the English one is an untranslated row`);
    if (row.inPalette) assert.ok(row.chord, `${row.id}: a palette verb is reachable by keyboard too`);
  }
  // 🔒️ A viewer may export because an export writes no store lane; it may not import because an
  // import replaces the document.
  assert.deepEqual(fixture.publicationLanes.viewer, { exportDocument: ["HostOnly"] });
  assert.deepEqual(fixture.publicationLanes.editor.importDocument, ["Artifact", "Config"]);
  assert.deepEqual(fixture.publicationLanes.editor.exportDocument, ["HostOnly"]);
  assert.deepEqual(fixture.publicationLanes.editor.importDocumentRequest, ["HostOnly"]);

  // 📦️ One payload is cut into exactly the declared chunks, and the chunks reassemble it.
  for (const row of fixture.chunking.cases) {
    const payload = "x".repeat(row.payloadBytes);
    const chunks = chunksOf(payload, fixture.chunking.chunkBytes);
    assert.equal(chunks.length, row.chunks, `${row.id}: chunk count`);
    assert.ok(chunks.every((chunk) => new TextEncoder().encode(chunk).length <= fixture.chunking.chunkBytes), `${row.id}: no chunk exceeds the extent`);
    assert.equal(chunks.join(""), payload, `${row.id}: the chunks reassemble the payload`);
  }
  // 🔤️ A multi-byte payload is still sliced by UTF-8 extent, never by code unit.
  const multibyte = "ü".repeat(fixture.chunking.chunkBytes);
  for (const chunk of chunksOf(multibyte, fixture.chunking.chunkBytes)) assert.ok(new TextEncoder().encode(chunk).length <= fixture.chunking.chunkBytes, "a non-ASCII payload still respects the byte extent");

  // 🧵️ Every declared arrival answers the same step, which is what an import's progress and
  // cancellation are made of.
  const maximumChunks = 16;
  for (const row of fixture.chunking.staging) {
    const ledger = new ImportLedger();
    let step: LedgerStep | undefined;
    for (const event of row.events) step = ledger.admit("cube.stl", event.chunk, event.chunkCount, maximumChunks);
    assert.ok(step, `${row.id}: at least one event`);
    if (row.outcome === "complete") {
      assert.equal(step.kind, "complete", `${row.id}: the run closed`);
      assert.equal(ledger.open, 0, `${row.id}: a closed run holds no slot`);
    } else if (row.outcome === "staged") {
      assert.equal(step.kind, "staged", `${row.id}: the run is still open`);
      assert.equal((step as { nextChunk: number }).nextChunk, row.nextChunk, `${row.id}: the progress cursor`);
      assert.equal(ledger.open, 1, `${row.id}: exactly one run stays open`);
    } else {
      assert.equal(step.kind, "fault", `${row.id}: the arrival was refused`);
      assert.equal((step as { fault: string }).fault, row.outcome, `${row.id}: which fault`);
      assert.ok(fixture.faultCodes[row.outcome], `${row.id}: the fault carries a stable wire code`);
      assert.equal(ledger.open, 0, `${row.id}: a refused run keeps no slot`);
    }
  }

  // 📦️ Every shape a shell can answer a pick with decodes to the same bytes.
  for (const row of fixture.dataUrl.cases) assert.equal(decodePayload(row.payload), row.bytes, `${row.id}: decoded payload`);

  console.log(
    `generation3d document-io export=${fixture.exportFormats.length} import=${fixture.importFormats.length} withheld=${fixture.importWithheld.length} ` +
      `editorActions=${fixture.editorActions.length} viewerActions=${fixture.viewerActions.length} chunkBytes=${fixture.chunking.chunkBytes} ` +
      `chunkCases=${fixture.chunking.cases.length} stagingCases=${fixture.chunking.staging.length} accept=${fixture.acceptFilter}`,
  );
}
