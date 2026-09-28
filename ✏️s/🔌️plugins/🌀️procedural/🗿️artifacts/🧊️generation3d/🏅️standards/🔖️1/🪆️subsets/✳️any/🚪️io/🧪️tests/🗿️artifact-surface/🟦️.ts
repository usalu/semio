import assert from "node:assert/strict";
import { readFileSync, readdirSync } from "node:fs";
import { fileURLToPath } from "node:url";

/** 🗿️ Third-party twin of the ARTIFACT-IO SURFACE (`🧫️fixtures/🚪️io/🗿️artifact-surface.json`) — an
 * independent TypeScript model of the import/export surface the generation3d editor and viewer
 * publish, driven from the SAME fixture the Rust laws drive and importing none of their code.
 *
 * 🔍️ Independent in the way that matters: where Rust asks `document_io` what a format's extension,
 * MIME and binary-ness are, this file goes to each `s.stdio.<format>` artifact's OWN
 * `📜️artifact-definition.json` on disk and reads them there. A roster that drifted would have to
 * drift identically in the Rust module, in the fixture and in every owning artifact's definition
 * before both halves agreed again.
 *
 * 📦️ The framework reassembles a picked file's chunks before the import runs
 * (`semio_framework::kernel::ImportStaging`, whose `stagingCases` live in the kernel's own
 * `🧫️fixtures/📤️file-open-import/🔣️.json`), so this surface states only the one size refusal.
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

interface ArtifactSurfaceFixture {
  readonly schema: string;
  readonly artifactKind: string;
  readonly exportFormats: readonly FormatRow[];
  readonly importFormats: readonly FormatRow[];
  readonly acceptFilter: string;
  readonly editorActions: readonly ActionRow[];
  readonly viewerActions: readonly ActionRow[];
  readonly publicationLanes: Record<string, Record<string, readonly string[]>>;
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
  const fixture = JSON.parse(readFileSync(`${here}/../../../🧫️fixtures/🚪️io/🗿️artifact-surface.json`, "utf8")) as ArtifactSurfaceFixture;
  assert.equal(fixture.schema, "generation3d.artifact-io-surface.v1");
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

  // 📥️ Every declared import dialect is offered, each exactly once.
  const offered = fixture.importFormats.map((row) => row.id);
  assert.equal(new Set(offered).size, offered.length, "no import id is offered twice");
  assert.equal(fixture.acceptFilter, offered.map((id) => fixture.importFormats.find((row) => row.id === id)!.extension).join(","), "the accept filter is every importable extension in roster order");
  for (const row of fixture.importFormats) assert.ok(row.extension.startsWith("."), `${row.id}: an accept entry is an extension`);

  // 🎬️ Both surfaces really OFFER the verbs — the finding this lane closed was that neither did.
  const editorIds = fixture.editorActions.map((row) => row.id);
  assert.deepEqual(editorIds, ["importDocumentRequest", "exportDocument", "importDocument"], "the editor offers the picker, the export and the import sink");
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

  // 📏️ An import refuses by size only, with one stable code: the framework hands the whole picked file.
  assert.deepEqual(Object.keys(fixture.faultCodes), ["capacity"], "the import's one refusal is its size budget");
  assert.match(fixture.faultCodes.capacity ?? "", /^generation3d-import-/u, "the capacity code is this artifact's own");

  // 📦️ Every shape a shell can answer a pick with decodes to the same bytes.
  for (const row of fixture.dataUrl.cases) assert.equal(decodePayload(row.payload), row.bytes, `${row.id}: decoded payload`);

  console.log(
    `generation3d artifact-io export=${fixture.exportFormats.length} import=${fixture.importFormats.length} ` +
      `editorActions=${fixture.editorActions.length} viewerActions=${fixture.viewerActions.length} faultCodes=${Object.keys(fixture.faultCodes).join(",")} ` +
      `accept=${fixture.acceptFilter}`,
  );
}
