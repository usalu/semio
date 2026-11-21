import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import { createSessionAppSwitchGateV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts";
import { admitProgramRetirementV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🚪️program-retirement/🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("./note-standalone-archive-focus.json", import.meta.url), "utf8"));
const shell = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost";

test("a ready same-app archive replaces the standalone session through the gate", async () => {
  using db = new Database(":memory:");
  const oracle = db.query(`SELECT result,
    CASE WHEN result = 'switched' THEN $candidate ELSE $predecessor END AS visible
    FROM (
      SELECT CASE WHEN NOT $replace THEN 'unchanged' WHEN NOT $quiet THEN 'draining'
        WHEN NOT $current THEN 'cancelled' ELSE 'switched' END AS result
    )`);
  for (const row of fixture.cases) {
    expect(oracle.get({ $replace: +row.replaceCurrent, $quiet: +row.quiet, $current: +row.candidateCurrent, $candidate: fixture.candidate, $predecessor: fixture.predecessor }))
      .toEqual({ result: row.result, visible: row.visible });
    const app = { id: "fixture#editor" };
    const previous = { pluginId: "fixture", app, instanceId: fixture.predecessor, viewState: {} };
    const gate = createSessionAppSwitchGateV1<typeof app, Record<string, unknown>>();
    const steps: string[] = [];
    let visible = previous.instanceId;
    const answer = await gate.run({
      session: previous,
      resolveApp: () => app,
      appId: (resolved) => resolved.id,
      quiesce: async () => ({ settled: row.quiet, pending: row.quiet ? 0 : 1 }),
      seal: () => {},
      unseal: () => {},
      createInstance: async () => {
        if (!row.candidateCurrent) throw new Error("cancelled");
        return fixture.candidate;
      },
      retire: async ({ instanceId }) => { expect(instanceId).toBe(fixture.predecessor); },
      defaultViewState: () => ({}),
      publish: ({ instanceId }) => { visible = instanceId; },
      seedLayout: () => {},
      refresh: async () => {},
      trace: (step) => { steps.push(step); },
    }, { pluginId: previous.pluginId, appId: app.id, replaceCurrent: row.replaceCurrent })
      .then(outcome => outcome.status, error => error.message);
    expect({ result: answer, visible }).toEqual({ result: row.result, visible: row.visible });
    if (row.result === "switched") expect(steps.filter(step => step !== "retire")).toEqual(["quiesce", "seal", "create", "retire-started", "publish", "seed", "refresh"]);
    if (row.result === "cancelled") expect(steps).toEqual(["quiesce", "seal", "create-failed", "unseal"]);
    if (row.result === "draining") expect(steps).toEqual(["quiesce"]);
    if (row.result === "unchanged") expect(steps).toEqual([]);
  }
});

test("archive import adopts the prepared standalone session and retains Studio focus", () => {
  const transfer = readFileSync(`${shell}/📤️document-transfer-tabs/🔶️scope/🟦️.tsx`, "utf8");
  const publication = readFileSync(`${shell}/🛫️spawned-session/🟠️switch-plugin-app/🟦️.ts`, "utf8");
  const importer = transfer.split("const importDocumentArchiveOnce")[1].split("const keepProgramDocument")[0];
  expect(importer).toContain("hostMode");
  expect(importer).toContain("focusSpawnedProgram");
  expect(importer).toContain("adoptPreparedPrimarySession");
  expect(importer).toContain("instanceId !== null && !published");
  expect(publication).toContain("handle !== prepared.handle");
  expect(publication).toContain("admitProgramRetirementRef.current(owners, current, prepared?.retirement)");
  const retirement = readFileSync(`${shell}/🗄️shell-uri-documents/🟠️close-document/🟦️.ts`, "utf8");
  expect(retirement).toContain("AbortSignal.any([abort.signal, work.signal])");
  expect(retirement).toContain("work?.persist ?? core.persistProgramDocumentRef.current");
  expect(importer).toContain("persistProgramDocument(owner, { signal, progress: task.progress })");
  expect(publication).toContain("documentInitialized: documentSource !== undefined || prepared !== undefined");
});

test("cancelling an adoption releases the predecessor while its save is pending", async () => {
  const transfer = new AbortController();
  const shellClose = new AbortController();
  let startSaving!: () => void;
  const saving = new Promise<void>(resolve => { startSaving = resolve; });
  let sealed = false;
  const pending = admitProgramRetirementV1({
    owners: [fixture.predecessor],
    signal: AbortSignal.any([shellClose.signal, transfer.signal]),
    current: () => true,
    quiesce: async () => true,
    seal: () => { sealed = true; return () => { sealed = false; }; },
    persist: () => { startSaving(); return new Promise(() => {}); },
    confirmDiscard: async () => false,
  });
  await saving;
  expect(sealed).toBe(true);
  transfer.abort();
  expect(await pending).toBeNull();
  expect(sealed).toBe(false);
});
