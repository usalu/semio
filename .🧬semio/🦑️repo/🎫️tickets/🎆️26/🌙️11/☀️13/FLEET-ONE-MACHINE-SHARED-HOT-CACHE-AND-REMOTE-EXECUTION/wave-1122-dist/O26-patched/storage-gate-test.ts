import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { testPythonV1 } from "../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import { componentBatchesV1 } from "../../🟦️.ts";
import fixture from "../../🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import { admitPassStorageV1, batchAdditionsV1, COMPONENT_STORAGE_GATE_POLICY_V1 as policy, creditPresentBytesV1, describeStorageGateV1, learnedBatchBytesV1, measurePresentDepsBytesV1, readStorageGrowthV1, recordStorageGrowthV1, storageChargeBytesV1, storageGateV1, storageGrowDecisionV1, storageGrowthKeyV1, type StorageGatePortsV1, type StorageRunV1 } from "../🟦️.ts";

const oracle = fileURLToPath(new URL("./🐍️oracle/🐍️.py", import.meta.url));
const python = (cases: unknown): unknown[] => { const result = spawnSync(testPythonV1() ?? "python3", ["-I", oracle], { input: JSON.stringify(cases), encoding: "utf8", maxBuffer: 1 << 26 }); expect(result.status, result.stderr).toBe(0); return JSON.parse(result.stdout); };
const GiB = 1024 ** 3, MiB = 1024 ** 2;
const scratch = (): string => mkdtempSync(join(tmpdir(), "o26-"));

type World = { headroom: number; running: StorageRunV1[]; settled: Promise<void>[]; log: string[]; slept: number; grown: number[]; reclaimed: number[]; clock: number; held: number; cancelled: boolean; hostFree: number | undefined; imageBytes: number };
function world(over: Partial<World> = {}): World { return { headroom: 0, running: [], settled: [], log: [], slept: 0, grown: [], reclaimed: [], clock: 0, held: 0, cancelled: false, hostFree: undefined, imageBytes: 80 * GiB, ...over }; }
function ports(state: World, over: Partial<StorageGatePortsV1> & Pick<StorageGatePortsV1, "predictedBytes">): StorageGatePortsV1 {
  return { pass: "release-00-b1", measured: true, reserveBytes: 2 * GiB, headroom: () => state.headroom, running: () => state.running, settled: () => state.settled, reconcile: () => undefined, hold: () => { state.held++; }, check: () => { if (state.cancelled) throw new Error("cancelled"); }, log: (line) => state.log.push(line), now: () => state.clock, sleep: async (ms) => { state.slept++; state.clock += ms; }, pollMilliseconds: 5, progressMilliseconds: 100, ...over };
}

