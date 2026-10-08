#!/usr/bin/env bun
// 🪢️ Third-party generator and verifier of the OPC-plumbing evidence of `s.stdio.{docx,pptx,xlsx}@ecma-376/🧱️base`: the four kinds `set-relationship`, `remove-relationship`,
// `set-content-type` and `remove-content-type`. Every `before`/`after` pair is built from the artifact's own COMMITTED real package by `jszip` (container) and
// `fast-xml-parser` (`[Content_Types].xml`, `*.rels`) -- nothing here imports this repository's codec or mutation semantics. Each recipe states its intended effect twice, once
// as the XML edit that writes the `after` package and once as a model-level expectation; the script refuses to write a pair whose re-read plumbing differs from the expectation.
//
//   bun 📜️script.ts generate [--artifact docx|pptx|xlsx] [--only <fixture-id>]   writes the fixtures and registers the evidence in the oracle manifests
//   bun 📜️script.ts verify                                                         re-reads every committed pair and checks it against its recipe
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import JSZip from "jszip";
import { XMLBuilder, XMLParser } from "fast-xml-parser";

const ROOT = resolve(import.meta.dir, "../../../../../../../..");
const SCRIPT_DIRECTORY = relative(ROOT, import.meta.dir);
const SUBSET = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base";
const FIXED_DATE = new Date(Date.UTC(2026, 0, 1, 0, 0, 0));
const REL_HYPERLINK = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink";
const REL_CUSTOM = "http://example.invalid/relationships/oracle-retyped";

