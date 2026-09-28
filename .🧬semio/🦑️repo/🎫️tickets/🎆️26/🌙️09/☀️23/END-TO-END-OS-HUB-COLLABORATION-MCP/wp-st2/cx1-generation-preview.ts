#!/usr/bin/env bun
/** 🔮️ ST2 runner: H14's `h14-bootstrap-generation.ts` (read-only mode) against any tree — the overlay with CX1 applied gives
 * the stdio+GIS bootstrap generation the window-3 landing will write; the live tree must derive exactly its stored id.
 * usage: bun cx1-generation-preview.ts <root> <version…>   (never `--write`) */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const [root, ...versions] = process.argv.slice(2);
if (versions.includes("--write")) throw new Error("preview only");
const generated = join(import.meta.dir, "generated", "cx1-generation-preview");
mkdirSync(generated, { recursive: true });
const source = readFileSync("/Users/ueli/Documents/semio/.tmp-ticket/wp-h14/h14-bootstrap-generation.ts", "utf8")
  .replace('const repo = "/Users/ueli/Documents/semio";', `const repo = ${JSON.stringify(root)};`)
  .replace('const copyDirectory = join(repo, ".tmp-ticket/wp-h14/🗑️generated/hub-script-copy");', `const copyDirectory = ${JSON.stringify(join(generated, "hub-script-copy"))};`);
if (!source.includes(JSON.stringify(root)) || !source.includes(generated)) throw new Error("h14 tool anchors moved");
const copy = join(generated, "h14-bootstrap-generation.ts");
writeFileSync(copy, source);
process.argv = [process.argv[0], copy, ...versions];
await import(copy);
