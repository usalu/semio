/** 🧪️ Independent JSON Schema projection and topological oracle for Layout graph witnesses. */
import Ajv from "ajv";
import jsonPatch from "fast-json-patch";
const { applyPatch } = jsonPatch;

const nonempty = { type: "string", pattern: "\\S" };
const ids = { type: "array", uniqueItems: true, items: nonempty };
const finite = { type: "number" };
const positive = { type: "number", exclusiveMinimum: 0 };
const nonnegative = { type: "number", minimum: 0 };
const nullable = (schema) => ({ anyOf: [schema, { type: "null" }] });
const allowed = (values) => values.length ? { enum: [...new Set(values)] } : false;
const ajv = new Ajv({ strict: false, allErrors: true, strictNumbers: true });

/** 📋️ Validates independent projections rather than calling the native validator or mutation guards. */
export function graphValid(document) {
  document = JSON.parse(JSON.stringify(document));
  const facts = [];
  const rules = [];
  const check = (value, schema) => { facts.push(value); rules.push(schema); };
  const referenceText={type:'string',pattern:'[^\\p{White_Space}]'},referenceDialect={type:'object',required:['artifactKind','standard','subset'],additionalProperties:false,properties:{artifactKind:{const:'s.draw.drawing'},standard:{const:'1'},subset:{const:'*'}}},reference={type:'object',required:['handle'],additionalProperties:false,properties:{handle:{type:'object',required:['childId','target'],additionalProperties:false,properties:{childId:referenceText,target:{type:'object',required:['artifactId','dialect'],additionalProperties:false,properties:{artifactId:referenceText,dialect:referenceDialect}}}}}};
  const references=[...(document.backgroundDrawing===undefined?[]:[document.backgroundDrawing]),...document.pages.flatMap(page=>page.backgroundDrawing===undefined?[]:[page.backgroundDrawing])];
  check(references,{type:'array',items:reference});check(references.map(member=>member.handle.childId),{type:'array',uniqueItems:true});check(references.map(member=>member.handle.target.artifactId),{type:'array',uniqueItems:true});
  const members = (rows) => rows.map((row) => row.id);
  const rect = (box, zero = true, rotated = false) => {
    check([box.x, box.y, ...(rotated ? [box.rotation] : [])], { type: "array", items: finite });
    check([box.w, box.h], { type: "array", items: zero ? nonnegative : positive });
  };
  const threads = (frames) => {
    const texts = frames.filter((frame) => frame.kind === "text");
    const targets = members(texts);
    const byId = new Map(texts.map((frame) => [frame.id, frame]));
    for (const frame of texts) check(frame.threadNext, nullable(allowed(targets)));
    const incoming = new Map(texts.map((frame) => [frame.id, 0]));
    for (const frame of texts) if (incoming.has(frame.threadNext)) incoming.set(frame.threadNext, incoming.get(frame.threadNext) + 1);
    for (const frame of texts) if (frame.threadNext !== null) check(frame.storyId, byId.has(frame.threadNext) ? { const: byId.get(frame.threadNext).storyId } : false);
    check([...incoming.values()], { type: "array", items: { type: "integer", maximum: 1 } });
    const ready = texts.filter((frame) => incoming.get(frame.id) === 0).map((frame) => frame.id);
    for (let at = 0; at < ready.length; at++) {
      const next = byId.get(ready[at]).threadNext;
      if (incoming.has(next)) {
        incoming.set(next, incoming.get(next) - 1);
        if (incoming.get(next) === 0) ready.push(next);
      }
    }
    check(ready.length, { const: texts.length });
  };
  const content = (page, parent = false) => {
    check(members(page.layers), ids);
    check(members(page.frames), ids);
    check(page.layerIds, { const: members(page.layers) });
    for (const layer of page.layers) check(layer.objectIds, { const: page.frames.filter((frame) => frame.layerId === layer.id).map((frame) => frame.id) });
    for (const frame of page.frames) {
      check(frame.layerId, allowed(members(page.layers)));
      rect(frame.bounds, false, true);
      if (frame.kind === "text") {
        check(frame.storyId, allowed(members(document.stories)));
        check(frame.columns, { type: "integer", minimum: 1 });
        rect(frame.inset);
      } else if (frame.kind === "image") check(frame.linkId, allowed(members(document.links)));
      else for (const color of [frame.fill, frame.stroke]) check(color, nullable({ type: "array", items: finite, minItems: 4, maxItems: 4 }));
    }
    if (parent) threads(page.frames);
  };
  check(document.schema, { const: "layout.layout" });
  check(document.grid.baselineGrid, positive);
  check(document.grid.baselineOffset, finite);
  for (const collection of ["paragraphStyles", "characterStyles", "stories", "links", "parentPages", "spreads", "pages"]) check(members(document[collection]), ids);
  check(document.pages.flatMap((page) => members(page.frames)), ids);
  for (const style of document.paragraphStyles) {
    for (const name of [style.name, style.fontFamily]) check(name, nonempty);
    for (const size of [style.fontSize, style.leading]) check(size, positive);
    check(style.fontWeight, { type: "integer", minimum: 1 });
    check(style.tracking, finite);
    check(style.alignment, { enum: ["left", "center", "right", "justify"] });
  }
  for (const style of document.characterStyles) {
    check(style.fontSize, nullable(positive));
    check(style.fontWeight, nullable({ type: "integer", minimum: 1 }));
    check(style.tracking, nullable(finite));
    check(style.color, nullable({ type: "array", minItems: 4, maxItems: 4, items: finite }));
  }
  for (const story of document.stories) {
    const boundaries = [0];
    let length = 0;
    for (const character of story.content) { length += new TextEncoder().encode(character).length; boundaries.push(length); }
    for (const run of story.styleRuns) {
      check(run.start, allowed(boundaries.filter((offset) => offset <= run.end)));
      check(run.end, allowed(boundaries));
      check(run.paragraphStyleId, nullable(allowed(members(document.paragraphStyles))));
      check(run.characterStyleId, nullable(allowed(members(document.characterStyles))));
    }
  }
  for (const parent of document.parentPages) {
    check(parent.name, nonempty);
    check([parent.width, parent.height], { type: "array", items: positive });
    content(parent, true);
  }
  for (const spread of document.spreads) {
    check(spread.name, nonempty);
    check(spread.pageIds, { ...ids, items: allowed(document.pages.filter((page) => page.spreadId === spread.id).map((page) => page.id)) });
  }
  for (const page of document.pages) {
    check([page.width, page.height], { type: "array", items: positive });
    check(Object.values(page.margins), { type: "array", items: finite });
    check(page.columns.count, { type: "integer", minimum: 1 });
    check(page.columns.gutter, nonnegative);
    page.guides.forEach((guide) => rect(guide));
    check(page.spreadId, allowed(document.spreads.filter((spread) => spread.pageIds.includes(page.id)).map((spread) => spread.id)));
    content(page);
    const parent = document.parentPages.find((row) => row.id === page.parentPageId);
    check(page.parentPageId, nullable(allowed(members(document.parentPages))));
    check(page.overrides.map((entry) => entry.objectId), { ...ids, items: allowed(parent ? members(parent.frames) : []) });
    for (const entry of page.overrides) if (entry.bounds) rect(entry.bounds, true, true);
    if (parent) check([...members(parent.frames), ...members(page.frames)], ids);
  }
  threads(document.pages.flatMap((page) => page.frames));
  return ajv.compile({ type: "array", items: rules, minItems: rules.length, maxItems: rules.length })(facts);
}

