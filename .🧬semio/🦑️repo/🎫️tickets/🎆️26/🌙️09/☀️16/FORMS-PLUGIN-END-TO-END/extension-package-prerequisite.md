# Extension Package Public Interface

The declared procedural geometry owner stopped before compilation because its script requested packageAdmittedExtensionComponentV1 from the artifacts/publication facade, which does not export it. The existing public process/extension-package interface owns that function and its compiler-admitted request. A direct public-module import verifies the function exists. Extension scripts now import that canonical interface; no compatibility export was added. The exact same stale import was corrected in these sibling package routers so the shared script infrastructure is consistent. Actual declared geometry execution remains required.

- ✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/📜️imperative/🧩️extensions/🎮️control/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/📦️packages/🦀️rust/📜️script.ts
- ✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/📦️packages/🦀️rust/📜️script.ts
