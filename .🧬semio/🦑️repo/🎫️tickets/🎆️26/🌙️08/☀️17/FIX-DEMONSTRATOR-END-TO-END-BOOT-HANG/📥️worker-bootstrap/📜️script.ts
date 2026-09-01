#!/usr/bin/env bun
/** 🏗️ Checks the declaration packet without mounting worker or receiver behavior. */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { createHash } from "node:crypto";
import assert from "node:assert/strict";
import Ajv from "ajv";
import ts from "typescript";
import { produce } from "immer";
import { OwnedResidentAdmission, OwnedResidentLedger, OwnedResidentRetirement, type ResidentStep } from "../../../../../../../../🧰️framework/🔨️modules/🌱️value/💾️resident/🟦️component.ts";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

type Resources = { bytes: number; slots: number; owners: number };
type Part = { id: string; scope: "shared" | "client" | "worker"; copies: number; base: number; fields: string[]; slots: number; owners: number };
type Declaration = { clientFields: string[]; slotFields: string[]; parts: Part[]; totals: Record<string, Resources>; model: { neutralRecord: Resources; neutralCell: Resources }; prerequisites: string[]; funding: { oldDomain: Resources; newDomain: Resources; retained: Resources; word: string; wordBytes: number; bootstrapReadsPurpose: boolean; bootstrapWritesPurpose: boolean; requiresLiveOriginalRecord: boolean } };
type State = { roots: string[]; worker: string | null; bindings: string[]; active: string | null; violation: string | null; newAdmissions: boolean; refunded: boolean; destination: string | null; precreated: string[]; attempted: string[]; factoryResult: string; firstFault: string | null; callerFaults: string[] };
type Case = { id: string; events: string[][]; expected: Omit<State, "destination"> };
type AdmissionCase = { id: string; purpose: string; requested: string; client: string; consumer: string; ledger: string; cellLedger: string; phase: string; accepted: boolean };
type GateCase = { id: string; ending: string; evidence: string; cell: string; live: boolean; fault: boolean; expected: { purpose: string; cell: string; construct: boolean } };
type NeutralCase = { id: string; expected: { steps: string[]; bytes: number; pending: boolean; terminal: boolean } };
type FactoryValueCase = { id: string; value: "undefined" | "object" | "promise"; construct: boolean; success: boolean; retained: boolean };
type Fixture = { sourceShapes: Record<string, number>; admissionPhases: [string, number][]; admissionCases: AdmissionCase[]; cases: Case[]; deniedClaims: string[]; fundingCases: { id: string; order: string[]; sharedRoot: number; workerController: number; purposeCharges: number }[]; gateCases: GateCase[]; neutralCases: NeutralCase[]; factoryValueCases: FactoryValueCase[] };
const root = new URL("../../../../../../../../", import.meta.url);
const packet = new URL("./", import.meta.url);
const load = (base: URL, file: string): string => readFileSync(new URL(file, base), "utf8");
const json = (file: string): unknown => JSON.parse(load(packet, file));
const initial = (): State => ({ roots: [], worker: null, bindings: [], active: null, violation: null, newAdmissions: true, refunded: false, destination: null, precreated: [], attempted: [], factoryResult: "unreturned", firstFault: null, callerFaults: [] });
const keep = (state: State, value: string): void => { if (!state.roots.includes(value)) state.roots.push(value); };
function advance(state: State, event: string[]): void {
  const [op, first, second] = event;
  if (op === "admit") { keep(state, "slot:A"); return; }
  assert(state.roots.includes("slot:A"), "original slot must precede construction/callbacks");
  if (op === "precreate") { assert(!state.worker && state.newAdmissions); for (const kind of ["message", "error", "messageerror"]) { state.precreated.push(kind); for (const owner of ["handler", "environment", "binding"]) keep(state, owner + ":" + kind); } return; }
  if (op === "construct") { if (state.newAdmissions && !state.worker) { assert.equal(state.precreated.length, 3); state.worker = "worker:A"; keep(state, state.worker); } return; }
  if (op === "factory-result") { assert(first && state.factoryResult === "unreturned"); state.factoryResult = first; if (first !== "undefined") keep(state, first); if (first !== "undefined" || !state.worker) state.newAdmissions = false; return; }
  if (op === "fault") { assert(first); if (!state.firstFault) { state.firstFault = first; keep(state, first); } else if (state.firstFault !== first && !state.callerFaults.includes(first)) state.callerFaults.push(first); state.newAdmissions = false; return; }
  if (op === "attempt-bind") { assert(state.worker && first && state.precreated.includes(first)); state.attempted.push(state.worker + "|handler:" + first + "|binding:" + first); return; }
  if (op === "bind") { assert(state.worker && first && state.attempted.includes(state.worker + "|handler:" + first + "|binding:" + first)); if (!state.bindings.includes(first)) state.bindings.push(first); return; }
  if (op === "attach") { keep(state, "attach:A"); keep(state, "sab:A"); return; }
  if (op === "capture") { assert(second); assert.equal(state.active, null, "no unproved ingress overwrite"); state.active = second; state.destination = null; keep(state, second); return; }
  if (op === "extract") { assert(state.active && first); keep(state, first); return; }
  if (op === "violate") { assert(state.active); if (!state.violation) { state.violation = state.active; state.active = null; } state.newAdmissions = false; return; }
  if (op === "handoff") { assert(state.active && first); state.destination = first; keep(state, "destination:" + first); return; }
  if (op === "observe") { if (state.destination === first) { state.active = null; state.destination = null; } return; }
  if (op === "fence" || op === "terminate") { state.newAdmissions = false; return; }
  assert(op === "reroute" || op === "refund", "unknown model event");
}
const publicState = ({ destination: _, ...state }: State): Omit<State, "destination"> => state;
const resources = (parts: Part[], scope: Part["scope"]): Resources => parts.filter(part => part.scope === scope).reduce((total, part) => ({ bytes: total.bytes + part.copies * (part.base + 16 * part.fields.length), slots: total.slots + part.copies * part.slots, owners: total.owners + part.copies * part.owners }), { bytes: 0, slots: 0, owners: 0 });
const sum = (...values: Resources[]): Resources => values.reduce((a, b) => ({ bytes: a.bytes + b.bytes, slots: a.slots + b.slots, owners: a.owners + b.owners }), { bytes: 0, slots: 0, owners: 0 });
function gateEnding(vector: GateCase): GateCase["expected"] {
  const { ending, evidence, cell, live, fault } = vector;
  const claimed = (ending === "claimed" || ending === "claim-fault") && evidence === "original-claimed-and-pending-empty" && cell === "claimed";
  const untouched = ending === "short" && evidence === "not-called" && cell === "none";
  const empty = (ending === "blocked" || ending === "known-refusal") && evidence === "returned-and-observed-empty" && cell === "none" && !fault;
  const cancelled = ending === "cancel" && evidence === "original-retirement" && cell === "terminal";
  const faultHeld = ending === "fault-close" && evidence === "pending-release-observed" && cell === "fault-held" && fault;
  return { purpose: untouched || empty || claimed || cancelled || faultHeld ? "none" : "held", cell, construct: claimed && ending === "claimed" && live && !fault };
}
const grant = (maxBytes: number) => ({ maxItems: 1, maxBytes });
function neutralCase(vector: NeutralCase): void {
  const id = vector.id, consumer = {}, other = {};
  const ledger = new OwnedResidentLedger({ bytes: id === "capacity-refusal-no-cell" ? 295 : 4096, slots: 128, owners: 128, control: { bytes: 0, slots: 0, owners: 0 } });
  const steps: string[] = [];
  const see = (value: ResidentStep): void => { steps.push(value.kind); };
  let cell: OwnedResidentAdmission | null = null;
  let reads = 0;
  const first = Object.defineProperty({}, "message", { get() { reads++; throw new Error("fault getter must not run"); } });
  const distinct = Object.defineProperty({}, "stack", { get() { reads++; throw new Error("distinct getter must not run"); } });
  if (id === "closed-ledger-no-cell") ledger.beginClose();
  if (id === "prepare-then-wrapper-throw") {
    let caught: unknown;
    try { see(ledger.prepareAdmission(consumer, "data", grant(296))); throw first; } catch (error) { caught = error; }
    assert.equal(caught, first); cell = ledger.preparedAdmission(consumer); assert(cell); see(cell.retainFailure(caught, grant(64))); assert.equal(cell.failure, first);
  } else {
    see(ledger.prepareAdmission(consumer, "data", grant(id === "short-preparation" ? 295 : 296)));
    cell = ledger.preparedAdmission(consumer);
    if (id === "same-client-alias-and-foreign-refusal") {
      assert(cell); see(ledger.prepareAdmission(consumer, "data", grant(296))); assert.equal(ledger.preparedAdmission(consumer), cell);
      see(ledger.prepareAdmission(other, "data", grant(296))); assert.equal(ledger.preparedAdmission(other), null); see(ledger.claimAdmission(other, cell, grant(64))); assert.equal(ledger.preparedAdmission(consumer), cell);
    } else if (id === "claim-then-wrapper-throw") {
      assert(cell); const original = cell; let caught: unknown;
      try { see(ledger.claimAdmission(consumer, original, grant(64))); throw first; } catch (error) { caught = error; }
      assert.equal(caught, first); assert(original.claimed); assert.equal(ledger.preparedAdmission(consumer), null); see(original.retainFailure(caught, grant(64))); assert.equal(original.failure, first);
    } else if (id === "healthy-prepared-cancellation") {
      assert(cell); cell.beginClose(); see(cell.closeStep(grant(63))); assert.equal(ledger.preparedAdmission(consumer), cell);
      see(cell.closeStep(grant(64))); assert.equal(ledger.preparedAdmission(consumer), null); assert(!cell.terminalIsEmpty()); assert.equal(ledger.usage.data.bytes, 296);
      see(cell.closeStep(grant(295))); see(cell.closeStep(grant(296))); assert(OwnedResidentRetirement.matches(cell.retirement, cell));
    } else if (id === "first-and-distinct-faults" || id === "fault-held-cell-does-not-block-other-client") {
      assert(cell); see(cell.retainFailure(first, grant(64)));
      if (id === "first-and-distinct-faults") { see(cell.retainFailure(first, grant(64))); see(cell.retainFailure(distinct, grant(64))); assert.equal(cell.failure, first); assert.notEqual(cell.failure, distinct); }
      see(cell.closeStep(grant(64))); assert.equal(ledger.preparedAdmission(consumer), null); see(cell.closeStep(grant(296))); assert.equal(cell.retirement, null); assert.equal(cell.failure, first);
      if (id === "fault-held-cell-does-not-block-other-client") { see(ledger.prepareAdmission(other, "data", grant(296))); const next = ledger.preparedAdmission(other); assert(next && next !== cell); assert.equal(cell.failure, first); }
    } else if (id.startsWith("identity-only-")) {
      assert(cell); see(ledger.claimAdmission(consumer, cell, grant(64))); const result = ledger.reserveRecord("data", { bytes: 224, slots: 1, owners: 1 }, cell, grant(264)); see(result.step);
      const record = cell.result?.record; assert(record && record === result.record); const shell = {}; see(record.install(shell, grant(64))); assert(record.matchesLiveShell(shell));
      if (id === "identity-only-ledger-close") ledger.beginClose(); else if (id === "identity-only-record-close") record.beginClose(); else if (id === "identity-only-cell-close") cell.beginClose(); else { assert.equal(id, "identity-only-fault-close"); assert.equal(cell.retainFailure(first, grant(64)).kind, "pending"); assert.equal(cell.failure, first); }
      assert(record.matchesShell(shell)); assert(!record.matchesLiveShell(shell)); assert.equal(record.install({}, grant(64)).kind, "rejected"); assert.equal(cell.result?.record, record);
    }
  }
  assert.equal(reads, 0, vector.id);
  assert.deepEqual({ steps, bytes: ledger.usage.data.bytes, pending: ledger.preparedAdmission(consumer) !== null, terminal: cell?.terminalIsEmpty() ?? false }, vector.expected, vector.id);
}
function factoryValueCase(vector: FactoryValueCase): void {
  let reads = 0;
  const original: unknown = vector.value === "undefined" ? undefined : vector.value === "promise" ? new Promise<void>(() => {}) : {};
  if (original !== undefined) Object.defineProperty(original, "then", { get() { reads++; throw new Error("factory then must not run"); } });
  const retained: { worker: object | null; factoryResult: unknown } = { worker: null, factoryResult: undefined };
  const factory = (): unknown => { if (vector.construct) retained.worker = {}; return original; };
  retained.factoryResult = factory();
  const success = retained.factoryResult === undefined && retained.worker !== null;
  assert.equal(success, vector.success, vector.id); assert.equal(retained.factoryResult, original); assert.equal(retained.factoryResult !== undefined, vector.retained); assert.equal(reads, 0);
}
function fields(source: string, name: string): string[] {
  const file = ts.createSourceFile(name + ".ts", source, ts.ScriptTarget.Latest, true);
  const declaration = file.statements.find(node => (ts.isClassDeclaration(node) || ts.isTypeAliasDeclaration(node)) && node.name?.text === name);
  assert(declaration);
  const members = ts.isClassDeclaration(declaration) ? declaration.members : ts.isTypeAliasDeclaration(declaration) && ts.isTypeLiteralNode(declaration.type) ? declaration.type.members : [];
  return members.filter(node => ts.isPropertyDeclaration(node) || ts.isPropertySignature(node)).map(node => node.name.getText(file).replace(/^#/, ""));
}
class CheckScript extends BundleScript {
  async run(): Promise<void> {
    const paths = ["🧬️schema.json", "🧬️declaration.json", "🧪️schema.json", "🧪️fixture.json", "📜️script.ts"];
    const actor = "🧰️framework/🔨️modules/🎭️actor/";
    const shardPath = actor + "📦️packages/🟦️typescript/🧵️shard-client.ts";
    const inventoryPath = actor + "📤️return/📨️response/🎟️credit/📋️metadata/📥️inbox/🧪️fixture.json";
    const neutralPath = "🧰️framework/🔨️modules/🌱️value/💾️resident/🟦️component.ts";
    const captures = [...paths.map(path => new URL(path, packet)), new URL(shardPath, root), new URL(inventoryPath, root), new URL(neutralPath, root)];
    const hashes = () => captures.map(url => ({ path: fileURLToPath(url), sha256: createHash("sha256").update(readFileSync(url)).digest("hex") }));
    const before = hashes();
    const ajv = new Ajv({ strict: true, allErrors: true });
    const declaration = json("🧬️declaration.json") as Declaration;
    const fixture = json("🧪️fixture.json") as Fixture;
    for (const [schema, value] of [["🧬️schema.json", declaration], ["🧪️schema.json", fixture]] as const) {
      const validate = ajv.compile(json(schema) as object);
      assert(validate(value), JSON.stringify(validate.errors));
      assert(!validate({ ...value, extra: true }), "closed schema");
    }
    assert.equal(new Set(declaration.parts.map(part => part.id)).size, declaration.parts.length);
    assert.deepEqual([...declaration.parts.find(part => part.id === "client-controller-delta")!.fields, ...declaration.parts.find(part => part.id === "shared-purpose-delta")!.fields], declaration.clientFields);
    assert.deepEqual(declaration.parts.find(part => part.id === "original-slot")?.fields, declaration.slotFields);
    const shared = resources(declaration.parts, "shared"), client = resources(declaration.parts, "client"), worker = resources(declaration.parts, "worker");
    assert.deepEqual(client, declaration.totals.clientDomain);
    assert.deepEqual(worker, declaration.totals.workerDomain);
    const overhead = sum(declaration.model.neutralRecord, declaration.model.neutralCell);
    assert.deepEqual(sum(client, overhead), declaration.totals.clientRetained);
    assert.deepEqual(sum(worker, overhead), declaration.totals.workerRetained);
    assert.deepEqual(shared, declaration.totals.sharedPurposeDelta);
    assert.deepEqual(sum({ bytes: 208, slots: 1, owners: 1 }, shared), declaration.totals.sharedControllerDomain);
    assert.deepEqual(sum(declaration.totals.sharedControllerDomain!, overhead), declaration.totals.sharedControllerRetained);
    assert.deepEqual(sum(shared, client, worker, overhead, overhead), declaration.totals.oneWorkerCombined);
    assert.deepEqual(sum(declaration.totals.sharedControllerRetained!, client, worker, overhead, overhead), declaration.totals.workerOnlyRetained);
    assert.equal(declaration.funding.word, "clientAdmissionPurpose");
    assert.equal(declaration.funding.wordBytes, shared.bytes);
    assert.deepEqual(declaration.funding.newDomain, sum(declaration.funding.oldDomain, shared));
    assert.deepEqual(declaration.funding.retained, declaration.totals.sharedControllerRetained);
    assert(!declaration.funding.bootstrapReadsPurpose && !declaration.funding.bootstrapWritesPurpose && declaration.funding.requiresLiveOriginalRecord);
    const exactWorker = declaration.parts.filter(part => part.scope === "worker").reduce((total, part) => total + BigInt(part.copies) * (BigInt(part.base) + 16n * BigInt(part.fields.length)), 0n);
    assert.equal(exactWorker.toString(), String(worker.bytes));
    const source = load(root, shardPath);
    const clientFields = fields(source, "ShardClient"), slotFields = fields(source, "ShardSlot"), pendingFields = fields(source, "PendingEntry");
    const inventory = JSON.parse(load(root, inventoryPath));
    for (const [name, current] of [["ShardClient", clientFields], ["ShardSlot", slotFields], ["PendingEntry", pendingFields]] as const) assert.deepEqual(current, inventory.layouts.find((layout: { declaration: string }) => layout.declaration === name).fields);
    assert.equal(clientFields.length, fixture.sourceShapes.clientFields);
    assert.equal(64 + 16 * clientFields.length, fixture.sourceShapes.clientBytes);
    assert.equal(slotFields.length, fixture.sourceShapes.slotFields);
    assert.equal(64 + 16 * slotFields.length, fixture.sourceShapes.slotBytes);
    assert.equal(pendingFields.length, fixture.sourceShapes.pendingFields);
    assert.equal(64 + 16 * pendingFields.length, fixture.sourceShapes.pendingBytes);
    const controllerFields = clientFields.filter(field => field.startsWith("uiResident") || field === "clientAdmissionPurpose");
    assert.equal(controllerFields.length, fixture.sourceShapes.uiControllerFields);
    assert.equal(64 + 16 * controllerFields.length, fixture.sourceShapes.uiControllerBytes);
    assert.equal(declaration.funding.newDomain.bytes, fixture.sourceShapes.uiControllerBytes);
    const workerController = clientFields.filter(field => field.startsWith("workerBootstrap") || field.startsWith("workerAdmission"));
    assert.equal(workerController.length, fixture.sourceShapes.workerControllerFields);
    assert.equal(16 * workerController.length, fixture.sourceShapes.workerControllerBytes);
    assert.equal(fixture.sourceShapes.clientBytes - fixture.sourceShapes.uiControllerBytes - fixture.sourceShapes.workerControllerBytes, fixture.sourceShapes.unadmittedClientRemainderBytes);
    assert.deepEqual(workerController, declaration.clientFields.filter(field => field !== "clientAdmissionPurpose"), "metadata-only words are now source-mounted; slot and receiver remain absent");
    assert.equal(declaration.clientFields.filter(field => field === "clientAdmissionPurpose").length, 1);
    assert.deepEqual(declaration.slotFields.slice(0, 6), slotFields);
    assert.equal(source.includes("onmessageerror:"), false, "current missing callback remains explicit");
    assert(source.includes("this.shards.push(this.spawnShard(index))"), "current constructor gap remains explicit");
    assert(source.includes("this.handleMessage(slot, event.data as InboundMessage)"), "current pre-capture extraction remains explicit");
    assert.equal(new Set(fixture.cases.map(value => value.id)).size, fixture.cases.length);
    for (const vector of fixture.cases) {
      const mutable = initial();
      for (const event of vector.events) advance(mutable, event);
      const immutable = vector.events.reduce((state, event) => produce(state, draft => advance(draft, event)), initial());
      assert.deepEqual(publicState(mutable), vector.expected, vector.id);
      assert.deepEqual(publicState(immutable), vector.expected, "Immer replay: " + vector.id);
    }
    for (const vector of fixture.admissionCases) {
      const sameOwner = vector.client === vector.consumer && vector.ledger === vector.cellLedger;
      const samePurpose = vector.phase === "empty" ? vector.purpose === "none" : vector.phase === "prepared" && vector.purpose === vector.requested;
      assert.equal(sameOwner && samePurpose, vector.accepted, vector.id);
    }
    for (const vector of fixture.fundingCases) {
      let sharedRoot = 0, workerController = 0, purposeCharges = 0;
      for (const operation of vector.order) {
        if (operation === "close-ui") continue;
        if (sharedRoot === 0) { sharedRoot = declaration.funding.retained.bytes; purposeCharges++; }
        assert(sharedRoot >= declaration.funding.retained.bytes);
        if (operation === "worker-root") workerController = declaration.totals.clientRetained!.bytes;
      }
      assert.deepEqual({ sharedRoot, workerController, purposeCharges }, { sharedRoot: vector.sharedRoot, workerController: vector.workerController, purposeCharges: vector.purposeCharges }, vector.id);
    }
    for (const vector of fixture.gateCases) assert.deepEqual(gateEnding(vector), vector.expected, vector.id);
    for (const vector of fixture.neutralCases) neutralCase(vector);
    for (const vector of fixture.factoryValueCases) factoryValueCase(vector);
    assert.deepEqual(fixture.admissionPhases.map(phase => phase[1]), [296, 64, 64, 64, 264, 64, 64, 64]);
    assert.equal(fixture.deniedClaims.length, 10);
    const after = hashes(); assert.deepEqual(after, before, "captured declaration/source drift");
    console.log(JSON.stringify({ status: "PASS", scope: "declaration/model plus isolated existing neutral primitives; no Shard bootstrap runtime", schemas: 2, sourceLayouts: 3, cases: fixture.cases.length, admissionCases: fixture.admissionCases.length, fundingCases: fixture.fundingCases.length, gateCases: fixture.gateCases.length, neutralCases: fixture.neutralCases.length, factoryValueCases: fixture.factoryValueCases.length, thirdPartyReplay: "Ajv strict; TypeScript AST; Immer same normative reducer, not independent production semantics", resources: declaration.totals, hashes: after }));
  }
}
await runBundleScriptMain(new ScriptRouter(import.meta.dir).register("check", CheckScript), import.meta.url, { defaultCommand: "check" });
