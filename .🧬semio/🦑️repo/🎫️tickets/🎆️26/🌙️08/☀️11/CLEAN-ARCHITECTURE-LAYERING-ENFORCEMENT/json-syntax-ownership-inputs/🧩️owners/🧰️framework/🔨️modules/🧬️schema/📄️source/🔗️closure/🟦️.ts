import { decodeJsonSyntax, JsonSyntaxError, JsonSyntaxWorkspace, type JsonSyntaxNode, type JsonSyntaxErrorCode } from "../../../🎒️pack/🔤️json/📥️decode/🟦️.ts";
import { JsonMemberPolicy } from "../../../🎒️pack/🔤️json/🧩️members/🟦️.ts";

/** 📄️ Captured immutable source coordinates, with an explicit retrieval URI. */
export interface SchemaSourceInput { readonly id: string; readonly path: string; readonly baseUri: string; readonly source: string }
/** 🧭️ Declares the schema dialect, finite input authority, and opaque annotation vocabulary. */
export interface SchemaReferenceRequest { readonly dialect: "draft-07" | "2020-12"; readonly root: SchemaSourceInput; readonly resources: readonly SchemaSourceInput[]; readonly annotations: readonly string[] }
/** 📏️ Bounds parser and closure work without implicit operation limits. */
export interface SchemaReferenceLimits { readonly sourceBytes: number; readonly documentBytes: number; readonly coordinateBytes: number; readonly depth: number; readonly schemas: number; readonly resources: number; readonly references: number; readonly chunk: number }
/** 📈️ Counts admitted raw bytes, schema positions, resource identities, references, and physical inputs. */
export interface SchemaReferenceProgress { readonly sourceBytes: number; readonly coordinateBytes: number; readonly schemas: number; readonly resources: number; readonly references: number; readonly inputs: number }
/** 🚦️ Supplies cancellation, progress, and the host's asynchronous scheduling boundary. */
export interface SchemaReferenceControl { readonly syntax: JsonSyntaxWorkspace; readonly maximumUnits: number; readonly maximumOwnedBytes: number; readonly limits: SchemaReferenceLimits; readonly cancelled: () => boolean; readonly progress: (value: SchemaReferenceProgress) => void; readonly yield: () => Promise<void> }
/** 📍️ Identifies a schema position relative to its exact physical input. */
export interface SchemaSourcePosition { readonly id: string; readonly path: string; readonly pointer: string }
/** 🔗️ Records every reference occurrence independently of physical-document reachability. */
export interface SchemaReferenceEdge { readonly origin: SchemaSourcePosition; readonly reference: string; readonly resourceUri: string; readonly target: SchemaSourcePosition }
/** 🪪️ Associates a canonical retrieval or schema resource URI with one physical position. */
export interface SchemaSourceResource { readonly uri: string; readonly origin: SchemaSourcePosition }
/** 🗂️ Preserves source authority and the unique reachable physical inputs excluding the root. */
export interface SchemaReferenceClosure { readonly root: SchemaSourceInput; readonly documents: readonly SchemaSourceInput[]; readonly edges: readonly SchemaReferenceEdge[]; readonly resources: readonly SchemaSourceResource[] }
/** 🏷️ Enumerates closed source authority refusals and the owned lower reader vocabulary. */
export type SchemaReferenceErrorCode = JsonSyntaxErrorCode | "coordinate-byte-budget" | "document-byte-budget" | "duplicate-anchor" | "duplicate-input-identity" | "duplicate-input-path" | "duplicate-resource-id" | "invalid-anchor" | "invalid-base-uri" | "invalid-input" | "invalid-resource-id" | "invalid-schema" | "invalid-vocabulary" | "malformed-json-pointer" | "malformed-reference-fragment" | "reference-budget" | "resource-budget" | "schema-budget" | "source-byte-budget" | "unknown-anchor" | "unknown-keyword" | "unknown-resource" | "unknown-schema-pointer" | "unsupported-dialect" | "unsupported-dynamic-reference" | "unsupported-keyword" | "unsupported-vocabulary";
/** ⚠️ Reports deterministic owned refusal with source position authority. */
export class SchemaReferenceError extends Error {
  constructor(readonly code: SchemaReferenceErrorCode, readonly origin: SchemaSourcePosition | null, message: string) { super(message); this.name = "SchemaReferenceError"; }
}

