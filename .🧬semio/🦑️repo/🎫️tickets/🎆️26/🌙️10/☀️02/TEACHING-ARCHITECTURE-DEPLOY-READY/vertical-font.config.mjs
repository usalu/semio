import base from "../../../../../../../🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🎚️config/🟦️.ts";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
const root = dirname(fileURLToPath(import.meta.url));
export default { ...base, testDir: root, testMatch: "vertical-font.spec.mjs", outputDir: join(root, "🗑️generated/vf8"), workers: 1, reporter: [["list"]], projects: [{ name: "desktop" }] };
