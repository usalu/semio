import { En1990ParseError, parseEn1990Artifact } from "./🟦️.ts";

const ok = parseEn1990Artifact({
  annex: "De",
  projectId: "p",
  structureKind: "building",
  altitudeM: 0,
  consequenceClass: 2,
  reliabilityClass: 2,
  designWorkingLifeCategory: 4,
  designWorkingLifeYears: 50,
  referencePeriodYears: 50,
  supervisionLevel: "DSL2",
  inspectionLevel: "IL2",
  kFiDeclared: 1,
  betaComputed: 3.8,
  permanents: [],
  variables: [],
  accidentals: [],
  seismics: [],
  members: [],
  bridgeSls: [],
  effects: [],
});
if (ok.projectId !== "p") throw new Error("valid parse failed");

let threw = false;
try {
  parseEn1990Artifact({ annex: "De", projectId: "p" });
} catch (e) {
  threw = e instanceof En1990ParseError && e.code === "missing";
}
if (!threw) throw new Error("expected missing-field En1990ParseError");

threw = false;
try {
  parseEn1990Artifact({ ...ok, consequenceClass: "2" as unknown as number });
} catch (e) {
  threw = e instanceof En1990ParseError && e.code === "mistyped" && e.path.includes("consequenceClass");
}
if (!threw) throw new Error("expected mistyped consequenceClass");

console.log("parse-en1990-artifact:ok");