test("the gate laws are the ones of the Python model for every state of a generated grid, and the spec vectors hold", () => {
  expect(storageChargeBytesV1({ predictedBytes: 100, writtenBytes: 30 })).toBe(70);
  expect(storageChargeBytesV1({ predictedBytes: 100, writtenBytes: 130 })).toBe(0);
  expect(storageGateV1({ headroomBytes: 10, reserveBytes: 2, predictedBytes: 8, running: [] })).toEqual({ admitted: true, neededBytes: 10, chargedBytes: 0, freeBytes: 10, shortBytes: 0 });
  expect(storageGateV1({ headroomBytes: 10, reserveBytes: 2, predictedBytes: 9, running: [{ id: "a", predictedBytes: 5, writtenBytes: 1 }] })).toEqual({ admitted: false, neededBytes: 11, chargedBytes: 4, freeBytes: 6, shortBytes: 5 });
  const cases: unknown[] = [];
  for (const headroom of [-5, 0, 7, 10, 40, 100]) for (const reserve of [0, 2]) for (const predicted of [0, 8, 30, 60]) for (const running of [[], [{ id: "a", predicted: 50, written: 0 }], [{ id: "a", predicted: 50, written: 50 }, { id: "b", predicted: 20, written: 30 }], [{ id: "a", predicted: 50, written: 20 }, { id: "b", predicted: 20, written: 5 }, { id: "c", predicted: 10, written: 10 }]]) cases.push({ kind: "gate", headroom, reserve, predicted, running });
  const gates = cases.length;
  for (const running of [0, 1, 3]) for (const hostFree of [undefined, 0, 50, 90, 150, 400]) for (const need of [1, 3, 20, 70]) for (const hostReserve of [0, 72]) cases.push({ kind: "grow", running, hostFree: hostFree === undefined ? null : hostFree, need, hostReserve, image: 80, align: 1, initial: 1 });
  const grows = cases.length - gates;
  for (const present of [0, 10, 70, 500]) cases.push({ kind: "credit", present: { release: present, debug: 5 }, rows: [{ lane: "release", bytes: 51 }, { lane: "release", bytes: 24 }, { lane: "debug", bytes: 3 }, { lane: "release", bytes: 9 }] });
  for (const rows of [null, [], [40], [30, 50], [10, 20, 30, 40, 100], [7, 7, 7]]) for (const measured of [true, false]) cases.push({ kind: "learned", rows, bytes: 99, measured });
  cases.push({ kind: "additions", present: ["a", "b"], identities: ["b", "c", "d"] }, { kind: "additions", present: [], identities: ["x"] });
  const expected = python(cases.map((row: any) => row.kind === "gate" ? { ...row, running: row.running.map((r: any) => ({ id: r.id, predicted: r.predicted, written: r.written })) } : row));
  expect(cases.length).toBeGreaterThan(280);
  cases.forEach((row: any, index) => {
    const answer = expected[index] as any;
    if (row.kind === "gate") expect(storageGateV1({ headroomBytes: row.headroom, reserveBytes: row.reserve, predictedBytes: row.predicted, running: row.running.map((r: any) => ({ id: r.id, predictedBytes: r.predicted, writtenBytes: r.written })) }), JSON.stringify(row)).toEqual(answer);
    else if (row.kind === "grow") expect(storageGrowDecisionV1({ needBytes: row.need, running: row.running, imageBytes: row.image, hostFreeBytes: row.hostFree ?? undefined, hostReserveBytes: row.hostReserve, alignBytes: row.align, initialBytes: row.initial }), JSON.stringify(row)).toEqual(answer);
    else if (row.kind === "credit") expect(creditPresentBytesV1(row.rows as { lane: string; bytes: number }[], (r) => r.lane, (lane) => row.present[lane] ?? 0).map((r) => ({ bytes: r.bytes, credit: r.credit })), JSON.stringify(row)).toEqual(answer);
    else if (row.kind === "learned") expect(learnedBatchBytesV1({ bytes: row.bytes, measured: row.measured }, row.rows ?? undefined), JSON.stringify(row)).toEqual(answer);
    else { const present = new Set<string>(row.present); expect({ added: [...batchAdditionsV1(new Set<string>(row.identities), present)].sort(), present: [...present].sort() }).toEqual(answer); }
  });
  expect(grows).toBe(144);
});

test("the play case: a second pass starts while the first runs because only what the first still has to write is charged, not its whole prediction", async () => {
  const state = world({ headroom: 47.7 * GiB, running: [{ id: "release-00-b0", predictedBytes: 51.3 * GiB, writtenBytes: 33.7 * GiB }] });
  const old = 47.7 * GiB - 51.3 * GiB;
  expect(old).toBeLessThan(24 * GiB);
  const gate = await admitPassStorageV1(ports(state, { predictedBytes: 24 * GiB }));
  expect(gate.admitted).toBe(true);
  expect(gate.chargedBytes).toBeCloseTo(17.6 * GiB, -6);
  expect(state.slept).toBe(0);
  expect(state.held).toBe(1);
  expect(state.log).toEqual([]);
});

