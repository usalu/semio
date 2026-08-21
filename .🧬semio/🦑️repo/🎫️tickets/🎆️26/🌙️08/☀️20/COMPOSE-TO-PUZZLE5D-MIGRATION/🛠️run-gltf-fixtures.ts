#!/usr/bin/env bun
/** 🛠️ Emits every hand-authored `stdio.gltf` mutation-fixture case in this ticket's spec tables. */
import { emit, type Spec } from "./🛠️emit-gltf-fixture.ts";

const REPO = new URL("../../../../../../..", import.meta.url).pathname;

const tables: Promise<{ SPECS: readonly Spec[] }>[] = [
  import("./🛠️gltf-specs-collections.ts"),
  import("./🛠️gltf-specs-deletes.ts"),
  import("./🛠️gltf-specs-moves.ts"),
  import("./🛠️gltf-specs-reorders.ts"),
  import("./🛠️gltf-specs-meshes-a.ts"),
  import("./🛠️gltf-specs-meshes-b.ts"),
  import("./🛠️gltf-specs-nodes.ts"),
  import("./🛠️gltf-specs-document.ts"),
];

let count = 0;
for (const table of tables) {
  for (const spec of (await table).SPECS) {
    emit(REPO, spec);
    count += 1;
  }
}
console.log(`🧪️ emitted ${count} gltf mutation fixture case(s) into ${REPO}`);
