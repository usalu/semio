import { createRequire } from "node:module";
import { RuntimeCapabilityMissingV1, runtimeMessageV1 } from "../../🗣️messages/🟦️.ts";
import type { RuntimeNativeSystemV1 } from "../../📋️contract/🟦️.ts";

let provided: RuntimeNativeSystemV1 | undefined;

/** 🧲️ Loads the native-binding provider the environment designates; Node has no built-in foreign function interface, so a provider module exporting `nativeSystemV1` must be installed. */
export function nativeSystemFromProviderV1(): RuntimeNativeSystemV1 {
  if (provided !== undefined) return provided;
  const designation = process.env.SEMIO_RUNTIME_NATIVE_PROVIDER;
  if (designation === undefined || designation === "") throw new RuntimeCapabilityMissingV1("ffi", "node", runtimeMessageV1("reasonNativeNone", {}));
  const module = createRequire(import.meta.url)(designation) as { nativeSystemV1?: RuntimeNativeSystemV1 };
  if (module.nativeSystemV1 === undefined) throw new RuntimeCapabilityMissingV1("ffi", "node", runtimeMessageV1("reasonNativeNoExport", { designation }));
  provided = module.nativeSystemV1;
  return provided;
}
