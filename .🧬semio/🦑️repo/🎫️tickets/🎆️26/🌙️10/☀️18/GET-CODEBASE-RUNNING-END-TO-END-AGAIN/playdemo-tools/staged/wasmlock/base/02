import{refuseCachingV1}from"../../📖️texts/🟦️.ts";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import registry from "./🔣️.json" with { type: "json" };
/** 🌿️ The binding generator must have the same identity as the locked Rust crate. */
export function wasmBindgenVersion(lock: string): string {
  const versions = [...lock.matchAll(/^name = "wasm-bindgen"\r?\nversion = "([^"]+)"$/gm)].map((match) => match[1]!);
  if (versions.length !== 1) throw new Error("Cargo.lock must resolve exactly one wasm-bindgen version");
  return versions[0]!;
}

/** 🔒️ Every lockfile a wasm build resolves against, declared in `./🔣️.json`: the pinned binding generator must equal the crate version in each. */
export const WASM_BINDGEN_LOCKS_V1: readonly string[] = registry.wasmLocks;

/** 🔎️ Names each lockfile whose `wasm-bindgen` crate differs from the pinned generator version (`wasm-pack --mode no-install` refuses a mismatch). */
export function wasmBindgenLockMismatches(pinned: string, locks: Readonly<Record<string, string>>): string[] {
  return Object.entries(locks).flatMap(([path, text]) => {
    try {
      const version = wasmBindgenVersion(text);
      return version === pinned ? [] : [`${path}: wasm-bindgen ${version}, pinned binding generator ${pinned}`];
    } catch (error) {
      return [`${path}: ${error instanceof Error ? error.message : String(error)}`];
    }
  });
}

/** 🧾️ Refuses a workspace whose declared wasm lockfiles disagree with the root pin; regenerate through the official lock path (`repo cargo -- update --offline -p wasm-bindgen -p js-sys -p web-sys -p wasm-bindgen-futures`). */
export function requireWasmBindgenLocksV1(workspace: string): void {
  const locks = Object.fromEntries(WASM_BINDGEN_LOCKS_V1.map((path) => [path, readFileSync(join(workspace, path), "utf8")]));
  const mismatches = wasmBindgenLockMismatches(wasmBindgenVersion(locks["Cargo.lock"]!), locks);
  if (mismatches.length) throw refuseCachingV1("failed", "wasm-bindgen.lock-drift", { mismatches: mismatches.join("\n") });
}
