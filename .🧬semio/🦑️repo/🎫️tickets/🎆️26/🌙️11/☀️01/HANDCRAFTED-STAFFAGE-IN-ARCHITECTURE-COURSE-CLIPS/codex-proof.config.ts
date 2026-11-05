import base from "./test-runtime.config.ts";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("./", import.meta.url));
export default { ...base, root, test: { ...base.test, root, include: ["codex-proof.test.ts"], maxWorkers: 1 } };
