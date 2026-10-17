// 🧫️ Sensitivity self-test: builds a synthetic two-crate workspace, runs the scanner on a clean tree (expects 0 hits) and on a seeded tree (expects every seeded reference class, and none of the decoys).
import {mkdirSync, rmSync, writeFileSync} from "node:fs";
import {dirname, join} from "node:path";
import {execFileSync} from "node:child_process";

const HERE = import.meta.dir;
const BASE = join(HERE, "selftest-work");
rmSync(BASE, {recursive: true, force: true});

const put = (root: string, path: string, text: string) => { const full = join(root, path); mkdirSync(dirname(full), {recursive: true}); writeFileSync(full, text); };

const owner = `
pub mod real { pub fn ok() {} pub fn fixture() {} }
#[cfg(test)] pub fn fixture() {}
#[cfg(test)] pub struct DemoSnap;
#[cfg(test)] pub mod fixtures { pub fn make() {} pub const K: u8 = 1; }
#[cfg(all(test, feature = "x"))] pub fn all_gated() {}
#[cfg(any(test, feature = "testkit"))] pub fn kit() {}
#[cfg(test)] pub use real::ok as ok_alias;
#[cfg(test)] #[macro_export] macro_rules! tm { () => {} }
pub struct S;
impl S { #[cfg(test)] pub fn only_test(&self) {} pub fn ok(&self) {} }
#[cfg(test)] pub mod hidden_file;
#[cfg(not(test))] pub fn prod_only() {}
#[cfg(test)] mod tests { pub fn private_mod_item() {} }
`;
const consumerSeeded = `
use ctc_owner::fixture;
use ctc_owner::fixtures::{make, K};
use ctc_owner::DemoSnap as D;
use ctc_owner::fixtures;
use renamed_owner::hidden_file::deep;
use ctc_owner::real::ok;
use ctc_owner::*;
pub fn go() {
    fixtures::make();
    ctc_owner::fixtures::make();
    ctc_owner::tm!();
    ctc_owner::S::only_test(&ctc_owner::S);
    let s = ctc_owner::S;
    s.only_test();
    ctc_owner::all_gated();
    ctc_owner::kit();
    ctc_owner::ok_alias();
    ctc_owner::real::ok();
    s.ok();
    ok();
    kit();
    let _ = ok_alias;
    let _ = (make, K, D, fixture, deep);
}
`;
const consumerClean = `
use ctc_owner::real::ok;
use ctc_owner::real::fixture;
use ctc_owner::*;
pub fn go() {
    ctc_owner::real::ok();
    ctc_owner::S.ok();
    ctc_owner::prod_only();
    ok();
    let fixture = 1;
    let _ = fixture;
}
`;
const build = (root: string, consumerLib: string, consumerTest: string, ownerTest: string) => {
  put(root, "Cargo.toml", `[workspace]\nmembers = ["owner", "consumer"]\nresolver = "2"\n[workspace.dependencies]\nctc-owner = { path = "owner" }\n`);
  put(root, "owner/Cargo.toml", `[package]\nname = "ctc-owner"\nversion = "0.0.0"\nedition = "2021"\n[features]\ntestkit = []\nx = []\n`);
  put(root, "owner/src/lib.rs", owner);
  put(root, "owner/src/hidden_file.rs", `pub fn deep() {}\n`);
  put(root, "owner/tests/own.rs", ownerTest);
  put(root, "consumer/Cargo.toml", `[package]\nname = "ctc-consumer"\nversion = "0.0.0"\nedition = "2021"\n[dependencies]\nctc-owner = { workspace = true }\nrenamed_owner = { package = "ctc-owner", path = "../owner" }\n[dev-dependencies]\nctc-owner = { workspace = true, features = ["testkit"] }\n`);
  put(root, "consumer/src/lib.rs", consumerLib);
  put(root, "consumer/tests/it.rs", consumerTest);
};
build(join(BASE, "clean"), consumerClean, `use ctc_consumer::go;\n#[test] fn t() { go(); }\n`, `#[test] fn t() { ctc_owner::real::ok(); }\n`);
build(join(BASE, "seeded"), consumerSeeded, `use ctc_consumer::go;\n#[test] fn t() { go(); }\n`, `#[test] fn t() { ctc_owner::fixture(); }\n`);

const run = (root: string) => JSON.parse(execFileSync("bun", [join(HERE, "ctc-scan.ts"), "--root", root, "--out", join(root, "out")], {encoding: "utf8"}).trim().split("\n").pop()!);
const clean = run(join(BASE, "clean"));
const seeded = run(join(BASE, "seeded"));
console.log("clean", JSON.stringify(clean));
console.log("seeded", JSON.stringify(seeded));
const hits = JSON.parse(require("node:fs").readFileSync(join(BASE, "seeded", "out-hits.json"), "utf8")) as {class: string; item: string; file: string; line: number; sanctioned: boolean}[];
const items = new Set(hits.map(h => h.item.split(" ")[0] + "|" + h.class));
const expectItems = ["fixture", "fixtures::make", "fixtures::K", "DemoSnap", "fixtures", "hidden_file::deep", "tm", "S::only_test", "all_gated", "kit", "ok_alias"];
const missing = expectItems.filter(e => !hits.some(h => h.item.split(" ")[0]!.endsWith(e) && !h.item.startsWith("real::")));
const decoy = hits.filter(h => h.item.startsWith("real::") || h.item.includes("prod_only") || h.item.includes("private_mod_item"));
const glob = hits.filter(h => h.class.startsWith("glob"));
console.log("missing expected:", JSON.stringify(missing));
console.log("decoy hits:", JSON.stringify(decoy));
console.log("glob hits:", glob.length, "own-integration hit:", hits.some(h => h.file.endsWith("owner/tests/own.rs")), "kit sanctioned:", hits.filter(h => h.item.startsWith("kit")).map(h => h.sanctioned).join(","));
console.log("items:", [...items].sort().join("; "));
const ok = clean.hits === 0 && seeded.hits > 0 && missing.length === 0 && decoy.length === 0 && glob.length >= 1 && hits.some(h => h.file.endsWith("owner/tests/own.rs"));
console.log(ok ? "SELFTEST PASS" : "SELFTEST FAIL");
process.exit(ok ? 0 : 1);
