import { fileURLToPath } from "node:url";
import production from "../../../../../../../♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/🎚️config/🟦️.ts";

export default { ...production, testDir: fileURLToPath(new URL("./", import.meta.url)), testMatch: ["generator-wire.spec.ts"] };