test("written bytes are charged once: a pass that wrote its whole prediction charges nothing, one that wrote more never charges below zero", async () => {
  for (const written of [60 * GiB, 80 * GiB]) {
    const state = world({ headroom: 30 * GiB, running: [{ id: "release-00-b0", predictedBytes: 60 * GiB, writtenBytes: written }] });
    const gate = await admitPassStorageV1(ports(state, { predictedBytes: 20 * GiB }));
    expect(gate.chargedBytes).toBe(0);
    expect(gate.freeBytes).toBe(30 * GiB);
  }
});

test("a pass the room cannot hold parks, says needed, charged, free and what releases it in English and German, and resumes when the running pass settles", async () => {
  const state = world({ headroom: 30 * GiB, running: [{ id: "release-00-b0", predictedBytes: 50 * GiB, writtenBytes: 10 * GiB }] });
  let settle = (): void => undefined;
  state.settled = [new Promise<void>((accept) => { settle = accept; })];
  const waiting = admitPassStorageV1(ports(state, { predictedBytes: 8 * GiB, sleep: async (ms) => { state.slept++; state.clock += ms; await new Promise((accept) => setTimeout(accept, 2)); if (state.slept === 3) { state.running = []; state.headroom = 52 * GiB; state.settled = []; settle(); } } }));
  const gate = await waiting;
  expect(gate.admitted).toBe(true);
  expect(state.slept).toBeGreaterThanOrEqual(3);
  expect(state.held).toBe(1);
  expect(state.log.length).toBeGreaterThanOrEqual(1);
  const line = state.log[0]!;
  expect(line).toContain("release-00-b1 waits for storage: it needs 10240 MiB (8192 MiB still to write + 2048 MiB reserve), -10240 MiB are free after 40960 MiB are charged to 1 running pass(es)");
  expect(line).toContain("(10240 MiB so far)");
  expect(line).toContain("it starts when 20480 MiB more are free");
  expect(line).toContain("release-00-b1 wartet auf Speicher: es braucht 10240 MiB (8192 MiB noch zu schreiben + 2048 MiB Reserve)");
  expect(line).toContain("das Image wächst nur, solange kein Durchgang darauf läuft");
  expect(describeStorageGateV1("waiting", { pass: "p", needed: 1, predicted: 1, reserve: 1, free: 1, charged: 1, short: 1, running: 1, written: 1, unmeasured: 1 }, { key: "busy", values: {} })).toContain("unmeasured units estimated by kind");
});

test("the wait line is printed once per progress interval, not at every poll, and cancellation ends a parked wait", async () => {
  const state = world({ headroom: 5 * GiB, running: [{ id: "release-00-b0", predictedBytes: 50 * GiB, writtenBytes: 0 }] });
  state.settled = [new Promise<void>(() => undefined)];
  let sleeps = 0;
  const parked = admitPassStorageV1(ports(state, { predictedBytes: 8 * GiB, progressMilliseconds: 50, pollMilliseconds: 10, sleep: async (ms) => { sleeps++; state.clock += ms; await new Promise((accept) => setTimeout(accept, 1)); if (sleeps === 30) state.cancelled = true; } }));
  await expect(parked).rejects.toThrow("cancelled");
  expect(state.log.length).toBe(Math.floor(29 * 10 / 50) + 1);
  expect(state.held).toBe(0);
});

test("with nothing running the gate asks the collector once, then grows the image when the host keeps its reserve, and admits", async () => {
  const state = world({ headroom: 20 * GiB, hostFree: 400 * GiB });
  const grow = { facts: () => ({ imageBytes: state.imageBytes, hostFreeBytes: state.hostFree }), run: async (target: number) => { state.grown.push(target); state.headroom += target - state.imageBytes; state.imageBytes = target; } };
  const gate = await admitPassStorageV1(ports(state, { predictedBytes: 40 * GiB, reclaim: async ({ needBytes }) => { state.reclaimed.push(needBytes); }, grow }));
  expect(gate.admitted).toBe(true);
  expect(state.reclaimed).toEqual([22 * GiB]);
  expect(state.grown).toEqual([102 * GiB]);
  expect(state.held).toBe(1);
  expect(state.log.some((line) => line.includes("the image grew by 22528 MiB to 104448 MiB") && line.includes("das Image wuchs um 22528 MiB auf 104448 MiB"))).toBe(true);
  expect(state.log.some((line) => line.includes("short of its predicted footprint") && line.includes("fehlen zum vorhergesagten Platzbedarf"))).toBe(true);
});

