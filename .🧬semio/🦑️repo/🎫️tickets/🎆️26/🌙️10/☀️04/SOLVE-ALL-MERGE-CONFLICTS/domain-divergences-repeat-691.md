
## 0 ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts

### HEAD
```diff
diff --git a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
index 3463b16bb08..eb93d902eb1 100644
--- a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
+++ b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
@@ -7,5 +7,5 @@ import fixture from "./🧫️fixtures/🔣️.json";
 import {fem2dSnapshotToSqliteDatabase,fem2dSnapshotFromSqliteDatabase} from "../🟦️.ts";
 import type {FemDof} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
-import {exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase,type SqliteValue} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
+import {exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase,type SqliteValue} from "@semio-tech/framework/io/sqlite-snapshot";
 const bits=(word:string)=>({bits:BigInt("0x"+word)});
 function dof(value:string):FemDof{switch(value){case"Tx":case"Ty":case"Tz":case"Rx":case"Ry":case"Rz":return value;default:throw Error("fixture DOF")}}

```

### INCOMING
```diff
diff --git a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
index 3463b16bb08..dfc149f5d05 100644
--- a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
+++ b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
@@ -1,2 +1,3 @@
+import * as snapshotSqlite from "../🟦️.ts";
 import { expect, test } from "bun:test";
 import { Database } from "bun:sqlite";
@@ -36,6 +37,6 @@ function copy(d:SqliteDatabase):{tables:{name:string;sql:string;rows:{rowid:bigi
 
 test("fem2d handwritten semantic SQLite capability is owned by its snapshot", () => {
- expect(Object.hasOwn(snapshot, "fem2dSnapshotToSqliteDatabase")).toBe(true);
- expect(Object.hasOwn(snapshot, "fem2dSnapshotFromSqliteDatabase")).toBe(true);
+ expect(Object.hasOwn(snapshotSqlite, "fem2dSnapshotToSqliteDatabase")).toBe(true);
+ expect(Object.hasOwn(snapshotSqlite, "fem2dSnapshotFromSqliteDatabase")).toBe(true);
 });
 
@@ -93,4 +94,11 @@ test("fem2d independent SQLite DDL owns relationships and exact scalar columns",
 for(const word of fixture.ieee754Words)test(`fem2d canonical snapshot artifact and diff exact word ${word}`,async()=>{
   const source=state();setWords(source,word);
+  expect(()=>artifact.parseFem2dArtifact(source)).toThrow("canonical unique DOF set required");
+  source.supports=source.supports.map(support=>({...support,fixed:[...new Set(support.fixed)]}));
+  expect(()=>artifact.parseFem2dArtifact(source)).toThrow("scoped identity segment");
+  source.nodes.forEach((row,index)=>{row.id=fixture.canonicalIdentities.nodes[index]!;});
+  expect(()=>artifact.parseFem2dArtifact(source)).toThrow("repeated entity identity");
+  source.elements.forEach((row,index)=>{row.id=fixture.canonicalIdentities.elements[index]!;});
+  source.loadCases.forEach(loadCase=>loadCase.loads.forEach((row,index)=>{row.id=fixture.canonicalIdentities.loads[index]!;}));
   for(const count of fixture.countWords){source.analysis.modalCount=BigInt(count);source.analysis.bucklingCount=BigInt(count);
    expect(snapshot.parseFemAnalysisSettings(source.analysis)).toEqual(source.analysis);
@@ -100,5 +108,9 @@ for(const word of fixture.ieee754Words)test(`fem2d canonical snapshot artifact a
    const delta=diff.parseFem2dNodesDelta({removed:[],inserted,moved:[],modified:[]});
    expect(delta.inserted).toEqual(inserted);
-   expect(diff.parseFem2dNodesDelta({removed:[],inserted:[],moved:[],modified:source.nodes.map(patch=>({id:patch.id,patch}))}).modified).toEqual(source.nodes.map(patch=>({id:patch.id,patch})));
+   expect(()=>diff.parseFem2dNodesDelta({removed:[],inserted:[],moved:[],modified:source.nodes.map(patch=>({id:patch.id,patch}))})).toThrow("unknown node field");
+   const modified=source.nodes.map(({id,...patch})=>({id,patch}));
+   const finite=(BigInt("0x"+word)&0x7ff0000000000000n)!==0x7ff0000000000000n;
+   for(const {patch} of modified){if(finite)expect(diff.parseFem2dNodePatch(patch)).toEqual(patch);else expect(()=>diff.parseFem2dNodePatch(patch)).toThrow("invalid node value");}
+   expect(diff.parseFem2dNodesDelta({removed:[],inserted:[],moved:[],modified}).modified).toEqual(modified);
    expect(await fem2dSnapshotFromSqliteDatabase(await fem2dSnapshotToSqliteDatabase(artifact.parseFem2dArtifact(source)))).toEqual(source);
   }

```


## 1 ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts

### HEAD
```diff
diff --git a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts
index 904f591de70..6c906ba6254 100644
--- a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts
+++ b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts
@@ -1,4 +1,4 @@
-import {parseSchemaRecord} from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
-import {type Binary64,parseBinary64} from "../../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🌱️value/🔢️ieee754/🟦️.ts";
+import {parseSchemaRecord} from "@semio-tech/framework/schema/record";
+import {type Binary64,parseBinary64} from "@semio-tech/framework/value/ieee754";
 /** 🧬️ Fem2d snapshot schema — artifact-lane fields only. */
 

```

### INCOMING
```diff
diff --git a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts
index 904f591de70..2061409d8e2 100644
--- a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts
+++ b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts
@@ -1,4 +1,6 @@
+import {encodeScopedIdentity} from "../../../../../../../../../../../🧰️framework/🔨️modules/📡️replication/🧾️wire/🪪️scoped-identity/🟦️.ts";
 import {parseSchemaRecord} from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
 import {type Binary64,parseBinary64} from "../../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🌱️value/🔢️ieee754/🟦️.ts";
+
 /** 🧬️ Fem2d snapshot schema — artifact-lane fields only. */
 
