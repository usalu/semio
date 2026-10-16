import ts from "typescript";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const FOLDER = join(REPO, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split");
const ORIGINAL = join(FOLDER, "met-tools/gis-original.ts.txt");
const DRY = process.argv.includes("--dry");

const ns = (emoji: string, slug: string): string => emoji.replace(/️/gu, "") + "️" + slug;
const text = readFileSync(ORIGINAL, "utf8");
const sf = ts.createSourceFile("original.ts", text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);

const GIS_TESTS_DIR = join(REPO, "✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests");
const OS_ROOT = resolve(GIS_TESTS_DIR, "../../../../../../🧰️framework/🛍️products/💻️os");
const OS_MODULES = join(OS_ROOT, ns("🔨", "modules"));
const OS_TESTS = join(OS_ROOT, ns("🧪", "tests"));
for (const dir of [GIS_TESTS_DIR, OS_ROOT, OS_MODULES, OS_TESTS, join(OS_MODULES, ns("📇", "directory")), join(OS_ROOT, ns("🎚", "config"))]) if (!existsSync(dir)) throw new Error(`missing ${dir}`);
const DIRECTORY_TESTS = join(OS_MODULES, ns("📇", "directory"), ns("🧪", "tests"));
const CONFIG_TESTS = join(OS_ROOT, ns("🎚", "config"), ns("🧪", "tests"));
const FILE = ns("🟦", "") + ".ts";

type Scope = "B" | "O" | "A" | "D";
type Item = { readonly scope: Scope; readonly index: number };

const fnDecl = sf.statements.find((s): s is ts.FunctionDeclaration => ts.isFunctionDeclaration(s) && s.name?.text === "registerTests1")!;
const B = [...fnDecl.body!.statements];
const lineOf = (pos: number) => sf.getLineAndCharacterOfPosition(pos).line + 1;
const callbackBody = (statement: ts.Statement): ts.Block => {
  const call = (statement as ts.ExpressionStatement).expression as ts.CallExpression;
  const callback = call.arguments[call.arguments.length - 1] as ts.ArrowFunction;
  return callback.body as ts.Block;
};
const O = [...callbackBody(B[71]!).statements];
const A = [...callbackBody(B[64]!).statements];
const D = [...callbackBody(B[68]!).statements];
const scopes: Record<Scope, readonly ts.Statement[]> = { B, O, A, D };
const nodeOf = (item: Item) => scopes[item.scope][item.index]!;

const declNames = (statement: ts.Statement): string[] => {
  const out: string[] = [];
  if (ts.isVariableStatement(statement)) {
    for (const declaration of statement.declarationList.declarations) {
      if (ts.isIdentifier(declaration.name)) out.push(declaration.name.text);
      else if (ts.isObjectBindingPattern(declaration.name)) for (const element of declaration.name.elements) if (ts.isIdentifier(element.name)) out.push(element.name.text);
    }
  } else if ((ts.isFunctionDeclaration(statement) || ts.isTypeAliasDeclaration(statement) || ts.isClassDeclaration(statement)) && statement.name) out.push(statement.name.text);
  return out;
};
const identifiersOf = (node: ts.Node, out: Set<string>) => {
  if (ts.isIdentifier(node)) {
    const parent = node.parent;
    const property = (ts.isPropertyAccessExpression(parent) && parent.name === node) || (ts.isPropertyAssignment(parent) && parent.name === node) || (ts.isQualifiedName(parent) && parent.right === node);
    if (!property) out.add(node.text);
  }
  node.forEachChild((child) => identifiersOf(child, out));
};

const depNames = new Set<string>(["testSeams", "DOCUMENT_BACKBONE_RETENTION_LIMITS", "handleAck"]);
for (const binding of (B[1] as ts.VariableStatement).declarationList.declarations[0]!.name.getChildren(sf)) void binding;
for (const element of ((B[1] as ts.VariableStatement).declarationList.declarations[0]!.name as ts.ObjectBindingPattern).elements) depNames.add((element.name as ts.Identifier).text);
const GIS_DEPS = new Set(["bindInferenceApprovalUndoToMountedPair", "driveInferencePort", "inferenceApprovalUndoEpoch", "inferenceApprovalUndoOwner", "reissueInferenceApprovalUndoForRebootstrap", "retainInferenceApprovalUndo", "undoInferenceApproval", "idleGisMapInferencePortStatusV1", "parseGisMapInferenceApprovalReceiptV1"]);
const VITEST_MEMBERS = new Set(["beforeEach", "describe", "expect", "it", "vi"]);

type ModuleImport = { readonly name: string; readonly spec: string; readonly typeOnly: boolean };
const moduleImports = new Map<string, ModuleImport>();
for (const statement of sf.statements) {
  if (!ts.isImportDeclaration(statement) || !statement.importClause) continue;
  const clause = statement.importClause;
  const spec = (statement.moduleSpecifier as ts.StringLiteral).text;
  if (clause.namedBindings && ts.isNamedImports(clause.namedBindings)) for (const element of clause.namedBindings.elements) moduleImports.set(element.name.text, { name: element.name.text, spec, typeOnly: clause.isTypeOnly });
}

type Info = {
  readonly id: string;
  readonly item: Item;
  readonly kind: "helper" | "type" | "test" | "hook" | "other";
  readonly names: readonly string[];
  readonly used: Set<string>;
  readonly line: number;
  readonly endLine: number;
};
const collectLocalsFrom = (_node: ts.Node, _locals: Set<string>): void => {};
const infos = new Map<string, Info>();
const idOf = (item: Item) => `${item.scope}${item.index}`;
for (const scope of ["B", "O", "A", "D"] as const) {
  scopes[scope].forEach((node, index) => {
    const item = { scope, index };
    const used = new Set<string>();
    identifiersOf(node, used);
    const own = new Set(declNames(node));
    const locals = new Set<string>();
    const collectLocals = (current: ts.Node) => {
      if ((ts.isVariableDeclaration(current) || ts.isParameter(current) || ts.isBindingElement(current)) && ts.isIdentifier(current.name)) locals.add(current.name.text);
      if ((ts.isFunctionDeclaration(current) || ts.isClassDeclaration(current)) && current.name && current !== node) locals.add(current.name.text);
      current.forEachChild(collectLocals);
    };
    node.forEachChild(collectLocals);
    if (ts.isVariableStatement(node)) for (const declaration of node.declarationList.declarations) if (declaration.initializer) collectLocalsFrom(declaration.initializer, locals);
    for (const name of locals) if (!own.has(name)) used.delete(name);
    let kind: Info["kind"] = "other";
    if (ts.isTypeAliasDeclaration(node)) kind = "type";
    else if (ts.isFunctionDeclaration(node) || ts.isClassDeclaration(node) || ts.isVariableStatement(node)) kind = "helper";
    else if (ts.isExpressionStatement(node) && ts.isCallExpression(node.expression)) {
      const head = node.expression.expression.getText(sf);
      kind = head === "beforeEach" ? "hook" : /^(vitest\.)?(describe|it)$/u.test(head) ? "test" : "other";
    }
    infos.set(idOf(item), { id: idOf(item), item, kind, names: declNames(node), used, line: lineOf(node.getStart(sf)), endLine: lineOf(node.end) });
  });
}
const OUTER_OTHER = new Set(["B0", "B1", "B2", "B3", "B48"]);
const ownersByName = new Map<string, Info[]>();
for (const info of infos.values()) {
  if (OUTER_OTHER.has(info.id)) continue;
  if (info.kind === "helper" || info.kind === "type") for (const name of info.names) ownersByName.set(name, [...(ownersByName.get(name) ?? []), info]);
}
const resolveOwner = (name: string, scope: Scope): Info | undefined => {
  const candidates = ownersByName.get(name);
  if (!candidates) return undefined;
  return candidates.find((candidate) => candidate.item.scope === scope) ?? candidates.find((candidate) => candidate.item.scope === "B");
};
const refsCache = new Map<string, Info[]>();
const refsOf = (info: Info): Info[] => {
  if (!refsCache.has(info.id)) {
    const out = new Map<string, Info>();
    for (const name of info.used) {
      if (info.names.includes(name)) continue;
      const owner = resolveOwner(name, info.item.scope);
      if (owner && owner.id !== info.id) out.set(owner.id, owner);
    }
    refsCache.set(info.id, [...out.values()]);
  }
  return refsCache.get(info.id)!;
};

const reImportType = (info: Info) => info.kind === "type" && /=\s*import\(/u.test(nodeOf(info.item).getText(sf));

type Group = {
  readonly key: string;
  readonly dir: string;
  readonly fn: string;
  readonly emoji: string;
  readonly doc: string;
  readonly docEmoji?: string;
  readonly flavor: "worker" | "gis";
  readonly items: readonly string[];
  readonly wrapper?: { readonly scope: "O" | "A" | "D"; readonly title: string };
  readonly region?: string;
};
const range = (scope: Scope, from: number, to: number) => Array.from({ length: to - from + 1 }, (_unused, i) => `${scope}${from + i}`);

const groups: Group[] = [
  { key: "sessionBrokerPort", dir: join(DIRECTORY_TESTS, ns("🔑", "session-broker-port")), fn: "registerSessionBrokerPortTests", emoji: "🔑", doc: "The Shell's private session-broker port: acknowledged reads, the closed proof, and serial refresh of session authority through the actual worker broker.", flavor: "worker", items: ["B8", "B9", "B10", "B11"] },
  { key: "spaceArtifactCreation", dir: join(OS_TESTS, ns("🏗", "space-artifact-creation")), fn: "registerSpaceArtifactCreationTests", emoji: "🏗", doc: "The worker's space-artifact creation owner: catalog publication, status polling, cancellation and conflict handling over the exact retained creation owner.", flavor: "worker", items: ["B50"] },
  { key: "eventPageBootstrap", dir: join(DIRECTORY_TESTS, ns("📄", "event-page-bootstrap")), fn: "registerDirectoryEventPageBootstrapTests", emoji: "📄", doc: "The Directory event-page bootstrap: page delivery, Home acknowledgement, the canonical event-page route and the declared session routes.", flavor: "worker", items: ["B51"] },
  { key: "hubSessionCapability", dir: join(DIRECTORY_TESTS, ns("🎟", "hub-session-capability")), fn: "registerHubSessionCapabilityTests", emoji: "🎟", doc: "The hub session capability lane: the human's own bearer on every hub call, refusal without a capability and the private session port.", flavor: "worker", items: ["B52"] },
  { key: "documentActorRecovery", dir: join(OS_TESTS, ns("🚨", "document-actor-recovery-worker")), fn: "registerDocumentActorRecoveryWorkerTests", emoji: "🚨", doc: "Document actor recovery in the worker: in-flight batches return to the outbox, catch-up gates input and a refusing child reopens.", flavor: "worker", items: ["B55"] },
  { key: "wireBridge", dir: join(OS_TESTS, ns("🌉", "wire-bridge")), fn: "registerWireBridgeTests", emoji: "🌉", doc: "The worker's wire bridge: envelope round trips, frame decoding, Commands batches and session stamping.", flavor: "worker", items: ["B56"] },
  { key: "artifactBootstrapRestore", dir: join(OS_TESTS, ns("🧱", "artifact-bootstrap-restore")), fn: "registerArtifactBootstrapRestoreTests", emoji: "🧱", doc: "Atomic artifact bootstrap restore: the bootstrap owner, staged pairs, rebootstrap bounds and the durable folder envelope.", flavor: "worker", items: ["A0", "A1", "A2", "A3", "A4", "A5", "A7", "A8", "A9", "A10"], wrapper: { scope: "A", title: "artifact bootstrap atomic restore" }, region: "🧪️ArtifactBootstrapRestore" },
  { key: "identityConfigFacet", dir: join(CONFIG_TESTS, ns("🪪", "identity-config-facet")), fn: "registerIdentityConfigFacetTests", emoji: "🪪", doc: "The identity config facet: the folder lane binding, sign-in and sign-out mutations and the last-envelope-wins fold.", flavor: "worker", items: ["B65"], region: "🔖️IdentityTests" },
  { key: "configMutationParity", dir: join(CONFIG_TESTS, ns("🔧", "config-mutation-parity")), fn: "registerConfigMutationParityTests", emoji: "🔧", doc: "TypeScript parity of the opening and merge-policy config mutations and the Nx inputs that track their sources.", flavor: "worker", items: ["B66"], region: "🔖️ConfigMutationTests" },
  { key: "spaceAdministration", dir: join(DIRECTORY_TESTS, ns("🏘", "space-administration")), fn: "registerSpaceAdministrationTests", emoji: "🏘", doc: "Space administration through the worker: verified transport, page reads, sealed command receipts, invites and denial handling.", flavor: "worker", items: ["B67"], region: "🔖️DirectoryLaneTests" },
  { key: "directoryLane", dir: join(DIRECTORY_TESTS, ns("🛤", "directory-lane")), fn: "registerDirectoryLaneTests", emoji: "🛤", doc: "The worker's Directory command lane: sealed retries, bounded transport, ordered drains and the scoped stream.", flavor: "worker", items: ["D1", "D2", "D3", "D4", "D5", "D6", "D7", "D8", "D9", "D10", "D11", "D12"], wrapper: { scope: "D", title: "backbone-worker directory lane" }, region: "🔖️DirectoryLaneTests" },
  { key: "gisDirectoryLane", dir: join(GIS_TESTS_DIR, ns("🛤", "directory-lane")), fn: "registerGisDirectoryLaneTests", emoji: "🛤", docEmoji: "🚉", doc: "GIS directory lane: a scoped stream retired terminally on 4401 revokes the retained inference approval Undo owner.", flavor: "gis", items: ["D13"], wrapper: { scope: "D", title: "backbone-worker directory lane" }, region: "🔖️DirectoryLaneTests" },
  { key: "documentOpen", dir: join(OS_TESTS, ns("📂", "document-open")), fn: "registerDocumentOpenTests", emoji: "📂", doc: "Browser document open: attempt ownership, installed-artifact authority and the authenticated plan and receipt exchange.", flavor: "worker", items: ["O13", "O14", "O15", "O16", "O17", "O18", "O38"], wrapper: { scope: "O", title: "backbone-worker offline resilience" }, region: "🔖️OfflineResilienceTests" },
  { key: "executionTargetLease", dir: join(OS_TESTS, ns("🎯", "execution-target-lease")), fn: "registerExecutionTargetLeaseTests", emoji: "🎯", doc: "Execution target lease: server asset verification, hostile asset refusal, single-field substitution and bounded body readers.", flavor: "worker", items: ["O27", "O28", "O30", "O31", "O32", "O37"], wrapper: { scope: "O", title: "backbone-worker offline resilience" }, region: "🔖️OfflineResilienceTests" },
  { key: "browserActorReservation", dir: join(OS_TESTS, ns("🎭", "browser-actor-reservation")), fn: "registerBrowserActorReservationTests", emoji: "🎭", doc: "Browser document actor reservation: private grant, scope, generation and retirement laws and activation on an exact socket Session.", flavor: "worker", items: ["O33", "O34"], wrapper: { scope: "O", title: "backbone-worker offline resilience" }, region: "🔖️OfflineResilienceTests" },
  { key: "documentBackboneBatches", dir: join(OS_TESTS, ns("📨", "document-backbone-batches")), fn: "registerDocumentBackboneBatchTests", emoji: "📨", doc: "Document backbone batches: grant actor isolation, retained bytes, refused and transient batches and rebootstrap fencing.", flavor: "worker", items: ["O39", "O40", "O41", "O42", "O43", "O44", "O45"], wrapper: { scope: "O", title: "backbone-worker offline resilience" }, region: "🔖️OfflineResilienceTests" },
  { key: "folderWatch", dir: join(OS_TESTS, ns("👀", "folder-watch")), fn: "registerFolderWatchTests", emoji: "👀", doc: "Stream-primary folder watch: the coalesced poll, its suppression while the stream is open, one stream per folder and abort on close.", flavor: "worker", items: ["O47", "O48", "O49", "O50"], wrapper: { scope: "O", title: "backbone-worker offline resilience" }, region: "🔖️OfflineResilienceTests" },
  { key: "hubReconnectOutbox", dir: join(OS_TESTS, ns("📴", "hub-reconnect-outbox")), fn: "registerHubReconnectOutboxTests", emoji: "📴", doc: "The bounded lossless mutation outbox and the hub reconnect backoff: offline queueing, flush on Welcome and abort of the reconnect loop.", flavor: "worker", items: ["O51", "O52", "O53", "O54", "O55", "O56"], wrapper: { scope: "O", title: "backbone-worker offline resilience" }, region: "🔖️OfflineResilienceTests" },
  { key: "scopeSafePresence", dir: join(OS_TESTS, ns("👥", "scope-safe-presence")), fn: "registerScopeSafePresenceTests", emoji: "👥", doc: "Scope-safe presence: verified surface authority binds only after the exact socket Session and equal document ids stay isolated.", flavor: "worker", items: ["B72"] },
  { key: "checkpointPairDecoder", dir: join(OS_TESTS, ns("📍", "checkpoint-pair-decoder")), fn: "registerCheckpointPairDecoderTests", emoji: "📍", doc: "The canonical checkpoint pair as the worker decodes it: the hub's own framing, every refusal and the genesis applied receipt.", flavor: "worker", items: ["B73"] },
  { key: "documentCatchUp", dir: join(OS_TESTS, ns("🪜", "document-catch-up")), fn: "registerDocumentCatchUpTests", emoji: "🪜", doc: "Document catch-up: a remote batch before the child exists, remote operations folded over pending local ones and the rebuild welcome.", flavor: "worker", items: ["B74", "B75", "B76"] },
  { key: "actorCommandWindows", dir: join(OS_TESTS, ns("🧭", "actor-command-windows")), fn: "registerActorCommandWindowTests", emoji: "🧭", doc: "Browser actor command windows: an app command is admitted in every window kind of the verified app.", flavor: "worker", items: ["B77"] },
  { key: "inferencePort", dir: join(GIS_TESTS_DIR, ns("💡", "inference-port")), fn: "registerGisInferencePortTests", emoji: "💡", doc: "The host-owned GIS Map inference port driven through a fake authenticated transport, and the retained inference job across a refused successor opening.", flavor: "gis", items: ["B4", "B69", "B70"], region: "💡️InferencePortTests" },
  { key: "gisDocumentOpen", dir: join(GIS_TESTS_DIR, ns("🗺", "document-open")), fn: "registerGisDocumentOpenTests", emoji: "🗺", doc: "GIS document open: the GIS wasm lease, the viewer's renderer-unavailable state and the cold pair transfer with inference approval Undo.", flavor: "gis", items: ["O29", "O35", "O36"], wrapper: { scope: "O", title: "backbone-worker offline resilience" }, region: "🔖️OfflineResilienceTests" },
  { key: "peerRebootstrap", dir: join(GIS_TESTS_DIR, ns("🔁", "peer-rebootstrap")), fn: "registerGisPeerRebootstrapTests", emoji: "🔁", doc: "GIS browser document peers refetch the same exact pair after a scoped rebootstrap.", flavor: "gis", items: ["A6"], wrapper: { scope: "A", title: "artifact bootstrap atomic restore" }, region: "🧪️ArtifactBootstrapRestore" },
];

const SUPPORT_DIR = join(OS_TESTS, ns("🧰", "worker-law-fakes"));
const SUPPORT_FILE = join(SUPPORT_DIR, FILE);
const GIS_TYPES_DIR = join(GIS_TESTS_DIR, ns("🧰", "dependencies"));
const GIS_TYPES_FILE = join(GIS_TYPES_DIR, FILE);
const AGGREGATOR_DIR = join(OS_TESTS, ns("📚", "worker-law-suites"));
const AGGREGATOR_FILE = join(AGGREGATOR_DIR, FILE);

const rebase = (spec: string, fromDir: string, toDir: string): string => {
  if (!spec.startsWith(".")) return spec;
  const target = resolve(fromDir, spec);
  let rel = relative(toDir, target);
  if (!rel.startsWith(".")) rel = "./" + rel;
  return rel;
};
const importPath = (absoluteTarget: string, toDir: string): string => {
  let rel = relative(toDir, absoluteTarget);
  if (!rel.startsWith(".")) rel = "./" + rel;
  return rel;
};

const lineStartOf = (pos: number) => text.lastIndexOf("\n", pos - 1) + 1;
const REGION = /^\s*\/\/\s*#(end)?region/u;
type Replacement = { start: number; end: number; value: string };

const itemSource = (item: Item, toDir: string, options: { readonly export?: boolean; readonly dedent?: number; readonly indent?: number; readonly resetFirst?: boolean; readonly hookInside?: boolean } = {}): string => {
  const node = nodeOf(item);
  const start = node.getStart(sf);
  const comments = (ts.getLeadingCommentRanges(text, node.getFullStart()) ?? []).filter((range) => !REGION.test(text.slice(range.pos, range.end)));
  const indent = text.slice(lineStartOf(start), start);
  const pieces: Replacement[] = [];
  const rebaseLiteral = (literal: ts.StringLiteralLike) => {
    const value = literal.text;
    if (!value.startsWith(".")) return;
    pieces.push({ start: literal.getStart(sf) + 1, end: literal.end - 1, value: rebase(value, GIS_TESTS_DIR, toDir) });
  };
  const walk = (current: ts.Node) => {
    if (ts.isCallExpression(current) && current.expression.kind === ts.SyntaxKind.ImportKeyword && current.arguments[0] && ts.isStringLiteralLike(current.arguments[0])) rebaseLiteral(current.arguments[0]);
    if (ts.isImportTypeNode(current) && ts.isLiteralTypeNode(current.argument) && ts.isStringLiteralLike(current.argument.literal)) rebaseLiteral(current.argument.literal);
    if (ts.isNewExpression(current) && current.expression.getText(sf) === "URL" && current.arguments?.[1]?.getText(sf) === "import.meta.url" && current.arguments[0] && ts.isStringLiteralLike(current.arguments[0])) rebaseLiteral(current.arguments[0]);
    current.forEachChild(walk);
  };
  walk(node);
  const bodyStart = start;
  let body = text.slice(bodyStart, node.end);
  pieces.sort((a, b) => b.start - a.start);
  for (const piece of pieces) body = body.slice(0, piece.start - bodyStart) + piece.value + body.slice(piece.end - bodyStart);
  if (options.export) body = "export " + body;
  if (options.hookInside) {
    const callback = callbackBody(node as ts.Statement);
    const offset = callback.getStart(sf) + 1 - bodyStart;
    body = body.slice(0, offset) + "\n" + indent + "  beforeEach(resetWorkerLaw);" + body.slice(offset);
  }
  if (options.resetFirst) {
    const callback = callbackBody(node as ts.Statement);
    const offset = callback.getStart(sf) + 1 - bodyStart;
    body = body.slice(0, offset) + "\n" + indent + "  resetWorkerLaw();" + body.slice(offset);
  }
  const lead = comments.map((range) => indent + text.slice(range.pos, range.end)).join("\n");
  let result = (lead ? lead + "\n" : "") + indent + body;
  if (options.indent) result = result.split("\n").map((line) => (line.length ? " ".repeat(options.indent!) + line : line)).join("\n");
  if (options.dedent) result = result.split("\n").map((line) => (line.startsWith(" ".repeat(options.dedent!)) ? line.slice(options.dedent!) : line)).join("\n");
  return result;
};

const infoOf = (id: string) => infos.get(id)!;
const groupItems = (group: Group) => group.items.map(infoOf);

const transitiveUses = (roots: readonly Info[]): Set<string> => {
  const out = new Set<string>();
  const visit = (owner: Info) => {
    if (out.has(owner.id)) return;
    out.add(owner.id);
    for (const next of refsOf(owner)) visit(next);
  };
  for (const root of roots) for (const owner of refsOf(root)) visit(owner);
  return out;
};

const usedBy = new Map<string, Set<string>>();
const groupDirect = new Map<string, Set<string>>();
for (const group of groups) {
  const direct = new Set<string>();
  for (const info of groupItems(group)) for (const name of info.used) direct.add(name);
  if (group.flavor === "worker" || group.flavor === "gis") direct.add("beforeEach");
  groupDirect.set(group.key, direct);
  const ids = transitiveUses(groupItems(group));
  for (const id of ids) {
    if (group.items.includes(id)) continue;
    if (!usedBy.has(id)) usedBy.set(id, new Set());
    usedBy.get(id)!.add(group.key);
  }
}
const resetUses = new Set<string>();
identifiersOf(callbackBody(B[49]!), resetUses);
const supportIds = new Set<string>();
for (const [id, keys] of usedBy) if (keys.size >= 2 && !reImportType(infoOf(id))) supportIds.add(id);
const closeSupport = (owner: Info | undefined) => {
  if (!owner || reImportType(owner) || supportIds.has(owner.id)) return;
  supportIds.add(owner.id);
  for (const next of refsOf(owner)) closeSupport(next);
};
for (const id of [...supportIds]) for (const next of refsOf(infoOf(id))) closeSupport(next);
for (const name of resetUses) closeSupport(resolveOwner(name, "B"));
for (const name of ["WORKER_LAW_CAPABILITY", "WORKER_LAW_CAPABILITY_TWO"]) closeSupport(resolveOwner(name, "B"));

const FACTORY_SEEDS = new Set<string>([...depNames, ...VITEST_MEMBERS, "source", "vitest", "dependencies", "resetWorkerLaw"]);
const pureCache = new Map<string, boolean>();
const isPure = (id: string): boolean => {
  if (pureCache.has(id)) return pureCache.get(id)!;
  pureCache.set(id, true);
  const info = infoOf(id);
  let pure = info.kind === "helper" || info.kind === "type";
  const node = nodeOf(info.item);
  const locals = new Set<string>();
  node.forEachChild(function collect(child) {
    if (ts.isParameter(child) && ts.isIdentifier(child.name)) locals.add(child.name.text);
    if (ts.isVariableDeclaration(child) && ts.isIdentifier(child.name)) locals.add(child.name.text);
    child.forEachChild(collect);
  });
  for (const used of info.used) {
    if (info.names.includes(used)) continue;
    if (FACTORY_SEEDS.has(used) && !locals.has(used)) pure = false;
  }
  for (const owner of refsOf(info)) if (owner.id !== id && !isPure(owner.id)) pure = false;
  pureCache.set(id, pure);
  return pure;
};

const supportOrdered = [...supportIds].sort((a, b) => infoOf(a).line - infoOf(b).line);
const supportTypes = supportOrdered.filter((id) => infoOf(id).kind === "type");
const supportPure = supportOrdered.filter((id) => infoOf(id).kind === "helper" && isPure(id));
const supportFactory = supportOrdered.filter((id) => infoOf(id).kind === "helper" && !isPure(id));

type Needs = { readonly vitestMembers: string[]; readonly deps: string[]; readonly factoryNames: string[]; readonly pureNames: string[]; readonly supportTypeNames: string[]; readonly imports: ModuleImport[]; readonly aliasIds: string[]; readonly inlineIds: string[]; readonly usesSource: boolean; readonly usesDependenciesParam: boolean };
const factoryNameSet = new Set(supportFactory.flatMap((id) => infoOf(id).names));
const pureNameSet = new Set(supportPure.flatMap((id) => infoOf(id).names));
const supportTypeNameSet = new Set(supportTypes.flatMap((id) => infoOf(id).names));

const needsOf = (group: Group): Needs => {
  const vitestMembers = new Set<string>();
  const deps = new Set<string>();
  const factory = new Set<string>();
  const pure = new Set<string>();
  const supportTypeNames = new Set<string>();
  const imports = new Map<string, ModuleImport>();
  const aliasIds = new Set<string>();
  const inlineIds = new Set<string>();
  const names = new Set<string>(["beforeEach"]);
  const visitedIds = new Set<string>();
  const queue: Info[] = [...groupItems(group)];
  while (queue.length) {
    const current = queue.shift()!;
    if (visitedIds.has(current.id)) continue;
    visitedIds.add(current.id);
    for (const name of current.used) names.add(name);
    for (const owner of refsOf(current)) {
      if (supportIds.has(owner.id)) {
        for (const name of owner.names) {
          if (factoryNameSet.has(name)) factory.add(name);
          if (pureNameSet.has(name)) pure.add(name);
          if (supportTypeNameSet.has(name)) supportTypeNames.add(name);
        }
        continue;
      }
      if (group.items.includes(owner.id)) continue;
      if (reImportType(owner)) {
        aliasIds.add(owner.id);
        continue;
      }
      inlineIds.add(owner.id);
      queue.push(owner);
    }
  }
  for (const name of names) {
    if (VITEST_MEMBERS.has(name)) vitestMembers.add(name);
    if (depNames.has(name)) deps.add(name);
    if (moduleImports.has(name)) imports.set(name, moduleImports.get(name)!);
  }
  const usesSource = names.has("source");
  return { vitestMembers: [...vitestMembers], deps: [...deps], factoryNames: [...factory], pureNames: [...pure], supportTypeNames: [...supportTypeNames], imports: [...imports.values()], aliasIds: [...aliasIds], inlineIds: [...inlineIds], usesSource, usesDependenciesParam: names.has("dependencies") };
};

const docstring = (emoji: string, body: string) => `/** ${emoji}️ ${body} */`;
const indentLines = (value: string, pad: string) => value.split("\n").map((line) => (line.length ? pad + line : line)).join("\n");
const sortedByLine = (ids: Iterable<string>) => [...ids].sort((a, b) => infoOf(a).line - infoOf(b).line);

const outputs = new Map<string, string>();
const importLine = (names: readonly string[], spec: string, typeOnly: boolean) => `import ${typeOnly ? "type " : ""}{ ${names.join(", ")} } from ${JSON.stringify(spec)};`;

const buildGroup = (group: Group): string => {
  const dir = group.dir;
  const needs = needsOf(group);
  const needsUse = new Set<string>();
  for (const info of groupItems(group)) for (const name of info.used) needsUse.add(name);
  for (const id of needs.inlineIds) for (const name of infoOf(id).used) needsUse.add(name);
  const lines: string[] = [];
  lines.push(docstring(group.docEmoji ?? group.emoji, group.doc));
  const valueImports = new Map<string, string[]>();
  const typeImports = new Map<string, string[]>();
  for (const entry of needs.imports) {
    const spec = rebase(entry.spec, GIS_TESTS_DIR, dir);
    const target = entry.typeOnly ? typeImports : valueImports;
    if (!target.has(spec)) target.set(spec, []);
    target.get(spec)!.push(entry.name);
  }
  const workerType = join(OS_MODULES, ns("🏪", "store"), ns("👷", "worker"), FILE);
  const supportNames = [...needs.factoryNames, ...needs.pureNames];
  if (group.flavor === "worker") typeImports.set(importPath(workerType, dir), [...(typeImports.get(importPath(workerType, dir)) ?? []), "BackboneWorkerTestDependencies"]);
  const supportImportPath = importPath(SUPPORT_FILE, dir);
  const supportValueNames = [...new Set(["workerLawFakesV1", ...needs.pureNames])];
  const supportTypeNames = [...new Set(["TestSource", ...needs.supportTypeNames])];
  for (const [spec, names] of valueImports) lines.push(importLine(names, spec, false));
  for (const [spec, names] of typeImports) lines.push(importLine(names, spec, true));
  lines.push(importLine(supportValueNames, supportImportPath, false));
  lines.push(importLine(supportTypeNames, supportImportPath, true));
  if (group.flavor === "gis") lines.push(importLine(["ServiceCompositionTestDependenciesV1", ...(needsUse.has("InferenceOperationV1") ? ["InferenceOperationV1"] : [])], importPath(GIS_TYPES_FILE, dir), true));
  lines.push("");
  for (const id of sortedByLine(needs.aliasIds)) lines.push(itemSource(infoOf(id).item, dir, { dedent: 2 }));
  for (const id of sortedByLine(needs.inlineIds)) if (infoOf(id).kind === "type") lines.push(itemSource(infoOf(id).item, dir, { dedent: infoOf(id).item.scope === "B" ? 2 : 4 }));
  lines.push("type Vitest = NonNullable<ImportMeta[\"vitest\"]>;");
  lines.push("");
  const dependenciesType = group.flavor === "gis" ? "ServiceCompositionTestDependenciesV1" : "BackboneWorkerTestDependencies";
  lines.push(`export async function ${group.fn}(vitest: Vitest, dependencies: ${dependenciesType}, source: TestSource): Promise<void> {`);
  const members = new Set(needs.vitestMembers);
  members.add("beforeEach");
  if (group.wrapper) members.add("describe");
  const wantedMembers = ["beforeEach", "describe", "expect", "it", "vi"].filter((name) => members.has(name));
  lines.push(`  const { ${wantedMembers.join(", ")} } = vitest;`);
  lines.push(`  const { resetWorkerLaw${needs.factoryNames.length ? ", " + needs.factoryNames.join(", ") : ""} } = workerLawFakesV1(vitest, dependencies, source);`);
  const depList = needs.deps.filter((name) => name !== "testSeams");
  if (needs.deps.includes("testSeams")) depList.unshift("testSeams");
  if (depList.length) lines.push(`  const { ${depList.join(", ")} } = dependencies;`);
  const inlineHelpers = sortedByLine(needs.inlineIds).filter((id) => infoOf(id).kind === "helper");
  const body: string[] = [];
  const itemIsBLevel = (id: string) => infoOf(id).item.scope === "B";
  if (group.wrapper) {
    const note = group.key === "folderWatch" || group.key === "hubReconnectOutbox" ? text.split("\n").slice(3610, 3614).join("\n") + "\n" : "";
    const wrapperHead = `${note}  describe(${JSON.stringify(group.wrapper.title)}, () => {`;
    const innerHelpers = inlineHelpers.map((id) => itemSource(infoOf(id).item, dir));
    const outerHelpers = innerHelpers;
    void outerHelpers;
    const bodyParts: string[] = ["    beforeEach(resetWorkerLaw);"];
    for (const id of inlineHelpers) bodyParts.push(itemSource(infoOf(id).item, dir, itemIsBLevel(id) ? { indent: 2 } : {}));
    for (const id of group.items) bodyParts.push(itemSource(infoOf(id).item, dir));
    body.push(wrapperHead, bodyParts.join("\n\n"), "  });");
  } else {
    for (const id of inlineHelpers) body.push(itemSource(infoOf(id).item, dir));
    for (const id of group.items) {
      const info = infoOf(id);
      if (info.kind !== "test") {
        body.push(itemSource(info.item, dir));
        continue;
      }
      const node = nodeOf(info.item) as ts.ExpressionStatement;
      const head = (node.expression as ts.CallExpression).expression.getText(sf);
      if (head === "describe") body.push(itemSource(info.item, dir, { hookInside: true }));
      else body.push(itemSource(info.item, dir, { resetFirst: true }));
    }
  }
  const content = body.join("\n\n");
  if (group.region) lines.push(`  //#region ${group.region}`, content, `  //#endregion ${group.region}`);
  else lines.push(content);
  lines.push("}");
  lines.push("");
  void supportNames;
  return lines.join("\n");
};

const NEUTRAL: readonly (readonly [string, string])[] = [
  ["s.gis.gismap", "s.demo.demo"],
  ['"GIS Map"', '"Demo Map"'],
  ['"GIS-Karte"', '"Demo-Karte"'],
  ["gis.map.operation", "demo.map.operation"],
  ["s.cad.cad", "s.demo.demo"],
  ['pluginId: "cad"', 'pluginId: "demo"'],
  ["s.note.note@1", "s.demo.demo@1"],
  ["noteShellCommand", "demoShellCommand"],
  ['"semio:gis"', '"semio:demo"'],
];
const FIXTURES = ns("🧫", "fixtures");
const TESTS_SLASH = ns("🧪", "tests") + "/";
const MODULES = ns("🔨", "modules");
const DIRECTORY = ns("📇", "directory");
const SCHEMA = ns("🧬", "schema");
const ADMINISTRATION = ns("🏛", "administration");
const fixtureMoves: readonly (readonly [string, string])[] = [
  [`const base = "./${MODULES}/${DIRECTORY}/${SCHEMA}/";`, `const base = "./${MODULES}/${DIRECTORY}/${TESTS_SLASH}${FIXTURES}/";`],
  [`base + "${ADMINISTRATION}/${FIXTURES}/`, `base + "${ADMINISTRATION}/`],
  [`const base = "./${MODULES}/${DIRECTORY}/${SCHEMA}/${ADMINISTRATION}/${FIXTURES}/`, `const base = "./${MODULES}/${DIRECTORY}/${TESTS_SLASH}${FIXTURES}/${ADMINISTRATION}/`],
  ['profile: "wasm-release"', 'profile: "release"'],
  [
    '      expect(identityActorConfig("actor-1", "/tmp/s-user1")).toEqual({\n        documentId: IDENTITY_CONFIG_SCHEMA,\n        schema: IDENTITY_CONFIG_SCHEMA,\n        bindings: [{ kind: "folder", dataClass: "persistedLocalOnly", path: "/tmp/s-user1/os" }],\n        actor: "actor-1",\n      });\n      expect(identityActorConfig("actor-1")).toEqual({ documentId: IDENTITY_CONFIG_SCHEMA, schema: IDENTITY_CONFIG_SCHEMA, bindings: [], actor: "actor-1" });',
    '      const actorIdentityGrant = { maximumItems: 1024, maximumCopyBytes: 65536, maximumCapacityBytes: 65536, maximumReleaseBytes: 65536, maximumDepth: 64 };\n      expect(identityActorConfig("actor-1", actorIdentityGrant, "/tmp/s-user1")).toEqual({\n        documentId: IDENTITY_CONFIG_SCHEMA,\n        schema: IDENTITY_CONFIG_SCHEMA,\n        bindings: [{ kind: "folder", dataClass: "persistedLocalOnly", path: "/tmp/s-user1/os" }],\n        actor: "actor-1",\n        actorIdentityGrant,\n      });\n      expect(identityActorConfig("actor-1", actorIdentityGrant)).toEqual({ documentId: IDENTITY_CONFIG_SCHEMA, schema: IDENTITY_CONFIG_SCHEMA, bindings: [], actor: "actor-1", actorIdentityGrant });',
  ],
];
const moveCounts = new Map<string, number>();
const repointFixtures = (value: string): string => {
  let result = value;
  for (const [from, to] of fixtureMoves) {
    const pieces = result.split(from);
    moveCounts.set(from, (moveCounts.get(from) ?? 0) + pieces.length - 1);
    result = pieces.join(to);
  }
  const parts = result.split(FIXTURES);
  let out = parts[0]!;
  for (let index = 1; index < parts.length; index += 1) out += (out.endsWith(TESTS_SLASH) ? "" : TESTS_SLASH) + FIXTURES + parts[index];
  return out;
};
const neutralize = (value: string): string => NEUTRAL.reduce((acc, [from, to]) => acc.split(from).join(to), value);
for (const group of groups) outputs.set(join(group.dir, FILE), repointFixtures(group.flavor === "worker" ? neutralize(buildGroup(group)) : buildGroup(group)));

const buildSupport = (): string => {
  const dir = SUPPORT_DIR;
  const lines: string[] = [];
  lines.push(docstring("🧰", "Shared fakes and fixtures of the worker-law suites: the fake hub socket and stream endpoint, the verified cold pair installer, the document-open and execution-target fixtures and the common reset hook. One memoized instance per dependency bag."));
  const importedValues = new Map<string, string[]>();
  const importedTypes = new Map<string, string[]>();
  const usedNames = new Set<string>();
  for (const id of supportOrdered) for (const name of infoOf(id).used) usedNames.add(name);
  for (const name of resetUses) usedNames.add(name);
  for (const name of usedNames) {
    const entry = moduleImports.get(name);
    if (!entry) continue;
    const spec = rebase(entry.spec, GIS_TESTS_DIR, dir);
    const target = entry.typeOnly ? importedTypes : importedValues;
    if (!target.has(spec)) target.set(spec, []);
    target.get(spec)!.push(name);
  }
  const workerType = join(OS_MODULES, ns("🏪", "store"), ns("👷", "worker"), FILE);
  importedTypes.set(importPath(workerType, dir), [...(importedTypes.get(importPath(workerType, dir)) ?? []), "BackboneWorkerTestDependencies"]);
  for (const [spec, names] of importedValues) lines.push(importLine(names, spec, false));
  for (const [spec, names] of importedTypes) lines.push(importLine(names, spec, true));
  lines.push("");
  lines.push("export type TestSource = { readonly directory: string; readonly url: string };");
  lines.push("type Vitest = NonNullable<ImportMeta[\"vitest\"]>;");
  const aliasNeeded = new Set<string>();
  for (const id of supportOrdered) for (const owner of refsOf(infoOf(id))) if (reImportType(owner)) aliasNeeded.add(owner.id);
  for (const name of resetUses) { const owner = resolveOwner(name, "B"); if (owner && reImportType(owner)) aliasNeeded.add(owner.id); }
  for (const id of sortedByLine(aliasNeeded)) lines.push(itemSource(infoOf(id).item, dir, { dedent: 2 }));
  lines.push("");
  for (const id of supportTypes) lines.push(itemSource(infoOf(id).item, dir, { export: true, dedent: infoOf(id).item.scope === "B" ? 2 : 4 }));
  for (const id of supportPure) lines.push(itemSource(infoOf(id).item, dir, { export: true, dedent: infoOf(id).item.scope === "B" ? 2 : 4 }));
  lines.push("");
  const factoryDeps = new Set<string>();
  for (const id of supportFactory) for (const name of infoOf(id).used) if (depNames.has(name)) factoryDeps.add(name);
  for (const name of resetUses) if (depNames.has(name)) factoryDeps.add(name);
  const factoryMembers = new Set<string>();
  for (const id of supportFactory) for (const name of infoOf(id).used) if (VITEST_MEMBERS.has(name)) factoryMembers.add(name);
  for (const name of resetUses) if (VITEST_MEMBERS.has(name)) factoryMembers.add(name);
  const returned = supportFactory.flatMap((id) => infoOf(id).names);
  lines.push("const memo = new WeakMap<object, unknown>();");
  lines.push("");
  const constNames = ["WORKER_LAW_CAPABILITY", "WORKER_LAW_CAPABILITY_TWO"];
  void constNames;
  lines.push("export function workerLawFakesV1(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource) {");
  lines.push("  const cached = memo.get(dependencies);");
  lines.push("  if (cached) return cached as ReturnType<typeof create>;");
  lines.push("  const created = create(vitest, dependencies, source);");
  lines.push("  memo.set(dependencies, created);");
  lines.push("  return created;");
  lines.push("}");
  lines.push("");
  lines.push("function create(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource) {");
  const members = ["describe", "expect", "it", "vi"].filter((name) => factoryMembers.has(name));
  if (members.length) lines.push(`  const { ${members.join(", ")} } = vitest;`);
  const depList = [...factoryDeps].filter((name) => name !== "testSeams");
  if (factoryDeps.has("testSeams")) depList.unshift("testSeams");
  lines.push(`  const { ${depList.join(", ")} } = dependencies;`);
  for (const id of supportFactory) lines.push(itemSource(infoOf(id).item, dir));
  const resetBody = callbackBody(B[49]!);
  const resetText = text.slice(resetBody.getStart(sf) + 1, resetBody.end - 1);
  lines.push("  function resetWorkerLaw(): void {" + resetText + "}");
  lines.push(`  return { ${["resetWorkerLaw", ...returned].join(", ")} };`);
  lines.push("}");
  lines.push("");
  return lines.join("\n");
};
outputs.set(SUPPORT_FILE, repointFixtures(neutralize(buildSupport())));

const gisTypes = (): string => {
  const lines: string[] = [];
  lines.push(docstring("🏷", "Dependency types of the GIS service-composition suites: the worker test bag extended by the GIS inference worker's own test seams and models."));
  lines.push(importLine(["BackboneWorkerTestDependencies"], importPath(join(OS_MODULES, ns("🏪", "store"), ns("👷", "worker"), FILE), GIS_TYPES_DIR), true));
  lines.push(importLine(["createGisMapInferenceWorkerV1"], rebase(moduleImports.get("createGisMapInferenceWorkerV1")!.spec, GIS_TESTS_DIR, GIS_TYPES_DIR), true));
  lines.push(importLine(["idleGisMapInferencePortStatusV1", "parseGisMapInferenceApprovalReceiptV1"], rebase(moduleImports.get("idleGisMapInferencePortStatusV1")!.spec, GIS_TESTS_DIR, GIS_TYPES_DIR), true).replace("import type {", "import type {").replace(/\{ (.*) \}/u, (_all, names) => `{ ${names} }`));
  lines.push("");
  lines.push("export type OwnerDriverV1 = ReturnType<typeof createGisMapInferenceWorkerV1>;");
  lines.push("export type InferenceOperationV1 = NonNullable<OwnerDriverV1[\"test\"][\"inferencePort\"]>;");
  const declared = (sf.statements.find((s) => ts.isTypeAliasDeclaration(s) && s.name.text === "ServiceCompositionTestDependenciesV1") as ts.TypeAliasDeclaration).getText(sf);
  lines.push("export " + declared.replace("idleGisMapInferencePortStatusV1: typeof idleGisMapInferencePortStatusV1", "idleGisMapInferencePortStatusV1: typeof idleGisMapInferencePortStatusV1"));
  lines.push("");
  return lines.join("\n");
};
outputs.set(GIS_TYPES_FILE, gisTypes());

const workerGroups = groups.filter((group) => group.flavor === "worker" && group.key !== "sessionBrokerPort").concat(groups.filter((group) => group.key === "sessionBrokerPort"));
const aggregatorGroups = [...groups.filter((group) => group.flavor === "worker")].sort((a, b) => infoOf(a.items[0]!).line - infoOf(b.items[0]!).line);
void workerGroups;
const buildAggregator = (): string => {
  const dir = AGGREGATOR_DIR;
  const lines: string[] = [];
  lines.push(docstring("📚", "The worker-law suites: every browser-worker behaviour suite that runs against the neutral worker test bag, registered in their original order from the worker's in-source block."));
  lines.push(importLine(["BackboneWorkerTestDependencies"], importPath(join(OS_MODULES, ns("🏪", "store"), ns("👷", "worker"), FILE), dir), true));
  lines.push(importLine(["TestSource"], importPath(SUPPORT_FILE, dir), true));
  for (const group of aggregatorGroups) lines.push(importLine([group.fn], importPath(join(group.dir, FILE), dir), false));
  lines.push("");
  lines.push("type Vitest = NonNullable<ImportMeta[\"vitest\"]>;");
  lines.push("");
  lines.push("/** 🔭️ Rebases the worker's own test source onto the OS root every fixture path is spelled against. */");
  lines.push("export function workerLawSourceV1(registrar: TestSource): TestSource {");
  lines.push("  return { directory: registrar.directory, url: new URL(\"../../../🟦️.ts\", registrar.url).href };");
  lines.push("}");
  lines.push("");
  lines.push("export async function registerWorkerLawTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, registrar: TestSource): Promise<void> {");
  lines.push("  const source = workerLawSourceV1(registrar);");
  for (const group of aggregatorGroups) lines.push(`  await ${group.fn}(vitest, dependencies, source);`);
  lines.push("}");
  lines.push("");
  return lines.join("\n");
};
outputs.set(AGGREGATOR_FILE, buildAggregator());

const report = {
  supportIds: supportOrdered.map((id) => `${id}:${infoOf(id).names.join("|") || infoOf(id).kind}`),
  supportPure: supportPure.map((id) => infoOf(id).names.join("|")),
  supportFactory: supportFactory.map((id) => infoOf(id).names.join("|")),
  supportTypes: supportTypes.map((id) => infoOf(id).names.join("|")),
  perGroup: groups.map((group) => { const n = needsOf(group); return { key: group.key, inline: n.inlineIds.map((id) => `${id}:${infoOf(id).names.join("|")}`), factory: n.factoryNames, pure: n.pureNames, depsCount: n.deps.length, gisDeps: n.deps.filter((d) => GIS_DEPS.has(d)) }; }),
};
writeFileSync(join(FOLDER, "🗑️generated/met-split-report.json"), JSON.stringify(report, null, 1));
for (const group of groups) {
  const n = needsOf(group);
  if (group.flavor === "worker" && n.deps.some((d) => GIS_DEPS.has(d))) throw new Error(`worker group ${group.key} uses GIS deps ${n.deps.filter((d) => GIS_DEPS.has(d))}`);
  if (group.flavor === "worker") for (const id of [...group.items, ...n.inlineIds]) if (/inferencePort|inferenceApprovalUndo|\bGis|gismap|gisMap|\bs\.gis\b/u.test(nodeOf(infoOf(id).item).getText(sf))) console.warn(`[note] worker group ${group.key} item ${id} mentions gis/inference`);
}
if (DRY) {
  console.log(JSON.stringify(report, null, 1));
} else {
  for (const [file, content] of outputs) {
    mkdirSync(dirname(file), { recursive: true });
    writeFileSync(file, content);
    console.log("wrote", relative(REPO, file), content.split("\n").length);
  }
  console.log("fixture moves", JSON.stringify([...moveCounts]));
  writeFileSync(join(FOLDER, "met-tools/gis-new-files.txt"), [...outputs.keys()].map((file) => relative(REPO, file)).join("\n") + "\n");
}
