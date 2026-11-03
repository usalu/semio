import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2]!;
let t = readFileSync(file, "utf8");
const from = `  it("measures a representation that authors no front or plan geometry", () => {
    const strip = (rep: AuthoredRepresentation): AuthoredRepresentation => ({ ...rep, geometry: { elevation: rep.geometry.elevation } as AuthoredRepresentation["geometry"] });
    const result = measure(100, [rect("a", 0, 0.25, 1.8)], strip);
    expect(result.violations).toEqual([]);
    expect(result.suitable).toBe(true);
  });`;
if (!t.includes(from)) throw new Error("anchor");
t = t.replace(from, `  it("refuses a representation that authors no front or plan geometry instead of measuring only the side elevation", () => {
    const strip = (rep: AuthoredRepresentation): AuthoredRepresentation => ({ ...rep, geometry: { elevation: rep.geometry.elevation } as AuthoredRepresentation["geometry"] });
    expect(() => measure(100, [rect("a", 0, 0.25, 1.8)], strip)).toThrow();
  });`);
writeFileSync(file, t);
