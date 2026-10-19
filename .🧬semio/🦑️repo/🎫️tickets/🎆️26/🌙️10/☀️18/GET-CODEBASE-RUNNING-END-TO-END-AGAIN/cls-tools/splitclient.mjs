// usage: node splitclient.mjs <orig-shard-client.ts> <shard-client-dir>
// Redesigns `class ShardClient` into a facade plus collaborators that share one private state object.
import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/node_modules/");
const ts = require("typescript");

const [origPath, outDir] = process.argv.slice(2);
const text = fs.readFileSync(origPath, "utf8");
const sf = ts.createSourceFile("shard-client.ts", text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);

const STATE_FIELDS = ["shards", "actorShard", "actorActivations", "instanceLifecycles", "instanceTurns", "pending", "actorsPastFirstTurn", "exclusiveIndices", "heartbeatSabView", "heartbeatTimeoutMs", "firstTurnTimeoutMs", "watchdogIntervalMs", "now", "createWorker", "onShardLost", "onActorTrap", "onLiveness", "onHostEffect", "maxOutstandingEffectsPerActor", "outstandingEffectsByActor", "effectReplySeq", "nextRoundRobin", "requestSeq", "activationGeneration", "watchdogHandle"];
const STATE_HASH = { "#residentLedger": "residentLedger", "#originalWorkStock": "originalWorkStock", "#originalClosures": "originalClosures", "#originalReturns": "originalReturns" };
const RESIDENT_FIELD = /^#(uiResident\w+|clientAdmissionPurpose|workerBootstrap\w+|workerAdmission\w+)$/u;

const COLLABORATORS = {
  residents: { dir: "💾️resident-bootstrap", cls: "ShardResidentBootstrap", emoji: "💾️", doc: "Drives the resident-ledger admission of the worker bootstrap metadata and of the shared UI resident pool as bounded, restartable steps; the phase fields live in {@link ResidentAdmissionState}.", methods: ["prepareWorkerBootstrap", "#captureWorkerBootstrapFault", "#recoverWorkerBootstrap", "closeWorkerBootstrapStep", "prepareUiResidentPool", "captureUiResidentPoolFault", "#prepareSharedResidentController", "#recoverUiResidentController", "#prepareUiResidentController", "#recoverUiResidentPool", "#handoffUiResidentFault", "#closeUiResidentAdmission", "ownsUiResidentPool", "closeUiResidentPoolStep", "installUiResidentPool", "releaseUiResidentPool"] },
  transport: { dir: "📬️request-transport", cls: "ShardRequestTransport", emoji: "📬️", doc: "Spawns shard workers, posts correlated requests and settles them from the per-worker inbound messages.", methods: ["spawnShard", "handleMessage", "recomputeOldestPending", "failShard", "rejectActorPending", "nextRequestId", "send"] },
  routing: { dir: "🧭️routing", cls: "ShardRouting", emoji: "🧭️", doc: "Pins actors onto shards, leases the exclusive shards and terminates, rebuilds or disposes whole shards.", methods: ["assignShard", "leaseExclusive", "releaseExclusive", "shardIndexFor", "requireShard", "terminate", "rebuild", "disposeAll"] },
  activations: { dir: "🪪️exact-activation", cls: "ShardExactActivations", emoji: "🪪️", doc: "Owns the exact activation of every actor: activation, revocable operation leases and teardown.", methods: ["activate", "activationIsActive", "inboundActivation", "captureActorActivation", "dispose", "disposeActivation"] },
  lifecycles: { dir: "🚪️instance-lifecycle", cls: "ShardInstanceLifecycles", emoji: "🚪️", doc: "Owns the exact instance lifecycle of an activation: open, poll, close and acknowledgement crossings with their receipts.", methods: ["captureInstanceLifecycle", "beginInstanceLifecycleClose", "sendInstanceLifecycle", "acceptInstanceLifecycleResult"] },
  returns: { dir: "↩️captured-return", cls: "ShardCapturedReturns", emoji: "↩️", doc: "Admits, drives and retires the fixed-response captured return of an instance.", methods: ["reserveInstanceReturn", "retireUnusedInstanceReturn", "reserveReturnResponse", "sendCapturedReturn", "acceptCapturedReturn"] },
  uiAcknowledgements: { dir: "📥️ui-acknowledgement", cls: "ShardUiAcknowledgements", emoji: "📥️", doc: "Captures the native UI patches of an instance turn and acknowledges them in one lifecycle crossing.", methods: ["recordInstanceTurn", "captureInstanceUiPatch", "admitInstanceUiAcknowledgement", "patchAckEvent", "submitInstanceUiAcknowledgements", "submitInstanceUiAcknowledgement"] },
  effects: { dir: "🌉️host-effect-bridge", cls: "ShardHostEffectBridge", emoji: "🌉️", doc: "Bridges the `effect-request` frames a guest sends up to the host effect handler and posts the replies back down.", methods: ["handleInboundFrame", "handleEffectRequest", "settleEffect", "abortOutstandingEffects", "removeEffect", "cancelOneEffect", "postEffectReply", "replyEffectComplete", "replyEffectError"] },
  watchdog: { dir: "⏱️watchdog", cls: "ShardWatchdog", emoji: "⏱️", doc: "Folds every sign of life into the per-shard heartbeat state and terminates and rebuilds a silent shard.", methods: ["noteLiveness", "recordHeartbeat", "outstandingRequests", "pollHeartbeatSab", "checkHeartbeats", "startWatchdog", "stopWatchdog", "shardMetricsSamples"] },
};
const FACADE_DELEGATES = ["prepareWorkerBootstrap", "closeWorkerBootstrapStep", "prepareUiResidentPool", "captureUiResidentPoolFault", "ownsUiResidentPool", "closeUiResidentPoolStep", "installUiResidentPool", "releaseUiResidentPool", "leaseExclusive", "releaseExclusive", "shardIndexFor", "activate", "captureActorActivation", "dispose", "captureInstanceLifecycle", "pollHeartbeatSab", "checkHeartbeats", "startWatchdog", "stopWatchdog", "shardMetricsSamples", "terminate", "rebuild", "disposeAll"];
const FACADE_PRIVATE_DELEGATES = ["requireShard", "nextRequestId", "send"];
const FACADE_KEPT = ["turn", "envelope", "grant", "startJob", "stepJob", "cancelJob", "takeSegmentedDownloadChunk", "checkpoint", "restore", "codec"];

