import sys
p = sys.argv[1]
s = open(p, encoding="utf8").read()


def rep(a, b):
    global s
    assert a in s, a[:70]
    s = s.replace(a, b, 1)


rep('export type Status = "ok" | "unavailable" | "fail";', 'export type Status = "ok" | "unavailable" | "range-end" | "fail";')
rep(
    "export type Failures = { readonly count: number; readonly first: readonly { readonly subject: string; readonly message: string }[] };",
    "export type Failures = { readonly count: number; readonly first: readonly { readonly subject: string; readonly message: string }[]; readonly worst?: { readonly deltaMm: number; readonly subject: string; readonly pair: string } };",
)
rep(
    "  readonly failures: Readonly<Record<string, Failures>>;\n};",
    "  readonly failures: Readonly<Record<string, Failures>>;\n  readonly rangeEnd?: Readonly<Record<string, Failures>>;\n};",
)
rep(
    "/** 🎯️ The tolerance of the consistency checks: the larger of 2 cm and 3 % of the largest dimension. */\nexport const toleranceM = (asset: Asset): number => Math.max(0.02, 0.03 * Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM));",
    "/** 🎯️ The D18 tolerance of a dimension: the larger of 15 mm and 2 % of it. */\nexport const d18 = (nominalM: number): number => Math.max(0.015, 0.02 * nominalM);\n\n/** 🎯️ The D18 tolerance of the synchrony checks: D18 of the largest dimension of the asset. */\nexport const toleranceM = (asset: Asset): number => d18(Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM));",
)
rep(
    "  fail(views: readonly Viewpoint[], modes: readonly View[], check: string, subject: string, message: string): void {\n    for (const view of views)\n      for (const mode of modes) {\n        const cell = this.cells.get(this.keyOf(view, mode))!;\n        const entry = cell.failures.get(check) ?? { count: 0, first: [] };\n        entry.count++;\n",
    "  fail(views: readonly Viewpoint[], modes: readonly View[], check: string, subject: string, message: string, delta?: { readonly deltaMm: number; readonly pair: string }): void {\n    for (const view of views)\n      for (const mode of modes) {\n        const cell = this.cells.get(this.keyOf(view, mode))!;\n        const entry = cell.failures.get(check) ?? { count: 0, first: [] as { subject: string; message: string }[] };\n        entry.count++;\n        if (delta !== undefined && (entry.worst === undefined || delta.deltaMm > entry.worst.deltaMm)) entry.worst = { deltaMm: delta.deltaMm, subject, pair: delta.pair };\n",
)
rep(
    "readonly cells = new Map<string, { checked: number; failures: Map<string, { count: number; first: { subject: string; message: string }[] }> }>();",
    "readonly cells = new Map<string, { checked: number; failures: Map<string, { count: number; first: { subject: string; message: string }[]; worst?: { deltaMm: number; subject: string; pair: string } }> }>();",
)
rep(
    "    if (!(Math.max(Math.abs(a[0] - b[0]), Math.abs(a[1] - b[1])) <= tolerance)) collector.fail(views, MODE_LIST, check, subject.id, `${label} differ: [${a[0].toFixed(3)}, ${a[1].toFixed(3)}] vs [${b[0].toFixed(3)}, ${b[1].toFixed(3)}] m (tolerance ${tolerance.toFixed(3)})`);",
    "    const delta = Math.max(Math.abs(a[0] - b[0]), Math.abs(a[1] - b[1]));\n    if (!(delta <= tolerance)) collector.fail(views, MODE_LIST, check, subject.id, `${label} differ: [${a[0].toFixed(3)}, ${a[1].toFixed(3)}] vs [${b[0].toFixed(3)}, ${b[1].toFixed(3)}] m (tolerance ${tolerance.toFixed(3)})`, { deltaMm: delta * 1000, pair: views.join(\"/\") });",
)
rep(
    "    if (!(Math.abs(measured - nominal) <= tolerance)) collector.fail([view], MODE_LIST, check, subject.id, `${label} is ${measured.toFixed(3)} m but the dimensions say ${nominal.toFixed(3)} m`);",
    "    if (!(Math.abs(measured - nominal) <= d18(nominal))) collector.fail([view], MODE_LIST, check, subject.id, `${label} is ${measured.toFixed(3)} m but the dimensions say ${nominal.toFixed(3)} m`, { deltaMm: Math.abs(measured - nominal) * 1000, pair: `${view} vs dimensions` });",
)
rep(
    "    const slack = (toleranceM(asset) + 0.001) * k + 1;\n    const gotW = (named.maxX - named.minX) / 1;\n    const gotH = named.maxY - named.minY;\n    if (Math.abs(gotW - w * k) > slack) fail(\"paper-width\", `drawn width ${gotW.toFixed(2)} mm but ${w}·${k.toFixed(3)} = ${(w * k).toFixed(2)} mm`);\n    if (Math.abs(gotH - h * k) > slack) fail(\"paper-height\", `drawn height ${gotH.toFixed(2)} mm but ${h}·${k.toFixed(3)} = ${(h * k).toFixed(2)} mm`);",
    "    const gotW = named.maxX - named.minX;\n    const gotH = named.maxY - named.minY;\n    if (Math.abs(gotW - w * k) > d18(w) * k) collector.fail([view], [mode], \"paper-width\", subject.id, `drawn width ${gotW.toFixed(2)} mm but ${w}·${k.toFixed(3)} = ${(w * k).toFixed(2)} mm`, { deltaMm: (Math.abs(gotW - w * k) / k) * 1000, pair: `${view} paper vs dimensions` });\n    if (Math.abs(gotH - h * k) > d18(h) * k) collector.fail([view], [mode], \"paper-height\", subject.id, `drawn height ${gotH.toFixed(2)} mm but ${h}·${k.toFixed(3)} = ${(h * k).toFixed(2)} mm`, { deltaMm: (Math.abs(gotH - h * k) / k) * 1000, pair: `${view} paper vs dimensions` });",
)
rep(
    'fail("blank-shapes", `no ${view} shape of the representation is drawn`);',
    'fail("blank-shapes", `no ${view} shape of the representation is drawn: ${blankReason(state, view, mode)}`);',
)
rep(
    "function checkScene(",
    """/** 🕳️ Why a scene draws none of the shapes of a view: per shape its role, pen and filled flag, then the reason class (`zero-pen`, `degenerate`, `occluded-or-unexplained`). */
export function blankReason(state: EvaluatedState, view: Viewpoint, mode: View): string {
  const rep = state.representation.kind === "scale" ? state.asset.representations[`${state.representation.denominator}` as keyof Asset["representations"]] : undefined;
  const pens = rep?.status === "authored" ? (rep.pens as unknown as Record<string, number | null>) : undefined;
  const lines = state.shapes[view]
    .filter((e) => e.shape.role !== "ground")
    .map((e) => {
      const points = e.rings.flat();
      const w = points.length === 0 ? 0 : Math.max(...points.map((p) => p.x)) - Math.min(...points.map((p) => p.x));
      const h = points.length === 0 ? 0 : Math.max(...points.map((p) => p.y)) - Math.min(...points.map((p) => p.y));
      const pen = e.shape.penMm ?? pens?.[e.shape.role] ?? null;
      const reason = points.length === 0 || (w < 1e-9 && h < 1e-9) ? "degenerate" : (pen === null || pen === 0) && !e.shape.filled ? "zero-pen" : "occluded-or-unexplained";
      return `${e.shape.id}[role ${e.shape.role}, pen ${pen === null ? "none" : pen} mm, ${e.shape.filled ? "filled" : "open"}, ${reason}]`;
    });
  return `${mode}: ${lines.length} shapes: ${lines.slice(0, 6).join("; ")}`;
}

function checkScene(""",
)
rep(
    'collector.fail([flag.view], MODE_LIST, "derived-view", "default pose",',
    'collector.fail([flag.view], MODE_LIST, flag.against.startsWith("1:") ? "derived-view-band-copy" : "derived-view-in-band", "default pose",',
)
rep(
    "      const failures = Object.fromEntries([...raw.failures].map(([check, value]) => [check, value]));",
    """      const all = Object.fromEntries([...raw.failures].map(([check, value]) => [check, value]));
      const rangeEndCheck = (entry.kind === "band-min" || entry.kind === "band-max") && all["unavailable-scale-unsuitable"] !== undefined;
      const { "unavailable-scale-unsuitable": rangeEnd, ...rest } = all;
      const failures = rangeEndCheck ? rest : all;""",
)
rep(
    '      const status: Status = Object.keys(failures).length > 0 ? "fail" : reason !== undefined ? "unavailable" : "ok";\n      return { asset: asset.id, category: asset.category, scale: entry.scale, scaleKind: entry.kind, requested: entry.requested, view, mode, status, checked: raw.checked, ...(reason !== undefined ? { reason } : {}), failures };',
    '      const status: Status = Object.keys(failures).length > 0 ? "fail" : rangeEndCheck ? "range-end" : reason !== undefined ? "unavailable" : "ok";\n      return { asset: asset.id, category: asset.category, scale: entry.scale, scaleKind: entry.kind, requested: entry.requested, view, mode, status, checked: raw.checked, ...(reason !== undefined ? { reason } : {}), failures, ...(rangeEndCheck ? { rangeEnd: { "range-end-unsuitable": rangeEnd! } } : {}) };',
)
rep("const zero = (): Counts => ({ ok: 0, unavailable: 0, fail: 0 });", 'const zero = (): Counts => ({ ok: 0, unavailable: 0, "range-end": 0, fail: 0 });')
rep("export type Counts = { ok: number; unavailable: number; fail: number };", 'export type Counts = { ok: number; unavailable: number; "range-end": number; fail: number };')
rep(
    'modes: instant.tag === "uniform" || instant.tag === "mid" ? SAMPLED_MODES : MODE_LIST }));',
    'modes: MODE_LIST, languages: instant.tag === "uniform" || instant.tag === "mid" ? ["en"] : LANGUAGES }));',
)
rep("modes: SAMPLED_MODES })));", 'modes: MODE_LIST, languages: ["en"] })));')
rep(
    "isDefault: pose.id === asset.presentation.defaultPose, modes: MODE_LIST }));",
    "isDefault: pose.id === asset.presentation.defaultPose, modes: MODE_LIST, languages: LANGUAGES }));",
)
rep(
    "readonly isDefault: boolean; readonly modes: readonly View[] };",
    "readonly isDefault: boolean; readonly modes: readonly View[]; readonly languages: readonly Language[] };",
)
rep(
    "/** 🎨️ The representative modes rendered",
    '/** 🌍️ The languages rendered at poses and at the start, key, extreme and end instants; the other instants render English. */\nexport const LANGUAGES: readonly Language[] = ["en", "de"];\n\n/** 🎨️ The representative modes rendered',
)
rep("type AuthoredRepresentation, type SymbolRepresentation,", "type AuthoredRepresentation, type Language, type SymbolRepresentation,")
rep("isDefault: false, modes: MODE_LIST }, source, collector, symbol);", 'isDefault: false, modes: MODE_LIST, languages: ["en"] }, source, collector, symbol);')
rep(
    "export function drawOptions(mode: View, denominator: number, textless = false): DrawOptions {",
    'export function drawOptions(mode: View, denominator: number, textless = false, language: Language = "en"): DrawOptions {',
)
rep('defaultDepictionOptions(mode, "en", denominator);\n  return rest as DrawOptions;', "defaultDepictionOptions(mode, language, denominator);\n  return rest as DrawOptions;")
open(p, "w", encoding="utf8").write(s)
