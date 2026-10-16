import base from "../../../../../../../../🧰️framework/🛍️products/💻️os/🧪️tests/🎚️config/🟦️.ts";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { frameworkSourceUrl } from "@semio-tech/framework/test/source-text";
const frameworkWorker = fileURLToPath(frameworkSourceUrl("🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"));
const worker = fileURLToPath(new URL("../../../../../../../🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/👷️worker/🟦️.ts", import.meta.url));
export default defineConfig({ ...base, test: { ...base.test, name: "gis-gismap-service-composition", include: [], includeSource: [frameworkWorker, worker], coverage: { include: [worker] }, passWithNoTests: false } });
