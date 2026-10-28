import{refuseCachingV1}from"../../📖️texts/🟦️.ts";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { runtimeParseTomlV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/📄️parse/🟦️.ts";
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

/** 🧭️ Names the lockfile Cargo resolves a WASM source owner against: the `Cargo.lock` beside the nearest manifest, the owner's own included, that declares a workspace ([Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html#root-package)). */
export function wasmBindgenLockOfV1(workspace: string, sourceOwner: string): string {
  for (let directory = sourceOwner; ; directory = dirname(directory)) {
    const manifest = join(workspace, directory, "Cargo.toml");
    if (existsSync(manifest) && (runtimeParseTomlV1(readFileSync(manifest, "utf8")) as { workspace?: unknown }).workspace !== undefined) return join(directory, "Cargo.lock").replaceAll("\\", "/");
    if (directory === dirname(directory)) return join(sourceOwner, "Cargo.lock").replaceAll("\\", "/");
  }
}

/** ⚖️ Throws the bilingual lock-drift refusal naming every given lockfile whose `wasm-bindgen` differs from the root pin. */
function refuseMismatches(workspace: string, paths: readonly string[]): void {
  const locks = Object.fromEntries(paths.map((path) => [path, readFileSync(join(workspace, path), "utf8")]));
  const mismatches = wasmBindgenLockMismatches(wasmBindgenVersion(readFileSync(join(workspace, "Cargo.lock"), "utf8")), locks);
  if (mismatches.length) throw refuseCachingV1("failed", "wasm-bindgen.lock-drift", { mismatches: mismatches.join("\n") });
}

/** 🧾️ Refuses a workspace whose declared wasm lockfiles disagree with the root pin; regenerate through the official lock path (`repo cargo -- update --offline -p wasm-bindgen -p js-sys -p web-sys -p wasm-bindgen-futures`). */
export function requireWasmBindgenLocksV1(workspace: string): void {
  refuseMismatches(workspace, WASM_BINDGEN_LOCKS_V1);
}

/** 🛑️ Refuses one WASM source owner before its compile when the lockfile it resolves against disagrees with the root pin, so the mismatch never surfaces as wasm-pack's "Not able to find or install a local wasm-bindgen" after the compile. */
export function requireWasmBindgenSourceLockV1(workspace: string, sourceOwner: string): void {
  refuseMismatches(workspace, [wasmBindgenLockOfV1(workspace, sourceOwner)]);
}
