// usage: node splitreservation.mjs <orig-actor-reservation.ts> <out-dir>
// Redesigns `class DocumentBrowserActorReservation` into a facade plus collaborators that share one private session object.
import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/node_modules/");
const ts = require("typescript");

const [origPath, outDir] = process.argv.slice(2);
const text = fs.readFileSync(origPath, "utf8");
const sf = ts.createSourceFile("actor-reservation.ts", text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);

const SESSION_FIELDS = ["generation", "scope", "windowKindId", "windowKeys", "surfaceKeys", "abort", "child", "activation", "lifetime", "coldOwner", "coldApplied", "coldTransfer", "socket", "linkSuspended", "pendingUiPatch", "renderedUiPatch", "renderedSurfaceRevisions", "acknowledgedUiRevision", "mountedUiRevision", "renderedViewState", "viewRefresh", "documentBinding", "documentBindingGeneration", "documentBackboneReady", "backboneDraining", "pendingBackboneBeforeBinding", "pendingBackboneBeforeBindingBytes", "pollTail", "turnTail", "localEffectTail", "localEffectFailure", "lastActionSequence", "lastEphemeral", "retirement", "closed", "renderedUiRevision", "lease", "grant", "retire"];
const RENAMED = { state: "artifact" };

const COLLABORATORS = {
  lanes: { dir: "🛤️poll-lanes", cls: "ReservationPollLanes", emoji: "🛤️", doc: "Serializes every guest `poll` and job crossing on the poll lane and every turn on the turn lane, so a closed or replaced child never receives late work.", methods: ["invokeWithPages", "invokeJobs", "driveSpawnedJobs", "invokePoll", "invokeCommandPage", "enqueueTurn"] },
  effects: { dir: "📤️turn-effects", cls: "ReservationTurnEffects", emoji: "📤️", doc: "Routes the WIT effects of one guest turn to the backbone port, the typed-operation and publication decoders and the job driver.", methods: ["routeTurnEffects"] },
  patches: { dir: "🩹️ui-patch-reconciliation", cls: "ReservationUiPatches", emoji: "🩹️", doc: "Offers the guest's UI patches to the main thread, feeds the verdicts back and drives a turn to its settled result.", methods: ["captureUiPatch", "reportExecutionTargetFault", "awaitUiPatchResult", "settleUiPatch", "reconcileUiPatches", "driveTurnResult"] },
  backbone: { dir: "📬️document-backbone", cls: "ReservationDocumentBackbone", emoji: "📬️", doc: "Binds the document backbone port of the child and admits, retains and delivers the inbound backbone messages.", methods: ["assertDocumentOwnerCurrent", "currentDocumentSource", "bindDocumentBackbone", "flushPendingBackbone", "retainBackboneBeforeBinding", "admitInboundBackbone", "receiveBackbone"] },
  coldPair: { dir: "🧊️cold-pair-transfer", cls: "ReservationColdPairTransfer", emoji: "🧊️", doc: "Installs the verified cold document pair into the live child page by page and applies it.", methods: ["installColdPair", "transferColdPair"] },
  activation: { dir: "🚀️guest-activation", cls: "ReservationGuestActivation", emoji: "🚀️", doc: "Reserves the child, activates it on the exact socket Session and opens the guest instance.", methods: ["reserve", "activate", "openGuest"] },
  retirement: { dir: "♻️guest-retirement", cls: "ReservationGuestRetirement", emoji: "♻️", doc: "Closes the reservation and retires the guest instance through the lifecycle close protocol.", methods: ["retireGuest", "close"] },
  surfaces: { dir: "🖼️surface-rendering", cls: "ReservationSurfaceRendering", emoji: "🖼️", doc: "Renders and refreshes the verified windows, panels and sections of the child and publishes the mounted state.", methods: ["publishMountedIfReady", "refreshHostView", "bindApprovalUndoIfMounted", "renderSurface", "visibleEvents", "refreshDocumentSurfaces"] },
  actions: { dir: "🎯️action-dispatch", cls: "ReservationActionDispatch", emoji: "🎯️", doc: "Applies one authenticated action in issue order on the turn lane.", methods: ["awaitUiQuiescence", "dispatchAction"] },
  link: { dir: "🔌️link-suspension", cls: "ReservationLinkSuspension", emoji: "🔌️", doc: "Keeps a mounted child through a short link loss and resumes it on the re-admitted socket.", methods: ["linkCurrent", "suspendLink", "suspended", "adoptGrant", "resumeLink"] },
};
const FACADE_DELEGATES = ["reserve", "activate", "installColdPair", "refreshHostView", "bindApprovalUndoIfMounted", "receiveBackbone", "dispatchAction", "settleUiPatch", "suspendLink", "adoptGrant", "resumeLink", "close"];
const FACADE_GETTER_DELEGATES = ["suspended"];
const FACADE_KEPT = ["retirementOutcome", "ephemeralSnapshot"];

