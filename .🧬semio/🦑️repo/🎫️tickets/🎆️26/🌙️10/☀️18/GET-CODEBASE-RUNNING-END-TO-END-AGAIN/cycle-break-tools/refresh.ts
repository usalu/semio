/** ♻️ Drops the cached file lists and rescans (after direct edits or moves). */
import { rmSync } from "node:fs";
import { resolve } from "node:path";
import { GENERATED } from "../exports-stage-tools/lib.ts";
for (const f of ["all-files.txt", "scan.json", "ls-_package_json.txt"]) rmSync(resolve(GENERATED, f), { force: true });
const { scan } = await import("../exports-stage-tools/scan.ts");
const r = scan(true);
console.log("files", r.files, "edges", r.edges.length, "bare", r.bare.length);
