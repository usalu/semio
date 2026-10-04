import { expect, test } from "bun:test";
import { readFileSync, existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import Parser from "web-tree-sitter";
const root = resolve(import.meta.dir, "../../..");
const schema = JSON.parse(readFileSync(resolve(import.meta.dir, "🔣️.json"), "utf8"));
const corpus: { files: string[]; broken: string } = JSON.parse(readFileSync(resolve(import.meta.dir, "../🧫️fixtures/🔣️.json"), "utf8"));
test("CAD unmounted owner input is a closed neutral four-file corpus", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(corpus)).toBe(true);
  expect(validate({ ...corpus, extra: true })).toBe(false);
  for (const file of corpus.files) expect(existsSync(resolve(root, file))).toBe(true);
});
test("independent Rust grammar admits actual unmounted owners and rejects malformed ownership syntax", async () => {
  await Parser.init({ locateFile: () => resolve(dirname(fileURLToPath(import.meta.resolve("web-tree-sitter"))), "tree-sitter.wasm") });
  const parser = new Parser();
  parser.setLanguage(await Parser.Language.load(resolve(dirname(fileURLToPath(import.meta.resolve("tree-sitter-wasms/package.json"))), "out/tree-sitter-rust.wasm")));
  try {
    for (const file of corpus.files) {
      const tree = parser.parse(readFileSync(resolve(root, file), "utf8"));
      try { expect(tree.rootNode.hasError()).toBe(false); } finally { tree.delete(); }
    }
    const broken = parser.parse(corpus.broken);
    try { expect(broken.rootNode.hasError()).toBe(true); } finally { broken.delete(); }
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
