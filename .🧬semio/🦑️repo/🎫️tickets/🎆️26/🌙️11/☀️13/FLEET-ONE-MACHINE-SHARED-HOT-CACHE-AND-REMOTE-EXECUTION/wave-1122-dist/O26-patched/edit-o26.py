"""O26: anchored edit of the component-pass driver (usage: edit-o26.py <root>); every anchor must match exactly once, else the file is left untouched."""
import sys, pathlib
root = pathlib.Path(sys.argv[1])
CP = "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts"
file = root / CP
text = file.read_text()
edits = [
 ('import { componentFleetChargeV1,',
  'import { admitPassStorageV1, batchAdditionsV1, learnedBatchBytesV1, readStorageGrowthV1, recordStorageGrowthV1, storageGrowthKeyV1, type StorageGrowPortV1 } from "./💾️storage-gate/🟦️.ts";\nimport { componentFleetChargeV1,'),
 ('export function componentBatchesV1(pass: NativeBuildPassV1, graph: CargoUnitGraphV1, history: ReadonlyMap<string, readonly UnitObservationV1[]>, budgetBytes: number, footprints: UnitFootprintPolicyV1): readonly ComponentBatchV1[] {',
  'export function componentBatchesV1(pass: NativeBuildPassV1, graph: CargoUnitGraphV1, history: ReadonlyMap<string, readonly UnitObservationV1[]>, budgetBytes: number, footprints: UnitFootprintPolicyV1, present?: Set<string>): readonly ComponentBatchV1[] {'),
 ('  return groups.map((group, index) => ({ pass: groups.length === 1 ? pass : { ...pass, id: `${pass.id}-b${index}`, packages: group.packages }, parent: pass.id, ...predict(group.identities) }));',
  '  return groups.map((group, index) => ({ pass: groups.length === 1 ? pass : { ...pass, id: `${pass.id}-b${index}`, packages: group.packages }, parent: pass.id, ...predict(present === undefined ? group.identities : batchAdditionsV1(group.identities, present)) }));'),
 ('  reclaimStorage?: (request: Readonly<{ needBytes: number }>) => Promise<void>;\n',
  '  reclaimStorage?: (request: Readonly<{ needBytes: number }>) => Promise<void>;\n  growStorage?: StorageGrowPortV1;\n'),
 ('inflight = new Map<string, Readonly<{ bytes: number; settled: Promise<void> }>>(), outputBytes = new Map<string, number>();',
  'inflight = new Map<string, Readonly<{ bytes: number; settled: Promise<void> }>>(), outputBytes = new Map<string, number>(), passWritten = new Map<string, number>(), passCompiled = new Map<string, number>();'),
 ('      outputBytes.set(unit.artifact, unitOutputBytesV1(unit.artifact, listed.get(directory)!, (name) => statSync(join(directory, name)).size));\n',
  '      const size = unitOutputBytesV1(unit.artifact, listed.get(directory)!, (name) => statSync(join(directory, name)).size);\n      outputBytes.set(unit.artifact, size);\n      passWritten.set(unit.pass, (passWritten.get(unit.pass) ?? 0) + size);\n      if (!unit.cache) passCompiled.set(unit.pass, (passCompiled.get(unit.pass) ?? 0) + size);\n'),
 ('  const availableBytes = (): number => headroomBytes() - [...inflight.values()].reduce((sum, row) => sum + row.bytes, 0);\n',
  '  const runningRows = (except?: string): readonly Readonly<{ id: string; predictedBytes: number; writtenBytes: number }>[] => { try { measureOutputs(); } catch { void 0; } return [...inflight].filter(([id]) => id !== except).map(([id, row]) => ({ id, predictedBytes: row.bytes, writtenBytes: passWritten.get(id) ?? 0 })); };\n  const availableBytes = (except?: string): number => headroomBytes() - runningRows(except).reduce((sum, row) => sum + Math.max(0, row.predictedBytes - row.writtenBytes), 0);\n'),
 ('@@GATE@@', '@@GATE@@'),
 ('            const available=availableBytes()+held.bytes;\n', '            const available=availableBytes(pass.id);\n'),
 ('      measureOutputs(pass.id);\n      finishedPasses.add(pass.id);\n',
  '      measureOutputs(pass.id);\n      recordStorageGrowthV1(options.cacheRoot, storageGrowthKeyV1(pass), passCompiled.get(pass.id) ?? 0, Date.now());\n      finishedPasses.add(pass.id);\n'),
 ('    batches = (await Promise.all(plan.passes.map(async (pass) => componentBatchesV1(pass, await unitGraphs.get(pass.id)!, history, budget - reserveBytes, { quantile: ADMISSION_POLICY_V1.quantile, bounds: policy.unitFootprintBoundsBytes })))).flat();\n',
  '    const growth = (() => { try { return readStorageGrowthV1(options.cacheRoot); } catch { return new Map<string, readonly number[]>(); } })(), present = new Map<string, Set<string>>(), planned: ComponentBatchV1[] = [];\n    for (const pass of plan.passes) {\n      const lane = `${pass.profile}|${pass.target}`;\n      if (!present.has(lane)) present.set(lane, new Set());\n      for (const row of componentBatchesV1(pass, await unitGraphs.get(pass.id)!, history, budget - reserveBytes, { quantile: ADMISSION_POLICY_V1.quantile, bounds: policy.unitFootprintBoundsBytes }, present.get(lane))) planned.push({ ...row, ...learnedBatchBytesV1(row, growth.get(storageGrowthKeyV1(row.pass))) });\n    }\n    batches = planned;\n'),
 ('import { admitPassStorageV1, batchAdditionsV1, learnedBatchBytesV1,',
  'import { admitPassStorageV1, batchAdditionsV1, creditPresentBytesV1, describeStorageGateV1, learnedBatchBytesV1, measurePresentDepsBytesV1,'),
 ('    batches = planned;\n',
  '    const credited = creditPresentBytesV1(planned, (row) => row.pass.profile, (lane) => measurePresentDepsBytesV1([generation.build], lane));\n    batches = credited;\n    for (const row of credited) if (row.credit > 0) log(`[native-build] ${describeStorageGateV1("credit", { pass: row.pass.id, credit: (row.credit / 1024 ** 2).toFixed(0), predicted: ((row.bytes + row.credit) / 1024 ** 2).toFixed(0), remaining: (row.bytes / 1024 ** 2).toFixed(0) })}`);\n'),
]
GATE_CALL = '    let release = (): void => undefined;\n    const settled = new Promise<void>((accept) => { release = accept; });\n    await admitPassStorageV1({ pass: pass.id, predictedBytes: predicted.bytes, measured: predicted.measured, reserveBytes, headroom: headroomBytes, running: () => runningRows(), settled: () => [...[...inflight.values()].map((row) => row.settled), ...pendingReclaims], hold: () => { inflight.set(pass.id, { bytes: predicted.bytes, settled }); }, reconcile: reconcileHeadroom, ...(options.reclaimStorage ? { reclaim: options.reclaimStorage } : {}), ...(options.growStorage ? { grow: options.growStorage } : {}), check, log, now: Date.now, sleep: (milliseconds) => new Promise<void>((accept) => setTimeout(accept, milliseconds)) });\n'
a = text.index("    while (availableBytes() < predicted.bytes + reserveBytes && (inflight.size || pendingReclaims.size))")
b = text.index("    try { await runAdmittedPass(pass, held); }", a)
if text.count("    while (availableBytes() < predicted.bytes + reserveBytes") != 1 or "Native build storage refused" not in text[a:b] or "inflight.set(pass.id" not in text[a:b]:
    sys.exit("gate block anchor mismatch")
text = text[:a] + GATE_CALL + text[b:]
edits = [row for row in edits if row[0] != "@@GATE@@"]
for old, new in edits:
    n = text.count(old)
    if n != 1:
        sys.exit(f"anchor matched {n} times: {old[:90]!r}")
    text = text.replace(old, new)
file.write_text(text)
print("edited", len(edits))
