import assert from "node:assert/strict";
import Ajv from "ajv";
import { Document, NodeIO } from "@gltf-transform/core";
import { readFileSync, readdirSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import { BufferGeometry, Float32BufferAttribute, Int8BufferAttribute, Uint8BufferAttribute, Uint16BufferAttribute, Matrix4, Quaternion, ShapeUtils, Vector2, Vector3 } from "three";
import { toTrianglesDrawMode } from "three/addons/utils/BufferGeometryUtils.js";
import { polygonMeshFromPrepared, polygonMeshFromObj, polygonMeshFromPly, mergePreparedMeshes, meshFormatDiagnostics, exportSourceChannels, applyGltfSceneToHost, materialFieldsForExport, gltfMaterialSurface, preparedGltfChannels, gltfImportAdmission, restoreGltfAuthoredAttributes } from "../../🟦️.ts";
import { parseGltfDocument } from "../../../../../../../../../../🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🟦️.ts";
import { binary64, binary64Value } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import type { FlowHostSnapshot, NeuralDictionary } from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import type { MeshAttribute } from "../../../../../../../../../../🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🟦️.ts";

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
  readonly registryText: { readonly source: string; readonly sourceKinds: readonly string[]; readonly target: string; readonly preserveGraph: boolean };
  readonly exportInputs: { readonly document: readonly string[]; readonly preparedGeometry: readonly string[]; readonly rejectGeometryAsDocument: string; readonly rejectDocumentAsGeometry: string };
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
  testGeneration3dIoInputContracts();
  testGeneration3dMeshSurface();
  testGeneration3dPolygonImport();
  testGeneration3dGltfSceneImport();

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

/** 🚪️ Validates the neutral document/prepared-geometry contract against the independent schema oracle. */
export function testGeneration3dIoInputContracts(): number {
  const here = fileURLToPath(new URL(".", import.meta.url));
  const fixture = JSON.parse(readFileSync(`${here}/../../../🧫️fixtures/🚪️io/🗿️artifact-surface.json`, "utf8")) as ArtifactSurfaceFixture;
  const schema = JSON.parse(readFileSync(`${here}/../../../🧬️schema/🔣️.json`, "utf8"));
  const validate = new Ajv({ strict: false }).compile(schema.$defs.Generation3dExportInputs);
  assert.ok(validate(fixture.exportInputs), JSON.stringify(validate.errors));
  assert.equal(validate({ ...fixture.exportInputs, document: ["obj"] }), false);
  assert.equal(validate({ ...fixture.exportInputs, preparedGeometry: ["txt"] }), false);
  assert.deepEqual(fixture.exportInputs.document, fixture.exportFormats.filter(row => !row.geometry).map(row => row.id));
  assert.deepEqual(fixture.exportInputs.preparedGeometry, fixture.exportFormats.filter(row => row.geometry).map(row => row.id));


  const validateRegistry = new Ajv({ strict: false }).compile(schema.$defs.Generation3dRegistryTextRoundTrip);
  assert.ok(validateRegistry(fixture.registryText), JSON.stringify(validateRegistry.errors));
  assert.equal(validateRegistry({ ...fixture.registryText, target: "s.stdio.obj" }), false);
  assert.equal(validateRegistry({ ...fixture.registryText, sourceKinds: ["prepared-mesh"] }), false);
  return 8;
}

/** 🎨️ Prepared geometry carries the same authored channels as the independent buffer oracle. */
export function testGeneration3dMeshSurface(): number {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎨️surface/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv({ strict: false }).compile(schema.$defs.Generation3dPreparedMesh);
  assert.equal(fixture.schema, "generation3d.mesh-surface.v1");
  assert.ok(validate(fixture.gltfExport.prepared), JSON.stringify(validate.errors));
  const cutoffCase = JSON.parse(readFileSync(new URL("../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🎨️world3d-inline-surface/🔣️.json", import.meta.url), "utf8")).alphaCutoffCase;
  const cutoffPrepared = structuredClone(fixture.gltfExport.prepared);
  cutoffPrepared.materials.paint.alphaCutoff = cutoffCase.value;
  assert.ok(validate(cutoffPrepared), JSON.stringify(validate.errors));
  assert.deepEqual(preparedGltfChannels(cutoffPrepared as Parameters<typeof preparedGltfChannels>[0]), preparedGltfChannels(fixture.gltfExport.prepared));
  const cutoffDocument = parseGltfDocument({ asset: { version: "2.0" }, materials: [{ alphaMode: "MASK", alphaCutoff: cutoffCase.value }], meshes: [{ primitives: [{ attributes: {}, material: 0 }] }] });
  assert.equal(gltfMaterialSurface(cutoffDocument, cutoffDocument.meshes[0].primitives[0]).alphaCutoff, cutoffCase.value);
  const exportedChannels = preparedGltfChannels(fixture.gltfExport.prepared), gltfOracle = new Document();
  for (const [semantic, expected] of Object.entries(fixture.gltfExport.expanded)) {
    assert.deepEqual(exportedChannels[semantic], expected, `prepared glTF ${semantic}`);
    const width = semantic === "POSITION" ? 3 : semantic.startsWith("TEXCOORD_") ? 2 : semantic === "_temperature" ? 1 : 4;
    const type = width === 1 ? "SCALAR" : width === 2 ? "VEC2" : width === 3 ? "VEC3" : "VEC4";
    const accessor = gltfOracle.createAccessor().setType(type).setArray(new Float32Array(exportedChannels[semantic]));
    assert.deepEqual(Array.from(accessor.getArray()!), expected, `independent glTF Transform ${semantic}`);
  }
  for (const refusal of fixture.gltfExport.refusals.filter((row: { path: string[]; remove?: boolean }) => row.path[0] === "attributes" && !row.remove)) {
    const prepared = structuredClone(fixture.gltfExport.prepared);
    let target = prepared;
    for (const key of refusal.path.slice(0, -1)) target = target[key];
    target[refusal.path.at(-1)] = refusal.value;
    assert.throws(() => preparedGltfChannels(prepared), Error, `prepared glTF refusal ${refusal.message}`);
  }
  const threeExport = new BufferGeometry();
  threeExport.setAttribute("position", new Float32BufferAttribute(fixture.gltfExport.prepared.positions, 3));
  threeExport.setIndex(fixture.gltfExport.prepared.indices);
  const expandedThree = threeExport.toNonIndexed();
  assert.deepEqual(Array.from(expandedThree.getAttribute("position").array), fixture.gltfExport.expanded.POSITION);
  threeExport.dispose(); expandedThree.dispose();
  for (const row of fixture.cases) {
    assert.ok(validate(row.prepared), JSON.stringify(validate.errors));
    const output = polygonMeshFromPrepared(row.prepared);
    assert.deepEqual(output, row.polygon, row.id);
    const geometry = new BufferGeometry();
    geometry.setAttribute("position", new Float32BufferAttribute(row.prepared.positions, 3));
    geometry.setIndex(row.prepared.indices);
    for (const [field, width, channel] of [["normals", 3, "normal"], ["uvs", 2, "uv"], ["colors", 3, "color"]] as const) {
      if (!row.prepared[field]) continue;
      const buffer = new Float32BufferAttribute(row.prepared[field], width);
      geometry.setAttribute(channel, buffer);
      const actual = output.attributes?.[channel]?.values;
      assert.deepEqual(actual?.map(value => (value as number[]).slice(0, width)), Array.from({ length: buffer.count }, (_, index) => Array.from({ length: width }, (_, component) => buffer.array[index * width + component])), `${row.id}: independent ${channel}`);
    }
    geometry.computeBoundingBox();
    assert.deepEqual(geometry.boundingBox?.min.toArray(), [0, 0, 0]);
    assert.deepEqual(geometry.boundingBox?.max.toArray(), [1, 1, 0]);
    geometry.dispose();
  }
  for (const row of fixture.faceChannels) {
    assert.ok(validate(row.prepared), JSON.stringify(validate.errors));
    const polygon = polygonMeshFromPrepared(row.prepared);
    assert.deepEqual(polygon.attributes, row.prepared.attributes, row.id);
    const oracle = new BufferGeometry();
    oracle.setAttribute("position", new Float32BufferAttribute(row.prepared.positions, 3));
    oracle.setIndex(row.prepared.indices);
    const expanded = oracle.toNonIndexed();
    assert.deepEqual(Array.from(expanded.getAttribute("position").array), row.expanded.positions, row.id);
    for (const [name, width, expected] of [["normal", 3, "normals"], ["uv", 2, "uvs"], ["color", 4, "colors"]] as const) {
      const attribute = polygon.attributes![name];
      const values = polygon.faces.flatMap((face, index) => face.map(() => attribute.values[attribute.indices?.[index] ?? index] as number[]));
      const buffer = new Float32BufferAttribute(values.flat(), width);
      expanded.setAttribute(name, buffer);
      assert.deepEqual(Array.from(buffer.array), row.expanded[expected].flat(), `${row.id}: independent Three ${name}`);
    }
    expanded.dispose();
    oracle.dispose();
  }
  for (const row of fixture.gltfAccessors.cases) {
    const document = parseGltfDocument(row.document);
    assert.equal(document.meshes[0].primitives[0].attributes.length, 4);
    const bytes = Buffer.from(row.document.buffers[0].uri.split(",")[1], "base64");
    const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    for (const [index, field, width] of [[1, "normals", 3], [2, "uvs", 2], [3, "colors", 4]] as const) {
      const accessor = row.document.accessors[index];
      const source = row.document.bufferViews[accessor.bufferView];
      const values = Array.from({ length: accessor.count * width }, (_, component) => index === 1 ? view.getInt8(source.byteOffset + Math.floor(component / width) * (source.byteStride ?? width) + component % width) : index === 2 ? view.getUint16(source.byteOffset + component * 2, true) : view.getUint8(source.byteOffset + component));
      const attribute = index === 1 ? new Int8BufferAttribute(values, width, true) : index === 2 ? new Uint16BufferAttribute(values, width, true) : new Uint8BufferAttribute(values, width, true);
      const normalized = Array.from({ length: attribute.count }, (_, vertex) => [attribute.getX(vertex), attribute.getY(vertex), ...(width > 2 ? [attribute.getZ(vertex)] : []), ...(width > 3 ? [attribute.getW(vertex)] : [])]);
      assert.deepEqual(normalized, row.expected[field], `${row.id}: independent Three ${field}`);
    }
  }

  const admission = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🚧️import-admission/🔣️.json", import.meta.url), "utf8"));
  const admittedSource = fixture.gltfSurfaceAuthoring.cases[0].document;
  assert.deepEqual(gltfImportAdmission(parseGltfDocument(admittedSource)), admission.expected);
  const admissionOracle = new Document(), admissionPositions = admissionOracle.createAccessor().setType("VEC3").setArray(new Float32Array(admission.expected.vertices * 3)), admissionIndices = admissionOracle.createAccessor().setType("SCALAR").setArray(new Uint16Array(admission.expected.triangles * 3));
  assert.equal(gltfImportAdmission(parseGltfDocument(admittedSource)).vertices, admissionPositions.getCount());
  assert.equal(gltfImportAdmission(parseGltfDocument(admittedSource)).triangles, admissionIndices.getCount() / 3);
  for (const row of admission.refusals) {
    const invalid = structuredClone(admittedSource), primitive = invalid.meshes[0].primitives[0];
    if (row.accessor !== undefined) invalid.accessors[row.accessor].count = row.count;
    if (row.indicesCount !== undefined) invalid.accessors[primitive.indices].count = row.indicesCount;
    if (row.mode !== undefined) primitive.mode = row.mode;
    if (row.channels !== undefined) {
      if (row.vertexCount !== undefined) { for (const accessor of invalid.accessors) if (accessor.type !== "SCALAR") accessor.count = row.vertexCount; }
      while (Object.keys(primitive.attributes).length < row.channels) { const index = invalid.accessors.push({ ...invalid.accessors[0], type: row.channelType ?? "VEC3" }) - 1; primitive.attributes[`_CHANNEL_${index}`] = index; }
    }
    if (row.primitives !== undefined) invalid.meshes[0].primitives = Array.from({ length: row.primitives }, () => primitive);
    assert.throws(() => gltfImportAdmission(parseGltfDocument(invalid), row.inputBytes ?? 0), /capacity/, row.id);
  }
  for (const row of admission.modes) {
    const document = structuredClone(admittedSource), primitive = document.meshes[0].primitives[0];
    primitive.mode = row.mode;document.accessors[primitive.indices].count = row.count;
    assert.equal(gltfImportAdmission(parseGltfDocument(document)).triangles,row.triangles);
    if (row.mode >= 4) { const geometry = new BufferGeometry().setAttribute("position",new Float32BufferAttribute(new Array(row.count * 3).fill(0),3)).setIndex(Array.from({length:row.count},(_,index) => index));const oracle = row.mode === 4 ? geometry : toTrianglesDrawMode(geometry,row.mode === 5 ? 1 : 2);assert.equal(oracle.getIndex()!.count / 3,row.triangles); }
  }
  console.log(`[DEBUG] glTF admission portable refusals=${admission.refusals.length} modes=${admission.modes.length} independentGltfTransformAndThreeCounts=true`);

  const authoredImport = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔁️authored-attributes/🔣️.json", import.meta.url), "utf8")), preparedImport = fixture.gltfExport.prepared;
  for (const row of authoredImport.cases) {
    const source = structuredClone(admittedSource);
    const extras = { semioAttributes: preparedImport.attributes, semioAttributeStreams: authoredImport.streams, semioSourceVertices: row.vertices, semioSourceCorners: row.corners, semioSourceFaces: row.faces };
    for (const semantic of Object.values(authoredImport.streams) as string[]) source.meshes[0].primitives[0].attributes[semantic] = 0;
    source.meshes[0].primitives[0].extras = extras;
    const primitive = parseGltfDocument(source).meshes[0].primitives[0];
    const polygon = { vertices: row.vertices.map((index: number) => preparedImport.positions.slice(index * 3,index * 3 + 3)), faces: row.indices, materials: { "mat-0": preparedImport.materials.paint }, textures: preparedImport.textures };
    const attributes = restoreGltfAuthoredAttributes(primitive,polygon,authoredImport.materialIds);
    assert.deepEqual(Object.keys(attributes).sort(),Object.keys(row.expected).sort(),row.id);
    for (const [name, expected] of Object.entries(row.expected)) {
      assert.equal(attributes[name].domain,preparedImport.attributes[name].domain,row.id + name);
      assert.equal(attributes[name].interpolation,preparedImport.attributes[name].interpolation,row.id + name);
      assert.deepEqual(attributes[name].indices,expected,row.id + name);
      assert.deepEqual(attributes[name].values,name === "material" ? ["mat-0","mat-0"] : preparedImport.attributes[name].values,row.id + name);
    }
    const independentMetadata = new Document().createPrimitive().setExtras(extras).getExtras();
    assert.deepEqual(independentMetadata.semioAttributes,preparedImport.attributes);
    const partial = structuredClone(source);
    partial.meshes[0].primitives[0].extras.semioAttributes = Object.fromEntries(authoredImport.fallback.authored.map((name: string) => [name,preparedImport.attributes[name]]));
    partial.meshes[0].primitives[0].extras.semioAttributeStreams = {};
    const normal: MeshAttribute = { domain: "vertex",semantic: "normal",interpolation: "linear",values: row.vertices.map(() => authoredImport.fallback.normal) };
    const preserved = restoreGltfAuthoredAttributes(parseGltfDocument(partial).meshes[0].primitives[0],{ ...polygon,attributes: { normal } },authoredImport.materialIds);
    assert.deepEqual(Object.keys(preserved).sort(),authoredImport.fallback.expected);
    assert.deepEqual(preserved.normal,normal);
    if (row.id === "material-split-triangle") for (const refusal of authoredImport.refusals) {
      const invalid = structuredClone(source);
      if (refusal.remove) delete invalid.meshes[0].primitives[0].extras[refusal.field]; else invalid.meshes[0].primitives[0].extras[refusal.field] = refusal.value;
      assert.throws(() => restoreGltfAuthoredAttributes(parseGltfDocument(invalid).meshes[0].primitives[0],polygon,authoredImport.materialIds),Error,refusal.id);
    }
  }
  console.log(`[DEBUG] glTF authored palette import cases=2 refusals=${authoredImport.refusals.length} fallbackNormals=true independentGltfTransformMetadata=true`);

  for (const row of fixture.gltfSurfaceAuthoring.cases) {
    const document = parseGltfDocument(row.document), primitive = document.meshes[0].primitives[0];
    const actual = gltfMaterialSurface(document, primitive);
    assert.deepEqual(actual, row.expected.material, row.id);
    const bytes = Buffer.from(row.document.buffers[0].uri.split(",")[1], "base64"), view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    for (const [semantic, name, width] of [["TEXCOORD_1", "uv1", 2], ["TANGENT", "tangent", 4], ["_TEMPERATURE", "_TEMPERATURE", 1]] as const) {
      if (row.document.meshes[0].primitives[0].attributes[semantic] === undefined) continue;
      const accessor = row.document.accessors[row.document.meshes[0].primitives[0].attributes[semantic]], source = row.document.bufferViews[accessor.bufferView];
      const values = Array.from({ length: accessor.count * width }, (_, index) => accessor.componentType === 5123 ? view.getUint16(source.byteOffset + index * 2, true) : view.getFloat32(source.byteOffset + index * 4, true));
      const attribute = accessor.componentType === 5123 ? new Uint16BufferAttribute(values, width, true) : new Float32BufferAttribute(values, width);
      const normalized = Array.from({ length: attribute.count }, (_, index) => width === 1 ? attribute.getX(index) : [attribute.getX(index), attribute.getY(index), ...(width > 2 ? [attribute.getZ(index), attribute.getW(index)] : [])]);
      assert.deepEqual(normalized, row.expected.attributes[name].values, `${row.id}: independent Three ${name}`);
    }
    const authored = row.document.materials[0], independent = new Document(), texture = independent.createTexture(), material = independent.createMaterial().setEmissiveFactor(authored.emissiveFactor).setAlphaMode(authored.alphaMode).setAlphaCutoff(authored.alphaCutoff).setDoubleSided(authored.doubleSided).setNormalScale(authored.normalTexture.scale).setOcclusionStrength(authored.occlusionTexture.strength);
    assert.deepEqual(actual.emissive, material.getEmissiveFactor());
    assert.equal(actual.alphaMode, material.getAlphaMode());
    assert.equal(actual.alphaCutoff, material.getAlphaCutoff());
    assert.equal(actual.doubleSided, material.getDoubleSided());
    assert.deepEqual(actual.normalScale, [material.getNormalScale(), row.document.meshes[0].primitives[0].attributes.TANGENT === undefined ? -material.getNormalScale() : material.getNormalScale()]);
    assert.equal(actual.occlusionStrength, material.getOcclusionStrength());
    for (const [role, setter, getter] of [["baseColorTexture", "setBaseColorTexture", "getBaseColorTextureInfo"], ["metallicRoughnessTexture", "setMetallicRoughnessTexture", "getMetallicRoughnessTextureInfo"], ["normalTexture", "setNormalTexture", "getNormalTextureInfo"], ["occlusionTexture", "setOcclusionTexture", "getOcclusionTextureInfo"], ["emissiveTexture", "setEmissiveTexture", "getEmissiveTextureInfo"]] as const) {
      material[setter](texture);
      const info = material[getter]()!, binding = authored.pbrMetallicRoughness[role] ?? authored[role], sampler = row.document.samplers[0];
      info.setTexCoord(binding.texCoord ?? 0).setMagFilter(sampler.magFilter).setMinFilter(sampler.minFilter).setWrapS(sampler.wrapS).setWrapT(sampler.wrapT);
      assert.equal(actual.textureCoordinates[role], info.getTexCoord());
      assert.deepEqual(actual.textureSamplers[role], { magFilter: info.getMagFilter(), minFilter: info.getMinFilter(), wrapS: info.getWrapS(), wrapT: info.getWrapT() });
    }
    for (const refusal of fixture.gltfSurfaceAuthoring.refusals) {
      const invalid = structuredClone(row.document);
      invalid.materials[0][refusal.materialField][refusal.field] = refusal.value;
      assert.throws(() => gltfMaterialSurface(parseGltfDocument(invalid), primitive), Error, refusal.id);
    }
  }
  for (const row of fixture.refusals) assert.throws(() => polygonMeshFromPrepared({ ...fixture.cases[0].prepared, [row.field]: row.value }), Error, row.id);
  for (const row of fixture.diagnostics) {
    const actual = meshFormatDiagnostics([fixture.cases[row.case].prepared], row.format);
    assert.deepEqual(actual.map(diagnostic => diagnostic.code), row.codes);
    for (const diagnostic of actual) {
      assert.ok(diagnostic.labels.en.length > 0);
      assert.ok(diagnostic.labels.de.length > 0);
    }
  }
  const validateExport = new Ajv({ strict: false }).compile(schema.$defs.Generation3dExportRequest);
  for (const row of fixture.materialDefaults) {
    assert.deepEqual(materialFieldsForExport(row.source), row.expected, row.id);
    const oracle = new Document().createMaterial();
    if (row.source.baseColor) oracle.setBaseColorFactor(row.source.baseColor);
    if (row.source.metallic !== undefined) oracle.setMetallicFactor(row.source.metallic);
    if (row.source.roughness !== undefined) oracle.setRoughnessFactor(row.source.roughness);
    assert.deepEqual(materialFieldsForExport(row.source), { baseColor: oracle.getBaseColorFactor(), metallic: oracle.getMetallicFactor(), roughness: oracle.getRoughnessFactor() });
  }
  for (const row of fixture.diagnosticDetails) {
    const meshes = row.parts.map((part: number) => {
      const mesh = structuredClone(fixture.cases[part].prepared);
      if (row.positions) mesh.positions = row.positions;
      if (row.colors) mesh.colors = row.colors;
      if (row.extraAttributes) mesh.attributes = { ...mesh.attributes, ...row.extraAttributes };
      if (row.extraMaterial) Object.assign(mesh.materials.red, row.extraMaterial);
      return mesh;
    });
    assert.deepEqual(meshFormatDiagnostics(meshes, row.format).map(diagnostic => diagnostic.code), row.codes, row.id);
  }
  for (const row of fixture.exportScope.cases) {
    assert.ok(validateExport(row.request), JSON.stringify(validateExport.errors));
    assert.deepEqual(exportSourceChannels(fixture.exportScope.host, row.request.widgetId), row.channels);
  }
  for (const widgetId of fixture.exportScope.refusals) assert.throws(() => exportSourceChannels(fixture.exportScope.host, widgetId), Error);
  assert.equal(validateExport({ format: "gltf", widgetId: "" }), false);
  for (const row of fixture.mergeSurface.cases) {
    const meshes = row.parts.map((index: number) => fixture.cases[index].prepared);
    const merged = mergePreparedMeshes(meshes);
    const polygon = polygonMeshFromPrepared(merged);
    assert.equal(polygon.vertices.length, row.vertices);
    assert.equal(polygon.faces.length, row.faces);
    assert.deepEqual(Object.keys(polygon.materials ?? {}), row.materials);
    assert.deepEqual(Object.keys(polygon.textures ?? {}), row.textures);
    assert.deepEqual(polygon.attributes?.material.values, row.materials);
    assert.deepEqual(polygon.attributes?.material.indices, [0, 1]);
    for (const id of row.materials) assert.ok(Object.hasOwn(polygon.textures ?? {}, polygon.materials?.[id].baseColorTexture as string));
    const geometry = new BufferGeometry();
    geometry.setAttribute("position", new Float32BufferAttribute(merged.positions, 3));
    geometry.setIndex(merged.indices);
    assert.equal(geometry.getAttribute("position").count, row.vertices);
    assert.deepEqual(Array.from(geometry.getIndex()?.array ?? []), [0, 1, 2, 3, 4, 5]);
    geometry.dispose();
  }
  for (const parts of fixture.mergeSurface.refusals) assert.throws(() => mergePreparedMeshes(parts.map((index: number) => fixture.cases[index].prepared)), Error, "incompatible attribute domains require separate surfaces");
  console.log(`[DEBUG] generation3d surface bridge cases=${fixture.cases.length} refusals=${fixture.refusals.length} independentThree=true`);
  return fixture.cases.length + fixture.refusals.length;
}

/** 🧩️ Concave faces stay editable, with area checked by the independent polygon triangulator. */
/** 🎬️ Imported placements remain editable graph inputs; Three.js supplies the affine oracle. */
export function testGeneration3dGltfSceneImport(): void {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎬️scene-import/🔣️.json", import.meta.url), "utf8"));
  assert.equal(fixture.schema, "generation3d.gltf-scene-graph.v1");
  const hostFor = (count: number): FlowHostSnapshot => ({ schema: "flow.host_snapshot", camera: { x: binary64(0), y: binary64(0), zoom: binary64(1) }, layout: {}, widgets: Array.from({ length: count }, (_, index) => ({ kind: "neuron", id: `imported-geometry${index ? `-${index}` : ""}`, neuronKind: "brep.mesh.construct", params: {}, inputPorts: [], outputPorts: [], preview: true })), synapses: [] });
  for (const row of fixture.cases) {
    const document = parseGltfDocument(row.document), host = hostFor(document.meshes.reduce((sum, mesh) => sum + mesh.primitives.length, 0));
    const result = applyGltfSceneToHost(host, document);
    if (!document.nodes.length) { assert.deepEqual(result, host); continue; }
    const previews = result.widgets.filter(widget => widget.kind === "outputPreview");
    assert.equal(previews.length, row.paths.length);
    assert.ok(result.widgets.every(widget => widget.kind !== "neuron" || widget.neuronKind !== "brep.mesh.construct" || !widget.preview));
    for (const [instance, preview] of previews.entries()) {
      let source = result.synapses.find(wire => wire.to === preview.id)!.from;
      const path = row.paths[instance] as number[], actualMatrices: number[][] = [];
      for (const nodeIndex of [...path].reverse()) {
        const widget = result.widgets.find(widget => widget.id === source)!;
        assert.ok(widget.kind === "neuron");
        assert.equal(widget.neuronKind, "brep.mesh.transform");
        const matrix = widget.params.matrix;
        assert.ok(matrix.kind === "dictionary");
        const values = Array.from({ length: 16 }, (_, index) => {
          const scalar = matrix.value[String(index)];
          assert.equal(scalar.kind, "dictionary");
          const value = (scalar as { value: NeuralDictionary }).value.value;
          assert.ok(value.kind === "decimal");
          return binary64Value(value.value);
        });
        const node = row.document.nodes[nodeIndex], oracle = node.matrix ? new Matrix4().fromArray(node.matrix) : new Matrix4().compose(new Vector3(...(node.translation ?? [0, 0, 0]) as [number, number, number]), new Quaternion(...(node.rotation ?? [0, 0, 0, 1]) as [number, number, number, number]), new Vector3(...(node.scale ?? [1, 1, 1]) as [number, number, number]));
        values.forEach((value, index) => assert.ok(Math.abs(value - oracle.elements[index]) < 1e-12, `node ${nodeIndex}, coefficient ${index}`));
        actualMatrices.unshift(values);
        source = result.synapses.find(wire => wire.to === source && wire.toPort === "mesh")!.from;
      }
      assert.equal(source, `imported-geometry${row.sources[instance] ? `-${row.sources[instance]}` : ""}`);
      const world = actualMatrices.reduce((matrix, local) => new Matrix4().fromArray(local).multiply(matrix), new Matrix4());
      const expected = path.reduce((matrix: Matrix4, nodeIndex: number) => {
        const node = row.document.nodes[nodeIndex];
        const local = node.matrix ? new Matrix4().fromArray(node.matrix) : new Matrix4().compose(new Vector3(...(node.translation ?? [0, 0, 0]) as [number, number, number]), new Quaternion(...(node.rotation ?? [0, 0, 0, 1]) as [number, number, number, number]), new Vector3(...(node.scale ?? [1, 1, 1]) as [number, number, number]));
        return local.multiply(matrix);
      }, new Matrix4());
      assert.ok(new Vector3(1, 2, 3).applyMatrix4(world).distanceTo(new Vector3(1, 2, 3).applyMatrix4(expected)) < 1e-10);
    }
    assert.deepEqual(host, hostFor(document.meshes.reduce((sum, mesh) => sum + mesh.primitives.length, 0)), "the import owns a fresh graph result");
  }
  for (const row of fixture.refusals) {
    const { id, ...source } = row;
    const document = parseGltfDocument({ asset: { version: "2.0" }, meshes: [{ primitives: [{ attributes: {} }] }], ...source });
    assert.throws(() => applyGltfSceneToHost(hostFor(1), document), Error, row.id);
  }
  process.stdout.write(`[DEBUG] generation3d glTF scene cases=${fixture.cases.length} refusals=${fixture.refusals.length} independentThree=true\n`);
}

export function testGeneration3dPolygonImport(): number {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🧩️polygon-import/🔣️.json", import.meta.url), "utf8"));
  const view = new DataView(new ArrayBuffer(8));
  const binary64 = (value: number) => { view.setFloat64(0, value); return { bits: view.getBigUint64(0) }; };
  for (const row of fixture.cases) {
    const source = structuredClone(row.source);
    let output;
    if (row.format === "obj") {
      for (const point of source.vertices) for (const field of ["x", "y", "z"]) point[field] = binary64(point[field]);
      for (const point of source.normals) for (const field of ["x", "y", "z"]) point[field] = binary64(point[field]);
      for (const point of source.texcoords) for (const field of ["u", "v"]) point[field] = binary64(point[field]);
      for (const group of [...source.groups, ...source.objects]) group.faces = group.faces.map(BigInt);
      for (const range of [...source.usemtl, ...source.smoothingGroups]) range.faceIndexFrom = BigInt(range.faceIndexFrom);
      output = polygonMeshFromObj(source);
      const incomplete = structuredClone(source);
      delete incomplete.faces[0].vertices[0].texcoord;
      assert.throws(() => polygonMeshFromObj(incomplete), Error, "partially authored UVs need an explicit repair");
    } else {
      for (const element of source.elements) {
        element.count = BigInt(element.count);
        for (const row of element.rows) for (const cell of row.values) if (cell.kind === "double") cell.value = binary64(cell.value);
      }
      output = polygonMeshFromPly(source);
      const incomplete = structuredClone(source);
      incomplete.elements[0].count = 9n;
      assert.throws(() => polygonMeshFromPly(incomplete), Error, "element occurrence count is validated");
    }
    assert.deepEqual(output, row.polygon, row.id);
    assert.equal(output.faces.length, 1, "import preserves the original editable face");
    const contour = output.faces[0].map(index => new Vector2(output.vertices[index][0], output.vertices[index][1]));
    const triangles = ShapeUtils.triangulateShape(contour, []);
    const area = triangles.reduce((sum, triangle) => sum + Math.abs(ShapeUtils.area(triangle.map(index => contour[index]))), 0);
    assert.equal(area, row.area, `${row.id}: independent concave area`);
  }
  console.log(`[DEBUG] generation3d polygon import cases=${fixture.cases.length} refusals=2 independentThreeEarcut=true`);
  return fixture.cases.length + 2;
}

/** 🔬️ Reads the actual native byte codec output with the independent glTF Transform implementation. */
export async function verifyPreparedGltfExportOracle(path: string): Promise<void> {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎨️surface/🔣️.json", import.meta.url), "utf8")).gltfExport;
  const document = await new NodeIO().read(path), primitive = document.getRoot().listMeshes()[0].listPrimitives()[0];
  for (const [semantic, expected] of Object.entries(fixture.expanded)) assert.deepEqual(Array.from(primitive.getAttribute(semantic)!.getArray()!), expected, `native byte oracle ${semantic}`);
  assert.deepEqual(primitive.getExtras().semioAttributes, fixture.prepared.attributes);
  assert.deepEqual(primitive.getExtras().semioAttributeStreams, fixture.attributeStreams);
  assert.deepEqual(primitive.getExtras().semioSourceCorners, [0,1,2,3,4,5]);
  assert.deepEqual(primitive.getExtras().semioSourceVertices, fixture.prepared.indices);
  assert.deepEqual(primitive.getExtras().semioSourceFaces, [0,1]);
  assert.deepEqual(document.getRoot().listMeshes()[0].getExtras().semioComponentReferences, fixture.componentReferences);
  for (const [extra, field] of [["semioFaceIds", "faceIds"], ["semioEdgeIds", "edgeIds"], ["semioVertexIds", "vertexIds"], ["semioEdgePositions", "edgePositions"]]) assert.deepEqual(document.getRoot().listMeshes()[0].getExtras()[extra], fixture.prepared[field]);
  const material = primitive.getMaterial()!;
  assert.equal(material.getExtras().semioMaterialId, fixture.materialIdentity.id);
  assert.equal(material.getExtras().semioMaterialPart, fixture.materialIdentity.part);
  assert.deepEqual(material.getBaseColorFactor(), fixture.prepared.materials.paint.baseColor);
  assert.equal(material.getMetallicFactor(), 0.25); assert.equal(material.getRoughnessFactor(), 0.75);
  assert.deepEqual(material.getEmissiveFactor(), [0.125, 0.25, 0.5]); assert.equal(material.getAlphaMode(), "MASK");
  assert.equal(material.getAlphaCutoff(), 0.25); assert.equal(material.getDoubleSided(), true);
  assert.equal(material.getNormalScale(), 0.5); assert.equal(material.getOcclusionStrength(), 0.25);
  const roles = ["baseColorTexture", "metallicRoughnessTexture", "normalTexture", "occlusionTexture", "emissiveTexture"];
  const infos = [material.getBaseColorTextureInfo(), material.getMetallicRoughnessTextureInfo(), material.getNormalTextureInfo(), material.getOcclusionTextureInfo(), material.getEmissiveTextureInfo()];
  for (let index = 0; index < roles.length; index++) {
    const role = roles[index], info = infos[index]!, sampler = fixture.prepared.materials.paint.textureSamplers[role];
    assert.equal(info.getTexCoord(), fixture.prepared.materials.paint.textureCoordinates[role]);
    assert.equal(info.getWrapS(), sampler.wrapS); assert.equal(info.getWrapT(), sampler.wrapT);
    assert.equal(info.getMinFilter(), sampler.minFilter); assert.equal(info.getMagFilter(), sampler.magFilter);
  }
  assert.equal(document.getRoot().listTextures().length, 1);
  assert.deepEqual(Array.from(document.getRoot().listTextures()[0].getImage()!), fixture.prepared.textures.image.bytes);
  const edgeDocument = await new NodeIO().read(join(dirname(path), "prepared-rich-edge.gltf")), edgePrimitive = edgeDocument.getRoot().listMeshes()[0].listPrimitives()[0];
  const edgeAttributes = edgePrimitive.getExtras().semioAttributes as Record<string, MeshAttribute>;
  assert.deepEqual(edgeAttributes.edgeUv, fixture.edgeUvAttribute);
  assert.equal((edgePrimitive.getExtras().semioAttributeStreams as Record<string, string>).edgeUv, undefined);
  for (const semantic of ["TEXCOORD_0", "TEXCOORD_63"]) assert.deepEqual(Array.from(edgePrimitive.getAttribute(semantic)!.getArray()!), fixture.expanded[semantic]);
  console.log("[DEBUG] independent glTF Transform native byte oracle: richChannels=8 images=1 roleSamplers=5 materials=true edgeUvPalette=true");
}