test("the image never grows into the host reserve: a refusal names what the host would keep, in both languages", async () => {
  const state = world({ headroom: 20 * GiB, hostFree: 90 * GiB });
  const grow = { facts: () => ({ imageBytes: state.imageBytes, hostFreeBytes: state.hostFree }), run: async () => { state.grown.push(1); } };
  await expect(admitPassStorageV1(ports(state, { predictedBytes: 40 * GiB, grow }))).rejects.toThrow(/Native build storage refused: release-00-b1 needs 43008 MiB \(40960 MiB predicted \+ 2048 MiB reserve\), 20480 MiB remain after 0 MiB charged to 0 running pass\(es\) settled.*growing by 22528 MiB would leave the host 69568 MiB free, below its 73728 MiB reserve.*Speicher des Native-Builds abgelehnt.*unter seiner Reserve von 73728 MiB/su);
  expect(state.grown).toEqual([]);
  expect(state.held).toBe(0);
  expect(storageGrowDecisionV1({ needBytes: 1, running: 1, imageBytes: 80 * GiB, hostFreeBytes: 900 * GiB, hostReserveBytes: policy.growHostReserveBytes, alignBytes: policy.growAlignBytes, initialBytes: policy.growInitialBytes }).kind).toBe("busy");
  expect(storageGrowDecisionV1({ needBytes: 1, running: 0, imageBytes: 80 * GiB, hostFreeBytes: undefined, hostReserveBytes: policy.growHostReserveBytes, alignBytes: policy.growAlignBytes, initialBytes: policy.growInitialBytes }).kind).toBe("host-unknown");
});

test("a failed grow is stated and the pass is refused, the image is not assumed bigger", async () => {
  const state = world({ headroom: 20 * GiB, hostFree: 400 * GiB });
  const grow = { facts: () => ({ imageBytes: state.imageBytes, hostFreeBytes: state.hostFree }), run: async () => { throw new Error("hdiutil: resize failed\nsecond line"); } };
  await expect(admitPassStorageV1(ports(state, { predictedBytes: 40 * GiB, grow }))).rejects.toThrow(/the grow failed: hdiutil: resize failed.*das Wachsen schlug fehl: hdiutil: resize failed/su);
  expect(state.held).toBe(0);
});

test("with no grower the refusal still states why, and a pass the room holds is admitted at once without any wait line", async () => {
  const state = world({ headroom: 10 * GiB });
  await expect(admitPassStorageV1(ports(state, { predictedBytes: 40 * GiB }))).rejects.toThrow(/no image grow is available here.*hier ist kein Wachsen des Images verfügbar/su);
  const fits = world({ headroom: 50 * GiB });
  expect((await admitPassStorageV1(ports(fits, { predictedBytes: 40 * GiB }))).admitted).toBe(true);
  expect(fits.log).toEqual([]);
});

test("a later batch of the shared build directory adds only the units no earlier batch accounted: the increment is its own units, the shared dependencies are counted once", () => {
  const vector = JSON.parse(JSON.stringify(fixture.batches));
  const whole = componentBatchesV1(vector.pass, vector.graph, new Map(Object.entries(vector.histories.warm)) as never, 1_000_000, vector.footprints)[0]!;
  const split = componentBatchesV1(vector.pass, vector.graph, new Map(Object.entries(vector.histories.warm)) as never, 460, vector.footprints);
  expect(split.map((row) => row.bytes)).toEqual([450, 570]);
  const present = new Set<string>();
  const increments = componentBatchesV1(vector.pass, vector.graph, new Map(Object.entries(vector.histories.warm)) as never, 460, vector.footprints, present);
  expect(increments.map((row) => row.pass.id)).toEqual(["release-00-b0", "release-00-b1"]);
  expect(increments[0]!.bytes).toBe(450);
  expect(increments[1]!.bytes).toBe(whole.bytes - 450);
  expect(increments.reduce((sum, row) => sum + row.bytes, 0)).toBe(whole.bytes);
  expect(present.size).toBeGreaterThan(0);
});

