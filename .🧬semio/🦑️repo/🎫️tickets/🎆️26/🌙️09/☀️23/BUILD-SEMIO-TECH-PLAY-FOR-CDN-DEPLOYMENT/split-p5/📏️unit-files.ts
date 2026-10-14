#!/usr/bin/env bun
/**
 * 🔬️ Per-file breakdown of one crate unit (cfg-selected active lines, bytes) with directory roll-ups, using the
 * tolerant copy of the canonical closure reader (generated includes are listed, not refused).
 *
 * usage: bun 📏️unit-files.ts <entry.rs> [native|wasm|test] [--depth N] [--files]
 */
import { readFile } from "node:fs/promises";
import { relative, resolve, dirname } from "node:path";
import { readRustSourceClosureV1, generated } from "./📏️closure-tolerant.ts";

const REPO = resolve(import.meta.dir, "../../../../../../../..");
const NATIVE = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix", "debug_assertions", "panic=unwind"];
const WASM = ["target_arch=wasm32", "target_os=wasi", "target_env=p2", "target_pointer_width=32", "target_family=wasm", "debug_assertions", "panic=abort"];
const args = process.argv.slice(2), entry = resolve(args[0]!), mode = args[1] ?? "native";
const depth = Number(args[args.indexOf("--depth") + 1] ?? 0) || 99, listFiles = args.includes("--files");
const extra = args.filter(a => a.startsWith("feature=") || a.startsWith("cfg="))
  .map(a => a.startsWith("cfg=") ? a.slice(4) : a);
const cfg = mode === "wasm" ? WASM : mode === "test" ? [...NATIVE, "test"] : NATIVE;
const closure = await readRustSourceClosureV1([entry], REPO, [...cfg, ...extra], async p => await readFile(p).catch(e => e.code === "ENOENT" ? null : Promise.reject(e)));
const base = relative(REPO, dirname(entry)).split("/").length - 0;
const roll = new Map<string, { lines: number; bytes: number; files: number }>();
let tl = 0, tb = 0;
for (const f of closure) {
  tl += f.activeLines; tb += f.bytes;
  const parts = f.path.split("/"), key = parts.slice(0, Math.min(parts.length - 1, base + depth)).join("/");
  const r = roll.get(key) ?? { lines: 0, bytes: 0, files: 0 }; r.lines += f.activeLines; r.bytes += f.bytes; r.files++; roll.set(key, r);
  if (listFiles) console.log(`F\t${f.activeLines}\t${f.bytes}\t${f.kind}\t${f.path}`);
}
for (const [k, r] of [...roll].sort()) console.log(`D\t${r.lines}\t${r.bytes}\t${r.files}\t${k}`);
console.log(`TOTAL\t${tl}\t${tb}\t${closure.length}`);
for (const g of new Set(generated)) console.log(`GENERATED\t${g}`);
