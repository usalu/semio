import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
export default { test: { environment: "jsdom", include: ["**/*.test.ts"], root: dirname(fileURLToPath(import.meta.url)) } };
