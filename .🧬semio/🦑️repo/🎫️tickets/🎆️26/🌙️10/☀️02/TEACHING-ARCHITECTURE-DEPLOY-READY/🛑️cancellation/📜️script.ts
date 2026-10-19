import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
const root = process.cwd();
const product = resolve(root, "🧰️framework/🛍️products/📚️course");
const modules = resolve(product, "🎯️targets/⚛️react/🔨️modules");
const moduleOf = (path: string) => import(pathToFileURL(resolve(modules, path, "🟦️.ts")).href);
const { ProctorLink } = await moduleOf("🧭️session/📡️proctor-link");
const { CourseSession } = await moduleOf("🧭️session");
const { Deputy } = await moduleOf("🫡️deputy");
const { ProctorClient, ProctorUnavailable } = await moduleOf("🛂️proctor");
const { localStore, memoryStorageOrigin } = await moduleOf("💾️persistence");
const { ARCHITECTURE_COURSE_MATERIAL: material } = await import(pathToFileURL(resolve(root, "🎓️teaching/🏛️architecture/📚️course/📚️course/🟦️.ts")).href);
const fixture = JSON.parse(readFileSync(resolve(product, "🧫️fixtures/🛑️cancellation/🔣️.json"), "utf8"));
const oracle = Bun.spawnSync(["node", "-e", 'const c=new AbortController();const reason=new Error("cancel");c.abort(reason);fetch("data:text/plain,committed",{signal:c.signal}).then(()=>{console.log(JSON.stringify({cancelled:false}));process.exitCode=1},e=>console.log(JSON.stringify({cancelled:e===reason})))']);
assert.equal(oracle.exitCode, 0, oracle.stderr.toString());
assert.deepEqual(JSON.parse(oracle.stdout.toString()), { cancelled: fixture.admission.cancelled });
console.log("[DEBUG] independent Node/Undici pre-aborted request oracle PASS");
const failures: string[] = [];
const check = async (name: string, action: () => Promise<void>) => {
  try { await action(); console.log(`[DEBUG] ${name} PASS`); }
  catch (error) { failures.push(name); console.log(`[DEBUG] ${name} FAIL`, error); }
};
await check("patience does not admit an already cancelled request", async () => {
  const controller = new AbortController();
  const reason = new Error("cancelled before the request");
  controller.abort(reason);
  const link = new ProctorLink({ silent() {} }, { minMs: 1, maxMs: 1 }, 1000, Date.now);
  let calls = 0;
  await assert.rejects(link.patient(async (signal: AbortSignal) => { calls++; signal.throwIfAborted(); return "committed"; }, controller.signal), error => error === reason);
  assert.equal(calls, fixture.admission.operationCalls);
});
for (const vector of fixture.scenarios) await check(`${vector.command} cancelled ${vector.cancelAt} preserves the actual offline session and journal`, async () => {
  const proctor = new ProctorClient(() => ({ send: async () => { throw new ProctorUnavailable("offline probe"); } }), material.course.id);
  proctor.silent();
  const area = memoryStorageOrigin().tab();
  const session = new CourseSession({ proctor, store: localStore(area, material.course.id), deputy: new Deputy(material), timing: { minMs: 100, maxMs: 100 } });
  session.start();
  try {
    const signal = new AbortController().signal;
    assert.equal(await session.identify({ kind: "anonymous" }, signal), undefined);
    assert.equal(await session.startRun(material.quizzes[0].id, "medium", signal), undefined);
    const { step, runs } = session.getSnapshot().state;
    assert.equal(step.screen, "run");
    const task = runs[step.run].sheet.tasks[0];
    assert.equal(task.kind, "classification");
    session.answer(step.run, task.id, { kind: "classification", assignments: Object.fromEntries(task.items.map((item: any) => [item.id, task.categories[0].id])) });
    const queued = session.outbox.queued();
    const persisted = Object.fromEntries(area.keys().map((key: string) => [key, area.get(key)]));
    const controller = new AbortController();
    const reason = new Error("the learner cancelled");
    if (vector.cancelAt === "before") controller.abort(reason);
    const operation = vector.command === "start-run" ? session.startRun(material.quizzes[0].id, "expert", controller.signal) : session.submit(step.run, controller.signal, (phase: any) => { if (phase.phase === vector.cancelAt) controller.abort(reason); });
    const outcome = await operation.then((value: unknown) => ({ value }), (error: unknown) => ({ error }));
    console.log("[DEBUG] session cancellation outcome", { vector, cancelled: "error" in outcome && outcome.error === reason, commandsBefore: queued.length, commandsAfter: session.outbox.queued().length, status: session.getSnapshot().state.runs[step.run].status });
    assert.deepEqual(outcome, { error: reason });
    assert.equal(session.getSnapshot().state.runs[step.run].status, vector.runStatus);
    assert.equal(session.outbox.queued().length, queued.length + vector.addedCommands);
    assert.deepEqual(session.outbox.queued(), queued);
    assert.deepEqual(Object.fromEntries(area.keys().map((key: string) => [key, area.get(key)])), persisted);
  } finally { session.stop(); }
});
assert.deepEqual(failures, []);
console.log("[DEBUG] direct cancellation regression PASS; managed React and browser gates remain pending");
