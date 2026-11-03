p = "🟦️.ts"
s = open(p, encoding="utf-8").read()
s = s.replace('import { mkdtempSync, readFileSync, readdirSync, rmSync } from "node:fs";', 'import { mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";')
s = s.replace('import { fixture } from "../🧰️support/🧰️.ts";', 'import { STAFFAGE_ROOT, fixture, hasTool, python } from "../🧰️support/🧰️.ts";')
s = s.replace('''  it("answers a repeated shape from the memo''', '''  it.skipIf(!hasTool("uv"))("equals the shapely opening width of every shape within 3 % or 0.05 mm, the third-party oracle of the pinned values", { timeout: 300_000 }, () => {
    const directory = mkdtempSync(join(tmpdir(), "staffage-widths-"));
    try {
      const input = join(directory, "shapes.json");
      writeFileSync(input, JSON.stringify(vectors.featureWidth));
      const reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🚀️export-performance/🐍️.py"), ["numpy", "scipy", "shapely"], ["--widths", input])) as Record<string, number>;
      for (const shape of vectors.featureWidth) expect(Math.abs(shape.referenceMm - reference[shape.name]!), `${shape.name}: ${shape.referenceMm} against ${reference[shape.name]}`).toBeLessThanOrEqual(Math.max(0.03 * reference[shape.name]!, 0.05));
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });

  it("answers a repeated shape from the memo''', 1)
s = s.replace("the feature width against the pre-optimisation module and the analytic widths", "the feature width against pinned values (regenerated from the corrected measure), the analytic widths and a shapely opening oracle")
s = s.replace('it(`equals the pre-optimisation module on ${shape.name}', 'it(`equals the pinned width on ${shape.name}')
open(p, "w", encoding="utf-8").write(s)