const ownerOf = new Map();
for (const [key, spec] of Object.entries(COLLABORATORS)) for (const m of spec.methods) ownerOf.set(m, key);

const classNode = sf.statements.find(s => ts.isClassDeclaration(s) && s.name?.text === "ShardClient");
const members = new Map();
for (const member of classNode.members) {
  const name = member.name ? member.name.getText(sf) : (ts.isClassStaticBlockDeclaration(member) ? "<static>" : ts.isConstructorDeclaration(member) ? "<ctor>" : "<?>");
  if (ts.isMethodDeclaration(member)) members.set(name, member);
}
for (const spec of Object.values(COLLABORATORS)) for (const m of spec.methods) if (!members.has(m)) throw new Error("missing method " + m);
for (const m of [...FACADE_KEPT, ...FACADE_DELEGATES, ...FACADE_PRIVATE_DELEGATES]) if (!members.has(m)) throw new Error("missing facade method " + m);

const crossCalled = new Set();
const collectCross = (node, current) => {
  const visit = n => {
    if (ts.isPropertyAccessExpression(n) && n.expression.kind === ts.SyntaxKind.ThisKeyword) {
      const name = n.name.text;
      const owner = ownerOf.get(name) ?? ownerOf.get("#" + name);
      if (owner !== undefined && owner !== current) crossCalled.add(name);
    }
    ts.forEachChild(n, visit);
  };
  visit(node);
};
for (const [key, spec] of Object.entries(COLLABORATORS)) for (const m of spec.methods) collectCross(members.get(m), key);
for (const m of FACADE_KEPT) collectCross(members.get(m), "facade");
for (const m of [...FACADE_DELEGATES, ...FACADE_PRIVATE_DELEGATES, "sendCapturedReturn", "reserveReturnResponse"]) crossCalled.add(m);