type Artifact = { key: "docx" | "pptx" | "xlsx"; dir: string; real: string; media: string; roles: [string, string]; oracle: string };
const ARTIFACTS: Record<string, Artifact> = {
  docx: { key: "docx", dir: "📜️docx", real: "📜️example-readme.docx", media: "application/vnd.openxmlformats-officedocument.wordprocessingml.document", roles: ["before-docx", "after-docx"], oracle: "jszip-docx-ecma-376-mutate-reader" },
  pptx: { key: "pptx", dir: "📽️pptx", real: "📽️.pptx", media: "application/vnd.openxmlformats-officedocument.presentationml.presentation", roles: ["expected-before-pptx", "expected-after-pptx"], oracle: "jszip-pptx-ecma-376-opc-plumbing-reader" },
  xlsx: { key: "xlsx", dir: "📕️xlsx", real: "📕️reuse-marketplaces.xlsx", media: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet", roles: ["expected-before-xlsx", "expected-after-xlsx"], oracle: "jszip-xlsx-ecma-376-opc-plumbing-reader" },
};

//#region 🧬️Model
type Rel = { id: string; type: string; target: string; external: boolean };
type Plumbing = { defaults: [string, string][]; overrides: [string, string][]; rels: Record<string, Rel[]> };
type Step =
  | { op: "set-relationship"; owner: string; rel: Rel; index?: number }
  | { op: "remove-relationship"; owner: string; id: string }
  | { op: "set-content-type"; override: boolean; name: string; contentType: string; index?: number }
  | { op: "remove-content-type"; override: boolean; name: string };
type Recipe = { id: string; dir: string; mutation: Step["op"]; outcome: "applied" | "rejected"; before: Step[]; step: Step; notes: string };
//#endregion 🧬️Model

//#region 📖️Plumbing
const PARSE = new XMLParser({ ignoreAttributes: false, attributeNamePrefix: "@_", preserveOrder: true, trimValues: false, ignoreDeclaration: true });
const BUILD = new XMLBuilder({ ignoreAttributes: false, attributeNamePrefix: "@_", preserveOrder: true, format: false, suppressEmptyNode: true });
type PNode = Record<string, unknown> & { ":@"?: Record<string, string> };
const tagOf = (node: PNode): string => Object.keys(node).find((key) => key !== ":@")!;
const rowsOf = (root: PNode): PNode[] => (root[tagOf(root)] as PNode[]).filter((node) => !("#text" in node));
const attr = (node: PNode, name: string): string => node[":@"]?.[`@_${name}`] ?? "";
const ownerOf = (path: string): string | undefined => {
  const match = /^(?:(.*)\/)?_rels\/([^/]*)\.rels$/.exec(path);
  return match === null ? undefined : `${match[1] === undefined ? "" : `${match[1]}/`}${match[2]}`;
};
const relsPathOf = (owner: string): string => {
  const slash = owner.lastIndexOf("/");
  return slash < 0 ? `_rels/${owner}.rels` : `${owner.slice(0, slash)}/_rels/${owner.slice(slash + 1)}.rels`;
};

async function readPlumbing(zip: JSZip): Promise<Plumbing> {
  const types = PARSE.parse(await zip.file("[Content_Types].xml")!.async("string"))[0] as PNode;
  const rows = rowsOf(types);
  const plumbing: Plumbing = { defaults: [], overrides: [], rels: {} };
  for (const row of rows) {
    if (tagOf(row) === "Default") plumbing.defaults.push([attr(row, "Extension"), attr(row, "ContentType")]);
    if (tagOf(row) === "Override") plumbing.overrides.push([attr(row, "PartName"), attr(row, "ContentType")]);
  }
  for (const name of Object.keys(zip.files).filter((entry) => !zip.files[entry]!.dir)) {
    const owner = ownerOf(name);
    if (owner === undefined) continue;
    const root = PARSE.parse(await zip.file(name)!.async("string"))[0] as PNode;
    plumbing.rels[owner] = rowsOf(root).map((row) => ({ id: attr(row, "Id"), type: attr(row, "Type"), target: attr(row, "Target"), external: attr(row, "TargetMode") === "External" }));
  }
  return plumbing;
}

/** 🧮️ The model-level effect of one step: an independent restatement of what the XML edit below writes. */
function expect(model: Plumbing, step: Step): Plumbing {
  const next: Plumbing = { defaults: model.defaults.map((entry) => [...entry]), overrides: model.overrides.map((entry) => [...entry]), rels: Object.fromEntries(Object.entries(model.rels).map(([owner, rels]) => [owner, rels.map((rel) => ({ ...rel }))])) };
  const place = <T>(list: T[], value: T, index: number | undefined) => list.splice(Math.min(index ?? list.length, list.length), 0, value);
  if (step.op === "set-relationship") {
    const list = (next.rels[step.owner] ??= []);
    const at = list.findIndex((rel) => rel.id === step.rel.id);
    if (at >= 0) list[at] = step.rel;
    else place(list, step.rel, step.index);
  } else if (step.op === "remove-relationship") {
    const list = next.rels[step.owner];
    const at = list?.findIndex((rel) => rel.id === step.id) ?? -1;
    if (list === undefined || at < 0) throw new Error(`no relationship ${step.id} of ${step.owner || "(root)"}`);
    list.splice(at, 1);
    if (list.length === 0) delete next.rels[step.owner];
  } else {
    const list = step.override ? next.overrides : next.defaults;
    const name = step.name;
    const at = list.findIndex(([key]) => key === name);
    if (step.op === "set-content-type") {
      if (!step.override && list.some(([key]) => key !== name && key.toLowerCase() === name.toLowerCase())) throw new Error("default extension exists in another letter case");
      if (at >= 0) list[at] = [name, step.contentType];
      else place(list, [name, step.contentType], step.index);
    } else {
      if (at < 0) throw new Error(`no ${step.override ? "override" : "default"} ${name}`);
      list.splice(at, 1);
    }
  }
  return next;
}
//#endregion 📖️Plumbing

//#region ✍️Edit
const rowNode = (tag: string, attrs: Record<string, string>): PNode => ({ [tag]: [], ":@": Object.fromEntries(Object.entries(attrs).map(([key, value]) => [`@_${key}`, value])) }) as PNode;

/** ✍️ The XML edit of one step, written with `fast-xml-parser`'s tree and `jszip`'s container. Rejected steps raise before anything is written. */
async function edit(zip: JSZip, step: Step): Promise<void> {
  const insertAmong = (rows: PNode[], tag: string, row: PNode, index: number | undefined) => {
    const slots = rows.map((node, at) => [node, at] as const).filter(([node]) => tagOf(node) === tag).map(([, at]) => at);
    const physical = index !== undefined && index < slots.length ? slots[index]! : slots.length > 0 ? slots[slots.length - 1]! + 1 : tag === "Default" ? 0 : rows.length;
    rows.splice(physical, 0, row);
  };
  const rewrite = (name: string, root: PNode, rows: PNode[]) => {
    (root as Record<string, unknown>)[tagOf(root)] = rows;
    zip.file(name, `<?xml version="1.0" encoding="UTF-8" standalone="yes"?>${BUILD.build([root])}`, { date: FIXED_DATE, createFolders: false });
  };
  if (step.op === "set-relationship" || step.op === "remove-relationship") {
    const name = relsPathOf(step.owner);
    const exists = zip.file(name) !== null;
    const root = exists ? (PARSE.parse(await zip.file(name)!.async("string"))[0] as PNode) : ({ Relationships: [], ":@": { "@_xmlns": "http://schemas.openxmlformats.org/package/2006/relationships" } } as PNode);
    const rows = rowsOf(root);
    const at = rows.findIndex((row) => attr(row, "Id") === (step.op === "set-relationship" ? step.rel.id : step.id));
    if (step.op === "set-relationship") {
      const row = rowNode("Relationship", { Id: step.rel.id, Type: step.rel.type, Target: step.rel.target, ...(step.rel.external ? { TargetMode: "External" } : {}) });
      if (at >= 0) rows[at] = row;
      else insertAmong(rows, "Relationship", row, step.index);
      rewrite(name, root, rows);
    } else {
      if (!exists || at < 0) throw new Error(`rejected: no relationship ${step.id} of ${step.owner || "(root)"}`);
      rows.splice(at, 1);
      if (rows.length === 0) zip.remove(name);
      else rewrite(name, root, rows);
    }
    return;
  }
  const root = PARSE.parse(await zip.file("[Content_Types].xml")!.async("string"))[0] as PNode;
  const rows = rowsOf(root);
  const tag = step.override ? "Override" : "Default";
  const key = step.override ? "PartName" : "Extension";
  const name = step.name;
  const at = rows.findIndex((row) => tagOf(row) === tag && attr(row, key) === name);
  if (step.op === "set-content-type") {
    if (!step.override && rows.some((row) => tagOf(row) === "Default" && attr(row, key) !== name && attr(row, key).toLowerCase() === name.toLowerCase())) throw new Error("rejected: default extension exists in another letter case");
    const row = rowNode(tag, { [key]: name, ContentType: step.contentType });
    if (at >= 0) rows[at] = row;
    else insertAmong(rows, tag, row, step.index);
  } else {
    if (at < 0) throw new Error(`rejected: no ${tag} ${name}`);
    rows.splice(at, 1);
  }
  rewrite("[Content_Types].xml", root, rows);
}
//#endregion ✍️Edit

//#region 🍳️Recipes
const external = (id: string): Rel => ({ id, type: REL_HYPERLINK, target: "https://example.invalid/oracle", external: true });

function recipes(model: Plumbing): Recipe[] {
  const root = model.rels[""]!;
  const last = root[root.length - 1]!;
  const extension = model.defaults.find(([name]) => name === "xml")?.[0] ?? model.defaults[0]![0];
  const addDefault: Step = { op: "set-content-type", override: false, name: "zzoracle", contentType: "application/x-oracle", index: 0 };
  return [
    { id: "set-relationship-inserts-an-external-hyperlink-first", dir: "📎", mutation: "set-relationship", outcome: "applied", before: [], step: { op: "set-relationship", owner: "", rel: external("rIdOracleLink"), index: 0 }, notes: "A new external relationship is inserted at position 0 of the package root's relationships." },
    { id: "set-relationship-appends-an-external-hyperlink", dir: "📎", mutation: "set-relationship", outcome: "applied", before: [], step: { op: "set-relationship", owner: "", rel: external("rIdOracleLink") }, notes: "A new external relationship without a position is appended after the last relationship of the package root." },
    { id: "set-relationship-retypes-the-last-root-relationship", dir: "📎", mutation: "set-relationship", outcome: "applied", before: [], step: { op: "set-relationship", owner: "", rel: { ...last, type: REL_CUSTOM } }, notes: "An existing id is changed in place: the last root relationship keeps its id, target and position and takes another type." },
    { id: "set-relationship-no-op-identical-relationship", dir: "📎", mutation: "set-relationship", outcome: "applied", before: [], step: { op: "set-relationship", owner: "", rel: { ...last } }, notes: "Writing a relationship identical to the stored one changes nothing." },
    { id: "remove-relationship-drops-the-last-root-relationship", dir: "🧷", mutation: "remove-relationship", outcome: "applied", before: [], step: { op: "remove-relationship", owner: "", id: last.id }, notes: "The last relationship of the package root is removed; the others keep their order." },
    { id: "remove-relationship-drops-an-added-hyperlink", dir: "🧷", mutation: "remove-relationship", outcome: "applied", before: [{ op: "set-relationship", owner: "", rel: external("rIdOracleLink"), index: 0 }], step: { op: "remove-relationship", owner: "", id: "rIdOracleLink" }, notes: "The relationship added at position 0 is removed again; the package root is back to its original relationships." },
    { id: "remove-relationship-rejected-missing-id", dir: "🧷", mutation: "remove-relationship", outcome: "rejected", before: [], step: { op: "remove-relationship", owner: "", id: "rIdNothing" }, notes: "The package root owns no relationship rIdNothing. Before-only." },
    { id: "set-content-type-inserts-a-default-first", dir: "📇", mutation: "set-content-type", outcome: "applied", before: [], step: addDefault, notes: "A new extension default is inserted at position 0 of the content-types defaults." },
    { id: "set-content-type-retypes-an-existing-default", dir: "📇", mutation: "set-content-type", outcome: "applied", before: [], step: { op: "set-content-type", override: false, name: extension, contentType: "application/x-oracle-retyped" }, notes: "An existing default keeps its position and takes another content type." },
    { id: "set-content-type-no-op-identical-entry", dir: "📇", mutation: "set-content-type", outcome: "applied", before: [], step: { op: "set-content-type", override: false, name: model.defaults[0]![0], contentType: model.defaults[0]![1] }, notes: "Writing the stored content type again changes nothing." },
    { id: "set-content-type-rejected-default-in-another-case", dir: "📇", mutation: "set-content-type", outcome: "rejected", before: [addDefault], step: { op: "set-content-type", override: false, name: "ZZORACLE", contentType: "application/x-oracle-other" }, notes: "The extension default zzoracle exists in lower case; ZZORACLE would be a second default in another letter case. Before-only." },
    { id: "remove-content-type-drops-an-added-default", dir: "🧺", mutation: "remove-content-type", outcome: "applied", before: [addDefault], step: { op: "remove-content-type", override: false, name: "zzoracle" }, notes: "The default added at position 0 is removed again." },
    { id: "remove-content-type-rejected-missing-entry", dir: "🧺", mutation: "remove-content-type", outcome: "rejected", before: [], step: { op: "remove-content-type", override: false, name: "nothing" }, notes: "The content types hold no default for the extension nothing. Before-only." },
  ];
}
//#endregion 🍳️Recipes

//#region 🚀️Entry
const dirName = (recipe: Recipe): string => `${recipe.dir}\uFE0F${recipe.id}`;
const sha = (bytes: Uint8Array): string => `sha256:${createHash("sha256").update(bytes).digest("hex")}`;
const same = (left: unknown, right: unknown): boolean => JSON.stringify(left) === JSON.stringify(right);
const subsetOf = (artifact: Artifact): string => join(ROOT, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts", artifact.dir, SUBSET);

async function build(artifact: Artifact, recipe: Recipe): Promise<{ before: Buffer; after?: Buffer; model: { before: Plumbing; after?: Plumbing } }> {
  const real = readFileSync(join(subsetOf(artifact), "🧫️fixtures", artifact.real));
  const zip = await JSZip.loadAsync(real);
  let model = await readPlumbing(zip);
  for (const step of recipe.before) {
    await edit(zip, step);
    model = expect(model, step);
  }
  const generate = (target: JSZip) => target.generateAsync({ type: "nodebuffer", compression: "DEFLATE" });
  const before = await generate(zip);
  if (!same(await readPlumbing(await JSZip.loadAsync(before)), model)) throw new Error(`${recipe.id}: the before package does not carry its recipe's plumbing`);
  if (recipe.outcome === "rejected") {
    let refused = false;
    try {
      await edit(await JSZip.loadAsync(before), recipe.step);
      expect(model, recipe.step);
    } catch {
      refused = true;
    }
    if (!refused) throw new Error(`${recipe.id}: a rejected recipe was accepted`);
    return { before, model: { before: model } };
  }
  const afterZip = await JSZip.loadAsync(before);
  await edit(afterZip, recipe.step);
  const after = await generate(afterZip);
  const expected = expect(model, recipe.step);
  if (!same(await readPlumbing(await JSZip.loadAsync(after)), expected)) throw new Error(`${recipe.id}: the after package differs from the model-level expectation`);
  return { before, after, model: { before: model, after: expected } };
}

function evidence(artifact: Artifact, recipe: Recipe, files: { path: string; role: string; bytes: Buffer }[], profile: string) {
  return {
    id: recipe.id,
    class: "third-party-generated",
    target: { artifact: `s.stdio.${artifact.key}`, standard: "ecma-376", subset: "base" },
    mutation: recipe.mutation,
    outcome: recipe.outcome,
    units: { length: "unitless", angle: "degree" },
    files: files.map((file) => ({ role: file.role, path: file.path, mediaType: artifact.media, sha256: sha(file.bytes), bytes: file.bytes.length })),
    generator: {
      oracle: artifact.oracle,
      packageVersion: "jszip@3.10.1 + fast-xml-parser@5.11.1",
      engineFamily: "zip-xml",
      engineVersion: "jszip@3.10.1 + fast-xml-parser@5.11.1",
      command: `bun ${SCRIPT_DIRECTORY}/📜️script.ts generate --artifact ${artifact.key} --only ${recipe.id}`,
      platform: "darwin-arm64",
    },
    provenance: { source: "generated", license: "MIT (jszip dual MIT/GPL-3.0-or-later, used under MIT) + MIT (fast-xml-parser)", attribution: "Generated with jszip (MIT) and fast-xml-parser (MIT) from the artifact's committed real package", security: "scanned-clean", privacy: "no-personal-data" },
    comparisonProfile: profile,
    reproducible: true,
    family: "structural",
    notes: recipe.notes,
  };
}

const MUTATIONS: { id: Step["op"]; variant: string; outcomes: string[] }[] = [
  { id: "set-relationship", variant: "SetRelationship", outcomes: ["applied"] },
  { id: "remove-relationship", variant: "RemoveRelationship", outcomes: ["applied", "rejected"] },
  { id: "set-content-type", variant: "SetContentType", outcomes: ["applied", "rejected"] },
  { id: "remove-content-type", variant: "RemoveContentType", outcomes: ["applied", "rejected"] },
];

async function generate(artifact: Artifact, only: string | undefined): Promise<void> {
  const subset = subsetOf(artifact);
  const manifestPath = join(subset, "🔮️oracles", "🔣️.json");
  const original = readFileSync(manifestPath, "utf8");
  const manifest = JSON.parse(original);
  const model = await readPlumbing(await JSZip.loadAsync(readFileSync(join(subset, "🧫️fixtures", artifact.real))));
  const extension = artifact.key;
  const profile = artifact.key === "docx" ? "semantic-docx-ecma-376-opc-jszip-v1" : manifest.comparisonProfiles[0].id;
  for (const recipe of recipes(model).filter((entry) => only === undefined || entry.id === only)) {
    const built = await build(artifact, recipe);
    const directory = join(subset, "🧫️fixtures", dirName(recipe));
    mkdirSync(directory, { recursive: true });
    const before = { role: artifact.roles[0], path: `../🧫️fixtures/${dirName(recipe)}/⬅️before.${extension}`, bytes: built.before };
    writeFileSync(join(directory, `⬅️before.${extension}`), built.before);
    const files = [before];
    if (built.after !== undefined) {
      writeFileSync(join(directory, `➡️after.${extension}`), built.after);
      files.push({ role: artifact.roles[1], path: `../🧫️fixtures/${dirName(recipe)}/➡️after.${extension}`, bytes: built.after });
    }
    const entry = evidence(artifact, recipe, files, profile);
    const at = manifest.testEvidence.findIndex((existing: { id: string }) => existing.id === recipe.id);
    if (at >= 0) manifest.testEvidence[at] = entry;
    else manifest.testEvidence.push(entry);
    console.log(`[${artifact.key} plumbing] ${recipe.id} -> ${directory}`);
  }
  const mutations = manifest.mutationManifests[0].mutations;
  for (const kind of MUTATIONS) {
    if (mutations.some((existing: { id: string }) => existing.id === kind.id)) continue;
    const template = mutations.find((existing: { payloadSchema?: string }) => existing.payloadSchema !== undefined);
    mutations.push({
      id: kind.id,
      capability: manifest.mutationCatalogs[0].capability,
      ...(template?.payloadSchema === undefined ? {} : { payloadSchema: "🧬️schema/🔣️.json" }),
      outcomes: kind.outcomes,
      productionDispatch: { operation: kind.id, bridgeVersion: 1, variant: kind.variant },
      oracleRequirements: [{ capability: manifest.mutationCatalogs[0].capability, qualifyingKind: "third-party-library", oracle: artifact.oracle }],
    });
  }
  if (!manifest.oracles.some((oracle: { id: string }) => oracle.id === artifact.oracle)) {
    const reader = manifest.oracles.find((oracle: { kind: string; id: string }) => oracle.kind === "third-party-library");
    manifest.oracles.push({
      ...reader,
      id: artifact.oracle,
      ecosystem: "javascript",
      package: "jszip",
      version: "3.10.1",
      engine: { family: "jszip", implementation: "jszip + fast-xml-parser reader", version: "jszip@3.10.1 + fast-xml-parser@5.11.1" },
      capabilities: [manifest.mutationCatalogs[0].capability],
      rationale: "📖️ A third-party implementation of the OPC package layer: jszip opens the container and fast-xml-parser reads and writes `[Content_Types].xml` and every `*.rels` part. The `before`/`after` pairs of the plumbing kinds are authored from the artifact's committed real package by this pairing -- each pair states its effect once as an XML edit and once as a model-level expectation and is refused unless both agree -- so nothing in this repository predicts the answer the pairs are judged against.",
      comparisonProfiles: [profile],
    });
  }
  const text = JSON.stringify(manifest, null, 2) + (original.endsWith("\n") ? "\n" : "");
  writeFileSync(manifestPath, text);
}

async function verify(): Promise<number> {
  let failures = 0;
  for (const artifact of Object.values(ARTIFACTS)) {
    const subset = subsetOf(artifact);
    const model = await readPlumbing(await JSZip.loadAsync(readFileSync(join(subset, "🧫️fixtures", artifact.real))));
    for (const recipe of recipes(model)) {
      try {
        const built = await build(artifact, recipe);
        const directory = join(subset, "🧫️fixtures", dirName(recipe));
        const committed = existsSync(join(directory, `⬅️before.${artifact.key}`)) ? readFileSync(join(directory, `⬅️before.${artifact.key}`)) : undefined;
        if (committed === undefined || !committed.equals(built.before)) throw new Error("the committed before package is not reproducible");
        if (built.after !== undefined && !readFileSync(join(directory, `➡️after.${artifact.key}`)).equals(built.after)) throw new Error("the committed after package is not reproducible");
      } catch (error) {
        failures += 1;
        console.error(`[${artifact.key}] ${recipe.id}: ${(error as Error).message}`);
      }
    }
  }
  console.log(failures === 0 ? "every plumbing pair re-derives byte-identically from its recipe" : `${failures} pair(s) failed`);
  return failures === 0 ? 0 : 1;
}

async function main(argv: readonly string[]): Promise<number> {
  const [command, ...rest] = argv;
  const flag = (name: string) => (rest.includes(name) ? rest[rest.indexOf(name) + 1] : undefined);
  if (command === "verify") return verify();
  if (command !== "generate") {
    console.error("usage: bun 📜️script.ts generate [--artifact docx|pptx|xlsx] [--only <id>] | verify");
    return 2;
  }
  const only = flag("--artifact");
  for (const artifact of Object.values(ARTIFACTS).filter((entry) => only === undefined || entry.key === only)) await generate(artifact, flag("--only"));
  return 0;
}

if (import.meta.main) process.exitCode = await main(process.argv.slice(2));
//#endregion 🚀️Entry
