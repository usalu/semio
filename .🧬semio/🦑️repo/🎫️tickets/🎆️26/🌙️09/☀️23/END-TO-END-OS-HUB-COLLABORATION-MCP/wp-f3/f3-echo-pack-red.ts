/** 🔴️ F3 — red check of the echo-pack law: the pre-F3 projection (drops buffer + selection only) against the fixture. */
import { isDeepStrictEqual } from "node:util";
import { decodePackValue, encodePackValue } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️.ts";
import fixture from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🧫️fixtures/🪞️echo-pack/🔣️.json";
const old = (scene: Record<string, unknown>, external: boolean) => new Uint8Array(encodePackValue(external ? scene : Object.fromEntries(Object.entries(scene).filter(([key]) => key !== "buffer" && key !== "selectionJson"))));
const same = (a: Uint8Array, b: Uint8Array) => a.length === b.length && a.every((v, i) => v === b[i]);
for (const row of fixture.cases) {
  const p = old(row.previous, row.external), e = old(row.echo, row.external);
  const ok = isDeepStrictEqual(decodePackValue(e), row.synced) && same(p, e) === row.samePack;
  console.log(`${ok ? "PASS" : "FAIL"} ${row.id}`);
}
