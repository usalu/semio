// usage: bun scan_repo.ts OUT.json  (cwd = overlay; scans the real checkout read-only through the staged TypeScript scanner; SEMIO_SCAN_OVERRIDE=map.json replaces file contents)
import { spawnSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import protocol from "/private/tmp/conc-tests/ov/🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🔣️.json" with { type: "json" };
import { compileLawV1, scanIsTestPathV1, scanLanguageV1, scanTestsV1 } from "/private/tmp/conc-tests/ov/🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🟦️.ts";
const root = "/Users/ueli/Documents/semio", law = compileLawV1(protocol as never);
const run = spawnSync("git", ["ls-files", "-z", "--", "*🧪️tests/*", "*_test.go", "*.test.*", "*.spec.*"], { cwd: root, encoding: "utf8", maxBuffer: 1 << 28 });
const listing = run.stdout.split("\0").filter(path => path !== "" && !path.startsWith(".🧬semio/🦑️repo/🎫️tickets/") && scanLanguageV1(law, path) !== undefined && scanIsTestPathV1(law, path));
const override: Record<string, string> = process.env.SEMIO_SCAN_OVERRIDE ? JSON.parse(readFileSync(process.env.SEMIO_SCAN_OVERRIDE, "utf8")) : {};
const started = performance.now();
const result = scanTestsV1(law, listing, path => { try { return override[path] !== undefined ? readFileSync(override[path]!, "utf8") : readFileSync(join(root, path), "utf8"); } catch { return undefined; } });
writeFileSync(process.argv[2]!, JSON.stringify({ scanned: result.scanned, helpers: result.helpers, milliseconds: Math.round(performance.now() - started), findings: result.findings }));
console.log("scanned", result.scanned, "helpers", result.helpers, "findings", result.findings.length, "ms", Math.round(performance.now() - started));
