#!/usr/bin/env bun
/** Export lowpoly default fixture JSON for lowpoly/example/default.lowpoly.json */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { runCargo } from "../../../repo/lib/js/index.ts";

const repoRoot = join(import.meta.dir, "../../../..");
const outPath = join(repoRoot, "lowpoly/example/default.lowpoly.json");
const snippet = `
#[test]
fn export_default_fixture_once() {
    let fixture = lowpoly_core::default_fixture();
    let json = serde_json::to_string(&fixture).expect("fixture json");
    eprintln!("LOWPOLY_FIXTURE_JSON_START");
    eprintln!("{json}");
    eprintln!("LOWPOLY_FIXTURE_JSON_END");
}
`;
writeFileSync(join(import.meta.dir, "export_fixture_test.snippet.rs"), snippet);
runCargo(["test", "-p", "lowpoly_core", "default_fixture_has_rock_object", "--", "--nocapture"], repoRoot);
const { execFileSync } = await import("node:child_process");
const output = execFileSync("cargo", ["test", "-p", "lowpoly_core", "export_default_fixture_once", "--", "--nocapture"], {
	cwd: repoRoot,
	encoding: "utf8",
	stdio: ["ignore", "pipe", "pipe"],
});
const match = output.match(/LOWPOLY_FIXTURE_JSON_START\n([\s\S]*?)\nLOWPOLY_FIXTURE_JSON_END/);
if (!match?.[1]) throw new Error("failed to export fixture json");
writeFileSync(outPath, match[1]);
console.log(`[DEBUG] wrote ${outPath}`);
