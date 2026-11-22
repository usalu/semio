"""One-shot editor of the tests that pinned the emitter cones: they pin the flow geometry instead."""
import sys, pathlib
S = pathlib.Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage")
def edit(rel, pairs):
    path = S / rel
    text = path.read_text(encoding="utf-8")
    for old, new in pairs:
        if text.count(old) != 1:
            sys.exit(f"{rel}: expected one match of {old[:80]!r}, found {text.count(old)}")
        text = text.replace(old, new)
    path.write_text(text, encoding="utf-8")

edit("🧪️tests/🧊️model-glyph-size/🟦️.ts", [
    ("(force and reaction arrows, moment arcs, emitter plumes, the centre-of-mass mark) are the paper size at the scale but at most 0.35 (arrows, plumes) and 0.06 (centre-of-mass radius) of the largest extent of the asset, so that at 1:200 and 1:500 the body still fills the frame; drawings keep pure paper sizes.",
     "(force and reaction arrows, moment arcs, the centre-of-mass mark) are the paper size at the scale but at most 0.35 (arrows) and 0.06 (centre-of-mass radius) of the largest extent of the asset, so that at 1:200 and 1:500 the body still fills the frame; drawings keep pure paper sizes. The stream tubes of the air flow follow the same idea: half the paper width of the stream at the scale, but at most 0.012 of the largest extent in radius, the arrow heads shrinking with them."),
    ("import { ARROW_SHARE, MASS_SHARE, modelSizing, paperSizing } from \"../../🔨️modules/🧊️volume/🪧️glyphs/🟦️.ts\";",
     "import { FLOW_STYLE } from \"../../🔨️modules/🎨️depiction/🌊️flow-view/🟦️.ts\";\nimport { FLOW_MODEL, flowUnitOf } from \"../../🔨️modules/🧊️volume/🌊️flow/🟦️.ts\";\nimport { ARROW_SHARE, MASS_SHARE, modelSizing, paperSizing } from \"../../🔨️modules/🧊️volume/🪧️glyphs/🟦️.ts\";"),
    ("massRadiusM: 0.65, plumeM: 2.5 });", "massRadiusM: 0.65 });"),
    ("    expect(sizing.plumeM).toBeCloseTo(0.7, 12);\n", ""),
    ("    expect([fine.unit, fine.massRadiusM, fine.plumeM]).toEqual([0.001, expect.closeTo(0.0013, 12), 0.005]);",
     "    expect([fine.unit, fine.massRadiusM]).toEqual([0.001, expect.closeTo(0.0013, 12)]);"),
    ("  it(\"keeps the arrow of a coarse scale as large relative to the body as the cap says: 1:500 is no longer than 1:50\", async () => {",
     "  it(\"sizes the stream tubes and arrow heads of the air flow by paper width, capped at 0.012 of the extent in radius\", { timeout: 240_000 }, async () => {\n    expect(FLOW_MODEL.tubeShare).toBe(0.012);\n    expect(flowUnitOf(50, 1.73)).toBe(0.05);\n    expect(flowUnitOf(500, 1.73)).toBeCloseTo((0.012 * 1.73) / 0.15, 12);\n    const asset = scaledFigure();\n    const extent = extentOf(asset);\n    for (const scale of authoredScales(asset.volumes)) {\n      const document = await modelOf(asset, { scale, view: \"thermal\", poses: [\"stand\"], clips: [] });\n      const unit = flowUnitOf(scale, extent);\n      const tubes = document.solids.filter((s) => s.kind === \"loft\" && s.tags?.includes(\"flow\") === true) as unknown as { sections: { rx: number; ry: number }[] }[];\n      const heads = document.solids.filter((s) => s.kind === \"revolve\" && s.tags?.includes(\"flow\") === true) as unknown as { profile: [number, number][] }[];\n      expect(tubes.length, `tubes at 1:${scale}`).toBeGreaterThan(0);\n      expect(heads.length, `arrow heads at 1:${scale}`).toBeGreaterThan(0);\n      const radii = tubes.flatMap((t) => t.sections.map((s) => s.rx));\n      for (const t of tubes) for (const s of t.sections) expect(s.ry).toBe(s.rx);\n      expect(Math.max(...radii), `widest tube at 1:${scale}`).toBeLessThanOrEqual(Math.min((FLOW_STYLE.streamMaxMm / 2) * (scale / 1000), FLOW_MODEL.tubeShare * extent) * (1 + 1e-12));\n      expect(Math.min(...radii), `thinnest tube at 1:${scale}`).toBeGreaterThanOrEqual((FLOW_STYLE.streamMinMm / 2) * unit * (1 - 1e-12));\n      for (const h of heads) expect(h.profile).toEqual([[-FLOW_STYLE.headLengthMm * unit, 0], [-FLOW_STYLE.headLengthMm * unit, (FLOW_STYLE.headWidthMm / 2) * unit], [0, 0]]);\n    }\n  });\n\n  it(\"keeps the arrow of a coarse scale as large relative to the body as the cap says: 1:500 is no longer than 1:50\", async () => {"),
])
edit("🧪️tests/🧊️model-glyph-size/🥒️.feature", [
    ("    Then no arrow or plume is longer than 0.35", "    Then no arrow is longer than 0.35"),
    ("  Scenario: Paper stays paper",
     "  Scenario: Stream tubes of the air flow\n    Given the thermal scenes of an asset with flowing air at 1:50, 1:100, 1:200 and 1:500\n    Then every stream tube is round, its radius is half the paper width of the stream at the scale (0.09 to 0.3 mm of paper) and at most 0.012 of the largest extent of the asset, and every arrow head is 1.1 by 0.75 mm of paper shrunk by the same cap\n  Scenario: Paper stays paper"),
])
edit("🧪️tests/🧊️model-glyph-edges/🟦️.ts", [
    ("the outline of a force arrow, moment arc or emitter cone is visible exactly in the variants in which its surface is visible, never under another view.",
     "the outline of a force arrow, moment arc or of a tube end, arrow head or puff of the air flow is visible exactly in the variants in which its surface is visible, never under another view; the flow shows in the thermal variant only."),
    ("    if (asset.id !== sampleFigure().id) expect(glyphsSeen, \"the asset has glyphs in some variant\").toBeGreaterThan(0);",
     "    expect(glyphsSeen, \"the asset has glyphs in some variant\").toBeGreaterThan(0);\n    const flowOf = (variant: string): string[] => [...(visible[variant]?.triangles ?? [])].filter((solid) => solid.includes(\"-flow-\"));\n    expect(flowOf(\"thermal\").length, \"the thermal variant shows the air flow\").toBeGreaterThan(0);\n    for (const variant of Object.keys(visible)) if (variant !== \"thermal\") expect(flowOf(variant), `${variant} hides the air flow`).toEqual([]);\n    for (const solid of Object.values(visible).flatMap((entry) => [...entry.triangles, ...entry.lines])) expect(solid).not.toMatch(/emitter/u);"),
])
edit("🧪️tests/🧊️model-glyph-edges/🥒️.feature", [
    ("A force arrow, a moment arc or an emitter cone is drawn only by the variant of its view.", "A force arrow, a moment arc or the air flow (stream tubes, arrow heads, vapour and fog puffs) is drawn only by the variant of its view."),
    ("    Given the GLB of a pose of an asset with force arrows, moment arcs or emitter cones", "    Given the GLB of a pose of an asset with force arrows, moment arcs or flowing air"),
])
edit("🧪️tests/🧊️axonometric-depiction/🟦️.ts", [
    ("    expect([...thermal].filter((id) => !own.has(id) && !id.startsWith(\"emitter-\"))).toEqual([]);", "    expect([...thermal].filter((id) => !own.has(id))).toEqual([]);"),
])
print("ok")
