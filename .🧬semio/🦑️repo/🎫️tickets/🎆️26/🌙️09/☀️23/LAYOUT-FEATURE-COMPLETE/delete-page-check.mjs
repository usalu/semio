import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import ts from "typescript";

const source = "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️retained/🗑️page-deletion";
const entry = resolve(source, "🟦️.ts");
const program = ts.createProgram([entry], {
  target: ts.ScriptTarget.ES2022,
  module: ts.ModuleKind.ESNext,
  moduleResolution: ts.ModuleResolutionKind.Bundler,
  noEmit: true,
  allowImportingTsExtensions: true,
  skipLibCheck: true,
  strict: true,
  noUncheckedIndexedAccess: true,
});
const diagnostics = ts.getPreEmitDiagnostics(program);
assert.equal(diagnostics.length, 0, ts.formatDiagnosticsWithColorAndContext(diagnostics, {
  getCurrentDirectory: () => process.cwd(), getCanonicalFileName: value => value, getNewLine: () => "\n",
}));
const { DeletePagePlan } = await import(entry);
const fixture = JSON.parse(await readFile(resolve(source, "🧪️tests/🔣️.json"), "utf8"));
const grant = copy => ({
  maximumItems: 1, maximumCopyBytes: copy, maximumCapacityBytes: 0, maximumReleaseBytes: 0, maximumDepth: 1,
});
function close(plan) {
  plan.beginClose();
  for (let step = 0; step < 8 && !plan.terminalIsEmpty(); step++) {
    const demand = plan.closeDemand().copyBytes;
    assert.equal(plan.close(grant(demand), 0).copyBytes, 0);
    plan.close(grant(demand), 1);
  }
  assert.ok(plan.terminalIsEmpty());
}
for (const input of fixture.cases) {
  const prefix = input.framePrefix?.character.repeat(input.framePrefix.count) ?? "";
  const event = { id: input.delete };
  const base = {
    pages: input.pages.map(page => ({
      id: page.id, spreadId: page.spread,
      frames: page.frames.map(([id, successor]) => ({
        kind: "text", id: prefix + id, threadNext: successor === null ? null : prefix + successor,
      })),
    })),
    spreads: input.spreads.map(([id, pageIds]) => ({ id, pageIds })),
  };
  const plan = new DeletePagePlan();
  for (let step = 0; step < 128 && plan.result() === undefined; step++) {
    const demand = plan.demand().copyBytes;
    assert.equal(plan.advance(event, base, grant(demand), 0).copyBytes, 0);
    assert.equal(plan.advance(event, base, grant(demand - 1), 1).copyBytes, 0);
    plan.advance(event, base, grant(demand), 1);
    if (step === 0) {
      assert.throws(() => plan.advance({ ...event }, base, grant(demand), 1), /source-changed/);
      assert.throws(() => plan.advance(event, { ...base }, grant(demand), 1), /source-changed/);
    }
  }
  assert.deepEqual(plan.result(), input.expected, input.name);
  close(plan);
  for (const cut of fixture.cuts) {
    const cancelled = new DeletePagePlan();
    for (let step = 0; step < cut && cancelled.result() === undefined; step++) {
      cancelled.advance(event, base, grant(cancelled.demand().copyBytes), 1);
    }
    close(cancelled);
  }
}
console.log(`[DEBUG] ${fixture.cases.length} TypeScript DeletePage decisions, ${fixture.cases.length * fixture.cuts.length} cancellation cuts, two source-substitution checks per input`);