/** 🧩️ Parent content is independently canonicalized with JSON Patch before graph admission. */
export function normalizedParent(page) {
  const patches = [{ op: "replace", path: "/layerIds", value: page.layers.map((layer) => layer.id) }];
  page.layers.forEach((layer, index) => patches.push({ op: "replace", path: "/layers/" + index + "/objectIds", value: page.frames.filter((frame) => frame.layerId === layer.id).map((frame) => frame.id) }));
  return applyPatch(structuredClone(page), patches, true, false).newDocument;
}

/** ⚖️ Returns the exact portable observations or throws on an oracle disagreement. */
export function verifyGraphValidation(corpus) {
  let documents = 0;
  let parents = 0;
  for (const vector of corpus.documents) {
    const saved = JSON.stringify(corpus.base);
    const document = applyPatch(structuredClone(corpus.base), vector.patch, true, false).newDocument;
    if (graphValid(document) !== vector.valid) throw new Error(vector.name + ": document oracle mismatch");
    if (JSON.stringify(corpus.base) !== saved) throw new Error(vector.name + ": oracle mutated base");
    documents++;
  }
  for (const vector of corpus.parents) {
    const page = normalizedParent(vector.page);
    const document = { ...structuredClone(corpus.base), parentPages: [...corpus.base.parentPages, page] };
    if (graphValid(document) !== vector.valid) throw new Error(vector.name + ": parent oracle mismatch");
    if (vector.valid && JSON.stringify(page) !== JSON.stringify(vector.normalizedPage)) throw new Error(vector.name + ": normalization mismatch");
    parents++;
  }
  return { documents, parents };
}
