#!/usr/bin/env python3
"""🚫️ S5-AGNOSTIC (design §22.20/§22.23): the `schema mutation-editability` gate reads the leaf descriptor's `"editable": false`
(schema `📡️spr/🎮️command/🧬️schema` `MutationLeafDescriptor.properties.editable`) as the verdict `inert` — a withdraw-only leaf by
declaration: counted (`withdrawOnly`, and among `inert`), never among `editable`, and never the subject of an editability finding (its
payload references are not resolved, since no editor ever opens on it). The structural rule `parentLeafReadsChild` (design §20.15) is
about what a leaf's fold reads, not about editing, and still applies. Also adds the gate self-test over the real remodel tree with an
independent descriptor walk as oracle. Anchored on the exact current text, count-asserted, one write per file, idempotent.
Usage: [--apply]"""
import sys

ROOT = "/Users/ueli/Documents/semio/"
GATE = ROOT + "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts"
TEST = ROOT + "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️mutation-history-gates/🟦️.ts"

GATE_EDITS = [
    ("""/** ✏️ The history-edit verdict of one operation shape: `editable` (an input schema and no foreign-step capability), `inert` (no input
 * schema — a non-payload phase of a `#[mutation_leaf(payload = …)]` leaf) or `foreign` (a composite that may emit foreign steps). */""",
     """/** ✏️ The history-edit verdict of one operation shape: `editable` (an input schema and no foreign-step capability), `inert` (no input
 * schema — a leaf whose descriptor declares `"editable": false`, withdraw-only by declaration (design §22.20), or a non-payload phase of
 * a `#[mutation_leaf(payload = …)]` leaf) or `foreign` (a composite that may emit foreign steps). */"""),
    ("""/** ✏️ One leaf of a `#[derive(Mutations)]` aggregate: its verdict and, for a payload-marked leaf, the inert phase variants beside its
 * editable payload. */
export type MutationLeafEditability = Readonly<{ owner: string; aggregate: string; path: string; kind: string; variant: string; verdict: "editable" | "foreign"; inert: readonly string[] }>;""",
     """/** ✏️ One leaf of a `#[derive(Mutations)]` aggregate: its verdict (`inert` when its descriptor declares it withdraw-only) and, for a
 * payload-marked leaf, the inert phase variants beside its editable payload. */
export type MutationLeafEditability = Readonly<{ owner: string; aggregate: string; path: string; kind: string; variant: string; verdict: MutationEditabilityVerdict; inert: readonly string[] }>;"""),
    ("""/** 📊️ One plugin's share of the editability census. */
export type MutationEditabilityCensusRow = { readonly owner: string; aggregates: number; leaves: number; editable: number; foreign: number; inert: number; handwritten: number; findings: number; readonly refused: Record<string, number> };""",
     """/** 📊️ One plugin's share of the editability census: `withdrawOnly` counts the leaves declared `"editable": false`, `inert` those plus
 * every inert phase of a payload-marked leaf. */
export type MutationEditabilityCensusRow = { readonly owner: string; aggregates: number; leaves: number; editable: number; foreign: number; inert: number; withdrawOnly: number; handwritten: number; findings: number; readonly refused: Record<string, number> };"""),
    ("""    const row = census.get(owner) ?? { owner, aggregates: 0, leaves: 0, editable: 0, foreign: 0, inert: 0, handwritten: 0, findings: 0, refused: {} };
    census.set(owner, row);
    return row;
  };
  const refuse = (row: MutationEditabilityCensusRow, kind: string, path: string, detail: string): void => {""",
     """    const row = census.get(owner) ?? { owner, aggregates: 0, leaves: 0, editable: 0, foreign: 0, inert: 0, withdrawOnly: 0, handwritten: 0, findings: 0, refused: {} };
    census.set(owner, row);
    return row;
  };
  const refuse = (row: MutationEditabilityCensusRow, kind: string, path: string, detail: string): void => {"""),
    ("""      const composite = readJsonObject(repoRoot, `${leaf.directory}/🔣️.json`)?.composition === "composite";
      const wrapper = tree.wrappers.get(leaf.directory)?.find((candidate) => candidate.name === aggregate.payloadTypes.get(variant));
      const inert = wrapper === undefined ? [] : [...wrapper.variants.keys()].filter((phase) => phase !== wrapper.payloadVariant);
      leaves.push({ owner, aggregate: aggregate.name, path: leaf.directory, kind: leaf.kind, variant, verdict: composite ? "foreign" : "editable", inert });
""",
     """      const descriptor = readJsonObject(repoRoot, `${leaf.directory}/🔣️.json`);
      const composite = descriptor?.composition === "composite";
      const withdrawOnly = descriptor?.editable === false;
      const wrapper = tree.wrappers.get(leaf.directory)?.find((candidate) => candidate.name === aggregate.payloadTypes.get(variant));
      const inert = wrapper === undefined ? [] : [...wrapper.variants.keys()].filter((phase) => phase !== wrapper.payloadVariant);
      leaves.push({ owner, aggregate: aggregate.name, path: leaf.directory, kind: leaf.kind, variant, verdict: withdrawOnly ? "inert" : composite ? "foreign" : "editable", inert });
      row.leaves += 1;
      row.inert += inert.length;
      if (withdrawOnly) {
        row.withdrawOnly += 1;
        row.inert += 1;
        continue;
      }
"""),
    ("""      if (unpublished.length > 0) refuse(row, "leafReferenceUnpublished", leaf.schemaPath, `${leaf.kind}'s payload schema references ${unpublished.join(", ")}, which neither its own tree nor a plugin its crate depends on (published beside the leaf) nor a framework scope holds, so the history editor cannot resolve it`);
      row.leaves += 1;
      row.inert += inert.length;
      if (composite) row.foreign += 1;""",
     """      if (unpublished.length > 0) refuse(row, "leafReferenceUnpublished", leaf.schemaPath, `${leaf.kind}'s payload schema references ${unpublished.join(", ")}, which neither its own tree nor a plugin its crate depends on (published beside the leaf) nor a framework scope holds, so the history editor cannot resolve it`);
      if (composite) row.foreign += 1;"""),
    (""" * generic editor can edit: each leaf of a `#[derive(Mutations)]` aggregate is `editable` unless its descriptor composes a plan
 * (`foreign`, the `may_emit_foreign_steps` capability), a payload-marked leaf adds its inert phases; a generic aggregate gets no emitted""",
     """ * generic editor can edit: each leaf of a `#[derive(Mutations)]` aggregate is `editable` unless its descriptor declares
 * `"editable": false` (`inert`: withdraw-only by declaration, design §22.20 — counted, never the subject of an editability finding) or
 * composes a plan (`foreign`, the `may_emit_foreign_steps` capability), a payload-marked leaf adds its inert phases; a generic aggregate gets no emitted"""),
    ("""  const total = report.census.reduce((sum, row) => ({ aggregates: sum.aggregates + row.aggregates, leaves: sum.leaves + row.leaves, editable: sum.editable + row.editable, foreign: sum.foreign + row.foreign, inert: sum.inert + row.inert, handwritten: sum.handwritten + row.handwritten }), { aggregates: 0, leaves: 0, editable: 0, foreign: 0, inert: 0, handwritten: 0 });""",
     """  const total = report.census.reduce((sum, row) => ({ aggregates: sum.aggregates + row.aggregates, leaves: sum.leaves + row.leaves, editable: sum.editable + row.editable, foreign: sum.foreign + row.foreign, inert: sum.inert + row.inert, withdrawOnly: sum.withdrawOnly + row.withdrawOnly, handwritten: sum.handwritten + row.handwritten }), { aggregates: 0, leaves: 0, editable: 0, foreign: 0, inert: 0, withdrawOnly: 0, handwritten: 0 });"""),
    ("""    console.log(["owner", "aggregates", "leaves", "editable", "foreign", "inert", "handwritten", "findings", ...classes].join("\\t"));
    for (const row of report.census) console.log([row.owner, row.aggregates, row.leaves, row.editable, row.foreign, row.inert, row.handwritten, row.findings, ...classes.map((name) => row.refused[name] ?? 0)].join("\\t"));""",
     """    console.log(["owner", "aggregates", "leaves", "editable", "foreign", "inert", "withdrawOnly", "handwritten", "findings", ...classes].join("\\t"));
    for (const row of report.census) console.log([row.owner, row.aggregates, row.leaves, row.editable, row.foreign, row.inert, row.withdrawOnly, row.handwritten, row.findings, ...classes.map((name) => row.refused[name] ?? 0)].join("\\t"));"""),
    ("""(${total.foreign} composite with foreign-step capability, ${total.inert} inert phase(s));""",
     """(${total.foreign} composite with foreign-step capability, ${total.withdrawOnly} withdraw-only by declaration, ${total.inert - total.withdrawOnly} inert phase(s));"""),
]