const leadingTrivia = member => member.getFullText(sf).slice(0, member.getStart(sf) - member.getFullStart());
const docOf = member => {
  const trivia = leadingTrivia(member);
  const match = /\/\*\*[\s\S]*?\*\/\s*$/u.exec(trivia);
  return match ? match[0].trimEnd() : "";
};

function transformMember(member, current, mode) {
  const start = member.getStart(sf), end = member.getEnd();
  const edits = [];
  const memberName = member.name;
  if (memberName && ts.isPrivateIdentifier(memberName)) edits.push({ pos: memberName.getStart(sf), end: memberName.getEnd(), text: memberName.text.slice(1) });
  const visit = n => {
    if (ts.isPropertyAccessExpression(n) && n.expression.kind === ts.SyntaxKind.ThisKeyword) {
      const name = n.name.text;
      let replacement = null;
      if (name in STATE_HASH) replacement = `this.${mode === "facade" ? "hub.state" : "state"}.${STATE_HASH[name]}`;
      else if (RESIDENT_FIELD.test(name)) replacement = `this.resident.${name.slice(1)}`;
      else if (name.startsWith("#")) replacement = `this.${name.slice(1)}`;
      else if (STATE_FIELDS.includes(name)) replacement = `this.${mode === "facade" ? "hub.state" : "state"}.${name}`;
      else if (ownerOf.has(name) && (mode === "facade" ? !FACADE_DELEGATES.includes(name) && !FACADE_PRIVATE_DELEGATES.includes(name) : ownerOf.get(name) !== current)) replacement = `this.hub.${ownerOf.get(name)}.${name}`;
      else if (ownerOf.has("#" + name) && ownerOf.get("#" + name) !== current) replacement = `this.hub.${ownerOf.get("#" + name)}.${name}`;
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
  out = out.replace(/^(\s*)private (async )?/u, (m, indent, asyncKw) => crossCalled.has(member.name.getText(sf).replace(/^#/u, "")) ? `${indent}${asyncKw ?? ""}` : m);
  if (/^#/.test(member.name.getText(sf))) out = "private " + out;
  const doc = docOf(member);
  return (doc ? "  " + doc + "\n" : "") + "  " + out;
}

const indent = s => s;

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

const stateHeader = `/** 🗃️ The single private state object of one {@link ShardClient}: every mutable field and every injected policy of the transport lives here, and the facade and its collaborators share it instead of holding many private fields. */`;

const write = (dir, content) => {
  const target = path.join(outDir, dir, "🟦️.ts");
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, content);
  return content.split("\n").length;
};

const sizes = {};

for (const [key, spec] of Object.entries(COLLABORATORS)) {
  const body = spec.methods.map(m => indent(transformMember(members.get(m), key, "collab"))).join("\n\n");
  const stateExtra = key === "residents";
  const residentState = stateExtra ? `
/** 📋️ The phase fields of the UI resident pool admission and of the worker bootstrap admission; field names and order are the census the admission accounting prices. */
export class ResidentAdmissionState {
  uiResidentControllerCell: OwnedResidentAdmission | null = null;
  uiResidentControllerRecord: OwnedResidentRecord | null = null;
  uiResidentCell: OwnedResidentAdmission | null = null;
  uiResidentRecord: OwnedResidentRecord | null = null;
  uiResidentPool: OwnedUiResidentPool | null = null;
  uiResidentPhase: ${classNode.members.find(m => m.name?.getText(sf) === "#uiResidentPhase").type.getText(sf)} = "controller-empty";
  uiResidentWitness: OwnedUiResidentPoolRetirement | null = null;
  uiResidentFault: unknown = NO_RESIDENT_FAULT;
  uiResidentClosing = false;
  clientAdmissionPurpose: "none" | "ui-pool" | "worker-root" = "none";
  workerBootstrapCell: OwnedResidentAdmission | null = null;
  workerBootstrapRecord: OwnedResidentRecord | null = null;
  workerBootstrapPhase: ${classNode.members.find(m => m.name?.getText(sf) === "#workerBootstrapPhase").type.getText(sf)} = "empty";
  workerBootstrapFault: unknown = NO_RESIDENT_FAULT;
  workerAdmissionCell: OwnedResidentAdmission | null = null;
  workerAdmissionRecord: OwnedResidentRecord | null = null;
  workerAdmissionIndex: number | null = null;
  workerAdmissionShell: ShardSlot | null = null;
}
` : "";
  const classCode = `${residentState}
/** ${spec.emoji} ${spec.doc} */
export class ${spec.cls} {
  private readonly state: ShardClientState;
${stateExtra ? "  private readonly resident = new ResidentAdmissionState();\n" : ""}  constructor(private readonly hub: ShardClientHub) { this.state = hub.state; }

${body}
}
`;
  const extra = [`import type { ShardClientState } from "../🗃️state/🟦️.ts";`, `import type { ShardClientHub } from "../🟦️.ts";`];
  const code = importsFor(classCode, false, extra) + "\n" + classCode;
  sizes[spec.dir] = write(spec.dir, code);
}

const hubClasses = Object.entries(COLLABORATORS);
const hubCode = `/** 🧩️ Wires the collaborators of one {@link ShardClient} around its shell and its single private state; collaborators reach each other only through this hub, and only at call time, never while they are constructed. */
export class ShardClientHub {
${hubClasses.map(([k, s]) => `  readonly ${k}: ${s.cls};`).join("\n")}
  constructor(readonly shell: ShardClient, readonly state: ShardClientState) {
${hubClasses.map(([k, s]) => `    this.${k} = new ${s.cls}(this);`).join("\n")}
  }
}
`;
const hubImports = hubClasses.map(([, s]) => `import { ${s.cls} } from "./${s.dir}/🟦️.ts";`);


const field = name => classNode.members.find(m => m.name?.getText(sf) === name);
const fieldDoc = name => { const f = field(name); return f ? docOf(f) : ""; };
const ctor = classNode.members.find(ts.isConstructorDeclaration);
const ctorStatements = ctor.body.statements.map(st => st.getText(sf));
const bodyFor = predicate => ctorStatements.filter(predicate).map(t => "    " + t.replace(/this\.#originalWorkStock/gu, "this.originalWorkStock").replace(/this\.#residentLedger/gu, "this.residentLedger")).join("\n");
const stateBody = bodyFor(t => !t.includes("this.shards.push"));
const stateCode = `
${stateHeader}
export class ShardClientState {
  readonly residentLedger: OwnedResidentLedger;
  readonly shards: ShardSlot[] = [];
  readonly actorShard = new Map<string, number>();
  readonly actorActivations = new Map<string, ShardActivation>();
  readonly instanceLifecycles = new Map<number, ShardInstanceOwner>();
  readonly instanceTurns = new WeakMap<object, { readonly owner: ShardInstanceOwner; readonly patches: WeakMap<object, OwnedNativeUiPatchAuthority> }>();
  readonly pending = new Map<string, PendingEntry>();
  ${fieldDoc("actorsPastFirstTurn").replace(/\n  /gu, "\n  ")}
  readonly actorsPastFirstTurn = new Set<string>();
  readonly exclusiveIndices: ReadonlySet<number>;
  readonly heartbeatSabView: Int32Array | null;
  readonly heartbeatTimeoutMs: number;
  readonly firstTurnTimeoutMs: number;
  readonly watchdogIntervalMs: number;
  readonly now: () => number;
  readonly createWorker: CreateShardWorker;
  readonly onShardLost?: ShardClientOptions["onShardLost"];
  readonly onActorTrap?: ShardClientOptions["onActorTrap"];
  readonly onLiveness?: ShardClientOptions["onLiveness"];
  readonly onHostEffect?: HostEffectHandler;
  readonly maxOutstandingEffectsPerActor: number;
  ${fieldDoc("outstandingEffectsByActor")}
  readonly outstandingEffectsByActor = new Map<string, HostEffectLedger>();
  effectReplySeq = 0;
  nextRoundRobin = 0;
  requestSeq = 0;
  activationGeneration = 0n;
  watchdogHandle: ReturnType<typeof setInterval> | null = null;
  readonly originalWorkStock: OriginalBootstrapOwner;
  readonly originalClosures = new Map<bigint, ShardActivation>();
  readonly originalReturns = new Map<bigint, Promise<void>>();
  constructor(options: ShardClientOptions) {
${stateBody}
  }
}
`;
sizes["🗃️state"] = write("🗃️state", importsFor(stateCode, false) + "\n" + stateCode);

const parts = {
  delegates: FACADE_DELEGATES.concat(FACADE_PRIVATE_DELEGATES).map(name => {
    const m = members.get(name);
    const params = m.parameters.map(p => ({ name: p.name.getText(sf), optional: !!p.questionToken || !!p.initializer, type: p.type ? p.type.getText(sf) : "unknown" }));
    return { name, owner: ownerOf.get(name), params, ret: m.type ? m.type.getText(sf) : null, isPrivate: FACADE_PRIVATE_DELEGATES.includes(name), generics: m.typeParameters ? `<${m.typeParameters.map(t => t.getText(sf)).join(", ")}>` : "" };
  }),
  kept: FACADE_KEPT.map(name => ({ name, code: transformMember(members.get(name), "facade", "facade") })),
};
const delegateCode = d => {
  const params = d.params.map(p => `${p.name}${p.optional ? "?" : ""}: ${p.type}`).join(", ");
  const call = `this.hub.${d.owner}.${d.name}(${d.params.map(p => p.name).join(", ")})`;
  return `  ${d.isPrivate ? "private " : ""}${d.name}${d.generics}(${params})${d.ret ? ": " + d.ret : ""} { return ${call}; }`;
};
const afterClass = sf.statements.filter(st => st.getStart(sf) > classNode.getEnd());
const testDeps = afterClass.map(st => st.getText(sf)).join("\n");
const reexports = sf.statements.filter(st => ts.isExportDeclaration(st)).map(st => st.getText(sf)).join("\n");
const headerDoc = sf.getFullText().slice(sf.statements[2].getFullStart(), sf.statements[2].getStart(sf)).match(/\/\*\*[\s\S]*?\*\//u)[0];
const classDoc = docOf(classNode);
const facadeBody = `
${reexports}

${headerDoc}

${hubCode}
//#region 🧵️ShardClient
${classDoc}
export class ShardClient {
  readonly #residentLedger: OwnedResidentLedger;
  private readonly hub: ShardClientHub;
  constructor(options: ShardClientOptions) {
    const state = new ShardClientState(options);
    this.#residentLedger = state.residentLedger;
    this.hub = new ShardClientHub(this, state);
    for (let index = 0; index < options.shardCount; index += 1) state.shards.push(this.hub.transport.spawnShard(index));
  }

  static { bindCapturedReturnDispatch((client, state, work, budget) => client.hub.returns.sendCapturedReturn(state, work, budget), (client, state, grant) => client.hub.returns.reserveReturnResponse(state, grant)); }
  ${members.get("matchesResidentLedger")?.getText(sf) ?? classNode.members.find(m => m.name?.getText(sf) === "matchesResidentLedger").getText(sf)}

  ${docOf(classNode.members.find(m => m.name?.getText(sf) === "matchesActivation"))}
  ${classNode.members.find(m => m.name?.getText(sf) === "matchesActivation").getText(sf)}

${parts.delegates.map(delegateCode).join("\n")}

${parts.kept.map(k => k.code).join("\n\n")}
}
//#endregion 🧵️ShardClient

${testDeps}
`;
const facadeImports = importsFor(facadeBody.replace(reexports, ""), true, [...hubImports, `import { ShardClientState } from "./🗃️state/🟦️.ts";`]);
fs.writeFileSync(path.join(outDir, "..", "ShardClient.generated.ts"), facadeImports + "\n" + facadeBody);
sizes["facade"] = (facadeImports + "\n" + facadeBody).split("\n").length;
console.log(JSON.stringify(sizes, null, 2));