type SchemaNode = { input: SchemaSourceInput; pointer: string; node: JsonSyntaxNode; uri: string };
type PendingReference = { origin: SchemaSourcePosition; reference: string; uri: string };
const dynamic = new Set(["$dynamicRef", "$dynamicAnchor", "$recursiveRef", "$recursiveAnchor"]);
const annotations = new Set(["title", "description", "default", "examples", "const", "enum", "type", "format", "readOnly", "writeOnly", "deprecated", "$comment", "multipleOf", "minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum", "minLength", "maxLength", "pattern", "minItems", "maxItems", "uniqueItems", "minProperties", "maxProperties", "required", "contentEncoding", "contentMediaType"]);
const maps07 = ["definitions", "$defs", "properties", "patternProperties"];
const maps2020 = ["$defs", "properties", "patternProperties", "dependentSchemas"];
const single07 = ["additionalProperties", "additionalItems", "contains", "propertyNames", "not", "if", "then", "else"];
const single2020 = ["additionalProperties", "contains", "propertyNames", "not", "if", "then", "else", "unevaluatedProperties", "unevaluatedItems", "contentSchema"];
const arrays07 = ["allOf", "anyOf", "oneOf"];
const arrays2020 = [...arrays07, "prefixItems"];
const reserved = new Set([...dynamic, ...annotations, ...maps07, ...maps2020, ...single07, ...single2020, ...arrays2020, "$id", "$schema", "$ref", "$anchor", "$vocabulary", "items", "dependencies", "dependentRequired", "minContains", "maxContains"]);
const encoder = new TextEncoder();