TEST_OLD = """  test("a hand-written aggregate declares why it is not edited, or is a finding", () => {"""
TEST_NEW = """  test("a leaf whose descriptor declares editable false is inert, counted withdraw-only and never a finding (independent descriptor walk)", () => {
    const under = "✏️s/🔌️plugins/📸️remodel";
    const declared: string[] = [];
    const walk = (directory: string): void => {
      for (const entry of readdirSync(join(repoRoot, directory), { withFileTypes: true })) {
        if (!entry.isDirectory() || ["node_modules", "target", "dist", "🧫️fixtures", "🧪️tests"].includes(entry.name)) continue;
        const path = `${directory}/${entry.name}`;
        const descriptor = existsSync(join(repoRoot, path, "🔣️.json")) ? (JSON.parse(readFileSync(join(repoRoot, path, "🔣️.json"), "utf8")) as Record<string, unknown>) : null;
        if (descriptor !== null && typeof descriptor.semanticKind === "string" && descriptor.editable === false) declared.push(path);
        walk(path);
      }
    };
    walk(under);
    const report = mutationEditabilityReport(repoRoot, under, [under]);
    const inert = report.leaves.filter((leaf) => leaf.verdict === "inert").map((leaf) => leaf.path);
    expect(declared.length).toBeGreaterThan(0);
    expect(inert.sort()).toEqual(declared.sort());
    expect(report.census.reduce((sum, row) => sum + row.withdrawOnly, 0)).toBe(declared.length);
    expect(report.diagnostics.filter((entry) => declared.some((path) => entry.path.startsWith(`${path}/`) || entry.path === path))).toEqual([]);
  }, 120_000);
""" + TEST_OLD
IMPORT_OLD = """import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";"""
IMPORT_NEW = """import { existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";"""


