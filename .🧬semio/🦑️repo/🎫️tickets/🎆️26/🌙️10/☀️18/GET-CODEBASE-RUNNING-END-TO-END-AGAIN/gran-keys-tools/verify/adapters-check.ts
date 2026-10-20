// 🔬 The nx and Bazel code paths hash the same resolution identity, and the live batch reader agrees with an independent TOML parser on real crates (read-only).
import {mkdtempSync, mkdirSync, writeFileSync, readFileSync, realpathSync} from "node:fs";
import {tmpdir} from "node:os";
import {join, dirname} from "node:path";
import {createHash} from "node:crypto";
const SB = process.env.GK_ROOT ?? "/private/tmp/gk/sb", L = SB + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", REAL = "/Users/ueli/Documents/semio";
const lock: any = await import(L + "/🏗️build/📥️inputs/🔒️lock/📥️read/🟨️.mjs");
const read: any = await import(L + "/🏗️build/📥️inputs/🧮️digest/📥️read/🟨️.mjs");
const engine: any = await import(L + "/🏗️build/📥️inputs/🦀️resolution/🟦️.ts");
const routing: any = await import(SB + "/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
const caller: any = await import(L + "/🏗️build/📥️caller/🟦️.ts");
const policy = engine.readCargoResolutionPolicyV1(JSON.parse(readFileSync(L + "/🏗️build/📥️inputs/🦀️resolution/🔣️.json", "utf8")));
const fixture = JSON.parse(readFileSync(L + "/🏗️build/📥️inputs/🦀️resolution/🧪️tests/🔣️.json", "utf8"));
const capabilities = caller.createNativeBuildCallerCapabilitiesV1(SB, ["run", "fixture:build"], "bazel", {SEMIO_BUILD_MODE: "prototype"});
const run = <T>(fn: (control: any) => Promise<T>) => routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({version: 1, owner: "gran-keys:check", maximumElapsedMilliseconds: 600000}, capabilities, Date.now()), (original: any) => fn(routing.createIssuedResourceControlV1(original, "source:resolution-check", 600000)));
const sha = (s: string) => createHash("sha256").update(s).digest("hex");
const out: Record<string, unknown> = {};

