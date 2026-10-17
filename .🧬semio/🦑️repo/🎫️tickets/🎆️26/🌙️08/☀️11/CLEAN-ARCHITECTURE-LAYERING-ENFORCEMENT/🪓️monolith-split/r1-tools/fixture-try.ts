import { mkdirSync, mkdtempSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { carveWrites, planArtifactCrates } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const files: Record<string,string> = {
 "Cargo.toml": '[workspace]\nresolver = "2"\nmembers = [ "📦️packages/🦀️rust" ]\n\n[workspace.package]\nversion = "0.1.0"\nedition = "2021"\nrust-version = "1.95"\n',
 "Cargo.lock": 'version = 4\n\n[[package]]\nname = "demo-det"\nversion = "0.1.0"\n',
 "📦️packages/🦀️rust/Cargo.toml": '[package]\nworkspace = "../.."\nname = "demo-det"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n',
 "📦️packages/🦀️rust/📜️script.ts": 'await runArtifactRustPackageMain(import.meta.dir, "demo-det",{snapshotSqliteTests:[]});\n',
 "🦀️.rs": '//! root\n#[path = "."]\npub mod io {\n    #[path = "🚪️io/🦀️.rs"]\n    mod component;\n    pub use component::*;\n}\n',
 "🚪️io/🦀️.rs": "//! io\npub fn helper() -> u8 { 1 }\n#[path = \"📝️text/🦀️.rs\"]\npub mod text;\n",
 "🚪️io/📝️text/🦀️.rs": "//! text\npub fn helper_text() -> u8 { 2 }\n#[cfg(test)]\n#[path = \"🧪️tests/🦀️.rs\"]\nmod tests;\n",
 "🚪️io/📝️text/🧪️tests/🦀️.rs": "use super::*;\nuse crate::io::helper;\n\n#[test]\nfn text_helper_is_two() { assert_eq!(helper_text() + helper(), 3); }\n",
};
const root = mkdtempSync(join(tmpdir(), "semio-det-"));
for (const [p, c] of Object.entries(files)) { mkdirSync(dirname(join(root, p)), { recursive: true }); writeFileSync(join(root, p), c); }
const tiny = { target: { files: 1, bytes: 230000, lines: 3500 }, hard: { files: 32, bytes: 262144, lines: 4000 } };
const plan = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...tiny, tests: { fixtures: false, aggregate: false, hoist: true, detach: true } } as any);
console.log("problems", plan.problems);
for (const c of plan.crates) console.log(c.name, c.attach.map((d: any) => d.eff.join("::")).join(" + "), "| slices:", c.slices.flatMap((s: any) => s.decls.map((d: any) => d.eff.join("::"))).join(","));
const { writes } = await carveWrites(plan);
for (const w of writes) if (w.path.includes("mirror") || w.path.endsWith("📦️packages/🦀️rust/🦀️.rs")) { console.log("== " + w.path); console.log(w.content); }
rmSync(root, { recursive: true, force: true });
