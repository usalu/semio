import { RUNTIME_REGISTRY_V1, requireRuntimeMemberV1, runtimeHostIdV1 } from "../🧭️selection/🟦️.ts";
import type { RuntimeBundleOptionsV1, RuntimeBundlePluginV1, RuntimeBundleResultV1, RuntimeImportRecordV1, RuntimeLoaderV1, RuntimeTranspilerOptionsV1, RuntimeTranspilerV1 } from "../📋️contract/🟦️.ts";

/** 🔬️ A source transpiler whose implementation is chosen by the hosting runtime. */
export class TranspilerV1 implements RuntimeTranspilerV1 {
  private readonly inner: RuntimeTranspilerV1;
  constructor(options?: RuntimeTranspilerOptionsV1) { this.inner = requireRuntimeMemberV1("transpiler")(options); }
  transformSync(source: string, loader?: RuntimeLoaderV1): string { return this.inner.transformSync(source, loader); }
  scan(source: string): { readonly exports: string[]; readonly imports: RuntimeImportRecordV1[] } { return this.inner.scan(source); }
  scanImports(source: string): RuntimeImportRecordV1[] { return this.inner.scanImports(source); }
}
/** 📦️ Bundles entry points. */
export function runtimeBundleV1(options: RuntimeBundleOptionsV1): Promise<RuntimeBundleResultV1> { return requireRuntimeMemberV1("bundle")(options); }
/** 🔌️ Registers a plugin as a module resolution and loading hook of this process. */
export function runtimeLoaderPluginV1(plugin: RuntimeBundlePluginV1): void { requireRuntimeMemberV1("loaderPlugin")(plugin); }
/** 📥️ Makes a module specifier that imports captured source bytes without touching the file system; `release` frees it once the import settled. */
export function runtimeSourceUrlV1(bytes: Uint8Array): { readonly url: string; release(): void } {
  const row = (RUNTIME_REGISTRY_V1.runtimes as readonly { id: string; sourceUrl: "blob" | "data" }[]).find(candidate => candidate.id === runtimeHostIdV1())!;
  if (row.sourceUrl === "data") return { url: "data:text/javascript;base64," + Buffer.from(bytes).toString("base64"), release: () => undefined };
  const url = URL.createObjectURL(new Blob([Uint8Array.from(bytes)], { type: "text/javascript" }));
  return { url, release: () => URL.revokeObjectURL(url) };
}