const ownerOf = new Map();
for (const [key, spec] of Object.entries(COLLABORATORS)) for (const m of spec.methods) ownerOf.set(m, key);

const classNode = sf.statements.find(s => ts.isClassDeclaration(s) && s.name?.text === "DocumentBrowserActorReservation");
const members = new Map();
for (const member of classNode.members) {
  if ((ts.isMethodDeclaration(member) || ts.isGetAccessorDeclaration(member)) && member.name) members.set(member.name.getText(sf), member);
}
for (const spec of Object.values(COLLABORATORS)) for (const m of spec.methods) if (!members.has(m)) throw new Error("missing method " + m);
for (const m of [...FACADE_DELEGATES, ...FACADE_KEPT, ...FACADE_GETTER_DELEGATES, "renderedUiRevision"]) if (!members.has(m)) throw new Error("missing facade member " + m);

const crossCalled = new Set();
const collectCross = (node, current) => {
  const visit = n => {
    if (ts.isPropertyAccessExpression(n) && n.expression.kind === ts.SyntaxKind.ThisKeyword) {
      const name = n.name.text, owner = ownerOf.get(name);
      if (owner !== undefined && owner !== current) crossCalled.add(name);
    }
    ts.forEachChild(n, visit);
  };
  visit(node);
};
for (const [key, spec] of Object.entries(COLLABORATORS)) for (const m of spec.methods) collectCross(members.get(m), key);
for (const m of [...FACADE_DELEGATES, ...FACADE_GETTER_DELEGATES]) crossCalled.add(m);

const leadingTrivia = member => member.getFullText(sf).slice(0, member.getStart(sf) - member.getFullStart());
const docOf = member => {
  const match = /\/\*\*[\s\S]*?\*\/\s*$/u.exec(leadingTrivia(member));
  return match ? match[0].trimEnd() : "";
};

function transformMember(member, current, mode) {
  const start = member.getStart(sf), end = member.getEnd();
  const edits = [];
  const sessionPath = mode === "facade" ? "this.hub.session" : mode === "session" ? "this" : "this.session";
  const visit = n => {
    if (ts.isPropertyAccessExpression(n) && n.expression.kind === ts.SyntaxKind.ThisKeyword) {
      const name = n.name.text;
      let replacement = null;
      if (name in RENAMED) replacement = `${sessionPath}.${RENAMED[name]}`;
      else if (SESSION_FIELDS.includes(name)) replacement = `${sessionPath}.${name}`;
      else if (ownerOf.has(name) && (mode === "facade" ? !FACADE_DELEGATES.includes(name) && !FACADE_GETTER_DELEGATES.includes(name) : ownerOf.get(name) !== current)) replacement = `this.hub.${ownerOf.get(name)}.${name}`;
      if (replacement !== null) { edits.push({ pos: n.getStart(sf), end: n.getEnd(), text: replacement }); return; }
    } else if (n.kind === ts.SyntaxKind.ThisKeyword && !(n.parent && ts.isPropertyAccessExpression(n.parent) && n.parent.expression === n)) {
      edits.push({ pos: n.getStart(sf), end: n.getEnd(), text: mode === "facade" ? "this" : "this.hub.shell" });
    }
    ts.forEachChild(n, visit);
  };
  visit(member);
  edits.sort((a, b) => b.pos - a.pos);
  let out = text.slice(start, end);
  for (const edit of edits) out = out.slice(0, edit.pos - start) + edit.text + out.slice(edit.end - start);
  out = out.replace(/^(\s*)private (async )?/u, (m, indent, asyncKw) => crossCalled.has(member.name.getText(sf)) ? `${indent}${asyncKw ?? ""}` : m);
  const doc = docOf(member);
  return (doc ? "  " + doc + "\n" : "") + "  " + out;
}

const importDecls = [];
for (const statement of sf.statements) {
  if (!ts.isImportDeclaration(statement) || !statement.importClause?.namedBindings || !ts.isNamedImports(statement.importClause.namedBindings)) continue;
  const specifier = statement.moduleSpecifier.text, typeOnly = statement.importClause.isTypeOnly;
  importDecls.push({ specifier, names: statement.importClause.namedBindings.elements.map(e => ({ name: e.name.text, type: typeOnly || e.isTypeOnly })) });
}
const depth = relative => relative.startsWith("./") ? "../" + relative.slice(2) : relative.startsWith("../") ? "../" + relative : relative;

