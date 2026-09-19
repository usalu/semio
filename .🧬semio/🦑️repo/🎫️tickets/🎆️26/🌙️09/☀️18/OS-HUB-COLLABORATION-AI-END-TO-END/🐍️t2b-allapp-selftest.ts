/** 🧭️ Runs the exported all-app discovery self-tests directly, to show whether the "missing gate" case
 * can execute while `INTERACTIVITY_ALL_APP_REQUIRED_GATES` is the empty tuple. */
import { resolve } from "node:path";
const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const { interactivityAllAppDiscoverySelfTests } = await import(resolve(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧪️tests/🔬️interactivity-all-app-discovery/🟦️.ts"));
console.log(`all-app self-tests = ${await interactivityAllAppDiscoverySelfTests()}`);