def main() -> None:
    gate = open(GATE, encoding="utf-8").read()
    test = open(TEST, encoding="utf-8").read()
    if 'withdrawOnly ? "inert"' in gate:
        print("SKIP: the editability gate already reads editable: false")
        return
    for index, (old, _) in enumerate(GATE_EDITS):
        if gate.count(old) != 1:
            sys.exit(f"ANCHOR gate #{index}: {gate.count(old)} matches")
    for name, old in (("test", TEST_OLD), ("import", IMPORT_OLD)):
        if test.count(old) != 1:
            sys.exit(f"ANCHOR {name}: {test.count(old)} matches")
    after_gate = gate
    for old, new in GATE_EDITS:
        after_gate = after_gate.replace(old, new)
    after_test = test.replace(TEST_OLD, TEST_NEW).replace(IMPORT_OLD, IMPORT_NEW)
    if "--apply" not in sys.argv:
        print(f"WOULD edit the gate ({len(GATE_EDITS)} hunks) and add one gate self-test")
        return
    if open(GATE, encoding="utf-8").read() != gate or open(TEST, encoding="utf-8").read() != test:
        sys.exit("RACE: a file changed while staging")
    open(GATE, "w", encoding="utf-8").write(after_gate)
    open(TEST, "w", encoding="utf-8").write(after_test)
    print("WROTE")


if __name__ == "__main__":
    main()
