import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";

const fixture = JSON.parse(readFileSync(new URL("./note-window-config-authority-work.json", import.meta.url), "utf8"));
const canonicalSource = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧳️source/🦀️.rs";

test("the canonical authority Source can spend the Work its selected close requires", () => {
  using reference = new Database(":memory:");
  const oracle = reference.query(`SELECT work AS quote, $available >= work AND $copyAvailable >= $copyRequired AS advanced
    FROM (SELECT CASE WHEN $empty THEN 1 ELSE $required END AS work)`);
  for (const row of fixture.cases) {
    expect(oracle.get({ $empty: +row.sourceEmpty, $required: row.required, $available: row.available, $copyAvailable: row.copyAvailable, $copyRequired: fixture.selectedCloseCopyBytes }))
      .toEqual({ quote: row.quote, advanced: +row.advanced });
  }
  const camera = fixture.cases[0];
  expect(camera.available).toBe(fixture.previousQuote);
  expect(camera.required).toBe(fixture.observedRequiredWork);
  expect(camera.advanced).toBe(false);
});

test("the canonical Source quotes the current authority close and keeps the original effect", () => {
  const source = readFileSync(canonicalSource, "utf8");
  const implementation = source.slice(source.indexOf("ArtifactStoreCanonicalOwner<M> for ArtifactStoreCanonicalSource<M>"));
  const work = implementation.split("fn next_work_demand")[1].split("fn next_demand")[0];
  const phase = work.split(`${fixture.phase}=>{`)[1];
  expect(phase).toBeDefined();
  expect(phase).toContain("if owner.terminal_is_empty(){1}");
  expect(phase).toContain("owner.next_close_work_demand(owner.next_close_copy_byte_demand()?)?");
  expect(source).toContain("o.close_step(g.maximum_copy_bytes,&mut effect.nested(child(g)))");
});
