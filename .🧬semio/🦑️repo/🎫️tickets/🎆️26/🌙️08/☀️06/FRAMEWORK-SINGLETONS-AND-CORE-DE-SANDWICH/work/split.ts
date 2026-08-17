#!/usr/bin/env bun
/** ✂️ Splits framework-core's `📦️lib.rs` godfile into Shape V2 `<topic>/🦀️component.rs` files and a wiring entry file, then proves the split is content-lossless by re-inlining and byte-comparing. */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const SRC = join(REPO, "🧰️framework/⚡️implementations/🦀️rust/📦️lib.rs");
const OWNER = join(REPO, "🧰️framework");
const ENTRY = join(OWNER, "📦️packages/🦀️rust/📦️lib.rs");

/** 🗺️ open/close line numbers (1-based, inclusive of the `pub mod X {` and its `}` lines) from `scan.ts`. */
const TOP = [
  { name: "action_bus", dir: "🎯️action-bus", open: 6, close: 93 },
  { name: "mesh", dir: "🔺️mesh", open: 95, close: 3171 },
  { name: "platform", dir: "🖥️platform", open: 3173, close: 3424 },
  { name: "ui", dir: "🧩️ui", open: 3426, close: 8407 },
] as const;
const NESTED = { name: "kernel", parent: "🧩️ui", dir: "🧠️kernel", open: 6260, close: 7058 } as const;

const lines = readFileSync(SRC, "utf8").split("\n");
const at = (n: number) => lines[n - 1];

function body(open: number, close: number): string[] {
  return lines.slice(open, close - 1);
}

const written = new Map<string, string>();
function emit(path: string, text: string) {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, text);
  written.set(path, text);
}

// 🧠️ kernel first — it is carved out of ui's body.
const kernelBody = body(NESTED.open, NESTED.close);
emit(join(OWNER, NESTED.parent, NESTED.dir, "🦀️component.rs"), kernelBody.join("\n") + "\n");

const entryOut: string[] = [];
let cursor = 1;
for (const mod of TOP) {
  for (let n = cursor; n < mod.open; n++) entryOut.push(at(n));
  entryOut.push(`#[path = "../../${mod.dir}/🦀️component.rs"]`);
  entryOut.push(at(mod.open).replace(/\s*\{\s*$/, ";"));

  let modBody = body(mod.open, mod.close);
  if (mod.dir === NESTED.parent) {
    const relOpen = NESTED.open - mod.open;
    const relClose = NESTED.close - mod.open;
    const head = modBody.slice(0, relOpen - 1);
    const tail = modBody.slice(relClose);
    modBody = [...head, `#[path = "${NESTED.dir}/🦀️component.rs"]`, at(NESTED.open).replace(/\s*\{\s*$/, ";"), ...tail];
  }
  emit(join(OWNER, mod.dir, "🦀️component.rs"), modBody.join("\n") + "\n");
  cursor = mod.close + 1;
}
for (let n = cursor; n <= lines.length; n++) entryOut.push(at(n));
emit(ENTRY, entryOut.join("\n"));

// ── 🔁️ Round-trip proof: re-inline every emitted file back into the entry file and byte-compare. ──
function inline(text: string, resolve: (rel: string) => string): string {
  const out: string[] = [];
  const src = text.split("\n");
  for (let i = 0; i < src.length; i++) {
    const m = src[i].match(/^#\[path = "([^"]+)"\]$/);
    if (m && /^\s*(?:pub\s+)?mod\s+[A-Za-z_][A-Za-z0-9_]*;\s*$/.test(src[i + 1] ?? "")) {
      const nested = inline(readFileSync(resolve(m[1]), "utf8"), (r) => join(dirname(resolve(m[1])), r));
      out.push(src[i + 1].replace(/;\s*$/, " {"));
      out.push(...nested.replace(/\n$/, "").split("\n"));
      out.push("}");
      i++;
      continue;
    }
    out.push(src[i]);
  }
  return out.join("\n");
}

const rebuilt = inline(readFileSync(ENTRY, "utf8"), (rel) => join(dirname(ENTRY), rel));
const original = readFileSync(SRC, "utf8");
if (rebuilt !== original) {
  const a = original.split("\n");
  const b = rebuilt.split("\n");
  for (let i = 0; i < Math.max(a.length, b.length); i++) {
    if (a[i] !== b[i]) {
      console.error(`❌ round-trip mismatch at line ${i + 1}\n  orig: ${JSON.stringify(a[i])}\n  back: ${JSON.stringify(b[i])}`);
      break;
    }
  }
  console.error(`lengths: orig=${a.length} rebuilt=${b.length}`);
  process.exit(1);
}
console.log("✅ round-trip byte-identical");
for (const [p, t] of written) console.log(`  ${p.replace(REPO + "/", "")}  ${t.split("\n").length - 1} lines`);
