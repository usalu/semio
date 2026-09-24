import { discoverTestCases, validateAllContracts } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const time = <T>(label: string, run: () => T): T => { const t = performance.now(); const value = run(); console.log(label, Math.round(performance.now() - t)); return value; };
const all = time("discover-1", () => discoverTestCases(root));
time("discover-2", () => discoverTestCases(root));
time("contracts-all", () => validateAllContracts(root));
time("contracts-single", () => validateAllContracts(root, all.slice(0, 1)));
