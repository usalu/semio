import { mkdirSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { spawnSync } from "node:child_process";
import assert from "node:assert/strict";
import Ajv from "ajv/dist/2020.js";

const policy = "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/👥️audience/";
const read = name => JSON.parse(readFileSync(policy + name, "utf8"));
const contract = read("🔣️.json");
const validate = new Ajv().compile(read("🧬️schema.json"));
assert.equal(validate(contract), true, JSON.stringify(validate.errors));
assert.equal(validate({ ...contract, variants: [...contract.variants].reverse() }), false);
assert.equal(validate({ ...contract, variants: [{ id: "spectator", subtitles: "overlay" }] }), false);
const cases = read("🧪️tests/🔣️.cases.json");
assert.deepEqual(Object.fromEntries(contract.variants.map(variant => [variant.id, variant.subtitles === "overlay"])), cases.subtitles);
console.log("[DEBUG] AJV audience contract and independent subtitle policy checks passed.");

const generated = resolve(import.meta.dirname, "🗑️generated");
mkdirSync(generated, { recursive: true });
const extra = process.argv.slice(2);
const tests = extra.length ? extra : ["🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.👥️audience.🧪️tests.🐍️"];
const args = ["tools", "exec", "uv", "--", "run", "--project", "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python", "--python", "3.12", "--group", "test", "python", "-m", "unittest", ...tests, "-v"];
const env = { ...process.env, PYTHONUTF8: "1", PYTHONIOENCODING: "utf-8", PYTHONDONTWRITEBYTECODE: "1", SEMIO_TEST_MANIM: "1", SEMIO_TEACHING_OUTPUT_ROOT: generated, SEMIO_TEACHING_CACHE_DIR: resolve(generated, "fonts"), UV_PROJECT_ENVIRONMENT: resolve(".🧬semio/🎓️teaching/⚡️cache/python") };
delete env.PYTHONPYCACHEPREFIX;
const result = spawnSync("repo", args, { env, stdio: "inherit", windowsHide: true });
if (result.error) throw result.error;
process.exitCode = result.status ?? 1;