const usageIn = code => {
  const used = new Map();
  const file = ts.createSourceFile("x.ts", code, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const inTypePosition = n => { for (let p = n.parent; p; p = p.parent) { if (ts.isTypeNode(p) && !ts.isExpressionWithTypeArguments(p)) return true; if (ts.isStatement(p) || ts.isClassElement(p) || ts.isExpression(p) && !ts.isTypeNode(p) && false) return false; } return false; };
  const visit = n => {
    if (ts.isIdentifier(n)) {
      const p = n.parent;
      const isPropertyName = (ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isMethodDeclaration(p) && p.name === n) || (ts.isPropertyDeclaration(p) && p.name === n) || (ts.isPropertySignature(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n) || ts.isExportSpecifier(p);
      if (!isPropertyName) { const entry = used.get(n.text) ?? { value: false }; if (!inTypePosition(n)) entry.value = true; used.set(n.text, entry); }
    }
    ts.forEachChild(n, visit);
  };
  visit(file);
  return used;
};
const identifiersIn = code => new Set(usageIn(code).keys());

function importsFor(code, fromLeaf, extra = []) {
  const usage = usageIn(code);
  const lines = [];
  for (const decl of importDecls) {
    const hit = decl.names.filter(n => usage.has(n.name)).map(n => ({ ...n, type: n.type || !usage.get(n.name).value }));
    if (hit.length === 0) continue;
    const specifier = fromLeaf ? decl.specifier : depth(decl.specifier);
    const allType = hit.every(n => n.type);
    lines.push(`import ${allType ? "type " : ""}{ ${hit.map(n => (n.type && !allType ? "type " : "") + n.name).join(", ")} } from "${specifier}";`);
  }
  return [...extra, ...lines].join("\n");
}

const write = (dir, content) => {
  const target = path.join(outDir, dir, "🟦️.ts");
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, content);
  return content.split("\n").length;
};
const sizes = {};

for (const [key, spec] of Object.entries(COLLABORATORS)) {
  const body = spec.methods.map(m => transformMember(members.get(m), key, "collab")).join("\n\n");
  const classCode = `
/** ${spec.emoji} ${spec.doc} */
export class ${spec.cls} {
  private readonly session: ReservationSession;
  constructor(${body.includes("this.hub") ? "private readonly hub" : "hub"}: ReservationHub) { this.session = hub.session; }

${body}
}
`;
  const extra = [`import type { ReservationSession } from "../🗃️state/🟦️.ts";`, `import type { ReservationHub } from "../🟦️.ts";`];
  sizes[spec.dir] = write(spec.dir, importsFor(classCode, false, extra) + "\n" + classCode);
}

const field = name => classNode.members.find(m => m.name?.getText(sf) === name);
const fieldDecl = name => { const f = field(name); const doc = docOf(f); return (doc ? "  " + doc + "\n" : "") + "  " + f.getText(sf).replace(/^private /u, ""); };
const sessionFieldNames = ["scope", "windowKindId", "windowKeys", "surfaceKeys", "abort", "child", "activation", "lifetime", "coldOwner", "coldApplied", "coldTransfer", "socket", "linkSuspended", "pendingUiPatch", "renderedUiPatch", "renderedSurfaceRevisions", "acknowledgedUiRevision", "mountedUiRevision", "renderedViewState", "viewRefresh", "documentBinding", "documentBindingGeneration", "documentBackboneReady", "backboneDraining", "pendingBackboneBeforeBinding", "pendingBackboneBeforeBindingBytes", "pollTail", "turnTail", "localEffectTail", "localEffectFailure", "lastActionSequence", "lastEphemeral", "retirement", "closed"];
const ctor = classNode.members.find(ts.isConstructorDeclaration);
const stmts = ctor.body.statements;
const idx = (needle) => stmts.findIndex(st => st.getText(sf).includes(needle));
const fieldsStmt = stmts[idx("const fields = lease.fields();")].getText(sf);
const scopeStmts = ["this.scope =", "this.windowKindId =", "this.windowKeys =", "this.surfaceKeys =", "this.generation ="].map(n => stmts[idx(n)].getText(sf));
const renderedGetter = members.get("renderedUiRevision");
const sessionCode = `
/** 🗃️ The single private session object of one {@link DocumentBrowserActorReservation}: the artifact state, the lease and grant it answers to and every mutable lane, ledger and flag of the child; the facade and its collaborators share it instead of holding many private fields. */
export class ReservationSession {
  readonly generation: bigint;
${sessionFieldNames.map(fieldDecl).join("\n")}
  constructor(readonly artifact: ArtifactState, readonly lease: DocumentExecutionTargetLease, public grant: DocumentBrowserActorGrant, readonly retire: () => void, nextGeneration: () => bigint) {
    const fields = lease.fields();
    ${scopeStmts.map(s => s.replace(/^this\.generation = \+\+documentBrowserActorGeneration;/u, "this.generation = nextGeneration();").replace(/lease\.renderSurfaces\(\)/gu, "lease.renderSurfaces()")).join("\n    ")}
  }

${transformMember(renderedGetter, "session", "session").replace(/this\.renderedSurfaceRevisions/gu, "this.renderedSurfaceRevisions").replace(/^  private get/mu, "  get")}
}
`;
sizes["🗃️state"] = write("🗃️state", importsFor(sessionCode, false) + "\n" + sessionCode);

const hubEntries = Object.entries(COLLABORATORS);
const hubCode = `/** 🧩️ Wires the collaborators of one {@link DocumentBrowserActorReservation} around its shell and its single private session; collaborators reach each other only through this hub, and only at call time, never while they are constructed. */
export class ReservationHub {
${hubEntries.map(([k, s]) => `  readonly ${k}: ${s.cls};`).join("\n")}
  constructor(readonly shell: DocumentBrowserActorReservation, readonly session: ReservationSession) {
${hubEntries.map(([k, s]) => `    this.${k} = new ${s.cls}(this);`).join("\n")}
  }
}
`;
const hubImports = hubEntries.map(([, s]) => `import { ${s.cls} } from "./${s.dir}/🟦️.ts";`);

const delegateCode = name => {
  const m = members.get(name);
  const params = m.parameters.map(p => `${p.name.getText(sf)}${p.questionToken || p.initializer ? "?" : ""}: ${p.type ? p.type.getText(sf) : "unknown"}`).join(", ");
  const args = m.parameters.map(p => p.name.getText(sf)).join(", ");
  const owner = ownerOf.get(name);
  const ret = m.type ? ": " + m.type.getText(sf) : "";
  return `  ${name}(${params})${ret} { return this.hub.${owner}.${name}(${args}); }`;
};
const getterDelegate = name => { const m = members.get(name); return `  get ${name}()${m.type ? ": " + m.type.getText(sf) : ""} { return this.hub.${ownerOf.get(name)}.${name}; }`; };
const kept = FACADE_KEPT.map(n => transformMember(members.get(n), "facade", "facade"));
const generationExhaustion = stmts[idx("documentBrowserActorGeneration === 0xffff")].getText(sf);
const abortBlock = stmts.slice(idx("this.generation =") + 1).map(st => st.getText(sf));
const commentBlock = (() => {
  const gen = stmts[idx("this.generation =")];
  const trivia = text.slice(gen.getEnd(), stmts[idx("this.generation =") + 1].getStart(sf));
  return trivia.trim();
})();
const rewrite = s => s.replace(/state\.docAbort/gu, "session.artifact.docAbort").replace(/lease\.retirement/gu, "session.lease.retirement").replace(/this\.retire/gu, "session.retire");
const facadeBody = `
let documentBrowserActorGeneration = 0n;

${hubCode}
${docOf(classNode)}
export class DocumentBrowserActorReservation {
  readonly generation: bigint;
  private readonly hub: ReservationHub;
  private readonly retire = () => this.close();

  constructor(state: ArtifactState, lease: DocumentExecutionTargetLease, grant: DocumentBrowserActorGrant) {
    ${generationExhaustion}
    const session = new ReservationSession(state, lease, grant, this.retire, () => ++documentBrowserActorGeneration);
    this.generation = session.generation;
    this.hub = new ReservationHub(this, session);
    ${commentBlock.split("\n").map(l => l.trim() ? l.trim() : "").join("\n    ")}
    ${abortBlock.map(rewrite).join("\n    ")}
  }

${kept.join("\n\n")}

${getterDelegate("suspended")}

${FACADE_DELEGATES.map(delegateCode).join("\n")}
}
`;
const facadeImports = importsFor(facadeBody.replace(/^\s*\/\/.*$/gmu, ""), true, [...hubImports, `import { ReservationSession } from "./🗃️state/🟦️.ts";`]);
const leafCode = facadeImports + "\n" + facadeBody;
fs.writeFileSync(path.join(outDir, "..", "ActorReservation.generated.ts"), leafCode);
sizes["leaf"] = leafCode.split("\n").length;
console.log(JSON.stringify(sizes, null, 2));
