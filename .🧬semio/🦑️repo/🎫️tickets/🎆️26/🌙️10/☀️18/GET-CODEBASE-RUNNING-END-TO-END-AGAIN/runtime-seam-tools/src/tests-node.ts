import { mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { GlobV1, SqliteDatabaseV1, runtimeIdentityV1, runtimeParseJsoncV1, runtimeParseTomlV1, runtimeResolveV1, runtimeSemverSatisfiesV1, runtimeServeV1, runtimeSpawnSyncV1, runtimeWhichV1, runtimeSupportsV1, isMainModuleV1, runtimeFileV1 } from "../../🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };

const failures: string[] = [];
const expectEqual = (label: string, actual: unknown, expected: unknown): void => { if (JSON.stringify(actual) !== JSON.stringify(expected)) failures.push(`${label}: ${JSON.stringify(actual)} != ${JSON.stringify(expected)}`); };

for (const [pattern, path, expected] of fixture.glob as [string, string, boolean][]) expectEqual(`glob ${pattern} ${path}`, new GlobV1(pattern).match(path), expected);
for (const [id, text, expected] of fixture.tomlValid as [string, string, unknown][]) expectEqual(`toml ${id}`, JSON.parse(JSON.stringify(runtimeParseTomlV1(text))), expected);
for (const [id, text] of fixture.tomlInvalid as [string, string][]) { try { runtimeParseTomlV1(text); failures.push(`toml-invalid ${id}`); } catch { /* expected */ } }
for (const [id, text, expected] of fixture.jsonc as [string, string, unknown][]) expectEqual(`jsonc ${id}`, runtimeParseJsoncV1(text), expected);
expectEqual("semver", [runtimeSemverSatisfiesV1("1.2.3", "^1.0.0"), runtimeSemverSatisfiesV1("2.0.0", "^1.0.0")], [true, false]);
expectEqual("which", [runtimeWhichV1("no-such-program-semio"), typeof runtimeWhichV1("node")], [null, "string"]);
expectEqual("spawn", runtimeSpawnSyncV1([process.execPath, "-e", "process.stdout.write('ok')"]).stdout.toString(), "ok");
expectEqual("support", [runtimeSupportsV1("glob"), runtimeSupportsV1("ffi")], [true, false]);
expectEqual("main", isMainModuleV1(import.meta.url), true);
const database = new SqliteDatabaseV1(":memory:");
database.exec("create table t(a)"); database.run("insert into t values (?)", 1);
expectEqual("sqlite", [database.query("select a from t").get(), database.query("select a from t where a = 2").get()], [{ a: 1 }, null]);
database.close();
const root = mkdtempSync(join(tmpdir(), "runtime-seam-node-"));
try {
  mkdirSync(join(root, "a"), { recursive: true }); writeFileSync(join(root, "a/x.ts"), "export {}");
  expectEqual("scan", [...new GlobV1("**/*.ts").scanSync({ cwd: root })], ["a/x.ts"]);
  expectEqual("resolve", runtimeResolveV1("./x.ts", join(root, "a")), realpathSync(join(root, "a/x.ts")));
  const server = await runtimeServeV1({ hostname: "127.0.0.1", port: 0, fetch: request => new Response(`${request.method} ${new URL(request.url).pathname}`) });
  expectEqual("serve", await (await fetch(`http://127.0.0.1:${server.port}/p`)).text(), "GET /p");
  await server.stop(true);
  expectEqual("file", await runtimeFileV1(join(root, "a/x.ts")).text(), "export {}");
} finally { rmSync(root, { recursive: true, force: true }); }
console.log(JSON.stringify({ runtime: runtimeIdentityV1().id, failures }));
process.exitCode = failures.length === 0 ? 0 : 1;
