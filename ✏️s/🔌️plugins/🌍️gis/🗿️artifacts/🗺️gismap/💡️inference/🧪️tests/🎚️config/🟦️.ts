import base from "../../../../../../../../🧰️framework/🛍️products/💻️os/🧪️tests/🎚️config/🟦️.ts";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
const frameworkWorker = fileURLToPath(new URL("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts", import.meta.url));
const worker = fileURLToPath(new URL("../../../../../../../🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/👷️worker/🟦️.ts", import.meta.url));
export default defineConfig({ ...base, test: { ...base.test, name: "gis-gismap-service-composition", include: [], includeSource: [frameworkWorker, worker], coverage: { include: [worker] }, passWithNoTests: false } });
