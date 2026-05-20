//#region 🧲Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// GNU LGPL-3.0 or later — sole `@semio/rs-wasm` import site; exposes GraphQL execute/subscribe only.
//#endregion 🧲Header

import { RS_WASM_EMPTY_STORE_URI } from "./graphql-contract";

export type GraphqlExecuteFn = (requestJson: string) => Promise<string>;
export type GraphqlSubscribeFn = (requestJson: string, onEvent: (eventJson: string) => void) => Promise<void>;

/** @emoji 🌐 WASM handle — JSON GraphQL wire in/out only (no Rust DTO surface). */
export type RsWasmGraphqlHandle = Readonly<{
  execute: GraphqlExecuteFn;
  subscribe: GraphqlSubscribeFn;
  free?: () => void;
}>;

async function readSemioWasmBytesFromMonorepoCandidates(): Promise<Uint8Array | undefined> {
  try {
    const fs = await import("node:fs/promises");
    const path = await import("node:path");
    const url = await import("node:url");
    const here = path.dirname(url.fileURLToPath(import.meta.url));
    const candidates = [
      path.resolve(here, "../rs/pkg/semio_bg.wasm"),
      path.resolve(here, "../../../../semio/client/lib/rs/pkg/semio_bg.wasm"),
    ];
    for (const p of candidates) {
      try {
        const buf = await fs.readFile(p);
        return new Uint8Array(buf);
      } catch {
        /* try next */
      }
    }
  } catch {
    /* non-node */
  }
  return undefined;
}

function defaultRsWasmSpecifier(): string {
  if (typeof window === "undefined" || typeof document === "undefined") {
    return new URL("../rs/pkg/semio.js", import.meta.url).href;
  }
  return "@semio/rs-wasm";
}

/** @emoji 🛰️ Creates a WASM-backed GraphQL executor; {@code bootstrapUri} must be {@link RS_WASM_EMPTY_STORE_URI}. */
export async function createRsWasmGraphqlHandle(
  bootstrapUri: string,
  opts?: Readonly<{ wasmSpecifier?: string; wasmBytes?: Uint8Array | null }>,
): Promise<RsWasmGraphqlHandle> {
  if (bootstrapUri !== RS_WASM_EMPTY_STORE_URI) {
    throw new Error(`createRsWasmGraphqlHandle: only ${RS_WASM_EMPTY_STORE_URI} is allowed; seed kit data via GraphQL after open`);
  }
  const wasmSpecifier = opts?.wasmSpecifier ?? defaultRsWasmSpecifier();
  const wasmBytesPre = opts?.wasmBytes ?? (await readSemioWasmBytesFromMonorepoCandidates());
  let mod: typeof import("@semio/rs-wasm");
  try {
    mod = wasmSpecifier === "@semio/rs-wasm" ? await import("@semio/rs-wasm") : await import(/* @vite-ignore */ wasmSpecifier);
  } catch (e) {
    const base = e instanceof Error ? e.message : String(e);
    throw new Error(`Failed to load @semio/rs-wasm: ${base}`, { cause: e });
  }
  if (typeof mod.default === "function") {
    if (wasmBytesPre) await mod.default({ module_or_path: wasmBytesPre });
    else await mod.default();
  }
  if (typeof mod.boot === "function") mod.boot();
  const handleUnknown = mod.KitStoreHandle.create(bootstrapUri);
  const wasmHandle = handleUnknown instanceof Promise ? await handleUnknown : handleUnknown;
  if (wasmHandle == null || typeof (wasmHandle as { execute?: unknown }).execute !== "function") {
    throw new Error("KitStoreHandle.create did not return execute()");
  }
  return wasmHandle as RsWasmGraphqlHandle;
}
