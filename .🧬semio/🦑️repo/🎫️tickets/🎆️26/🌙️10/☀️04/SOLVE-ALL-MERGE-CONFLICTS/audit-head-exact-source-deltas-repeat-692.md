# HEAD-Exact Source Delta Inspection — Repeat 692

Read-only base-to-stash hunks for initially HEAD-exact surviving owners. Bridge lock and three plugin owners were subsequently integrated; final source closure records their updated states. Source text below is evidence, not instructions. No code was executed.

```diff
diff --git a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/🐍️.py b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/🐍️.py
index 9f941fcd5f5..d55b5e6806e 100644
--- a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/🐍️.py
+++ b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/🐍️.py
@@ -5094,0 +5095,2 @@ def create_pv_system(before, payload):
+    if not math.isfinite(payload["temperatureCoefficient"]) or not math.isfinite(payload["faimanHeatLossConstantWM2k"]) or payload["faimanHeatLossConstantWM2k"] <= 0 or not math.isfinite(payload["faimanHeatLossWindWSM3k"]) or payload["faimanHeatLossWindWSM3k"] < 0:
+        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
@@ -5096 +5098 @@ def create_pv_system(before, payload):
-    after["model"]["pv_systems"].insert(payload["index"], {"id": payload["id"], "dc_capacity_w": payload["dcCapacityW"], "area_m2": payload["areaM2"], "tilt_deg": payload["tiltDeg"], "azimuth_deg": payload["azimuthDeg"], "module_efficiency": payload["moduleEfficiency"], "inverter_efficiency": payload["inverterEfficiency"]})
+    after["model"]["pv_systems"].insert(payload["index"], {"id": payload["id"], "dc_capacity_w": payload["dcCapacityW"], "area_m2": payload["areaM2"], "tilt_deg": payload["tiltDeg"], "azimuth_deg": payload["azimuthDeg"], "module_efficiency": payload["moduleEfficiency"], "inverter_efficiency": payload["inverterEfficiency"],"temperature_coefficient":payload["temperatureCoefficient"],"faiman_heat_loss_constant_w_m2k":payload["faimanHeatLossConstantWM2k"],"faiman_heat_loss_wind_w_s_m3k":payload["faimanHeatLossWindWSM3k"],"mounting":payload["mounting"]})
@@ -5122 +5124 @@ def _invert_delete_pv_system(before, payload):
-    return [("create-pv-system", {"index": index, "id": item["id"], "dcCapacityW": item["dc_capacity_w"], "areaM2": item["area_m2"], "tiltDeg": item["tilt_deg"], "azimuthDeg": item["azimuth_deg"], "moduleEfficiency": item["module_efficiency"], "inverterEfficiency": item["inverter_efficiency"]})]
+    return [("create-pv-system", {"index": index, "id": item["id"], "dcCapacityW": item["dc_capacity_w"], "areaM2": item["area_m2"], "tiltDeg": item["tilt_deg"], "azimuthDeg": item["azimuth_deg"], "moduleEfficiency": item["module_efficiency"], "inverterEfficiency": item["inverter_efficiency"],"temperatureCoefficient":item["temperature_coefficient"],"faimanHeatLossConstantWM2k":item["faiman_heat_loss_constant_w_m2k"],"faimanHeatLossWindWSM3k":item["faiman_heat_loss_wind_w_s_m3k"],"mounting":item["mounting"]})]
diff --git a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts
index ef2229da63c..4f62e841e84 100644
--- a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts
+++ b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts
@@ -174,2 +174,5 @@ type puzzlePuzzle5dDiffGuardSizeBounds = { readonly minItems?: number; readonly
-export const puzzlePuzzle5dDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
-  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle5dDiffGuardReject(at, "value is not an object");
+export const puzzlePuzzle5dDiffGuardObject = (value: unknown, at: string, keys?: readonly string[]): Readonly<Record<string, unknown>> => {
+  if (value === null || typeof value !== "object" || Array.isArray(value)) return puzzlePuzzle5dDiffGuardReject(at, "value is not an object");
+  if (keys && Object.keys(value).some(key => !keys.includes(key))) return puzzlePuzzle5dDiffGuardReject(at, "unknown field");
+  return value as Record<string, unknown>;
+};
@@ -206 +209 @@ export function parsePuzzle5dKindCompatibilityKey(value: unknown, at = "$"): Puz
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["source","target"]);
@@ -214 +217 @@ export function parsePuzzle5dPart2dPatch(value: unknown, at = "$"): Puzzle5dPart
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["x","y","shape","radius","width","height","text","iconKind","hidden","locked"]);
@@ -230 +233 @@ export function parsePuzzle5dPart3dPatch(value: unknown, at = "$"): Puzzle5dPart
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["origin","meshUrl","orientation","scale","label"]);
@@ -232 +235 @@ export function parsePuzzle5dPart3dPatch(value: unknown, at = "$"): Puzzle5dPart
-    origin: row["origin"] === undefined ? undefined : puzzlePuzzle5dDiffGuardArray(row["origin"], `${at}.origin`, { minItems: 3, maxItems: 3 }).map((item, index) => puzzlePuzzle5dDiffGuardNumber(item, `${${at}.origin}[${index}]`)),
+    origin: row["origin"] === undefined ? undefined : puzzlePuzzle5dDiffGuardArray(row["origin"], `${at}.origin`, { minItems: 3, maxItems: 3 }).map((item, index) => puzzlePuzzle5dDiffGuardNumber(item, `${at}.origin[${index}]`)),
@@ -234 +237 @@ export function parsePuzzle5dPart3dPatch(value: unknown, at = "$"): Puzzle5dPart
-    orientation: row["orientation"] === undefined ? undefined : row["orientation"] === null ? null : puzzlePuzzle5dDiffGuardArray(row["orientation"], `${at}.orientation`, { minItems: 4, maxItems: 4 }).map((item, index) => puzzlePuzzle5dDiffGuardNumber(item, `${${at}.orientation}[${index}]`)),
+    orientation: row["orientation"] === undefined ? undefined : row["orientation"] === null ? null : puzzlePuzzle5dDiffGuardArray(row["orientation"], `${at}.orientation`, { minItems: 4, maxItems: 4 }).map((item, index) => puzzlePuzzle5dDiffGuardNumber(item, `${at}.orientation[${index}]`)),
@@ -241 +244 @@ export function parsePuzzle5dGrip2dPatch(value: unknown, at = "$"): Puzzle5dGrip
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["angle","gripKind","radius"]);
@@ -250 +253 @@ export function parsePuzzle5dGrip3dPatch(value: unknown, at = "$"): Puzzle5dGrip
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["position","direction","radius","label"]);
@@ -252,2 +255,2 @@ export function parsePuzzle5dGrip3dPatch(value: unknown, at = "$"): Puzzle5dGrip
-    position: row["position"] === undefined ? undefined : puzzlePuzzle5dDiffGuardArray(row["position"], `${at}.position`, { minItems: 3, maxItems: 3 }).map((item, index) => puzzlePuzzle5dDiffGuardNumber(item, `${${at}.position}[${index}]`)),
-    direction: row["direction"] === undefined ? undefined : row["direction"] === null ? null : puzzlePuzzle5dDiffGuardArray(row["direction"], `${at}.direction`, { minItems: 3, maxItems: 3 }).map((item, index) => puzzlePuzzle5dDiffGuardNumber(item, `${${at}.direction}[${index}]`)),
+    position: row["position"] === undefined ? undefined : puzzlePuzzle5dDiffGuardArray(row["position"], `${at}.position`, { minItems: 3, maxItems: 3 }).map((item, index) => puzzlePuzzle5dDiffGuardNumber(item, `${at}.position[${index}]`)),
+    direction: row["direction"] === undefined ? undefined : row["direction"] === null ? null : puzzlePuzzle5dDiffGuardArray(row["direction"], `${at}.direction`, { minItems: 3, maxItems: 3 }).map((item, index) => puzzlePuzzle5dDiffGuardNumber(item, `${at}.direction[${index}]`)),
@@ -260 +263 @@ export function parsePuzzle5dGripPatch(value: unknown, at = "$"): Puzzle5dGripPa
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["gripKind","2d","3d"]);
@@ -269 +272 @@ export function parsePuzzle5dPartPatch(value: unknown, at = "$"): Puzzle5dPartPa
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["partKind","anchor","2d","3d","grips"]);
@@ -280 +283 @@ export function parsePuzzle5dFastenerPatch(value: unknown, at = "$"): Puzzle5dFa
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["source","target","fastenerKind","gap","shift","rise","rotation","turn","tilt","x","y"]);
@@ -297 +300 @@ export function parsePuzzle5dTargetVolumePatch(value: unknown, at = "$"): Puzzle
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["origin","orientation","scale","hidden","locked"]);
@@ -299,2 +302,2 @@ export function parsePuzzle5dTargetVolumePatch(value: unknown, at = "$"): Puzzle
-    origin: row["origin"] === undefined ? undefined : puzzlePuzzle5dDiffGuardArray(row["origin"], `${at}.origin`, { minItems: 3, maxItems: 3 }).map((item, index) => puzzlePuzzle5dDiffGuardNumber(item, `${${at}.origin}[${index}]`)),
-    orientation: row["orientation"] === undefined ? undefined : row["orientation"] === null ? null : puzzlePuzzle5dDiffGuardArray(row["orientation"], `${at}.orientation`, { minItems: 4, maxItems: 4 }).map((item, index) => puzzlePuzzle5dDiffGuardNumber(item, `${${at}.orientation}[${index}]`)),
+    origin: row["origin"] === undefined ? undefined : puzzlePuzzle5dDiffGuardArray(row["origin"], `${at}.origin`, { minItems: 3, maxItems: 3 }).map((item, index) => puzzlePuzzle5dDiffGuardNumber(item, `${at}.origin[${index}]`)),
+    orientation: row["orientation"] === undefined ? undefined : row["orientation"] === null ? null : puzzlePuzzle5dDiffGuardArray(row["orientation"], `${at}.orientation`, { minItems: 4, maxItems: 4 }).map((item, index) => puzzlePuzzle5dDiffGuardNumber(item, `${at}.orientation[${index}]`)),
@@ -308 +311 @@ export function parsePuzzle5dKindCompatibilityPatch(value: unknown, at = "$"): P
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["bidirectional","important","specificity"]);
@@ -317 +320 @@ export function parsePuzzle5dMetaPatch(value: unknown, at = "$"): Puzzle5dMetaPa
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["description"]);
@@ -324 +327 @@ export function parsePuzzle5dGripRemoval(value: unknown, at = "$"): Puzzle5dGrip
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["id","index"]);
@@ -327 +330 @@ export function parsePuzzle5dGripRemoval(value: unknown, at = "$"): Puzzle5dGrip
-    index: puzzlePuzzle5dDiffGuardNumber(row["index"], `${at}.index`),
+    index: puzzlePuzzle5dDiffGuardInteger(row["index"], `${at}.index`, { minimum: 0 }),
@@ -332 +335 @@ export function parsePuzzle5dGripInsertion(value: unknown, at = "$"): Puzzle5dGr
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["index","row"]);
@@ -334 +337 @@ export function parsePuzzle5dGripInsertion(value: unknown, at = "$"): Puzzle5dGr
-    index: puzzlePuzzle5dDiffGuardNumber(row["index"], `${at}.index`),
+    index: puzzlePuzzle5dDiffGuardInteger(row["index"], `${at}.index`, { minimum: 0 }),
@@ -340 +343 @@ export function parsePuzzle5dGripRelocation(value: unknown, at = "$"): Puzzle5dG
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["id","from","to"]);
@@ -343,2 +346,2 @@ export function parsePuzzle5dGripRelocation(value: unknown, at = "$"): Puzzle5dG
-    from: puzzlePuzzle5dDiffGuardNumber(row["from"], `${at}.from`),
-    to: puzzlePuzzle5dDiffGuardNumber(row["to"], `${at}.to`),
+    from: puzzlePuzzle5dDiffGuardInteger(row["from"], `${at}.from`, { minimum: 0 }),
+    to: puzzlePuzzle5dDiffGuardInteger(row["to"], `${at}.to`, { minimum: 0 }),
@@ -349 +352 @@ export function parsePuzzle5dGripModification(value: unknown, at = "$"): Puzzle5
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["id","patch"]);
@@ -357 +360 @@ export function parsePuzzle5dGripsDelta(value: unknown, at = "$"): Puzzle5dGrips
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["removed","inserted","moved","modified"]);
@@ -367 +370 @@ export function parsePuzzle5dPartRemoval(value: unknown, at = "$"): Puzzle5dPart
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["id","index"]);
@@ -370 +373 @@ export function parsePuzzle5dPartRemoval(value: unknown, at = "$"): Puzzle5dPart
-    index: puzzlePuzzle5dDiffGuardNumber(row["index"], `${at}.index`),
+    index: puzzlePuzzle5dDiffGuardInteger(row["index"], `${at}.index`, { minimum: 0 }),
@@ -375 +378 @@ export function parsePuzzle5dPartInsertion(value: unknown, at = "$"): Puzzle5dPa
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["index","row"]);
@@ -377 +380 @@ export function parsePuzzle5dPartInsertion(value: unknown, at = "$"): Puzzle5dPa
-    index: puzzlePuzzle5dDiffGuardNumber(row["index"], `${at}.index`),
+    index: puzzlePuzzle5dDiffGuardInteger(row["index"], `${at}.index`, { minimum: 0 }),
@@ -383 +386 @@ export function parsePuzzle5dPartRelocation(value: unknown, at = "$"): Puzzle5dP
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["id","from","to"]);
@@ -386,2 +389,2 @@ export function parsePuzzle5dPartRelocation(value: unknown, at = "$"): Puzzle5dP
-    from: puzzlePuzzle5dDiffGuardNumber(row["from"], `${at}.from`),
-    to: puzzlePuzzle5dDiffGuardNumber(row["to"], `${at}.to`),
+    from: puzzlePuzzle5dDiffGuardInteger(row["from"], `${at}.from`, { minimum: 0 }),
+    to: puzzlePuzzle5dDiffGuardInteger(row["to"], `${at}.to`, { minimum: 0 }),
@@ -392 +395 @@ export function parsePuzzle5dPartModification(value: unknown, at = "$"): Puzzle5
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["id","patch"]);
@@ -400 +403 @@ export function parsePuzzle5dPartsDelta(value: unknown, at = "$"): Puzzle5dParts
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["removed","inserted","moved","modified"]);
@@ -410 +413 @@ export function parsePuzzle5dFastenerRemoval(value: unknown, at = "$"): Puzzle5d
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["id","index"]);
@@ -413 +416 @@ export function parsePuzzle5dFastenerRemoval(value: unknown, at = "$"): Puzzle5d
-    index: puzzlePuzzle5dDiffGuardNumber(row["index"], `${at}.index`),
+    index: puzzlePuzzle5dDiffGuardInteger(row["index"], `${at}.index`, { minimum: 0 }),
@@ -418 +421 @@ export function parsePuzzle5dFastenerInsertion(value: unknown, at = "$"): Puzzle
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["index","row"]);
@@ -420 +423 @@ export function parsePuzzle5dFastenerInsertion(value: unknown, at = "$"): Puzzle
-    index: puzzlePuzzle5dDiffGuardNumber(row["index"], `${at}.index`),
+    index: puzzlePuzzle5dDiffGuardInteger(row["index"], `${at}.index`, { minimum: 0 }),
@@ -426 +429 @@ export function parsePuzzle5dFastenerRelocation(value: unknown, at = "$"): Puzzl
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["id","from","to"]);
@@ -429,2 +432,2 @@ export function parsePuzzle5dFastenerRelocation(value: unknown, at = "$"): Puzzl
-    from: puzzlePuzzle5dDiffGuardNumber(row["from"], `${at}.from`),
-    to: puzzlePuzzle5dDiffGuardNumber(row["to"], `${at}.to`),
+    from: puzzlePuzzle5dDiffGuardInteger(row["from"], `${at}.from`, { minimum: 0 }),
+    to: puzzlePuzzle5dDiffGuardInteger(row["to"], `${at}.to`, { minimum: 0 }),
@@ -435 +438 @@ export function parsePuzzle5dFastenerModification(value: unknown, at = "$"): Puz
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["id","patch"]);
@@ -443 +446 @@ export function parsePuzzle5dFastenersDelta(value: unknown, at = "$"): Puzzle5dF
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["removed","inserted","moved","modified"]);
@@ -453 +456 @@ export function parsePuzzle5dTargetVolumeRemoval(value: unknown, at = "$"): Puzz
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["id","index"]);
@@ -456 +459 @@ export function parsePuzzle5dTargetVolumeRemoval(value: unknown, at = "$"): Puzz
-    index: puzzlePuzzle5dDiffGuardNumber(row["index"], `${at}.index`),
+    index: puzzlePuzzle5dDiffGuardInteger(row["index"], `${at}.index`, { minimum: 0 }),
@@ -461 +464 @@ export function parsePuzzle5dTargetVolumeInsertion(value: unknown, at = "$"): Pu
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["index","row"]);
@@ -463 +466 @@ export function parsePuzzle5dTargetVolumeInsertion(value: unknown, at = "$"): Pu
-    index: puzzlePuzzle5dDiffGuardNumber(row["index"], `${at}.index`),
+    index: puzzlePuzzle5dDiffGuardInteger(row["index"], `${at}.index`, { minimum: 0 }),
@@ -469 +472 @@ export function parsePuzzle5dTargetVolumeRelocation(value: unknown, at = "$"): P
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["id","from","to"]);
@@ -472,2 +475,2 @@ export function parsePuzzle5dTargetVolumeRelocation(value: unknown, at = "$"): P
-    from: puzzlePuzzle5dDiffGuardNumber(row["from"], `${at}.from`),
-    to: puzzlePuzzle5dDiffGuardNumber(row["to"], `${at}.to`),
+    from: puzzlePuzzle5dDiffGuardInteger(row["from"], `${at}.from`, { minimum: 0 }),
+    to: puzzlePuzzle5dDiffGuardInteger(row["to"], `${at}.to`, { minimum: 0 }),
@@ -478 +481 @@ export function parsePuzzle5dTargetVolumeModification(value: unknown, at = "$"):
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["id","patch"]);
@@ -486 +489 @@ export function parsePuzzle5dTargetVolumesDelta(value: unknown, at = "$"): Puzzl
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["removed","inserted","moved","modified"]);
@@ -496 +499 @@ export function parsePuzzle5dKindCompatibilityRemoval(value: unknown, at = "$"):
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["id","index"]);
@@ -499 +502 @@ export function parsePuzzle5dKindCompatibilityRemoval(value: unknown, at = "$"):
-    index: puzzlePuzzle5dDiffGuardNumber(row["index"], `${at}.index`),
+    index: puzzlePuzzle5dDiffGuardInteger(row["index"], `${at}.index`, { minimum: 0 }),
@@ -504 +507 @@ export function parsePuzzle5dKindCompatibilityInsertion(value: unknown, at = "$"
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["index","row"]);
@@ -506 +509 @@ export function parsePuzzle5dKindCompatibilityInsertion(value: unknown, at = "$"
-    index: puzzlePuzzle5dDiffGuardNumber(row["index"], `${at}.index`),
+    index: puzzlePuzzle5dDiffGuardInteger(row["index"], `${at}.index`, { minimum: 0 }),
@@ -512 +515 @@ export function parsePuzzle5dKindCompatibilityRelocation(value: unknown, at = "$
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["id","from","to"]);
@@ -515,2 +518,2 @@ export function parsePuzzle5dKindCompatibilityRelocation(value: unknown, at = "$
-    from: puzzlePuzzle5dDiffGuardNumber(row["from"], `${at}.from`),
-    to: puzzlePuzzle5dDiffGuardNumber(row["to"], `${at}.to`),
+    from: puzzlePuzzle5dDiffGuardInteger(row["from"], `${at}.from`, { minimum: 0 }),
+    to: puzzlePuzzle5dDiffGuardInteger(row["to"], `${at}.to`, { minimum: 0 }),
@@ -521 +524 @@ export function parsePuzzle5dKindCompatibilityModification(value: unknown, at =
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["id","patch"]);
@@ -529 +532 @@ export function parsePuzzle5dKindCompatibilityDelta(value: unknown, at = "$"): P
-  const row = puzzlePuzzle5dDiffGuardObject(value, at);
+  const row = puzzlePuzzle5dDiffGuardObject(value, at, ["removed","inserted","moved","modified"]);
diff --git a/🎓️teaching/🏛️architecture/❓️quiz/README.md b/🎓️teaching/🏛️architecture/❓️quiz/README.md
deleted file mode 100644
index 3cd0c0baba8..00000000000
--- a/🎓️teaching/🏛️architecture/❓️quiz/README.md
+++ /dev/null
@@ -1,562 +0,0 @@
-# ❓️ Quizze · Architektur und Technologie
-
-The quiz website of **quizze.architektur-und-technologie.de** and its proctor at **semio.iek.uni-hannover.de**: the
-architecture catalog (`🔣️.json`, id `architecture`) with the quizzes it names, rendered by `@semio-tech/quiz-react` in a
-static site on a CDN and proctored by `proctor` (`🎓️teaching/🛂️proctor`), an API-only server over one SQLite file that
-runs as a zero-touch Docker stack. Both hosts, the image and the port live in one place, `🚀️deploy/🔣️.json`; the Vite
-config, the operator verbs and the tests read them there.
-
-| Path | What |
-|---|---|
-| `🔣️.json` | the catalog: introduction, quiz paths (relative to this file), badges |
-| `🟦️.ts`, `🌐️.html` | the site entry: `mountQuiz(root, { proctor: <baked proctor origin or "">, tenant: "architecture", material, logo, legal: site.legal, pets: () => import("../🐾️pets/🟦️.ts")… })` — `logo` the emblem, `legal` the imprint and privacy links of `🚀️deploy/🔣️.json`, `pets` the menagerie loaded lazily (see [Pets](#pets)) |
-| `📚️catalog/🟦️.ts` | the material the site hands the quiz: the catalog and the quizzes it names, imported statically in catalog order, so the page shows the catalog at once and goes on working [while the proctor is away](#while-the-proctor-is-away) |
-| `🎨️.css` | the brand layer: the semio palette compiled to custom properties, its fonts shipped with the build |
-| `🏗️builder/🌐️vite/🟦️.ts` | Vite configuration: host HTML, `CNAME`, aliases; `build` bakes `VITE_PROCTOR_URL`, seals the document with its Content-Security-Policy and emits `robots.txt` and `manifest.webmanifest`; `serve` proxies the gateway routes to the dev proctor |
-| `📦️packages/🟦️typescript` | `@teaching/architecture-quiz`: `package.json`, `📋️project.json`, `📜️script.ts` |
-| `🧪️tests/🧪️catalog/🟦️.ts` | the catalog and every quiz validate in the TS core and against the draft-07 contract (ajv); the material module ships exactly these documents; every badge for perfection asks for medium or harder |
-| `🧪️tests/🧪️deploy/🟦️.ts` | `🚀️deploy/🔣️.json` holds to its schema (ajv); no deployment file drifts from it and seeded drifts are found; the Dockerfile, `compose.yaml` and the workflow (parsed with `yaml`), the Caddyfile, the CDN artifact check and the bundle for the proctor host (its `.env` and steps) hold what the runbook promises |
-| `🧪️tests/🧱️local-stack/🟦️.ts` | waiting for a server, stopping an owned command, and the static origin of the rehearsal (against Vite's preview server) |
-| `🧪️tests/🐾️pet-cast/🟦️.ts` | the pets fit the quizzes: every species and the ensemble of `../🐾️pets` validate (ajv and the pets product), grounds exist in the quiz files, every quiz and the home screen have a cast; every state is reachable and leads back, tricks and gear have their clips, the chemistry names what exists and looks and rests within sane bounds (see [Pets](#pets)) |
-| `🧱️stack/🟦️.ts` | the local stack: `dev` (proctor and site in one terminal) and the parts the end-to-end gate boots its stacks from |
-| `🎭️e2e/` | the end-to-end gate (`🟦️.ts`), its Playwright projects (`🎚️config`) and the learner the specs drive (`🚶️learner`) |
-| `🚀️deploy/` | `🔣️.json` (hosts, image, port) and its contract (`🧬️schema/🔣️.json`), `Dockerfile` with its build context (`Dockerfile.dockerignore`), `compose.yaml`, `Caddyfile` and the operator verbs (`🟦️.ts`): `publish`, `docker-image-build`, `docker-image-check`, `docker-stack-check`, `docker-stack-bundle`, `docker-image-publish`, `deploy-check` |
-
-## Challenges
-
-A learner plays every run at one of four challenges, chosen on the quiz's page (the device remembers the choice; `medium`
-at first). The catalog's introduction explains them to learners; the rules live in the quiz core (`CHALLENGE_RULES`), so
-the proctor and the device decide alike, and the quiz files say nothing about them, so their revisions stay.
-
-| Challenge | The keys | While playing | Points per quiz at most |
-|---|---|---|---|
-| easy | shown: the ladder of values a sorting is placed on, the cards of a matching, category descriptions and diagram numbers | beside a value placed farther off than its set's reach — on a logarithmic scale a factor of 1000, or half the orders of magnitude the values span where that is less; on a linear scale half their range — a question doubts the relation the learner's own keys claim between it and another item (preferably one placed right and familiar, the one whose relation is the most wrong), naming items by their short labels: a claim the wrong way round as "Are you sure “Burning tea light” is higher in power than “Kettle”?", amounts that add up as "Are you sure 10 × “Person sitting still” together only add up to the power of 1 × “Kettle”?" (the keys claim too little) or "Are you sure it takes 1,000 × “Burning tea light” to add up to the power of 1 × “Kettle”?" (too much), other quantities as "…only / really 1,000 times as high in U-value as…", linear ones as "…only / really 5 °C higher in … than…", always naming the quantity (German "in puncto U-Wert"), huge counts in words or as "1.1 × 10¹⁶"; beside a misplaced item of a classification: whether it is higher or lower than a standard placed right on one axis of the spider profile it was put into, or fits that profile with the axis at its value, else whether it and another item belong to the same or to different categories, else whether it belongs to the category it was put into, with that category's description. At most three hints per task, the most wrong first. A hint never states the truth and never says only "too high" or "how many" | 100 |
-| medium | shown | nothing | 200 |
-| hard | hidden: a typed estimate per value to sort or match; descriptions and diagram numbers gone | an estimate beyond the reach scores as a miss | 300 |
-| expert | hidden, as hard | every task runs against its own clock from *Start the clock*; the run may be submitted with tasks unanswered, which score as misses | 400 |
-
-A run earns its score times the points of its challenge; per quiz the run with the most points counts, and the
-leaderboard sums those. Every badge of this catalog but `completionist` asks for a perfect result at medium or harder
-(`"challenge": "medium"` on its rule), so the hints of easy earn none.
-
-## Develop
-
-Run everything from the dashboard (`bun run dashboard`, then `Ctrl+B n` and the words of the command id) or, without
-the TUI, with `bun run dashboard run <command id> [--detach --wait-ready]` (`semio …` is shorthand for `bun run dashboard …`),
-which prints the ready URL of a server:
-
-| Dashboard command id | What it does | Port |
-|---|---|---|
-| `@teaching/architecture-quiz:dev` | backend and frontend together; parameters `steady=true` (a flag: sets `TEACHING_ARCHITECTURE_QUIZ_WATCH=off`, so neither the site nor the proctor follows source edits) and `stack=beside` (a second, independent stack beside the first) | `8791` and `6061`; `beside`: `8793` and `6063` |
-| `@teaching/architecture-quiz:dev-site` | the site alone | `6061` (`TEACHING_ARCHITECTURE_QUIZ_PORT`) |
-| `@teaching/proctor:dev` | the proctor alone | `8791` (`PROCTOR_PORT`) |
-| `@teaching/architecture-quiz:test` | the tests | |
-| `@teaching/architecture-quiz:typecheck` | the entry, the Vite configuration, the stack, the deploy verbs, the end-to-end gate and every test and spec against the compiler; it first regenerates the git-ignored bindings its sources read | |
-| `@teaching/architecture-quiz:test-e2e` | the end-to-end gate | `6161`/`8891`, `6162`/`8892` |
-| `@teaching/architecture-quiz:check` | `proctor check` on the catalog | |
-| `@teaching/proctor:rebuild` | refold the dev proctor's read models | |
-
-```bash
-bun run dashboard run @teaching/architecture-quiz:dev --detach --wait-ready                      # the stack on 6061
-bun run dashboard run @teaching/architecture-quiz:dev --param steady=true --param stack=beside --detach --wait-ready  # a steady second stack on 6063
-```
-
-The beside stack keeps its proctor data in `.🧬semio/🎓️teaching/proctor-beside`, so the two stacks never share a store.
-
-### One command for backend and frontend
-
-`dev` (`🧱️stack/🟦️.ts`) runs the whole stack in one terminal:
-
-1. A proctor that already answers `GET /instance` on `PROCTOR_PORT` is reused, and never stopped by this command; when
-   it serves another quiz contract than the site (an older build, say), the command says so in one `[WARN]` line — the
-   site then sends it nothing and keeps everything on the device — and stopping it and running `dev` again builds the
-   current one. Otherwise the proctor is built (a first build takes minutes; a progress line follows every ten seconds) and launched
-   with the defaults of `@teaching/proctor:dev`: the same data directory `.🧬semio/🎓️teaching/proctor-dev/` and this catalog.
-   That directory is disposable development data. When it holds a database of a storage format this proctor does not read
-   (the proctor refuses such a file, and there is no migration), `dev` and `@teaching/proctor:dev` move it aside to
-   `.🧬semio/🎓️teaching/proctor-dev.v<format>-<time>/`, start with empty data and say so in one line; delete what was moved
-   aside when you do not need it. A directory you named yourself with `PROCTOR_DATA` is never touched: the command ends
-   at once with one line that names it and says how to go on. To reset development data at any time, stop the proctor and
-   delete `.🧬semio/🎓️teaching/proctor-dev/`.
-2. The command waits until the proctor answers ready. If the proctor exits first, the command fails and says so.
-3. The site's dev server starts (`dev-site`). A site that already answers on its port is reused the same way.
-4. The proctor this command launched is supervised, each step said in one `[stack]` line. Whenever a Rust source of a
-   crate it is built from changes (cargo's own resolution of the teaching workspace names those crates), it is built
-   anew while the running one serves and then launched in its place — a build that fails leaves the running one
-   serving; whenever the catalog or one of its quizzes changes, it is launched anew; whenever it ends by itself, it is
-   launched again after one second, then ever more patiently, at most every half minute. Every such swap is a short
-   absence of the proctor, which the site tolerates like any other. `TEACHING_ARCHITECTURE_QUIZ_WATCH=off` stops the
-   following (and the site's reloads).
-5. One Ctrl+C, a termination or a closed terminal stops both, with every process they started. If the site exits, the
-   proctor is stopped as well.
-
-The dev site bakes no proctor origin and proxies `/instance`, `/commands`, `/queries`, `/actors` (WebSocket event streams
-included) and `/scopes` to `http://127.0.0.1:${PROCTOR_PORT:-8791}`, so the browser talks to one origin in dev. A proctor
-that goes away while a socket is proxied never takes the dev server along, also under Bun 1.3, whose sockets lack the
-`destroySoon` the proxy ends them with (`semioServeUpgradeVitePlugin`).
-
-### End-to-end gate
-
-`test-e2e` (`🎭️e2e/🟦️.ts`) boots throw-away stacks and drives the real site in Chromium with Playwright. It never touches
-the dev ports or the dev data.
-
-| Topology | Site | Proctor |
-|---|---|---|
-| `dev` | the dev server on `6161`, one origin through the proxy | development mode on `8891` |
-| `rehearsal` | the release build with `http://127.0.0.1:8892` baked in, served as static files on `6162` the way a CDN serves them | production mode on `8892`, granting exactly `http://127.0.0.1:6162` |
-
-Both run by default, side by side. Name one to run only that one, and pass anything else on to `playwright test`:
-
-```sh
-bun repo run @teaching/architecture-quiz:test-e2e                                # both topologies
-bun repo run @teaching/architecture-quiz:test-e2e -- rehearsal                   # the release rehearsal only
-bun repo run @teaching/architecture-quiz:test-e2e -- dev --grep "leaderboard"    # one spec against the dev stack
-bun repo run @teaching/architecture-quiz:test-e2e -- --serial --keep             # one after the other; keep the run directory
-bun repo run @teaching/architecture-quiz:test-e2e -- --proctor <executable>      # run that proctor instead of building one
-```
-
-The release document's Content-Security-Policy is part of the rehearsal: its `connect-src` names the baked proctor origin,
-and any violation fails the spec that caused it. The dev server of the `dev` topology has its own dependency cache and does
-not watch the sources, so an edit during a run never reloads a page under test.
-
-The specs live in `🧪️tests/` beside the node tests, one directory each, and share the learner of
-`🎭️e2e/🚶️learner/🟦️.ts`. Every answer is computed from the quiz sources by item id, and a spec fails on any console
-error, page error, failed request or policy violation it did not announce.
-
-A page of the quiz keeps a processor core busy while a test drives it. One topology therefore runs with four Playwright
-workers and two topologies at once with two each; `--workers <n>` overrides both. On a 16-thread machine both topologies
-take seven minutes together, eleven beside other builds. A step that does not see what it expects fails after twenty
-seconds; a whole test may take ten minutes.
-
-| Spec | What a learner does |
-|---|---|
-| `🚀️site-boot` | the site loads and shows the introduction (runs first, so a cold dev server is warm for the others) |
-| `🪪️first-visit` | introduction, then anonymous, pseudonym or name; a known pseudonym recalls its progress on a fresh device; switching identity asks first |
-| `🥞️layered-home` | the nine cards around the leaderboard, the live screen-sized pages behind the glass that the mouse pans between the cards and that come to the screen, clear, on a card (on a device that reports reduced motion too), hash and Escape, and the navbar's ways — the overview, back and forward along the trail, up from a run to its quiz — around what the quizzes are about in its middle |
-| `🎯️quiz-runs` | every quiz to a perfect score and every badge; one mistake per task for partial credit (the classification mistake is dragged with the mouse) |
-| `⛰️challenge-levels` | physics at every challenge: easy with the keys on the ladder and, beside each value placed far off until it is fixed, a question asserted by its structure with items named by their short labels (extremes exchanged: the order question without a number, the larger first, against an item placed right; the two largest exchanged: "together only add up to" and "it takes", no long digit runs), exactly three on a sorting placed upside down, nothing general anywhere (100 points, no badge); easy on a matching (both exchanged cards asked for their order, on two quantities the one named, in German "in puncto Heizlast") and on spider profiles (above or below a standard placed right on the named axis, else the profile's value), in English and German; medium (200 points and the physics badges), hard with typed guesses (300 points; one guess far off costs score and shows as a miss), expert behind a clock per task driven past its deadline with Playwright's page clock (read-only, open tasks score nothing, points of 400) |
-| `🏆️live-leaderboard` | rows ordered by total, arriving on a watching device without a reload; the same learners on today's, this week's and this month's leaderboard and on the one of the quiz they played; a column heading clicked to sort |
-| `🗣️both-languages` | every screen, the document's language and title and the notice of what is stored in English and German, switched both ways; the language follows the browser and is asked for when the browser names neither |
-| `🐕️pet-walk` | the pets: on the home screen within seconds, hidden from assistive technology and never a hit target of their own; a click says hello, then asks for a trick, then for a purr; picked up, a pet hangs and, let go high, opens its parachute and lands; circling a pet changes its state; no two pets overlap during a lively minute with drags; a control under a pet keeps its click; the settings' play group and the footer switch work by keyboard; the cast of a quiz on its page and in its run; `still` and `off`; reduced motion as the default only (still pets that answer no hand until the learner chooses, calm pets that walk afterwards) |
-| `📱️phone` | 375 × 812: home as the same grid swiped page by page along both axes, one quiz played |
-| `👥️shared-presence` | two devices: online count, cursors, what the other thinks, what everyone answered (runs alone) |
-| `🔌️connection-shortage` | a reload mid-run; the proctor stopped and started again mid-run (runs alone) |
-| `📴️proctor-away` | the proctor stopped before a learner ever arrives: a pseudonym taken, a quiz played to its score, feedback and badges, the result kept over a reload, all on the device; the proctor started again: the connection calms, a fresh device finds the same learner with the same result; a pseudonym the proctor already knew continues as its holder (runs alone) |
-| `🛟️proctor-faults` | the proctor dies between every two tasks of an expert run and in the moment the run is submitted: full points, no notice, a fresh device finds the run; a proctor of an older contract (its `GET /instance` answered one version lower in the browser) is sent no command and no query while the device decides a whole run, one of a newer contract shows the page out of date with a reload, and once they agree everything arrives (runs alone) |
-
-Each run keeps its stacks under the git-ignored `.🧬semio/🎓️teaching/architecture-quiz-e2e/<run>/` and deletes that
-directory when it passes. A failed run keeps it — the proctor and site logs per topology, and `report/` with Playwright's
-traces and screenshots — and prints its path. Chromium is taken from `PLAYWRIGHT_BROWSERS_PATH`, else from the repo's
-tool cache, and is downloaded once if it is missing.
-
-### Pets
-
-The site hands the quiz its pets: `🟦️.ts` passes `pets: () => import("../🐾️pets/🟦️.ts")…` to `mountQuiz`, and the quiz
-fetches that menagerie together with the render target (`@semio-tech/pets-react`) only for a learner whose preference is
-not `off`. The menagerie is the architecture one ([`../🐾️pets`](../🐾️pets/README.md)): twenty species, each the
-likeness of a thing the quiz items are about, their bonds, and one cast for the home screen and one per quiz, so the pets
-on screen fit the topic. They stand on the top edges of the cards and on the floor of the window, keep clear of text and
-controls, rest during a run and never take a click; the preferences offer `off`, `still`, `calm` (the default) and
-`lively`, and a device that asks for reduced motion gets them motionless until its learner chooses — a choice made in
-the preferences or with the switch on every screen holds whatever the device asks for.
-
-| What | Where |
-|---|---|
-| species, ensemble (bonds, casts), roster and how to add a pet | `../🐾️pets/` and its README |
-| the quiz's glue (preference, scene, lazy loading, mounting) | `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🐾️pets` |
-| the model and the layer | `🧰️framework/🛍️products/🐾️pets` (`@semio-tech/pets`, `@semio-tech/pets-react`) |
-| judging rigs and casts by eye | dashboard command `@semio-tech/pets-react:dev` (`bun run dashboard run @semio-tech/pets-react:dev --param stories-port=6074 --param menagerie=🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts --detach --wait-ready`), http://127.0.0.1:6074/ |
-
-In a release build the pets are four lazy script chunks and one lazy stylesheet beside the entry (the menagerie, the
-render target with the model, their shared constants, the half of the quiz's glue that comes with the pets — the layer
-as the quiz sets it up and "Play with the pets" —, the paints), all content-hashed under `assets/` and loaded from
-the site's own origin, so the document's Content-Security-Policy needs nothing new and the entry script budget is not
-charged with them. `🧪️tests/🐾️pet-cast` holds the menagerie to the quiz files; `🧪️tests/🐕️pet-walk` drives the pets in
-both topologies of the end-to-end gate.
-
-## While the proctor is away
-
-The site and the proctor are two deployments, and the site does not need the proctor to be there. It carries its material
-(`📚️catalog/🟦️.ts`: the catalog and its quizzes, solutions included), so the quiz client has the proctor's own deciders
-on the device (the deputy, see the quiz product's README, *While the proctor is away*):
-
-| The proctor | What a learner gets |
-|---|---|
-| answers | everything is decided by the proctor, as before; the header says *All answers saved* |
-| does not answer (not deployed yet, down, behind a firewall the learner is outside of, no network) | the catalog shows at once; a learner takes an identity, starts quizzes, answers, submits, sees the score, the feedback and the badges — decided and saved in the browser's storage. The header says *Quiz server not reachable – everything is saved on this device* (with the number of commands still to send), the leaderboard shows this device's own standing and says so, the others online and what they answered are not there |
-| answers again | everything waiting is sent in the order it happened, each command once; the proctor decides it again and its views replace the device's (its clock, its score). A pseudonym or name the proctor already knew continues as its holder, and the learner is told |
-| serves another quiz contract (an older deployment, a newer one the page predates) | it is sent nothing and treated as away, as above; a newer proctor shows *This page is older than the quiz server* with *Reload page*. Once the two agree — the proctor or the page was replaced — everything waiting arrives |
-| dies again and again, also in the middle of a submission | nothing is lost and nothing is told: whatever it did not decide the device decides, and the proctor decides it again once it is back |
-
-A learner waits at most three seconds for a proctor that says nothing before the device decides. What is saved on the
-device stays there until the proctor has it: a learner who clears the browser's data before that loses it. The proctor
-scores every run itself, so the leaderboard never trusts a device.
-
-The gate proves it in a real browser in both topologies (`🧪️tests/📴️proctor-away`, `🧪️tests/🛟️proctor-faults`); the
-unit proofs are in the quiz product (`🚶️learner-journey`, `🫡️deputy-decisions`, `📬️outbox-delivery`,
-`🤝️wire-version`).
-
-## Proof of concept by hand
-
-The split deployment without GitHub and without a registry: two directories, each carried to its place by hand. Stage
-them in this order — a build of the site empties `dist`, the bundle included:
-
-```sh
-bun repo run @teaching/architecture-quiz:publish                # 1. the site           → dist/pages/quizzes
-bun repo run @teaching/architecture-quiz:docker-image-build     # 2. the proctor image (a first build takes minutes)
-bun repo run @teaching/architecture-quiz:docker-stack-bundle    #    the stack with it  → dist/proctor
-```
-
-Dashboard command ids: `@teaching/architecture-quiz:publish`, `@teaching/architecture-quiz:docker-image-build`,
-`@teaching/architecture-quiz:docker-stack-bundle`.
-
-**1. The site.** Upload the content of `📦️packages/🟦️typescript/dist/pages/quizzes` to the static host or CDN that serves
-`https://quizze.architektur-und-technologie.de` ⚠️. Any static host works: the app addresses its pages by hash, so no
-rewrite rule is needed; `_headers`, `CNAME` and `.nojekyll` are honoured where the host knows them and ignored elsewhere.
-The build talks to `https://semio.iek.uni-hannover.de` and to nothing else (its Content-Security-Policy says so), and
-the proctor grants exactly the origin `https://quizze.architektur-und-technologie.de`: served under another name, the
-site needs that origin added to `PROCTOR_ALLOWED_ORIGINS` in the `.env` of the stack.
-
-**2. The proctor.** `docker-stack-bundle` stages everything `semio.iek.uni-hannover.de` needs in
-`📦️packages/🟦️typescript/dist/proctor` (`--out <directory>` stages elsewhere, `--tag <tag>` bundles another image tag):
-
-| File | What |
-|---|---|
-| `compose.yaml`, `Caddyfile` | the stack, copied out of the image (an image built from other stack files than this checkout's is refused) |
-| `.env` | pins `PROCTOR_TAG` to the bundled image and lists the other overrides (`PROCTOR_HOST`, `PROCTOR_ALLOWED_ORIGINS`, `QUIZ_HTTP_PORT`, `QUIZ_HTTPS_PORT`) as comments |
-| `certificates/` | empty; the place for a certificate of your own |
-| `proctor-image.tar` | `docker save` of `ghcr.io/usalu/architecture-quiz-proctor:<tag>` |
-| `README.txt` | the steps below |
-
-Copy the directory to the machine ⚠️ (`scp -r dist/proctor <user>@semio.iek.uni-hannover.de:architecture-quiz`), then
-there, in `~/architecture-quiz`, on `linux/amd64` with Docker Engine 25 or newer and its compose plugin 2.24 or newer:
-
-1. `docker load --input proctor-image.tar`
-2. The certificate, one of:
-   - Let's Encrypt ⚠️, nothing to do: Caddy obtains and renews it once ports 80/tcp, 443/tcp and 443/udp of the machine are
-     open from the internet.
-   - Your own: one file `certificates/<any name>.pem` with the certificate and its full chain first and the private key
-     after it, owned by root with mode `0600` (Caddy runs as root without any capability and does not start on a file
-     it cannot read). Caddy serves it and orders none. After replacing it:
-     `docker compose up --detach --force-recreate caddy`.
-3. `docker compose up --detach --wait`
-4. `curl --fail https://semio.iek.uni-hannover.de/instance` ⚠️ answers JSON that starts `{"id":"teaching-proctor"`.
-
-`semio.iek.uni-hannover.de` resolves to 130.75.188.198, and on 2026-10-02 its ports 80 and 443 did not answer from
-outside the university. Until the university firewall lets them in, Let's Encrypt cannot issue (supply a certificate
-instead) and a learner outside the campus network does not reach the proctor at all, whatever the certificate.
-
-The bundle was run on 2026-10-02 as the machine runs it (`docker load`, `docker compose up --detach --wait`, with
-`PROCTOR_HOST=localhost` and Caddy's internal CA): `/instance` answered through Caddy, a preflight from
-`https://quizze.architektur-und-technologie.de` was granted and one from a foreign origin was not. `docker-stack-check`
-proves the supplied certificate: it mints one for `localhost` with another CA, puts it into `certificates/` and sees
-Caddy serve exactly that one. Everything under [Deploy](#deploy) from *Update* on applies to this stack as it is; for an
-update, stage a new bundle, take a backup, then repeat steps 1 and 3 in the same directory (your `.env` and
-`certificates/` stay when you copy only the new `proctor-image.tar`, `compose.yaml` and `Caddyfile`).
-
-## Deploy
-
-Two artifacts, released independently by one manual workflow (`.github/workflows/architecture-quiz.yml`) — or carried
-by hand, see [Proof of concept by hand](#proof-of-concept-by-hand):
-
-| Artifact | Where it runs | Built and proven by |
-|---|---|---|
-| the site: static files with `index.html`, `404.html` (the same document, for deep links), hashed `assets/`, `robots.txt`, `manifest.webmanifest`, `CNAME`, `.nojekyll`, `_headers` | a static CDN at `https://quizze.architektur-und-technologie.de` (GitHub Pages) | `🚚️publish🎓️teaching🏛️architecture❓️quiz` (`bun repo run @teaching/architecture-quiz:publish`) |
-| the proctor image `ghcr.io/usalu/architecture-quiz-proctor` as `sha-<commit>`, `<workspace version>` and `latest` | the university's Docker host at `https://semio.iek.uni-hannover.de`, behind Caddy | `📦️build…🐳️docker-image`, `⚖️gate…🐳️docker-image`, `⚖️gate…🐳️docker-stack`, `📦️bundle…🐳️docker-stack`, `🚚️publish…🐳️docker-image` |
-
-`🚀️deploy/🔣️.json` is the one authored source of both hosts, the image repository and the port (contract:
-`🚀️deploy/🧬️schema/🔣️.json`). The Dockerfile, `compose.yaml`, the `Caddyfile`, the workflow and this file repeat those
-values because Docker, Caddy and GitHub read them there; `🧪️tests/🧪️deploy` and the readiness gate fail on any drift.
-
-### Readiness gate
-
-`⚖️gate🎓️teaching🏛️architecture❓️quiz🚀️deploy` (`bun repo run @teaching/architecture-quiz:deploy-check`) says whether the
-checkout is ready to deploy. It needs a running Docker daemon, pushes and uploads nothing, announces every step with its
-number, and stops at the first failure or at Ctrl+C (whatever a step created is removed):
-
-1. nothing drifted from `🚀️deploy/🔣️.json` (hosts, image, port, pinned base images and actions, targets named in docs);
-2. the sources pass the compiler (`typecheck`): the entry, the Vite configuration, the stack, the deploy verbs, the
-   end-to-end gate and every test and spec;
-3. the catalog and every quiz are valid in the Rust core (`proctor check`);
-4. the site builds for the production proctor and is the CDN artifact: entry points, a document sealed by its
-   Content-Security-Policy, hashed assets, no source map or development leftover, within the size budget
-   (`QUIZ_SITE_BUDGET`: what the document links — the entry script 260 000 B and the stylesheet 60 000 B gzip — and
-   4 MB on disk; a run with its results and the pets are lazy chunks the first screen does not wait for, so they are
-   not charged: on 2026-10-03 the entry weighed 253 374 B, the run chunk 13 867 B, the results chunk 3 217 B and the
-   quiz's half of the pets' glue 1 499 B);
-5. the proctor image builds;
-6. the image is what it claims (labels, unprivileged account, no shell, the stack files inside) and serves the API as the
-   site meets it, read-only and without capabilities;
-7. the stack works exactly as a host gets it: the two stack files copied out of the image, TLS through Caddy, the
-   hardening Docker applied, a backup while serving, an update without a failed request, a restore, a supplied
-   certificate served instead of Caddy's own;
-8. the end-to-end gate (`test-e2e`): the release build in a real browser against a production-mode proctor.
-
-It ends with `ready to deploy` and warns while `site.legal` in `🚀️deploy/🔣️.json` names no imprint or privacy notice.
-
-### What only the owner can do, in order
-
-Every command below was run on 2026-10-02 against Docker Desktop (Linux engine, amd64) with `PROCTOR_HOST=localhost`,
-except the lines marked ⚠️, whose outward part (DNS, GitHub, the registry, a public certificate) cannot be rehearsed.
-
-**0. The legal pages.** Put the two absolute `https://` URLs into `site.legal.imprint` and `site.legal.privacy` of
-`🚀️deploy/🔣️.json` and commit; the footer links only what is set, the site must be built again afterwards (step 2.3),
-and the readiness gate warns until both are there.
-
-**1. DNS and the firewall.** Only the site name is in the owner's zone (at the provider of
-`architektur-und-technologie.de`):
-
-```
-quizze.architektur-und-technologie.de.                                CNAME  usalu.github.io.
-_github-pages-challenge-usalu.quizze.architektur-und-technologie.de.  TXT    <value shown by GitHub in step 2>
-```
-
-If that zone has `CAA` records, they must allow `letsencrypt.org` (GitHub Pages obtains its certificate there).
-
-The proctor's name is the university's and exists: `semio.iek.uni-hannover.de` resolves to 130.75.188.198. What is
-missing is the way in ⚠️: on 2026-10-02 ports 80 and 443 of that address did not answer from outside the university.
-Ask the university's network operators to let 80/tcp, 443/tcp and 443/udp through to the machine. Without that, learners
-outside the campus network do not reach the proctor, and Let's Encrypt cannot issue a certificate — then supply one
-(step 3). A `CAA` record of `uni-hannover.de` that does not allow `letsencrypt.org` has the same effect.
-
-**2. GitHub, once** ⚠️:
-
-1. Account → Settings → Pages → *Add a domain* → `quizze.architektur-und-technologie.de`; create the `TXT` record it
-   shows and press *Verify*. A verified domain cannot be claimed by another account's Pages site, and that host is the one
-   origin the proctor grants.
-2. Repository → Settings → Pages → *Source*: **GitHub Actions**; *Custom domain*: `quizze.architektur-und-technologie.de`;
-   after the certificate is issued tick *Enforce HTTPS*. With this source the custom domain is this setting: the
-   artifact's `CNAME` file is ignored, and dot files are not uploaded.
-3. Repository → Actions → *architecture quiz* → *Run workflow* on branch `main` (both boxes ticked). The run is skipped on
-   any other branch. The `site` job tests, builds, verifies and deploys the site; the `proctor` job builds the image,
-   checks the image and the stack and only then pushes `sha-<commit>`, `<version>` and `latest`, printing the digest.
-4. After the first push: profile → Packages → `architecture-quiz-proctor` → *Package settings* → *Change visibility* →
-   **Public**, so the host pulls without credentials. (To keep it private instead, run `docker login ghcr.io` on the host
-   once with a token that has `read:packages`.)
-
-**3. The Docker host, once.** `semio.iek.uni-hannover.de`: a `linux/amd64` machine with Docker Engine 25 or newer and its
-compose plugin (2.24 or newer), reachable on ports 80/tcp, 443/tcp and 443/udp. The whole footprint is the directory
-`~/architecture-quiz` with two files, and both come out of the image:
-
-```sh
-mkdir -p ~/architecture-quiz && cd ~/architecture-quiz
-docker pull ghcr.io/usalu/architecture-quiz-proctor:latest   # ⚠️ needs the published image
-container=$(docker create ghcr.io/usalu/architecture-quiz-proctor:latest) && docker cp "$container:/srv/quiz/deploy/." . && docker rm --volumes "$container"
-docker compose up --detach --wait
-```
-
-(Without the registry the same directory comes from `docker-stack-bundle`, image included: see
-[Proof of concept by hand](#proof-of-concept-by-hand).)
-
-`docker compose up` starts the proctor (unprivileged, read-only root, no capabilities, reachable by Caddy only) and, once
-it is healthy, Caddy, which obtains and renews the certificate for the proctor host by itself ⚠️. Nothing is configured:
-the image carries the production configuration. Overrides go into an `.env` file beside `compose.yaml`: `PROCTOR_HOST`,
-`PROCTOR_TAG`, `PROCTOR_ALLOWED_ORIGINS`, `QUIZ_HTTP_PORT`, `QUIZ_HTTPS_PORT` (these five also from the environment), and
-any other variable of the proctor — both services read that file as their environment.
-
-**A certificate of your own.** Where Let's Encrypt cannot reach the machine, put the certificate into the directory
-`certificates` beside `compose.yaml` (Docker creates it at the first `up`) before, or any time after, the first start:
-
-```sh
-cat fullchain.pem privkey.pem | sudo tee certificates/semio.pem > /dev/null   # the chain first, the private key after it
-sudo chown root:root certificates/semio.pem && sudo chmod 600 certificates/semio.pem
-docker compose up --detach --force-recreate caddy
-```
-
-The file name is free as long as it ends in `.pem`. Caddy loads every such file at its start; with a certificate that
-names the proctor host it serves that one and orders none (its log says `skipping automatic certificate management
-because one or more matching certificates are already loaded`); for a host without one it goes on obtaining its own.
-Caddy runs as root without any capability, so it reads a file only as its owner or as everyone: a bundle it cannot read
-(owned by another account with mode `0600`) stops Caddy at its start with `permission denied`, and `docker compose logs
-caddy` names the file. Renewal is yours: replace the file before the certificate expires and recreate `caddy` again.
-Rehearsed with the pinned Caddy image: a supplied bundle served for `localhost` (`docker-stack-check`) and for the
-proctor host's own name, no order made; a file owned by root with `0600` read, one owned by another account refused with
-`0600` and read with `0644` (in a Docker volume ⚠️ — Docker Desktop does not carry a host file's owner into a bind mount,
-so the first two lines above are not rehearsed as written).
-
-**4. Verify** ⚠️ (needs DNS, the open firewall and both deployments; run the `curl` lines from outside the university):
-
-| Check | Expected |
-|---|---|
-| `curl --fail --silent https://semio.iek.uni-hannover.de/instance` | `200`, JSON starting `{"id":"teaching-proctor"` |
-| `curl --silent --output /dev/null --write-out '%{http_code} %{redirect_url}\n' http://semio.iek.uni-hannover.de/instance` | `308 https://semio.iek.uni-hannover.de/instance` |
-| `curl --silent --include --request OPTIONS --header 'Origin: https://quizze.architektur-und-technologie.de' --header 'Access-Control-Request-Method: POST' https://semio.iek.uni-hannover.de/queries` | `204` with `access-control-allow-origin: https://quizze.architektur-und-technologie.de` |
-| `curl --silent --verbose --output /dev/null https://semio.iek.uni-hannover.de/instance 2>&1 \| grep -E 'issuer\|expire'` | the issuer you expect (Let's Encrypt, or the authority of the certificate you supplied) and an expiry date in the future |
-| `docker compose ps` on the host | `proctor` and `caddy` both `running (healthy)` |
-| `https://quizze.architektur-und-technologie.de` in a browser | the introduction, then a pseudonym, a quiz run and the leaderboard; the header shows *Connected*, the console is free of errors |
-| `curl --head https://quizze.architektur-und-technologie.de/does-not-exist` | `404` with the site's document (GitHub Pages answers deep links with `404.html`) |
-
-### Update
-
-On the host, in `~/architecture-quiz`, after a workflow run pushed a new image:
-
-```sh
-docker compose exec -T proctor proctor backup - > "proctor-before-update-$(date +%Y-%m-%dT%H%M%S).sqlite"
-docker compose images proctor
-docker compose pull proctor   # ⚠️ needs the registry
-container=$(docker create ghcr.io/usalu/architecture-quiz-proctor:latest) && docker cp "$container:/srv/quiz/deploy/." . && docker rm --volumes "$container"
-docker compose up --detach --wait
-```
-
-The backup comes first: it is the only way back across a release that changed what the events mean. `docker compose images`
-shows the image that was running, the third and fourth lines fetch the new image and its stack files, and `up` replaces
-only what changed. The volumes stay. Without the registry, copy `proctor-image.tar`, `compose.yaml` and the `Caddyfile`
-of a new bundle (`docker-stack-bundle`) into the directory and run `docker load --input proctor-image.tar` in place of
-those two lines: `up` replaces the proctor because its tag now names another image. While the proctor container is replaced, Caddy holds requests for up to 20 seconds
-instead of failing them, and the site keeps unsent answers in its outbox. At boot the proctor validates the catalog,
-refolds its read models when the catalog or the projector changed (the health check waits up to five minutes for that)
-and then listens. A quiz whose file changed gets a new revision: an open run on the old revision can no longer record
-answers (`quiz-revised`) and is voided when it is submitted or started again; submitted results, badges and the
-leaderboard stay. Run the workflow's `site` job for a new client; the two parts are independent.
-
-`docker compose up --detach` alone never updates anything (`pull_policy: missing`); it only starts what is stopped — which
-is what to run after a host reboot found the stack down.
-
-### Rollback
-
-Every release is also pushed as `sha-<first 12 digits of the commit>`, which is never moved:
-
-```sh
-PROCTOR_TAG=sha-0123456789ab docker compose up --detach --wait
-```
-
-(or `PROCTOR_TAG=sha-…` in `.env`, so that the choice survives the next `up`). If the newer release wrote events the older
-one does not know, restore the backup taken before the update as well. A proctor that meets a database of another storage
-format refuses to start and names both formats; the data is untouched.
-
-### Backup and restore
-
-The whole state is `proctor.sqlite` (with its write-ahead log beside it while the proctor runs) on the volume
-`architecture-quiz_proctor-data`. Events are the truth; every read model in the file can be rebuilt from them.
-`caddy-data` holds the ACME account and the certificates Caddy obtained and can be re-issued; a certificate you supplied
-lives in the directory `certificates`, not in a volume — keep its source elsewhere too.
-
-```sh
-docker compose exec -T proctor proctor backup - > "proctor-$(date +%Y-%m-%dT%H%M%S).sqlite"
-```
-
-is safe while the proctor serves: one read transaction writes a whole, consistent database without blocking a learner.
-Keep copies off the host. From another machine one line takes the backup and brings it home ⚠️ (needs the host):
-
-```sh
-ssh <user>@semio.iek.uni-hannover.de 'cd ~/architecture-quiz && docker compose exec -T proctor proctor backup -' > "proctor-$(date +%Y-%m-%dT%H%M%S).sqlite"
-```
-
-and a daily one on the host itself, kept for 30 days, is one `crontab -e` line:
-
-```
-17 3 * * * cd ~/architecture-quiz && mkdir -p backups && docker compose exec -T proctor proctor backup - > "backups/proctor-$(date +\%F).sqlite" && find backups -name 'proctor-*.sqlite' -mtime +30 -delete
-```
-
-A backup holds what the learners entered (pseudonyms, names); treat it like the database.
-
-Restore replaces the database of a stopped proctor. The proctor refuses a file that is not a whole proctor database,
-and refuses while a proctor serves the volume:
-
-```sh
-docker compose stop proctor
-docker compose run --rm -T --no-deps proctor restore - < proctor-2026-10-02T093000.sqlite
-docker compose up --detach --wait
-```
-
-The read models never need care: the proctor refolds them at boot whenever the catalog or its projector changed. To
-force it (after editing the database by hand, say), stop the proctor and run its `rebuild` once:
-
-```sh
-docker compose stop proctor
-docker compose run --rm --no-deps proctor rebuild
-docker compose up --detach --wait
-```
-
-### Erasing a learner on request
-
-The proctor removes one learner — the events, the handles they hold, their scores and every read model that names them
-— from the database of a stopped proctor. `--handle "<pseudonym or name>"`, `--tag <the 8 digits on the leaderboard>`
-or `--learner <id>` selects; the dry run prints what would go and changes nothing:
-
-```sh
-docker compose stop proctor
-docker compose run --rm --no-deps proctor erase --handle "Ada Lovelace" --dry-run
-docker compose run --rm --no-deps proctor erase --handle "Ada Lovelace"
-docker compose up --detach --wait
-```
-
-Backups taken before still hold that learner: delete them, or take them again.
-
-### Pruning registrations nobody played under
-
-Signing up needs no password, so registrations collect that nobody ever plays under — visitors who looked and left, or
-a script. They cost disk and count against the proctor's cap of registrations (`PROCTOR_MAX_LEARNERS`, 100000); once
-that cap is reached, new learners are refused until it is raised or pruned. The proctor removes, from the database of
-a stopped proctor, every learner that never submitted a run and registered longer ago than `--older-than` (a whole
-number and `m`, `h`, `d` or `w`: `36h`, `7d`, `2w`) together with the handles it holds, and nobody who played. The dry
-run prints the counts — never a name — and changes nothing; a real run reports its progress, and Ctrl+C stops it
-between two batches (run it again for the rest):
-
-```sh
-docker compose stop proctor
-docker compose run --rm --no-deps proctor prune --older-than 7d --dry-run
-docker compose run --rm --no-deps proctor prune --older-than 7d
-docker compose up --detach --wait
-```
-
-Once a month is plenty: one client address may register 1200 learners at once and 100 more per hour
-(`PROCTOR_LIMIT_SIGNUPS_BURST`, `PROCTOR_LIMIT_SIGNUPS_PER_HOUR`), so a single address needs 41 days to reach the cap,
-and 100000 registrations weigh 170 to 650 MiB on the volume.
-
-### Logs and limits
-
-`docker compose logs --since 1h proctor caddy` (add `--follow` to watch). Docker keeps at most 5 × 10 MB per service. The
-proctor logs its start, its stop and errors; Caddy logs certificate management and proxy errors. There is no access log:
-neither service records client addresses, so the stack stores no personal data besides what learners enter. To trace an
-abuse, add `log` to the site block of the `Caddyfile` and `docker compose up --detach` — and decide first how long those
-addresses may be kept. The proctor is limited to 2 CPUs, 1 GiB and 512 processes, Caddy to 1 CPU, 256 MiB and 256, so a
-runaway process cannot take the host down. Its own request limits (per client address and in total), the sign-ups it
-accepts per client address, its cap of registrations and the size of a request body are variables of the proctor
-(`🎓️teaching/🛂️proctor/README.md`); set one in `.env` and both services follow.
-
-Nothing watches the stack. Point an uptime monitor at `https://semio.iek.uni-hannover.de/instance` (expects `200`) —
-one outside the university, so that it sees what a learner at home sees — and at the site, and look at the host's disk
-now and then: the database only grows, except by an erasure or a pruning. With a certificate of your own, let the
-monitor warn before it expires: nothing renews it.
-
-### When the certificate cannot be issued
-
-Caddy retries by itself; `docker compose logs caddy | grep -i -E 'obtain|acme|challenge|error'` says why it fails.
-
-1. `dig +short semio.iek.uni-hannover.de` must print the host's address, 130.75.188.198 (and `AAAA` only an address the
-   host really answers on: a wrong `AAAA` record fails the challenge for IPv6-first validators).
-2. Ports 80 and 443 must reach Caddy from the internet: `curl --head http://semio.iek.uni-hannover.de/` from a network
-   outside the university must answer `308` from Caddy, not time out (the university firewall, the host firewall). This
-   is the expected obstacle for this host.
-3. A `CAA` record that does not allow `letsencrypt.org` forbids the issuance.
-4. After many failed attempts Let's Encrypt rate-limits the host name for up to an hour; fix the cause first, then wait.
-   Never delete the `caddy-data` volume to "retry": it holds the account and every issued certificate.
-
-When the cause cannot be removed, supply a certificate (step 3 of [What only the owner can do](#what-only-the-owner-can-do-in-order)):
-with it Caddy stops ordering.
-
-`PROCTOR_HOST=localhost QUIZ_HTTP_PORT=18080 QUIZ_HTTPS_PORT=18443 docker compose up --detach --wait` on any machine runs
-the same stack with Caddy's internal certificate authority, which is what `docker-stack-check` does.
-
-### Never
-
-`docker compose down --volumes` deletes the database and the certificates Caddy obtained. `docker compose down` (without
-the flag) only removes the containers and keeps both.
-
-### Staging, other CDNs, other architectures
-
-- The site bakes its proctor origin at build time: `PROCTOR_URL`, else the production proctor. A staging site on another
-  origin needs both: `PROCTOR_URL` when building it (`publish` then flags the artifact as a rehearsal), and its origin
-  added, comma-separated, to `PROCTOR_ALLOWED_ORIGINS` on the proctor host.
-- GitHub Pages cannot send response headers and caches every file for ten minutes. The document therefore carries its
-  Content-Security-Policy as a `meta` tag (scripts only from the site and by hash, connections only to the proctor), and
-  a visitor holding the old document for those minutes may ask for an asset the new deployment no longer has; a reload
-  fixes it. The staged `dist/pages/quizzes` also runs on CDNs that honour `_headers` (Cloudflare Pages, Netlify): there
-  hashed assets are immutable, documents are revalidated, and every response forbids framing.
-- The workflow publishes `linux/amd64`. On `linux/arm64` build the same Dockerfile on the host in a clone
-  (`docker build -f 🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/Dockerfile -t ghcr.io/usalu/architecture-quiz-proctor .`):
-  `compose.yaml` uses a local image as it is.
-- With an `AAAA` record, check once that the proctor sees real client addresses over IPv6 (Docker's userland proxy can
-  replace them with the bridge gateway, which would put every IPv6 learner into one rate-limit bucket):
-  `docker compose logs proctor` after a request made with `curl -6`.
diff --git a/🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs b/🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs
index 68b3f11ca22..c6c9a958e77 100644
--- a/🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs
+++ b/🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs
@@ -20,0 +21,4 @@ pub(crate) use original_source_fixture::{FixtureSource,FixtureSourceBirth};
+
+#[cfg(test)]
+#[path="♻️close/🎟️demand/🧪️tests/🦀️.rs"]
+mod close_demand_tests;
diff --git a/🧰️framework/🔨️modules/🎒️pack/🌱️value/🦀️.rs b/🧰️framework/🔨️modules/🎒️pack/🌱️value/🦀️.rs
index cd55b60e5d3..cbd6853e3bd 100644
--- a/🧰️framework/🔨️modules/🎒️pack/🌱️value/🦀️.rs
+++ b/🧰️framework/🔨️modules/🎒️pack/🌱️value/🦀️.rs
@@ -1281 +1281 @@ pub fn encode_document(spec: &RecordSpec, record: &RecordValue, options: &Encode
-    let write_options = semio_framework_pack_format::WriteOptions { required_flags: 0, optional_flags: if options.canonical { semio_framework_pack_format::OPTIONAL_CANONICAL } else { 0 }, codec: options.codec };
+    let write_options = semio_framework_pack_format::WriteOptions { required_flags:if semio_framework_pack_record_encode::record_required_chunked(record,options)?{semio_framework_pack_format::REQUIRED_CHUNKED}else{0}, optional_flags: if options.canonical { semio_framework_pack_format::OPTIONAL_CANONICAL } else { 0 }, codec: options.codec };
diff --git a/🧰️framework/🔨️modules/🎒️pack/📐️format/🗂️catalog/🦀️.rs b/🧰️framework/🔨️modules/🎒️pack/📐️format/🗂️catalog/🦀️.rs
index 22b0ce0054c..92953af106c 100644
--- a/🧰️framework/🔨️modules/🎒️pack/📐️format/🗂️catalog/🦀️.rs
+++ b/🧰️framework/🔨️modules/🎒️pack/📐️format/🗂️catalog/🦀️.rs
@@ -771,0 +772,24 @@ impl RetainedPackCatalogCursor {
+    pub fn original_step_demands(&self) -> Result<RetirementDemand, RetainedPackCatalogFault> {
+        if let Some(fault)=self.fault{return Err(fault)}
+        let mapped=|error|RetainedPackCatalogFault::from_paged_refusal(error,"retained-pack.catalog-original-demand",self.document_bytes);
+        if let Some((owner,target,offset))=self.allocation_need()?{
+            let capacity=self.requested_allocation_bytes(owner,target,offset)?;
+            if self.allocated_bytes_checked().and_then(|owned|owned.checked_add(capacity)).is_none_or(|total|total>self.maximum_allocation_bytes){return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.catalog-original-demand",offset))}
+            let (copy,depth)=match owner{
+                RetainedPackCatalogAllocationOwner::SymbolSpans=>(self.symbols.symbols.next_reserve_copy_byte_demand(),self.symbols.symbols.next_reserve_depth_demand()),
+                RetainedPackCatalogAllocationOwner::SymbolScalars=>(self.symbols.scalars.next_reserve_copy_byte_demand(),self.symbols.scalars.next_reserve_depth_demand()),
+                RetainedPackCatalogAllocationOwner::Chunks=>(self.chunks.next_reserve_copy_byte_demand(),self.chunks.next_reserve_depth_demand()),
+                RetainedPackCatalogAllocationOwner::ObservedChunks=>(self.observed_chunks.next_reserve_copy_byte_demand(),self.observed_chunks.next_reserve_depth_demand()),
+            };
+            return Ok(RetirementDemand{copy_bytes:copy.map_err(mapped)?,capacity_bytes:capacity,depth:depth.map_err(mapped)?,..Default::default()})
+        }
+        let depth=self.symbols.symbols.next_push_depth_demand().map_err(mapped)?.max(self.symbols.scalars.next_push_depth_demand().map_err(mapped)?).max(self.chunks.next_push_depth_demand().map_err(mapped)?).max(self.observed_chunks.next_push_depth_demand().map_err(mapped)?).max(1);
+        Ok(RetirementDemand{copy_bytes:std::mem::size_of::<RetainedPackSegmentEvent>(),depth,..Default::default()})
+    }
+    pub fn original_close_depth_demand(&self) -> Result<usize, PagedListError> {
+        let mut depth=1;
+        depth=depth.max(self.symbols.scalars.next_pop_depth_demand()?).max(self.symbols.symbols.next_pop_depth_demand()?).max(self.chunks.next_pop_depth_demand()?).max(self.observed_chunks.next_pop_depth_demand()?);
+        depth=depth.max(self.symbols.scalars.next_release_depth_demand()?).max(self.symbols.symbols.next_release_depth_demand()?).max(self.chunks.next_release_depth_demand()?).max(self.observed_chunks.next_release_depth_demand()?);
+        Ok(depth)
+    }
+
@@ -1184,0 +1209 @@ impl RetainedPackCatalogCursor {
+        if !semio_framework_pack_format::chunk_custody_matches(superblock.header.required_flags,raw.chunk_count,raw.chunk_table_span){return self.remember(Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.catalog-required-chunked",12)));}
diff --git a/🧰️framework/🔨️modules/🕸️graph/⏯️layout-run/🦀️.rs b/🧰️framework/🔨️modules/🕸️graph/⏯️layout-run/🦀️.rs
index 848df03f3d0..d0c04b11c2b 100644
--- a/🧰️framework/🔨️modules/🕸️graph/⏯️layout-run/🦀️.rs
+++ b/🧰️framework/🔨️modules/🕸️graph/⏯️layout-run/🦀️.rs
@@ -1496 +1496 @@ impl<E: LayoutRunOpEncoder> InteractiveJob for LayoutRunJob<E> {
-        if cx.should_yield() || grant.maximum_items == 0 { return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() }); }
+        if cx.retirement_should_yield() || grant.maximum_items == 0 { return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() }); }
diff --git a/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts b/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts
index 346be1f66e6..19dc6d4a28f 100644
--- a/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts
+++ b/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts
@@ -76 +76 @@ export class LintScript extends BundleScript {
-    const { runCargoLintV1, readCargoTestPolicyV1 } = await import("../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts");
+    const { readCargoTestPolicyV1 } = await import("../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts");
diff --git a/🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📜️script.ts b/🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📜️script.ts
index 4496ade0333..0ed3437b96e 100644
--- a/🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📜️script.ts
+++ b/🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📜️script.ts
@@ -1,0 +2 @@
+
diff --git a/🧰️framework/🛍️products/💻️os/🎚️config/🏭️bridge/Cargo.lock b/🧰️framework/🛍️products/💻️os/🎚️config/🏭️bridge/Cargo.lock
index 5ebecec1b91..3dba31e5abb 100644
--- a/🧰️framework/🛍️products/💻️os/🎚️config/🏭️bridge/Cargo.lock
+++ b/🧰️framework/🛍️products/💻️os/🎚️config/🏭️bridge/Cargo.lock
@@ -505,0 +506 @@ dependencies = [
+ "semio-framework-value-snapshot-read",
@@ -811,0 +813 @@ dependencies = [
+ "semio-framework-hash",
@@ -828,0 +831,7 @@ dependencies = [
+name = "semio-framework-value-snapshot-read"
+version = "0.1.0"
+dependencies = [
+ "semio-framework-value",
+]
+
+[[package]]
diff --git a/🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🟦️.ts b/🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🟦️.ts
index 6b7312f5c28..4ff75b80d3f 100644
--- a/🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🟦️.ts
+++ b/🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🟦️.ts
@@ -10 +10,2 @@ import { changeMergePolicy, diff as changeMergePolicyDiff, inverse as changeMerg
-import type { AdmitLocalDocument, LocalCatalog, LocalDocument, LocalDocumentStorage } from "./📥️admit-local-document/🟦️.ts";
+import { absorbIdentityDiff, absorbLocalCatalogDiff, applyIdentityDiff, applyLocalCatalogDiff } from "./⚙️apply-diff/🟦️.ts";
+import type { AdmitLocalDocument, LocalCatalog, LocalCatalogDiff, LocalCatalogRow, LocalDocument, LocalDocumentStorage } from "./📥️admit-local-document/🟦️.ts";
@@ -18 +19 @@ import { detachLocalFolder, diff as detachLocalFolderDiff, inverse as detachLoca
-import type { Identity, SignIn } from "./🪪️sign-in/🟦️.ts";
+import type { Identity, IdentityDiff, IdentitySessionEdit, SignIn } from "./🪪️sign-in/🟦️.ts";
@@ -93,2 +94,2 @@ export type IdentityConfigMutation = ({ readonly mutation: "signIn" } & SignIn)
-/** 🧮️ Delegates diff behavior to the direct semantic leaf named by the tag. */
-export function diffIdentityConfigMutation(mutation: IdentityConfigMutation, base: Identity | null): Identity | null {
+/** 🔺️ Delegates diff behavior to the direct semantic leaf named by the tag: the sparse diff it names, never a record. */
+export function diffIdentityConfigMutation(mutation: IdentityConfigMutation, base: Identity | null): IdentityDiff {
@@ -103 +104 @@ export function inverseIdentityConfigMutation(mutation: IdentityConfigMutation,
-/** 🧮️ Applies an identity mutation's whole-record diff. */
+/** 🧮️ Applies an identity mutation: its sparse diff through the central twin applier. */
@@ -105 +106,15 @@ export function applyIdentityConfigMutation(base: Identity | null, mutation: Ide
-  return diffIdentityConfigMutation(mutation, base);
+  return applyIdentityDiff(base, diffIdentityConfigMutation(mutation, base));
+}
+
+/** 🔺️ What one identity mutation records: its sparse diff and the sum of its concrete inverse mutations' diffs (replayed last to
+ * first, each read against the state the previous one left), so the inverse is the mutation's own, never derived from the diff. */
+export function identityMutationDiffPair(mutation: IdentityConfigMutation, base: Identity | null): { readonly diff: IdentityDiff; readonly inverse: IdentityDiff } {
+  const diff = diffIdentityConfigMutation(mutation, base);
+  let state = applyIdentityDiff(base, diff);
+  let inverse: IdentityDiff = {};
+  for (const step of [...inverseIdentityConfigMutation(mutation, base)].reverse()) {
+    const stepDiff = diffIdentityConfigMutation(step, state);
+    state = applyIdentityDiff(state, stepDiff);
+    inverse = absorbIdentityDiff(inverse, stepDiff);
+  }
+  return { diff, inverse };
@@ -109 +124 @@ export { changeMergePolicy, clearDefaultApp, setAppearance, setCustomDriver, set
-export type { ChangeMergePolicy, ClearDefaultApp, Identity, MergePolicySetting, SetAppearance, SetCustomDriver, SetCustomTheme, SetDefaultApp, SetDriver, SetKeybindingOverride, SetLayout, SetLocale, SetNamedLayout, SetTerminology, SetTheme, SignIn, SignOut };
+export type { ChangeMergePolicy, ClearDefaultApp, Identity, IdentityDiff, IdentitySessionEdit, MergePolicySetting, SetAppearance, SetCustomDriver, SetCustomTheme, SetDefaultApp, SetDriver, SetKeybindingOverride, SetLayout, SetLocale, SetNamedLayout, SetTerminology, SetTheme, SignIn, SignOut };
@@ -116,2 +131,2 @@ export type LocalCatalogConfigMutation = ({ readonly mutation: "admitLocalDocume
-/** 🧮️ Delegates local-catalog behavior to the direct semantic leaf named by the tag. */
-export function applyLocalCatalogConfigMutation(base: LocalCatalog, mutation: LocalCatalogConfigMutation): LocalCatalog {
+/** 🔺️ Delegates local-catalog diff behavior to the direct semantic leaf named by the tag: the sparse diff it names, never a catalog. */
+export function diffLocalCatalogConfigMutation(mutation: LocalCatalogConfigMutation, base: LocalCatalog): LocalCatalogDiff {
@@ -120,0 +136,19 @@ export function applyLocalCatalogConfigMutation(base: LocalCatalog, mutation: Lo
+/** 🧮️ Applies a local-catalog mutation: its sparse diff through the central twin applier. */
+export function applyLocalCatalogConfigMutation(base: LocalCatalog, mutation: LocalCatalogConfigMutation): LocalCatalog {
+  return applyLocalCatalogDiff(base, diffLocalCatalogConfigMutation(mutation, base));
+}
+
+/** 🔺️ What one local-catalog mutation records: its sparse diff and the sum of its concrete inverse mutations' diffs (replayed last
+ * to first, each read against the state the previous one left), so the inverse is the mutation's own, never derived from the diff. */
+export function localCatalogMutationDiffPair(mutation: LocalCatalogConfigMutation, base: LocalCatalog): { readonly diff: LocalCatalogDiff; readonly inverse: LocalCatalogDiff } {
+  const diff = diffLocalCatalogConfigMutation(mutation, base);
+  let state = applyLocalCatalogDiff(base, diff);
+  let inverse: LocalCatalogDiff = {};
+  for (const step of [...inverseLocalCatalogConfigMutation(mutation, base)].reverse()) {
+    const stepDiff = diffLocalCatalogConfigMutation(step, state);
+    state = applyLocalCatalogDiff(state, stepDiff);
+    inverse = absorbLocalCatalogDiff(inverse, stepDiff);
+  }
+  return { diff, inverse };
+}
+
@@ -127 +161 @@ export { admitLocalDocument, LOCAL_CATALOG_CONFIG_SCHEMA, retireLocalDocument };
-export type { AdmitLocalDocument, LocalCatalog, LocalDocument, LocalDocumentStorage, RetireLocalDocument };
+export type { AdmitLocalDocument, LocalCatalog, LocalCatalogDiff, LocalCatalogRow, LocalDocument, LocalDocumentStorage, RetireLocalDocument };
diff --git a/🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs b/🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs
index 0432d0b2432..dc332698521 100644
--- a/🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs
+++ b/🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs
@@ -1421 +1421 @@ impl<H: AppChannelHost, B: BlobStore + 'static> SpaceRunner<H, B> {
-                sink.record(RunMutation::FinishRunNode(FinishRunNode { node_record })).await?;
+                sink.record(RunMutation::FinishRunNode(FinishRunNode { node_record, index: None })).await?;
@@ -1486 +1486 @@ impl<H: AppChannelHost, B: BlobStore + 'static> SpaceRunner<H, B> {
-            sink.record(RunMutation::FinishRunNode(FinishRunNode { node_record })).await?;
+            sink.record(RunMutation::FinishRunNode(FinishRunNode { node_record, index: None })).await?;
diff --git a/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🏛️reserved/🦀️.rs b/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🏛️reserved/🦀️.rs
index 062852f03e4..75a90ccbbb2 100644
--- a/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🏛️reserved/🦀️.rs
+++ b/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🏛️reserved/🦀️.rs
@@ -84 +84,2 @@ pub(crate) fn framework_reserved_job_factory(_job:u64,input:&mut Option<Vec<u8>>
-    crate::reactor::jobs::admit_original_job(input,restored,cx,|input,restored|FrameworkReservedBoundedJob{source:Some((input,restored)),source_close:None,output:None,output_close:None,phase:0,scan:0,prefix:0,matched:true,cancelled:false})
+    cx.original_source_parent()?;let binding=semio_framework_job::OriginalSourceBinding::borrow(cx);
+    crate::reactor::jobs::admit_original_job(input,restored,cx,|input,restored|FrameworkReservedBoundedJob::new(input,restored,binding))
@@ -87,0 +89 @@ struct FrameworkReservedBoundedJob {
+    binding:semio_framework_job::OriginalSourceBinding,
@@ -99,41 +101,39 @@ struct FrameworkReservedBoundedJob {
-impl crate::reactor::jobs::BoundedJob for FrameworkReservedBoundedJob {
-    fn step(&mut self,_original:&mut crate::reactor::jobs::IoRunControl<'_,'_>,_snapshot:&mut crate::reactor::jobs::SqliteSnapshotControl<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<crate::reactor::jobs::JobStep,OriginalTurnFailure>{
-        use crate::reactor::jobs::JobStep;
-        if self.cancelled||cx.is_cancelled(){return Ok(JobStep::Running(None))}
-        let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(JobStep::Running(None))}
-        match self.phase{
-            0=>{
-                let raw=&self.source.as_ref().unwrap().0;
-                if raw.len()<16{self.phase=1;return Ok(JobStep::Running(None))}
-                if grant.maximum_copy_bytes==0{return Ok(JobStep::Running(None))}
-                self.matched&=raw[self.scan]==FRAMEWORK_RESERVED_JOB_MAGIC[self.scan];self.scan+=1;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()})?;
-                if self.scan==FRAMEWORK_RESERVED_JOB_MAGIC.len(){self.prefix=if self.matched{16}else{0};self.phase=1;self.scan=0}
-                Ok(JobStep::Running(None))
-            },
-            1=>{
-                let length=self.source.as_ref().unwrap().0.len()-self.prefix;if grant.maximum_capacity_bytes<length{return Ok(JobStep::Running(None))}
-                let output=self.output.get_or_insert_with(Vec::new);let before=output.capacity();output.try_reserve_exact(length).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original reserved job output allocation refused"))?;
-                self.phase=2;cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:output.capacity()-before,..Default::default()})?;Ok(JobStep::Running(None))
-            },
-            2=>{
-                let raw=&self.source.as_ref().unwrap().0;let remaining=raw.len()-self.prefix-self.scan;if remaining==0{self.phase=3;return Ok(JobStep::Running(None))}
-                let count=remaining.min(grant.maximum_copy_bytes).min(64);if count==0{return Ok(JobStep::Running(None))}
-                self.output.as_mut().unwrap().extend_from_slice(&raw[self.prefix+self.scan..self.prefix+self.scan+count]);self.scan+=count;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()})?;Ok(JobStep::Running(None))
-            },
-            3=>{if crate::reactor::jobs::close_original(&mut self.source,&mut self.source_close,cx)?{self.phase=4}Ok(JobStep::Running(None))},
-            4=>{cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;self.phase=5;Ok(JobStep::Done(self.output.take().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original reserved job has no ready output"))?))},
-            _=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original reserved job is terminal").into()),
-        }
-    }
-
-    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,OriginalTurnFailure>{
-        self.cancelled=true;
-        Ok(crate::reactor::jobs::close_original(&mut self.source,&mut self.source_close,cx)?&&crate::reactor::jobs::close_original(&mut self.output,&mut self.output_close,cx)?)
-    }
-
-    fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
-        if let Some(owner)=&self.source_close{return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?,..Default::default()})}
-        if let Some(owner)=&self.output_close{return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?,..Default::default()})}
-        if self.cancelled{return Ok(RetirementDemand{depth:usize::from(self.source.is_some()||self.output.is_some()),..Default::default()})}
-        Ok(match self.phase{0|2=>RetirementDemand{copy_bytes:1,depth:1,..Default::default()},1=>RetirementDemand{capacity_bytes:self.source.as_ref().unwrap().0.len()-self.prefix,depth:1,..Default::default()},3|4=>RetirementDemand{depth:1,..Default::default()},_=>Default::default()})
-    }
+impl FrameworkReservedBoundedJob{
+ fn new(input:Vec<u8>,restored:Option<Vec<u8>>,binding:semio_framework_job::OriginalSourceBinding)->Self{Self{binding,source:Some((input,restored)),source_close:None,output:None,output_close:None,phase:0,scan:0,prefix:0,matched:true,cancelled:false}}
+ fn validate_original(&self,cx:&semio_framework_job::StepContext<'_>)->Result<(),ValueError>{cx.original_source_parent()?;if !self.binding.matches(cx){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"reserved job requires its exact original source and clock"))}Ok(())}
+ fn step_original(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<crate::reactor::jobs::JobStep,OriginalTurnFailure>{
+  use crate::reactor::jobs::JobStep;
+  self.validate_original(cx)?;if !cx.retained_recipient_is_admitted(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"reserved job recipient exceeds its original grant").into())}
+  if self.cancelled||cx.is_cancelled()||cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(JobStep::Running(None))}
+  let before=cx.retained_progress();let work=cx.original_work_units_spent();let result=self.advance_original(cx);
+  if cx.retained_progress()!=before&&cx.original_work_units_spent()==work{cx.consume_fuel(1);}
+  result
+ }
+ fn advance_original(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<crate::reactor::jobs::JobStep,OriginalTurnFailure>{
+  use crate::reactor::jobs::JobStep;let grant=cx.retained_grant();
+  match self.phase{
+   0=>{
+    let raw=&self.source.as_ref().unwrap().0;
+    if raw.len()<16{cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;self.phase=1;return Ok(JobStep::Running(None))}
+    if grant.maximum_copy_bytes==0{return Ok(JobStep::Running(None))}
+    self.matched&=raw[self.scan]==FRAMEWORK_RESERVED_JOB_MAGIC[self.scan];self.scan+=1;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()})?;
+    if self.scan==FRAMEWORK_RESERVED_JOB_MAGIC.len(){self.prefix=if self.matched{16}else{0};self.phase=1;self.scan=0}
+    Ok(JobStep::Running(None))
+   },
+   1=>{
+    let length=self.source.as_ref().unwrap().0.len()-self.prefix;
+    let result=semio_framework_job::admit_original_construction(cx,&mut self.output,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:0,capacity_bytes:length,release_bytes:0,depth:1},|_|{let mut output=Vec::new();output.try_reserve_exact(length).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original reserved job output allocation refused"))?;let capacity=output.capacity();Ok((output,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity,..Default::default()}))});
+    if self.output.is_some(){self.phase=2;}result?;Ok(JobStep::Running(None))
+   },
+   2=>{
+    let raw=&self.source.as_ref().unwrap().0;let remaining=raw.len()-self.prefix-self.scan;
+    if remaining==0{cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;self.phase=3;return Ok(JobStep::Running(None))}
+    let count=remaining.min(grant.maximum_copy_bytes).min(64);if count==0{return Ok(JobStep::Running(None))}
+    self.output.as_mut().unwrap().extend_from_slice(&raw[self.prefix+self.scan..self.prefix+self.scan+count]);self.scan+=count;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()})?;Ok(JobStep::Running(None))
+   },
+   3=>{if crate::reactor::jobs::close_original(&mut self.source,&mut self.source_close,cx)?{self.phase=4}Ok(JobStep::Running(None))},
+   4=>{cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;self.phase=5;Ok(JobStep::Done(self.output.take().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original reserved job has no ready output"))?))},
+   _=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original reserved job is terminal").into()),
+  }
+ }
+}
@@ -141,3 +141,17 @@ impl crate::reactor::jobs::BoundedJob for FrameworkReservedBoundedJob {
-    fn cancel(&mut self){self.cancelled=true}
-    fn checkpoint(&self)->Option<Vec<u8>>{None}
-    fn terminal_drop_is_shallow(&self)->bool{self.source.is_none()&&self.source_close.is_none()&&self.output.is_none()&&self.output_close.is_none()}
+impl crate::reactor::jobs::BoundedJob for FrameworkReservedBoundedJob {
+ fn step(&mut self,_original:&mut crate::reactor::jobs::IoRunControl<'_,'_>,_snapshot:&mut crate::reactor::jobs::SqliteSnapshotControl<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<crate::reactor::jobs::JobStep,OriginalTurnFailure>{self.step_original(cx)}
+ fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,OriginalTurnFailure>{
+  self.validate_original(cx)?;if self.terminal_drop_is_shallow(){return Ok(true)}
+  if !self.cancelled{let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);self.cancelled=true;return Ok(false)}
+  Ok(crate::reactor::jobs::close_original(&mut self.source,&mut self.source_close,cx)?&&crate::reactor::jobs::close_original(&mut self.output,&mut self.output_close,cx)?)
+ }
+ fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
+  if self.terminal_drop_is_shallow(){return Ok(Default::default())}
+  if !self.cancelled{return Ok(RetirementDemand{depth:1,..Default::default()})}
+  if let Some(owner)=&self.source_close{return owner.next_demand(copy)}
+  if let Some(owner)=&self.output_close{return owner.next_demand(copy)}
+  Ok(RetirementDemand{depth:1,..Default::default()})
+ }
+ fn cancel(&mut self){self.cancelled=true}
+ fn checkpoint(&self)->Option<Vec<u8>>{None}
+ fn terminal_drop_is_shallow(&self)->bool{self.source.is_none()&&self.source_close.is_none()&&self.output.is_none()&&self.output_close.is_none()}
@@ -144,0 +159,4 @@ impl crate::reactor::jobs::BoundedJob for FrameworkReservedBoundedJob {
+
+#[cfg(test)]
+#[path="🧪️tests/🦀️.rs"]
+mod tests;
diff --git a/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs b/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs
index 6f0241e1446..2621ae524d8 100644
--- a/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs
+++ b/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs
@@ -467,0 +468 @@ pub use turn::poll_kernel;
+pub(crate) use turn::settle_failed_original_publication;
diff --git a/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/🦀️.rs b/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/🦀️.rs
index ee7caf49d49..10bcddcd3e3 100644
--- a/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/🦀️.rs
+++ b/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/🦀️.rs
@@ -1,0 +2 @@
+pub(crate) use semio_framework_actor::typed_operation_diagnostics as diagnostic_publication;
@@ -119 +120 @@ pub(crate) struct MountedTypedCommandFullOperation<A: ArtifactApp> {
-    pub(crate) source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation),
+    pub(crate) source_identity:semio_framework_job::OriginalSourceBinding,
@@ -130,0 +132,3 @@ pub(crate) struct MountedTypedCommandFullOperation<A: ArtifactApp> {
+    pub(crate) producer_source: Option<semio_framework::action_bus::OriginalToolSourceBinding>,
+    pub(crate) producer_request: Option<crate::tool_contract::ArtifactOwnedToolJobRequest<A>>,
+    pub(crate) producer_request_retirement: Option<ControlledRetirement<crate::tool_contract::ArtifactOwnedToolJobRequest<A>>>,
@@ -157,0 +162 @@ pub(crate) struct MountedTypedCommandFullOperation<A: ArtifactApp> {
+    pub(crate) diagnostics: Option<diagnostic_publication::OperationDiagnosticPublication>,
@@ -193 +198 @@ impl<A:ArtifactApp> MountedTypedCommandFullOperation<A>{
- pub(crate) fn lease_retirement_ready(&self)->bool{self.stage==MountedTypedCommandFullOperationStage::Retiring&&self.session.is_none()&&self.session_rejected.is_none()&&!self.worker_semantic_pending&&!self.worker_outcome_pending&&self.worker_fault_capture.is_none()&&self.publication.is_none()&&self.publication_retirement.is_none()&&self.pending_artifact_publication.is_none()&&!self.pending_publication_outcome.pending()&&self.pending_child_publication.is_none()&&self.owned_child_group.is_none()&&self.captured_child_content.is_none()&&self.result_page.is_none()&&self.pending_window_config_receipt.is_none()&&self.raw_input.is_none()&&self.output_chunks.is_none()&&self.output_retirement.is_none()&&self.reserved_producer.is_none()&&self.completion.is_none()&&self.completion_retirement.is_none()&&self.meta.view_state.is_none()&&self.window_config_authority.is_none()&&self.window_transient_authority.is_none()&&self.actor_capture.is_none()&&self.verb.capacity()==0&&!self.meta.actor.has_owner()&&self.retained_close_fault.is_none()&&(self.cancellation_lease.is_some()||self.cancellation_retirement.is_some())}
+ pub(crate) fn lease_retirement_ready(&self)->bool{self.stage==MountedTypedCommandFullOperationStage::Retiring&&self.session.is_none()&&self.producer_request.is_none()&&self.producer_request_retirement.is_none()&&self.session_rejected.is_none()&&!self.worker_semantic_pending&&!self.worker_outcome_pending&&self.worker_fault_capture.is_none()&&self.publication.is_none()&&self.publication_retirement.is_none()&&self.pending_artifact_publication.is_none()&&!self.pending_publication_outcome.pending()&&self.pending_child_publication.is_none()&&self.owned_child_group.is_none()&&self.captured_child_content.is_none()&&self.result_page.is_none()&&self.pending_window_config_receipt.is_none()&&self.raw_input.is_none()&&self.output_chunks.is_none()&&self.output_retirement.is_none()&&self.reserved_producer.is_none()&&self.completion.is_none()&&self.completion_retirement.is_none()&&self.meta.view_state.is_none()&&self.window_config_authority.is_none()&&self.window_transient_authority.is_none()&&self.actor_capture.is_none()&&self.verb.capacity()==0&&!self.meta.actor.has_owner()&&self.retained_close_fault.is_none()&&(self.cancellation_lease.is_some()||self.cancellation_retirement.is_some())}
@@ -253,2 +258,5 @@ impl<A: ArtifactApp> MountedTypedCommandFullOperation<A> {
-        if self.source_identity!=(cx.operation(),cx.generation()){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"mounted worker requires its original source identity independently of its registry record").into())}
-        let grant=cx.retained_grant();let demand=if cx.is_cancelled()&&self.session_rejected.as_ref().is_some_and(|source|source.admission_pending()){self.worker_retirement_demands(grant.maximum_copy_bytes)?}else{self.worker_step_demands(grant.maximum_copy_bytes)?};
+        self.validate_original_producer_source(cx)?;
+        if self.stage != MountedTypedCommandFullOperationStage::Worker {return Ok(PluginLifecycleStep::Blocked{reason:"typed command awaits its bounded revision-validated publication or ACK turn"});}
+        if !self.source_identity.matches(cx){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"mounted worker requires its original source identity independently of its registry record").into())}
+        if (self.producer_request.is_some()||self.original_reserved_producer_pending())&&!self.user_cancel_requested&&!self.terminal_seen&&!cx.is_cancelled(){return self.drive_original_producer(cx);}
+        let grant=cx.retained_grant();let demand=if cx.is_cancelled()&&(self.original_producer_pending()||self.session_rejected.as_ref().is_some_and(|source|source.admission_pending())){self.worker_retirement_demands(grant.maximum_copy_bytes)?}else{self.worker_step_demands(grant.maximum_copy_bytes)?};
@@ -258,0 +267 @@ impl<A: ArtifactApp> MountedTypedCommandFullOperation<A> {
+        if self.original_producer_pending(){return self.retire_original_producer(grant);}
@@ -260 +269 @@ impl<A: ArtifactApp> MountedTypedCommandFullOperation<A> {
-            if rejected.admission_pending()&&!cx.is_cancelled(){
+            if rejected.admission_pending()&&!cx.is_cancelled()&&!self.user_cancel_requested&&!self.terminal_seen{
@@ -417,6 +426,29 @@ impl<A: ArtifactApp> MountedTypedCommandFullOperation<A> {
-    pub(crate) fn take_result_page(&mut self) -> Option<TypedOperationResultPage> {
-        if self.result_page_presented {
-            return None;
-        }
-        self.result_page_presented = true;
-        self.result_page.clone()
+    pub(crate) fn result_page_presentation_demands(&self)->Result<RetirementDemand,ValueError>{self.result_page.as_ref().map_or(Ok(Default::default()),|page|{let mut d=page.presentation_demand()?;d.copy_bytes+=size_of::<bool>();Ok(d)})}
+    pub(crate) fn take_result_page(&mut self,grant:RetainedCloneGrant) -> Result<(Option<TypedOperationResultPage>,RetainedCloneProgress),Fault> {
+        if self.result_page_presented { return Ok((None,Default::default())); }
+        let demand=self.result_page_presentation_demands().map_err(plugin_retirement_fault)?;
+        if !plugin_turn_admitted(demand,grant)?{return Ok((None,Default::default()));}
+        let Some(page)=self.result_page.as_mut()else{return Ok((None,Default::default()))};
+        let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<bool>(),..grant};
+        let(result,mut progress)=page.present(child).map_err(plugin_retirement_fault)?;
+        if result.is_some(){self.result_page_presented=true;progress.copied_bytes+=size_of::<bool>();}
+        Ok((result,progress))
+    }
+
+    /// 🧪️ Supplies each production presentation turn with the fixture issuer's unchanged grant.
+    #[cfg(test)]
+    pub(crate) fn take_fixture_result_page(&mut self,grant:RetainedCloneGrant)->Option<TypedOperationResultPage>{
+        for _ in 0..2{let((page,progress),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||self.take_result_page(grant).expect("original fixture page presentation"));assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));if page.is_some()||progress==RetainedCloneProgress::default(){return page;}}
+        panic!("original fixture page sealing omitted its next projection");
+    }
+    /// 🧪️ Pays the original page retirement through the production close port after exact ACK.
+    #[cfg(test)]
+    pub(crate) fn close_fixture_original_page(&mut self,grant:RetainedCloneGrant){for _ in 0..100000{if !self.result_page_retirement_pending(){return;}let(p,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||self.result_page_retirement_step(grant).expect("original fixture producer page close"));assert!(p.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert_ne!(p,Default::default(),"original fixture close grant funds the exact producer page");}panic!("original fixture producer page did not close");}
+    /// ♻️ Exact ACK retains the original page until its funded physical closure.
+    pub(crate) fn result_page_retirement_pending(&self)->bool{self.result_page.is_some()&&self.stage!=MountedTypedCommandFullOperationStage::AwaitingAck}
+    pub(crate) fn result_page_retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.result_page.as_ref().map_or(Ok(Default::default()),|page|{let mut d=page.next_close_demand(body.saturating_sub(size_of::<Option<TypedOperationResultPage>>()+size_of::<bool>()))?;d.copy_bytes+=size_of::<Option<TypedOperationResultPage>>()+size_of::<bool>();Ok(d)})}
+    pub(crate) fn result_page_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,Fault>{
+        if !self.result_page_retirement_pending(){return Ok(Default::default());}
+        let demand=self.result_page_retirement_demands(grant.maximum_copy_bytes).map_err(plugin_retirement_fault)?;if !plugin_turn_admitted(demand,grant)?{return Ok(Default::default());}let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<Option<TypedOperationResultPage>>()-size_of::<bool>(),..grant};let page=self.result_page.as_mut().unwrap();let mut progress=page.close_step(child).map_err(plugin_retirement_fault)?.progress();
+        if page.terminal_is_empty(){self.result_page=None;self.result_page_presented=false;progress.copied_bytes+=size_of::<Option<TypedOperationResultPage>>()+size_of::<bool>();}
+        Ok(progress)
@@ -447 +479,3 @@ impl<A: ArtifactApp> MountedTypedCommandFullOperation<A> {
-        self.result_page = None;
+        if lane == TypedOperationResultLane::Diagnostic && !self.diagnostics.as_mut().is_some_and(|owner|owner.acknowledge(token)) {
+            return Err(plugin_sdk_fault("diagnostic result ACK lost its exact original source authority"));
+        }
@@ -449 +482,0 @@ impl<A: ArtifactApp> MountedTypedCommandFullOperation<A> {
-        self.result_page_presented = false;
@@ -455,0 +489 @@ impl<A: ArtifactApp> MountedTypedCommandFullOperation<A> {
+        if self.stage==MountedTypedCommandFullOperationStage::Retiring{if let Some(owner)=self.diagnostics.as_mut(){owner.begin_close();}}
@@ -462,0 +497 @@ impl<A: ArtifactApp> MountedTypedCommandFullOperation<A> {
+        if self.original_producer_pending(){return if self.user_cancel_requested||self.terminal_seen{self.original_producer_retirement_demands(body)}else{Ok(item)};}
@@ -476 +511 @@ impl<A: ArtifactApp> MountedTypedCommandFullOperation<A> {
-        self.session.is_some() || self.session_rejected.is_some() || self.worker_semantic_pending || self.worker_outcome_pending || self.worker_fault_capture.is_some()
+        self.original_producer_pending() || self.session.is_some() || self.session_rejected.is_some() || self.worker_semantic_pending || self.worker_outcome_pending || self.worker_fault_capture.is_some()
@@ -481,0 +517 @@ impl<A: ArtifactApp> MountedTypedCommandFullOperation<A> {
+        if self.original_producer_pending(){return self.original_producer_retirement_demands(body);}
@@ -498,0 +535 @@ impl<A: ArtifactApp> MountedTypedCommandFullOperation<A> {
+        self.validate_original_producer_source(cx)?;
@@ -502,0 +540 @@ impl<A: ArtifactApp> MountedTypedCommandFullOperation<A> {
+        if self.original_producer_pending(){return self.retire_original_producer(grant);}
@@ -554,0 +593 @@ impl<A: ArtifactApp> MountedTypedCommandFullOperation<A> {
+        if self.result_page_retirement_pending(){return self.result_page_retirement_demands(body);}
@@ -556,0 +596 @@ impl<A: ArtifactApp> MountedTypedCommandFullOperation<A> {
+        if let Some(owner)=self.diagnostics.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<diagnostic_publication::OperationDiagnosticPublication>>(),..item})}else{owner.next_demand(body)}}
@@ -572,0 +613 @@ impl<A: ArtifactApp> MountedTypedCommandFullOperation<A> {
+        self.validate_original_producer_source(cx)?;
@@ -578,0 +620 @@ impl<A: ArtifactApp> MountedTypedCommandFullOperation<A> {
+        if self.result_page_retirement_pending(){return self.result_page_retirement_step(grant).map(PluginLifecycleStep::Progress);}
@@ -631,0 +674 @@ impl<A: ArtifactApp> MountedTypedCommandFullOperation<A> {
+        if let Some(owner)=self.diagnostics.as_mut(){owner.begin_close();}
@@ -645,0 +689,2 @@ impl<A: ArtifactApp> MountedTypedCommandFullOperation<A> {
+            && self.producer_request.is_none()
+            && self.producer_request_retirement.is_none()
@@ -656,0 +702 @@ impl<A: ArtifactApp> MountedTypedCommandFullOperation<A> {
+            && self.diagnostics.is_none()
@@ -1005,0 +1052 @@ async fn drive_host_typed_continuations<P: PluginApp>(app: &mut P, receiver: u32
+    let mut receiving_job=crate::app_testing::artifact_app_laws::FixtureOriginalJob::new(crate::app_testing::artifact_app_laws::fixture_retained_turn());
@@ -1012 +1059 @@ async fn drive_host_typed_continuations<P: PluginApp>(app: &mut P, receiver: u32
-        app.advance_typed_operation_publication(identity,mounted_policy.preparation).await.expect("one host continuation unit");
+        receiving_job.receive(mounted_policy.preparation,|receiving|::semio_framework_async::poll::resolve_ready(app.advance_typed_operation_publication(identity,mounted_policy.preparation,receiving))).expect("one host continuation unit");
@@ -1014,2 +1061,2 @@ async fn drive_host_typed_continuations<P: PluginApp>(app: &mut P, receiver: u32
-        while let Some(page) = app.take_typed_operation_result_page(receiver) {
-            assert!(app.acknowledge_typed_operation_result(page.token).expect("presented result page accepts its exact token"));
+        while let Some(mut page) = crate::app_testing::artifact_app_laws::take_fixture_result_page(app,receiver,mounted_policy.preparation).expect("original fixture page presentation") {
+            assert!(app.acknowledge_typed_operation_result(crate::app_testing::artifact_app_laws::close_fixture_result_page(&mut page,mounted_policy.close)).expect("presented result page accepts its exact token"));
@@ -1036,0 +1084 @@ async fn drive_host_call_with_deferred_acks<P: PluginApp>(app: &mut P, receiver:
+    let mut receiving_job=crate::app_testing::artifact_app_laws::FixtureOriginalJob::new(crate::app_testing::artifact_app_laws::fixture_retained_turn());
@@ -1047 +1095 @@ async fn drive_host_call_with_deferred_acks<P: PluginApp>(app: &mut P, receiver:
-        app.advance_typed_operation_publication(identity,mounted_policy.preparation).await.expect("one host continuation unit");
+        receiving_job.receive(mounted_policy.preparation,|receiving|::semio_framework_async::poll::resolve_ready(app.advance_typed_operation_publication(identity,mounted_policy.preparation,receiving))).expect("one host continuation unit");
@@ -1049 +1097 @@ async fn drive_host_call_with_deferred_acks<P: PluginApp>(app: &mut P, receiver:
-        while let Some(page) = app.take_typed_operation_result_page(receiver) {
+        while let Some(mut page) = crate::app_testing::artifact_app_laws::take_fixture_result_page(app,receiver,mounted_policy.preparation).expect("original fixture page presentation") {
@@ -1053 +1101 @@ async fn drive_host_call_with_deferred_acks<P: PluginApp>(app: &mut P, receiver:
-            acknowledgements.push(page.token);
+            acknowledgements.push(crate::app_testing::artifact_app_laws::close_fixture_result_page(&mut page,mounted_policy.close));
@@ -1293,0 +1342,2 @@ pub use lifted_full_operation_job::*;
+
+include!("🏭️producer/🦀️.rs");
diff --git a/🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧱️model/🦀️.rs b/🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧱️model/🦀️.rs
index c66ff5b641f..9840292ec0d 100644
--- a/🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧱️model/🦀️.rs
+++ b/🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧱️model/🦀️.rs
@@ -884 +884 @@ impl OperatorWorkFailure{
- pub fn retained_progress(&self)->RetainedCloneProgress{match self{Self::Evaluation{retained,..}=>*retained,Self::Original(failure)=>failure.primary.retained_progress()}}
+ pub fn retained_progress(&self)->RetainedCloneProgress{match self{Self::Evaluation{retained,..}=>*retained,Self::Original(failure)=>failure.retained_progress()}}
diff --git a/🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/📦️packages/🦀️rust/🦀️.rs b/🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/📦️packages/🦀️rust/🦀️.rs
index 8c23123afa1..6b52ff96221 100644
--- a/🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/📦️packages/🦀️rust/🦀️.rs
+++ b/🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/📦️packages/🦀️rust/🦀️.rs
@@ -1249 +1249 @@ pub struct Ticket {
-    #[serde(skip)]
+    #[serde(rename = "interactions", default, skip_serializing_if = "Vec::is_empty")]
diff --git a/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🟦️.ts b/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🟦️.ts
index 119324c3440..0cc421d5758 100644
--- a/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🟦️.ts
+++ b/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🟦️.ts
@@ -120 +120 @@ test("process budgets let captured nextest compilation finish before budgeted as
-    await receiveScriptProcessInvocation(process.env, original => runRepositoryCargoTests(["semio-s-artifact-stdio-mp4"], process.cwd(), original.control));
+    await receiveScriptProcessInvocation(process.env, original => runRepositoryCargoTests(["semio-s-artifact-stdio-mp4"], process.cwd(), original));
@@ -147 +147 @@ for (const nextest of [true, false]) {
-    await receiveScriptProcessInvocation(process.env, original => runRepositoryCargoTests(["semio-s-artifact-stdio-mp4"], process.cwd(), original.control));
+    await receiveScriptProcessInvocation(process.env, original => runRepositoryCargoTests(["semio-s-artifact-stdio-mp4"], process.cwd(), original));
@@ -176 +176 @@ test("process budgets bind nested native workspace profiles to the repository co
-    const code=`const{mock}=await import("bun:test");const preparePath=${JSON.stringify(fileURLToPath(new URL("../../🗂️workspaces/🦀️cargo/📜️script.ts",import.meta.url)))};const originalBunSpawnSync=Bun.spawnSync.bind(Bun);Bun.spawnSync=(args,options)=>args[1]===preparePath?{exitCode:0,stdout:new Uint8Array(),stderr:new Uint8Array()}:originalBunSpawnSync(args,options);const native=await import("node:child_process");const spawn=native.spawn,spawnSync=native.spawnSync;mock.module("node:child_process",()=>({...native,spawnSync:(cmd,args,opts)=>cmd==="cargo"?{status:0}:spawnSync(cmd,args,opts),spawn:(cmd,args,opts)=>{if(cmd!=="cargo")return spawn(cmd,args,opts);console.error(JSON.stringify(args));return spawn(process.execPath,["-e",args.includes("list")?"console.log('{}')":"console.log('native-complete')"],opts)}}));const{runRepositoryCargoTests,getWorkspaceRoot}=await import(${JSON.stringify(libraryPath)});console.error(JSON.stringify({root:getWorkspaceRoot()}));const{receiveScriptProcessInvocation}=await import(${JSON.stringify(processInvocationPath)});await receiveScriptProcessInvocation(process.env,original=>runRepositoryCargoTests([${JSON.stringify(corpus.nativePackage.name)}],process.cwd(),original.control));`;
+    const code=`const{mock}=await import("bun:test");const preparePath=${JSON.stringify(fileURLToPath(new URL("../../🗂️workspaces/🦀️cargo/📜️script.ts",import.meta.url)))};const originalBunSpawnSync=Bun.spawnSync.bind(Bun);Bun.spawnSync=(args,options)=>args[1]===preparePath?{exitCode:0,stdout:new Uint8Array(),stderr:new Uint8Array()}:originalBunSpawnSync(args,options);const native=await import("node:child_process");const spawn=native.spawn,spawnSync=native.spawnSync;mock.module("node:child_process",()=>({...native,spawnSync:(cmd,args,opts)=>cmd==="cargo"?{status:0}:spawnSync(cmd,args,opts),spawn:(cmd,args,opts)=>{if(cmd!=="cargo")return spawn(cmd,args,opts);console.error(JSON.stringify(args));return spawn(process.execPath,["-e",args.includes("list")?"console.log('{}')":"console.log('native-complete')"],opts)}}));const{runRepositoryCargoTests,getWorkspaceRoot}=await import(${JSON.stringify(libraryPath)});console.error(JSON.stringify({root:getWorkspaceRoot()}));const{receiveScriptProcessInvocation}=await import(${JSON.stringify(processInvocationPath)});await receiveScriptProcessInvocation(process.env,original=>runRepositoryCargoTests([${JSON.stringify(corpus.nativePackage.name)}],process.cwd(),original));`;

```

# Deleted Owner Deltas

## `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚑️flag/🎮️prepare/↩️inverse/🦀️.rs`

```diff
diff --git a/1bd390643826dd415e8c9466d1950a80c42acfab b/95a84f14224b3bd4a70adb4e04b86aa70b64c027
index 1bd39064382..95a84f14224 100644
--- a/1bd390643826dd415e8c9466d1950a80c42acfab
+++ b/95a84f14224b3bd4a70adb4e04b86aa70b64c027
@@ -3,2 +3,10 @@
-use crate::standards::v1::subsets::any::schema::mutations::native_preparation_child::{Puzzle2dCloseAxis,close_child_demand,close_demand_methods};
-use super::{Puzzle2dFlagIntent,Puzzle2dFlagPlan,Puzzle2dFlagPreparationCursor,Puzzle2dFlagPreparationStep,Puzzle2dSnapshot};
+use crate::standards::v1::subsets::any::schema::mutations::native_preparation_child::Puzzle2dCloseAxis;
+
+use crate::standards::v1::subsets::any::schema::mutations::native_preparation_child::close_child_demand;
+
+use crate::standards::v1::subsets::any::schema::mutations::native_preparation_child::close_demand_methods;
+use super::Puzzle2dFlagIntent;
+use super::Puzzle2dFlagPlan;
+use super::Puzzle2dFlagPreparationCursor;
+use super::Puzzle2dFlagPreparationStep;
+use super::Puzzle2dSnapshot;
@@ -36 +44 @@ impl<T:Puzzle2dFlagIntent> Puzzle2dFlagInverseCursor<T> {
-    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, T>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
+    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, T>, body:usize,grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
@@ -57 +65 @@ impl<T:Puzzle2dFlagIntent> Puzzle2dFlagInverseCursor<T> {
-                let step = self.identifier.advance(source.project(1, |snapshot| T::identifier_at(snapshot,index)), grant)?;
+                let step = self.identifier.advance(source.project(1, |snapshot| T::identifier_at(snapshot,index)),body, grant)?;

```
## `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚑️flag/🎮️prepare/↩️inverse/🧪️tests/🦀️.rs`

```diff
diff --git a/d602d10af811acf04c3f880346ad8816b8743647 b/987a8bab13d0ad791d0a93366ad9aa8558c853d0
index d602d10af81..987a8bab13d 100644
--- a/d602d10af811acf04c3f880346ad8816b8743647
+++ b/987a8bab13d0ad791d0a93366ad9aa8558c853d0
@@ -42 +42 @@ fn retire(inverse: PagedList<Puzzle2dMutation, {usize::MAX}>) {
-fn law<T:Puzzle2dFlagIntent+Clone>(snapshot:&Puzzle2dSnapshot,payload:T,expected:&serde_json::Value,large:bool){
+fn law<T: 'static + Puzzle2dFlagIntent+Clone>(snapshot:&Puzzle2dSnapshot,payload:T,expected:&serde_json::Value,large:bool){
@@ -52 +52,4 @@ fn history_edit_puzzle2d_owned_optional_flag_inverse_preserves_first_native_owne
- use super::super::{ChangeNodeLocked,ChangeNodeVisible,ChangeEdgeLocked,ChangeEdgeVisible};
+ use super::super::ChangeNodeLocked;
+ use super::super::ChangeNodeVisible;
+ use super::super::ChangeEdgeLocked;
+ use super::super::ChangeEdgeVisible;
@@ -67 +70,2 @@ fn history_edit_puzzle2d_owned_required_region_flag_inverse_preserves_literal_bo
- use crate::standards::v1::subsets::any::schema::mutations::{ChangeTargetRegionHidden,ChangeTargetRegionLocked};
+ use crate::standards::v1::subsets::any::schema::mutations::ChangeTargetRegionHidden;
+ use crate::standards::v1::subsets::any::schema::mutations::ChangeTargetRegionLocked;
@@ -71,0 +76,31 @@ fn history_edit_puzzle2d_owned_required_region_flag_inverse_preserves_literal_bo
+
+/// ➕️ The concrete inverse this owner prepares sums to the negative of the forward diff (law L3) on the committed applied vectors of its family.
+#[semio_framework_async_macros::async_test]
+async fn the_prepared_family_inverse_sums_to_the_negative_diff() {
+    macro_rules! vector {
+        ($kind:literal, $case:literal) => {
+            (
+                include_str!(concat!("../../../../../../🧫️fixtures/🧬️mutations/", $kind, "/", $case, "/📸️snapshot/⬅️before/🔣️.json")),
+                include_str!(concat!("../../../../../../🧫️fixtures/🧬️mutations/", $kind, "/", $case, "/🦠️mutation/🔣️.json")),
+            )
+        };
+    }
+    for (before, mutation) in [
+    vector!("👁️change-node-visible", "🙈️hides-a-capsule"),
+    vector!("👁️change-node-visible", "🙈️hides-node-a"),
+    vector!("🔒change-node-locked", "🔒️locks-node-a"),
+    vector!("🔒change-node-locked", "🔒️locks"),
+    vector!("👀change-edge-visible", "🙈️hides-edge-1"),
+    vector!("👀change-edge-visible", "🙈️hides"),
+    vector!("🔐change-edge-locked", "🔒️locks-edge-1"),
+    vector!("🔐change-edge-locked", "🔒️locks"),
+    vector!("🙈change-target-region-hidden", "🙈️hides"),
+    vector!("🔏change-target-region-locked", "🔏️locks"),
+    vector!("🌟change-node-root", "🌳️promotes-node-a-to-root"),
+    vector!("🌟change-node-root", "🌳️promotes"),
+    ] {
+        let before: crate::Puzzle2dSnapshot = semio_framework_pack_json::from_json_str(before, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes");
+        let mutation: crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation = semio_framework_pack_json::from_json_str(mutation, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes");
+        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
+    }
+}

```
## `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚑️flag/🎮️prepare/📸️candidate/🦀️.rs`

```diff
diff --git a/39cedd6e5827bcef32b161bab532a769ce968107 b/f3e267b0400b2eaac9962d8cc0518c6a3bd69a56
index 39cedd6e582..f3e267b0400 100644
--- a/39cedd6e5827bcef32b161bab532a769ce968107
+++ b/f3e267b0400b2eaac9962d8cc0518c6a3bd69a56
@@ -3,2 +3,11 @@
-use crate::standards::v1::subsets::any::schema::mutations::native_preparation_child::{Puzzle2dCloseAxis,close_child_demand,close_demand_methods};
-use super::{Puzzle2dFlagIntent,Puzzle2dFlagDisposition,Puzzle2dFlagPlan,Puzzle2dFlagPreparationCursor,Puzzle2dFlagPreparationStep,Puzzle2dSnapshot};
+use crate::standards::v1::subsets::any::schema::mutations::native_preparation_child::Puzzle2dCloseAxis;
+
+use crate::standards::v1::subsets::any::schema::mutations::native_preparation_child::close_child_demand;
+
+use crate::standards::v1::subsets::any::schema::mutations::native_preparation_child::close_demand_methods;
+use super::Puzzle2dFlagIntent;
+use super::Puzzle2dFlagDisposition;
+use super::Puzzle2dFlagPlan;
+use super::Puzzle2dFlagPreparationCursor;
+use super::Puzzle2dFlagPreparationStep;
+use super::Puzzle2dSnapshot;
@@ -33 +42 @@ impl<T:Puzzle2dFlagIntent> Puzzle2dFlagCandidateCursor<T> {
-    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, T>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
+    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, T>, body:usize,grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
@@ -50 +59 @@ impl<T:Puzzle2dFlagIntent> Puzzle2dFlagCandidateCursor<T> {
-                let step = self.snapshot_clone.advance(source, grant)?;
+                let step = self.snapshot_clone.advance(source,body, grant)?;

```
## `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚑️flag/🎮️prepare/📸️candidate/🧪️tests/🦀️.rs`

```diff
diff --git a/988bc85c76ec54c9e2227d16a0dcfafabbc89f17 b/43967b71547ae2f485ca6b754b2f1e46b419a14f
index 988bc85c76e..43967b71547 100644
--- a/988bc85c76ec54c9e2227d16a0dcfafabbc89f17
+++ b/43967b71547ae2f485ca6b754b2f1e46b419a14f
@@ -38 +38 @@ fn retire(snapshot: Puzzle2dSnapshot) {
-fn law<T:Puzzle2dFlagIntent+Clone>(snapshot:&Puzzle2dSnapshot,payload:T,row:&serde_json::Value,expected:Option<&Puzzle2dSnapshot>){
+fn law<T: 'static + Puzzle2dFlagIntent+Clone>(snapshot:&Puzzle2dSnapshot,payload:T,row:&serde_json::Value,expected:Option<&Puzzle2dSnapshot>){
@@ -47 +47,4 @@ fn history_edit_puzzle2d_native_optional_flag_candidate_preserves_full_native_ow
- use super::super::{ChangeNodeLocked,ChangeNodeVisible,ChangeEdgeLocked,ChangeEdgeVisible};
+ use super::super::ChangeNodeLocked;
+ use super::super::ChangeNodeVisible;
+ use super::super::ChangeEdgeLocked;
+ use super::super::ChangeEdgeVisible;
@@ -62 +65,2 @@ fn history_edit_puzzle2d_native_required_region_flag_candidate_preserves_full_ow
- use crate::standards::v1::subsets::any::schema::mutations::{ChangeTargetRegionHidden,ChangeTargetRegionLocked};
+ use crate::standards::v1::subsets::any::schema::mutations::ChangeTargetRegionHidden;
+ use crate::standards::v1::subsets::any::schema::mutations::ChangeTargetRegionLocked;

```
## `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚑️flag/🎮️prepare/🦀️.rs`

```diff
diff --git a/9d0389441b4701dd7a661632e2051d39d5a7fa64 b/2e18d3de87fdfc0f438b5d2fcf7ea45f378ed424
index 9d0389441b4..2e18d3de87f 100644
--- a/9d0389441b4701dd7a661632e2051d39d5a7fa64
+++ b/2e18d3de87fdfc0f438b5d2fcf7ea45f378ed424
@@ -2 +2,8 @@
-use super::{ChangeNodeLocked,ChangeNodeVisible,ChangeEdgeLocked,ChangeEdgeVisible,ChangeNodeRoot,ChangeTargetRegionHidden,ChangeTargetRegionLocked,Puzzle2dMutation};
+use super::ChangeNodeLocked;
+use super::ChangeNodeVisible;
+use super::ChangeEdgeLocked;
+use super::ChangeEdgeVisible;
+use super::ChangeNodeRoot;
+use super::ChangeTargetRegionHidden;
+use super::ChangeTargetRegionLocked;
+use super::Puzzle2dMutation;

```
## `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚑️flag/🎮️prepare/🧪️tests/🦀️.rs`

```diff
diff --git a/ab9ff47270a6aa078355648beceda1c25df9a9a4 b/3d8c699f9be80594a6cbc291054f999fedcc5c14
index ab9ff47270a..3d8c699f9be 100644
--- a/ab9ff47270a6aa078355648beceda1c25df9a9a4
+++ b/3d8c699f9be80594a6cbc291054f999fedcc5c14
@@ -5 +5 @@ fn close<T:Puzzle2dFlagIntent>(cursor:&mut Puzzle2dFlagPreparationCursor<T>){cur
-fn law<T:Puzzle2dFlagIntent+Clone>(snapshot:&Puzzle2dSnapshot,payload:T,row:&serde_json::Value){
+fn law<T: 'static + Puzzle2dFlagIntent+Clone>(snapshot:&Puzzle2dSnapshot,payload:T,row:&serde_json::Value){

```
## `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📤️retire-local-document/🟦️.ts`

```diff
diff --git a/d62cc0927d1be084e135b974538b122da88d4eb1 b/1a0c1ad60cdfb42bd6722f8952a4bb2c91504bcd
index d62cc0927d1..1a0c1ad60cd 100644
--- a/d62cc0927d1be084e135b974538b122da88d4eb1
+++ b/1a0c1ad60cdfb42bd6722f8952a4bb2c91504bcd
@@ -4 +4 @@ import type { LocalCatalogConfigMutation } from "../🟦️.ts";
-import type { LocalCatalog } from "../📥️admit-local-document/🟦️.ts";
+import type { LocalCatalog, LocalCatalogDiff } from "../📥️admit-local-document/🟦️.ts";
@@ -18,4 +18,4 @@ export function retireLocalDocument(documentId: string): LocalCatalogConfigMutat
-/** 🔺️ Drops the entry for the payload's id; an unlisted id leaves the catalog as it was. */
-export function diff(payload: RetireLocalDocument, base: LocalCatalog): LocalCatalog {
-  if (!base.documents.some((entry) => entry.documentId === payload.documentId)) return base;
-  return { documents: base.documents.filter((entry) => entry.documentId !== payload.documentId) };
+/** 🔺️ Names the one row it removes: the entry for the payload's id; an unlisted id changes nothing. */
+export function diff(payload: RetireLocalDocument, base: LocalCatalog): LocalCatalogDiff {
+  if (!base.documents.some((entry) => entry.documentId === payload.documentId)) return {};
+  return { documents: [{ key: payload.documentId }] };

```
## `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📥️admit-local-document/🟦️.ts`

```diff
diff --git a/f4cd7dc6f19de61479c1769d2e1d28ea65bac2d5 b/711ccb57bd43a3d8d93f3bebc2d53473058625a3
index f4cd7dc6f19..711ccb57bd4 100644
--- a/f4cd7dc6f19de61479c1769d2e1d28ea65bac2d5
+++ b/711ccb57bd43a3d8d93f3bebc2d53473058625a3
@@ -26,0 +27,13 @@ export const LOCAL_CATALOG_CONFIG_SCHEMA = "os.config.local-catalog";
+
+/** 🔑️ The absolute new content of one catalog row; no `value` removes the row. Twin of Rust `KeyedEdit<LocalDocument>`. */
+export interface LocalCatalogRow {
+  readonly key: string;
+  readonly value?: LocalDocument;
+}
+
+/** 🔺️ Sparse diff of a {@link LocalCatalog}: one absolute row per touched document id. Twin of Rust `LocalCatalogDiff`; the
+ * committed `🔺️diff` fixtures are its language-agnostic specification. Only the central twin applier
+ * (`../⚙️apply-diff/🟦️.ts`) turns it into a catalog. */
+export interface LocalCatalogDiff {
+  readonly documents?: readonly LocalCatalogRow[];
+}
@@ -38,2 +51,2 @@ export function admitLocalDocument(document: LocalDocument): LocalCatalogConfigM
-/** 🔺️ Replaces or inserts the entry for the payload's id, keeping the catalog ordered by id. */
-export function diff(payload: AdmitLocalDocument, base: LocalCatalog): LocalCatalog {
+/** 🔺️ Names the one row it sets: the entry for the payload's id. An entry that is listed exactly so already is no change. */
+export function diff(payload: AdmitLocalDocument, base: LocalCatalog): LocalCatalogDiff {
@@ -41,2 +54,2 @@ export function diff(payload: AdmitLocalDocument, base: LocalCatalog): LocalCata
-  if (base.documents.some((entry) => sameDocument(entry, admitted))) return base;
-  return { documents: [...base.documents.filter((entry) => entry.documentId !== admitted.documentId), admitted].sort((left, right) => (left.documentId < right.documentId ? -1 : left.documentId > right.documentId ? 1 : 0)) };
+  if (base.documents.some((entry) => sameDocument(entry, admitted))) return {};
+  return { documents: [{ key: admitted.documentId, value: admitted }] };

```
## `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚪️sign-out/🟦️.ts`

```diff
diff --git a/2e009bfdd31c2711c9f36c5bb40a038283507496 b/308f2a38a6e5a14f820aa544290b78dcc9be309c
index 2e009bfdd31..308f2a38a6e 100644
--- a/2e009bfdd31c2711c9f36c5bb40a038283507496
+++ b/308f2a38a6e5a14f820aa544290b78dcc9be309c
@@ -4 +4 @@ import type { IdentityConfigMutation } from "../🟦️.ts";
-import type { Identity } from "../🪪️sign-in/🟦️.ts";
+import type { Identity, IdentityDiff } from "../🪪️sign-in/🟦️.ts";
@@ -16,3 +16,3 @@ export function signOut(): IdentityConfigMutation {
-/** 🔺️ Constructs the whole-record signed-out state. */
-export function diff(_base: Identity | null): null {
-  return null;
+/** 🔺️ Names the one session it clears; signing out of a signed-out record changes nothing. */
+export function diff(base: Identity | null): IdentityDiff {
+  return base === null ? {} : { session: {} };

```
## `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🪪️sign-in/🟦️.ts`

```diff
diff --git a/8cb2b692ff310a95ba2638f395d66b942552f349 b/7142a6bd494b7dbbb1660206caf8f0fb58809fbd
index 8cb2b692ff3..7142a6bd494 100644
--- a/8cb2b692ff310a95ba2638f395d66b942552f349
+++ b/7142a6bd494b7dbbb1660206caf8f0fb58809fbd
@@ -17,0 +18,12 @@ export const IDENTITY_CONFIG_SCHEMA = "os.config.identity";
+
+/** 🎛️ The absolute new session; no `value` signs out. Twin of Rust `SettingEdit<Identity>`. */
+export interface IdentitySessionEdit {
+  readonly value?: Identity;
+}
+
+/** 🔺️ Sparse diff of the identity record: the absolute new session. Twin of Rust `IdentityDiff`; the committed `🔺️diff`
+ * fixtures are its language-agnostic specification. Only the central twin applier (`../⚙️apply-diff/🟦️.ts`) turns it into a
+ * record. */
+export interface IdentityDiff {
+  readonly session?: IdentitySessionEdit;
+}
@@ -29,9 +41,5 @@ export function signIn(payload: SignIn): IdentityConfigMutation {
-/** 🔺️ Constructs the whole-record post-sign-in identity. */
-export function diff(payload: SignIn, _base: Identity | null): Identity {
-  return {
-    userId: payload.userId,
-    email: payload.email,
-    displayName: payload.displayName,
-    hubBaseUrl: payload.hubBaseUrl,
-    issuedAtMs: payload.issuedAtMs,
-  };
+/** 🔺️ Names the one session it sets. A session that is held exactly so already is no change. */
+export function diff(payload: SignIn, base: Identity | null): IdentityDiff {
+  const identity: Identity = { userId: payload.userId, email: payload.email, displayName: payload.displayName, hubBaseUrl: payload.hubBaseUrl, issuedAtMs: payload.issuedAtMs };
+  if (base !== null && base.userId === identity.userId && base.email === identity.email && base.displayName === identity.displayName && base.hubBaseUrl === identity.hubBaseUrl && base.issuedAtMs === identity.issuedAtMs) return {};
+  return { session: { value: identity } };

```
## `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🧪️testing/♻️retirement/🦀️.rs`

```diff
diff --git a/2a2ac2c95968972cc80215a31889b09578e768c9 b/c572f9058d222839cd30e0381b089668e62796dc
index 2a2ac2c9596..c572f9058d2 100644
--- a/2a2ac2c95968972cc80215a31889b09578e768c9
+++ b/c572f9058d222839cd30e0381b089668e62796dc
@@ -34,0 +35 @@ impl<P: CountedPresence> ErasedSnapshotRetirement for Retirement<P> {
+    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { self.demands(body) }

```
## `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`

```diff
diff --git a/eaa2009ecba6c68eccbe1208b5bad2b1d9e5b1d5 b/59573243ad10273f924ba167e28d6731f52b0920
index eaa2009ecba..59573243ad1 100644
--- a/eaa2009ecba6c68eccbe1208b5bad2b1d9e5b1d5
+++ b/59573243ad10273f924ba167e28d6731f52b0920
@@ -1,0 +2 @@ use pack::snapshot::ArtifactSqliteSnapshot as _;
+use protocol::{ColdMutation,ColdMutationDiff,ColdDiffAlgebra};
@@ -3,0 +5,3 @@ use super::*;
+#[path = "../../🏗️initialization/🏠️owners/🧪️tests/🦀️.rs"]
+mod shared_semantic_source_tests;
+
@@ -45,0 +50 @@ async fn original_member_group_stage_and_adoption_preserve_physical_receipts_and
+ let diagnostic_rows:Vec<_>=fixture["diagnostics"].as_array().unwrap().iter().map(|row|crate::os_spr::MutationMessage{level:semio_framework_diagnostic::Severity::from_value(row["level"].clone().into()).unwrap(),code:row["code"].as_str().unwrap().to_owned().into(),message:row["message"].as_str().unwrap().to_owned(),target:row["target"].as_array().unwrap().iter().map(|target|target.as_str().unwrap().to_owned()).collect(),op_index:Some(row["opIndex"].as_u64().unwrap()as u32)}).collect();let diagnostic_pointers:Vec<_>=diagnostic_rows.iter().map(|row|row.message.as_ptr()).collect();let mut diagnostic_owner=crate::os_spr::command::MutationMessagePublication::new(diagnostic_rows);while !diagnostic_owner.ready(){diagnostic_owner.advance(grant.retained_grant()).unwrap();}stage.diagnostics=super::ArtifactStoreDiagnosticStage::from_publication(Some(diagnostic_owner));
@@ -59,0 +65 @@ async fn original_member_group_stage_and_adoption_preserve_physical_receipts_and
+ assert_eq!(store.messages_for_edit(id).iter().map(|row|row.message.as_ptr()).collect::<Vec<_>>(),diagnostic_pointers);assert_eq!(store.pending_report.messages.len(),1);assert_eq!(store.pending_report.messages[0].edit_id,id);assert_eq!(store.pending_report.messages[0].messages,store.messages_for_edit(id));assert_eq!(store.pending_report.worst,Some(semio_framework_diagnostic::Severity::Warning));for(original,report)in store.messages_for_edit(id).iter().zip(&store.pending_report.messages[0].messages){assert_ne!(original.message.as_ptr(),report.message.as_ptr());}eprintln!("[DEBUG] Original group diagnostic publication operation=919 generation={} edit={} rows={} worst=warning beforeACK=true",store.generation,id,store.messages_for_edit(id).len());
@@ -186,0 +193,7 @@ pub(super) fn assert_fixture_descriptor<T: crate::os_spr::MutationLeaf>(descript
+/// ➕️ L3 for one fixture leaf: the concrete inverse's diffs sum to the negative of the forward diff on every listed demo document (`None` is the deleted value).
+pub(crate) async fn assert_fixture_sum_law<Op: crate::os_spr::Mutation<DemoSnapshot> + crate::os_spr::ColdMutation<DemoSnapshot,ReferenceDiff=DemoDiff>>(mutation: &Op, bases: &[Option<i32>]) {
+    for n in bases {
+        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(mutation, &DemoSnapshot { n: *n }).await;
+    }
+}
+
@@ -193 +206 @@ where
-    Op: Mutation<DemoSnapshot, Diff = DemoDiff> + OpText + OpBinary + PartialEq + std::fmt::Debug,
+    Op: Mutation<DemoSnapshot, Diff = DemoDiff> + ColdMutation<DemoSnapshot,ReferenceDiff=DemoDiff> + ToValue + FromValue + OpText + OpBinary + PartialEq + std::fmt::Debug,
@@ -215 +228 @@ where
-    let actual = crate::os_spr::apply_diff(&diff, &before).unwrap();
+    let actual = protocol::apply_diff_cold(&diff, &before).unwrap();
@@ -217 +230 @@ where
-    assert_eq!(crate::os_spr::apply_diff(&decoded_diff, &before).unwrap(), actual);
+    assert_eq!(protocol::apply_diff_cold(&decoded_diff, &before).unwrap(), actual);
@@ -223 +236 @@ where
-        restored = crate::os_spr::apply_diff(inverse.diff(&restored).diff(), &restored).unwrap();
+        restored = protocol::apply_diff_cold(inverse.diff(&restored).diff(), &restored).unwrap();
@@ -239 +252 @@ where
-    Op: Mutation<DemoSnapshot, Diff = DemoDiff> + OpText + OpBinary + PartialEq + std::fmt::Debug,
+    Op: Mutation<DemoSnapshot, Diff = DemoDiff> + ColdMutation<DemoSnapshot,ReferenceDiff=DemoDiff> + ToValue + FromValue + OpText + OpBinary + PartialEq + std::fmt::Debug,
@@ -305 +318 @@ fn direct_store_fixture_absence_inverse_boundary() {
-    let deleted = crate::os_spr::apply_diff(mutation.diff(&before).diff(), &before).unwrap();
+    let deleted = protocol::apply_diff_cold(mutation.diff(&before).diff(), &before).unwrap();
@@ -307 +320 @@ fn direct_store_fixture_absence_inverse_boundary() {
-    assert_eq!(crate::os_spr::apply_diff(mutation.inverse(&before).expect("valid retained mutation inverse fixture")[0].diff(&deleted).diff(), &deleted).unwrap(), before);
+    assert_eq!(protocol::apply_diff_cold(mutation.inverse(&before).expect("valid retained mutation inverse fixture")[0].diff(&deleted).diff(), &deleted).unwrap(), before);
@@ -319 +332 @@ fn direct_store_fixture_structural_diff_composition() {
-            actual = crate::os_spr::apply_diff(&diff, &actual).unwrap();
+            actual = protocol::apply_diff_cold(&diff, &actual).unwrap();
@@ -324 +337 @@ fn direct_store_fixture_structural_diff_composition() {
-        assert_eq!(crate::os_spr::apply_diff(&combined, &before).unwrap(), expected);
+        assert_eq!(protocol::apply_diff_cold(&combined, &before).unwrap(), expected);
@@ -326 +339 @@ fn direct_store_fixture_structural_diff_composition() {
-        assert_eq!(crate::os_spr::apply_diff(&round_trip, &before).unwrap(), expected);
+        assert_eq!(protocol::apply_diff_cold(&round_trip, &before).unwrap(), expected);
@@ -328 +341 @@ fn direct_store_fixture_structural_diff_composition() {
-        assert_eq!(crate::os_spr::apply_diff(restore.diff(&actual).diff(), &actual).unwrap(), before);
+        assert_eq!(protocol::apply_diff_cold(restore.diff(&actual).diff(), &actual).unwrap(), before);
@@ -819,0 +833 @@ impl ErasedSnapshotRetirement for UnitOwnedRetirement {
+    fn next_demand(&self, body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(semio_framework_value::RetirementDemand { depth: usize::from(self.value.is_some()), ..Default::default() })}
@@ -1558 +1572 @@ where
-        let turn = RetainedCloneGrant { maximum_depth: demand.depth.max(grant.maximum_depth), ..grant };
+        let turn = RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes.max(grant.maximum_copy_bytes), maximum_capacity_bytes: demand.capacity_bytes.max(grant.maximum_capacity_bytes), maximum_release_bytes: demand.release_bytes.max(grant.maximum_release_bytes), maximum_depth: demand.depth.max(grant.maximum_depth), ..grant };
@@ -1630,3 +1644 @@ where
-    fn begin_one_item_wire_publication(&self, request: MemberStoreOneItemWireRequest) -> Result<Box<dyn ErasedMemberStoreOneItemPublication>, ArtifactStoreBatchAdmissionRejected<MemberStoreOneItemWire>> {
-        SpaceMember::begin_one_item_wire_publication(&self.0, request)
-    }
+
@@ -1637,2 +1649,2 @@ where
-    fn advance_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemAdvance, String> {
-        SpaceMember::advance_one_item_publication(&mut self.0, publication, grant)
+    fn advance_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, body:usize, grant: ArtifactStoreOneItemGrant, receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemAdvance, String> {
+        SpaceMember::advance_one_item_publication(&mut self.0, publication, body, grant,receiving)
@@ -1640,2 +1652,2 @@ where
-    fn prepare_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
-        SpaceMember::prepare_one_item_publication(&mut self.0, publication, grant)
+    fn prepare_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, body:usize, grant: ArtifactStoreOneItemGrant, receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemPreparationStep, String> {
+        SpaceMember::prepare_one_item_publication(&mut self.0, publication, body, grant,receiving)
@@ -1706,3 +1718 @@ where
-    async fn commit_checkpoint(&mut self, message: String, authors: Vec<Author>, identity: &mut crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
-        SpaceMember::commit_checkpoint(&mut self.0, message, authors, identity).await
-    }
+
@@ -1715,6 +1725,2 @@ where
-    async fn checkout(&mut self, checkpoint_id: &str, alternative_id: &str, identity: &mut crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
-        SpaceMember::checkout(&mut self.0, checkpoint_id, alternative_id, identity).await
-    }
-    async fn create_alternative(&mut self, name: String, identity: &mut crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
-        SpaceMember::create_alternative(&mut self.0, name, identity).await
-    }
+
+
@@ -1726,0 +1733,43 @@ where
+
+
+    async fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
+        self
+    }
+
+
+
+
+
+
+    async fn tail_group_id(&self) -> Option<String> {
+        SpaceMember::tail_group_id(&self.0).await
+    }
+    async fn tail_edit_id(&self) -> Option<String> {
+        SpaceMember::tail_edit_id(&self.0).await
+    }
+    async fn redo_tail(&self) -> Option<(String, Option<String>)> {
+        SpaceMember::redo_tail(&self.0).await
+    }
+
+
+
+
+
+
+}
+
+impl<P,Mutation> ReferenceSpaceMember for ArtifactStore<P,Mutation>
+where P:Clone+ToValue+FromValue+ArtifactPack+semio_framework_schema_composition::ArtifactCompositionFields+Send+Sync+'static,Mutation:Clone+ToValue+FromValue+super::Mutation<P>+crate::os_spr::ColdMutation<P>+OpBinary+OpText+Send+'static,
+{
+    fn begin_one_item_wire_publication(&self, request: MemberStoreOneItemWireRequest) -> Result<Box<dyn ErasedMemberStoreOneItemPublication>, ArtifactStoreBatchAdmissionRejected<MemberStoreOneItemWire>> {
+        ReferenceSpaceMember::begin_one_item_wire_publication(&self.0, request)
+    }
+    async fn commit_checkpoint(&mut self, message: String, authors: Vec<Author>, identity: &mut crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
+        ReferenceSpaceMember::commit_checkpoint(&mut self.0, message, authors, identity).await
+    }
+    async fn checkout(&mut self, checkpoint_id: &str, alternative_id: &str, identity: &mut crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
+        ReferenceSpaceMember::checkout(&mut self.0, checkpoint_id, alternative_id, identity).await
+    }
+    async fn create_alternative(&mut self, name: String, identity: &mut crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
+        ReferenceSpaceMember::create_alternative(&mut self.0, name, identity).await
+    }
@@ -1728 +1777 @@ where
-        SpaceMember::undo(&mut self.0, identity).await
+        ReferenceSpaceMember::undo(&mut self.0, identity).await
@@ -1731,4 +1780 @@ where
-        SpaceMember::redo(&mut self.0, identity).await
-    }
-    async fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
-        self
+        ReferenceSpaceMember::redo(&mut self.0, identity).await
@@ -1737 +1783 @@ where
-        SpaceMember::preview_wire(&self.0, ops).await
+        ReferenceSpaceMember::preview_wire(&self.0, ops).await
@@ -1740 +1786 @@ where
-        SpaceMember::dispatch_wire(&mut self.0, command, identity).await
+        ReferenceSpaceMember::dispatch_wire(&mut self.0, command, identity).await
@@ -1743 +1789 @@ where
-        SpaceMember::dispatch_wire_with_policy(&mut self.0, command, policy, identity).await
+        ReferenceSpaceMember::dispatch_wire_with_policy(&mut self.0, command, policy, identity).await
@@ -1746 +1792 @@ where
-        SpaceMember::event_log_payload(&self.0).await
+        ReferenceSpaceMember::event_log_payload(&self.0).await
@@ -1749 +1795 @@ where
-        SpaceMember::announce_tail_edit_payload(&mut self.0).await
+        ReferenceSpaceMember::announce_tail_edit_payload(&mut self.0).await
@@ -1752,10 +1798 @@ where
-        SpaceMember::ingest_remote_payload(&mut self.0, envelopes).await
-    }
-    async fn tail_group_id(&self) -> Option<String> {
-        SpaceMember::tail_group_id(&self.0).await
-    }
-    async fn tail_edit_id(&self) -> Option<String> {
-        SpaceMember::tail_edit_id(&self.0).await
-    }
-    async fn redo_tail(&self) -> Option<(String, Option<String>)> {
-        SpaceMember::redo_tail(&self.0).await
+        ReferenceSpaceMember::ingest_remote_payload(&mut self.0, envelopes).await
@@ -1764 +1801 @@ where
-        SpaceMember::stamp_tail_group_id(&mut self.0, group_id).await
+        ReferenceSpaceMember::stamp_tail_group_id(&mut self.0, group_id).await
@@ -1767 +1804 @@ where
-        SpaceMember::stamp_tail_origin(&mut self.0, origin).await
+        ReferenceSpaceMember::stamp_tail_origin(&mut self.0, origin).await
@@ -1770 +1807 @@ where
-        SpaceMember::set_owner(&mut self.0, owner).await;
+        ReferenceSpaceMember::set_owner(&mut self.0, owner).await;
@@ -1773 +1810 @@ where
-        SpaceMember::document_pack_bytes(&self.0).await
+        ReferenceSpaceMember::document_pack_bytes(&self.0).await
@@ -1776 +1813 @@ where
-        SpaceMember::envelope_pack_bytes(&self.0).await
+        ReferenceSpaceMember::envelope_pack_bytes(&self.0).await
@@ -1779 +1816 @@ where
-        SpaceMember::pack_at_checkpoint(&self.0, checkpoint_id).await
+        ReferenceSpaceMember::pack_at_checkpoint(&self.0, checkpoint_id).await
@@ -1823 +1860 @@ fixture_member_factory!(SeverityMutation);
-#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue, crate::os_dsl::DslArtifact, semio_framework_value_derive::RetainedClone, semio_framework_value_derive::RetireOwned)]
+#[derive(semio_framework_dsl_record_derive::DslRecord, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue, crate::os_dsl::DslArtifact, semio_framework_value_derive::RetainedClone, semio_framework_value_derive::RetireOwned)]
@@ -1828,0 +1866,6 @@ pub(crate) struct DemoSnapshot {
+std::thread_local! { static DEMO_PROJECTION_CLONES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }
+
+impl Clone for DemoSnapshot {
+    fn clone(&self) -> Self { DEMO_PROJECTION_CLONES.with(|count| count.set(count.get() + 1)); Self { n: self.n } }
+}
+
@@ -1972,0 +2016 @@ impl ErasedSnapshotRetirement for ExactDemoSnapshotRetirement {
+    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { self.demands(body) }
@@ -2078 +2122 @@ fn drive_retirement_terminal(retirement: Box<dyn ErasedSnapshotRetirement>) {
-        let turn = RetainedCloneGrant { maximum_depth: demand.depth.max(grant.maximum_depth), ..grant };
+        let turn = RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes.max(grant.maximum_copy_bytes), maximum_capacity_bytes: demand.capacity_bytes.max(grant.maximum_capacity_bytes), maximum_release_bytes: demand.release_bytes.max(grant.maximum_release_bytes), maximum_depth: demand.depth.max(grant.maximum_depth), ..grant };
@@ -2146,0 +2191,2 @@ impl<M: ArtifactCanonicalJson + ArtifactCanonicalJsonTree + semio_framework_valu
+    fn begin_copy_byte_demand(&self,input:&M,lane:HistoryLane)->Result<usize,semio_framework_value::ValueError>{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "fixture authoring catalog prepares no retained gesture"))}
+
@@ -2152,0 +2199,28 @@ impl<M: ArtifactCanonicalJson + ArtifactCanonicalJsonTree + semio_framework_valu
+/// 🏭️ The authoring catalog of the demo document: it names each original mutation's canonical wire and originates the batch digest the store's batched publication seals, and prepares no retained gesture of its own (the publication's explicit domain factory does).
+#[derive(semio_framework_value::FactoryPayloadRetirement)]
+struct DemoAuthoringFactory;
+
+impl ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation> for DemoAuthoringFactory {
+    fn operation_wire_source<'a>(&self, mutation: &'a DemoMutation) -> Option<ArtifactPreparedOperationSource<'a>> {
+        fixture_wire_source(mutation)
+    }
+
+    fn preflight(&self, _mutation: &DemoMutation, _lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
+        Err("fixture authoring catalog prepares no retained gesture".into())
+    }
+
+    fn begin_batch_digest(&self, edit: &mut Option<Box<Edit<DemoMutation>>>, grant: RetainedCloneGrant) -> Result<Option<(Box<dyn ArtifactStoreBatchDigest<DemoMutation>>, RetainedCloneProgress)>, ValueError> {
+        admit_artifact_batch_digest(edit, grant)
+    }
+
+    fn begin_demand(&self, _mutation: &DemoMutation, _lane: HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, ValueError> {
+        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "fixture authoring catalog prepares no retained gesture"))
+    }
+    fn begin_copy_byte_demand(&self,input:&DemoMutation,lane:HistoryLane)->Result<usize,semio_framework_value::ValueError>{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "fixture authoring catalog prepares no retained gesture"))}
+
+
+    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<DemoSnapshot, DemoMutation, DemoMutation>, _grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation>>, RetainedCloneProgress), (ValueError, ArtifactStoreOneItemPreparationRequest<DemoSnapshot, DemoMutation, DemoMutation>)> {
+        Err((ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "fixture authoring catalog prepares no retained gesture"), request))
+    }
+}
+
@@ -2175,2 +2249,2 @@ where
-    let preparation = semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: semio_framework_value::factory_arc_birth_bytes::<FixtureAuthoringFactory<DemoMutation>>(), depth: 1 };
-    complete_test_catalog(DocumentStoreOwners::admit_source_constructor_with_one_item_preparation(grant, preparation, || (snapshots, initial, mutations, ArtifactStoreCursorDisposer::<DemoSnapshot, DemoMutation>::new(), Arc::new(FixtureAuthoringFactory::<DemoMutation>(std::marker::PhantomData)) as Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>>)))
+    let preparation = semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: semio_framework_value::factory_arc_birth_bytes::<DemoAuthoringFactory>(), depth: 1 };
+    complete_test_catalog(DocumentStoreOwners::admit_source_constructor_with_one_item_preparation(grant, preparation, || (snapshots, initial, mutations, ArtifactStoreCursorDisposer::<DemoSnapshot, DemoMutation>::new(), Arc::new(DemoAuthoringFactory) as Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>>)))
@@ -2319,3 +2393 @@ impl crate::os_spr::DiffAlgebra<DemoSnapshot> for DemoDiff {
-    fn inverse(&self, base: &DemoSnapshot) -> Self {
-        Self { n: self.n.as_ref().map(|_| DemoFieldChange { value: base.n }) }
-    }
+
@@ -2327,3 +2399,3 @@ impl crate::os_spr::DiffAlgebra<DemoSnapshot> for DemoDiff {
-impl MutationDiff<DemoSnapshot> for DemoDiff {
-    fn apply(&self, snapshot: &DemoSnapshot, _capability: crate::os_spr::ApplyCapability) -> crate::os_spr::MutationApplyResult<DemoSnapshot> {
-        Ok(DemoSnapshot { n: self.n.as_ref().map_or(snapshot.n, |change| change.value) })
+impl crate::os_spr::ColdDiffAlgebra<DemoSnapshot> for DemoDiff {
+    fn inverse(&self, base: &DemoSnapshot) -> Self {
+        Self { n: self.n.as_ref().map(|_| DemoFieldChange { value: base.n }) }
@@ -2330,0 +2403,4 @@ impl MutationDiff<DemoSnapshot> for DemoDiff {
+}
+
+impl MutationDiff<DemoSnapshot> for DemoDiff {
+
@@ -2331,0 +2408,4 @@ impl MutationDiff<DemoSnapshot> for DemoDiff {
+
+}
+
+impl ColdMutationDiff<DemoSnapshot> for DemoDiff {
@@ -2336,0 +2417,5 @@ impl MutationDiff<DemoSnapshot> for DemoDiff {
+
+    fn apply(&self, snapshot: &mut DemoSnapshot, _capability: crate::os_spr::ApplyCapability) -> crate::os_spr::MutationApplyResult<()> {
+                if let Some(change) = &self.n { snapshot.n = change.value; }
+        Ok(())
+    }
@@ -2343,3 +2428 @@ impl crate::os_spr::DiffAlgebra<DemoSnapshot> for LossyDiff {
-    fn inverse(&self, _base: &DemoSnapshot) -> Self {
-        Self {}
-    }
+
@@ -2351,3 +2434,3 @@ impl crate::os_spr::DiffAlgebra<DemoSnapshot> for LossyDiff {
-impl MutationDiff<DemoSnapshot> for LossyDiff {
-    fn apply(&self, snapshot: &DemoSnapshot, _capability: crate::os_spr::ApplyCapability) -> crate::os_spr::MutationApplyResult<DemoSnapshot> {
-        Ok(snapshot.clone())
+impl crate::os_spr::ColdDiffAlgebra<DemoSnapshot> for LossyDiff {
+    fn inverse(&self, _base: &DemoSnapshot) -> Self {
+        Self {}
@@ -2354,0 +2438,8 @@ impl MutationDiff<DemoSnapshot> for LossyDiff {
+}
+
+impl MutationDiff<DemoSnapshot> for LossyDiff {
+
+
+}
+
+impl ColdMutationDiff<DemoSnapshot> for LossyDiff {
@@ -2355,0 +2447,4 @@ impl MutationDiff<DemoSnapshot> for LossyDiff {
+
+    fn apply(&self, _unpublished: &mut DemoSnapshot, _capability: crate::os_spr::ApplyCapability) -> crate::os_spr::MutationApplyResult<()> {
+        Ok(())
+    }
@@ -2649,0 +2745,2 @@ impl ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation> for Coun
+    fn begin_copy_byte_demand(&self,input:&DemoMutation,lane:HistoryLane)->Result<usize,semio_framework_value::ValueError>{self.inner.begin_copy_byte_demand(input, lane)}
+
@@ -2663,0 +2761 @@ async fn member_owned_batch_admission_defers_whole_source_census_and_retains_phy
+    let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());
@@ -2681 +2779 @@ async fn member_owned_batch_admission_defers_whole_source_census_and_retains_phy
-        assert!(matches!(member.prepare_one_item_publication(&mut *publication, ArtifactStoreOneItemGrant { maximum_items: 0, maximum_copy_bytes: 4096, ..one_item_test_policy() }).unwrap(), ArtifactStoreOneItemPreparationStep::Blocked));
+        assert!(matches!(receiving_fixture.receive(|receiving|member.prepare_one_item_publication(&mut *publication, 4096, ArtifactStoreOneItemGrant { maximum_items: 0, maximum_copy_bytes: 4096, ..one_item_test_policy() },receiving)).unwrap(), ArtifactStoreOneItemPreparationStep::Blocked));
@@ -2685 +2783 @@ async fn member_owned_batch_admission_defers_whole_source_census_and_retains_phy
-            assert!(matches!(member.prepare_one_item_publication(&mut *publication, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4096, ..one_item_test_policy() }).unwrap(), ArtifactStoreOneItemPreparationStep::Progress(_, _)));
+            assert!(matches!(receiving_fixture.receive(|receiving|member.prepare_one_item_publication(&mut *publication, 4096, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4096, ..one_item_test_policy() },receiving)).unwrap(), ArtifactStoreOneItemPreparationStep::Progress(_, _)));
@@ -2690 +2788 @@ async fn member_owned_batch_admission_defers_whole_source_census_and_retains_phy
-        assert!(matches!(member.prepare_one_item_publication(&mut *publication, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4096, ..one_item_test_policy() }).unwrap(), ArtifactStoreOneItemPreparationStep::Progress(_, _)));
+        assert!(matches!(receiving_fixture.receive(|receiving|member.prepare_one_item_publication(&mut *publication, 4096, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4096, ..one_item_test_policy() },receiving)).unwrap(), ArtifactStoreOneItemPreparationStep::Progress(_, _)));
@@ -2728,0 +2827 @@ fn member_open_partial_parse_and_initialization_owners_retire_exactly() {
+    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { self.inner.next_demand(body) }
@@ -2915,0 +3015,2 @@ impl MemberStoreOneItemWirePreparationFactory<DemoSnapshot, DemoMutation> for De
+    fn begin_copy_byte_demand(&self,input:&MemberStoreOneItemWire,lane:HistoryLane)->Result<usize,semio_framework_value::ValueError>{self.begin_demand(input,lane)?; std::mem::size_of::<ArtifactStoreOneItemPreparationRequest<DemoSnapshot, MemberStoreOneItemWire, DemoMutation>>().checked_add(std::mem::size_of::<DemoMemberWirePreparation>()).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original preparation header quote overflow"))}
+
@@ -3001 +3102 @@ impl ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation> for DemoMemberW
-    fn advance(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
+    fn advance(&mut self, body:usize,grant: ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
@@ -3005 +3106 @@ impl ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation> for DemoMemberW
-            return match inner.advance(ArtifactStoreOneItemGrant { maximum_depth: grant.maximum_depth - 1, ..grant })? {
+            return match inner.advance(body,ArtifactStoreOneItemGrant { maximum_depth: grant.maximum_depth - 1, ..grant },receiving)? {
@@ -3106,0 +3208 @@ async fn retained_member_publication_preserves_order_group_identity_and_exact_ma
+    let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());
@@ -3135 +3237 @@ async fn retained_member_publication_preserves_order_group_identity_and_exact_ma
-        assert!(matches!(member.advance_one_item_publication(&mut *publication, ArtifactStoreOneItemGrant { maximum_items: 0, maximum_copy_bytes: 4096, ..one_item_test_policy() }), Ok(ArtifactStoreOneItemAdvance::Blocked)));
+        assert!(matches!(receiving_fixture.receive(|receiving|member.advance_one_item_publication(&mut *publication, fixture["maximumBytes"].as_u64().unwrap()as usize, ArtifactStoreOneItemGrant { maximum_items: 0, maximum_copy_bytes: 4096, ..one_item_test_policy() },receiving)), Ok(ArtifactStoreOneItemAdvance::Blocked)));
@@ -3142,2 +3244,2 @@ async fn retained_member_publication_preserves_order_group_identity_and_exact_ma
-            match member.advance_one_item_publication(&mut *publication, grant).expect("bounded retained member unit") {
-                ArtifactStoreOneItemAdvance::Published(receipt) => {
+            match receiving_fixture.receive(|receiving|member.advance_one_item_publication(&mut *publication, fixture["maximumBytes"].as_u64().unwrap()as usize, grant,receiving)).expect("bounded retained member unit") {
+                ArtifactStoreOneItemAdvance::Published(receipt, _) => {
@@ -3154 +3256 @@ async fn retained_member_publication_preserves_order_group_identity_and_exact_ma
-        assert!(matches!(member.advance_one_item_publication(&mut *publication, grant), Ok(ArtifactStoreOneItemAdvance::AwaitingAck(_))));
+        assert!(matches!(receiving_fixture.receive(|receiving|member.advance_one_item_publication(&mut *publication, fixture["maximumBytes"].as_u64().unwrap()as usize, grant,receiving)), Ok(ArtifactStoreOneItemAdvance::AwaitingAck(_))));
@@ -3178,0 +3281 @@ async fn retained_member_publication_rejects_wrong_owner_staleness_and_cancels_w
+    let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());
@@ -3199 +3302 @@ async fn retained_member_publication_rejects_wrong_owner_staleness_and_cancels_w
-    assert!(wrong.advance_one_item_publication(&mut *stale, grant).is_err(), "matching schema/id/revision cannot impersonate the exact store owner");
+    assert!(receiving_fixture.receive(|receiving|wrong.advance_one_item_publication(&mut *stale, 4096, grant,receiving)).is_err(), "matching schema/id/revision cannot impersonate the exact store owner");
@@ -3203 +3306 @@ async fn retained_member_publication_rejects_wrong_owner_staleness_and_cancels_w
-        if matches!(member.advance_apply_batch(&mut replacement, grant).unwrap(), ArtifactStoreOneItemAdvance::Published(_)) {
+        if matches!(receiving_fixture.receive(|receiving|member.advance_apply_batch(&mut replacement, 4096, grant,receiving)).unwrap(), ArtifactStoreOneItemAdvance::Published(_, _)) {
@@ -3209 +3312 @@ async fn retained_member_publication_rejects_wrong_owner_staleness_and_cancels_w
-    assert!(member.advance_one_item_publication(&mut *stale, grant).is_err());
+    assert!(receiving_fixture.receive(|receiving|member.advance_one_item_publication(&mut *stale, 4096, grant,receiving)).is_err());
@@ -3215 +3318 @@ async fn retained_member_publication_rejects_wrong_owner_staleness_and_cancels_w
-            assert!(!matches!(member.advance_one_item_publication(&mut *cancelled, grant).unwrap(), ArtifactStoreOneItemAdvance::Published(_)));
+            assert!(!matches!(receiving_fixture.receive(|receiving|member.advance_one_item_publication(&mut *cancelled, 4096, grant,receiving)).unwrap(), ArtifactStoreOneItemAdvance::Published(_, _)));
@@ -3227,0 +3331 @@ async fn retained_member_group_preparation_reserves_real_history_without_partial
+    let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());
@@ -3257 +3361 @@ async fn retained_member_group_preparation_reserves_real_history_without_partial
-            let step = members[index].prepare_one_item_publication(&mut *publications[index], grant).expect("one group preparation or reservation turn");
+            let step = receiving_fixture.receive(|receiving|members[index].prepare_one_item_publication(&mut *publications[index], 4096, grant,receiving)).expect("one group preparation or reservation turn");
@@ -3271 +3375 @@ async fn retained_member_group_preparation_reserves_real_history_without_partial
-        assert!(members[index].advance_one_item_publication(&mut *publications[index], grant).is_err(), "ordinary publication cannot consume a group-reserved candidate");
+        assert!(receiving_fixture.receive(|receiving|members[index].advance_one_item_publication(&mut *publications[index], 4096, grant,receiving)).is_err(), "ordinary publication cannot consume a group-reserved candidate");
@@ -3278 +3382 @@ async fn retained_member_group_preparation_reserves_real_history_without_partial
-        members[0].advance_apply_batch(&mut competing, grant).unwrap();
+        receiving_fixture.receive(|receiving|members[0].advance_apply_batch(&mut competing, 4096, grant,receiving)).unwrap();
@@ -3280 +3384 @@ async fn retained_member_group_preparation_reserves_real_history_without_partial
-    assert!(members[0].advance_apply_batch(&mut competing, grant).is_err(), "the real reserved edit slot excludes a competing append before any visible mutation");
+    assert!(receiving_fixture.receive(|receiving|members[0].advance_apply_batch(&mut competing, 4096, grant,receiving)).is_err(), "the real reserved edit slot excludes a competing append before any visible mutation");
@@ -3283 +3387 @@ async fn retained_member_group_preparation_reserves_real_history_without_partial
-    assert!(members[1].prepare_one_item_publication(&mut *publications[1], grant).is_err(), "freshness is revalidated even after reservation");
+    assert!(receiving_fixture.receive(|receiving|members[1].prepare_one_item_publication(&mut *publications[1], 4096, grant,receiving)).is_err(), "freshness is revalidated even after reservation");
@@ -3306,0 +3411 @@ async fn retained_member_common_root_copies_history_under_grant_and_publishes_th
+    let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());
@@ -3337 +3442 @@ async fn retained_member_common_root_copies_history_under_grant_and_publishes_th
-                    if matches!(members[lane].prepare_one_item_publication(&mut *publications[lane], grant).unwrap(), ArtifactStoreOneItemPreparationStep::Prepared(_, _)) { prepared = true; break; }
+                    if matches!(receiving_fixture.receive(|receiving|members[lane].prepare_one_item_publication(&mut *publications[lane], fixture["maximumBytes"].as_u64().unwrap()as usize, grant,receiving)).unwrap(), ArtifactStoreOneItemPreparationStep::Prepared(_, _)) { prepared = true; break; }
@@ -3771,0 +3877,2 @@ impl ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation> for Demo
+    fn begin_copy_byte_demand(&self,input:&DemoMutation,lane:HistoryLane)->Result<usize,semio_framework_value::ValueError>{self.begin_demand(input,lane)?; std::mem::size_of::<ArtifactStoreOneItemPreparationRequest<DemoSnapshot, DemoMutation, DemoMutation>>().checked_add(std::mem::size_of::<DemoOneItemPreparation>()).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original preparation header quote overflow"))}
+
@@ -3824 +3931 @@ impl ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation> for DemoOneItem
-    fn advance(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
+    fn advance(&mut self, body:usize,grant: ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
@@ -3836 +3943 @@ impl ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation> for DemoOneItem
-            let step=sealer.advance(grant)?;
+            let step=sealer.advance(grant,receiving)?;
@@ -3875 +3982 @@ impl ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation> for DemoOneItem
-        let post=match crate::os_spr::apply_diff(outcome.diff(),base.get()){Ok(post)=>post,Err(error)=>{self.mutation=Some(mutation);*self.semantic_refusal = Some(error);return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "original mutation application refused"));}};
+        let post=match protocol::apply_diff_cold(outcome.diff(),base.get()){Ok(post)=>post,Err(error)=>{self.mutation=Some(mutation);*self.semantic_refusal = Some(error);return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "original mutation application refused"));}};
@@ -4040 +4147 @@ impl ArtifactEphemeralOneItemPreparation<DemoSnapshot, DemoMutation> for DemoEph
-    fn advance(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
+    fn advance(&mut self, grant: ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
@@ -4058 +4165 @@ impl ArtifactEphemeralOneItemPreparation<DemoSnapshot, DemoMutation> for DemoEph
-        let original = match crate::os_spr::apply_diff(outcome.diff(), &request.base) { Ok(original) => original, Err(error) => { *self.semantic_refusal = Some(error); return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "original ephemeral mutation application refused")); } };
+        let original = match protocol::apply_diff_cold(outcome.diff(), &request.base) { Ok(original) => original, Err(error) => { *self.semantic_refusal = Some(error); return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "original ephemeral mutation application refused")); } };
@@ -4134,0 +4242 @@ async fn derived_child_reads_require_the_exact_store_frontier_and_genuine_operat
+    let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());
@@ -4147 +4255 @@ async fn derived_child_reads_require_the_exact_store_frontier_and_genuine_operat
-    let derived = store.derive_provisional_snapshot(&base, &operation).expect("genuine source").expect("applicable operation");
+    let derived = store.derive_reference_provisional_snapshot(&base, &operation).expect("genuine source").expect("applicable operation");
@@ -4176 +4284 @@ async fn derived_child_reads_require_the_exact_store_frontier_and_genuine_operat
-        if matches!(store.step_derived_report_replay(replay.as_mut().expect("retained replay"), grant, &mut || false).expect("own actual ledger"),ReplayStep::Finished(_)) {break;}
+        if matches!(receiving_fixture.receive(|receiving|store.step_derived_report_replay(replay.as_mut().expect("retained replay"), grant, receiving,&mut || false)).expect("own actual ledger"),ReplayStep::Finished(_)) {break;}
@@ -4363 +4471 @@ fn drive_test_report_replay<P, M>(replay: &mut EditReplay<P, M>, edits: &impl Re
-where M: super::Mutation<P> + OpBinary,
+where P: Clone, M: super::Mutation<P> + OpBinary,
@@ -4533 +4641,2 @@ impl RetainedCloneEditCursor<DemoSnapshot, DemoMutation> for DemoRetainedCloneEd
-    fn advance(&mut self, base: RetainedCloneRef<'_, DemoSnapshot>, post: &mut DemoSnapshot, mutation: RetainedCloneRef<'_, DemoMutation>, grant: RetainedCloneGrant) -> Result<RetainedCloneEditStep, ValueError> {
+    fn advance_demands(&self, _: RetainedCloneRef<'_, DemoSnapshot>, _: &DemoSnapshot, _: RetainedCloneRef<'_, DemoMutation>, _: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { Ok(semio_framework_value::RetirementDemand { copy_bytes: if self.phase >= 2 { std::mem::size_of::<DemoSnapshot>() } else { 0 }, capacity_bytes: if self.phase == 1 { std::mem::size_of::<DemoMutation>() } else { 0 }, depth: 1, ..Default::default() }) }
+    fn advance(&mut self, base: RetainedCloneRef<'_, DemoSnapshot>, post: &mut DemoSnapshot, mutation: RetainedCloneRef<'_, DemoMutation>, body:usize,grant: RetainedCloneGrant) -> Result<RetainedCloneEditStep, ValueError> {
@@ -4567 +4676 @@ impl RetainedCloneEditCursor<DemoSnapshot, DemoMutation> for DemoRetainedCloneEd
-        let original = match crate::os_spr::apply_diff(outcome.diff(), base.get()) { Ok(original) => original, Err(error) => { *self.semantic_refusal = Some(error); return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "original retained mutation application refused")); } };
+        let original = match protocol::apply_diff_cold(outcome.diff(), base.get()) { Ok(original) => original, Err(error) => { *self.semantic_refusal = Some(error); return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "original retained mutation application refused")); } };
@@ -4669,0 +4779,2 @@ impl ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation> for Prob
+    fn begin_copy_byte_demand(&self,input:&DemoMutation,lane:HistoryLane)->Result<usize,semio_framework_value::ValueError>{self.begin_demand(input,lane)?; self.inner.begin_copy_byte_demand(input,lane)?.checked_add(std::mem::size_of::<ProbedRetainedClonePreparation>()*2).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original wrapper header quote overflow"))}
+
@@ -4707 +4818 @@ impl ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation> for ProbedRetai
-    fn advance(&mut self,grant:ArtifactStoreOneItemGrant)->Result<ArtifactStoreOneItemPreparationStep,ValueError> {
+    fn advance(&mut self,body:usize,grant:ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<ArtifactStoreOneItemPreparationStep,ValueError> {
@@ -4709 +4820 @@ impl ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation> for ProbedRetai
-        self.inner.as_mut().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"probe no longer owns its typed preparation"))?.advance(ArtifactStoreOneItemGrant { maximum_depth:grant.maximum_depth-1,..grant })
+        self.inner.as_mut().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"probe no longer owns its typed preparation"))?.advance(body,ArtifactStoreOneItemGrant { maximum_depth:grant.maximum_depth-1,..grant },receiving)
@@ -4812,3 +4923 @@ impl crate::os_spr::DiffAlgebra<RetainedTextSnapshot> for RetainedTextDiff {
-    fn inverse(&self, base: &RetainedTextSnapshot) -> Self {
-        Self { text: self.text.as_ref().map(|_| base.text.clone()) }
-    }
+
@@ -4820,3 +4929,3 @@ impl crate::os_spr::DiffAlgebra<RetainedTextSnapshot> for RetainedTextDiff {
-impl MutationDiff<RetainedTextSnapshot> for RetainedTextDiff {
-    fn apply(&self, base: &RetainedTextSnapshot, _capability: crate::os_spr::ApplyCapability) -> crate::os_spr::MutationApplyResult<RetainedTextSnapshot> {
-        Ok(RetainedTextSnapshot { text: self.text.clone().unwrap_or_else(|| base.text.clone()) })
+impl crate::os_spr::ColdDiffAlgebra<RetainedTextSnapshot> for RetainedTextDiff {
+    fn inverse(&self, base: &RetainedTextSnapshot) -> Self {
+        Self { text: self.text.as_ref().map(|_| base.text.clone()) }
@@ -4823,0 +4933 @@ impl MutationDiff<RetainedTextSnapshot> for RetainedTextDiff {
+}
@@ -4824,0 +4935,7 @@ impl MutationDiff<RetainedTextSnapshot> for RetainedTextDiff {
+impl MutationDiff<RetainedTextSnapshot> for RetainedTextDiff {
+
+
+
+}
+
+impl ColdMutationDiff<RetainedTextSnapshot> for RetainedTextDiff {
@@ -4829,0 +4947,5 @@ impl MutationDiff<RetainedTextSnapshot> for RetainedTextDiff {
+
+    fn apply(&self, base: &mut RetainedTextSnapshot, _capability: crate::os_spr::ApplyCapability) -> crate::os_spr::MutationApplyResult<()> {
+                if let Some(value) = &self.text { base.text = value.clone(); }
+        Ok(())
+    }
@@ -4855,0 +4978,11 @@ impl crate::os_spr::Mutation<RetainedTextSnapshot> for RetainedTextMutation {
+
+
+
+
+    fn conflict_target(&self) -> Vec<String> {
+        vec!["text".into()]
+    }
+}
+
+impl crate::os_spr::ColdMutation<RetainedTextSnapshot> for RetainedTextMutation {
+    type ReferenceDiff = RetainedTextDiff;
@@ -4860 +4992,0 @@ impl crate::os_spr::Mutation<RetainedTextSnapshot> for RetainedTextMutation {
-
@@ -4867,4 +4998,0 @@ impl crate::os_spr::Mutation<RetainedTextSnapshot> for RetainedTextMutation {
-
-    fn conflict_target(&self) -> Vec<String> {
-        vec!["text".into()]
-    }
@@ -4927 +5055,2 @@ impl RetainedCloneEditCursor<RetainedTextSnapshot, RetainedTextMutation> for Ret
-    fn advance(&mut self, _base: RetainedCloneRef<'_, RetainedTextSnapshot>, _post: &mut RetainedTextSnapshot, _mutation: RetainedCloneRef<'_, RetainedTextMutation>, _grant: RetainedCloneGrant) -> Result<RetainedCloneEditStep, ValueError> {
+    fn advance_demands(&self, _: RetainedCloneRef<'_, RetainedTextSnapshot>, _: &RetainedTextSnapshot, _: RetainedCloneRef<'_, RetainedTextMutation>, _: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "retained text edit must not quote after contiguous-capacity refusal")) }
+    fn advance(&mut self, _base: RetainedCloneRef<'_, RetainedTextSnapshot>, _post: &mut RetainedTextSnapshot, _mutation: RetainedCloneRef<'_, RetainedTextMutation>, body:usize,_grant: RetainedCloneGrant) -> Result<RetainedCloneEditStep, ValueError> {
@@ -5027 +5156 @@ async fn retained_clone_preparation_store_lifecycle_matches_neutral_oracle() {
-                    match store.advance_apply_batch(&mut publication, grant) {
+                    match receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, fixture["grant"]["maximumCopyBytes"].as_u64().unwrap()as usize, grant,receiving)) {
@@ -5029 +5158 @@ async fn retained_clone_preparation_store_lifecycle_matches_neutral_oracle() {
-                        Ok(ArtifactStoreOneItemAdvance::Published(_)) => panic!("{} published a rejected gesture", row["id"]),
+                        Ok(ArtifactStoreOneItemAdvance::Published(_, _)) => panic!("{} published a rejected gesture", row["id"]),
@@ -5042,2 +5171,2 @@ async fn retained_clone_preparation_store_lifecycle_matches_neutral_oracle() {
-                match store.advance_apply_batch(&mut publication, grant) {
-                    Ok(ArtifactStoreOneItemAdvance::Published(_)) => {
+                match receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, fixture["grant"]["maximumCopyBytes"].as_u64().unwrap()as usize, grant,receiving)) {
+                    Ok(ArtifactStoreOneItemAdvance::Published(_, _)) => {
@@ -5063 +5192 @@ async fn retained_clone_preparation_store_lifecycle_matches_neutral_oracle() {
-                        let advance = store.advance_apply_batch(&mut publication, grant).expect("retained clone success turn");
+                        let advance = receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, fixture["grant"]["maximumCopyBytes"].as_u64().unwrap()as usize, grant,receiving)).expect("retained clone success turn");
@@ -5066 +5195 @@ async fn retained_clone_preparation_store_lifecycle_matches_neutral_oracle() {
-                            ArtifactStoreOneItemAdvance::Published(_) => {
+                            ArtifactStoreOneItemAdvance::Published(_, _) => {
@@ -5081 +5210 @@ async fn retained_clone_preparation_store_lifecycle_matches_neutral_oracle() {
-                    let error = match store.advance_apply_batch(&mut publication, grant) {
+                    let error = match receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, fixture["grant"]["maximumCopyBytes"].as_u64().unwrap()as usize, grant,receiving)) {
@@ -5090 +5219 @@ async fn retained_clone_preparation_store_lifecycle_matches_neutral_oracle() {
-                        match store.advance_apply_batch(&mut publication, grant) {
+                        match receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, fixture["grant"]["maximumCopyBytes"].as_u64().unwrap()as usize, grant,receiving)) {
@@ -5096 +5225 @@ async fn retained_clone_preparation_store_lifecycle_matches_neutral_oracle() {
-                            Ok(ArtifactStoreOneItemAdvance::Published(_)) => panic!("faulting edit published"),
+                            Ok(ArtifactStoreOneItemAdvance::Published(_, _)) => panic!("faulting edit published"),
@@ -5111 +5240 @@ async fn retained_clone_preparation_store_lifecycle_matches_neutral_oracle() {
-                        match store.advance_apply_batch(&mut publication, grant) {
+                        match receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, fixture["grant"]["maximumCopyBytes"].as_u64().unwrap()as usize, grant,receiving)) {
@@ -5116 +5245 @@ async fn retained_clone_preparation_store_lifecycle_matches_neutral_oracle() {
-                            Ok(ArtifactStoreOneItemAdvance::Published(_)) => panic!("over-budget retained edit published"),
+                            Ok(ArtifactStoreOneItemAdvance::Published(_, _)) => panic!("over-budget retained edit published"),
@@ -5178 +5307 @@ async fn interrupted_retained_clone_publication_handoffs_to_store_maintenance()
-        assert!(matches!(store.advance_apply_batch(driver, grant), Ok(ArtifactStoreOneItemAdvance::Progress(_) | ArtifactStoreOneItemAdvance::PreparationProgress(..))));
+        assert!(matches!(receiving_fixture.receive(|receiving|store.advance_apply_batch(driver, row["closeGrant"]["maximumCopyBytes"].as_u64().unwrap()as usize, grant,receiving)), Ok(ArtifactStoreOneItemAdvance::Progress(_) | ArtifactStoreOneItemAdvance::PreparationProgress(..))));
@@ -5239 +5368 @@ async fn retained_clone_preparation_refuses_contiguous_capacity_larger_than_one_
-        let advance = store.advance_apply_batch(&mut publication, grant);
+        let advance = receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, fixture["grant"]["maximumCopyBytes"].as_u64().unwrap()as usize, grant,receiving));
@@ -5296 +5425 @@ async fn artifact_store_one_item_single_retry_ack_and_move_only_root_preserve_ge
-        if let ArtifactStoreOneItemAdvance::Published(receipt) = store.advance_apply_batch(&mut publication, grant).expect("one bounded durable step") {
+        if let ArtifactStoreOneItemAdvance::Published(receipt, _) = receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, 4096, grant,receiving)).expect("one bounded durable step") {
@@ -5338,0 +5468 @@ async fn publish_demo_batch_with(
+    let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());
@@ -5345,2 +5475,2 @@ async fn publish_demo_batch_with(
-        match store.advance_apply_batch(&mut publication, grant) {
-            Ok(ArtifactStoreOneItemAdvance::Published(receipt)) => return (publication, Ok(receipt)),
+        match receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, 4096, grant,receiving)) {
+            Ok(ArtifactStoreOneItemAdvance::Published(receipt, _)) => return (publication, Ok(receipt)),
@@ -5420 +5550 @@ async fn artifact_store_batch_publication_stages_two_hundred_mutations_into_one_
-        backwards = crate::os_spr::apply_diff(operation.diff(&backwards).diff(), &backwards).expect("staged inverse applies tail-first");
+        backwards = protocol::apply_diff_cold(operation.diff(&backwards).diff(), &backwards).expect("staged inverse applies tail-first");
@@ -5425 +5555 @@ async fn artifact_store_batch_publication_stages_two_hundred_mutations_into_one_
-        forwards = crate::os_spr::apply_diff(operation.diff(&forwards).diff(), &forwards).expect("staged inverse applies");
+        forwards = protocol::apply_diff_cold(operation.diff(&forwards).diff(), &forwards).expect("staged inverse applies");
@@ -5585 +5715 @@ async fn artifact_store_batch_cancel_mid_flight_retires_every_staged_owner_witho
-        store.advance_apply_batch(&mut publication, grant).expect("bounded staging turn");
+        receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, 4096, grant,receiving)).expect("bounded staging turn");
@@ -5588 +5718 @@ async fn artifact_store_batch_cancel_mid_flight_retires_every_staged_owner_witho
-    assert!(matches!(store.advance_apply_batch(&mut publication, grant), Ok(ArtifactStoreOneItemAdvance::Blocked)));
+    assert!(matches!(receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, 4096, grant,receiving)), Ok(ArtifactStoreOneItemAdvance::Blocked)));
@@ -5611 +5741 @@ async fn retained_latest_wins_cold_rebase_preserves_admitted_cursor_capacity_for
-        if matches!(store.advance_apply_batch(&mut publication, grant).unwrap(), ArtifactStoreOneItemAdvance::Published(_)) {
+        if matches!(receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, fixture["maximumCopyBytes"].as_u64().unwrap()as usize, grant,receiving)).unwrap(), ArtifactStoreOneItemAdvance::Published(_, _)) {
@@ -5636 +5766 @@ async fn artifact_store_one_item_digest_helper_matches_validation_and_rejects_fo
-        assert!(matches!(store.advance_apply_batch(&mut publication, grant), Ok(ArtifactStoreOneItemAdvance::Progress(_) | ArtifactStoreOneItemAdvance::PreparationProgress(..))));
+        assert!(matches!(receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, 4096, grant,receiving)), Ok(ArtifactStoreOneItemAdvance::Progress(_) | ArtifactStoreOneItemAdvance::PreparationProgress(..))));
@@ -5649 +5779 @@ async fn artifact_store_one_item_digest_helper_matches_validation_and_rejects_fo
-        match store.advance_apply_batch(&mut publication, grant) {
+        match receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, 4096, grant,receiving)) {
@@ -5651 +5781 @@ async fn artifact_store_one_item_digest_helper_matches_validation_and_rejects_fo
-            Ok(ArtifactStoreOneItemAdvance::Published(_)) => panic!("a domain-forged digest published"),
+            Ok(ArtifactStoreOneItemAdvance::Published(_, _)) => panic!("a domain-forged digest published"),
@@ -5709 +5839 @@ async fn artifact_store_stamped_publication_clock_is_admitted_ahead_and_refused_
-        if let ArtifactStoreOneItemAdvance::Published(receipt) = store.advance_apply_batch(&mut publication, grant).expect("one bounded stamped step") {
+        if let ArtifactStoreOneItemAdvance::Published(receipt, _) = receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, 4096, grant,receiving)).expect("one bounded stamped step") {
@@ -5735 +5865 @@ async fn artifact_store_stamped_publication_clock_is_admitted_ahead_and_refused_
-        if let ArtifactStoreOneItemAdvance::Published(receipt) = store.advance_apply_batch(&mut tied_publication, grant).expect("one bounded same-millisecond step") {
+        if let ArtifactStoreOneItemAdvance::Published(receipt, _) = receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut tied_publication, 4096, grant,receiving)).expect("one bounded same-millisecond step") {
@@ -5764 +5894 @@ async fn artifact_store_stamped_publication_clock_admits_a_genesis_document_behi
-        if let ArtifactStoreOneItemAdvance::Published(_) = store.advance_apply_batch(&mut publication, grant).expect("one bounded genesis step") {
+        if let ArtifactStoreOneItemAdvance::Published(_, _) = receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, 4096, grant,receiving)).expect("one bounded genesis step") {
@@ -5804 +5934 @@ async fn artifact_store_stamped_publication_carries_its_committed_identity_into_
-        if let ArtifactStoreOneItemAdvance::Published(_) = store.advance_apply_batch(&mut publication, grant).expect("one bounded stamped-identity step") {
+        if let ArtifactStoreOneItemAdvance::Published(_, _) = receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, 4096, grant,receiving)).expect("one bounded stamped-identity step") {
@@ -5820 +5950 @@ async fn artifact_store_stamped_publication_carries_its_committed_identity_into_
-        if let ArtifactStoreOneItemAdvance::Published(_) = store.advance_apply_batch(&mut gesture, grant).expect("one bounded unstamped step") {
+        if let ArtifactStoreOneItemAdvance::Published(_, _) = receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut gesture, 4096, grant,receiving)).expect("one bounded unstamped step") {
@@ -5839 +5969 @@ impl ArtifactEphemeralPreparationTask<DemoSnapshot, DemoMutation> for FaultingEp
-    fn advance(&mut self, _: &DemoSnapshot, _: &mut Option<DemoMutation>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactEphemeralPreparationTaskStep<DemoSnapshot>, ValueError> {
+    fn advance(&mut self, _: &DemoSnapshot, _: &mut Option<DemoMutation>, grant: ArtifactStoreOneItemGrant, receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<ArtifactEphemeralPreparationTaskStep<DemoSnapshot>, ValueError> {
@@ -5852,0 +5983 @@ impl ErasedSnapshotRetirement for FaultingEphemeralTask {
+    fn next_demand(&self, body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() })}
@@ -5902 +6033 @@ fn ephemeral_transfer_preparation_faults_close_presence_and_transient_owners() {
-        let presence_advance = presence.advance_publish_one(&mut presence_publication, grant);
+        let presence_advance = receiving_fixture.receive(|receiving|presence.advance_publish_one(&mut presence_publication, grant,receiving));
@@ -5904 +6035 @@ fn ephemeral_transfer_preparation_faults_close_presence_and_transient_owners() {
-        let transient_advance = transient.advance_publish_one(&mut transient_publication, grant);
+        let transient_advance = receiving_fixture.receive(|receiving|transient.advance_publish_one(&mut transient_publication, grant,receiving));
@@ -5931,0 +6063 @@ async fn an_ephemeral_one_item_publication_reports_monotone_progress_across_owne
+    let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());
@@ -5949 +6081 @@ async fn an_ephemeral_one_item_publication_reports_monotone_progress_across_owne
-        let advance = transient.advance_publish_one(&mut publication, grant).expect("ephemeral step");
+        let advance = receiving_fixture.receive(|receiving|transient.advance_publish_one(&mut publication, grant,receiving)).expect("ephemeral step");
@@ -5951 +6083 @@ async fn an_ephemeral_one_item_publication_reports_monotone_progress_across_owne
-        if matches!(advance, ArtifactStoreOneItemAdvance::Published(_)) {
+        if matches!(advance, ArtifactStoreOneItemAdvance::Published(_, _)) {
@@ -5970,0 +6103 @@ async fn presence_and_transient_one_item_publications_are_retained_stale_safe_ca
+    let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());
@@ -5979 +6112 @@ async fn presence_and_transient_one_item_publications_are_retained_stale_safe_ca
-        if let ArtifactStoreOneItemAdvance::Published(receipt) = presence.advance_publish_one(&mut presence_publication, grant).expect("presence step") {
+        if let ArtifactStoreOneItemAdvance::Published(receipt, _) = receiving_fixture.receive(|receiving|presence.advance_publish_one(&mut presence_publication, grant,receiving)).expect("presence step") {
@@ -6013 +6146 @@ async fn presence_and_transient_one_item_publications_are_retained_stale_safe_ca
-        if let ArtifactStoreOneItemAdvance::Published(receipt) = transient.advance_publish_one(&mut transient_publication, grant).expect("transient step") {
+        if let ArtifactStoreOneItemAdvance::Published(receipt, _) = receiving_fixture.receive(|receiving|transient.advance_publish_one(&mut transient_publication, grant,receiving)).expect("transient step") {
@@ -7146,5 +7279 @@ async fn history_lane_interaction_entries_survive_owned_document_round_trip() {
-//#region 🔖️InteractionStoreTests
-/// 🕹️ `InteractionStore::apply` mutates the local hover value and bumps `generation`,
-/// mirroring `PresenceStore`/`TransientStore` — the same `Mutation<S>::diff().apply()` seam,
-/// reused here with the file's existing `DemoSnapshot`/`DemoMutation` fixtures standing in for
-/// an app's hover-shaped type.
+/// 🫧️ Local transient defaults live in the typed ephemeral lane.
@@ -7152,2 +7281,3 @@ async fn history_lane_interaction_entries_survive_owned_document_round_trip() {
-async fn interaction_store_apply_updates_hover_and_bumps_generation() {
-    let mut store = InteractionStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(0) });
+async fn transient_store_default_seeds_local_state_and_generation() {
+    let store = TransientStore::<DemoSnapshot, DemoMutation>::default();
+    assert_eq!(store.current().await.n, Some(0));
@@ -7155 +7285 @@ async fn interaction_store_apply_updates_hover_and_bumps_generation() {
-    assert_eq!(store.hover().await.n, Some(0));
+}
@@ -7157,3 +7286,0 @@ async fn interaction_store_apply_updates_hover_and_bumps_generation() {
-    store.apply(&[DemoMutation::SetN(SetN { n: 7 })]).await.expect("valid interaction mutation");
-    assert_eq!(store.hover().await.n, Some(7), "apply routes through Mutation::diff/Diff::apply like PresenceStore");
-    assert_eq!(store.generation().await, 1);
@@ -7161,2 +7288,8 @@ async fn interaction_store_apply_updates_hover_and_bumps_generation() {
-    store.apply(&[]).await.expect("empty interaction batch");
-    assert_eq!(store.generation().await, 1, "an empty mutation batch must not bump generation, same as PresenceStore/TransientStore");
+fn publish_original_interaction(store: &mut ArtifactStore<DemoSnapshot, DemoMutation>, value: i32) {
+    let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());
+ let g=physical_test_close_grant();let grant=ArtifactStoreOneItemGrant{maximum_items:g.maximum_items,maximum_copy_bytes:g.maximum_copy_bytes,maximum_capacity_bytes:g.maximum_capacity_bytes,maximum_release_bytes:g.maximum_release_bytes,maximum_depth:g.maximum_depth};
+ let(actor,p)=store.local_actor_id().0.admit_clone(g).unwrap();assert!(p.fits(g));
+ let mut actor=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(actor).ok().unwrap());let mut inputs=vec![DemoMutation::SetN(SetN{n:value})];let mut transaction=None;
+ let(mut publication,p)=store.admit_apply_batch(semio_framework_job::allocate_operation_id(),store.generation(),store.content_revision_now(),&mut actor,&mut inputs,HistoryLane::Interaction,store.one_item_preparation_factory.as_ref(),&mut transaction,None,false,g).unwrap().expect("original fixture admits retained interaction");assert!(p.fits(g));
+ let before=store.generation();let denied=ArtifactStoreOneItemGrant{maximum_items:0,..grant};assert!(matches!(receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, physical_test_close_grant().maximum_copy_bytes,denied,receiving)).unwrap(),ArtifactStoreOneItemAdvance::Blocked));assert_eq!(store.generation(),before);let mut published=false;for _ in 0..262_144{match receiving_fixture.receive(|receiving|store.advance_apply_batch(&mut publication, physical_test_close_grant().maximum_copy_bytes,grant,receiving)).unwrap(){ArtifactStoreOneItemAdvance::PreparationProgress(_,p)=>assert!(p.fits(g)),ArtifactStoreOneItemAdvance::Published(_,p)=>{assert!(p.fits(g));published=true;break;},_=>{}}}assert!(published);assert!(publication.acknowledge());
+ for _ in 0..262_144{if publication.terminal_is_empty(){break;}assert!(publication.close_step(g).unwrap().progress().fits(g));}assert!(publication.terminal_is_empty());
@@ -7164,3 +7296,0 @@ async fn interaction_store_apply_updates_hover_and_bumps_generation() {
-
-/// 🔄️ `reset` discards the current hover outright (a host clears hover when a
-/// view/window closes) and still bumps `generation` so a pending broadcast reflects the clear.
@@ -7168,8 +7298,2 @@ async fn interaction_store_apply_updates_hover_and_bumps_generation() {
-async fn interaction_store_reset_discards_hover_and_bumps_generation() {
-    let mut store = InteractionStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(0) });
-    store.apply(&[DemoMutation::SetN(SetN { n: 3 })]).await.expect("valid interaction mutation");
-    assert_eq!(store.generation().await, 1);
-
-    store.reset(DemoSnapshot { n: Some(0) }).await;
-    assert_eq!(store.hover().await.n, Some(0));
-    assert_eq!(store.generation().await, 2, "reset bumps generation even though the value returns to default");
+async fn interaction_store_apply_updates_hover_and_bumps_generation(){
+ let mut store=ArtifactStore::bare(create_document_envelope("demo/v1","original-interaction",DemoSnapshot{n:Some(0)},None)).await;store.install_document_store_owners_exact(retained_demo_member_owners()).unwrap_or_else(|(error,_)|panic!("{error}"));store.closes_on_drop();let before=store.generation();publish_original_interaction(&mut store,7);assert_eq!(store.snapshot().unwrap().n,Some(7));assert_eq!(store.generation(),before+1);assert_eq!(store.envelope().lanes.get(&store.applied_edit_ids()[0]),Some(&HistoryLane::Interaction));
@@ -7177,2 +7300,0 @@ async fn interaction_store_reset_discards_hover_and_bumps_generation() {
-
-/// 🏗️ `Default` seeds from `S::default()`, same convention as `PresenceStore`/`TransientStore`.
@@ -7180,4 +7302,2 @@ async fn interaction_store_reset_discards_hover_and_bumps_generation() {
-async fn interaction_store_default_seeds_from_hover_default() {
-    let store = InteractionStore::<DemoSnapshot, DemoMutation>::default();
-    assert_eq!(store.hover().await.n, Some(0));
-    assert_eq!(store.generation().await, 0);
+async fn interaction_store_reset_discards_hover_and_bumps_generation(){
+ let mut store=ArtifactStore::bare(create_document_envelope("demo/v1","original-interaction-clear",DemoSnapshot{n:Some(0)},None)).await;store.install_document_store_owners_exact(retained_demo_member_owners()).unwrap_or_else(|(error,_)|panic!("{error}"));store.closes_on_drop();let before=store.generation();publish_original_interaction(&mut store,3);publish_original_interaction(&mut store,0);assert_eq!(store.snapshot().unwrap().n,Some(0));assert_eq!(store.generation(),before+2);assert_eq!(store.applied_edit_ids().len(),2);
@@ -7185 +7305,5 @@ async fn interaction_store_default_seeds_from_hover_default() {
-//#endregion 🔖️InteractionStoreTests
+#[semio_framework_async_macros::async_test]
+async fn interaction_store_default_seeds_from_hover_default(){
+ let mut store=ArtifactStore::bare(create_document_envelope("demo/v1","original-interaction-default",DemoSnapshot::default(),None)).await;store.install_document_store_owners_exact(retained_demo_member_owners()).unwrap_or_else(|(error,_)|panic!("{error}"));store.closes_on_drop();assert_eq!(store.snapshot().unwrap().n,Some(0));assert_eq!(store.generation(),0);
+}
+
@@ -7186,0 +7311,48 @@ async fn interaction_store_default_seeds_from_hover_default() {
+#[test]
+fn original_side_lane_comparison_uses_granted_utf8_chunks(){
+ let law:serde_json::Value=serde_json::from_str(include_str!("../../🎮️one-item/🛤️lane/🔣️.json")).unwrap();
+ for row in law["ordering"].as_array().unwrap(){let left=row["left"].as_str().unwrap();let right=row["right"].as_str().unwrap();let wanted=row["order"].as_i64().unwrap();
+  for body in law["grantCopy"].as_array().unwrap(){let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body.as_u64().unwrap()as usize,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};let mut cursor=0;let mut settled=None;
+   for _ in 0..128{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||protocol::HistoryFoldIndexInsert::<String,HistoryLane>::compare_original_text(left,right,cursor,g).unwrap());let(order,next,progress)=result;assert!(progress.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if let Some(order)=order{settled=Some(match order{std::cmp::Ordering::Less=>-1,std::cmp::Ordering::Equal=>0,std::cmp::Ordering::Greater=>1});break;}if progress==Default::default(){assert_eq!(next,cursor);break;}cursor=next;}
+   if let Some(order)=settled{assert_eq!(order,wanted);}else{assert!(g.maximum_copy_bytes<size_of::<usize>()+size_of::<Option<std::cmp::Ordering>>()+2);}
+  }
+ }
+ println!("[DEBUG] original side lane UTF8 ordering: denied turns preserve cursor, granted byte pages match neutral six-case corpus, heap0");
+}
+
+fn original_native_command_turn<R>(operation:semio_framework_job::OperationId,generation:u64,grant:RetainedCloneGrant,cancel:&semio_framework_async::CancelToken,sequence:&mut u64,run:impl FnOnce(&mut semio_framework_job::StepContext<'_>)->R)->(R,RetainedCloneProgress){
+ let mut receipt=RetainedCloneProgress::default();let mut cx=semio_framework_job::StepContext::new(operation,semio_framework_job::Generation(generation),semio_framework_job::StepBudget::new(1,u64::MAX,grant),cancel.clone(),||None,sequence,&mut receipt);let result=run(&mut cx);drop(cx);(result,receipt)
+}
+#[semio_framework_async_macros::async_test]
+async fn native_command_publication_preserves_original_context_issuer_and_denied_inputs(){
+    let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());
+ let g=physical_test_close_grant();let mut store=ArtifactStore::bare(create_document_envelope("demo/v1","native-command",DemoSnapshot{n:Some(0)},None)).await;store.install_document_store_owners_exact(retained_demo_member_owners()).unwrap_or_else(|(error,_)|panic!("{error}"));store.closes_on_drop();
+ let mut foreign=ArtifactStore::bare(create_document_envelope("demo/v1","native-command",DemoSnapshot{n:Some(0)},None)).await;foreign.install_document_store_owners_exact(retained_demo_member_owners()).unwrap_or_else(|(error,_)|panic!("{error}"));foreign.closes_on_drop();
+ let operation=semio_framework_job::allocate_operation_id();let generation=store.generation();let revision=store.content_revision_now();let cancel=semio_framework_async::CancelToken::root_now();let mut sequence=0;let mut publication=None;let(actor,p)=store.local_actor_id().0.admit_clone(g).unwrap();assert!(p.fits(g));let mut actor=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(actor).ok().unwrap());let mut command=ArtifactCommand::ApplyInLane{mutations:vec![DemoMutation::SetN(SetN{n:7})],lane:HistoryLane::Interaction,transaction:None};
+ let pointer=match &command{ArtifactCommand::ApplyInLane{mutations,..}=>mutations.as_ptr(),_=>unreachable!()};
+ for denied in[RetainedCloneGrant{maximum_items:0,..g},RetainedCloneGrant{maximum_copy_bytes:0,..g},RetainedCloneGrant{maximum_capacity_bytes:0,..g},RetainedCloneGrant{maximum_depth:0,..g}]{let(result,p)=original_native_command_turn(operation,generation,denied,&cancel,&mut sequence,|cx|store.admit_native_command(&mut command,&mut actor,generation,revision,&mut publication,cx));assert!(!result.unwrap());assert_eq!(p,Default::default());assert!(publication.is_none());match &command{ArtifactCommand::ApplyInLane{mutations,..}=>{assert_eq!(mutations.as_ptr(),pointer);assert_eq!(mutations.len(),1);},_=>unreachable!()};assert_eq!(store.generation(),generation);}
+ let(result,p)=original_native_command_turn(operation,generation,g,&cancel,&mut sequence,|cx|store.admit_native_command(&mut command,&mut actor,generation,revision,&mut publication,cx));assert!(result.unwrap());assert!(p.fits(g));let owner=publication.as_mut().unwrap();let phase=owner.phase();
+ let wrong_operation=semio_framework_job::allocate_operation_id();let(result,p)=original_native_command_turn(wrong_operation,generation,g,&cancel,&mut sequence,|cx|store.advance_native_command(owner,cx));assert!(result.is_err());assert_eq!(p,Default::default());assert_eq!(owner.phase(),phase);
+ let(result,p)=original_native_command_turn(operation,generation+1,g,&cancel,&mut sequence,|cx|store.advance_native_command(owner,cx));assert!(result.is_err());assert_eq!(p,Default::default());assert_eq!(owner.phase(),phase);
+ let(result,p)=original_native_command_turn(operation,generation,g,&cancel,&mut sequence,|cx|foreign.advance_native_command(owner,cx));assert!(result.is_err());assert_eq!(p,Default::default());assert_eq!(owner.phase(),phase);
+ assert_eq!(foreign.generation(),generation);assert_eq!(foreign.content_revision_now(),revision);
+ let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||receiving_fixture.receive(|receiving|foreign.advance_apply_batch(owner, physical_test_close_grant().maximum_copy_bytes,ArtifactStoreOneItemGrant::from_retained(g),receiving)));assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(owner.phase(),phase);
+ assert!(matches!(foreign.original_proposal_forward(owner).unwrap_err().kind,semio_framework_value::ValueRefusalKind::InvariantViolated));
+ assert!(foreign.flush_published_apply_batch(owner).await.is_err());assert_eq!(owner.phase(),phase);
+ let mut published=false;for _ in 0..262_144{let(result,p)=original_native_command_turn(operation,generation,g,&cancel,&mut sequence,|cx|store.advance_native_command(owner,cx));assert!(p.fits(g));if matches!(result.unwrap(),ArtifactStoreOneItemAdvance::Published(..)){published=true;break;}}assert!(published);assert_eq!(store.snapshot_ref().n,Some(7));assert_eq!(store.generation(),generation+1);
+ let(result,p)=original_native_command_turn(operation,generation,RetainedCloneGrant{maximum_copy_bytes:0,..g},&cancel,&mut sequence,|cx|store.acknowledge_native_command(owner,cx));assert!(!result.unwrap());assert_eq!(p,Default::default());
+ let(result,p)=original_native_command_turn(operation,generation,g,&cancel,&mut sequence,|cx|store.acknowledge_native_command(owner,cx));assert!(result.unwrap());assert!(p.fits(g));for _ in 0..262_144{if owner.terminal_is_empty(){break;}let(result,p)=original_native_command_turn(operation,generation,g,&cancel,&mut sequence,|cx|store.advance_native_command(owner,cx));result.unwrap();assert!(p.fits(g));}assert!(owner.terminal_is_empty());
+ println!("[DEBUG] native command original context: four denied axes retain Vec pointer and actor, foreign issuer/job denied0, retained interaction publishes once, funded ACK+close terminal");
+}
+
+#[semio_framework_async_macros::async_test]
+async fn original_envelope_parse_preserves_genesis_without_a_discarded_projection_clone(){
+ let law:serde_json::Value=serde_json::from_str(include_str!("../../🎮️one-item/🛤️lane/🔣️.json")).unwrap();
+ for row in law["envelope"].as_array().unwrap(){
+  let value=row["initial"].as_i64().unwrap()as i32;let envelope=create_document_envelope::<DemoSnapshot,DemoMutation>("demo/v1",row["name"].as_str().unwrap(),DemoSnapshot{n:Some(value)},None);let files=print_document_pack(&envelope).await.unwrap();envelope.retire_unadopted();
+  DEMO_PROJECTION_CLONES.with(|count|count.set(0));
+  let parsed=text_format::parse_document_pack_envelope::<DemoSnapshot,DemoMutation>(&files.pack,&files.spr).await.unwrap();
+  assert_eq!(parsed.vcs.genesis.facts().snapshot().n,Some(row["expected"].as_i64().unwrap()as i32));assert_eq!(parsed.vcs.genesis.pack(),files.pack.as_slice());assert!(parsed.vcs.edits.is_empty());assert_eq!(DEMO_PROJECTION_CLONES.with(std::cell::Cell::get),row["discardedProjectionClones"].as_u64().unwrap()as usize);parsed.retire_unadopted();
+ }
+ println!("[DEBUG] original envelope parser: three neutral genesis cases retain pack facts and metadata validation with discardedProjectionClones0; semantic replay stays owned by receiving Store");
+}
@@ -7207,0 +7380 @@ async fn presence_local_read_is_o1_and_never_clones_the_payload_at_capture() {
+    let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());
@@ -7215 +7388,13 @@ async fn presence_local_read_is_o1_and_never_clones_the_payload_at_capture() {
-    store.apply(&[DemoMutation::SetN(SetN { n: 2 })]).await.expect("presence apply");
+    let preparation_factory = DemoEphemeralPreparationFactory::admissible();
+    let mut publication = store.begin_publish_one(semio_framework_job::OperationId(2), 0, DemoMutation::SetN(SetN { n: 2 }), Some(&preparation_factory), Some(factory.clone())).unwrap_or_else(|_| panic!("original retained presence publication admits"));
+    let mut published = false;
+    for _ in 0..2048 {
+        if let ArtifactStoreOneItemAdvance::Published(receipt, _) = receiving_fixture.receive(|receiving|store.advance_publish_one(&mut publication, ephemeral_test_close_grant(),receiving)).expect("original presence publication advances") {
+            assert_eq!(receipt, LaneItemReceipt { generation_before: 0, generation_after: 1 });
+            published = true;
+            break;
+        }
+    }
+    assert!(published);
+    assert!(publication.acknowledge());
+    close_ephemeral_publication(&mut publication);
@@ -8309 +8494 @@ impl OpBinary for TimestampedMutation {
-async fn demo_member<'a, Mutation: self::Mutation<DemoSnapshot> + 'static, M: SpaceMember + 'static>(host: &'a mut SpaceHost<M>, document_id: &str) -> &'a mut ArtifactStore<DemoSnapshot, Mutation> {
+async fn demo_member<'a, Mutation: self::Mutation<DemoSnapshot> + Clone + ColdMutation<DemoSnapshot> + 'static, M: SpaceMember + 'static>(host: &'a mut SpaceHost<M>, document_id: &str) -> &'a mut ArtifactStore<DemoSnapshot, Mutation> {
@@ -8514 +8699 @@ async fn space_history_op_round_trips() {
-    test_support::assert_operation_round_trip(&SpaceHistorySnapshot::default(), SpaceHistoryMutation::CommitSpaceCheckpoint(CommitSpaceCheckpoint { checkpoint: checkpoint.clone() })).await;
+    test_support::assert_operation_round_trip(&SpaceHistorySnapshot::default(), SpaceHistoryMutation::CommitSpaceCheckpoint(CommitSpaceCheckpoint { checkpoint: checkpoint.clone(), index: None })).await;
@@ -8554 +8739 @@ async fn space_history_inverse_diffs_sum_to_the_negative_diff() {
-    law(&SpaceHistoryMutation::CommitSpaceCheckpoint(CommitSpaceCheckpoint { checkpoint: checkpoint.clone() }), &SpaceHistorySnapshot::default()).await;
+    law(&SpaceHistoryMutation::CommitSpaceCheckpoint(CommitSpaceCheckpoint { checkpoint: checkpoint.clone(), index: None }), &SpaceHistorySnapshot::default()).await;
@@ -9735,0 +9921,35 @@ async fn reload_replays_the_event_log_into_the_same_positions() {
+/// 🎯️ A reload adopts the central fold: after `reset` the store's current projection is exactly the genesis snapshot taken through
+/// [`protocol::apply_diff_cold`] once per applied operation, in applied order — the oracle below calls the central applier directly and
+/// the store holds no other route from an event log to a projection.
+#[semio_framework_async_macros::async_test]
+async fn a_reload_adopts_the_central_apply_fold_of_the_applied_event_log() {
+    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
+    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
+    let mut identity:crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<crate::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
+    let mut store = fresh_demo_store().await;
+    for command in [
+        ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None },
+        ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], transaction: None },
+        ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 8 })], transaction: None },
+        ArtifactCommand::Undo,
+    ] {
+        store.dispatch(command, &mut identity).await.expect("history step");
+    }
+    let candidate = owned_test_envelope(&store).await;
+    let mut reloaded = fresh_demo_store().await;
+    reloaded.reset(candidate).await.expect("reload");
+
+    let envelope = reloaded.envelope();
+    let mut oracle = envelope.vcs.genesis.facts().snapshot().clone();
+    for edit_id in reloaded.applied_edit_ids().iter() {
+        let edit = envelope.vcs.edits.iter().find(|edit| edit.id == *edit_id).expect("the applied edit is in the log");
+        for operation in edit.forwards.iter() {
+            oracle = protocol::apply_diff_cold(operation.diff(&oracle).diff(), &oracle).expect("the central applier folds the applied operation");
+        }
+    }
+    assert_eq!(reloaded.snapshot().expect("snapshot"), oracle, "the reloaded projection is the central apply fold of the applied operations");
+    assert_eq!(oracle.n, Some(3), "the undone edit is not part of the fold");
+    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
+    drop(identity.pause().expect("original Unit cumulative identity receipt"));
+}
+
@@ -9751 +9971 @@ async fn space_member_checkout_switches_at_the_alternative_tip_and_falls_back_to
-    SpaceMember::checkout(&mut store, &tip, &alt_id, &mut identity).await.expect("checkout at the tip routes through SwitchAlternative");
+    ReferenceSpaceMember::checkout(&mut store, &tip, &alt_id, &mut identity).await.expect("checkout at the tip routes through SwitchAlternative");
@@ -9757 +9977 @@ async fn space_member_checkout_switches_at_the_alternative_tip_and_falls_back_to
-    SpaceMember::checkout(&mut store, &tip, &alt_id, &mut identity).await.expect("checkout of the now-stale tip falls back to CheckoutCheckpoint");
+    ReferenceSpaceMember::checkout(&mut store, &tip, &alt_id, &mut identity).await.expect("checkout of the now-stale tip falls back to CheckoutCheckpoint");
@@ -10237 +10457 @@ async fn pack_at_checkpoint_reads_history_without_moving_the_live_cursor() {
-    let checkpoint = SpaceMember::commit_checkpoint(&mut store, "v1".into(), Vec::new(), &mut identity).await.expect("checkpoint");
+    let checkpoint = ReferenceSpaceMember::commit_checkpoint(&mut store, "v1".into(), Vec::new(), &mut identity).await.expect("checkpoint");
@@ -10268 +10488 @@ async fn member_link_resolver_resolves_head_checkpoint_and_degrades_snapshot_pin
-    let pinned = SpaceMember::commit_checkpoint(&mut member, "pinned".into(), Vec::new(), &mut identity).await.expect("checkpoint");
+    let pinned = ReferenceSpaceMember::commit_checkpoint(&mut member, "pinned".into(), Vec::new(), &mut identity).await.expect("checkpoint");
@@ -11352,0 +11573 @@ impl ErasedSnapshotRetirement for DemandingBufferRetirement {
+    fn next_demand(&self, body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(semio_framework_value::RetirementDemand { release_bytes: self.buffer.as_ref().map_or(0,Vec::capacity), depth: usize::from(self.buffer.is_some()), ..Default::default() })}
@@ -11374,0 +11596 @@ impl ErasedSnapshotRetirement for UnsupportedDemandRetirement {
+    fn next_demand(&self, body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"owner has no admitted physical retirement contract"))}
@@ -11876,0 +12099 @@ fn envelope_and_genesis_preserve_terminal_cursor_physical_grants() {
+    fn next_demand(&self, body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(Default::default())}
@@ -11940,0 +12164 @@ fn displaced_store_owner_preserves_terminal_physical_grants() {
+    fn next_demand(&self, body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(Default::default())}
@@ -11991,0 +12216 @@ async fn installed_store_disposer_preserves_terminal_physical_grant() {
+    fn next_demand(&self, body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(Default::default())}
@@ -12383 +12608 @@ fn ephemeral_member_original_preparation_frame_requires_exact_granted_release()
-        fn advance(&mut self, _: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> { Ok(ArtifactStoreOneItemPreparationStep::Blocked) }
+        fn advance(&mut self, _: ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> { Ok(ArtifactStoreOneItemPreparationStep::Blocked) }

```
## `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-demo-add-n/🦀️.rs`

```diff
diff --git a/faa1a591d1f151f6c67e591c7b850be9bdb7933c b/93fcf8278c451f0676b89d913e2abc579ff35acf
index faa1a591d1f..93fcf8278c4 100644
--- a/faa1a591d1f151f6c67e591c7b850be9bdb7933c
+++ b/93fcf8278c451f0676b89d913e2abc579ff35acf
@@ -5,0 +6,5 @@ fn direct_fixture_leaf_contract() {
+
+#[semio_framework_async_macros::async_test]
+async fn inverse_diffs_sum_to_the_negative_diff() {
+    crate::os_store::component::tests::assert_fixture_sum_law(&DemoMutation::AddN(AddN { delta: 4 }), &[Some(3)]).await;
+}

```
## `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-demo-assign-n/🦀️.rs`

```diff
diff --git a/2dabdeb97f21e0cbe9a68e21c75ef7090b2ee8cf b/773d84c4136cd615f8164f074678195f92d8c11c
index 2dabdeb97f2..773d84c4136 100644
--- a/2dabdeb97f21e0cbe9a68e21c75ef7090b2ee8cf
+++ b/773d84c4136cd615f8164f074678195f92d8c11c
@@ -5,0 +6,6 @@ fn direct_fixture_leaf_contract() {
+
+#[semio_framework_async_macros::async_test]
+async fn inverse_diffs_sum_to_the_negative_diff() {
+    crate::os_store::component::tests::assert_fixture_sum_law(&DemoMutation::AssignN(AssignN { n: Some(5) }), &[Some(3), None]).await;
+    crate::os_store::component::tests::assert_fixture_sum_law(&DemoMutation::AssignN(AssignN { n: None }), &[Some(3)]).await;
+}

```
## `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-demo-delete-n/🦀️.rs`

```diff
diff --git a/4c713e76f6e900a644ed4afd39658b00d9793f21 b/acf12decc5984f6b85a0b6b71deeea09c56d46ea
index 4c713e76f6e..acf12decc59 100644
--- a/4c713e76f6e900a644ed4afd39658b00d9793f21
+++ b/acf12decc5984f6b85a0b6b71deeea09c56d46ea
@@ -5,0 +6,5 @@ fn direct_fixture_leaf_contract() {
+
+#[semio_framework_async_macros::async_test]
+async fn inverse_diffs_sum_to_the_negative_diff() {
+    crate::os_store::component::tests::assert_fixture_sum_law(&DemoMutation::DeleteN(DeleteN {}), &[Some(3)]).await;
+}

```
## `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-demo-set-n/🦀️.rs`

```diff
diff --git a/e43c997a3fd87779a83d5ce415662b7c10c53471 b/7e5495349f575c90236bd071f13d0311aae3326a
index e43c997a3fd..7e5495349f5 100644
--- a/e43c997a3fd87779a83d5ce415662b7c10c53471
+++ b/7e5495349f575c90236bd071f13d0311aae3326a
@@ -5,0 +6,5 @@ fn direct_fixture_leaf_contract() {
+
+#[semio_framework_async_macros::async_test]
+async fn inverse_diffs_sum_to_the_negative_diff() {
+    crate::os_store::component::tests::assert_fixture_sum_law(&DemoMutation::SetN(SetN { n: 9 }), &[Some(3)]).await;
+}

```
## `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-severity-assign-n/🦀️.rs`

```diff
diff --git a/8c484a3df5ac90b643c0966b5fe3b1b559e0c08b b/183678b28428bca62ec34e157dbc8738a6a43e00
index 8c484a3df5a..183678b2842 100644
--- a/8c484a3df5ac90b643c0966b5fe3b1b559e0c08b
+++ b/183678b28428bca62ec34e157dbc8738a6a43e00
@@ -5,0 +6,5 @@ fn direct_fixture_leaf_contract() {
+
+#[semio_framework_async_macros::async_test]
+async fn inverse_diffs_sum_to_the_negative_diff() {
+    crate::os_store::component::tests::assert_fixture_sum_law(&SeverityMutation::AssignN(AssignN { n: Some(5) }), &[Some(3), None]).await;
+}

```
## `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-severity-set-n/🦀️.rs`

```diff
diff --git a/86840f34925caf1bfc25c53988fea8b9ef7ba071 b/6f32771749d1a2f0fff1b515012319e0fd23de68
index 86840f34925..6f32771749d 100644
--- a/86840f34925caf1bfc25c53988fea8b9ef7ba071
+++ b/6f32771749d1a2f0fff1b515012319e0fd23de68
@@ -5,0 +6,5 @@ fn direct_fixture_leaf_contract() {
+
+#[semio_framework_async_macros::async_test]
+async fn inverse_diffs_sum_to_the_negative_diff() {
+    crate::os_store::component::tests::assert_fixture_sum_law(&SeverityMutation::SetN(SetN { n: 9 }), &[Some(3)]).await;
+}

```
## `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-severity-set-warning-n/🦀️.rs`

```diff
diff --git a/1d571b607c3c9060e1ff5cc8063f280832bfacb2 b/084614c8be1d0d520e43ae3d1aaa5ef220d53860
index 1d571b607c3..084614c8be1 100644
--- a/1d571b607c3c9060e1ff5cc8063f280832bfacb2
+++ b/084614c8be1d0d520e43ae3d1aaa5ef220d53860
@@ -5,0 +6,5 @@ fn direct_fixture_leaf_contract() {
+
+#[semio_framework_async_macros::async_test]
+async fn inverse_diffs_sum_to_the_negative_diff() {
+    crate::os_store::component::tests::assert_fixture_sum_law(&SeverityMutation::SetWarningN(SetWarningN { n: 9 }), &[Some(3)]).await;
+}

```
## `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-timestamped-assign-n/🦀️.rs`

```diff
diff --git a/89fa69265395e36c452b88ce6a04b522f2ee8f88 b/efe4a3daf0d826ce3ec10864a629b0c1861bb340
index 89fa6926539..efe4a3daf0d 100644
--- a/89fa69265395e36c452b88ce6a04b522f2ee8f88
+++ b/efe4a3daf0d826ce3ec10864a629b0c1861bb340
@@ -5,0 +6,5 @@ fn direct_fixture_leaf_contract() {
+
+#[semio_framework_async_macros::async_test]
+async fn inverse_diffs_sum_to_the_negative_diff() {
+    crate::os_store::component::tests::assert_fixture_sum_law(&TimestampedMutation::AssignN(AssignN { n: Some(5), physical_ms: 7 }), &[Some(3), None]).await;
+}

```
## `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-timestamped-set-n/🦀️.rs`

```diff
diff --git a/a595d15369a4a633ad32ae152e99b68ee5f0c39c b/e302eb0f0c85f494f66e604070a0bd148e688955
index a595d15369a..e302eb0f0c8 100644
--- a/a595d15369a4a633ad32ae152e99b68ee5f0c39c
+++ b/e302eb0f0c85f494f66e604070a0bd148e688955
@@ -5,0 +6,5 @@ fn direct_fixture_leaf_contract() {
+
+#[semio_framework_async_macros::async_test]
+async fn inverse_diffs_sum_to_the_negative_diff() {
+    crate::os_store::component::tests::assert_fixture_sum_law(&TimestampedMutation::SetN(SetN { n: 9, physical_ms: 7 }), &[Some(3)]).await;
+}

```
## `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-validated-assign-n/🦀️.rs`

```diff
diff --git a/347ee71b5064d6c5c9a408b76f4d96830b6b0353 b/6e4fd86fc3202044c8558ec91e563ceaf6415a7b
index 347ee71b506..6e4fd86fc32 100644
--- a/347ee71b5064d6c5c9a408b76f4d96830b6b0353
+++ b/6e4fd86fc3202044c8558ec91e563ceaf6415a7b
@@ -5,0 +6,5 @@ fn direct_fixture_leaf_contract() {
+
+#[semio_framework_async_macros::async_test]
+async fn inverse_diffs_sum_to_the_negative_diff() {
+    crate::os_store::component::tests::assert_fixture_sum_law(&ValidatedMutation::AssignN(AssignN { n: Some(5) }), &[Some(3), None]).await;
+}

```
## `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️fixture-validated-set-n/🦀️.rs`

```diff
diff --git a/2ea98a6b6b4a82a170daaf8201f28e026805d4bd b/cee8ad19697733e1ede6c3ba3f5fe0033fa4071b
index 2ea98a6b6b4..cee8ad19697 100644
--- a/2ea98a6b6b4a82a170daaf8201f28e026805d4bd
+++ b/cee8ad19697733e1ede6c3ba3f5fe0033fa4071b
@@ -5,0 +6,5 @@ fn direct_fixture_leaf_contract() {
+
+#[semio_framework_async_macros::async_test]
+async fn inverse_diffs_sum_to_the_negative_diff() {
+    crate::os_store::component::tests::assert_fixture_sum_law(&ValidatedMutation::SetN(SetN { n: 9 }), &[Some(3)]).await;
+}

```
## `🧰️framework/🛍️products/💻️os/🔨️modules/🧵️backbone/🟦️.ts`

```diff
diff --git a/904488b63e18182d73ef6749d9187d0bfafbb99f b/2f89d007c020e5f5fd11f01da08fb14b48ce6c90
index 904488b63e1..2f89d007c02 100644
--- a/904488b63e18182d73ef6749d9187d0bfafbb99f
+++ b/2f89d007c020e5f5fd11f01da08fb14b48ce6c90
@@ -660,3 +660,4 @@ export function bindingsDataClass(bindings: readonly PersistenceBinding[]): Pers
-  const first = bindings[0];
-  if (!first) return "ephemeralLocalOnly";
-  return first.dataClass;
+  for (const dataClass of ["persistedShared", "persistedLocalOnly", "ephemeralShared"] as const) {
+    if (bindings.some(binding => binding.dataClass === dataClass)) return dataClass;
+  }
+  return "ephemeralLocalOnly";

```