@@ -183,5 +185,5 @@ export function parseFemCombination(value: unknown, at = "$"): FemCombination {
 export function parseFem2dSnapshot(value: unknown, at = "$"): Fem2dSnapshot {
   const row = parseSchemaRecord(value, ["nodes", "elements", "materials", "sections", "supports", "loadCases", "combinations", "regions", "analysis"], at);
-  return {
+  const snapshot = {
     nodes: femFem2dSnapshotGuardArray(row["nodes"], `${at}.nodes`).map((item, index) => parseFemNode(item, `${at}.nodes[${index}]`)),
     elements: femFem2dSnapshotGuardArray(row["elements"], `${at}.elements`).map((item, index) => parseFemElement(item, `${at}.elements[${index}]`)),
@@ -194,4 +196,15 @@ export function parseFem2dSnapshot(value: unknown, at = "$"): Fem2dSnapshot {
     analysis: parseFemAnalysisSettings(row["analysis"], `${at}.analysis`),
   };
+  const seen = new Set<string>();
+  const admit = (segments: readonly string[]) => {
+    const identity = encodeScopedIdentity(segments);
+    if (seen.has(identity)) throw Error(`${at}: repeated entity identity`);
+    seen.add(identity);
+  };
+  for (const [kind, rows] of [["node", snapshot.nodes], ["element", snapshot.elements], ["region", snapshot.regions], ["material", snapshot.materials], ["section", snapshot.sections], ["support", snapshot.supports], ["loadCase", snapshot.loadCases], ["combination", snapshot.combinations]] as const) {
+    for (const row of rows) admit([kind, row.id]);
+  }
+  for (const loadCase of snapshot.loadCases) for (const load of loadCase.loads) admit(["load", loadCase.id, load.id]);
+  return snapshot;
 }
 
@@ -254,5 +267,13 @@ export function parseFemSupport(value: unknown, at = "$"): FemSupport {
     id: femFem2dSnapshotGuardString(row["id"], `${at}.id`),
     nodeId: femFem2dSnapshotGuardString(row["nodeId"], `${at}.nodeId`),
-    fixed: femFem2dSnapshotGuardArray(row["fixed"], `${at}.fixed`).map((item, index) => femFem2dSnapshotGuardMember(item, `${at}.fixed[${index}]`, ["Tx", "Ty", "Tz", "Rx", "Ry", "Rz"] as const)),
+    fixed: parseFemFixedDofs(row["fixed"], `${at}.fixed`),
   };
 }
+
+/** 🔒️ Admits a canonical unique subsequence of the authoritative DOF order. */
+export function parseFemFixedDofs(value: unknown, at = "$"): FemDof[] {
+  const order = ["Tx", "Ty", "Tz", "Rx", "Ry", "Rz"] as const;
+  const dofs = femFem2dSnapshotGuardArray(value, at).map((value, index) => femFem2dSnapshotGuardMember(value, `${at}[${index}]`, order));
+  if (dofs.some((dof, index) => index > 0 && order.indexOf(dofs[index - 1]!) >= order.indexOf(dof))) throw Error(`${at}: canonical unique DOF set required`);
+  return dofs;
+}

```


## 2 ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🟦️.ts

### HEAD
```diff
diff --git a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🟦️.ts b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🟦️.ts
index db23d071f9d..0c23fe54b09 100644
--- a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🟦️.ts
+++ b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🟦️.ts
@@ -1,3 +1,3 @@
-import type {Binary64} from "../../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🌱️value/🔢️ieee754/🟦️.ts";
+import type {Binary64} from "@semio-tech/framework/value/ieee754";
 /** ⚡️ Fem2d direct-mutation discriminated union — TS mirror of the Rust `Fem2dMutation` dispatch enum. */
 

```

### INCOMING
```diff
diff --git a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🟦️.ts b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🟦️.ts
index db23d071f9d..7b24c3ee5d8 100644
--- a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🟦️.ts
+++ b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🟦️.ts
@@ -1,3 +1,10 @@
 import type {Binary64} from "../../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🌱️value/🔢️ieee754/🟦️.ts";
+import type { Fem2dLoadPatch } from "../🔺️diff/🟦️.ts";
+import type { Fem2dElementPatch } from "../🔺️diff/🟦️.ts";
+import type { Fem2dSupportPatch } from "../🔺️diff/🟦️.ts";
+import type { Fem2dRegionPatch } from "../🔺️diff/🟦️.ts";
+import type { Fem2dSectionPatch } from "../🔺️diff/🟦️.ts";
+import type { Fem2dNodePatch } from "../🔺️diff/🟦️.ts";
+import type { Fem2dMaterialPatch } from "../🔺️diff/🟦️.ts";
 /** ⚡️ Fem2d direct-mutation discriminated union — TS mirror of the Rust `Fem2dMutation` dispatch enum. */
 
@@ -195,7 +202,9 @@ export type Fem2dMutation =
   | ({ mutation: "deleteMaterial" } & DeleteMaterial)
   | ({ mutation: "replaceMaterial" } & ReplaceMaterial)
+  | ({ mutation: "changeMaterial" } & ChangeMaterial)
   | ({ mutation: "createSection" } & CreateSection)
   | ({ mutation: "deleteSection" } & DeleteSection)
   | ({ mutation: "replaceSection" } & ReplaceSection)
+  | ({ mutation: "changeSection" } & ChangeSection)
   | ({ mutation: "createSupport" } & CreateSupport)
   | ({ mutation: "deleteSupport" } & DeleteSupport)
@@ -213,6 +222,13 @@ export type Fem2dMutation =
   | ({ mutation: "updateAnalysisSettings" } & UpdateAnalysisSettings)
   | ({ mutation: "replaceNode" } & ReplaceNode)
+  | ({ mutation: "changeNode" } & ChangeNode)
   | ({ mutation: "replaceLoad" } & ReplaceLoad)
   | ({ mutation: "changeLoadCaseName" } & ChangeLoadCaseName)
   | ({ mutation: "replaceCombination" } & ReplaceCombination)
-  | ({ mutation: "moveSelection" } & MoveSelection);
+  | ({ mutation: "moveSelection" } & MoveSelection)
+  | ({ mutation: "changeRegion" } & ChangeRegion)
+  | ({ mutation: "changeSupport" } & ChangeSupport)
+  | ({ mutation: "changeElement" } & ChangeElement)
+  | ({ mutation: "changeLoad" } & ChangeLoad)
+  | ({ mutation: "changeCombinationName" } & ChangeCombinationName)
+  | ({ mutation: "changeCombinationTerm" } & ChangeCombinationTerm);

```


## 3 ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🟦️.ts

### HEAD
```diff
diff --git a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🟦️.ts b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🟦️.ts
index d351ec463ae..8654246bcb2 100644
--- a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🟦️.ts
+++ b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🟦️.ts
@@ -1,3 +1,3 @@
-import { parseViewport3dOrbit, type Viewport3dOrbit } from "../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧊️3d/🧬️schema/🟦️.ts";
+import { parseViewport3dOrbit, type Viewport3dOrbit } from "@semio-tech/framework/ui/viewport/3d/schema";
 import { parseResultMode, type ResultMode } from "../../../../../../../../../../../../../⚙️engine/🖥️app-surface/🧬️schema/👁️result-mode/🟦️.ts";
 import { parseFemResultsAnimation, type FemResultsAnimation } from "../../../../../../../../../../../../../⚙️engine/🖥️app-surface/🧬️schema/🔣️results-animation/🟦️.ts";

```

### INCOMING
```diff
diff --git a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🟦️.ts b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🟦️.ts
index d351ec463ae..c3f4214c16f 100644
--- a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🟦️.ts
+++ b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🟦️.ts
@@ -1,2 +1,3 @@
+import { decodeResultSourceIdentity } from "../../../../../../../../../../../../../../../🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🏷️result-source/🟦️.ts";
 import { parseViewport3dOrbit, type Viewport3dOrbit } from "../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧊️3d/🧬️schema/🟦️.ts";
 import { parseResultMode, type ResultMode } from "../../../../../../../../../../../../../⚙️engine/🖥️app-surface/🧬️schema/👁️result-mode/🟦️.ts";
@@ -25,4 +26,8 @@ export function parseFem3dResultsWindowConfig(value: unknown): Fem3dResultsWindo
   const mode = parseResultMode(row.resultMode);
   const resultSourceId = row.resultSourceId === undefined || row.resultSourceId === null ? row.resultSourceId : text(row.resultSourceId, "$.resultSourceId");
+  if (resultSourceId !== undefined && resultSourceId !== null) {
+    const source = decodeResultSourceIdentity(resultSourceId);
+    if (mode === "buckling" && source.kind !== "loadCase") throw new TypeError("$.resultSourceId must name a load case for buckling");
+  }
   return {
     ...parsed,

```


## 4 ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts

### HEAD
```diff
diff --git a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts
index edd668e2b55..2cd36e745d9 100644
--- a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts
+++ b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts
@@ -4,5 +4,5 @@ import assert from "node:assert/strict";
 import Ajv from "ajv/dist/2020";
 import draft7 from "ajv/dist/refs/json-schema-draft-07.json";
-import viewportSchema from "../../../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧊️3d/🧬️schema/🔣️.json";
+import { viewport3dSchema as viewportSchema } from "@semio-tech/framework/ui/viewport/3d/schema";
 import { applyPatch, type Operation } from "fast-json-patch";
 import schema from "../../🔣️.json" with { type: "json" };

```

### INCOMING
```diff
diff --git a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts
index edd668e2b55..2e1560f9e00 100644
--- a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts
+++ b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts
@@ -2,4 +2,5 @@ import resultModeSchema from "../../../../../../../../../../../../../../../⚙
 import resultsAnimationSchema from "../../../../../../../../../../../../../../../⚙️engine/🖥️app-surface/🧬️schema/🔣️results-animation/🔣️.json" with { type: "json" };
 import assert from "node:assert/strict";
+import { readFileSync } from "node:fs";
 import Ajv from "ajv/dist/2020";
 import draft7 from "ajv/dist/refs/json-schema-draft-07.json";
@@ -13,4 +14,12 @@ export function testFem3dResultsWindowConfigContract(): void {
   const validate = new Ajv({ strict: true, allErrors: true }).addMetaSchema(draft7).addSchema(viewportSchema).addSchema(resultModeSchema).addSchema(resultsAnimationSchema).compile(schema);
   const base = fixture.valid[0];
+  const sources = JSON.parse(readFileSync(new URL("../../../../../../../../🕹️interaction/🧫️fixtures/📊️result-source/🔣️.json", import.meta.url), "utf8")) as { cases: Array<{ identity: string; mode: string; admitted: boolean }> };
+  for (const row of sources.cases) {
+    const candidate = { ...structuredClone(base), resultSourceId: row.identity, resultMode: row.mode };
+    if (row.admitted) assert.equal(parseFem3dResultsWindowConfig(candidate).resultSourceId, row.identity);
+    else assert.throws(() => parseFem3dResultsWindowConfig(candidate), row.identity);
+  }
+  console.log("[DEBUG] FEM3d result sources preserve case/combination namespaces, reject raw/malformed IDs and require load cases for buckling");
+
   for (const candidate of fixture.valid) {
     assert(validate(candidate), JSON.stringify(validate.errors));
@@ -37,2 +46,3 @@ export function testFem3dResultsWindowConfigContract(): void {
   assert.deepEqual(parseFem3dResultsWindowConfig(patched), patched);
 }
+      
\ No newline at end of file

```


## 5 ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts

### HEAD
```diff
diff --git a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
index bc482996154..e5358dd3797 100644
--- a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
+++ b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
@@ -8,5 +8,5 @@ import fixture from "./🧫️fixtures/🔣️.json";
 import {fem3dSnapshotToSqliteDatabase,fem3dSnapshotFromSqliteDatabase} from "../🟦️.ts";
 import type {FemDof} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
-import {exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase,type SqliteValue} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
+import {exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase,type SqliteValue} from "@semio-tech/framework/io/sqlite-snapshot";
 const bits=(word:string)=>({bits:BigInt("0x"+word)});
 function dof(value:string):FemDof{switch(value){case"Tx":case"Ty":case"Tz":case"Rx":case"Ry":case"Rz":return value;default:throw Error("fixture DOF")}}

```

### INCOMING
```diff
diff --git a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
index bc482996154..0665e256134 100644
--- a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
+++ b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
@@ -89,4 +89,11 @@ test("fem3d independent SQLite DDL owns the full dimensional scalar and relation
 for(const word of fixture.ieee754Words)test(`fem3d canonical snapshot artifact and diff exact word ${word}`,async()=>{
   const source=state();everyWord(source,word);
+  expect(()=>artifact.parseFem3dArtifact(source)).toThrow("canonical unique DOF set required");
+  source.supports=source.supports.map(support=>({...support,fixed:[...new Set(support.fixed)]}));
+  expect(()=>artifact.parseFem3dArtifact(source)).toThrow("scoped identity segment");
+  source.nodes.forEach((row,index)=>{row.id=fixture.canonicalIdentities.nodes[index]!;});
+  expect(()=>artifact.parseFem3dArtifact(source)).toThrow("repeated entity identity");
+  source.elements.forEach((row,index)=>{row.id=fixture.canonicalIdentities.elements[index]!;});
+  source.loadCases.forEach(loadCase=>loadCase.loads.forEach((row,index)=>{row.id=fixture.canonicalIdentities.loads[index]!;}));
   for(const count of fixture.countWords){source.analysis.modalCount=BigInt(count);source.analysis.bucklingCount=BigInt(count);
    expect(snapshot.parseFemAnalysisSettings(source.analysis)).toEqual(source.analysis);
@@ -96,5 +103,9 @@ for(const word of fixture.ieee754Words)test(`fem3d canonical snapshot artifact a
    const delta=diff.parseFem3dNodesDelta({removed:[],inserted,moved:[],modified:[]});
    expect(delta.inserted).toEqual(inserted);
-   expect(diff.parseFem3dNodesDelta({removed:[],inserted:[],moved:[],modified:source.nodes.map(patch=>({id:patch.id,patch}))}).modified).toEqual(source.nodes.map(patch=>({id:patch.id,patch})));
+   expect(()=>diff.parseFem3dNodesDelta({removed:[],inserted:[],moved:[],modified:source.nodes.map(patch=>({id:patch.id,patch}))})).toThrow("unknown node field");
+   const modified=source.nodes.map(({id,...patch})=>({id,patch}));
+   const finite=(BigInt("0x"+word)&0x7ff0000000000000n)!==0x7ff0000000000000n;
+   for(const {patch} of modified){if(finite)expect(diff.parseFem3dNodePatch(patch)).toEqual(patch);else expect(()=>diff.parseFem3dNodePatch(patch)).toThrow("invalid node value");}
+   expect(diff.parseFem3dNodesDelta({removed:[],inserted:[],moved:[],modified}).modified).toEqual(modified);
    expect(await fem3dSnapshotFromSqliteDatabase(await fem3dSnapshotToSqliteDatabase(artifact.parseFem3dArtifact(source)))).toEqual(source);
   }

```


## 6 ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts

### HEAD
```diff
diff --git a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts
index 280c9cbb7a0..c8fc38c51fe 100644
--- a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts
+++ b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts
@@ -1,4 +1,4 @@
-import {parseSchemaRecord} from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
-import {type Binary64,parseBinary64} from "../../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🌱️value/🔢️ieee754/🟦️.ts";
+import {parseSchemaRecord} from "@semio-tech/framework/schema/record";
+import {type Binary64,parseBinary64} from "@semio-tech/framework/value/ieee754";
 /** 🧬️ Fem3d snapshot schema — artifact-lane fields only. */
 

```

### INCOMING
```diff
diff --git a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts
index 280c9cbb7a0..38173fc6ce8 100644
--- a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts
+++ b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts
@@ -1,2 +1,3 @@
+import {encodeScopedIdentity} from "../../../../../../../../../../../🧰️framework/🔨️modules/📡️replication/🧾️wire/🪪️scoped-identity/🟦️.ts";
 import {parseSchemaRecord} from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
 import {type Binary64,parseBinary64} from "../../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🌱️value/🔢️ieee754/🟦️.ts";
@@ -180,5 +181,5 @@ export function parseFemLoadCase(value: unknown, at = "$"): FemLoadCase {
 export function parseFem3dSnapshot(value: unknown, at = "$"): Fem3dSnapshot {
   const row = parseSchemaRecord(value, ["nodes", "elements", "materials", "sections", "supports", "loadCases", "combinations", "solids", "analysis"], at);
-  return {
+  const snapshot = {
     nodes: femFem3dSnapshotGuardArray(row["nodes"], `${at}.nodes`).map((item, index) => parseFemNode(item, `${at}.nodes[${index}]`)),
     elements: femFem3dSnapshotGuardArray(row["elements"], `${at}.elements`).map((item, index) => parseFemElement(item, `${at}.elements[${index}]`)),
@@ -191,4 +192,15 @@ export function parseFem3dSnapshot(value: unknown, at = "$"): Fem3dSnapshot {
     analysis: parseFemAnalysisSettings(row["analysis"], `${at}.analysis`),
   };
+  const seen = new Set<string>();
+  const admit = (segments: readonly string[]) => {
+    const identity = encodeScopedIdentity(segments);
+    if (seen.has(identity)) throw Error(`${at}: repeated entity identity`);
+    seen.add(identity);
+  };
+  for (const [kind, rows] of [["node", snapshot.nodes], ["element", snapshot.elements], ["solid", snapshot.solids], ["material", snapshot.materials], ["section", snapshot.sections], ["support", snapshot.supports], ["loadCase", snapshot.loadCases], ["combination", snapshot.combinations]] as const) {
+    for (const row of rows) admit([kind, row.id]);
+  }
+  for (const loadCase of snapshot.loadCases) for (const load of loadCase.loads) admit(["load", loadCase.id, load.id]);
+  return snapshot;
 }
 
@@ -258,5 +270,5 @@ export function parseFemSupport(value: unknown, at = "$"): FemSupport {
     id: femFem3dSnapshotGuardString(row["id"], `${at}.id`),
     nodeId: femFem3dSnapshotGuardString(row["nodeId"], `${at}.nodeId`),
-    fixed: femFem3dSnapshotGuardArray(row["fixed"], `${at}.fixed`).map((item, index) => femFem3dSnapshotGuardMember(item, `${at}.fixed[${index}]`, ["Tx", "Ty", "Tz", "Rx", "Ry", "Rz"] as const)),
+    fixed: parseFemFixedDofs(row["fixed"], `${at}.fixed`),
   };
 }
@@ -270,2 +282,10 @@ export function parseFemCombination(value: unknown, at = "$"): FemCombination {
   };
 }
+
+/** 🔒️ Admits a canonical unique subsequence of the authoritative DOF order. */
+export function parseFemFixedDofs(value: unknown, at = "$"): FemDof[] {
+  const order = ["Tx", "Ty", "Tz", "Rx", "Ry", "Rz"] as const;
+  const dofs = femFem3dSnapshotGuardArray(value, at).map((value, index) => femFem3dSnapshotGuardMember(value, `${at}[${index}]`, order));
+  if (dofs.some((dof, index) => index > 0 && order.indexOf(dofs[index - 1]!) >= order.indexOf(dof))) throw Error(`${at}: canonical unique DOF set required`);
+  return dofs;
+}

```


## 7 ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🟦️.ts

### HEAD
```diff
diff --git a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🟦️.ts b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🟦️.ts
index 69a531bcf88..74594ec250e 100644
--- a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🟦️.ts
+++ b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🟦️.ts
@@ -1,4 +1,4 @@
 import type { FemNode, FemDof, FemAxis, FemElement, FemMaterial, FemSection, FemSupport, FemLoad, FemLoadCase, FemSolid, FemCombination, FemAnalysisSettings } from "../📸️snapshot/🟦️.ts";
-import type {Binary64} from "../../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🌱️value/🔢️ieee754/🟦️.ts";
+import type {Binary64} from "@semio-tech/framework/value/ieee754";
 export type {FemNode,FemDof,FemAxis,FemElement,FemMaterial,FemSection,FemSupport,FemLoad,FemLoadCase,FemSolid,FemCombination,FemAnalysisSettings} from "../📸️snapshot/🟦️.ts";
 /** 🧩️ Direct mutations reuse the canonical persisted Snapshot entity types. */

```

### INCOMING
```diff
diff --git a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🟦️.ts b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🟦️.ts
index 69a531bcf88..3e088dbbc20 100644
--- a/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🟦️.ts
+++ b/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🟦️.ts
@@ -1,2 +1,65 @@
+import type { Fem3dLoadPatch } from "../🔺️diff/🟦️.ts";
+export type { Fem3dLoadPatch } from "../🔺️diff/🟦️.ts";
+/** 🩹️ Changes only supplied fields of a load with the addressed current kind. */
+export interface ChangeLoad { caseId: string; id: string; patch: Fem3dLoadPatch; }
+/** 🏷️ Owns one combination name. */
+export interface ChangeCombinationName { id: string; name: string; }
+/** ⚖️ Owns one keyed factor; null removes the term. */
+export interface ChangeCombinationTerm { id: string; caseId: string; factor: Binary64 | null; }
+
+import type { Fem3dElementPatch } from "../🔺️diff/🟦️.ts";
+export type { Fem3dElementPatch } from "../🔺️diff/🟦️.ts";
+
+/** 🩹️ Changes only supplied element fields. */
+export interface ChangeElement {
+  id: string;
+  patch: Fem3dElementPatch;
+}
+
+import type { Fem3dSupportPatch } from "../🔺️diff/🟦️.ts";
+export type { Fem3dSupportPatch } from "../🔺️diff/🟦️.ts";
+
+/** 🩹️ Changes only supplied support fields. */
+export interface ChangeSupport {
+  id: string;
+  patch: Fem3dSupportPatch;
+}
+
+import type { Fem3dSolidPatch } from "../🔺️diff/🟦️.ts";
+export type { Fem3dSolidPatch } from "../🔺️diff/🟦️.ts";
+
+/** 🩹️ Changes only supplied solid fields. */
+export interface ChangeSolid {
+  id: string;
+  patch: Fem3dSolidPatch;
+}
+
+import type { Fem3dSectionPatch } from "../🔺️diff/🟦️.ts";
+export type { Fem3dSectionPatch } from "../🔺️diff/🟦️.ts";
+
+/** 🩹️ Changes supplied section fields without replacing concurrent fields. */
+export interface ChangeSection {
+  id: string;
+  patch: Fem3dSectionPatch;
+}
+
+import type { Fem3dNodePatch } from "../🔺️diff/🟦️.ts";
+export type { Fem3dNodePatch } from "../🔺️diff/🟦️.ts";
+
+/** 🩹️ Changes supplied node fields without replacing concurrent fields. */
+export interface ChangeNode {
+  id: string;
+  patch: Fem3dNodePatch;
+}
+
+import type { Fem3dMaterialPatch } from "../🔺️diff/🟦️.ts";
+export type { Fem3dMaterialPatch } from "../🔺️diff/🟦️.ts";
+
+/** 🩹️ Changes supplied material fields without replacing concurrent fields. */
+export interface ChangeMaterial {
+  id: string;
+  patch: Fem3dMaterialPatch;
+}
+
 import type { FemNode, FemDof, FemAxis, FemElement, FemMaterial, FemSection, FemSupport, FemLoad, FemLoadCase, FemSolid, FemCombination, FemAnalysisSettings } from "../📸️snapshot/🟦️.ts";
 import type {Binary64} from "../../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🌱️value/🔢️ieee754/🟦️.ts";
@@ -200,7 +263,9 @@ export type Fem3dMutation =
   | ({ mutation: "deleteMaterial" } & DeleteMaterial)
   | ({ mutation: "replaceMaterial" } & ReplaceMaterial)
+  | ({ mutation: "changeMaterial" } & ChangeMaterial)
   | ({ mutation: "createSection" } & CreateSection)
   | ({ mutation: "deleteSection" } & DeleteSection)
   | ({ mutation: "replaceSection" } & ReplaceSection)
+  | ({ mutation: "changeSection" } & ChangeSection)
   | ({ mutation: "createSupport" } & CreateSupport)
   | ({ mutation: "deleteSupport" } & DeleteSupport)
@@ -218,6 +283,13 @@ export type Fem3dMutation =
   | ({ mutation: "updateAnalysisSettings" } & UpdateAnalysisSettings)
   | ({ mutation: "replaceNode" } & ReplaceNode)
+  | ({ mutation: "changeNode" } & ChangeNode)
   | ({ mutation: "replaceLoad" } & ReplaceLoad)
   | ({ mutation: "changeLoadCaseName" } & ChangeLoadCaseName)
   | ({ mutation: "replaceCombination" } & ReplaceCombination)
-  | ({ mutation: "moveSelection" } & MoveSelection);
+  | ({ mutation: "moveSelection" } & MoveSelection)
+  | ({ mutation: "changeSolid" } & ChangeSolid)
+  | ({ mutation: "changeSupport" } & ChangeSupport)
+  | ({ mutation: "changeElement" } & ChangeElement)
+  | ({ mutation: "changeLoad" } & ChangeLoad)
+  | ({ mutation: "changeCombinationName" } & ChangeCombinationName)
+  | ({ mutation: "changeCombinationTerm" } & ChangeCombinationTerm);

```


## 8 ✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🟦️.ts

### HEAD
```diff
diff --git a/✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🟦️.ts b/✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🟦️.ts
index dd5febf7a4e..d0b05f85a33 100644
--- a/✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🟦️.ts
+++ b/✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🟦️.ts
@@ -1,7 +1,7 @@
-import { resolveTestLevel } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
+import { resolveTestLevel } from "@semio-tech/framework/process/test-budget";
 import { existsSync, readFileSync } from "node:fs";
 import { delimiter, join, resolve } from "node:path";
 import { runCmd, runProbe, runRepositoryTestCommand } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
-import { BundleScript } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
+import { BundleScript } from "@semio-tech/framework/process/routing";
 import { ensureOracleTool, oracleEnvironment, oracleManifest, oracleToolDirectory } from "../🛠️toolchain/🟦️.ts";
 

```

### INCOMING
```diff
diff --git a/✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🟦️.ts b/✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🟦️.ts
index dd5febf7a4e..43863b681df 100644
--- a/✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🟦️.ts
+++ b/✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🟦️.ts
@@ -1,7 +1,7 @@
-import { resolveTestLevel } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
+import { resolveTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
 import { existsSync, readFileSync } from "node:fs";
 import { delimiter, join, resolve } from "node:path";
 import { runCmd, runProbe, runRepositoryTestCommand } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
-import { BundleScript } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
+import { BundleScript } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
 import { ensureOracleTool, oracleEnvironment, oracleManifest, oracleToolDirectory } from "../🛠️toolchain/🟦️.ts";
 

```


## 9 ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/⚙️systems/🟦️.ts

### HEAD
```diff
diff --git a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/⚙️systems/🟦️.ts b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/⚙️systems/🟦️.ts
index 9b9a6cb60f8..c9eca68896b 100644
--- a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/⚙️systems/🟦️.ts
+++ b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/⚙️systems/🟦️.ts
@@ -2,5 +2,5 @@
 import type {EnergyModel,EnergySetpointManagerKind} from "../../../../🧬️schema/📸️snapshot/⚡️model/🟦️.ts";
 import {columns,emit,write,ids,readIds,uint,bool,optionalId,literal,type Reader,type Options} from "../🟦️.ts";
-import type {ArtifactSqliteProjection} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
+import type {ArtifactSqliteProjection} from "@semio-tech/framework/io/sqlite-snapshot/artifact";
 const PEOPLE=columns([7,8,9,10]),LIGHTING=columns([6,7,8,9]),EQUIPMENT=columns([6,7,8]),THERMOSTAT=columns([7,8]),RESET=columns([1,2,3,4]),IDEAL=columns([5,6,7,8,9,10]),ZONE_EQUIPMENT=columns([7,8]),AIR_LOOP=columns([7]),PLANT=columns([6,7,8]),OUTDOOR=columns([5]),INFILTRATION=columns([7,8,9,10,11,12,13,14,15]),PV=columns([4,5,6,7,8,9]),BATTERY=columns([4,5,6,7]),SHW=columns([4,5,6]),SOLAR=columns([4,5,6,7,8]),REFRIGERATION=columns([5]),FAULT=columns([6]),DAYLIGHT=columns([5,6,7]);
 const EQUIPMENT_TYPES=["Baseboard","Radiant","FanCoil","Ptac","VrfTerminal","Erv","UnitHeater","WaterToAirHp"]as const,PLANT_TYPES=["Heating","Cooling","Condenser"]as const,INFILTRATION_TYPES=["ScheduledAch","PerExteriorArea","EffectiveLeakageArea","WindAndStack"]as const,FAULT_TYPES=["SensorBias","CoilFouling","DamperStuck","ChillerFouling","BoilerEfficiencyDegradation"]as const,FREQUENCIES=["Timestep","Hourly","Daily","Monthly","RunPeriod"]as const,ROOM_TYPES=["WellMixed","OneNodeDisplacement","TwoNodeBuoyancy","UnderFloorAirDistribution"]as const;

```

### INCOMING
```diff
diff --git a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/⚙️systems/🟦️.ts b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/⚙️systems/🟦️.ts
index 9b9a6cb60f8..9e99cf43551 100644
--- a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/⚙️systems/🟦️.ts
+++ b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/⚙️systems/🟦️.ts
@@ -3,5 +3,5 @@ import type {EnergyModel,EnergySetpointManagerKind} from "../../../../🧬️sch
 import {columns,emit,write,ids,readIds,uint,bool,optionalId,literal,type Reader,type Options} from "../🟦️.ts";
 import type {ArtifactSqliteProjection} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
-const PEOPLE=columns([7,8,9,10]),LIGHTING=columns([6,7,8,9]),EQUIPMENT=columns([6,7,8]),THERMOSTAT=columns([7,8]),RESET=columns([1,2,3,4]),IDEAL=columns([5,6,7,8,9,10]),ZONE_EQUIPMENT=columns([7,8]),AIR_LOOP=columns([7]),PLANT=columns([6,7,8]),OUTDOOR=columns([5]),INFILTRATION=columns([7,8,9,10,11,12,13,14,15]),PV=columns([4,5,6,7,8,9]),BATTERY=columns([4,5,6,7]),SHW=columns([4,5,6]),SOLAR=columns([4,5,6,7,8]),REFRIGERATION=columns([5]),FAULT=columns([6]),DAYLIGHT=columns([5,6,7]);
+const PEOPLE=columns([7,8,9,10]),LIGHTING=columns([6,7,8,9]),EQUIPMENT=columns([6,7,8]),THERMOSTAT=columns([7,8]),RESET=columns([1,2,3,4]),IDEAL=columns([5,6,7,8,9,10]),ZONE_EQUIPMENT=columns([7,8]),AIR_LOOP=columns([7]),PLANT=columns([6,7,8]),OUTDOOR=columns([5]),INFILTRATION=columns([7,8,9,10,11,12,13,14,15]),PV=columns([4,5,6,7,8,9,10,11,12]),BATTERY=columns([4,5,6,7]),SHW=columns([4,5,6]),SOLAR=columns([4,5,6,7,8]),REFRIGERATION=columns([5]),FAULT=columns([6]),DAYLIGHT=columns([5,6,7]);
 const EQUIPMENT_TYPES=["Baseboard","Radiant","FanCoil","Ptac","VrfTerminal","Erv","UnitHeater","WaterToAirHp"]as const,PLANT_TYPES=["Heating","Cooling","Condenser"]as const,INFILTRATION_TYPES=["ScheduledAch","PerExteriorArea","EffectiveLeakageArea","WindAndStack"]as const,FAULT_TYPES=["SensorBias","CoilFouling","DamperStuck","ChillerFouling","BoilerEfficiencyDegradation"]as const,FREQUENCIES=["Timestep","Hourly","Daily","Monthly","RunPeriod"]as const,ROOM_TYPES=["WellMixed","OneNodeDisplacement","TwoNodeBuoyancy","UnderFloorAirDistribution"]as const;
 export async function projectSystems(m:EnergyModel,p:ArtifactSqliteProjection,o:Options):Promise<void>{
@@ -21,5 +21,5 @@ export async function projectSystems(m:EnergyModel,p:ArtifactSqliteProjection,o:
  if(m.airflow_network!==null){const v=m.airflow_network;await p.insert("energy_airflow_network",[uint(v.outdoor_node_id)],1n);for(let n=0;n<v.zone_node_ids.length;n++){const pair=v.zone_node_ids[n]!;if(pair.length!==2)throw Error("Energy airflow node has exactly2components");await emit(p,"energy_airflow_zone_node",n,[uint(pair[0]),uint(pair[1])],[],o)}for(let n=0;n<v.link_ids.length;n++)await emit(p,"energy_airflow_link",n,[uint(v.link_ids[n]!)],[],o)}
  for(let n=0;n<m.electrical_load_centers.length;n++){const v=m.electrical_load_centers[n]!,id=await emit(p,"energy_electrical_load_center",n,[uint(v.id),v.name],[],o);await ids(p,"energy_electrical_generator",id,v.generator_ids,o);await ids(p,"energy_electrical_pv",id,v.pv_ids,o);await ids(p,"energy_electrical_battery",id,v.battery_ids,o)}
- for(let n=0;n<m.pv_systems.length;n++){const v=m.pv_systems[n]!;await emit(p,"energy_pv_system",n,[uint(v.id),v.dc_capacity_w,v.area_m2,v.tilt_deg,v.azimuth_deg,v.module_efficiency,v.inverter_efficiency],PV,o)}
+ for(let n=0;n<m.pv_systems.length;n++){const v=m.pv_systems[n]!;await emit(p,"energy_pv_system",n,[uint(v.id),v.dc_capacity_w,v.area_m2,v.tilt_deg,v.azimuth_deg,v.module_efficiency,v.inverter_efficiency,v.temperature_coefficient,v.faiman_heat_loss_constant_w_m2k,v.faiman_heat_loss_wind_w_s_m3k,v.mounting],PV,o)}
  for(let n=0;n<m.battery_storage.length;n++){const v=m.battery_storage[n]!;await emit(p,"energy_battery",n,[uint(v.id),v.capacity_kwh,v.max_charge_w,v.max_discharge_w,v.round_trip_efficiency],BATTERY,o)}
  for(let n=0;n<m.shw_systems.length;n++){const v=m.shw_systems[n]!;await emit(p,"energy_shw_system",n,[uint(v.id),v.heater_capacity_w,v.storage_volume_m3,v.setpoint_c,uint(v.schedule_id)],SHW,o)}
@@ -51,5 +51,5 @@ export async function reconstructSystems(r:Reader):Promise<Systems>{
  const network=await r.one("energy_airflow_network",2,[],false);if(network!==null)m.airflow_network={outdoor_node_id:network.u(1),zone_node_ids:(await r.rows("energy_airflow_zone_node",1n,5,[])).map(v=>[v.u(3),v.u(4)]),link_ids:(await r.rows("energy_airflow_link",1n,4,[])).map(v=>v.u(3))};
  for(const v of await r.rows("energy_electrical_load_center",1n,5,[]))m.electrical_load_centers.push({id:v.u(3),name:v.text(4),generator_ids:await readIds(r,"energy_electrical_generator",v.id),pv_ids:await readIds(r,"energy_electrical_pv",v.id),battery_ids:await readIds(r,"energy_electrical_battery",v.id)});
- for(const v of await r.rows("energy_pv_system",1n,10,PV))m.pv_systems.push({id:v.u(3),dc_capacity_w:v.float(4),area_m2:v.float(5),tilt_deg:v.float(6),azimuth_deg:v.float(7),module_efficiency:v.float(8),inverter_efficiency:v.float(9)});
+ for(const v of await r.rows("energy_pv_system",1n,14,PV))m.pv_systems.push({id:v.u(3),dc_capacity_w:v.float(4),area_m2:v.float(5),tilt_deg:v.float(6),azimuth_deg:v.float(7),module_efficiency:v.float(8),inverter_efficiency:v.float(9),temperature_coefficient:v.float(10),faiman_heat_loss_constant_w_m2k:v.float(11),faiman_heat_loss_wind_w_s_m3k:v.float(12),mounting:literal(v.text(13),["OpenRack","CloseRoof","InsulatedBack"]as const)});
  for(const v of await r.rows("energy_battery",1n,8,BATTERY))m.battery_storage.push({id:v.u(3),capacity_kwh:v.float(4),max_charge_w:v.float(5),max_discharge_w:v.float(6),round_trip_efficiency:v.float(7)});
  for(const v of await r.rows("energy_shw_system",1n,8,SHW))m.shw_systems.push({id:v.u(3),heater_capacity_w:v.float(4),storage_volume_m3:v.float(5),setpoint_c:v.float(6),schedule_id:v.u(7)});

```


## 10 ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts

### HEAD
```diff
diff --git a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
index d392cc322b2..3f838b73e93 100644
--- a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
+++ b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
@@ -26,5 +26,5 @@ import {fixture} from "./🧫️corpus/🟦️.ts";
 import { energyModelToSqliteDatabase, energyModelFromSqliteDatabase } from "../🟦️.ts";
 import type {EnergyModelSnapshot,EnergyModelArtifact,EnergyModelDiff} from "../../../../../../../../🟦️.ts";
-import {exportSqliteDatabase,importSqliteDatabase} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
+import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework/io/sqlite-snapshot";
 import {writeFileSync,mkdtempSync,rmSync} from "node:fs";
 import {join} from "node:path";

```

### INCOMING
```diff
diff --git a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
index d392cc322b2..823dcff1d54 100644
--- a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
+++ b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
@@ -56,5 +56,5 @@ test("Energy source semantic copy and row frontiers cancel with finite caller bu
 });
 test("Energy every populated IEEE field survives an independent SQLite edit",async()=>{
- const seed=await exportSqliteDatabase(await energyModelToSqliteDatabase(fixture()));for(const word of corpus.binary64Words){const bits=BigInt(`0x${word}`),buffer=new ArrayBuffer(8),view=new DataView(buffer);view.setBigUint64(0,bits);const value=view.getFloat64(0),kind=Number.isNaN(value)?"nan":value===Infinity?"positiveInfinity":value===-Infinity?"negativeInfinity":"finite",db=Database.deserialize(seed,{safeIntegers:true});let count=0;let bytes:Uint8Array;try{for(const table of db.query("SELECT name FROM sqlite_schema WHERE type='table'").all()as{name:string}[]){for(const c of db.query(`PRAGMA table_info(${table.name})`).all()as{name:string;type:string}[]){if(c.type!=="REAL")continue;count+=Number((db.query(`SELECT COUNT(*) AS count FROM ${table.name} WHERE ${c.name}_ieee754_bits IS NOT NULL`).get()as{count:bigint}).count);db.query(`UPDATE ${table.name} SET ${c.name}=?,${c.name}_ieee754_bits=?,${c.name}_ieee754_class=? WHERE ${c.name}_ieee754_bits IS NOT NULL`).run(Number.isNaN(value)?null:value,BigInt.asIntN(64,bits),kind)}}expect(count).toBe(173);bytes=db.serialize()}finally{db.close()}
+ const seed=await exportSqliteDatabase(await energyModelToSqliteDatabase(fixture()));for(const word of corpus.binary64Words){const bits=BigInt(`0x${word}`),buffer=new ArrayBuffer(8),view=new DataView(buffer);view.setBigUint64(0,bits);const value=view.getFloat64(0),kind=Number.isNaN(value)?"nan":value===Infinity?"positiveInfinity":value===-Infinity?"negativeInfinity":"finite",db=Database.deserialize(seed,{safeIntegers:true});let count=0;let bytes:Uint8Array;try{for(const table of db.query("SELECT name FROM sqlite_schema WHERE type='table'").all()as{name:string}[]){for(const c of db.query(`PRAGMA table_info(${table.name})`).all()as{name:string;type:string}[]){if(c.type!=="REAL")continue;count+=Number((db.query(`SELECT COUNT(*) AS count FROM ${table.name} WHERE ${c.name}_ieee754_bits IS NOT NULL`).get()as{count:bigint}).count);db.query(`UPDATE ${table.name} SET ${c.name}=?,${c.name}_ieee754_bits=?,${c.name}_ieee754_class=? WHERE ${c.name}_ieee754_bits IS NOT NULL`).run(Number.isNaN(value)?null:value,BigInt.asIntN(64,bits),kind)}}expect(count).toBe(176);bytes=db.serialize()}finally{db.close()}
  const snapshot=await energyModelFromSqliteDatabase(await importSqliteDatabase(bytes));const checked=Database.deserialize(await exportSqliteDatabase(await energyModelToSqliteDatabase(snapshot)),{safeIntegers:true});try{let actual=0;for(const table of checked.query("SELECT name FROM sqlite_schema WHERE type='table'").all()as{name:string}[]){for(const c of checked.query(`PRAGMA table_info(${table.name})`).all()as{name:string;type:string}[]){if(!c.name.endsWith("_ieee754_bits"))continue;for(const row of checked.query(`SELECT ${c.name} AS bits FROM ${table.name} WHERE ${c.name} IS NOT NULL`).all()as{bits:bigint}[]){expect(row.bits).toBe(BigInt.asIntN(64,bits));actual++}}}expect(actual).toBe(count);expect(checked.query("PRAGMA foreign_key_check").all()).toEqual([])}finally{checked.close()}}
 },30_000);
@@ -76,5 +76,5 @@ test("Energy independent complete physical semantic census authority",async()=>{
  for(const word of corpus.binary64Words){
   const bits=BigInt("0x"+word),view=new DataView(new ArrayBuffer(8));view.setBigUint64(0,bits);const value=view.getFloat64(0),kind=Number.isNaN(value)?"nan":value===Infinity?"positiveInfinity":value===-Infinity?"negativeInfinity":"finite",db=Database.deserialize(seed,{safeIntegers:true});let bytes:Uint8Array;let expected;let changed=0;
-  try{for(const {name} of db.query("SELECT name FROM sqlite_schema WHERE type='table'").all() as {name:string}[]){for(const column of db.query("PRAGMA table_info("+name+")").all() as {name:string;type:string}[]){if(column.type!=="REAL")continue;const c=column.name;changed+=Number((db.query("SELECT COUNT(*) AS count FROM "+name+" WHERE "+c+"_ieee754_bits IS NOT NULL").get() as {count:bigint}).count);db.query("UPDATE "+name+" SET "+c+"=?,"+c+"_ieee754_bits=?,"+c+"_ieee754_class=? WHERE "+c+"_ieee754_bits IS NOT NULL").run(Number.isNaN(value)?null:value,BigInt.asIntN(64,bits),kind)}}expect(changed).toBe(173);expected=energyPhysicalCensus(db);bytes=db.serialize()}finally{db.close()}
+  try{for(const {name} of db.query("SELECT name FROM sqlite_schema WHERE type='table'").all() as {name:string}[]){for(const column of db.query("PRAGMA table_info("+name+")").all() as {name:string;type:string}[]){if(column.type!=="REAL")continue;const c=column.name;changed+=Number((db.query("SELECT COUNT(*) AS count FROM "+name+" WHERE "+c+"_ieee754_bits IS NOT NULL").get() as {count:bigint}).count);db.query("UPDATE "+name+" SET "+c+"=?,"+c+"_ieee754_bits=?,"+c+"_ieee754_class=? WHERE "+c+"_ieee754_bits IS NOT NULL").run(Number.isNaN(value)?null:value,BigInt.asIntN(64,bits),kind)}}expect(changed).toBe(176);expected=energyPhysicalCensus(db);bytes=db.serialize()}finally{db.close()}
   const restored=await energyModelFromSqliteDatabase(await importSqliteDatabase(bytes)),check=Database.deserialize(await exportSqliteDatabase(await energyModelToSqliteDatabase(restored)),{safeIntegers:true});try{expect(energyPhysicalCensus(check)).toEqual(expected);cases.push({case:word,...expected})}finally{check.close()}
  }

```


## 11 ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs

### HEAD
```diff
diff --git a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
index 13c0aa01c78..4efea7041b4 100644
--- a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
+++ b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
@@ -70,5 +70,5 @@ fn sqlite_snapshot_energy_actual_bare_declared_both_native_encodings_and_exact_c
 #[semio_framework_async_macros::async_test]
 async fn sqlite_snapshot_energy_actual_declaration_typed_physical_io(){
- use semio_framework_os_kernel::io::io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot};use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("energy").label("Energy SQLite owner").version("0.0.1").package_id("semio:energy").artifact(crate::declaration().unwrap()).try_build().unwrap();let source=full_energy_fixture();let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.energy.model".into(),standard:"1".into(),subset:"*".into()};for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let mut phases=Vec::new();let bytes=async{let original_file_dialect=&dialect;let original_file_input=&source;let original_file_encoding=encoding;let original_file_limits=SqliteDatabaseLimits::default();let original_file_progress:&mut dyn FnMut(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |p|{phases.push(p.phase);true};semio_framework_os_kernel::io::testing::with_snapshot_file_decoding(semio_framework_os_kernel::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_export_sqlite_snapshot(original_file_dialect,original_file_input,original_file_encoding,sqlite_control,native_owner).await).await}.await.unwrap().value;let restored=async{let original_file_dialect=&dialect;let original_file_input=&bytes;let original_file_limits=SqliteDatabaseLimits::default();let original_file_progress:&mut dyn FnMut(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_|true;semio_framework_os_kernel::io::testing::with_snapshot_file_encoding(semio_framework_os_kernel::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_import_sqlite_snapshot::<EnergyModelSnapshot>(original_file_dialect,original_file_input,sqlite_control,native_owner).await).await}.await.unwrap().value;assert_eq!(energy_project(&restored),energy_project(&source));assert!(!phases.iter().any(|p|matches!(p,SqliteSnapshotPhase::EncodeNative|SqliteSnapshotPhase::DecodeNative)));}
+ use semio_framework_os_kernel::io::io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot};use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};semio_framework_plugin::PluginBuilder::<semio_framework_plugin::NeedsLabel, semio_framework_plugin::app::NoPluginApp>::new("energy").label("Energy SQLite owner").version("0.0.1").package_id("semio:energy").artifact(crate::declaration().unwrap()).try_build().unwrap();let source=full_energy_fixture();let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.energy.model".into(),standard:"1".into(),subset:"*".into()};for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let mut phases=Vec::new();let bytes=async{let original_file_dialect=&dialect;let original_file_input=&source;let original_file_encoding=encoding;let original_file_limits=SqliteDatabaseLimits::default();let original_file_progress:&mut dyn FnMut(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |p|{phases.push(p.phase);true};semio_framework_os_kernel::io::testing::with_snapshot_file_decoding(semio_framework_os_kernel::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_export_sqlite_snapshot(original_file_dialect,original_file_input,original_file_encoding,sqlite_control,native_owner).await).await}.await.unwrap().value;let restored=async{let original_file_dialect=&dialect;let original_file_input=&bytes;let original_file_limits=SqliteDatabaseLimits::default();let original_file_progress:&mut dyn FnMut(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_|true;semio_framework_os_kernel::io::testing::with_snapshot_file_encoding(semio_framework_os_kernel::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_import_sqlite_snapshot::<EnergyModelSnapshot>(original_file_dialect,original_file_input,sqlite_control,native_owner).await).await}.await.unwrap().value;assert_eq!(energy_project(&restored),energy_project(&source));assert!(!phases.iter().any(|p|matches!(p,SqliteSnapshotPhase::EncodeNative|SqliteSnapshotPhase::DecodeNative)));}
 }
 #[test]

```

### INCOMING
```diff
diff --git a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
index 13c0aa01c78..f3d5f1faf6f 100644
--- a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
+++ b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
@@ -75,5 +75,5 @@ async fn sqlite_snapshot_energy_actual_declaration_typed_physical_io(){
 fn sqlite_snapshot_energy_independent_engine_edits_every_ieee_field_then_erased_roundtrips(){
  use std::io::Write;use std::process::{Command,Stdio};use store::sqlite_snapshot::{export_sqlite_database,import_sqlite_database,SnapshotEncoding};let corpus:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let bytes=export_sqlite_database(&energy_project(&full_energy_fixture()),SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let provider=store::bare_artifact_codec::<EnergyModelSnapshot,crate::mutations::EnergyModelMutation>(crate::snapshot::ENERGY_MODEL_DOCUMENT_SCHEMA).snapshot_sqlite.unwrap();let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.energy.model".into(),standard:"1".into(),subset:"*".into()};
- let script="import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true}),bits=BigInt('0x'+process.env.SEMIO_TEST_WORD),signed=BigInt.asIntN(64,bits),b=new ArrayBuffer(8),v=new DataView(b);v.setBigUint64(0,bits);const number=v.getFloat64(0),kind=Number.isNaN(number)?'nan':number===Infinity?'positiveInfinity':number===-Infinity?'negativeInfinity':'finite';let count=0;for(const table of d.query(\"SELECT name FROM sqlite_schema WHERE type='table'\").all()){for(const column of d.query('PRAGMA table_info('+table.name+')').all()){if(column.type!=='REAL')continue;const c=column.name,n=Number(d.query('SELECT COUNT(*) AS n FROM '+table.name+' WHERE '+c+'_ieee754_bits IS NOT NULL').get().n);count+=n;d.query('UPDATE '+table.name+' SET '+c+'=?,'+c+'_ieee754_bits=?,'+c+'_ieee754_class=? WHERE '+c+'_ieee754_bits IS NOT NULL').run(Number.isNaN(number)?null:number,signed,kind);for(const row of d.query('SELECT '+c+'_ieee754_bits AS bits FROM '+table.name+' WHERE '+c+'_ieee754_bits IS NOT NULL').all())if(row.bits!==signed)throw Error('exact IEEE companion');}}if(count!==173)throw Error('all173 Energy words count '+count);if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,d.serialize());d.close();";
+ let script="import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true}),bits=BigInt('0x'+process.env.SEMIO_TEST_WORD),signed=BigInt.asIntN(64,bits),b=new ArrayBuffer(8),v=new DataView(b);v.setBigUint64(0,bits);const number=v.getFloat64(0),kind=Number.isNaN(number)?'nan':number===Infinity?'positiveInfinity':number===-Infinity?'negativeInfinity':'finite';let count=0;for(const table of d.query(\"SELECT name FROM sqlite_schema WHERE type='table'\").all()){for(const column of d.query('PRAGMA table_info('+table.name+')').all()){if(column.type!=='REAL')continue;const c=column.name,n=Number(d.query('SELECT COUNT(*) AS n FROM '+table.name+' WHERE '+c+'_ieee754_bits IS NOT NULL').get().n);count+=n;d.query('UPDATE '+table.name+' SET '+c+'=?,'+c+'_ieee754_bits=?,'+c+'_ieee754_class=? WHERE '+c+'_ieee754_bits IS NOT NULL').run(Number.isNaN(number)?null:number,signed,kind);for(const row of d.query('SELECT '+c+'_ieee754_bits AS bits FROM '+table.name+' WHERE '+c+'_ieee754_bits IS NOT NULL').all())if(row.bits!==signed)throw Error('exact IEEE companion');}}if(count!==176)throw Error('all176 Energy words count '+count);if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,d.serialize());d.close();";
  for word in corpus["binary64Words"].as_array().unwrap(){let mut child=Command::new("bun").args(["-e",script]).env("SEMIO_TEST_WORD",word.as_str().unwrap()).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let db=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let source=original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|sqlite_control|EnergyModelSnapshot::from_sqlite_database(&db, sqlite_control)).unwrap();assert_eq!(energy_project(&source),db);energy_closed_extent(&source,word.as_str().unwrap());for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=original_owners::with_encoding(|native_owner|source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap();let exported=original_owners::with_decoding(|native_owner|(provider.export)(crate::snapshot::ENERGY_MODEL_DOCUMENT_SCHEMA,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap().value;assert_eq!(exported,db);assert_eq!(original_owners::with_encoding(|native_owner|(provider.import)(crate::snapshot::ENERGY_MODEL_DOCUMENT_SCHEMA,&dialect, &mut Some(exported),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap().value,payload);}}
 }
@@ -116,2 +116,6 @@ fn sqlite_snapshot_energy_closed_complete_semantic_extent(){
  let source=full_energy_fixture();energy_closed_extent(&source,"authored");
 }
+
+#[path="../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🧪️tests/🪶️sqlite/🫴️native-caller/🦀️.rs"]
+mod native_caller;
+use native_caller::{native_caller_policy,with_original_decode,with_original_encode};

```


## 12 ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️corpus/🟦️.ts

### HEAD
```diff
diff --git a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️corpus/🟦️.ts b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️corpus/🟦️.ts
index a6fb8f88bd9..95a6c351e62 100644
--- a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️corpus/🟦️.ts
+++ b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️corpus/🟦️.ts
@@ -1,5 +1,5 @@
 /** 🧫️ Explicit test-only ownership of the independent finite Serde fixture. */
 import corpus from "../🧫️fixtures/🔣️.json";
-import {binary64} from "../../../../../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🌱️value/🔢️ieee754/🟦️.ts";
+import {binary64} from "@semio-tech/framework/value/ieee754";
 import type {EnergyModelSnapshot,EnergyVertex} from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
 const f=binary64;

```

### INCOMING
```diff
diff --git a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️corpus/🟦️.ts b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️corpus/🟦️.ts
index a6fb8f88bd9..2f71495fde7 100644
--- a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️corpus/🟦️.ts
+++ b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️corpus/🟦️.ts
@@ -3,5 +3,7 @@ import corpus from "../🧫️fixtures/🔣️.json";
 import {binary64} from "../../../../../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🌱️value/🔢️ieee754/🟦️.ts";
 import type {EnergyModelSnapshot,EnergyVertex} from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
+import type {EnergyPhotovoltaicMounting} from "../../../../../🧬️schema/📸️snapshot/⚡️model/🟦️.ts";
 const f=binary64;
+function photovoltaicMounting(value:string):EnergyPhotovoltaicMounting{if(value==="OpenRack"||value==="CloseRoof"||value==="InsulatedBack")return value;throw Error("fixture PV mounting differs")} 
 function vertex(v:readonly number[]):EnergyVertex{if(v.length!==3)throw Error("fixture vertex");return[f(v[0]!),f(v[1]!),f(v[2]!)]}
 export function fixture():EnergyModelSnapshot{
@@ -20,5 +22,5 @@ export function fixture():EnergyModelSnapshot{
  infiltrations:m.infiltrations.map(v=>({...v,method:"WindAndStack",design_flow_ach:f(v.design_flow_ach),flow_per_exterior_area_m3_s_m2:f(v.flow_per_exterior_area_m3_s_m2),effective_leakage_area_m2:f(v.effective_leakage_area_m2),discharge_coefficient:f(v.discharge_coefficient),stack_height_m:f(v.stack_height_m),constant_term_coefficient:f(v.constant_term_coefficient),temperature_term_coefficient:f(v.temperature_term_coefficient),velocity_term_coefficient:f(v.velocity_term_coefficient),velocity_squared_term_coefficient:f(v.velocity_squared_term_coefficient)})),
  mechanical_ventilations:m.mechanical_ventilations.map(v=>({...v,design_flow_m3_s:f(v.design_flow_m3_s),fan_total_efficiency:f(v.fan_total_efficiency),fan_delta_pressure_pa:f(v.fan_delta_pressure_pa)})),shading_surfaces:m.shading_surfaces.map(v=>({...v,vertices_m:v.vertices_m.map(vertex)})),airflow_network:{...m.airflow_network,zone_node_ids:m.airflow_network.zone_node_ids.map(v=>[v[0]!,v[1]!])},
- pv_systems:m.pv_systems.map(v=>({...v,dc_capacity_w:f(v.dc_capacity_w),area_m2:f(v.area_m2),tilt_deg:f(v.tilt_deg),azimuth_deg:f(v.azimuth_deg),module_efficiency:f(v.module_efficiency),inverter_efficiency:f(v.inverter_efficiency)})),battery_storage:m.battery_storage.map(v=>({...v,capacity_kwh:f(v.capacity_kwh),max_charge_w:f(v.max_charge_w),max_discharge_w:f(v.max_discharge_w),round_trip_efficiency:f(v.round_trip_efficiency)})),shw_systems:m.shw_systems.map(v=>({...v,heater_capacity_w:f(v.heater_capacity_w),storage_volume_m3:f(v.storage_volume_m3),setpoint_c:f(v.setpoint_c)})),solar_thermal_systems:m.solar_thermal_systems.map(v=>({...v,collector_area_m2:f(v.collector_area_m2),efficiency:f(v.efficiency),storage_volume_m3:f(v.storage_volume_m3),tilt_deg:f(v.tilt_deg),azimuth_deg:f(v.azimuth_deg)})),
+ pv_systems:m.pv_systems.map(v=>({...v,mounting:photovoltaicMounting(v.mounting),dc_capacity_w:f(v.dc_capacity_w),area_m2:f(v.area_m2),tilt_deg:f(v.tilt_deg),azimuth_deg:f(v.azimuth_deg),module_efficiency:f(v.module_efficiency),inverter_efficiency:f(v.inverter_efficiency),temperature_coefficient:f(v.temperature_coefficient),faiman_heat_loss_constant_w_m2k:f(v.faiman_heat_loss_constant_w_m2k),faiman_heat_loss_wind_w_s_m3k:f(v.faiman_heat_loss_wind_w_s_m3k)})),battery_storage:m.battery_storage.map(v=>({...v,capacity_kwh:f(v.capacity_kwh),max_charge_w:f(v.max_charge_w),max_discharge_w:f(v.max_discharge_w),round_trip_efficiency:f(v.round_trip_efficiency)})),shw_systems:m.shw_systems.map(v=>({...v,heater_capacity_w:f(v.heater_capacity_w),storage_volume_m3:f(v.storage_volume_m3),setpoint_c:f(v.setpoint_c)})),solar_thermal_systems:m.solar_thermal_systems.map(v=>({...v,collector_area_m2:f(v.collector_area_m2),efficiency:f(v.efficiency),storage_volume_m3:f(v.storage_volume_m3),tilt_deg:f(v.tilt_deg),azimuth_deg:f(v.azimuth_deg)})),
  refrigeration_systems:m.refrigeration_systems.map(v=>({...v,design_load_w:f(v.design_load_w)})),water_systems:m.water_systems.map(v=>({...v,peak_flow_l_s:f(v.peak_flow_l_s)})),faults:m.faults.map(v=>({...v,fault_type:"BoilerEfficiencyDegradation",severity:f(v.severity)})),output_variables:m.output_variables.map(v=>({...v,reporting_frequency:"RunPeriod"})),sizing_objects:m.sizing_objects.map(v=>({...v,sizing_type:"OutdoorAir",design_day_type:"Heating"})),daylight_zones:m.daylight_zones.map(v=>({...v,illuminance_target_lux:f(v.illuminance_target_lux),glare_limit:f(v.glare_limit),window_transmittance:f(v.window_transmittance)})),room_air_models:m.room_air_models.map(v=>({...v,model:"UnderFloorAirDistribution"})),
  schedules:{constants:m.schedules.constants.map(v=>({...v,value:f(v.value)})),daily:m.schedules.daily.map(v=>({...v,interpolation:"Discrete",hourly_values:v.hourly_values.map(f),limits:{min:f(v.limits.min),max:f(v.limits.max)}})),weekly:m.schedules.weekly,annual:m.schedules.annual.map(v=>({...v,holiday_dates:v.holiday_dates.map(v=>[v[0]!,v[1]!,v[2]!])})),time_series:m.schedules.time_series.map(v=>({...v,values:v.values.map(f)}))}

```


## 13 ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/⚡️model/🟦️.ts

### HEAD
```diff
diff --git a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/⚡️model/🟦️.ts b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/⚡️model/🟦️.ts
index 18d78f14fc9..9c7d4fb29a6 100644
--- a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/⚡️model/🟦️.ts
+++ b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/⚡️model/🟦️.ts
@@ -1,4 +1,4 @@
 /** ⚡️ Complete persisted Energy model scalar and relationship ownership. */
-import type {Binary64} from "../../../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🌱️value/🔢️ieee754/🟦️.ts";
+import type {Binary64} from "@semio-tech/framework/value/ieee754";
 export type EnergyFloat=Binary64;
 export type EnergyVertex=readonly[EnergyFloat,EnergyFloat,EnergyFloat];

```

### INCOMING
```diff
diff --git a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/⚡️model/🟦️.ts b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/⚡️model/🟦️.ts
index 18d78f14fc9..e8c80b84f0f 100644
--- a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/⚡️model/🟦️.ts
+++ b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/⚡️model/🟦️.ts
@@ -35,5 +35,6 @@ export interface EnergyAdjacencyPair{surface_a_id:number;surface_b_id:number}
 export interface EnergyAirflowNetwork{zone_node_ids:[number,number][];outdoor_node_id:number;link_ids:number[]}
 export interface EnergyElectricalLoadCenter{id:number;name:string;generator_ids:number[];pv_ids:number[];battery_ids:number[]}
-export interface EnergyPvSystem{id:number;dc_capacity_w:EnergyFloat;area_m2:EnergyFloat;tilt_deg:EnergyFloat;azimuth_deg:EnergyFloat;module_efficiency:EnergyFloat;inverter_efficiency:EnergyFloat}
+export type EnergyPhotovoltaicMounting="OpenRack"|"CloseRoof"|"InsulatedBack";
+export interface EnergyPvSystem{temperature_coefficient:EnergyFloat;faiman_heat_loss_constant_w_m2k:EnergyFloat;faiman_heat_loss_wind_w_s_m3k:EnergyFloat;mounting:EnergyPhotovoltaicMounting;id:number;dc_capacity_w:EnergyFloat;area_m2:EnergyFloat;tilt_deg:EnergyFloat;azimuth_deg:EnergyFloat;module_efficiency:EnergyFloat;inverter_efficiency:EnergyFloat}
 export interface EnergyBattery{id:number;capacity_kwh:EnergyFloat;max_charge_w:EnergyFloat;max_discharge_w:EnergyFloat;round_trip_efficiency:EnergyFloat}
 export interface EnergyShwSystem{id:number;heater_capacity_w:EnergyFloat;storage_volume_m3:EnergyFloat;setpoint_c:EnergyFloat;schedule_id:number}

```


## 14 ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts

### HEAD
```diff
diff --git a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts
index 18e7110a12d..d491c5bf805 100644
--- a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts
+++ b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts
@@ -1,3 +1,3 @@
-import {parseArtifactRef,type ArtifactRef} from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🟦️.ts";
+import {parseArtifactRef,type ArtifactRef} from "@semio-tech/framework/schema/artifact-reference";
 /** 🧬️ Energy-model mutation vocabulary — TypeScript twin of `🧬️mutations/🦀️.rs`.
  *

```

### INCOMING
```diff
diff --git a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts
index 18e7110a12d..d81d8a38e11 100644
--- a/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts
+++ b/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts
@@ -1596,4 +1596,8 @@ export interface CreatePvSystem {
   readonly moduleEfficiency: number;
   readonly inverterEfficiency: number;
+  readonly temperatureCoefficient: number;
+  readonly faimanHeatLossConstantWM2k: number;
+  readonly faimanHeatLossWindWSM3k: number;
+  readonly mounting: "OpenRack"|"CloseRoof"|"InsulatedBack";
 }
 
@@ -2246,4 +2250,19 @@ export interface RenameGasMaterial {
 }
 
+/** 🔅️ An explicit solar front reflectance field intent. */
+export interface ChangeGlazingMaterialSolarReflectanceFront{readonly mutation:"changeGlazingMaterialSolarReflectanceFront";readonly id:number;readonly newSolarReflectanceFront:number}
+
+/** 🔆️ An explicit solar back reflectance field intent. */
+export interface ChangeGlazingMaterialSolarReflectanceBack{readonly mutation:"changeGlazingMaterialSolarReflectanceBack";readonly id:number;readonly newSolarReflectanceBack:number}
+
+/** 🔎️ An explicit visible front reflectance field intent. */
+export interface ChangeGlazingMaterialVisibleReflectanceFront{readonly mutation:"changeGlazingMaterialVisibleReflectanceFront";readonly id:number;readonly newVisibleReflectanceFront:number}
+
+/** 🪞️ An explicit visible back reflectance field intent. */
+export interface ChangeGlazingMaterialVisibleReflectanceBack{readonly mutation:"changeGlazingMaterialVisibleReflectanceBack";readonly id:number;readonly newVisibleReflectanceBack:number}
+
+/** 🌡️ An explicit infrared transmittance field intent. */
+export interface ChangeGlazingMaterialInfraredTransmittance{readonly mutation:"changeGlazingMaterialInfraredTransmittance";readonly id:number;readonly newInfraredTransmittance:number}
+
 export type EnergyModelMutation =
   | RenameModel
@@ -2543,6 +2562,20 @@ export type EnergyModelMutation =
   | ChangeGasMaterialGas
   | RenameGasMaterial
-  | ChangeMaterialRoughness;
+  | ChangeMaterialRoughness
+  | ChangeGlazingMaterialSolarReflectanceFront
+  | ChangeGlazingMaterialSolarReflectanceBack
+  | ChangeGlazingMaterialVisibleReflectanceFront
+  | ChangeGlazingMaterialVisibleReflectanceBack
+  | ChangeGlazingMaterialInfraredTransmittance
+  | ChangePvSystemTemperatureCoefficient
+  | ChangePvSystemFaimanHeatLossConstant
+  | ChangePvSystemFaimanHeatLossWind
+  | ChangePvSystemMounting;
 
 /** 🪪️ Admits the owned link target without native URI interpretation. */
 export function parseEnergyReferenceTarget(value:unknown):ArtifactRef{const target=parseArtifactRef(value);if(target.artifactId===""||Object.values(target.dialect).some(value=>value===""))throw new TypeError("Energy link identity components must be nonempty");return target;}
+
+export interface ChangePvSystemTemperatureCoefficient { readonly mutation:"changePvSystemTemperatureCoefficient"; readonly id:number; readonly newTemperatureCoefficient:number }
+export interface ChangePvSystemFaimanHeatLossConstant { readonly mutation:"changePvSystemFaimanHeatLossConstant"; readonly id:number; readonly newFaimanHeatLossConstantWM2k:number }
+export interface ChangePvSystemFaimanHeatLossWind { readonly mutation:"changePvSystemFaimanHeatLossWind"; readonly id:number; readonly newFaimanHeatLossWindWSM3k:number }
+export interface ChangePvSystemMounting { readonly mutation:"changePvSystemMounting"; readonly id:number; readonly newMounting:"OpenRack"|"CloseRoof"|"InsulatedBack" }

```


## 15 ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts

### HEAD
```diff
diff --git a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
index e9cc127b638..9c9b5502986 100644
--- a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
+++ b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
@@ -11,7 +11,7 @@ import nativeSchema from"../../../../🧬️schema/📸️snapshot/🔣️.json"
 import artifactSchema from"../../../../🧬️schema/🔣️.json";
 import * as owner from"../../../../🧬️schema/📸️snapshot/🟦️.ts";
-import {binary64,type Binary64} from "../../../../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🌱️value/🔢️ieee754/🟦️.ts";
-import{exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
-import type{ArtifactSqliteOptions}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
+import {binary64,type Binary64} from "@semio-tech/framework/value/ieee754";
+import{exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase}from"@semio-tech/framework/io/sqlite-snapshot";
+import type{ArtifactSqliteOptions}from"@semio-tech/framework/io/sqlite-snapshot/artifact";
 type Handle=Omit<owner.Puzzle2dHandle,"angle"|"radius"|"scale">&{angle:Binary64;radius?:Binary64;scale?:Binary64};
 type Node=Omit<owner.Puzzle2dNode,"x"|"y"|"radius"|"width"|"height"|"scale"|"handles">&{x:Binary64;y:Binary64;radius?:Binary64;width?:Binary64;height?:Binary64;scale?:Binary64;handles:Handle[]};

```

### INCOMING
```diff
diff --git a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
index e9cc127b638..c8a0af923a8 100644
--- a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
+++ b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts
@@ -86,5 +86,5 @@ test("Puzzle2d native snapshot ownership has retained clone and retirement dispa
  const corpus=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/📸️snapshot/🧬️retained-clone/🔣️.json",import.meta.url)).json() as {recordOwners:string[];journeys:string[];control:{maximumItems:number}};
  expect(corpus.control.maximumItems).toBe(1);expect(corpus.journeys).toEqual(["empty","nested","largeUtf8","zeroGrant","cancelDuringCopy","takeOnce","boundedClose"]);
- const source=await Bun.file(new URL("../../../../../../../../🦀️.rs",import.meta.url)).text()+await Bun.file(new URL("../../../../🧬️schema/📸️snapshot/🦀️.rs",import.meta.url)).text();
+ const source=await Bun.file(new URL("../../../../🧬️schema/🦀️.rs",import.meta.url)).text()+await Bun.file(new URL("../../../../🧬️schema/📸️snapshot/🦀️.rs",import.meta.url)).text();
  for(const name of corpus.recordOwners)for(const role of ["RetainedClone","RetireOwned"])expect(source).toMatch(new RegExp("#\\[derive\\([^\\]]*semio_framework_value::"+role+"[^\\]]*\\)\\][^#]*?(?:#\\[[^\\]]*\\][^#]*?)*pub (?:struct|enum) "+name+"\\b"));
 });
@@ -94,5 +94,5 @@ test("Puzzle2d borrowed mutation ownership has exhaustive native clone and retir
  const corpus=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/📸️snapshot/🧬️retained-clone/🔣️.json",import.meta.url)).json() as {mutationOwners:string[];borrowedJourneys:string[]};
  expect(corpus.mutationOwners.length).toBe(37);expect(corpus.borrowedJourneys).toEqual(["allAuthoredLeaves","zeroGrant","sourceSwap","cancelDuringCopy","largeUtf8","takeOnce","boundedClose"]);
- const directory=new URL("../../../../🧬️schema/🧬️mutations/",import.meta.url),files=Array.from(new Bun.Glob("**/🦀️.rs").scanSync({cwd:fileURLToPath(directory)})).filter(path=>path==="🦀️.rs"||path.split("/").length===2);
+ const directory=new URL("../../../../🧬️schema/🧬️mutations/",import.meta.url),files=Array.from(new Bun.Glob("**/🦀️.rs").scanSync({cwd:fileURLToPath(directory)})).map(path=>path.replaceAll("\\","/")).filter(path=>path==="🦀️.rs"||path.split("/").length===3);
  const source=(await Promise.all(files.map(path=>Bun.file(new URL(path,directory)).text()))).join("\n");
  for(const name of corpus.mutationOwners)for(const role of ["RetainedClone","RetireOwned"])expect(source).toMatch(new RegExp("#\\[derive\\([^\\]]*semio_framework_value::"+role+"[^\\]]*\\)\\][^#]*?(?:#\\[[^\\]]*\\][^#]*?)*pub (?:struct|enum) "+name+"\\b"));
@@ -105,5 +105,5 @@ test("Puzzle2d physical text and arrays preserve the authored semantic shapes wi
  expect(corpus.physicalOwners.journeys).toEqual(["largeText","largeNodes","nestedHandles","nestedCatalogs","tinyGrant","zeroGrant","cancelDuringCopy","boundedClose"]);
  const validate=new Ajv({strict:false,validateFormats:false}).addSchema(artifactSchema).compile(nativeSchema);expect(validate(fixture.snapshot)).toBe(true);expect(JSON.parse(JSON.stringify(fixture.snapshot))).toEqual(fixture.snapshot);
- const source=await Bun.file(new URL("../../../../../../../../🦀️.rs",import.meta.url)).text()+await Bun.file(new URL("../../../../🧬️schema/📸️snapshot/🦀️.rs",import.meta.url)).text();
+ const source=await Bun.file(new URL("../../../../🧬️schema/🦀️.rs",import.meta.url)).text()+await Bun.file(new URL("../../../../🧬️schema/📸️snapshot/🦀️.rs",import.meta.url)).text();
  expect(source).toContain("PagedUtf8<{ usize::MAX }>");expect(source).toContain("PagedList<Puzzle2dNode, { usize::MAX }>");expect(source).toContain("PagedList<Puzzle2dHandle, { usize::MAX }>");expect(source).toContain("PagedList<Puzzle2dTargetRegion, { usize::MAX }>");
  const document=source.slice(source.indexOf("pub struct Puzzle2dCamera"),source.indexOf("//#endregion 🔖️Document"));expect(document).not.toMatch(/pub \w+: (?:String|Vec<|Option<String>)/);
@@ -146,5 +146,5 @@ test("Puzzle2d native forward ownership agrees with independent RFC6902 first-ma
 
 test("Puzzle2d borrowed create preparation retains native invariant priority and RFC6902 placement",async()=>{
- const root=new URL("../../../../🧬️schema/🧬️mutations/🟢️node/🌱create-node/🎮️prepare/",import.meta.url),corpus=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🪢️connect-handles/🎮️prepare/🔣️.json",root)).json();
+ const root=new URL("../../../../🧬️schema/🧬️mutations/🟢️node/🌱create-node/🎮️prepare/",import.meta.url),corpus=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🌱create-node/🎮️prepare/🔣️.json",import.meta.url)).json();
  const text=(value:string)=>value.startsWith("$large:")?corpus.control.largePrefix.repeat(corpus.control.largeRepeats)+value.slice(7):value;
  const number={type:"number"},positive={type:"number",exclusiveMinimum:0};
@@ -166,5 +166,5 @@ test("Puzzle2d borrowed create preparation retains native invariant priority and
 
 test("Puzzle2d create inverse retains unconditional literal intent under native paged ownership",async()=>{
- const root=new URL("../../../../🧬️schema/🧬️mutations/🟢️node/🌱create-node/🎮️prepare/",import.meta.url),corpus=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🪢️connect-handles/🎮️prepare/🔣️.json",root)).json();
+ const root=new URL("../../../../🧬️schema/🧬️mutations/🟢️node/🌱create-node/🎮️prepare/",import.meta.url),corpus=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🌱create-node/🎮️prepare/🔣️.json",import.meta.url)).json();
  expect(corpus.inverseOwnership).toEqual({maximumItems:1,maximumTurnBytes:4096,cancelAfterBodyBytes:128,journeys:["inverseBeforeRefusal","duplicateInverseIntent","borrowedIdOnly","largeUtf8","zeroGrant","sourceSwap","cancelDuringIdCopy","takeOnce","controlledRetirement"]});
  const text=(value:string)=>value.startsWith("$large:")?corpus.control.largePrefix.repeat(corpus.control.largeRepeats)+value.slice(7):value;
@@ -172,5 +172,5 @@ test("Puzzle2d create inverse retains unconditional literal intent under native
   if(row.status==="changed")expect(applyPatch(row.after.map(text),[{op:"remove",path:`/${row.position}`}]).newDocument).toEqual(row.nodes.map(text));
  }
- const source=await Bun.file(new URL("↩️inverse/🦀️.rs",root)).text();expect(source).toContain("pub type Puzzle2dCreateNodeInverseCursor");expect(source).toContain("fn inverse_id(&self)");expect(source).toContain("&self.node.id");const owner=await Bun.file(new URL("../../🏷️literal-id-inverse/🦀️.rs",root)).text();expect(owner).toContain("mutation.project(1, M::inverse_id)");expect(owner).toContain("PagedList<Puzzle2dMutation");expect(owner).toContain("ControlledRetirement");expect(owner).not.toMatch(/to_string_owner|\.clone\(|format!|serde_json|owned_retirement|Vec</);expect(source).not.toMatch(/to_string_owner|\.clone\(|format!|serde_json|owned_retirement|Vec</);
+ const source=await Bun.file(new URL("↩️inverse/🦀️.rs",root)).text();expect(source).toContain("pub type Puzzle2dCreateNodeInverseCursor");expect(source).toContain("fn inverse_id(&self)");expect(source).toContain("&self.node.id");const owner=await Bun.file(new URL("../../../../🧬️schema/🧬️mutations/🏷️literal-id-inverse/🦀️.rs",import.meta.url)).text();expect(owner).toContain("mutation.project(1, M::inverse_id)");expect(owner).toContain("PagedList<Puzzle2dMutation");expect(owner).toContain("ControlledRetirement");expect(owner).not.toMatch(/to_string_owner|\.clone\(|format!|serde_json|owned_retirement|Vec</);expect(source).not.toMatch(/to_string_owner|\.clone\(|format!|serde_json|owned_retirement|Vec</);
  console.log("[DEBUG] Puzzle2d CreateNode inverse preserves sixteen unconditional typed intents against SQLite and changed-case RFC6902 restoration");
 });
@@ -178,5 +178,5 @@ test("Puzzle2d create inverse retains unconditional literal intent under native
 
 test("Puzzle2d create candidate preserves independent ordered insertion and every untouched owner",async()=>{
- const root=new URL("../../../../🧬️schema/🧬️mutations/🟢️node/🌱create-node/🎮️prepare/",import.meta.url),corpus=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🪢️connect-handles/🎮️prepare/🔣️.json",root)).json();
+ const root=new URL("../../../../🧬️schema/🧬️mutations/🟢️node/🌱create-node/🎮️prepare/",import.meta.url),corpus=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🌱create-node/🎮️prepare/🔣️.json",import.meta.url)).json();
  expect(corpus.forwardOwnership).toEqual({maximumItems:1,maximumTurnBytes:4096,cancelAfterBodyBytes:128,changedOwner:"nativePagedSnapshot",unchangedOwner:"retainedOriginal",journeys:["append","indexedInsertion","clampedInsertion","refusalRetainsOriginal","borrowedNodeOnly","nestedHandles","allUntouchedOwners","zeroGrant","cancelDuringSnapshotCopy","cancelDuringNodeCopy","cancelDuringInsertion","takeOnce","controlledRetirement"]});
  const text=(value:string)=>value.startsWith("$large:")?corpus.control.largePrefix.repeat(corpus.control.largeRepeats)+value.slice(7):value;
@@ -187,5 +187,5 @@ test("Puzzle2d create candidate preserves independent ordered insertion and ever
 
 test("Puzzle2d borrowed delete cascade streams exact first-node and ordered edge descriptors",async()=>{
- const root=new URL("../../../../🧬️schema/🧬️mutations/🟢️node/🗑️delete-node/🎮️prepare/",import.meta.url),corpus=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🪢️connect-handles/🎮️prepare/🔣️.json",root)).json();
+ const root=new URL("../../../../🧬️schema/🧬️mutations/🟢️node/🗑️delete-node/🎮️prepare/",import.meta.url),corpus=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-node/🎮️prepare/🔣️.json",import.meta.url)).json();
  expect(corpus.journeys).toEqual(["firstNodeOnly","sourceEndpoint","targetEndpoint","selfEdgeOnce","originalEdgeOrder","missingNode","emptyHandles","largeUtf8","zeroGrant","sourceSwap","cancelDuringComparison","boundedClose"]);
  const text=(value:string)=>value.startsWith("$large:")?corpus.control.largePrefix.repeat(corpus.control.largeRepeats)+value.slice(7):value;
@@ -209,5 +209,5 @@ test("Puzzle2d borrowed delete cascade streams exact first-node and ordered edge
 
 test("Puzzle2d edge restoration preserves original ordinals and all optional flags",async()=>{
- const root=new URL("../../../../🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/↩️inverse/📑️ordered-restoration/",import.meta.url),corpus=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🪢️connect-handles/🎮️prepare/🔣️.json",root)).json();
+ const root=new URL("../../../../🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/↩️inverse/📑️ordered-restoration/",import.meta.url),corpus=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🪢️connect-handles/↩️inverse/📑️ordered-restoration/🔣️.json",import.meta.url)).json();
  for(const row of corpus.cases){const db=new Database(":memory:");try{
   db.run("CREATE TABLE edges(ordinal INTEGER PRIMARY KEY,record TEXT NOT NULL,source TEXT NOT NULL,target TEXT NOT NULL)");row.edges.forEach((edge:any,index:number)=>db.run("INSERT INTO edges VALUES(?,?,?,?)",[index,JSON.stringify(edge),edge.source,edge.target]));
@@ -219,5 +219,5 @@ test("Puzzle2d edge restoration preserves original ordinals and all optional fla
  for(const row of corpus.insertions){const index=row.index===null?row.before.length:Math.min(row.index,row.before.length);expect(applyPatch([...row.before],[{op:"add",path:`/${index}`,value:"new"}]).newDocument).toEqual(row.after)}
  const schema=await Bun.file(new URL("../../../../🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🧬️schema/🔣️.json",import.meta.url)).json();expect(schema.properties.index.type).toEqual(["integer","null"]);expect(schema.properties.index.minimum).toBe(0);
- const source=await Bun.file(new URL("../../../../🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🦀️.rs",import.meta.url)).text();expect(source).toContain("pub index: Option<usize>");expect(source).toContain("pub(crate) fn restore_edge");
+ const source=await Bun.file(new URL("../../../../🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🦀️.rs",import.meta.url)).text();expect(source).toContain("pub index: Option<usize>");expect(source).toContain("pub fn restore_edge<");
  console.log("[DEBUG] Puzzle2d exact edge order and optional flag restoration agree with six SQLite/RFC6902 cascades and six insertion cases");
 });
@@ -225,5 +225,5 @@ test("Puzzle2d edge restoration preserves original ordinals and all optional fla
 
 test("Puzzle2d owned delete inverse preserves complete original cascade records",async()=>{
- const root=new URL("../../../../🧬️schema/🧬️mutations/🟢️node/🗑️delete-node/🎮️prepare/",import.meta.url),corpus=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🪢️connect-handles/🎮️prepare/🔣️.json",root)).json();expect(corpus.inverseOwnership.maximumItems).toBe(1);expect(corpus.inverseOwnership.maximumTurnBytes).toBe(4096);expect(corpus.inverseOwnership.changedOwner).toBe("nativePagedInverse");
+ const root=new URL("../../../../🧬️schema/🧬️mutations/🟢️node/🗑️delete-node/🎮️prepare/",import.meta.url),corpus=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-node/🎮️prepare/🔣️.json",import.meta.url)).json();expect(corpus.inverseOwnership.maximumItems).toBe(1);expect(corpus.inverseOwnership.maximumTurnBytes).toBe(4096);expect(corpus.inverseOwnership.changedOwner).toBe("nativePagedInverse");
  for(const row of corpus.cases){const before={nodes:row.nodes.map((node:any,index:number)=>({...node,shape:"rectangle",x:index,y:-index,text:"😀\u0000",handles:node.handles.map((id:string)=>({id,angle:2,color:"retained"}))})),edges:row.edges.map((edge:any,index:number)=>({...edge,edgeKind:"",gap:-3,shift:2,rise:1,rotation:4,turn:5,tilt:-6,x:7,y:-8,sourceTip:"",targetTip:"é",...(index%3===0?{visible:false,locked:true}:index%3===1?{visible:true,locked:false}:{})})),untouched:{camera:[3,-4,1],metadata:"original"}};
   const patch=[...row.severedEdges].reverse().map((index:number)=>({op:"remove" as const,path:`/edges/${index}`}));if(row.node!==null)patch.push({op:"remove",path:`/nodes/${row.node}`});const after=applyPatch(structuredClone(before),patch).newDocument;
@@ -235,5 +235,5 @@ test("Puzzle2d owned delete inverse preserves complete original cascade records"
 
 test("Puzzle2d delete candidate preserves first matching edge IDs and refuses repeated targets",async()=>{
- const root=new URL("../../../../🧬️schema/🧬️mutations/🟢️node/🗑️delete-node/🎮️prepare/",import.meta.url),base=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🪢️connect-handles/🎮️prepare/🔣️.json",root)).json(),corpus=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🪢️connect-handles/🎮️prepare/📸️candidate/🔣️.json",root)).json();
+ const root=new URL("../../../../🧬️schema/🧬️mutations/🟢️node/🗑️delete-node/🎮️prepare/",import.meta.url),base=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-node/🎮️prepare/🔣️.json",import.meta.url)).json(),corpus=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-node/🎮️prepare/📸️candidate/🔣️.json",import.meta.url)).json();
  const text=(value:string)=>value.startsWith("$large:")?base.control.largePrefix.repeat(base.control.largeRepeats)+value.slice(7):value;
  for(const row of [...base.cases,...corpus.cases]){const db=new Database(":memory:");try{
@@ -256,5 +256,5 @@ test("Puzzle2d delete candidate preserves first matching edge IDs and refuses re
 
 test("Puzzle2d borrowed connect preparation keeps invariant priority and proximity intent",async()=>{
- const base="../../../../🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🎮️prepare/",corpus=await Bun.file(new URL(base+"../../../../🧪️tests/🧫️fixtures/🧬️mutations/🪢️connect-handles/🎮️prepare/🔣️.json",import.meta.url)).json(),{Vector2}=await import("three");
+ const base="../../../../🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🎮️prepare/",corpus=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🪢️connect-handles/🎮️prepare/🔣️.json",import.meta.url)).json(),{Vector2}=await import("three");
  const expanded=(id:string)=>id.startsWith("$large:")?corpus.control.largePrefix.repeat(corpus.control.largeRepeats)+id.slice(7):id;
  for(const entry of corpus.cases){
@@ -275,5 +275,5 @@ test("Puzzle2d borrowed connect preparation keeps invariant priority and proximi
 
 test("Puzzle2d connect inverse owns only unconditional literal ID intent",async()=>{
- const root=new URL("../../../../🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🎮️prepare/↩️inverse/",import.meta.url),corpus=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🪢️connect-handles/🎮️prepare/🔣️.json",root)).json();
+ const root=new URL("../../../../🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🎮️prepare/↩️inverse/",import.meta.url),corpus=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🪢️connect-handles/🎮️prepare/↩️inverse/🔣️.json",import.meta.url)).json();
  const text=(value:string)=>value.startsWith("$large:")?corpus.control.largePrefix.repeat(corpus.control.largeRepeats)+value.slice(7):value;
  for(const row of corpus.cases){expect(row.inverse).toEqual([{kind:"disconnect-handles",id:row.id}]);const id=text(row.id),db=new Database(":memory:");try{db.run("CREATE TABLE inverse_intent(kind TEXT NOT NULL,id TEXT NOT NULL)");db.run("INSERT INTO inverse_intent VALUES(?,?)",["disconnect-handles",id]);expect(db.query("SELECT kind,id,length(CAST(id AS BLOB)) AS bytes FROM inverse_intent").get()).toEqual({kind:"disconnect-handles",id,bytes:new TextEncoder().encode(id).length});}finally{db.close()}
@@ -281,10 +281,10 @@ test("Puzzle2d connect inverse owns only unconditional literal ID intent",async(
  console.log(`[DEBUG] Puzzle connect inverse ${row.name} preserves literal ID against SQLite and RFC6902 ordered restoration`);
  }
- const source=await Bun.file(new URL("🦀️.rs",root)).text();expect(source).toContain("pub type Puzzle2dConnectHandlesInverseCursor");expect(source).toContain("impl Puzzle2dLiteralIdInverse for ConnectHandles");expect(source).toContain("&self.id");const owner=await Bun.file(new URL("../../../🏷️literal-id-inverse/🦀️.rs",root)).text();expect(owner).toContain("mutation.project(1, M::inverse_id)");expect(owner).toContain("PagedList<Puzzle2dMutation");expect(owner).toContain("ControlledRetirement");expect(owner).not.toMatch(/to_string_owner|\.clone\(|format!|serde_json|owned_retirement|Vec</);expect(source).not.toMatch(/to_string_owner|\.clone\(|format!|serde_json|owned_retirement|Vec</);
+ const source=await Bun.file(new URL("🦀️.rs",root)).text();expect(source).toContain("pub type Puzzle2dConnectHandlesInverseCursor");expect(source).toContain("impl Puzzle2dLiteralIdInverse for ConnectHandles");expect(source).toContain("&self.id");const owner=await Bun.file(new URL("../../../../🧬️schema/🧬️mutations/🏷️literal-id-inverse/🦀️.rs",import.meta.url)).text();expect(owner).toContain("mutation.project(1, M::inverse_id)");expect(owner).toContain("PagedList<Puzzle2dMutation");expect(owner).toContain("ControlledRetirement");expect(owner).not.toMatch(/to_string_owner|\.clone\(|format!|serde_json|owned_retirement|Vec</);expect(source).not.toMatch(/to_string_owner|\.clone\(|format!|serde_json|owned_retirement|Vec</);
 });
 
 
 test("Puzzle2d connection edge assembly owns native fields without copying preconditions",async()=>{
- const root=new URL("../../../../🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🎮️prepare/🪢️edge/",import.meta.url),corpus=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🪢️connect-handles/🎮️prepare/🔣️.json",root)).json();
+ const root=new URL("../../../../🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🎮️prepare/🪢️edge/",import.meta.url),corpus=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🪢️connect-handles/🎮️prepare/🪢️edge/🔣️.json",import.meta.url)).json();
  const text=(value:string|null)=>value?.startsWith("$large:")?corpus.control.largePrefix.repeat(corpus.control.largeRepeats)+value.slice(7):value;
  for(const row of corpus.cases){const db=new Database(":memory:");try{db.run("CREATE TABLE native_edge(id TEXT,source TEXT,target TEXT,edge_kind TEXT,source_tip TEXT,target_tip TEXT)");const columns=["id","source","target","edgeKind","sourceTip","targetTip"];db.run("INSERT INTO native_edge VALUES(?,?,?,?,?,?)",columns.map(key=>text(row.mutation[key])));expect(Object.values(db.query("SELECT * FROM native_edge").get()!)).toEqual(columns.map(key=>text(row.edge[key])));
@@ -296,5 +296,5 @@ test("Puzzle2d connection edge assembly owns native fields without copying preco
 
 test("Puzzle2d connect candidate preserves every unrelated owner and native ordered placement",async()=>{
- const root=new URL("../../../../🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🎮️prepare/",import.meta.url),base=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🪢️connect-handles/🎮️prepare/🔣️.json",root)).json(),corpus=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🪢️connect-handles/🎮️prepare/📸️candidate/🔣️.json",root)).json();
+ const root=new URL("../../../../🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🎮️prepare/",import.meta.url),base=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🪢️connect-handles/🎮️prepare/🔣️.json",import.meta.url)).json(),corpus=await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🪢️connect-handles/🎮️prepare/📸️candidate/🔣️.json",import.meta.url)).json();
  for(const row of corpus.cases){const intent=base.cases.find((entry:any)=>entry.id===row.preparationCase);expect(intent).toBeDefined();const mutation={...intent.mutation,index:row.index},before={edges:row.edges.map((id:string)=>({id,visible:false,locked:true,sourceTip:"retained\u0000😀"})),untouched:{nodes:intent.nodes,camera:[3,-4,1],metadata:"native\u0000😀"}},changed=intent.status==="changed",position=Math.min(row.index??row.edges.length,row.edges.length),edge={id:mutation.id,source:mutation.source,target:mutation.target,edgeKind:mutation.edgeKind,sourceTip:mutation.sourceTip,targetTip:mutation.targetTip,gap:mutation.gap,shift:mutation.shift,rise:mutation.rise,rotation:mutation.rotation,turn:mutation.turn,tilt:mutation.tilt,x:mutation.x,y:mutation.y,visible:null,locked:null};
  const after=applyPatch(structuredClone(before),changed?[{op:"add",path:`/edges/${position}`,value:edge}]:[]).newDocument;expect(after.edges.map((edge:any)=>edge.id)).toEqual(row.after);expect(after.untouched).toEqual(before.untouched);if(changed)expect(after.edges[position]).toEqual(edge);else expect(after).toEqual(before);

```


## 16 ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📜️script.ts

### HEAD
```diff
diff --git a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📜️script.ts b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📜️script.ts
index ac886fcd08e..55ce3c1a4f8 100644
--- a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📜️script.ts
+++ b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📜️script.ts
@@ -1,4 +1,4 @@
 #!/usr/bin/env bun
-import { GenerateScript as GraphGenerateScript, PreviewGeneratedScript as GraphPreviewScript, OwnerGraphWireCheckScript } from "../../../../../../../🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts";
+import { GenerateScript as GraphGenerateScript, PreviewGeneratedScript as GraphPreviewScript, OwnerGraphWireCheckScript } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts";
 /** 📦️ puzzle-2d Rust artifact package router. */
 import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";

```

### INCOMING
```diff
diff --git a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📜️script.ts b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📜️script.ts
index ac886fcd08e..78882506c16 100644
--- a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📜️script.ts
+++ b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📜️script.ts
@@ -4,6 +4,9 @@ import { GenerateScript as GraphGenerateScript, PreviewGeneratedScript as GraphP
 import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
 import { BundleScript } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
+
 import { runRepositoryCommand } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
 import { resolve } from "node:path";
+import {readCargoTestPolicyV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
+import {configuredExactCargoLawPolicyV1,runExactCargoLaws} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";
 
 /** 🔍️ Checks this owner and its independent consumer through the public facade. */
@@ -15,3 +18,23 @@ class OwnedVerifyScript extends BundleScript {
   }
 }
-await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-puzzle-2d", {commands:{"graph-generate":GraphGenerateScript,"preview-generated":GraphPreviewScript,"graph-wire-check":OwnerGraphWireCheckScript,verify:OwnedVerifyScript},snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});
+/** ♻️ Verifies resumable standalone history retirement against the shared neutral corpus. */
+class StoreCloseScript extends BundleScript {
+  async run(segments: string[]): Promise<void> {
+    if (segments.length !== 1 || !["source", "native"].includes(segments[0])) throw new Error("test-store-close requires source or native");
+    if (segments[0] === "source") await runRepositoryCommand(this.invocation, process.execPath, ["test", resolve(this.root, "../../🔨️modules/🏠️host/🧰️owned/🧪️tests/♻️store-close/🟦️.ts")], this.repoRoot, "puzzle2d-store-close-source", {version:1,maximumElapsedMilliseconds:0,output:null});
+    else await runArtifactRustTests("semio-s-artifact-puzzle-2d", this.repoRoot, ["--lib", "standalone_history_close_resumes_past_the_original_ledger_page"], this.invocation);
+  }
+}
+/** 👥️ Validates canonical presence and real event-sourced Puzzle peer interaction without app camera copies. */
+class PresenceScript extends BundleScript{
+ async run(segments:string[]):Promise<void>{
+  if(segments.length!==1||!["source","native"].includes(segments[0]!))throw Error("test-presence requires source or native");
+  const camera=resolve(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👥️presence/📷️camera/🧪️tests/🟦️.ts");
+  await runRepositoryCommand(this.invocation,process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",camera],this.repoRoot,"puzzle2d-presence-source-types",{version:1,maximumElapsedMilliseconds:120000,output:{maximumBytes:8388608,maximumLines:65536}});
+  const {testCameraPresenceContract}=await import("../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👥️presence/📷️camera/🧪️tests/🟦️.ts");console.log("[DEBUG] Puzzle canonical presence source assertions="+await testCameraPresenceContract(this.repoRoot));
+  await runRepositoryCommand(this.invocation,"rustfmt",["--check","--edition","2021","--config","disable_all_formatting=true,skip_children=true",resolve(this.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"),resolve(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🦀️.rs")],this.repoRoot,"puzzle2d-presence-rust-source",{version:1,maximumElapsedMilliseconds:120000,output:{maximumBytes:8388608,maximumLines:65536}});
+  if(segments[0]==="source")return;const policy=readCargoTestPolicyV1(process.env);const manifest=resolve(this.root,"Cargo.toml");if(resolve(policy.manifestPath)!==manifest)throw Error("Puzzle presence requires its original native manifest");
+  await runExactCargoLaws({invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:policy.buildBudgetMs,lawMilliseconds:policy.assertionBudgets[policy.level]},cwd:this.repoRoot,cargoTargetDir:policy.targetDirectory,nativeEnv:{RUST_MIN_STACK:policy.rustMinStack},artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,manifestPaths:{"semio-s-artifact-puzzle-2d":manifest},groups:[{package:"semio-s-artifact-puzzle-2d",target:{kind:"lib",name:"semio_s_artifact_puzzle_2d"},cargoArgs:["--locked","--features","component-app-assembly"],laws:["editor::puzzle2d::component::unit_tests::mounted_camera_and_interaction_presence_reaches_peers_without_document_edits"]}]});
+ }
+}
+await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-puzzle-2d", {testFeatures:["component-app-assembly"],snapshotSqliteTestFeatures:["component-app-assembly"],commands:{"graph-generate":GraphGenerateScript,"preview-generated":GraphPreviewScript,"graph-wire-check":OwnerGraphWireCheckScript,verify:OwnedVerifyScript,"test-store-close":StoreCloseScript,"test-presence":PresenceScript},snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});

```


## 17 ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📜️script.ts

### HEAD
```diff
diff --git a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📜️script.ts b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📜️script.ts
index 55cbd03dce6..4a4272ac9af 100644
--- a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📜️script.ts
+++ b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📜️script.ts
@@ -1,4 +1,4 @@
 #!/usr/bin/env bun
-import { GenerateScript as GraphGenerateScript, PreviewGeneratedScript as GraphPreviewScript, OwnerGraphWireCheckScript } from "../../../../../../../🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts";
+import { GenerateScript as GraphGenerateScript, PreviewGeneratedScript as GraphPreviewScript, OwnerGraphWireCheckScript } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts";
 /** 📦️ puzzle-5d Rust artifact package router. */
 import { runArtifactRustPackageMain, runArtifactRustTests } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";

```

### INCOMING
```diff
diff --git a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📜️script.ts b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📜️script.ts
index 55cbd03dce6..c05e36a86ae 100644
--- a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📜️script.ts
+++ b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📜️script.ts
@@ -4,5 +4,5 @@ import { GenerateScript as GraphGenerateScript, PreviewGeneratedScript as GraphP
 import { runArtifactRustPackageMain, runArtifactRustTests } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
 import { runCmd } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
-import { BundleScript } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
+import { BundleScript } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
 import { readdirSync } from "node:fs";
 import { resolve } from "node:path";
@@ -11,4 +11,8 @@ import { resolve } from "node:path";
 class OwnedVerifyScript extends BundleScript {
   run(args: string[]): void {
+    if (args.length === 1 && args[0] === "original-work-construction") {
+      runCmd("bun", ["test", resolve(this.repoRoot, "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🧪️tests/🟦️.ts")], { cwd: this.repoRoot });
+      return;
+    }
     if (args.length !== 1 || args[0] !== "snapshot-sqlite-source") throw new Error("verify snapshot-sqlite-source");
     const roots: string[] = [];
@@ -27,5 +31,5 @@ class HostAdmissionTest extends BundleScript {
   async run(args: string[]): Promise<void> {
     if (args.length) throw new Error("Expected test host-admission");
-    await runArtifactRustTests("semio-s-artifact-puzzle-5d", this.repoRoot, ["--lib", "puzzle5d_host_admission_", "--", "--nocapture"], this.invocation.control);
+    await runArtifactRustTests("semio-s-artifact-puzzle-5d", this.repoRoot, ["--features", "component-app-assembly", "--lib", "puzzle5d_host_admission_", "--", "--nocapture"], this.invocation);
   }
 }

```


## 18 ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs

### HEAD
```diff
diff --git a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs
index 0d5e2ab91f3..19892da5e5f 100644
--- a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs
+++ b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs
@@ -406,5 +406,5 @@ fn a_registered_mesh_is_shared_by_id_across_sessions() {
 /// 🥽️ The language-neutral upload contract both ends implement: this decoder, and the renderer's pager
 /// (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx`),
-/// which is held to the same scene_snapshot by `🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`.
+/// which is held to the same scene_snapshot by `🧪️tests/🥽️brush-mesh-upload/🟦️.ts` of the puzzle 3d artifact.
 const BRUSH_MESH_UPLOAD_FIXTURE: &str = include_str!("../../../../🧪️tests/🧫️fixtures/🥽️brush-mesh-upload/🔣️.json");
 

```

### INCOMING
```diff
diff --git a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs
index 0d5e2ab91f3..1eb58d24c81 100644
--- a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs
+++ b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs
@@ -1,5 +1,14 @@
 use super::*;
-use semio_s_artifact_puzzle_3d_model::precompute_model_tests::context::*;
-use semio_s_artifact_puzzle_3d_model::{BrushHostRules, BrushKindWeights, CableKindCatalog, EngineSceneObject, KindCompatEntry, ObjectKind, ObjectKindRepresentation, ObjectKindVortexTemplate, VortexKindCatalog, VortexProps};
+use super::*;
+use semio_s_artifact_puzzle_3d_model::BrushHostRules;
+use semio_s_artifact_puzzle_3d_model::BrushKindWeights;
+use semio_s_artifact_puzzle_3d_model::CableKindCatalog;
+use semio_s_artifact_puzzle_3d_model::EngineSceneObject;
+use semio_s_artifact_puzzle_3d_model::KindCompatEntry;
+use semio_s_artifact_puzzle_3d_model::ObjectKind;
+use semio_s_artifact_puzzle_3d_model::ObjectKindRepresentation;
+use semio_s_artifact_puzzle_3d_model::ObjectKindVortexTemplate;
+use semio_s_artifact_puzzle_3d_model::VortexKindCatalog;
+use semio_s_artifact_puzzle_3d_model::VortexProps;
 
 fn catalog_host_engine() -> Puzzle3dCollision {
@@ -196,7 +205,7 @@ fn precompute_session_native_wrapper_exercises_public_methods() {
     assert!(session.brush_preview("host:v0", 0).is_none());
 
-    let mut object_weights = std::collections::BTreeMap::new();
-    object_weights.insert("Host".to_string(), 1.0);
-    session.dispatch(Puzzle3dEngineCommand::UpdateKindWeights { object_weights, vortex_weights: std::collections::BTreeMap::new() }).expect("update kind weights");
+    let mut object_weights = semio_framework_value::retained_clone::ordered_map::RetainedOrderedMap::new();
+    object_weights.cold_insert("Host".to_string(), 1.0);
+    session.dispatch(Puzzle3dEngineCommand::UpdateKindWeights { object_weights, vortex_weights: semio_framework_value::retained_clone::ordered_map::RetainedOrderedMap::new() }).expect("update kind weights");
 
 }
@@ -224,5 +233,5 @@ fn enqueue_brush_target_appends_once() {
 /// this one (relocated from the former `⚙️engine` root's `the_5d_facing_engine_surface_stays_public`,
 /// ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): every item it names must stay
-/// publicly reachable — pure data under `crate::standards::v1::subsets::any::schema::…`, the session/dispatch
+/// publicly reachable — pure data under `semio_s_artifact_puzzle_3d_model::standards::v1::subsets::any::schema::…`, the session/dispatch
 /// surface under `crate::editor::puzzle3d::precompute::…`. A rename or a visibility narrowing breaks
 /// this test long before it breaks 5d.

```


## 19 ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs

### HEAD
```diff
diff --git a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
index c3d32951bc1..1e6a7de5042 100644
--- a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
+++ b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
@@ -262,5 +262,5 @@ struct PublicOwned(Option<Puzzle3dSnapshot>);impl PublicOwned{fn new(value:Puzzl
 fn exact(a:&Puzzle3dSnapshot,b:&Puzzle3dSnapshot){use semio_framework_value::{DslValue,Number,ToValue};fn same(a:&DslValue,b:&DslValue){match(a,b){(DslValue::Number(Number::Float(a)),DslValue::Number(Number::Float(b)))=>assert_eq!(a.to_bits(),b.to_bits()),(DslValue::Array(a),DslValue::Array(b))=>{assert_eq!(a.len(),b.len());for(a,b)in a.iter().zip(b){same(a,b);}},(DslValue::Object(a),DslValue::Object(b))=>{assert_eq!(a.len(),b.len());for((ka,a),(kb,b))in a.iter().zip(b){assert_eq!(ka,kb);same(a,b);}},_=>assert_eq!(a,b)}}same(&a.to_value(),&b.to_value());}
 
-fn register(){semio_framework_plugin::Plugin::<PublicApps>::builder("puzzle").label("Populated public SQLite").version("0.0.1").package_id("semio:puzzle").declare_artifact(semio_s_artifact_puzzle_3d::artifact::<PublicApps>()).try_build().unwrap();}
+fn register(){semio_framework_plugin::PluginBuilder::<semio_framework_plugin::NeedsLabel, PublicApps>::new("puzzle").label("Populated public SQLite").version("0.0.1").package_id("semio:puzzle").declare_artifact(semio_s_artifact_puzzle_3d::artifact::<PublicApps>()).try_build().unwrap();}
 #[semio_framework_async_macros::async_test]
 async fn sqlite_snapshot_puzzle3d_populated_actual_app_public_both_forms(){

```

### INCOMING
```diff
diff --git a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
index c3d32951bc1..5bae82d6f71 100644
--- a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
+++ b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
@@ -3,5 +3,6 @@ use ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot
 use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;
 use semio_s_artifact_puzzle_3d_model::*;
-use store::{pack::snapshot::ArtifactDsl, pack::snapshot::ArtifactPack};
+use store::pack::snapshot::ArtifactDsl;
+use store::pack::snapshot::ArtifactPack;
 use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dPlaySnapshot;
 

```


## 20 ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📜️script.ts

### HEAD
```diff
diff --git a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📜️script.ts b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📜️script.ts
index 780600375e0..fd0a69639bd 100644
--- a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📜️script.ts
+++ b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📜️script.ts
@@ -1,4 +1,4 @@
 #!/usr/bin/env bun
-import { GenerateScript as GraphGenerateScript, PreviewGeneratedScript as GraphPreviewScript, OwnerGraphWireCheckScript } from "../../../../../../../🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts";
+import { GenerateScript as GraphGenerateScript, PreviewGeneratedScript as GraphPreviewScript, OwnerGraphWireCheckScript } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts";
 /** 📦️ puzzle-3d Rust artifact package router. */
 import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";

```

### INCOMING
```diff
diff --git a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📜️script.ts b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📜️script.ts
index 780600375e0..a954a0bab38 100644
--- a/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📜️script.ts
+++ b/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📜️script.ts
@@ -2,8 +2,8 @@
 import { GenerateScript as GraphGenerateScript, PreviewGeneratedScript as GraphPreviewScript, OwnerGraphWireCheckScript } from "../../../../../../../🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts";
 /** 📦️ puzzle-3d Rust artifact package router. */
-import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
+import { runArtifactRustPackageMain, runArtifactRustPackagesTests } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
 
 import { runCmd } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
-import { BundleScript } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
+import { BundleScript } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
 import { readdirSync } from "node:fs";
 import { resolve } from "node:path";
@@ -11,5 +11,10 @@ import { resolve } from "node:path";
 /** 🛂️ Verify the exact canonical provider through every published owning Source consumer. */
 class OwnedVerifyScript extends BundleScript {
-  run(args: string[]): void {
+  async run(args: string[]): Promise<void> {
+    if(args[0]==="original-fill-fixture"&&(args.length===1||args.length===2&&args[1]==="native")){runCmd("bun",["test",resolve(this.repoRoot,"✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🎟️caller/🧪️tests/🟦️.ts")],{cwd:this.repoRoot});if(args[1]==="native")await runArtifactRustPackagesTests(["semio-s-artifact-puzzle-3d-editor-session"],this.repoRoot,["--offline","--test","precompute-fill","--","--nocapture"],this.invocation);return;}
+    if (args.length === 1 && args[0] === "original-work-construction") {
+      runCmd("bun", ["test", resolve(this.repoRoot, "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🎬️action/🧪️tests/🟦️.ts"), resolve(this.repoRoot, "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🟦️.ts")], { cwd: this.repoRoot });
+      return;
+    }
     if (args.length !== 1 || args[0] !== "snapshot-sqlite-source") throw new Error("verify snapshot-sqlite-source");
     const roots: string[] = [];

```


## 21 ✏️s/🧑‍💻dev/🏗️fem/📦️packages/🟦️typescript/📜️script.ts

### HEAD
```diff
diff --git a/✏️s/🧑‍💻dev/🏗️fem/📦️packages/🟦️typescript/📜️script.ts b/✏️s/🧑‍💻dev/🏗️fem/📦️packages/🟦️typescript/📜️script.ts
index 188e61f1ac8..d59a348db59 100644
--- a/✏️s/🧑‍💻dev/🏗️fem/📦️packages/🟦️typescript/📜️script.ts
+++ b/✏️s/🧑‍💻dev/🏗️fem/📦️packages/🟦️typescript/📜️script.ts
@@ -9,5 +9,7 @@ import { dirname, extname, join, relative, resolve } from "node:path";
 
 /** 🧪️ Executes the contracts owned by this component. */
-import { policyReadFileSafe, toolJobFemLiveVisualPublicationExact, toolJobFemNumericalMicrocursorExact } from "../../../../../📜️script.ts";
+import { policyReadFileSafe } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts";
+import { toolJobFemLiveVisualPublicationExact } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏗️fem-visual/🟦️.ts";
+import { toolJobFemNumericalMicrocursorExact } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🔁️live-replay/🟦️.ts";
 import { toolJobFemNumericalMicrocursorSelfTests } from "../../../../🔌️plugins/🏗️fem/🧪️tests/🔬️tool-job-fem-numerical-microcursor/🟦️.ts";
 import { toolJobFemLiveVisualPublicationSelfTests } from "../../../../🔌️plugins/🏗️fem/🧪️tests/🔬️tool-job-fem-live-visual-publication/🟦️.ts";

```

### INCOMING
```diff
diff --git a/✏️s/🧑‍💻dev/🏗️fem/📦️packages/🟦️typescript/📜️script.ts b/✏️s/🧑‍💻dev/🏗️fem/📦️packages/🟦️typescript/📜️script.ts
index 188e61f1ac8..22433154041 100644
--- a/✏️s/🧑‍💻dev/🏗️fem/📦️packages/🟦️typescript/📜️script.ts
+++ b/✏️s/🧑‍💻dev/🏗️fem/📦️packages/🟦️typescript/📜️script.ts
@@ -120,5 +120,5 @@ if (segments[0] === "fem2d-window-config-contract" || segments[0] === "fem3d-win
       if (dimension === "3d") {
         const { runVitest } = await import("../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
-        await runVitest(join(this.repoRoot, "✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript"), ["-t", "story window ownership|story document replacement"], "../../🧪️tests/🎚️config/🟦️.ts");
+        await runVitest(join(this.repoRoot, "✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript"), ["-t", "story window ownership|story document replacement|story interaction", "--silent=false"], "../../🧪️tests/🎚️config/🟦️.ts");
       }
       const contract = await import(`${testRoot}/🟦️.ts`);
@@ -132,5 +132,5 @@ if (segments[0] === "fem2d-window-config-contract" || segments[0] === "fem3d-win
         testNumericalPageOwners();
       }
-      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", `${testRoot}/🟦️.ts`, ...(dimension === "3d" ? [mountedStiffnessOracle, numericalOwnerOracle, join(this.repoRoot, "✏️s/🧑‍💻dev/🏗️fem/📖️stories/🧭️coordination/🧪️tests/🪟️viewport/🟦️.ts")] : [])], { cwd: this.repoRoot });
+      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", `${testRoot}/🟦️.ts`, ...(dimension === "3d" ? [mountedStiffnessOracle, numericalOwnerOracle, join(this.repoRoot, "✏️s/🧑‍💻dev/🏗️fem/📖️stories/🧭️coordination/🧪️tests/🪟️viewport/🟦️.ts"), join(this.repoRoot, "✏️s/🧑‍💻dev/🏗️fem/📖️stories/🧭️coordination/🧪️tests/🕹️interaction/🟦️.ts")] : [])], { cwd: this.repoRoot });
       if (segments[1] === "native") {
         const { runCargo } = await import("../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");

```


## 22 🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📜️script.ts

### HEAD
```diff
diff --git a/🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📜️script.ts b/🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📜️script.ts
index b183b490a68..1bca8d1f930 100644
--- a/🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📜️script.ts
+++ b/🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📜️script.ts
@@ -46,5 +46,5 @@ class TestScript extends BundleScript {
   async run(segments: string[]): Promise<void> {
     const { rest } = resolveTestLevel(segments);
-    await runRepositoryCargoTests([PROCTOR_PACKAGE], join(this.repoRoot, TEACHING_WORKSPACE), this.invocation, rest);
+    await runRepositoryCargoTests([PROCTOR_PACKAGE], this.repoRoot, this.invocation, rest);
   }
 }

```

### INCOMING
```diff
diff --git a/🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📜️script.ts b/🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📜️script.ts
index b183b490a68..c125b3bc150 100644
--- a/🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📜️script.ts
+++ b/🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📜️script.ts
@@ -5,9 +5,9 @@ import { resolveTestLevel } from "../../../../🧰️framework/🔨️modules/
 /** 🛂️ `teaching-proctor` task router: `bun ./📜️script.ts <build [args…]|test [fundamental|quick|long|exhaustive] [args…]|dev|check [catalog]|rebuild|health|backup [directory/|file|-]|restore <file|->|erase (--handle <handle>|--tag <tag>|--learner <id>) [--dry-run]|prune --older-than <age> [--dry-run]|capacity [--learners n] [--compression n] [--report file] [--executable proctor] [--hall-only]>`.
  *
- * Every verb but `build`, `test` and `capacity` builds the binary and runs a private copy of it
+ * Every verb but `build`, `test` and `capacity` builds the binary and retains an immutable bounded copy of it
  * (`../../🏗️bootstrap/🟦️.ts`), so a running dev proctor never locks the executable any other build writes. `dev` serves
- * on `PROCTOR_PORT` (8791) over the git-ignored `.🧬semio/🎓️teaching/proctor-dev/` and the architecture catalog unless
+ * on `PROCTOR_PORT` (8791) over the git-ignored `.🧬semio/🎓️teaching/proctor-dev/` and the architecture course unless
  * the launcher set `PROCTOR_*` itself, and moves that folder aside (saying so) when it holds data of a storage format
- * this proctor does not read; `check` validates a catalog (default: the dev catalog); `rebuild` refolds every
+ * this proctor does not read; `check` validates a course (default: the dev course); `rebuild` refolds every
  * read model of the dev data directory from its event log (Ctrl+C stops between batches). The operator verbs act on the
  * same dev environment: `health` asks the dev proctor whether it serves; `backup` copies its database while it serves
@@ -26,17 +26,17 @@ import { resolveTestLevel } from "../../../../🧰️framework/🔨️modules/
 import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
 import { runScriptMain } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
-import { runCmd } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts";
+import { runRepositoryCommand } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
 import { runRepositoryCargoTests } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts";
 
 import {verifyCapturedCargoArtifactsV1} from "../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts";
-import {repositoryCargoNetworkV1} from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
 import { buildRepositoryCargoArtifacts } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
+import { repositoryCargoNetworkV1 } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
 import { join } from "node:path";
-import { PROCTOR_PACKAGE, TEACHING_WORKSPACE, proctorBackupTarget, proctorCatalog, proctorDevelopmentEnvironment, proctorSelection, runDevelopmentProctor, runProctor } from "../../🏗️bootstrap/🟦️.ts";
+import { PROCTOR_PACKAGE, TEACHING_WORKSPACE, proctorBackupTarget, proctorCourse, proctorDevelopmentEnvironment, proctorSelection, runDevelopmentProctor, runProctor } from "../../🏗️bootstrap/🟦️.ts";
 
-/** 🏗️ Builds the crate's cargo artifacts (`--release` for a deployment). */
+/** 🏗️ Builds the crate's bounded cargo artifacts under the caller's explicit compiler mode and network grant. */
 class BuildScript extends BundleScript {
   async run(segments: string[]): Promise<void> {
-    await buildRepositoryCargoArtifacts(`${this.root}/Cargo.toml`, segments, this.repoRoot, {producer:"artifact",invocation:this.invocation,consume:verifyCapturedCargoArtifactsV1,network:repositoryCargoNetworkV1(this.invocation,this.repoRoot), control: scriptCompilerAdmissionControlV1(this.invocation, "cargo:build", 0) });
+    await buildRepositoryCargoArtifacts(`${this.root}/Cargo.toml`, segments, this.repoRoot, { producer: "artifact", invocation: this.invocation, consume: verifyCapturedCargoArtifactsV1, network: repositoryCargoNetworkV1(this.invocation, this.repoRoot), control: scriptCompilerAdmissionControlV1(this.invocation, "cargo:build", 0) });
   }
 }
@@ -54,12 +54,12 @@ class TestScript extends BundleScript {
 class DevScript extends BundleScript {
   async run(): Promise<void> {
-    await runDevelopmentProctor(this.repoRoot);
+    await runDevelopmentProctor(this.repoRoot, this.invocation);
   }
 }
 
-/** ✅️ Validates a catalog (default: the dev catalog) and its quizzes; exits non-zero with the issues. */
+/** ✅️ Validates a course (default: the dev course) and its quizzes; exits non-zero with the issues. */
 class CheckScript extends BundleScript {
   async run(segments: string[]): Promise<void> {
-    await runProctor(this.repoRoot, ["check", proctorCatalog(this.repoRoot, segments[0])]);
+    await runProctor(this.repoRoot, this.invocation, ["check", proctorCourse(this.repoRoot, segments[0])]);
   }
 }
@@ -68,5 +68,5 @@ class CheckScript extends BundleScript {
 class RebuildScript extends BundleScript {
   async run(): Promise<void> {
-    await runProctor(this.repoRoot, ["rebuild"], proctorDevelopmentEnvironment(this.repoRoot));
+    await runProctor(this.repoRoot, this.invocation, ["rebuild"], proctorDevelopmentEnvironment(this.repoRoot));
   }
 }
@@ -75,5 +75,5 @@ class RebuildScript extends BundleScript {
 class HealthScript extends BundleScript {
   async run(): Promise<void> {
-    await runProctor(this.repoRoot, ["health"], proctorDevelopmentEnvironment(this.repoRoot));
+    await runProctor(this.repoRoot, this.invocation, ["health"], proctorDevelopmentEnvironment(this.repoRoot));
   }
 }
@@ -82,5 +82,5 @@ class HealthScript extends BundleScript {
 class BackupScript extends BundleScript {
   async run(segments: string[]): Promise<void> {
-    await runProctor(this.repoRoot, ["backup", proctorBackupTarget(this.repoRoot, segments[0])], proctorDevelopmentEnvironment(this.repoRoot));
+    await runProctor(this.repoRoot, this.invocation, ["backup", proctorBackupTarget(this.repoRoot, segments[0])], proctorDevelopmentEnvironment(this.repoRoot));
   }
 }
@@ -90,5 +90,5 @@ class RestoreScript extends BundleScript {
   async run(segments: string[]): Promise<void> {
     if (segments.length !== 1) throw new Error("restore takes the backup file to restore (or - for stdin)");
-    await runProctor(this.repoRoot, ["restore", segments[0]!], proctorDevelopmentEnvironment(this.repoRoot));
+    await runProctor(this.repoRoot, this.invocation, ["restore", segments[0]!], proctorDevelopmentEnvironment(this.repoRoot));
   }
 }
@@ -97,5 +97,5 @@ class RestoreScript extends BundleScript {
 class EraseScript extends BundleScript {
   async run(segments: string[]): Promise<void> {
-    await runProctor(this.repoRoot, ["erase", ...proctorSelection(segments)], proctorDevelopmentEnvironment(this.repoRoot));
+    await runProctor(this.repoRoot, this.invocation, ["erase", ...proctorSelection(segments)], proctorDevelopmentEnvironment(this.repoRoot));
   }
 }
@@ -105,5 +105,5 @@ class EraseScript extends BundleScript {
 class PruneScript extends BundleScript {
   async run(segments: string[]): Promise<void> {
-    await runProctor(this.repoRoot, ["prune", ...proctorSelection(segments)], proctorDevelopmentEnvironment(this.repoRoot));
+    await runProctor(this.repoRoot, this.invocation, ["prune", ...proctorSelection(segments)], proctorDevelopmentEnvironment(this.repoRoot));
   }
 }
@@ -111,6 +111,6 @@ class PruneScript extends BundleScript {
 /** 🏋️ Runs the capacity gate in a process of its own, whose HTTP client may hold a lecture hall of requests in flight. */
 class CapacityScript extends BundleScript {
-  run(segments: string[]): void {
-    runCmd(process.execPath, [`${this.root}/../../🧪️tests/🏋️capacity/🟦️.ts`, ...segments], { cwd: this.repoRoot, env: { ...process.env, BUN_CONFIG_MAX_HTTP_REQUESTS: "4096" }, budgetMs: 0 });
+  async run(segments: string[]): Promise<void> {
+    await runRepositoryCommand(this.invocation, process.execPath, [`${this.root}/../../🧪️tests/🏋️capacity/🟦️.ts`, ...segments], this.repoRoot, "proctor:capacity", { version: 1, maximumElapsedMilliseconds: 3600000, output: { maximumBytes: 67108864, maximumLines: 65536 } }, { env: { ...process.env, BUN_CONFIG_MAX_HTTP_REQUESTS: "4096" }, stdout: "inherit" });
   }
 }

```
