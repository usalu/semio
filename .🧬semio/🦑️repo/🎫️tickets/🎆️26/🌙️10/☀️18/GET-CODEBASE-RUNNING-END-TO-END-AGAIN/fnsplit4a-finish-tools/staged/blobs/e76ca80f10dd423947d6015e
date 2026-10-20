import { describe, expect, test } from "bun:test";
import { execFileSync } from "node:child_process";
import { cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { applyCarveWrites, carveWrites, planArtifactCrates, resolveCarveBackupDir } from "../../../🏗️authoring/📦️crate-tree/🟦️.ts";
import { demoteDanglingOuterDocs, parseRustSourceTree, rustOuterDocStart } from "../../../🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
import { rootAliasModules } from "../../../🏗️authoring/📦️crate-tree/🗺️plan/🧭️module-chains/🟦️.ts";
import { rewriteMountPaths } from "../../../🏗️authoring/📦️crate-tree/📝️render/🟦️.ts";
import { verifyAppliedLayout } from "../../../🏗️authoring/📦️crate-tree/🧫️sandbox/🟦️.ts";
import { transformTestClosure } from "../../../🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";
import { absolutizeSuperPaths } from "../../../🏗️authoring/📦️crate-tree/🧪️test-closure/🟢️shared/🟦️.ts";
import { tiny, weaveFiles } from "../🏗️virtual-crate/🟦️.ts";

describe("artifact crate carve", () => {
  test("absolutizes super paths of a detached test and widens a pub(super) restriction to pub(crate)", () => {
    const source = "use super::*;\npub(super) fn helper() {}\nmod inner { use super::super::Thing; pub(super) fn deep() {} }\n";
    const rewritten = absolutizeSuperPaths(source, ["editor", "procedure", "unit_tests"]);
    expect(rewritten).toContain("use crate::editor::procedure::*;");
    expect(rewritten).toContain("pub(crate) fn helper()");
    expect(rewritten).toContain("use crate::editor::procedure::Thing;");
    expect(rewritten).toContain("pub(super) fn deep()");
    expect(rewritten).not.toContain("pub(in ");
  });

  test("counts the private component module of a host as one super level when it absolutizes the paths of a detached test", () => {
    const rewritten = absolutizeSuperPaths("use super::*;\nfn t() { super::super::topology::run(); super::super::super::sibling::run(); }\n", ["schema", "inferences", "component", "tests"]);
    expect(rewritten).toContain("use crate::schema::inferences::*;");
    expect(rewritten).toContain("crate::schema::inferences::topology::run()");
    expect(rewritten).toContain("crate::schema::sibling::run()");
  });

  test("removing a mount takes its outer doc comment along and leaves region markers and plain comments alone", () => {
    const source = "//#region R\n/// doc one\n/// doc two\n#[cfg(test)]\nmod tests;\n//#endregion R\n// plain\n#[cfg(test)]\nmod other;\n";
    const mount = source.indexOf("#[cfg(test)]\nmod tests;");
    const cut = rustOuterDocStart(source, mount);
    expect(source.slice(0, cut)).toBe("//#region R\n");
    const plain = source.indexOf("#[cfg(test)]\nmod other;");
    expect(rustOuterDocStart(source, plain)).toBe(plain);
    expect(rustOuterDocStart("x /// inline\nmod a;", 14)).toBe(14);
  });

  test("demotes an outer doc comment that no longer documents an item and keeps every other one", () => {
    const source = "/// kept: documents the fn\n//#region R\npub fn a() {}\n/// orphan one\n/// orphan two\n//#region E\n\n//#endregion E\n";
    const fixed = demoteDanglingOuterDocs(source);
    expect(fixed).toContain("/// kept: documents the fn");
    expect(fixed).toContain("// orphan one\n// orphan two");
    expect(demoteDanglingOuterDocs("mod m {\n    /// orphan in block\n}\n")).toContain("    // orphan in block");
    expect(demoteDanglingOuterDocs("/// doc\n#[cfg(test)]\nmod t;\n")).toBe("/// doc\n#[cfg(test)]\nmod t;\n");
  });

  test("declares a cut file module pub in the mirror so the parent crate can re-export it, and leaves a pub one alone", () => {
    const source = '#[path = "child/🦀️.rs"]\nmod child;\n#[path = "open/🦀️.rs"]\npub mod open;\n';
    const [hidden, open] = parseRustSourceTree(source).modules;
    const widened = rewriteMountPaths(source.slice(hidden!.start, hidden!.end), { ...hidden!, start: 0, declStart: hidden!.declStart - hidden!.start, end: hidden!.end - hidden!.start }, "/r", "/r", true);
    expect(widened).toContain("pub mod child;");
    const kept = rewriteMountPaths(source.slice(open!.start, open!.end), { ...open!, start: 0, declStart: open!.declStart - open!.start, end: open!.end - open!.start }, "/r", "/r", true);
    expect(kept).not.toContain("pub pub");
    expect(kept).toContain("pub mod open;");
  });

  test("places the carried imports of an aggregated leaf test behind its inner doc comments", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-aggregate-inner-"));
    try {
      const leaf = (name: string, doc: boolean): Record<string, string> => ({
        [`tree/${name}/🦀️.rs`]: `use std::fmt::Debug;\npub fn ${name}() {}\n#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n`,
        [`tree/${name}/🧪️tests/🦀️.rs`]: `${doc ? "//! inner doc\n" : ""}use super::*;\n#[test]\nfn runs() { ${name}(); let _: Option<&dyn Debug> = None; }\n`,
      });
      const layout: Record<string, string> = {
        "🦀️.rs": '#[path = "tree/🦀️.rs"]\npub mod s;\n',
        "tree/🦀️.rs": '#[path = "."]\npub mod modules {\n    #[path = "a/🦀️.rs"]\n    pub mod a;\n    #[path = "b/🦀️.rs"]\n    pub mod b;\n    #[path = "c/🦀️.rs"]\n    pub mod c;\n}\n',
        ...leaf("a", true), ...leaf("b", false), ...leaf("c", false),
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(), { fixtures: false, aggregate: true, hoist: false });
      const created = [...result.creates.values()].join("\n");
      expect(created).toContain("mod a {\n//! inner doc\n");
      expect(created.indexOf("//! inner doc")).toBeLessThan(created.indexOf("use std::fmt::Debug;"));
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("gates an applied carve against a pre-carve directory when the fleet carves a tree with uncommitted edits", async () => {
    const repo = mkdtempSync(join(tmpdir(), "semio-applied-dir-"));
    const sandbox = mkdtempSync(join(tmpdir(), "semio-applied-dir-sbx-"));
    const snapshot = mkdtempSync(join(tmpdir(), "semio-applied-dir-pre-"));
    const artifact = "a/b/c";
    try {
      for (const [path, content] of Object.entries(weaveFiles(false))) { mkdirSync(dirname(join(repo, artifact, path)), { recursive: true }); writeFileSync(join(repo, artifact, path), content); }
      const git = (...args: string[]): void => { execFileSync("git", ["-c", "user.name=t", "-c", "user.email=t@t", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", ...args], { cwd: repo, stdio: "pipe" }); };
      git("init", "-q");
      git("add", "-A");
      git("commit", "-q", "-m", "baseline");
      cpSync(join(repo, artifact), join(snapshot, artifact), { recursive: true });
      const plan = await planArtifactCrates(repo, `${artifact}/📦️packages/🦀️rust`, { ...tiny });
      expect(plan.problems).toEqual([]);
      const { writes } = await carveWrites(plan);
      applyCarveWrites(repo, writes);
      const clean = await verifyAppliedLayout(repo, artifact, join(sandbox, "clean"), `dir:${snapshot}`);
      expect(clean.crates).toBeGreaterThan(1);
      expect(clean.baseCrates).toBe(0);
      expect(Object.values(clean.layout).map((row) => row.onlyCombined)).toEqual([0, 0]);
    } finally {
      rmSync(repo, { recursive: true, force: true });
      rmSync(sandbox, { recursive: true, force: true });
      rmSync(snapshot, { recursive: true, force: true });
    }
  });

  test("resolves an absolute --backup directory as given and never joins it onto the repository root", () => {
    const repo = mkdtempSync(join(tmpdir(), "semio-backup-repo-"));
    const outside = mkdtempSync(join(tmpdir(), "semio-backup-outside-"));
    try {
      const resolved = resolveCarveBackupDir(repo, join(outside, "f3-backup"));
      expect(resolved).toBe(join(realpathSync(outside), "f3-backup"));
      writeFileSync(join(repo, "keep.rs"), "old\n");
      applyCarveWrites(repo, [{ path: "keep.rs", action: "update", content: "new\n" }], resolved);
      expect(readFileSync(join(resolved, "keep.rs"), "utf8")).toBe("old\n");
      expect(readFileSync(join(repo, "keep.rs"), "utf8")).toBe("new\n");
      expect(existsSync(join(repo, outside.replace(/^[/\\]+/u, "")))).toBe(false);
    } finally {
      rmSync(repo, { recursive: true, force: true });
      rmSync(outside, { recursive: true, force: true });
    }
  });

  test("resolves a relative --backup directory below the ticket tree and refuses every directory inside the source tree", () => {
    const repo = mkdtempSync(join(tmpdir(), "semio-backup-source-"));
    try {
      const ticketDir = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/T/🗑️generated/f3-backup";
      expect(resolveCarveBackupDir(repo, ticketDir)).toBe(join(realpathSync(repo), ticketDir));
      expect(() => resolveCarveBackupDir(repo, "f3-backup")).toThrow("inside the source tree");
      expect(() => resolveCarveBackupDir(repo, join(repo, "✏️s/backup"))).toThrow("inside the source tree");
      expect(() => resolveCarveBackupDir(repo, ".")).toThrow("inside the source tree");
      expect(() => resolveCarveBackupDir(repo, ".🧬semio/other/backup")).toThrow("inside the source tree");
      expect(() => resolveCarveBackupDir(repo, "x/../../escape-ok-" + Date.now())).not.toThrow();
    } finally {
      rmSync(repo, { recursive: true, force: true });
    }
  });

  test("treats a root alias module whose nested modules alias the same targets as one alias and refuses a nested module with other targets", () => {
    const source = [
      "pub mod diff {\n    pub use crate::standards::v1::any::schema::diff::*;\n    pub mod schema {\n        pub use crate::standards::v1::any::schema::diff::*;\n    }\n}",
      "pub mod mutations {\n    pub use crate::standards::v1::any::schema::mutations::*;\n    pub mod schema {\n        pub use crate::elsewhere::*;\n    }\n}",
    ].join("\n");
    const aliases = rootAliasModules(source, parseRustSourceTree(source).modules);
    expect([...aliases.keys()]).toEqual(["diff"]);
    expect([...aliases.get("diff")!.nested]).toEqual(["schema"]);
  });
});
