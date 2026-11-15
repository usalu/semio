/** 🌊️ Contract: unrelated churn in the walked tree never restarts or refuses the ownership check; a moved directory is re-derived at most `maximumRederivations` times and then refused naming the first path and fact; a consumed manifest edit is refused at once naming its path and costs exactly one bounded re-do of the step. */
import { test, expect } from "bun:test";
import { mkdtempSync, mkdirSync, writeFileSync, rmSync, unlinkSync } from "node:fs";
import { join } from "node:path";
import * as ownership from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import { retryOnConcurrentEditV1 } from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";

const maximum = schema.$defs.FrontierReplayProtocol.const.maximumRederivations;
const put = (root: string, file: string, value: unknown): void => { const target = join(root, file); mkdirSync(join(target, ".."), { recursive: true }); writeFileSync(target, JSON.stringify(value)); };
const build = (): { container: string; root: string } => {
 const output = process.env.SEMIO_TEST_ARTIFACT_DIR; if (!output) throw Error("Owned churn artifacts required"); mkdirSync(output, { recursive: true });
 const container = mkdtempSync(join(output, "ch")), root = join(container, "r");
 put(container, "package.json", { name: "empty-parent", private: true, workspaces: [] }); mkdirSync(root);
 put(root, "package.json", { name: "churn", private: true, semio: { workspace: { schemaVersion: 1, members: ["members/**"], owners: ["*/package.json"] } }, workspaces: ["members/a"] });
 put(root, "members/a/package.json", { name: "churn-a", version: "1.0.0" }); mkdirSync(join(root, "members/a/src"), { recursive: true });
 return { container, root };
};
type Hook = (root: string, pass: number) => void;
async function run(root: string, hook: Hook, passes: number, onDerive?: () => void): Promise<{ events: string[]; moved: number }> {
 const stop = new AbortController(), events: string[] = []; let body = false, pass = 0, first = "", moved = 0;
 await ownership.withPublishedBunWorkspaceOwnershipV1(root, { kind: "all" }, { signal: stop.signal, frontierBatchPaths: 1, advance: async progress => {
  if (!body) return; events.push(progress.operation);
  if (progress.operation === "state" || progress.operation === "list") onDerive?.();
  if (progress.operation === "frontier" && progress.path === first && pass < passes) { pass++; moved++; hook(root, pass); }
 } }, async custody => { body = true; first = custody.observed().paths[0]!.path; hook(root, 0); await custody.recheck(); });
 return { events, moved };
}

test("1000 unrelated files created, edited and deleted during the check cause no re-derivation and no refusal", async () => {
 const { container, root } = build();
 try {
  let derivations = 0;
  const hook: Hook = (dir, pass) => { for (let index = 0; index < 1000; index++) { const file = join(dir, "members/a/src", "f" + (index % 50) + "-" + (index % 3) + ".ts"); writeFileSync(file, "x" + pass + index); if (index % 7 === 0) unlinkSync(file); } };
  const result = await run(root, hook, 3, () => derivations++);
  expect(result.moved).toBeGreaterThan(0);
  expect(derivations).toBe(0);
 } finally { rmSync(container, { recursive: true, force: true }); }
});

for (const k of [1, 2, 3]) test("an unrelated new directory in " + k + " consecutive replays is re-derived " + k + " times and accepted", async () => {
 const { container, root } = build();
 try {
  let derivations = 0, initial = 0;
  const hook: Hook = (dir, pass) => { if (pass > 0) mkdirSync(join(dir, "members/a/d" + pass), { recursive: true }); };
  const result = await run(root, hook, k, () => derivations++);
  initial = derivations;
  expect(result.moved).toBe(k);
  expect(initial).toBeGreaterThan(0);
 } finally { rmSync(container, { recursive: true, force: true }); }
});

test("a directory that moves in every replay is refused after the bound, naming the first moved path and the fact", async () => {
 const { container, root } = build();
 try {
  const hook: Hook = (dir, pass) => { if (pass > 0) mkdirSync(join(dir, "members/a/d" + pass), { recursive: true }); };
  const failure = await run(root, hook, maximum + 1).then(() => undefined, error => error);
  expect(failure?.code).toBe("SEMIO_CONCURRENT_EDIT");
  expect(failure?.message).toContain("Bun current ownership changed after refreshed continuation: ");
  expect(failure?.path).toBe(join(root, "members/a"));
  expect(failure?.message).toContain("; listing");
  expect(failure?.reason).toBe("roster");
 } finally { rmSync(container, { recursive: true, force: true }); }
});

test("a new package is refused at once naming its manifest and existence", async () => {
 const { container, root } = build();
 try {
  const hook: Hook = (dir, pass) => { if (pass === 1) put(dir, "members/late/package.json", { name: "late" }); };
  const failure = await run(root, hook, 1).then(() => undefined, error => error);
  expect(failure?.message).toContain("Bun current workspace membership changed during verification: members/late/package.json; existence");
  expect(failure?.path).toBe("members/late/package.json");
 } finally { rmSync(container, { recursive: true, force: true }); }
});

test("editing one consumed manifest costs exactly one bounded re-do of the step and the message names the path", async () => {
 const { container, root } = build();
 try {
  const events: { path: string; attempt: number }[] = []; let attempts = 0;
  await retryOnConcurrentEditV1("snapshot", async attempt => {
   attempts++;
   await run(root, (dir, pass) => { if (attempt === 1 && pass === 1) put(dir, "members/a/package.json", { name: "churn-a", version: "1.0.1" }); }, 1);
  }, { onRetry: event => events.push({ path: event.path, attempt: event.attempt }), sleep: async () => {} });
  expect(attempts).toBe(2);
  expect(events).toEqual([{ path: "members/a/package.json", attempt: 2 }]);
 } finally { rmSync(container, { recursive: true, force: true }); }
});
