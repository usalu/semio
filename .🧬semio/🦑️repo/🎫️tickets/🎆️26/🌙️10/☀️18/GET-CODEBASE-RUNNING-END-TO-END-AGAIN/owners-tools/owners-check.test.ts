import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { cargoPackageName, checkRegistry, excluded, syncRegistry, type Policy, type Registry } from "./owners-check.ts";

const policy: Policy = { collections: ["🧫️fixtures"], moduleMember: "🔨️modules", generated: ["node_modules", "target"] };
const write = (root: string, path: string, text: string) => { mkdirSync(join(root, path, ".."), { recursive: true }); writeFileSync(join(root, path), text); };

function tree(): { root: string; files: string[] } {
  const root = mkdtempSync(join(tmpdir(), "owners-"));
  const files: Record<string, string> = {
    "a/📋️project.json": JSON.stringify({ name: "@t/a" }),
    "a/package.json": JSON.stringify({ name: "@t/a" }),
    "course/📋️project.json": JSON.stringify({ name: "@t/course" }),
    "course/package.json": JSON.stringify({ name: "@t/course" }),
    "b/Cargo.toml": '[package]\nname = "crate-b"\n',
    "b/📋️project.json": JSON.stringify({ name: "@t/b" }),
    "crate/Cargo.toml": '[package]\nname = "crate-x"\n\n[dependencies]\n',
    "virt/Cargo.toml": '[workspace]\nmembers = []\n',
    "x/🧫️fixtures/f/📋️project.json": JSON.stringify({ name: "fixture" }),
  };
  for (const [path, text] of Object.entries(files)) write(root, path, text);
  return { root, files: Object.keys(files).filter((path) => !excluded(path, policy)) };
}

const registry = (): Registry => ({
  version: 1,
  catalogProviders: [],
  owners: [
    { name: "@t/a", root: "a", manifest: "a/📋️project.json", packages: ["@t/a"] },
    { name: "crate-b", root: "b", manifest: "b/Cargo.toml", packages: [] },
    { name: "@t/quiz", root: "quiz", manifest: "quiz/📋️project.json", packages: ["@t/quiz"] },
  ],
});

test("reads cargo package names and skips virtual manifests", () => {
  expect(cargoPackageName('[package]\nname = "x"\n[dependencies]\nname = "y"')).toBe("x");
  expect(cargoPackageName("[workspace]\nmembers = []")).toBeUndefined();
});

test("excludes example collections only outside the module member", () => {
  expect(excluded("x/🧫️fixtures/f/p", policy)).toBe(true);
  expect(excluded("🔨️modules/🧫️fixtures/p", policy)).toBe(false);
  expect(excluded("m/node_modules/p", policy)).toBe(true);
});

test("reports stale rows and projects without rows, and sync converges", () => {
  const { root, files } = tree();
  try {
    const kinds = checkRegistry(root, registry(), files, policy).map((f) => `${f.kind}:${f.owner ?? f.path}`).sort();
    expect(kinds).toEqual(["cargo-row-shadows-project:crate-b", "cargo-without-row:crate/Cargo.toml", "missing-root:@t/quiz", "project-without-row:b/📋️project.json", "project-without-row:course/📋️project.json"]);
    const result = syncRegistry(root, registry(), checkRegistry(root, registry(), files, policy));
    expect(result.removed).toEqual(["@t/quiz"]);
    expect(result.added.sort()).toEqual(["@t/course", "crate-x"]);
    expect(result.registry.owners.find((o) => o.manifest === "b/📋️project.json")?.name).toBe("@t/b");
    expect(checkRegistry(root, result.registry, files, policy)).toEqual([]);
    expect(result.registry.owners.find((o) => o.name === "@t/course")?.packages).toEqual(["@t/course"]);
  } finally { rmSync(root, { recursive: true, force: true }); }
});