function refuse(code: SchemaReferenceErrorCode, origin: SchemaSourcePosition | null, message: string): never { throw new SchemaReferenceError(code, origin, message); }
function data(value: unknown, keys: readonly string[], code: SchemaReferenceErrorCode): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value) || ![Object.prototype, null].includes(Object.getPrototypeOf(value))) refuse(code, null, "Expected exact own data");
  const own = Reflect.ownKeys(value);
  if (own.length !== keys.length || own.some(key => typeof key !== "string" || !keys.includes(key))) refuse(code, null, "Unexpected data members");
  const descriptors = Object.getOwnPropertyDescriptors(value);
  const result: Record<string, unknown> = Object.create(null);
  for (const key of keys) { const descriptor = descriptors[key]!; if (!Object.hasOwn(descriptor, "value") || !descriptor.enumerable) refuse(code, null, "Accessors and hidden members are not captured data"); result[key] = descriptor.value; }
  return result;
}
function dense(value: unknown): unknown[] {
  if (!Array.isArray(value) || Object.getPrototypeOf(value) !== Array.prototype) refuse("invalid-input", null, "Expected dense own array");
  const descriptors = Object.getOwnPropertyDescriptors(value), keys = Reflect.ownKeys(value);
  if (keys.length !== value.length + 1 || keys.some(key => typeof key !== "string" || (key !== "length" && !/^(0|[1-9][0-9]*)$/.test(key)))) refuse("invalid-input", null, "Expected dense own array");
  const result: unknown[] = [];
  for (let index = 0; index < value.length; index++) { const descriptor = descriptors[String(index)]; if (!descriptor || !Object.hasOwn(descriptor, "value") || !descriptor.enumerable) refuse("invalid-input", null, "Array accessors are not captured data"); result.push(descriptor.value); }
  return result;
}
function unicode(value: string, origin: SchemaSourcePosition | null): string {
  for (let index = 0; index < value.length; index++) { const unit = value.charCodeAt(index); if (unit >= 0xd800 && unit <= 0xdbff) { const next = value.charCodeAt(++index); if (!(next >= 0xdc00 && next <= 0xdfff)) refuse("invalid-unicode", origin, "Unpaired high surrogate"); } else if (unit >= 0xdc00 && unit <= 0xdfff) refuse("invalid-unicode", origin, "Unpaired low surrogate"); }
  return value;
}
function uri(value: string, base: string | undefined, code: SchemaReferenceErrorCode, origin: SchemaSourcePosition | null): URL {
  unicode(value, origin);
  if (/%(?![0-9a-fA-F]{2})/.test(value)) refuse(code, origin, "Malformed URI escape");
  try { return base === undefined ? new URL(value) : new URL(value, base); } catch { return refuse(code, origin, "Unresolvable URI"); }
}
function position(input: SchemaSourceInput, pointer: string): SchemaSourcePosition { return Object.freeze({ id: input.id, path: input.path, pointer }); }
function pointer(parent: string, child: string): string { return parent + "/" + child.replace(/~/g, "~0").replace(/\//g, "~1"); }
async function ordered<T>(values: readonly T[], key: (value: T) => string, checkpoint: () => Promise<void>): Promise<T[]> {
  let rows: { value: T; bytes: Uint8Array }[] = [];
  for (const value of values) { await checkpoint(); rows.push({ value, bytes: encoder.encode(key(value)) }); }
  const compare = async (a: Uint8Array, b: Uint8Array) => { for (let index = 0; index < Math.min(a.length, b.length); index++) { await checkpoint(); if (a[index] !== b[index]) return a[index]! - b[index]!; } return a.length - b.length; };
  for (let width = 1; width < rows.length; width *= 2) {
    const next: typeof rows = [];
    for (let start = 0; start < rows.length; start += width * 2) {
      let left = start, right = Math.min(start + width, rows.length);
      const middle = right, end = Math.min(start + width * 2, rows.length);
      while (left < middle || right < end) { await checkpoint(); next.push(right >= end || (left < middle && await compare(rows[left]!.bytes, rows[right]!.bytes) <= 0) ? rows[left++]! : rows[right++]!); }
    }
    rows = next;
  }
  const result: T[] = [];
  for (const row of rows) { await checkpoint(); result.push(row.value); }
  return result;
}
async function members(node: JsonSyntaxNode, origin: SchemaSourcePosition, checkpoint: () => Promise<void>): Promise<Map<string, JsonSyntaxNode>> {
  if (node.kind !== "object") refuse("invalid-schema", origin, "Schema map must be an object");
  const result = new Map<string, JsonSyntaxNode>();
  for (const member of node.members) { await checkpoint(); result.set(unicode(member.name, origin), member.value); }
  return result;
}
function schemaText(node: JsonSyntaxNode, origin: SchemaSourcePosition): string { if (node.kind !== "string") refuse("invalid-schema", origin, "Schema identity and reference must be strings"); return unicode(node.value, origin); }

/** 🔍️ Resolves captured schema references without implicit source discovery or partial admission. */
export async function schemaReferenceClosure(request: SchemaReferenceRequest, control: SchemaReferenceControl): Promise<SchemaReferenceClosure> {
  const operation = data(control, ["limits", "syntax", "maximumUnits", "maximumOwnedBytes", "cancelled", "progress", "yield"], "invalid-control");
  const bounds = data(operation.limits, ["sourceBytes", "documentBytes", "coordinateBytes", "depth", "schemas", "resources", "references", "chunk"], "invalid-control");
  for (const [key, value] of Object.entries(bounds)) if (!Number.isSafeInteger(value) || (value as number) < 1 || (key === "chunk" && (value as number) > 4096)) refuse("invalid-control", null, "Invalid finite operation limit");
  for (const key of ["cancelled", "progress", "yield"]) if (typeof operation[key] !== "function") refuse("invalid-control", null, "Expected operation callback");
  if (!(operation.syntax instanceof JsonSyntaxWorkspace) || !Number.isSafeInteger(operation.maximumUnits) || (operation.maximumUnits as number) < 1 || !Number.isSafeInteger(operation.maximumOwnedBytes) || (operation.maximumOwnedBytes as number) < 1) refuse("invalid-control", null, "Explicit parser ownership authority required");
  const limits = bounds as unknown as SchemaReferenceLimits;
  const cancelled = operation.cancelled as () => boolean, progress = operation.progress as SchemaReferenceControl["progress"], schedule = operation.yield as SchemaReferenceControl["yield"];
  const state = { sourceBytes: 0, coordinateBytes: 0, schemas: 0, resources: 0, references: 0, inputs: 0 };
  const check = () => { if (cancelled()) refuse("cancelled", null, "Schema source operation cancelled"); };
  check();
  const input = data(request, ["dialect", "root", "resources", "annotations"], "invalid-input");
  if (input.dialect !== "draft-07" && input.dialect !== "2020-12") refuse("invalid-input", null, "Unsupported schema dialect");
  const dialect = input.dialect;
  const capture = (value: unknown): SchemaSourceInput => {
    const row = data(value, ["id", "path", "baseUri", "source"], "invalid-input");
    if (typeof row.source !== "string" || row.source.length === 0) refuse("invalid-input", null, "Expected raw JSON source");
    if (row.source.length > limits.documentBytes) refuse("document-byte-budget", null, "Source code units exceed the document byte limit");
    for (const key of ["id", "path", "baseUri"]) if (typeof row[key] === "string" && (row[key] as string).length > limits.documentBytes) refuse("invalid-input", null, "Source coordinates exceed the document extent");
    for (const key of ["id", "path", "baseUri"]) if (typeof row[key] !== "string" || (row[key] as string).length === 0) refuse("invalid-input", null, "Expected nonempty source coordinate");
    const result = { id: row.id as string, path: row.path as string, baseUri: row.baseUri as string, source: row.source };
    return Object.freeze(result);
  };
  if (!Array.isArray(input.resources)) refuse("invalid-input", null, "Expected explicit resource array");
  if (input.resources.length + 1 > limits.resources) refuse("resource-budget", null, "Physical inputs exceed resource limit");
  const root = capture(input.root), supplied = dense(input.resources);
  const inputs = [root, ...supplied.map(capture)];
  if (!Array.isArray(input.annotations)) refuse("invalid-input", null, "Expected annotation array");
  if (input.annotations.length > limits.resources) refuse("invalid-vocabulary", null, "Annotation vocabulary exceeds operation extent");
  const custom = dense(input.annotations).map(value => { if (typeof value !== "string" || value.length === 0) refuse("invalid-vocabulary", null, "Expected annotation keyword"); return value; });
  let coordinateUnits = 0;
  for (const row of inputs) { coordinateUnits += row.id.length + row.path.length + row.baseUri.length; if (coordinateUnits > limits.coordinateBytes) refuse("coordinate-byte-budget", null, "Source coordinate extent exceeds operation byte authority"); }
  for (const keyword of custom) { coordinateUnits += keyword.length; if (coordinateUnits > limits.coordinateBytes) refuse("coordinate-byte-budget", null, "Annotation extent exceeds operation byte authority"); }
  if (new Set(custom).size !== custom.length || custom.some(key => reserved.has(key))) refuse("invalid-vocabulary", null, "Annotation vocabulary overlaps declared keywords");
  const opaque = new Set([...annotations, ...custom, ...(dialect === "2020-12" ? ["dependentRequired", "minContains", "maxContains"] : [])]);
  const mapKeywords = new Set(dialect === "draft-07" ? maps07 : maps2020), singleKeywords = new Set(dialect === "draft-07" ? single07 : single2020), arrayKeywords = new Set(dialect === "draft-07" ? arrays07 : arrays2020);
  const resources = new Map<string, SchemaNode>(), anchors = new Map<string, SchemaNode>(), schemas = new Map<string, Map<string, SchemaNode>>(), references: PendingReference[] = [];
  let work = 0;
  const emit = () => { check(); progress(Object.freeze({ ...state })); };
  const checkpoint = async () => { check(); if (++work % limits.chunk === 0) { emit(); await schedule(); check(); } };
  const admitCoordinate = async (value: string): Promise<void> => {
    for (let index = 0; index < value.length; index++) {
      await checkpoint();
      const unit = value.charCodeAt(index);
      let bytes = unit < 0x80 ? 1 : unit < 0x800 ? 2 : 3;
      if (unit >= 0xd800 && unit <= 0xdbff) { const next = value.charCodeAt(++index); if (!(next >= 0xdc00 && next <= 0xdfff)) refuse("invalid-unicode", null, "Coordinate has an unpaired surrogate"); bytes = 4; }
      else if (unit >= 0xdc00 && unit <= 0xdfff) refuse("invalid-unicode", null, "Coordinate has an unpaired surrogate");
      if (state.coordinateBytes + bytes > limits.coordinateBytes) refuse("coordinate-byte-budget", null, "Captured coordinate bytes exceed limit");
      state.coordinateBytes += bytes;
    }
  };
  const register = (name: string, node: SchemaNode): void => {
    const old = resources.get(name);
    if (old && (old.input.id !== node.input.id || old.pointer !== node.pointer)) refuse("duplicate-resource-id", position(node.input, node.pointer), "Resource identity is ambiguous");
    if (!old) { if (state.resources >= limits.resources) refuse("resource-budget", position(node.input, node.pointer), "Resource identities exceed limit"); resources.set(name, node); state.resources++; }
  };
  emit();
  const byId = new Map<string, SchemaSourceInput>(), paths = new Set<string>();
  for (const row of inputs) {
    for (const coordinate of [row.id, row.path, row.baseUri]) await admitCoordinate(coordinate);
    const base = uri(row.baseUri, undefined, "invalid-base-uri", null);
    if (base.hash || row.baseUri.includes("#")) refuse("invalid-base-uri", null, "Retrieval URI must not have a fragment");
    if (byId.has(row.id)) refuse("duplicate-input-identity", position(row, ""), "Input identity is repeated");
    if (paths.has(row.path)) refuse("duplicate-input-path", position(row, ""), "Input path is repeated");
    byId.set(row.id, row); paths.add(row.path);
  }
  for (const keyword of custom) await admitCoordinate(keyword);
  for (const source of inputs) {
    check();
    const previous = state.sourceBytes, remaining = limits.sourceBytes - previous, allowance = Math.min(remaining, limits.documentBytes);
    if (allowance < 1) refuse("source-byte-budget", position(source, ""), "Total source byte limit reached");
    let parsed: JsonSyntaxNode;
    try {
      parsed = await decodeJsonSyntax(source.source, JsonMemberPolicy.Reject, { workspace: operation.syntax, maximumUnits: operation.maximumUnits as number, maximumOwnedBytes: operation.maximumOwnedBytes as number, maximumBytes: allowance, maximumNodes: allowance, maximumDepth: limits.depth, chunk: limits.chunk, cancelled, yield: schedule, progress: value => { state.sourceBytes = previous + value.bytes; emit(); } });
    } catch (error) {
      if (!(error instanceof JsonSyntaxError)) throw error;
      const code = error.code === "byte-budget" ? (remaining <= limits.documentBytes ? "source-byte-budget" : "document-byte-budget") : error.code;
      refuse(code, position(source, ""), error.message);
    }
    state.inputs++;
    const physical = new Map<string, SchemaNode>(); schemas.set(source.id, physical);
    const base = uri(source.baseUri, undefined, "invalid-base-uri", position(source, "")).href;
    const stack = [{ pointer: "", node: parsed, base, depth: 1 }];
    while (stack.length) {
      const current = stack.pop()!, origin = position(source, current.pointer);
      await checkpoint();
      if (current.depth > limits.depth) refuse("depth-budget", origin, "Schema nesting exceeds limit");
      if (current.node.kind !== "object" && current.node.kind !== "boolean") refuse("invalid-schema", origin, "Schema must be an object or boolean");
      if (state.schemas >= limits.schemas) refuse("schema-budget", origin, "Schema positions exceed limit");
      state.schemas++;
      const fields = current.node.kind === "object" ? await members(current.node, origin, checkpoint) : new Map<string, JsonSyntaxNode>();
      let scope = current.base;
      if (fields.has("$id")) { const raw = schemaText(fields.get("$id")!, origin), id = uri(raw, scope, "invalid-resource-id", origin); if (id.hash || raw.includes("#")) refuse("invalid-resource-id", origin, "Fragment identifiers require declared static anchors"); scope = id.href; }
      const owned = { input: source, pointer: current.pointer, node: current.node, uri: scope }; physical.set(current.pointer, owned);
      if (current.pointer === "") register(base, owned);
      if (fields.has("$id") || current.pointer === "") { if (resources.has(scope) && resources.get(scope)!.pointer !== current.pointer) refuse("duplicate-resource-id", origin, "Resource identity is repeated"); register(scope, owned); }
      if (fields.has("$schema")) { const declared = schemaText(fields.get("$schema")!, origin); const accepted = dialect === "draft-07" ? ["http://json-schema.org/draft-07/schema", "http://json-schema.org/draft-07/schema#"] : ["https://json-schema.org/draft/2020-12/schema", "https://json-schema.org/draft/2020-12/schema#"]; if (!accepted.includes(declared)) refuse("unsupported-dialect", origin, "Schema dialect disagrees with request"); }
      if (fields.has("$anchor")) {
        if (dialect !== "2020-12") refuse("unsupported-keyword", origin, "Static anchor requires declared 2020 dialect");
        const name = schemaText(fields.get("$anchor")!, origin);
        if (!/^[A-Za-z_][-A-Za-z0-9._]*$/.test(name)) refuse("invalid-anchor", origin, "Invalid static anchor");
        const key = scope + "#" + name; if (anchors.has(key)) refuse("duplicate-anchor", origin, "Static anchor is repeated"); anchors.set(key, owned);
      }
      const children: { pointer: string; node: JsonSyntaxNode; base: string; depth: number }[] = [];
      const add = (at: string, node: JsonSyntaxNode) => { if (state.schemas + stack.length + children.length >= limits.schemas) refuse("schema-budget", position(source, at), "Pending schemas exceed limit"); children.push({ pointer: at, node, base: scope, depth: current.depth + 1 }); };
      for (const [key, node] of fields) {
        await checkpoint();
        const at = pointer(current.pointer, key), childOrigin = position(source, at);
        if (dynamic.has(key)) refuse("unsupported-dynamic-reference", childOrigin, "Dynamic reference semantics are not admitted");
        if (key === "$ref") { const reference = schemaText(node, childOrigin); if (references.length >= limits.references) refuse("reference-budget", childOrigin, "Reference occurrences exceed limit"); references.push({ origin: childOrigin, reference, uri: scope }); }
        else if (key === "$id" || key === "$schema" || key === "$anchor" || opaque.has(key)) continue;
        else if (key === "$vocabulary") refuse("unsupported-vocabulary", childOrigin, "Vocabulary authority is explicit in the request");
        else if (mapKeywords.has(key)) { if (node.kind !== "object") refuse("invalid-schema", childOrigin, "Schema map must be an object"); for (const member of node.members) { await checkpoint(); add(pointer(at, unicode(member.name, childOrigin)), member.value); } }
        else if (singleKeywords.has(key)) add(at, node);
        else if (arrayKeywords.has(key)) { if (node.kind !== "array") refuse("invalid-schema", childOrigin, "Schema alternatives must be an array"); for (let index = 0; index < node.items.length; index++) { await checkpoint(); add(pointer(at, String(index)), node.items[index]!); } }
        else if (key === "items") { if (node.kind === "array") { if (dialect !== "draft-07") refuse("invalid-schema", childOrigin, "Tuple items are not admitted in 2020 dialect"); for (let index = 0; index < node.items.length; index++) { await checkpoint(); add(pointer(at, String(index)), node.items[index]!); } } else add(at, node); }
        else if (key === "dependencies" && dialect === "draft-07") { for (const [name, schema] of await members(node, childOrigin, checkpoint)) { await checkpoint(); if (schema.kind === "array") { for (const item of schema.items) { await checkpoint(); if (item.kind !== "string") refuse("invalid-schema", childOrigin, "Property dependency must contain strings"); } } else add(pointer(at, name), schema); } }
        else refuse("unknown-keyword", childOrigin, "Keyword has no declared schema or annotation semantics");
      }
      for (let index = children.length - 1; index >= 0; index--) { await checkpoint(); stack.push(children[index]!); }
    }
    emit();
  }
  const edges: SchemaReferenceEdge[] = [];
  for (const pending of references) {
    await checkpoint();
    const resolved = uri(pending.reference, pending.uri, "malformed-reference-fragment", pending.origin), fragment = resolved.hash.slice(1);
    resolved.hash = "";
    const resourceUri = resolved.href, resource = resources.get(resourceUri);
    if (!resource) refuse("unknown-resource", pending.origin, "Reference resource is not explicitly registered");
    let decoded: string;
    try { decoded = decodeURIComponent(fragment); } catch { refuse("malformed-reference-fragment", pending.origin, "Invalid reference percent encoding"); }
    unicode(decoded, pending.origin);
    let target = resource;
    if (decoded.startsWith("/")) {
      const parts = decoded.slice(1).split("/");
      let targetPointer = resource.pointer;
      for (const part of parts) { await checkpoint(); if (/~(?:[^01]|$)/.test(part)) refuse("malformed-json-pointer", pending.origin, "Invalid JSON Pointer escape"); targetPointer = pointer(targetPointer, part.replace(/~1/g, "/").replace(/~0/g, "~")); }
      const candidate = schemas.get(resource.input.id)!.get(targetPointer);
      if (!candidate) refuse("unknown-schema-pointer", pending.origin, "Pointer does not identify a schema position");
      target = candidate;
    } else if (decoded !== "") { const candidate = anchors.get(resource.uri + "#" + decoded); if (!candidate) refuse("unknown-anchor", pending.origin, "Anchor is not explicitly registered"); target = candidate; }
    edges.push(Object.freeze({ origin: pending.origin, reference: pending.reference, resourceUri, target: position(target.input, target.pointer) })); state.references++; emit();
  }
  const outgoing = new Map<string, string[]>();
  for (const edge of edges) { await checkpoint(); const list = outgoing.get(edge.origin.id) ?? []; list.push(edge.target.id); outgoing.set(edge.origin.id, list); }
  const reached = new Set([root.id]), queue = [root.id];
  for (let index = 0; index < queue.length; index++) for (const target of outgoing.get(queue[index]!) ?? []) { await checkpoint(); if (!reached.has(target)) { reached.add(target); queue.push(target); } }
  const documentRows: SchemaSourceInput[] = [];
  for (const row of inputs) { await checkpoint(); if (row.id !== root.id && reached.has(row.id)) documentRows.push(row); }
  const documents = await ordered(documentRows, row => row.id, checkpoint);
  const resourceRows: SchemaSourceResource[] = [];
  for (const [uri, node] of resources) { await checkpoint(); resourceRows.push(Object.freeze({ uri, origin: position(node.input, node.pointer) })); }
  const indexed = await ordered(resourceRows, row => row.uri, checkpoint);
  emit();
  return Object.freeze({ root, documents: Object.freeze(documents), edges: Object.freeze(edges), resources: Object.freeze(indexed) });
}
