/** 🌱️ Replay law of the oracle seed classes (`oracle-seeds.mjs`): every file the oracle2-g huge-key runs named as undeclared (recorded in `evidence/oracle2-learned.json`) is either a by-design class, a member of the production closure or declared by the framework store inputs; and the workspace `Cargo.toml` of `dsl-derive-rs:generate-schema-index` is covered BY MEANING by its cargo resolution token (the file is read by the preparation, the identity moves with the parts of it the crate depends on).
 * Red without the seed classes: 30 + 1 files stay unclassified. Root: `KEYS5_ROOT` or the live checkout.
 * @see https://github.com/iarna/iarna-toml */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { createRequire } from "node:module";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join, normalize } from "node:path";
import { seedClassesV1 } from "../oracle-seeds.mjs";
import { globMatchesV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🟦️.ts";

const root = process.env.KEYS5_ROOT ?? "/Users/ueli/Documents/semio";
const evidence = JSON.parse(readFileSync(join(import.meta.dir, "../evidence/oracle2-learned.json"), "utf8")) as Record<string, { learned: string[] }>;
const production = new Set(readFileSync(join(import.meta.dir, "../evidence/print-test-viz.fresh-production-closure.txt"), "utf8").split("\n").filter((line) => line.startsWith("  + ")).map((line) => line.slice(4)));
const GENERATED = new Set(["🧰️framework/🔨️modules/🖼️assets/README.md"]);
const project = JSON.parse(readFileSync(join(root, "🧰️framework/📦️packages/🟦️typescript/📋️project.json"), "utf8"));
const storeInputs: string[] = project.targets["test-source-text"].inputs.filter((input: unknown) => typeof input === "string" && input.replace(/^!/u, "").includes("🏪️store"));
const bare = (pattern: string) => pattern.replace(/^!?\{workspaceRoot\}\//u, "");
const declaredByStore = (file: string) => storeInputs.some((p) => !p.startsWith("!") && globMatchesV1(bare(p), file)) && !storeInputs.some((p) => p.startsWith("!") && globMatchesV1(bare(p), file));
const DRS = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/Cargo.toml";

test("the recorded undeclared reads of the three huge keys are all classified once the seed classes are known", async () => {
  const seeds = await seedClassesV1({ root, cargoManifest: DRS });
  const seeded = new Set(Object.values(seeds).flat());
  const classified = (file: string, withSeeds: boolean) => (withSeeds && seeded.has(file)) || production.has(file) || GENERATED.has(file) || declaredByStore(file);
  const learned = Object.values(evidence).flatMap((run) => run.learned);
  expect(learned.length).toBe(31);
  expect(learned.filter((file) => !classified(file, false)).length).toBeGreaterThan(25);
  expect(learned.filter((file) => !classified(file, true))).toEqual([]);
  expect(seeded.has("📋️project.json")).toBe(true);
});

test("every learned package.json is a member of the dependency membership the nested bootstrap replays (independent reader: the workspaces lists of the root-level scope manifests)", async () => {
  const seeds = await seedClassesV1({ root });
  const membership = new Set([...seeds.membership, ...seeds.physical]);
  const learned = evidence.viz!.learned.filter((file) => file.endsWith("package.json"));
  expect(learned.length).toBe(27);
  expect(learned.filter((file) => !membership.has(file))).toEqual([]);
  const scopes = ["package.json", ...readdirSync(root, { withFileTypes: true }).filter((entry) => entry.isDirectory() && !entry.name.startsWith(".")).map((entry) => `${entry.name}/package.json`)].filter((file) => existsSync(join(root, file)) && Array.isArray((JSON.parse(readFileSync(join(root, file), "utf8")) as { workspaces?: unknown }).workspaces));
  expect(scopes.length).toBe(6);
  const direct = new Set<string>(scopes);
  for (const scope of scopes) {
    const base = scope.includes("/") ? scope.slice(0, scope.lastIndexOf("/")) : "";
    for (const entry of (JSON.parse(readFileSync(join(root, scope), "utf8")) as { workspaces: string[] }).workspaces) { const directory = normalize(join(base, entry)); if (existsSync(join(root, directory, "package.json"))) direct.add(`${directory}/package.json`); }
  }
  expect([...direct].filter((file) => !seeds.membership.includes(file))).toEqual([]);
  expect(seeds.membership.filter((file) => !direct.has(file))).toEqual([]);
  expect(seeds.physical.length).toBeGreaterThan(300);
  const { computeWorkspaces } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts"));
  const independent = new Set(seeds.physical);
  expect((computeWorkspaces(root) as string[]).map((directory) => `${directory}/package.json`).filter((file) => !independent.has(file))).toEqual([]);
});

test("the workspace Cargo.toml is read by the preparation and covered by meaning: its fixed parts and the lints a crate takes move the token parts, unrelated edits do not", async () => {
  const seeds = await seedClassesV1({ root, cargoManifest: DRS });
  expect(seeds.token).toContain("Cargo.lock");
  expect(seeds.token).toContain("rust-toolchain.toml");
  expect(seeds.token).toContain(".cargo/config.toml");
  expect(seeds.cargoWorkspace).toContain("🧰️framework/Cargo.toml");
  const R = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🦀️resolution");
  const { createCargoResolutionContextV1, cargoResolutionPartsV1, readCargoResolutionPolicyV1 } = await import(join(R, "🟦️.ts"));
  const policy = readCargoResolutionPolicyV1(JSON.parse(readFileSync(join(R, "🔣️.json"), "utf8")));
  const toml = createRequire(join(root, "package.json"))("@iarna/toml").parse;
  const sha = (buffer: Buffer | string) => createHash("sha256").update(buffer).digest("hex");
  const parts = async (file?: string, edit?: (text: string) => string) => {
    const read = (path: string): Buffer | null => { try { const buffer = readFileSync(join(root, path)); return edit && path === file ? Buffer.from(edit(buffer.toString("utf8"))) : buffer; } catch { return null; } };
    const host = { read: async (path: string) => { const b = read(path); return b === null ? null : { value: toml(b.toString("utf8")), sha: sha(b) }; }, digest: async (path: string) => { const b = read(path); return b === null ? null : sha(b); } };
    return cargoResolutionPartsV1(createCargoResolutionContextV1(host, policy), DRS, { tolerant: true });
  };
  const base = await parts();
  const moved = async (file: string, edit: (text: string) => string) => { const next = await parts(file, edit); return Object.entries(next).filter(([key, value]) => (base as Record<string, string>)[key] !== value).map(([key]) => key); };
  for (const file of ["Cargo.toml", "🧰️framework/Cargo.toml"]) {
    expect(seeds.token, file).toContain(file);
    expect(await moved(file, (text) => `${text}\n[profile.dev]\nopt-level = 1\n`), file).toEqual(["workspace"]);
    expect(await moved(file, (text) => text.replace("[workspace.lints.rust]", "[workspace.lints.rust]\nzzz_keys5_probe = \"allow\"")), file).toEqual(["workspace"]);
    expect(await moved(file, (text) => `${text}\n[workspace.dependencies.zzz-keys5-unused]\npath = \"nowhere\"\n`), file).toEqual([]);
    expect(await moved(file, (text) => `# keys5 comment\n${text}`), file).toEqual([]);
  }
  expect(await moved("🧰️framework/Cargo.toml", (text) => text.replace("cargo-features = [ \"trim-paths\" ]", "cargo-features = [ \"trim-paths\", \"profile-rustflags\" ]"))).toEqual(["workspace"]);
});
