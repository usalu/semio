#!/usr/bin/env bun
import concreteForest from "../../../../../../puzzle/5d/fixture/concrete-forest.5d.json";
import { brushCompatibleCandidates } from "../../../../../../puzzle/3d/react/index.tsx";
import { parseModel, project3dKindCatalogs } from "../../../../../../puzzle/5d/react/index.tsx";

const model = parseModel(concreteForest);
if (!model) {
  console.error("[DEBUG] parseModel failed");
  process.exit(1);
}
const cat3d = project3dKindCatalogs(model.kindCatalogs);
const targetGrip = model.parts[0]?.grips.find((grip) => grip.gripKind === "b-l");
const candidates = brushCompatibleCandidates(
  {
    objectId: model.parts[0]!.id,
    objectKind: model.parts[0]!.partKind ?? "",
    vortexKind: targetGrip?.gripKind ?? "b-l",
  },
  cat3d,
  model.kindCompatibility,
);
console.log("[DEBUG] concrete-forest 5d projected catalog objects:", cat3d?.objects?.length ?? 0);
console.log("[DEBUG] concrete-forest 5d brush candidates:", candidates.length);
if (candidates.length === 0) {
  process.exit(1);
}
