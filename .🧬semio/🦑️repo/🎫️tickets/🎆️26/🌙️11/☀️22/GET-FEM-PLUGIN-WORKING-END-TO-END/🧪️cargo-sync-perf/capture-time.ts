import { resolve, relative } from "node:path";
import { captureNxAuthorizerSourcesV1, nxAuthorizerCapturedBytesV1 } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import { runtimeEntrySourcePathV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🚪️entry/🟦️.ts";
import { nativeDirectoryRuntimeEntryV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/📜️script.ts";

const root = resolve(process.env.SEMIO_WORKSPACE_ROOT ?? "C:/semio");
const entry = relative(root, runtimeEntrySourcePathV1(nativeDirectoryRuntimeEntryV1())).replaceAll("\\", "/");
console.error("[DEBUG] entry", entry);
for (let i = 0; i < 2; i++) {
  const start = performance.now(), cpu = process.cpuUsage();
  const captured = await captureNxAuthorizerSourcesV1(root, entry, new AbortController().signal, { installed: true });
  const used = process.cpuUsage(cpu);
  console.error("[DEBUG] capture", i, "ms", Math.round(performance.now() - start), "cpu-ms", Math.round((used.user + used.system) / 1000), "files", captured.manifest.sources.length, "bytes", [...nxAuthorizerCapturedBytesV1(captured).values()].reduce((a, b) => a + b.byteLength, 0));
}
