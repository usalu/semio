import assert from "node:assert/strict";
import { readFileSync, mkdtempSync, statSync, utimesSync, readdirSync, mkdirSync, symlinkSync, writeFileSync, existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { createRequire } from "node:module";

/** 🔏️ Validates deterministic digest publication, failure preservation and explicit Nx ownership. */
export async function testGeneratorInputReceipt(workspace: string, generated: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🔏️receipt/🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧬️schema/🔣️.json"), "utf8"));
  const validate = new (require("ajv"))().compile(schema), stable = require("fast-json-stable-stringify");
  const receipt = { version: 1, kind: fixture.kind, digest: fixture.initialDigest };
  assert.equal(validate(receipt), true, JSON.stringify(validate.errors));
  const { publishGeneratorInputReceipt } = await import("../../🟦️.ts");
  const root = mkdtempSync(join(generated, "generator-input-receipt-"));
  const first = publishGeneratorInputReceipt(root, fixture.output, fixture.kind, fixture.initialDigest);
  assert.equal(first.changed, true); assert.equal(first.path, join(root, fixture.output));
  assert.equal(readFileSync(first.path, "utf8"), stable(receipt) + "\n");
  utimesSync(first.path, fixture.mtime, fixture.mtime);
  assert.equal(publishGeneratorInputReceipt(root, fixture.output, fixture.kind, fixture.initialDigest).changed, false);
  assert.equal(statSync(first.path).mtimeMs, fixture.mtime * 1000);
  for (const digest of fixture.invalidDigests) {
    assert.equal(validate({ ...receipt, digest }), false);
    assert.throws(() => publishGeneratorInputReceipt(root, fixture.output, fixture.kind, digest), /digest/i);
    assert.equal(readFileSync(first.path, "utf8"), stable(receipt) + "\n");
  }
  assert.throws(() => publishGeneratorInputReceipt(root, "../outside.json", fixture.kind, fixture.initialDigest), /workspace/i);
  assert.equal(publishGeneratorInputReceipt(root, fixture.output, fixture.kind, fixture.changedDigest).changed, true);
  assert.equal(readFileSync(first.path, "utf8"), stable({ ...receipt, digest: fixture.changedDigest }) + "\n");
  assert.deepEqual(readdirSync(dirname(first.path)), ["🔣️.json"]);
  mkdirSync(join(root, "foreign")); writeFileSync(join(root, "foreign/source.json"), "foreign");
  symlinkSync(join(root, "foreign"), join(root, "alias"), process.platform === "win32" ? "junction" : "dir");
  assert.throws(() => publishGeneratorInputReceipt(root, "alias/source.json", fixture.kind, fixture.initialDigest), /owner/i);
  assert.equal(readFileSync(join(root, "foreign/source.json"), "utf8"), "foreign");
  const caching = resolve(import.meta.dir, "../../.."), policy = JSON.parse(readFileSync(join(caching, "🔣️policy.json"), "utf8"));
  const contract = policy.generatorInputs[fixture.kind], project = JSON.parse(readFileSync(join(caching, "📋️project.json"), "utf8"));
  assert.equal(contract.target, "repo:generator-inputs");
  assert.equal(project.targets["generator-inputs"].cache, false, "the receipt digests bytes its own cache key cannot name, so its producer re-digests on every run");
  assert.equal(policy.cachedExact.includes("generator-inputs"), false, "no policy may force the receipt producer back into the cache");
  assert.deepEqual(project.targets["generator-inputs"].outputs, [`{workspaceRoot}/${contract.output}`]);
  const { isDiscoverySkipDirectory } = await import(resolve(caching, "../🔍️discovery/🟦️.ts"));
  assert.equal(isDiscoverySkipDirectory(contract.output.split("/")[0]), true, "The digest must not become its own catalog input");
  const plugin = (await import(resolve(caching, "../🟨️.mjs"))).default;
  const nodes = await plugin.createNodesV2[1]([`${caching.slice(workspace.length + 1)}/📋️project.json`, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📋️project.json"], {}, { workspaceRoot: workspace });
  const projects: any[] = nodes.flatMap(([, result]: any) => Object.values(result.projects));
  assert.equal(projects.find((candidate) => candidate.name === "repo")?.targets["generator-inputs"].cache, false, "the resolved receipt producer stays uncached");
  const owner = projects.find((candidate) => candidate.name === "@semio-tech/plugin-registry");
  assert.ok(owner);
  for (const name of ["generate", "check"]) {
    const target = owner.targets[name];
    assert.ok(target.dependsOn.includes(contract.target), `${name} runs after the receipt producer`);
    assert.ok(target.inputs.some((input: any) => input.dependentTasksOutputFiles === contract.output), `${name} hashes the current receipt`);
    assert.ok(!target.inputs.some((input: any) => input.runtime?.includes("generator-inputs")), `${name} must not repeat discovery inside the hasher`);
  }
  await testGeneratorInputReceiptReplay(workspace, generated, fixture.replay);
  console.log("🔏️ Digest publication matches stable JSON/Ajv, preserves unchanged bytes and mtime, rejects invalid writes, re-digests on every run and keys its cached consumers PASS");
}

/** 🔁️ Third-party oracle for the receipt's Nx ownership: Nx itself, in throwaway git workspaces whose producer publishes
 * the real receipt of a tracked source into an ignored path and whose cached consumer hashes it only through
 * `dependentTasksOutputFiles`. A producer cached on its membership inputs must be seen replaying a receipt older than
 * the bytes it digests (the stale-catalog failure), and an uncached one must make the consumer replay exactly while the
 * source is unchanged and re-run once it changes. */
async function testGeneratorInputReceiptReplay(workspace: string, generated: string, replay: any): Promise<void> {
  const require = createRequire(import.meta.url);
  const shape = {
    type: "object", additionalProperties: false, required: ["ignored", "source", "catalog", "versions", "producers"],
    properties: {
      ignored: { type: "array", minItems: 1, items: { type: "string", minLength: 1 } },
      source: { type: "string", minLength: 1 }, catalog: { type: "string", minLength: 1 },
      versions: { type: "array", minItems: 2, items: { type: "string" } },
      producers: { type: "array", minItems: 2, items: { type: "object", additionalProperties: false, required: ["name", "cache", "steps"], properties: { name: { type: "string" }, cache: { type: "boolean" }, steps: { type: "array", minItems: 2, items: { type: "object", additionalProperties: false, required: ["edit", "runs", "version"], properties: { edit: { type: "boolean" }, runs: { type: "integer", minimum: 0 }, version: { type: "integer", minimum: 0 } } } } } } },
    },
  };
  const validate = new (require("ajv"))().compile(shape);
  assert.equal(validate(replay), true, JSON.stringify(validate.errors));
  const nx = require.resolve("nx/bin/nx.js", { paths: [workspace] }), receiptModule = resolve(import.meta.dir, "../../🟦️.ts");
  for (const producer of replay.producers) {
    const root = mkdtempSync(join(generated, "generator-input-replay-"));
    mkdirSync(join(root, dirname(replay.source)), { recursive: true });
    writeFileSync(join(root, ".gitignore"), replay.ignored.join("\n") + "\n");
    writeFileSync(join(root, "package.json"), JSON.stringify({ name: "receipt-replay", private: true }) + "\n");
    writeFileSync(join(root, "nx.json"), JSON.stringify({ namedInputs: { default: ["{projectRoot}/project.json"] } }) + "\n");
    writeFileSync(join(root, replay.source), replay.versions[0]);
    writeFileSync(join(root, "produce.ts"), `import { createHash } from "node:crypto";\nimport { readFileSync } from "node:fs";\nimport { publishGeneratorInputReceipt } from ${JSON.stringify(receiptModule)};\npublishGeneratorInputReceipt(process.cwd(), "receipts/registry/🔣️.json", "registry-catalog", createHash("sha256").update(readFileSync(${JSON.stringify(replay.source)})).digest("hex"));\n`);
    writeFileSync(join(root, "consume.ts"), `import { appendFileSync, copyFileSync, mkdirSync } from "node:fs";\nimport { dirname } from "node:path";\nmkdirSync(dirname(${JSON.stringify(replay.catalog)}), { recursive: true });\ncopyFileSync(${JSON.stringify(replay.source)}, ${JSON.stringify(replay.catalog)});\nappendFileSync("runs.log", "run\\n");\n`);
    writeFileSync(join(root, "project.json"), JSON.stringify({
      name: "receipt-replay",
      targets: {
        "generator-inputs": { executor: "nx:run-commands", cache: producer.cache, inputs: ["default"], outputs: ["{workspaceRoot}/receipts/registry/🔣️.json"], options: { command: `${JSON.stringify(process.execPath)} ./produce.ts` } },
        generate: { executor: "nx:run-commands", cache: true, dependsOn: ["generator-inputs"], inputs: [{ dependentTasksOutputFiles: "receipts/registry/🔣️.json" }], outputs: [`{workspaceRoot}/${dirname(replay.catalog)}`], options: { command: `${JSON.stringify(process.execPath)} ./consume.ts` } },
      },
    }, null, 2) + "\n");
    symlinkSync(join(workspace, "node_modules"), join(root, "node_modules"), process.platform === "win32" ? "junction" : "dir");
    for (const args of [["init", "-q"], ["add", "-A"]]) assert.equal(Bun.spawnSync(["git", ...args], { cwd: root }).exitCode, 0, `git ${args.join(" ")}`);
    const { NX_SKIP_NX_CACHE: _skip, ...inherited } = process.env;
    const env = { ...inherited, NX_DAEMON: "false", NX_NO_CLOUD: "true", NX_TUI: "false", NX_CACHE_DIRECTORY: join(root, ".nx/cache"), NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/workspace-data") };
    for (const [index, step] of producer.steps.entries()) {
      if (step.edit) writeFileSync(join(root, replay.source), replay.versions[1]);
      const child = Bun.spawnSync(["node", nx, "run", "receipt-replay:generate", "--outputStyle=static"], { cwd: root, env, stdout: "pipe", stderr: "pipe" });
      assert.equal(child.exitCode, 0, `${producer.name} step ${index}: ${child.stdout}${child.stderr}`);
      const runs = existsSync(join(root, "runs.log")) ? readFileSync(join(root, "runs.log"), "utf8").split("\n").filter(Boolean).length : 0;
      assert.equal(runs, step.runs, `${producer.name} step ${index}: consumer executions`);
      assert.equal(readFileSync(join(root, replay.catalog), "utf8"), replay.versions[step.version], `${producer.name} step ${index}: catalog bytes`);
    }
  }
}
