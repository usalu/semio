import { expect, test } from "bun:test";
import { readFileSync, existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import Parser from "web-tree-sitter";
const root = resolve(import.meta.dir, "../../..");

const corpus: { files: string[]; mountedFiles: string[]; broken: string; brokenVisibility: string } = JSON.parse(readFileSync(resolve(import.meta.dir, "../🧫️fixtures/🔣️.json"), "utf8"));
test("CAD unmounted owner input is a closed neutral four-file corpus", () => {
  
  
  
  expect(corpus.files).toHaveLength(4);
  expect(new Set(corpus.files).size).toBe(4);
  expect(corpus.broken.length).toBeGreaterThan(0);
  for (const file of corpus.files) expect(existsSync(resolve(root, file))).toBe(true);
  expect(corpus.mountedFiles).toHaveLength(1);
  expect(corpus.brokenVisibility.length).toBeGreaterThan(0);
  for (const file of corpus.mountedFiles) expect(existsSync(resolve(root, file))).toBe(true);
});
test("independent Rust grammar admits actual CAD owners and rejects malformed ownership syntax", async () => {
  await Parser.init({ locateFile: () => resolve(dirname(fileURLToPath(import.meta.resolve("web-tree-sitter"))), "tree-sitter.wasm") });
  const parser = new Parser();
  parser.setLanguage(await Parser.Language.load(resolve(dirname(fileURLToPath(import.meta.resolve("tree-sitter-wasms/package.json"))), "out/tree-sitter-rust.wasm")));
  try {
    for (const file of [...corpus.files, ...corpus.mountedFiles]) {
      const tree = parser.parse(readFileSync(resolve(root, file), "utf8"));
      try { expect(tree.rootNode.hasError()).toBe(false); } finally { tree.delete(); }
    }
    for (const input of [corpus.broken, corpus.brokenVisibility]) {
      const broken = parser.parse(input);
      try { expect(broken.rootNode.hasError()).toBe(true); } finally { broken.delete(); }
    }
  } finally { parser.delete(); }
});
test("actual literal compile inputs exist before the owning compiler baseline", async () => {
  const framework = resolve(root, "../../../../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts");
  const { inspectRustCompileReferences } = await import(framework);
  let references = 0;
  for (const file of corpus.files) {
    const owner = resolve(root, file);
    for (const reference of inspectRustCompileReferences(readFileSync(owner, "utf8"))) {
      expect(existsSync(resolve(dirname(owner), reference.path))).toBe(true);
      references++;
    }
  }
  expect(references).toBeGreaterThanOrEqual(2);
});