// 1. fixture workspace on disk: nx batch path == bazel plural path == engine over Bun.TOML + the lock digest
const root = realpathSync(mkdtempSync(join(tmpdir(), "gk-adapters-")));
for (const [path, text] of Object.entries<string>(fixture.files)) { mkdirSync(dirname(join(root, path)), {recursive: true}); writeFileSync(join(root, path), text); }
const versions: Record<string, string> = {a: "0.1.0", b: "0.2.0", c: "0.3.0", devonly: "0.1.0", d: "0.1.0"};
const lockText = (names: string[]) => "version = 4\n" + names.map((n) => `\n[[package]]\nname = "${n}"\nversion = "${versions[n]}"\n`).join("");
writeFileSync(join(root, "Cargo.lock"), lockText(["a", "b", "c", "devonly"]).replace('name = "a"\nversion = "0.1.0"\n', 'name = "a"\nversion = "0.1.0"\ndependencies = ["b"]\n'));
writeFileSync(join(root, "nested/Cargo.lock"), lockText(["d"]));
const manifests = ["a/Cargo.toml", "b/Cargo.toml", "c/Cargo.toml", "devonly/Cargo.toml", "nested/d/Cargo.toml"];
const bazel = await run((control) => lock.selectedCargoResolutionDigests(root, manifests, control)) as Map<string, string>;
const nx = new Map<string, string>();
await run((control) => read.withExactSourceReadBatchV1(root, control, async (batch: any) => { const ctx = lock.cargoResolutionContextInBatchV1(batch, root); for (const m of manifests) nx.set(m, await lock.cargoResolutionDigestInBatchV1(batch, root, m, ctx)); }));
const direct = async (m: string) => {
  const files = new Map<string, {value: any; sha: string} | null>();
  const get = (p: string) => { if (!files.has(p)) { try { const t = readFileSync(join(root, p), "utf8"); files.set(p, {value: Bun.TOML.parse(t), sha: sha(t)}); } catch { files.set(p, null); } } return files.get(p)!; };
  const ctx = engine.createCargoResolutionContextV1({read: async (p: string) => get(p), digest: async (p: string) => get(p)?.sha ?? null}, policy);
  return engine.composeCargoResolutionV1(await engine.cargoResolutionPartsV1(ctx, m));
};
out.fixture = {crates: manifests.length, nxEqualsBazel: manifests.every((m) => nx.get(m) === bazel.get(m)), equalsEngine: (await Promise.all(manifests.map(async (m) => (await direct(m)) === nx.get(m)))).every(Boolean), distinct: new Set(nx.values()).size};
// a comment-only edit of the root manifest and of the toolchain files moves nothing; an inherited lints edit moves exactly the crates that inherit
const before = new Map(nx);
writeFileSync(join(root, "Cargo.toml"), "# a new comment\n" + fixture.files["Cargo.toml"]);
writeFileSync(join(root, "rust-toolchain.toml"), "# other comment\n" + fixture.files["rust-toolchain.toml"]);
writeFileSync(join(root, ".cargo/config.toml"), "# other comment\n" + fixture.files[".cargo/config.toml"].replace("target-a", "target-z"));
const afterComment = await run((control) => lock.selectedCargoResolutionDigests(root, manifests, control)) as Map<string, string>;
writeFileSync(join(root, "Cargo.toml"), fixture.files["Cargo.toml"].replace('unused_lifetimes = "warn"', 'unused_lifetimes = "deny"'));
const afterLints = await run((control) => lock.selectedCargoResolutionDigests(root, manifests, control)) as Map<string, string>;
out.edits = {commentOnlyMoved: manifests.filter((m) => afterComment.get(m) !== before.get(m)), lintsMoved: manifests.filter((m) => afterLints.get(m) !== before.get(m))};
writeFileSync(join(root, "Cargo.toml"), fixture.files["Cargo.toml"]);
writeFileSync(join(root, "Cargo.lock"), readFileSync(join(root, "Cargo.lock"), "utf8") + '\n[[package]]\nname = "unrelated"\nversion = "9.9.9"\n');
const afterLock = await run((control) => lock.selectedCargoResolutionDigests(root, manifests, control)) as Map<string, string>;
out.unrelatedLockEntryMoved = manifests.filter((m) => afterLock.get(m) !== before.get(m));

// 2. real crates: the live batch reader (read-only) agrees with an independent parser
const model = JSON.parse(readFileSync("/private/tmp/gk/model2-sample.json", "utf8")) as string[];
const t0 = performance.now();
const real = await run((control) => lock.selectedCargoResolutionDigests(REAL, model.map((d) => (d ? d + "/Cargo.toml" : "Cargo.toml")), control)) as Map<string, string>;
const seconds = (performance.now() - t0) / 1000;
let agree = 0, lockAgree = 0; const lockDigest = (m: string) => run((control) => lock.selectedCargoLockDigest(REAL, m, control));
let disagreements: string[] = [];
const files = new Map<string, {value: any; sha: string} | null>();
const get = (p: string) => { if (!files.has(p)) { try { const t = readFileSync(join(REAL, p), "utf8"); files.set(p, {value: Bun.TOML.parse(t), sha: sha(t)}); } catch { files.set(p, null); } } return files.get(p)!; };
const ctx = engine.createCargoResolutionContextV1({read: async (p: string) => get(p), digest: async (p: string) => get(p)?.sha ?? null}, policy);
for (const d of model) {
  const m = d ? d + "/Cargo.toml" : "Cargo.toml";
  let oldLock: string; try { oldLock = (await lockDigest(m)) as string; } catch (e) { oldLock = "unresolved:" + (e instanceof Error ? e.message : String(e)); }
  const parts = await engine.cargoResolutionPartsV1(ctx, m, {tolerant: true});
  if (parts.lock === oldLock) lockAgree++;
  if (engine.composeCargoResolutionV1(parts) === real.get(m)) agree++; else disagreements.push(m);
}
out.real = {crates: model.length, agree, lockEqualsLegacyDigest: lockAgree, seconds, disagreements: disagreements.slice(0, 5)};
console.log(JSON.stringify(out, null, 1));
