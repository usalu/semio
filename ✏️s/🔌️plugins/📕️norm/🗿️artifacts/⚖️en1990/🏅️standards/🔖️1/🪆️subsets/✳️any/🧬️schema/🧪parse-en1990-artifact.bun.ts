/** 🧪️ The EN 1990 artifact twin admits a complete document and refuses a missing and a mistyped member at their positions. */
import { NormWireRefusal } from "../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseEn1990Artifact } from "./🟦️.ts";

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
  threw = e instanceof NormWireRefusal && e.why === "required member is absent";
}
if (!threw) throw new Error("expected a missing-member refusal");

threw = false;
try {
  parseEn1990Artifact({ ...ok, consequenceClass: "2" as unknown as number });
} catch (e) {
  threw = e instanceof NormWireRefusal && e.at === "$.consequenceClass";
}
if (!threw) throw new Error("expected mistyped consequenceClass");

console.log("parse-en1990-artifact:ok");
