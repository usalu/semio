import { expect, test } from "bun:test";
import { readFileSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { transform, Moved } from "./transform.ts";

const moved: Moved = JSON.parse(readFileSync(`${import.meta.dir}/moved.json`, "utf8"));
const cases: { name: string; input: string; expected: string }[] = JSON.parse(readFileSync(`${import.meta.dir}/cases.json`, "utf8"));
const known = new Set(["list::persistent::PersistentList", "sort::HeapSortWork", "fixed_key_slots::FixedKeySlot", "fixed_key_slots::FixedKeySlots", "retained_equality::RetainedEquality", "native_decoding", "native_decoding::NativeReceiver", "native_decoding::NativeDecodeControl", "native_decoding::owned::NativeRecordSource", "list", "sort", "fixed_key_slots"]);
const ctx = { aliasHeads: ["pack"], globHeads: ["protocol"], moved, facade: "semio_framework_value", verify: (_lib: string, segs: string[]) => (known.has(segs.join("::")) || known.has(segs.slice(0, 1).join("::")) && segs.length === 1 ? ("ok" as const) : ("missing" as const)) };

for (const c of cases) {
  test(c.name, () => {
    const r = transform(c.input, ctx);
    expect(r.out).toBe(c.expected);
  });
}

test("third-party cross-check: rustfmt parses every output", () => {
  const dir = mkdtempSync(join(tmpdir(), "value-repoint-"));
  cases.forEach((c, i) => writeFileSync(join(dir, `c${i}.rs`), c.expected));
  for (let i = 0; i < cases.length; i++) {
    const r = Bun.spawnSync(["rustfmt", "--edition", "2021", "--check", "--config", "skip_children=true,disable_all_formatting=true", join(dir, `c${i}.rs`)]);
    expect(r.stderr.toString()).not.toMatch(/^error/m);
  }
});
