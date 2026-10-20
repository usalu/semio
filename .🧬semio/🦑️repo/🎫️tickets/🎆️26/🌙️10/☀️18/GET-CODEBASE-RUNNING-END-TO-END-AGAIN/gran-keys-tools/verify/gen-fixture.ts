import {readFileSync, writeFileSync} from "node:fs";
import {createCargoResolutionContextV1, crateIdentityV1, toolchainIdentityV1, readCargoResolutionPolicyV1, canonicalJsonV1, composeCargoResolutionV1, workspaceKeyViolationV1, CargoResolutionErrorV1} from "/private/tmp/gk/ov/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🦀️resolution/🟦️.ts";
const hostOf = (copy: Record<string, any>) => ({read: async (p: string) => (copy[p] ? {value: copy[p], sha: canonicalJsonV1(copy[p]).length.toString(16).padStart(64, "0")} : null), digest: async (p: string) => (copy[p] ? canonicalJsonV1(copy[p]).length.toString(16).padStart(64, "0") : null)});
const policy = readCargoResolutionPolicyV1(JSON.parse(readFileSync("/private/tmp/gk/policy.json", "utf8")));
const files: Record<string, string> = JSON.parse(readFileSync("/private/tmp/gk/fx/files.json", "utf8"));
const trees = Object.fromEntries(Object.entries(files).map(([p, t]) => [p, Bun.TOML.parse(t)]));
const crates = ["a", "b", "c", "devonly", "nested/d"];
const queries = crates.flatMap((c) => [[c, false], [c, true]] as [string, boolean][]);
const key = ([c, t]: [string, boolean]) => `${c}:${t ? "t" : "b"}`;
const applySet = (tree: any, path: (string | number)[], value: unknown) => { let n = tree; for (const p of path.slice(0, -1)) n = n[p]; if (value === null) delete n[path.at(-1)!]; else if (Array.isArray(value) && (value as any[])[0] === "$push") n[path.at(-1)!].push(...(value as any[]).slice(1)); else n[path.at(-1)!] = value; };
const run = async (edit?: {path: string; set: [(string | number)[], unknown][]}, extra?: Record<string, any>) => {
  const copy: Record<string, any> = structuredClone(extra ?? trees);
  if (edit) for (const [p, v] of edit.set) applySet(copy[edit.path], p, v);
  const ctx = createCargoResolutionContextV1(hostOf(copy), policy);
  const ids: Record<string, string> = {};
  for (const q of queries) ids[key(q)] = await crateIdentityV1(ctx, q[0], q[1]);
  return {ids, tool: await toolchainIdentityV1(ctx)};
};
const base = await run();
const edits = [
  {id: "members-append", path: "Cargo.toml", set: [[["workspace", "members"], ["$push", "e"]]], note: "membership is not part of any key"},
  {id: "exclude-edit", path: "Cargo.toml", set: [[["workspace", "exclude"], ["other"]]], note: "exclusion is not part of any key"},
  {id: "lints-edit", path: "Cargo.toml", set: [[["workspace", "lints", "rust", "unused_lifetimes"], "deny"]], note: "only crates with [lints] workspace = true"},
  {id: "package-edition-edit", path: "Cargo.toml", set: [[["workspace", "package", "edition"], "2024"]], note: "only crates that inherit edition"},
  {id: "package-rust-version-edit", path: "Cargo.toml", set: [[["workspace", "package", "rust-version"], "1.96"]], note: "no crate inherits rust-version"},
  {id: "dependency-serde-edit", path: "Cargo.toml", set: [[["workspace", "dependencies", "serde", "version"], "2"]], note: "crates that inherit serde and everything compiling with them"},
  {id: "dependency-tracing-edit", path: "Cargo.toml", set: [[["workspace", "dependencies", "tracing"], "0.2"]], note: "only devonly and crates compiling with it, c only as a test root"},
  {id: "profile-add", path: "Cargo.toml", set: [[["profile"], {dev: {"opt-level": 1}}]], note: "every crate of the workspace and every crate compiling with them"},
  {id: "features-edit", path: "Cargo.toml", set: [[["cargo-features"], ["edition2024", "x"]]], note: "gates apply to the whole workspace"},
  {id: "nested-edition-edit", path: "nested/Cargo.toml", set: [[["workspace", "package", "edition"], "2027"]], note: "crates of the nested workspace and crates compiling with them"},
  {id: "nested-profile-edit", path: "nested/Cargo.toml", set: [[["profile", "dev", "opt-level"], 3]], note: "the nested workspace's profile reaches d and b"},
  {id: "member-dependency-added", path: "a/Cargo.toml", set: [[["dependencies", "extra"], {path: "../c"}]], note: "a's own manifest is a file input; its closure keeps c"},
];
const results = [];
for (const e of edits) {
  const after = await run(e as any);
  const changed = queries.map(key).filter((k) => after.ids[k] !== base.ids[k]);
  results.push({id: e.id, path: e.path, set: e.set, note: e.note, changed, toolChanged: after.tool.toolchain !== base.tool.toolchain || after.tool.config !== base.tool.config});
}
const toolEdits = [
  {id: "pin-comment", path: "rust-toolchain.toml", text: files["rust-toolchain.toml"]!.replace("# pinned", "# pinned twice, reordered below").replace('"rust-src", "llvm-tools-preview"', '"llvm-tools-preview", "rust-src"'), changed: false},
  {id: "pin-channel", path: "rust-toolchain.toml", text: files["rust-toolchain.toml"]!.replace("2026-07-20", "2026-08-01"), changed: true},
  {id: "config-comment", path: ".cargo/config.toml", text: files[".cargo/config.toml"]!.replace("# configuration", "# a different comment"), changed: false},
  {id: "config-target-dir", path: ".cargo/config.toml", text: files[".cargo/config.toml"]!.replace("target-a", "target-b").replace("build-a", "build-b"), changed: false},
  {id: "config-flags", path: ".cargo/config.toml", text: files[".cargo/config.toml"]!.replace('"threads=0"', '"threads=4"'), changed: true},
];
for (const t of toolEdits) {
  const copy: Record<string, any> = structuredClone(trees);
  copy[t.path] = Bun.TOML.parse(t.text);
  const after = await toolchainIdentityV1(createCargoResolutionContextV1(hostOf(copy), policy));
  const moved = after.toolchain !== base.tool.toolchain || after.config !== base.tool.config;
  if (moved !== t.changed) throw new Error("tool edit " + t.id);
}
const cycleFiles = {"Cargo.toml": "[workspace]\nmembers = [\"x\", \"y\"]\n", "x/Cargo.toml": "[package]\nname = \"x\"\n[dependencies]\ny = { path = \"../y\" }\n", "y/Cargo.toml": "[package]\nname = \"y\"\n[dependencies]\nx = { path = \"../x\" }\n"};
const escapeFiles = {"Cargo.toml": "[workspace]\nmembers = [\"x\"]\n", "x/Cargo.toml": "[package]\nname = \"x\"\n[dependencies]\nz = { path = \"../../../outside\" }\n"};
const missingWorkspaceFiles = {"x/Cargo.toml": "[package]\nname = \"x\"\nworkspace = \"..\"\n"};
const withTrees = (row: {id: string; files: Record<string, string>; crate: string; tests: boolean; error: string}) => ({...row, trees: Object.fromEntries(Object.entries(row.files).map(([p, t]) => [p, Bun.TOML.parse(t)]))});
const refusals = [
  {id: "path-cycle", files: cycleFiles, crate: "x", tests: false, error: "cycle"},
  {id: "path-escape", files: escapeFiles, crate: "x", tests: false, error: "escape"},
  {id: "explicit-workspace-without-table", files: missingWorkspaceFiles, crate: "x", tests: false, error: "workspace"},
].map(withTrees);
const canonical = [
  {value: {b: 1, a: [2, {d: 1, c: null}], "é": "x", "z": {"": 0}}, text: '{"a":[2,{"c":null,"d":1}],"b":1,"z":{"":0},"é":"x"}'},
  {value: {"🦀": 1, "a": 2, "￿": 3}, text: '{"a":2,"￿":3,"🦀":1}'},
  {value: [], text: "[]"},
  {value: "tab\t\"quote\"", text: '"tab\\t\\"quote\\""'},
];
const keys = [
  "{workspaceRoot}/Cargo.toml", "{workspaceRoot}/Cargo.lock", "{workspaceRoot}/rust-toolchain.toml", "{workspaceRoot}/.cargo/config.toml", "{workspaceRoot}/./Cargo.lock",
  "{workspaceRoot}/**/Cargo.toml", "{workspaceRoot}/**/Cargo.lock", "{workspaceRoot}/**/*.rs", "{workspaceRoot}/**/🧬️schema/**/*", "{workspaceRoot}/**/{Cargo.toml,📋️project.json}",
  "{workspaceRoot}/crates/a/Cargo.toml", "{workspaceRoot}/🌎️hub/**/*.rs", "{workspaceRoot}/🧰️framework/Cargo.toml", "{projectRoot}/Cargo.toml", "!{workspaceRoot}/**/Cargo.toml", "{workspaceRoot}/rustfmt.toml", "{workspaceRoot}/**/🧬️schema/mutations/🔣️.json",
].map((input) => ({input, violation: workspaceKeyViolationV1(policy, input) ?? null}));
const exemptCases = [{command: "bun x native cargo metadata --manifest a/Cargo.toml", inputs: ["{workspaceRoot}/**/Cargo.lock", "{workspaceRoot}/Cargo.toml"], violations: []}, {command: "bun x native cargo check --manifest a/Cargo.toml", inputs: ["{workspaceRoot}/**/Cargo.lock", "{workspaceRoot}/Cargo.toml", "{projectRoot}/**/*"], violations: ["**/Cargo.lock", "Cargo.toml"]}];
const out = {version: 1, files, trees, canonical, crates, queries: queries.map(key), edits: results, toolEdits: toolEdits.map(({id, path, text, changed}) => ({id, path, text, changed})), refusals, keys, exemptCases, expect: {base: base.ids, tool: base.tool, composed: composeCargoResolutionV1({lock: "L", workspace: base.ids["a:t"]!, toolchain: base.tool.toolchain, config: base.tool.config})}};
writeFileSync("/private/tmp/gk/fixture.json", JSON.stringify(out, null, 1) + "\n");
console.log(JSON.stringify(results.map((r) => [r.id, r.changed.join(" "), r.toolChanged])));
