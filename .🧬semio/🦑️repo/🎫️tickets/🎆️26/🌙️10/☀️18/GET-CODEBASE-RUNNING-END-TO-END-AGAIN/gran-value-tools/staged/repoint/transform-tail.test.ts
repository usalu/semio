/** 🧪 Language-agnostic cases (cases-tail.json) for the facade-tail table, including whole-crate module aliases; outputs are cross-checked with rustfmt. */
import { expect, test } from "bun:test";
import { readFileSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { transform, Moved } from "./transform.ts";

const moved: Moved = JSON.parse(readFileSync(`${import.meta.dir}/moved-tail.json`, "utf8"));
const cases: { name: string; input: string; expected: string }[] = JSON.parse(readFileSync(`${import.meta.dir}/cases-tail.json`, "utf8"));
const ctx = { aliasHeads: ["protocol"], globHeads: [], moved, facade: "semio_framework_value", verify: () => "ok" as const };
for (const c of cases) test(c.name, () => expect(transform(c.input, ctx).out).toBe(c.expected));
test("third-party cross-check: rustfmt parses every output", () => {
  const dir = mkdtempSync(join(tmpdir(), "gran-tail-"));
  cases.forEach((c, i) => writeFileSync(join(dir, `c${i}.rs`), c.expected));
  for (let i = 0; i < cases.length; i++) expect(Bun.spawnSync(["rustfmt", "--edition", "2021", "--check", "--config", "skip_children=true,disable_all_formatting=true", join(dir, `c${i}.rs`)]).stderr.toString()).not.toMatch(/^error/m);
});