test("the growth ledger learns what a batch really wrote and replaces the computed footprint once it holds enough measurements; torn lines and non-positive rows are ignored", () => {
  const root = scratch();
  try {
    const pass = { profile: "release", target: "wasm32-wasip2", packages: [{ name: "b" }, { name: "a" }] };
    const key = storageGrowthKeyV1(pass);
    expect(key).toBe("release|wasm32-wasip2|a,b");
    expect(readStorageGrowthV1(root).size).toBe(0);
    recordStorageGrowthV1(root, key, 0, 1);
    recordStorageGrowthV1(root, key, 30 * GiB, 2);
    writeFileSync(join(root, policy.ledgerFile), `{"v":1,"key":"${key}","bytes":${30 * GiB},"atMs":2}\n{"v":1,"key":"torn`, { flag: "w" });
    recordStorageGrowthV1(root, key, 36 * GiB, 3);
    const rows = readStorageGrowthV1(root).get(key)!;
    expect(rows).toEqual([30 * GiB]);
    recordStorageGrowthV1(root, key, 40 * GiB, 4);
    const both = readStorageGrowthV1(root).get(key)!;
    expect(learnedBatchBytesV1({ bytes: 51 * GiB, measured: false }, rows)).toEqual({ bytes: 51 * GiB, measured: false });
    expect(learnedBatchBytesV1({ bytes: 51 * GiB, measured: false }, both).measured).toBe(true);
    expect(learnedBatchBytesV1({ bytes: 51 * GiB, measured: false }, both).bytes).toBeLessThan(51 * GiB);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("bytes already on disk from the run a restart replaces are measured from the dependency directories of the profile only and credited against its batches in order", () => {
  const root = scratch();
  try {
    const deps = join(root, "wasm32-wasip2", "release", "deps"), unit = join(root, "wasm32-wasip2", "release", "build", "pkg-a", "0123abcd", "out"), other = join(root, "wasm32-wasip2", "debug", "deps"), incremental = join(root, "release", "incremental", "x", "deps"), host = join(root, "release", "build", "pkg-b", "89ef", "out");
    for (const directory of [deps, unit, other, incremental, host]) mkdirSync(directory, { recursive: true });
    writeFileSync(join(deps, "liba.rlib"), Buffer.alloc(3000));
    writeFileSync(join(deps, "libb.rlib"), Buffer.alloc(2000));
    writeFileSync(join(unit, "libc.rlib"), Buffer.alloc(4000));
    writeFileSync(join(host, "libd.rlib"), Buffer.alloc(500));
    writeFileSync(join(other, "libc.rlib"), Buffer.alloc(7000));
    writeFileSync(join(incremental, "junk"), Buffer.alloc(9000));
    expect(measurePresentDepsBytesV1([root], "release")).toBe(9500);
    expect(measurePresentDepsBytesV1([join(root, "wasm32-wasip2")], "release")).toBe(9000);
    expect(measurePresentDepsBytesV1([root], "dev")).toBe(7000);
    expect(measurePresentDepsBytesV1([join(root, "missing")], "release")).toBe(0);
    const rows = creditPresentBytesV1([{ bytes: 4000, lane: "release" }, { bytes: 8000, lane: "release" }], (row) => row.lane, (lane) => measurePresentDepsBytesV1([root], lane));
    expect(rows.map((row) => [row.bytes, row.credit])).toEqual([[0, 4000], [2500, 5500]]);
    const text = describeStorageGateV1("credit", { pass: "release-00-b0", credit: 34509, predicted: 52573, remaining: 18064 });
    expect(text).toContain("34509 MiB of its predicted 52573 MiB are already on disk");
    expect(text).toContain("liegen schon im gemeinsamen Build-Verzeichnis");
  } finally { rmSync(root, { recursive: true, force: true }); }
});
