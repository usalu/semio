# TS suites per-suite table (tst2)

before = first naive pass (plain `bun test`, no runner environment); after = final pass (runner environment + emulated root hoist, then ticket-local artifact dir for the failing rest).

Totals: before 1064/1465 suites green (11025 pass / 1054 fail tests); after 1261/1465 (12848 / 707).

## Remaining failing suites by class

- SOURCE-LAW-DRIFT: 151
- MISSING-DEP: 19
- HARNESS-ENV: 13
- NEEDS-NATIVE/BROWSER: 12
- NO-DIST: 8
- BROKEN-IMPORT: 1


### SOURCE-LAW-DRIFT

| suite | before p/f | after p/f | first error |
|---|---|---|---|
| `PL/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | 7/1 | 7/1 | expect(received).toBe(expected) |
| `PL/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/✂️brep-intersect/🧪️tests/🔬️unit/🟦️.ts` | 4/1 | 4/1 | ? |
| `PL/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🐚️brep-topology/🧪️tests/🔬️unit/🟦️.ts` | 4/1 | 4/1 | ? |
| `PL/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🛠️brep-feature/🧪️tests/🔬️unit/🟦️.ts` | 3/2 | 3/2 | fillet rounds all twelve edges with spherical corners |
| `PL/🌀️procedural/🗿️artifacts/🧊️generation3d/🔨️modules/🏠️host/📐️geometry-service/🧪️tests/🔬️unit/🟦️.ts` | 0/7 | 0/7 | [{"instancePath":"","schemaPath":"#/oneOf","keyword":"oneOf","params":{"passingSchemas":[0,1]},"message":"must |
| `PL/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/♻️retirement/🧪️tests/🟦️.ts` | 0/1 | 0/1 | expect(received).toEqual(expected) |
| `PL/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | 10/48 | 10/48 | ? |
| `PL/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | 19/1 | 19/1 | expect(received).toBe(expected) |
| `PL/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | 65/2 | 65/2 | expect(received).toEqual(expected) |
| `PL/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📋️contract/🟦️.ts` | 15/1 | 15/1 | expect(received).toBe(expected) |
| `PL/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧪️tests/🔬️unit/🟦️.test.ts` | 53/2 | 53/2 | expect(received).toContain(expected) |
| `PL/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🟦️.ts` | 3/3 | 3/3 | expect(received).toContain(expected) |
| `PL/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | 0/1 | 18/1 | ./🦀️.rs |
| `PL/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🪪️stream-roles/🧪️tests/🟦️.ts` | 5/1 | 5/1 | expect(received).toBeDefined() |
| `PL/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🫙️prefix/🟦️.ts` | 25/22 | 25/22 | expect(received).toEqual(expected) |
| `PL/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | 48/15 | 48/15 | ? |
| `PL/🧪️tests/📁fixture-placement/🟦️.ts` | 35/4 | 35/4 | expect(received).toBe(expected) |
| `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/📐️surface/🧪️tests/🔬️neutral/🟦️.ts` | 1/1 | 1/1 | expect(received).toBe(expected) |
| `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/📐️surface/🧪️tests/🟦️.ts` | 4/1 | 4/1 | expect(received).toBe(expected) |
| `✏️s/🧑‍💻dev/🗄️stdio/🧪️tests/🏃️command-ownership/🟦️.ts` | 0/3 | 0/3 | Owned data capture output limit exceeded |
| `✏️s/🧪️tests/🗂️native-ownership/🟦️.ts` | 3/1 | 3/1 | ? |
| `🌎️hub/🧩️compositions/🗄️stdio/📥️execution/🧪️tests/🟦️.ts` | 0/1 | 0/1 | expect(received).toBeGreaterThan(expected) |
| `🌎️hub/🧪️tests/🧱️foundation-source/🟦️.ts` | 14/2 | 15/1 | expect(received).toThrow(expected) |
| `🧰️framework/📏️script-boundary/🧪️tests/🟦️.ts` | 1/2 | 1/2 | expect(received).toEqual(expected) |
| `M/⏪️time-travel/🧪️testing/📥️values/🧪️tests/🟦️.ts` | 1/1 | 1/1 | expect(received).toBe(expected) |
| `M/⏯️tool-run/🏛️ownership/🧪️tests/🟦️.ts` | 1/1 | 1/1 | /Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust/📜️script.ts -> /Users/ueli/Docu |
| `M/⏳️async/♻️cancel-return/📬️claim/🧪️tests/🟦️.ts` | 1/1 | 1/1 | expect(received).toContain(expected) |
| `M/◻️2d/🧮️compute/🧪️tests/🟦️.ts` | 0/1 | 34/11 | ? |
| `M/🌱️value/🧪️tests/🧩️neutral-owner/🟦️.ts` | 16/1 | 16/1 | failed to load manifest for workspace member `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀ |
| `M/🎒️pack/🌱️value/🛬️decode/🧾️source/🧪️tests/🟦️.ts` | 0/1 | 0/1 | no schema with key or ref "https://json-schema.org/draft/2020-12/schema" |
| `M/🎒️pack/🔤️json/📥️decode/🧾️receipt/🧪️tests/🟦️.ts` | 3/2 | 3/2 | expect(received).toContain(expected) |
| `M/🎒️pack/🔤️json/🧪️tests/🚮️absence/🟦️.ts` | 0/1 | 0/1 | ? |
| `M/🎒️pack/🚰️source/🧪️tests/🟦️.ts` | 1/1 | 1/1 | ? |
| `M/🎠️kernel/🏛️ownership/📥️command-ingress/🧪️tests/🟦️.ts` | 1/1 | 1/1 | ? |
| `M/🎭️actor/🎟️retained-turn/🎟️context/🧪️tests/🏛️boundary/🟦️.ts` | 0/1 | 1/1 | expect(received).not.toContain(expected) |
| `M/🎭️actor/🎟️retained-turn/🤝️admission/🧪️tests/🟦️.ts` | 1/1 | 1/1 | expect(received).toBe(expected) |
| `M/🎭️actor/🧪️tests/🏛️owner-boundary/🟦️.ts` | 0/1 | 0/1 | 📦️packages/🦀️rust/📜️script.ts: ../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🕸️wasm/🔣️policy/🟦️. |
| `M/🎭️actor/🧬️ownership/🧪️tests/🟦️.ts` | 3/1 | 3/1 | expect(received).toMatch(expected) |
| `M/🏃️process/🏗️build/🎛️environment/🧪️tests/🟦️.ts` | 0/1 | 1/1 | expect(received).toBe(expected) |
| `M/🏃️process/🏗️build/🎟️capacity/🛟️custodian/🧪️tests/🟦️.ts` | 0/1 | 0/1 | ? |
| `M/🏃️process/🏗️build/🎟️capacity/🧪️tests/🔐️inheritance/🟦️.ts` | 4/2 | 5/1 | ENOENT: no such file or directory, open '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚 |
| `M/🏃️process/🏗️build/🎟️capacity/🧪️tests/🪆️scope/🟦️.ts` | 4/1 | 4/1 | 51 /  return work; |
| `M/🏃️process/📦️artifacts/🏗️native-build/🎯️recipe/🧪️tests/🟦️.ts` | 2/3 | 2/3 | /Users/ueli/.bun/bin/bun /Users/ueli/Documents/semio/.cache/b/d/m/j/441fd20cf4b20f6dcafe/node_modules/.bun/typ |
| `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🧪️tests/📥️parent/🟦️.ts` | 21/27 | 23/25 | observe-then-retire:OwnedCommandResultV1(process.execPath,["test",fileURLToPath(import.meta.url),"--test-name- |
| `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🧪️tests/🟦️.ts` | 26/34 | 26/34 | expect(received).toBe(expected) |
| `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🧪️tests/🔤️ordering/🟦️.ts` | 0/1 | 0/1 | expect(received).toBe(expected) |
| `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🧪️tests/🟦️.ts` | 2/44 | 10/36 | Original process admission closed |
| `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🪟️windows/🧪️tests/🟦️.ts` | 0/27 | 11/16 | expect(received).toBe(expected) |
| `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🧪️tests/📦️owner/🟦️.ts` | 3/13 | 3/13 | success |
| `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🧪️tests/🟦️.ts` | 16/61 | 16/61 | Original process admission closed |
| `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧪️tests/🟦️.ts` | 8/2 | 8/2 | expect(received).toContain(expected) |
| `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🧪️tests/📥️paths/🟦️.ts` | 0/1 | 0/1 | expect(received).toBe(expected) |
| `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🧪️tests/📥️receipt/🟦️.ts` | 1/1 | 1/1 | ? |
| `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🧪️tests/🟦️.ts` | 2/2 | 3/1 | expect(received).toBe(expected) |
| `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📥️operation/🧪️tests/🟦️.ts` | 2/8 | 3/7 | expect(received).toBe(expected) |
| `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📥️receiver/🧪️tests/📥️release/🟦️.ts` | 0/2 | 0/2 | expect(received).toBe(expected) |
| `M/🏃️process/📦️artifacts/🏗️native-build/📥️inputs/🧪️tests/📥️consumers/🟦️.ts` | 0/2 | 0/2 | expect(received).toBeUndefined() |
| `M/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🎯️target/🧪️tests/📋️callers/🟦️.ts` | 0/1 | 0/1 | /Users/ueli/.bun/bin/bun /Users/ueli/Documents/semio/.cache/b/d/m/j/441fd20cf4b20f6dcafe/node_modules/.bun/typ |
| `M/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🏎️overlap/🟦️.ts` | 1/1 | 1/1 | expect(received).toBe(expected) |
| `M/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/📥️inputs/🟦️.ts` | 0/3 | 0/3 | expect(received).toBe(expected) |
| `M/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🟦️.ts` | 9/19 | 9/19 | expect(received).toBe(expected) |
| `M/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/🧪️tests/🟦️.ts` | 11/4 | 11/4 | bun test v1.3.14 (0d9b296a) |
| `M/🏃️process/📦️artifacts/💾️cache/🧪️tests/🟦️.ts` | 27/16 | 28/15 | Original process birth already received |
| `M/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🟦️.ts` | 53/4 | 53/4 | expect(received).toContain(expected) |
| `M/🏃️process/📦️artifacts/📤️publication/📥️sources/🧪️tests/🟦️.ts` | 15/6 | 16/5 | expect(received).toBe(expected) |
| `M/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️inventory/🟦️.ts` | 1/6 | 3/4 | Build test law body deadline exhausted |
| `M/🏃️process/🧪️testing/🦀️cargo/🎟️admission/🧪️tests/🟦️.ts` | 0/1 | 0/1 | expect(received).toThrow(expected) |
| `M/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🧪️tests/🍎️image.ts` | 0/1 | 0/1 | Original process admission closed |
| `M/🏃️process/🧪️testing/🦀️cargo/🏃️execution/🧪️tests/🟦️.ts` | 15/2 | 15/2 | expect(received).toBe(expected) |
| `M/🏃️process/🧪️testing/🦀️cargo/🧪️tests/📥️preparation/🟦️.ts` | 0/1 | 0/1 | expect(received).toContain(expected) |
| `M/🏃️process/🧭️routing/🎛️command/🧪️tests/❄️cold/🟦️.ts` | 0/1 | 0/1 | ? |
| `M/🏃️process/🧭️routing/🎛️command/🧪️tests/🚧️boundary/🟦️.ts` | 0/3 | 0/3 | expect(received).toEqual(expected) |
| `M/🏃️process/🧭️routing/📤️exports/🧪️tests/🟦️.ts` | 0/1 | 0/1 | expect(received).toBe(expected) |
| `M/🏃️process/🧭️routing/📥️invocation/🏃️process/📤️child/🧪️tests/🟦️.ts` | 0/1 | 0/1 | expect(received).toBe(expected) |
| `M/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📊️profile/🟦️.ts` | 0/1 | 0/1 | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/🟦️.ts |
| `M/🏃️process/🧭️routing/📥️invocation/🧪️tests/⏱️finite/🟦️.ts` | 3/1 | 3/1 | expect(received).toBe(expected) |
| `M/🏃️process/🧭️routing/🧪️tests/🟦️.ts` | 0/1 | 54/2 | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/🟦️.ts |
| `M/📡️replication/⚠️error/🧪️tests/🟦️.ts` | 3/1 | 3/1 | expect(received).toContain(expected) |
| `M/📡️replication/🎮️mutation/🧾️transaction/🎟️admission/🧪️tests/🟦️.ts` | 1/1 | 1/1 | expect(received).toContain(expected) |
| `M/🔲️pixels/🧪️testing/🧭️ownership/🧪️tests/🟦️.ts` | 10/1 | 10/1 | 1 / /** 🧩️ Describes a non-null, non-array object whose member values are unknown. */ |
| `M/🕸️graph/🧪️tests/🧩️suite/🟦️.ts` | 21/2 | 22/1 | Build failed with 1 error: |
| `M/🖱️ui/🌐️locale/🧪️tests/🟦️.ts` | 0/1 | 55/3 | expect(received).toContain(expected) |
| `M/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🎟️presented-source/🧪️tests/🟦️.ts` | 1/0 | 0/1 | expect(received).toBe(expected) |
| `M/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🧪️tests/🟦️.ts` | 9/5 | 12/2 | expect(received).toContain(expected) |
| `M/🖱️ui/🧠️runtime/🎭️present/📦️resources/📤️transport/📨️message/🧪️tests/🟦️.ts` | 0/1 | 0/1 | expect(received).toBe(expected) |
| `M/🖱️ui/🧪️tests/♻️physical-job-close/🟦️.ts` | 4/5 | 7/2 | expect(received).toBe(expected) |
| `M/🖱️ui/🧪️tests/📐️dock-axis-geometry/🟦️.ts` | 0/1 | 0/1 | ? |
| `M/🖱️ui/🧪️tests/📦️prepared-close/🟦️.ts` | 3/1 | 3/1 | expect(received).toBe(expected) |
| `M/🖱️ui/🧪️tests/🔬️targets-wgpu-mounted-layout-unit/🟦️.ts` | 2/3 | 4/1 | ? |
| `M/🖱️ui/🧪️tests/🧊️feature-ownership/🟦️.ts` | 2/4 | 0/0 | ? |
| `M/🖱️ui/🧪️tests/🧭️native-command/🟦️.ts` | 0/1 | 0/1 | ? |
| `M/🖱️ui/🧬️contract/🧪️tests/🔮️corpus-oracles/🟦️.ts` | 3/1 | 3/1 | ? |
| `M/🖱️ui/🧬️contract/🧪️tests/🧱️owners/🟦️.ts` | 0/1 | 4/3 | Missing Rust source input: 🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/♻️retirement/🌳️typed/🧩️components.js |
| `M/🗣️dsl/🧬️schema/✨️derive/🧪️tests/🧱️ownership/🟦️.ts` | 7/1 | 7/1 | Build failed with 2 errors: |
| `M/🚪️io/⏱️control/🛫️snapshot/🧪️tests/🟦️.ts` | 6/1 | 6/1 | expect(received).toBe(expected) |
| `M/🚪️io/🪶️sqlite-snapshot/🧪️tests/🤝️interoperability/🟦️.ts` | 0/3 | 0/3 | ? |
| `M/🧊️3d/🥽️mesh/🛠️modeling/✂️clipping/🧪️tests/🟦️.ts` | 0/1 | 8/7 | expect(received).toEqual(expected) |
| `M/🧊️3d/🥽️mesh/🧪️tests/📦️targets/🧪️packaging/🟦️.ts` | 0/1 | 0/2 | expect(received).toEqual(expected) |
| `M/🧬️schema/✅️validator/🧪️tests/🧩️neutral-owner/🟦️.ts` | 0/1 | 0/1 | ? |
| `M/🧬️schema/🧩️composition/📥️admission/🧪️tests/🟦️.ts` | 0/1 | 3/1 | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🦀️.rs |
| `M/🧬️schema/🧪️tests/🧱️neutrality/🟦️.ts` | 12/2 | 12/2 | expect(received).toContain(expected) |
| `M/🧬️schema/🧪️tests/🩹️fragment-validation-oracle/🟦️.ts` | 0/1 | 0/1 | ? |
| `M/🧵️job/⏱️budget/🧪️tests/🔮️corpus-oracles/🟦️.ts` | 1/1 | 1/1 | microsecond exact worker binding: none |
| `M/🧵️job/⏱️context/📸️capture/🧪️tests/🟦️.ts` | 1/1 | 1/1 | expect(received).toBe(expected) |
| `M/🧵️job/♻️retirement/🎟️context/🧪️tests/🟦️.ts` | 3/5 | 3/5 | ? |
| `M/🧵️job/♻️retirement/👷️authority/🧪️tests/🟦️.ts` | 0/5 | 0/5 | expect(received).toBe(expected) |
| `M/🧵️job/♻️session-return/🧪️tests/🟦️.ts` | 1/1 | 1/1 | expect(received).toContain(expected) |
| `M/🧵️job/🎟️context/🛑️cancel/🤝️alias/🧪️tests/🟦️.ts` | 0/1 | 0/1 | expect(received).toContain(expected) |
| `M/🧵️job/🎟️context/🪙️loan/🧪️tests/🟦️.ts` | 2/1 | 2/1 | ? |
| `M/🧵️job/🏃️work/🎟️grant/🧪️tests/🟦️.ts` | 1/2 | 1/2 | expect(received).toBe(expected) |
| `M/🧵️job/🧪️tests/📦️physical-close/🔮️oracle/🟦️.ts` | 1/1 | 1/1 | expect(received).toBe(expected) |
| `M/🫧️transient/🧪️tests/🟦️.ts` | 0/1 | 4/2 | ? |
| `OS/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🧪️tests/🔬️unit/🟦️.ts` | 22/12 | 22/12 | ? |
| `OS/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | 15/11 | 17/9 | expect(received).toBe(expected) |
| `OS/🔨️modules/🏪️store/📬️outcome/🧪️tests/🟦️.ts` | 0/1 | 0/1 | expect(received).toBe(expected) |
| `OS/🔨️modules/🏪️store/🚪️io/🚫️refusal/📬️outcome/🧪️tests/🟦️.ts` | 0/1 | 0/1 | expect(received).toBe(expected) |
| `OS/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/📦️catalog/🧪️tests/🟦️.ts` | 0/1 | 0/1 | ? |
| `OS/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🧪️tests/🛬️receiving/🟦️.ts` | 4/2 | 4/2 | expect(received).toContain(expected) |
| `OS/🔨️modules/🏪️store/🧩️composition/🚪️open/🧪️tests/🔬️unit/📏️retirement/🌊️hydration-and-source-birth/🟦️.ts` | 9/12 | 9/12 | expect(received).toContain(expected) |
| `OS/🔨️modules/🏪️store/🧩️composition/🚪️open/🧪️tests/🔬️unit/📏️retirement/📄️catalog-retirement/🟦️.ts` | 6/8 | 6/8 | expect(received).toContain(expected) |
| `OS/🔨️modules/🏪️store/🧩️composition/🚪️open/🧪️tests/🔬️unit/📏️retirement/📸️snapshot-custody/🟦️.ts` | 14/14 | 14/14 | expect(received).toContain(expected) |
| `OS/🔨️modules/🏪️store/🧪️tests/🧪️deferred-reprojection/🟦️.ts` | 0/1 | 0/1 | ? |
| `OS/🔨️modules/🏪️store/🧪️tests/🧪️supersede-law/🟦️.ts` | 0/1 | 0/1 | ? |
| `OS/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🟦️.ts` | 0/1 | 0/1 | ? |
| `OS/🔨️modules/🏪️store/🧪️tests/🧪️tool-transaction/🟦️.ts` | 0/1 | 0/1 | ? |
| `OS/🔨️modules/🏪️store/🧪️tests/🧪️viewer-head/🟦️.ts` | 0/1 | 0/1 | ? |
| `OS/🔨️modules/🏪️store/🧬️snapshot-clone/🧪️tests/📦️assembly/🟦️.ts` | 0/1 | 0/1 | ? |
| `OS/🔨️modules/💡️inference/🧪️tests/🔬️unit/🟦️.ts` | 5/2 | 5/2 | ENOENT: no such file or directory, open '/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🛫️en |
| `OS/🔨️modules/📡️spr/📜️history/🛬️decode/🧪️tests/🟦️.ts` | 0/1 | 0/1 | expect(received).toBe(expected) |
| `OS/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🧪️tests/📇️catalog/🟦️.ts` | 0/1 | 0/1 | ? |
| `OS/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎬️media-slots/🎟️collect/🧪️tests/🟦️.ts` | 2/4 | 3/3 | expect(received).toBe(expected) |
| `OS/🔨️modules/📺️renderer/🧑‍🎨engine/🔮️oracles/🎨️world3d-scene-shading/📋️data/🧪️tests/🟦️.ts` | 2/1 | 2/1 | expect(received).toBe(expected) |
| `OS/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/♻️frame-close/🟦️.ts` | 9/5 | 9/5 | expect(received).toBe(expected) |
| `OS/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/♻️hot-swap-session-handover/🟦️.ts` | 7/1 | 7/1 | expect(received).toBeGreaterThan(expected) |
| `OS/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-admitted-surface-map/🟦️.ts` | 0/1 | 0/1 | expect(received).toContain(expected) |
| `OS/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing/🧪️tests/🧭️ownership/🟦️.ts` | 3/1 | 3/1 | 1 / /** 🧩️ Describes a non-null, non-array object whose member values are unknown. */ |
| `OS/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧵️worker-cell/🧪️tests/🟦️.ts` | 3/2 | 3/2 | ? |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | 55/1 | 55/1 | expect(received).toEqual(expected) |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | 41/1 | 41/1 | expect(received).toEqual(expected) |
| `OS/🔨️modules/🔌️plugin/♻️lifecycle/🏪️stores/🧪️tests/🟦️.ts` | 7/1 | 7/1 | expect(received).toBe(expected) |
| `OS/🔨️modules/🔌️plugin/🌐️browser-bundle/🛂️descriptor/🧪️tests/🟦️.ts` | 0/1 | 0/1 | ? |
| `OS/🔨️modules/🔌️plugin/📇️registry/📥️runtime/🧪️tests/🟦️.ts` | 2/1 | 2/1 | error: process didn't exit successfully: `bun '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️mod |
| `OS/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🧬️semantic-source-pairs/🟦️.ts` | 0/1 | 0/1 | writer: emitted descriptor JSON and pack forms disagree |
| `OS/🔨️modules/🔌️plugin/🖥️host/🧬️component-codec/🎟️retained-turn/💼️jobs/🧪️tests/🟦️.ts` | 8/3 | 9/2 | expect(received).toContain(expected) |
| `OS/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🎟️grant/🧪️tests/🟦️.ts` | 3/1 | 3/1 | expect(received).toBe(expected) |
| `OS/🔨️modules/🔌️plugin/🧪️tests/📦️units/🟦️.ts` | 0/1 | 5/8 | expect(received).toContain(expected) |
| `OS/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/📋️inventory/🧪️tests/🟦️.ts` | 0/1 | 0/1 | expect(received).toBe(expected) |
| `OS/🔨️modules/🔌️plugin/🧵️retained-command/📸️checkpoint/👣️cursor/🧪️tests/🟦️.ts` | 1/2 | 1/2 | expect(received).toBe(expected) |
| `OS/🔨️modules/🔌️plugin/🧵️retained-command/🧬️context/🧪️tests/🟦️.ts` | 0/1 | 0/1 | expect(received).toBe(expected) |
| `OS/🔨️modules/🛎️services/🧪️tests/🧮️compute-retained/🟦️.ts` | 11/1 | 11/1 | expect(received).toBe(expected) |
| `🧰️framework/🧪️tests/🧱️ownership/🟦️.ts` | 0/3 | 0/3 | expect(received).toBe(expected) |

### MISSING-DEP

| suite | before p/f | after p/f | first error |
|---|---|---|---|
| `🧰️framework/📦️packages/🟦️typescript/🧪️tests/📤️module-package-surface/🟦️.ts` | 5/1 | 5/1 | `fast-json-stable-stringify` |
| `🧰️framework/📦️packages/🟦️typescript/🧪️tests/🔌️sdk-surface/🟦️.ts` | 4/2 | 4/2 | `polygon-clipping` |
| `M/⏳️async/🔂️poll/🧪️tests/🟦️.ts` | 0/1 | 0/1 | `rxjs` |
| `M/🏃️process/🎛️owned-execution/📥️capture/📡️channel/🧪️tests/🟦️.ts` | 0/1 | 0/1 | `cross-spawn` |
| `M/🏃️process/🎛️owned-execution/📥️capture/🧪️tests/📥️input/🟦️.ts` | 0/1 | 3/1 | `cross-spawn` |
| `M/🏃️process/🎛️owned-execution/🧪️tests/🟦️.ts` | 3/2 | 3/2 | `tree-kill` |
| `M/🏃️process/🧪️testing/🟨️bun/📁️files/🧪️tests/🟦️.ts` | 1/1 | 1/1 | `@noble/hashes` |
| `M/🏃️process/🪓️termination/🧪️tests/🟦️.ts` | 0/1 | 0/1 | `tree-kill` |
| `M/📁️filesystem/🧾️observation/🧪️tests/🎛️control/🟦️.ts` | 0/1 | 1/1 | `@noble/hashes` |
| `M/📚️compiler/📖️syntax/🦀️rust/🧪️tests/🚮️absence/🟦️.ts` | 0/1 | 0/1 | `tree-sitter-wasms` |
| `M/🖌️raster/🎥️video/🖋️svg-export/🧪️tests/🟦️.ts` | 1/2 | 1/2 | `jsdom` |
| `M/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🌗️contrast/🟦️.ts` | 7/1 | 7/1 | `color` |
| `M/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🚪️command/🧪️tests/🟦️.ts` | 0/1 | 0/1 | `execa` |
| `M/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🧪️tests/🟦️.ts` | 0/1 | 0/1 | `execa` |
| `M/🧬️schema/📄️source/🔗️closure/🧪️tests/🚮️absence/🟦️.ts` | 0/1 | 0/1 | `jsonc-parser` |
| `OS/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧪️tests/🚪️lifetime/🟦️.ts` | 1/1 | 1/1 | `@bytecodealliance/preview2-shim` |
| `P/📓️print/📦️artifacts/🎛️operation/🧪️tests/🟦️.ts` | 0/1 | 43/6 | `gl-matrix` |
| `P/📓️print/📦️artifacts/📥️material/🧪️tests/🟦️.ts` | 0/1 | 38/5 | `gl-matrix` |
| `P/📓️print/🧪️tests/🧬️native-chart-grammar/🎨️paint/🧪️tests/🟦️.ts` | 0/1 | 2/2 | `gl-matrix` |

### HARNESS-ENV

| suite | before p/f | after p/f | first error |
|---|---|---|---|
| `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🏗️preparation/🟦️.ts` | 0/1 | 0/1 | Original playground preparation invocation required |
| `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🧩️native-prerequisites/🟦️.ts` | 0/5 | 0/5 | Original bounded Nx namespace required |
| `M/🏃️process/🏗️build/🎟️capacity/🧪️tests/🟦️.ts` | 0/5 | 0/5 | Live Original resource control issuance required |
| `M/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧪️tests/🟦️.ts` | 0/1 | 6/2 | Live Original resource control issuance required |
| `M/🏃️process/📦️artifacts/🏗️native-build/📈️telemetry/🧪️tests/🟦️.ts` | 3/4 | 5/1 | Live Original resource control issuance required |
| `M/🏃️process/📦️artifacts/🐹️go-build/🧪️tests/🟦️.ts` | 1/1 | 1/1 | Live Original resource control issuance required |
| `M/🏃️process/🔒️leases/♻️retirement/🧭️continuation/🧪️tests/📦️child/📜️script.ts` | 0/1 | 0/1 | Exact owned continuation worker selection required |
| `M/🏃️process/🔒️leases/🧪️tests/🎟️writer/🟦️.ts` | 0/5 | 0/5 | Authored finite writer fixture output required |
| `M/🏃️process/🧭️routing/🎛️command/🧪️tests/🟦️.ts` | 2/2 | 2/2 | Live Original process issuance required |
| `M/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📤️fractional/🟦️.ts` | 0/2 | 0/2 | Live Original process issuance required |
| `M/📁️filesystem/🛡️durability/🧪️tests/🟦️.ts` | 0/1 | 0/1 | Live Original resource control issuance required |
| `OS/🔨️modules/🏪️store/👷️worker/🧪️tests/📥️values/🟦️.ts` | 0/1 | 0/1 | Actual actor value validator required |
| `OS/🔨️modules/🧑‍💻dev/🚚️distribution/📥️source/🧪️tests/🟦️.ts` | 3/1 | 3/1 | Actual repository root required for distribution source verification |

### NEEDS-NATIVE/BROWSER

| suite | before p/f | after p/f | first error |
|---|---|---|---|
| `🌎️hub/🧪️tests/🌳️unit-census/🟦️.ts` | 4/1 | 4/1 | cargo metadata --locked --offline --format-version 1 --manifest-path /Users/ueli/Documents/semio/🌎️hub/Cargo.t |
| `M/🏃️process/🏗️build/🧪️tests/🏗️native/🟦️.ts` | 3/6 | 4/5 | Live Original resource control issuance required |
| `M/🏃️process/📦️artifacts/🏗️native-build/🎰️jobserver/🧪️tests/🟦️.ts` | 4/2 | 4/2 | Live Original resource control issuance required |
| `M/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📐️profile/🧪️tests/🟦️.ts` | 5/3 | 6/1 | expect(received).toBeLessThan(expected) |
| `M/🏃️process/📦️artifacts/🏗️native-build/🕸️units/🧪️tests/🟦️.ts` | 6/5 | 6/5 | Original native caller capabilities required |
| `M/🏃️process/📦️artifacts/🏗️native-build/🧪️tests/🟦️.ts` | 0/1 | 18/8 | Original process admission closed |
| `M/🏃️process/🧪️testing/🦀️cargo/🔒️lease/🧪️tests/🟦️.ts` | 1/2 | 1/2 | cargo metadata --format-version=1 --locked --offline --manifest-path /Users/ueli/Documents/semio/.🧬semio/🦑️rep |
| `M/🏃️process/🧪️testing/🦀️cargo/🧪️tests/🟦️.ts` | 2/5 | 2/5 | Original process admission closed |
| `M/🏃️process/🧭️routing/🏠️workspace/🧪️tests/🟦️.ts` | 0/1 | 3/1 | rustc: 1 |
| `OS/🔨️modules/🏪️store/🧩️composition/🌳️graph/🤝️group/🧪️tests/🟦️.ts` | 4/1 | 4/1 | expect(received).toBe(expected) |
| `OS/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧲️engine-surface-retention/🟦️.ts` | 8/4 | 8/4 | launch: Executable doesn't exist at /Users/ueli/Library/Caches/ms-playwright/chromium_headless_shell-1248/chro |
| `OS/🔨️modules/🔌️plugin/🏇️mounted-owner/🧪️tests/🟦️.ts` | 12/6 | 13/5 | Nested mounted oracle requires an explicit native result artifact |

### NO-DIST

| suite | before p/f | after p/f | first error |
|---|---|---|---|
| `PL/✒️writer/🗿️artifacts/✒️writer/📦️packages/🟦️typescript/🧪️tests/🪶️sqlite/🟦️.ts` | 0/1 | 0/1 | `@semio-tech/writer-writer` (package entry is dist/, not built) |
| `PL/🌍️gis/🗿️artifacts/🏔️gisterrain/📦️packages/🟦️typescript/🧪️tests/🪶️sqlite/🟦️.ts` | 0/1 | 0/1 | `@semio-tech/gis-gisterrain` (package entry is dist/, not built) |
| `PL/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/🧪️tests/🪶️sqlite/🟦️.ts` | 0/1 | 0/1 | `@semio-tech/gis-gismap-js` (package entry is dist/, not built) |
| `PL/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🟦️typescript/🧪️tests/🪶️sqlite/🟦️.ts` | 0/1 | 0/1 | `@semio-tech/lowpoly-lowpoly` (package entry is dist/, not built) |
| `PL/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🟦️typescript/🧪️tests/🪶️sqlite/🟦️.ts` | 0/1 | 0/1 | `@semio-tech/reasoning-wires` (package entry is dist/, not built) |
| `PL/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪶️consumers/🟦️.ts` | 0/1 | 16/2 | `@semio-tech/forms-js` (package entry is dist/, not built) |
| `PL/🪐️space/🗿️artifacts/🏠️home/📦️packages/🟦️typescript/🧪️tests/🪶️sqlite/🟦️.ts` | 0/1 | 0/1 | `@semio-tech/space-home` (package entry is dist/, not built) |
| `PL/🪐️space/🗿️artifacts/🪐️space/📦️packages/🟦️typescript/🧪️tests/🪶️sqlite/🟦️.ts` | 0/1 | 0/1 | `@semio-tech/space-space` (package entry is dist/, not built) |

### BROKEN-IMPORT

| suite | before p/f | after p/f | first error |
|---|---|---|---|
| `M/🗣️dsl/🧪️tests/🚮️absence/🟦️.ts` | 0/1 | 0/1 | repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts |

## Green after, red before (198)

Cause of the change per class: harness environment (runner envelope `SEMIO_SCRIPT_PROCESS_INVOCATION`, short `SEMIO_TEST_ARTIFACT_DIR`, `SEMIO_TICKET_DIR`, `SEMIO_BUILD_MODE`) and the emulated root hoist for undeclared packages; source edits are listed in `result-tst2.md`.

- `PL/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` (9/1 -> 10/0)
- `PL/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` (0/1 -> 6/0)
- `PL/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-widget-input/🧪️tests/🟦️.ts` (0/1 -> 2/0)
- `PL/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎯️selection/🧪️tests/🔬️unit/🟦️.ts` (0/1 -> 81/0)
- `PL/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` (12/1 -> 13/0)
- `PL/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🧪️change-block-field/🟦️.ts` (0/1 -> 4/0)
- `PL/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📎️references/🧪️tests/🟦️.ts` (2/1 -> 3/0)
- `PL/📕️norm/📇️registry/🧬️contract/🧪️tests/🧱️definition-ownership/🟦️.ts` (0/1 -> 3/0)
- `PL/📕️norm/🧪️tests/🔮️oracle-source-ownership/🟦️.ts` (5/1 -> 6/0)
- `PL/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-layer/🧪️tests/🔬️unit/🟦️.ts` (0/1 -> 1/0)
- `PL/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️edit-selection/🧪️tests/🔬️unit/🟦️.ts` (0/1 -> 9/0)
- `PL/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-image/🧪️tests/🔬️unit/🟦️.ts` (0/1 -> 19/0)
- `PL/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-layer/🧪️tests/🔬️unit/🟦️.ts` (0/1 -> 3/0)
- `PL/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🧪️tests/🔬️unit/🟦️.ts` (0/1 -> 9/0)
- `PL/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧮️status/🧪️tests/🔬️unit/🟦️.ts` (0/1 -> 2/0)
- `PL/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎨️fill/🧪️tests/🔬️unit/🟦️.ts` (0/1 -> 5/0)
- `PL/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🎨️paint/🧪️tests/🔬️unit/🟦️.ts` (0/1 -> 29/0)
- `PL/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📋️prepare/🧪️tests/🔬️unit/🟦️.ts` (0/1 -> 109/0)
- `PL/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📷️raster/🧪️tests/🔬️unit/🖼️assets/🟦️.ts` (0/1 -> 49/0)
- `PL/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📷️raster/🧪️tests/🔬️unit/🖼️images/🟦️.ts` (0/1 -> 25/0)
- `PL/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🔤️text/🧪️tests/🟦️.ts` (0/1 -> 44/0)
- `PL/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔷️shape/✏️coordinates/🧪️tests/🔬️unit/🟦️.ts` (0/1 -> 1/0)
- `PL/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/↔️translation/🧪️tests/🔬️unit/🟦️.ts` (0/1 -> 2/0)
- `PL/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/✏️editing/🧪️tests/🔬️unit/🟦️.ts` (0/1 -> 15/0)
- `PL/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🎨️style/🧬️schema/🧬️mutations/🖼️update-image/🧪️tests/🔬️unit/🟦️.ts` (0/1 -> 1/0)
- `PL/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🎨️style/🧬️schema/🧬️mutations/🧩️set-group-isolation/🧪️tests/🔬️unit/🟦️.ts` (0/1 -> 2/0)
- `PL/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧬️schema/🧬️mutations/✏️update-path-geometry/🧪️tests/🔬️unit/🟦️.ts` (0/1 -> 2/0)
- `PL/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧬️schema/🧬️mutations/🔷️shape-coordinate/🧪️tests/🔬️unit/🟦️.ts` (0/1 -> 1/0)
- `PL/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-layer/🧪️tests/🟦️.ts` (0/1 -> 8/0)
- `PL/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔒️protection/🟦️.ts` (0/1 -> 10/0)
- `PL/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧪️tests/🎛️history-inputs/🟦️.ts` (0/1 -> 1/0)
- `PL/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` (0/1 -> 9/0)
- `PL/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` (0/1 -> 18/0)
- `PL/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎮️commands/🩹️patch-pixel-region/🧪️tests/🟦️.test.ts` (0/1 -> 2/0)
- `PL/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🌈️scanline/🟦️.ts` (0/1 -> 1/0)
- `PL/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🚦️audit/🟦️.ts` (0/1 -> 4/0)
- `PL/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` (0/1 -> 10/0)
- `PL/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🧬️owned-native-contract/🟦️.test.ts` (0/1 -> 8/0)
- `PL/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🟦️.ts` (0/1 -> 4/0)
- `PL/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` (0/1 -> 9/0)
- `✏️s/🧑‍💻dev/🔁️rebuild/🧪️tests/🟦️.ts` (0/1 -> 4/0)
- `✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🧪️tests/🟦️.ts` (0/1 -> 3/0)
- `✏️s/🧪️tests/🏘️plugin-composition/🌀️procedural-formats/🧪️laws/🟦️.ts` (0/1 -> 7/0)
- `🌎️hub/🧩️compositions/🗄️stdio/🧩️component/🏗️compiler/♻️capture/🧪️tests/🟦️.ts` (2/1 -> 3/0)
- `🌎️hub/🧪️tests/🧱️socket-grant-command-source/🟦️.ts` (6/1 -> 7/0)
- `M/⏪️time-travel/♻️retirement/🧪️tests/🟦️.ts` (0/1 -> 9/0)
- `M/⏪️time-travel/🧪️tests/🧪️conformance/🟦️.ts` (0/1 -> 23/0)
- `M/⏯️tool-run/🧪️tests/🧪️conformance/🟦️.ts` (0/1 -> 28/0)
- `M/⏱️trace/📈️percentile/🔐️gate/🧪️tests/🟦️.ts` (0/1 -> 1/0)
- `M/⏳️async/🛑️cancel/♻️retirement/🧪️tests/🟦️.ts` (1/1 -> 2/0)
- `M/⏳️async/🛑️cancel/🎟️admission/🧪️tests/🟦️.ts` (0/1 -> 1/0)
- `M/◻️2d/📝️text/🔤️font/🧪️tests/🟦️.ts` (0/1 -> 8/0)
- `M/◻️2d/📝️text/🔤️font/🧵️run/🧪️tests/🟦️.ts` (0/1 -> 1/0)
- `M/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/↔️bidi/🧪️tests/🟦️.ts` (0/1 -> 3/0)
- `M/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/🧪️tests/🟦️.ts` (0/1 -> 9/0)
- `M/◻️2d/📝️text/🔤️font/🧵️shape/🏷️class/🧪️tests/🟦️.ts` (0/1 -> 1/0)
- `M/◻️2d/📝️text/🔤️font/🧵️shape/📇️face/🧪️tests/🟦️.ts` (0/1 -> 1/0)
- `M/◻️2d/📝️text/🔤️font/🧵️shape/🧩️context/🧪️tests/🟦️.ts` (0/1 -> 3/0)
- `M/◻️2d/📝️text/🔤️font/🧵️shape/🧩️context/🪄️apply/🧪️tests/🟦️.ts` (0/1 -> 2/0)
- `M/◻️2d/📝️text/🔤️font/🧵️shape/🧪️tests/🟦️.ts` (0/1 -> 11/0)
- `M/◻️2d/📝️text/🔤️font/🧵️shape/🧵️layout/🧪️tests/🟦️.ts` (0/1 -> 1/0)
- `M/◻️2d/📝️text/🔤️font/🧵️shape/🪄️substitute/🧪️tests/🟦️.ts` (0/1 -> 1/0)
- `M/✍️editor/⚠️error/🧪️tests/🟦️.ts` (0/1 -> 1/0)
- `M/🌱️value/♻️retirement/📋️queue/🧪️tests/🟦️.ts` (0/1 -> 4/0)
- `M/🌱️value/♻️retirement/🧪️tests/🧺️heap.ts` (0/1 -> 1/0)
- `M/🌱️value/🏷️type/🧪️tests/🚮️absence/🟦️.ts` (0/1 -> 1/0)
- `M/🌱️value/🗂️ordered/♻️retirement/🧪️tests/🟦️.ts` (0/1 -> 8/0)
- `M/🌱️value/🧪️tests/📤️api/⚠️scalars/🟦️.ts` (0/1 -> 2/0)
- `M/🌱️value/🧪️tests/🧭️fixture-placement/🟦️.ts` (0/1 -> 2/0)
- `M/🎒️pack/🌱️intrinsic/🎮️retained/🧪️tests/🟦️.ts` (0/1 -> 2/0)
- `M/🎒️pack/🌱️value/🧪️tests/🔬️schema-hash/💰️storage/🟦️.ts` (0/1 -> 2/0)
- `M/🎒️pack/🌱️value/🧪️tests/🫳️preflight/🟦️.ts` (0/1 -> 7/0)
- `M/🎒️pack/📐️format/🧪️tests/🔣️absolute-varint/🟦️.ts` (0/1 -> 2/0)
- `M/🎒️pack/📐️format/🧪️tests/🔮️oracles/🔢️leb/🧪️tests/🟦️.ts` (0/2 -> 2/0)
- `M/🎒️pack/🔤️json/📥️decode/🔣️string/📦️bytes/🧪️tests/🟦️.ts` (1/1 -> 2/0)
- `M/🎒️pack/🔤️json/📥️decode/🧪️tests/🟦️.ts` (0/1 -> 5/0)
- `M/🎒️pack/🔤️json/🧪️tests/🧱️ownership/🟦️.ts` (0/1 -> 10/0)
- `M/🎒️pack/🧪️tests/🧭️fixture-placement/🟦️.ts` (0/0 -> 1/0)
- `M/🎠️kernel/🧪️tests/📦️receipt/🟦️.ts` (0/1 -> 6/0)
- `M/🎭️actor/🧪️tests/🧭️fixture-placement/🟦️.ts` (0/1 -> 2/0)
- `M/🏃️process/🎛️owned-execution/📥️capture/🧪️tests/🟦️.ts` (0/1 -> 6/0)
- `M/🏃️process/🎛️owned-execution/🧪️tests/📤️progress/🟦️.ts` (0/1 -> 1/0)
- `M/🏃️process/🎛️owned-execution/🧪️tests/📤️style/🟦️.ts` (0/2 -> 1/0)
- `M/🏃️process/🏗️build/🗃️state/🧪️tests/🟦️.ts` (1/1 -> 2/0)
- `M/🏃️process/🏗️build/🧪️tests/🏗️managed/🟦️.ts` (0/1 -> 1/0)
- `M/🏃️process/💾️state/🧪️tests/🟦️.ts` (8/1 -> 9/0)
- `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🧱️fixed/🧪️tests/🟦️.ts` (1/4 -> 4/0)
- `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/📥️allocation/🧪️tests/🟦️.ts` (1/2 -> 2/0)
- `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/📥️receipt/🧪️tests/🟦️.ts` (3/7 -> 9/0)
- `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🗂️system/🧪️tests/🟦️.ts` (3/4 -> 6/0)
- `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🧪️tests/🟦️.ts` (12/11 -> 22/0)
- `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📥️receiver/🧪️tests/🟦️.ts` (1/1 -> 2/0)
- `M/🏃️process/📦️artifacts/🏗️native-build/📥️inputs/🧪️tests/⚙️operation/🟦️.ts` (0/1 -> 1/0)
- `M/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🧪️tests/🟦️.ts` (0/1 -> 1/0)
- `M/🏃️process/📦️artifacts/🏗️native-build/📦️observation/🧪️tests/🟦️.ts` (0/1 -> 1/0)
- `M/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📐️profile/🧪️tests/♻️preparation/🟦️.ts` (1/2 -> 2/0)
- `M/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🧪️tests/🟦️.ts` (0/1 -> 1/0)
- `M/🏃️process/📦️artifacts/🏗️native-build/🗃️stage-cache/🧪️tests/🟦️.ts` (0/1 -> 5/0)
- `M/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🎟️stage/🧪️tests/🟦️.ts` (2/1 -> 3/0)
- `M/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/🧪️tests/🟦️.ts` (0/1 -> 17/0)
- `M/🏃️process/📦️artifacts/💾️cache/📋️model/📨️receiver/🧪️tests/🟦️.ts` (0/1 -> 1/0)
- `M/🏃️process/📦️artifacts/🗃️archive/🧪️tests/🟦️.ts` (0/1 -> 2/0)
- `M/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️claims/🟦️.ts` (1/4 -> 4/0)
- `M/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️frontier/🟦️.ts` (1/3 -> 3/0)
- `M/🏃️process/🔒️leases/♻️retirement/🧭️continuation/🧪️tests/🟦️.ts` (2/8 -> 9/0)
- `M/🏃️process/🔒️leases/📦️bounded/🧪️tests/♻️preparation/🟦️.ts` (1/2 -> 2/0)
- `M/🏃️process/🔒️leases/📦️bounded/🧪️tests/🟦️.ts` (1/4 -> 4/0)
- `M/🏃️process/🧪️testing/🦀️cargo/🎯️exact/📥️invocation/🧪️tests/🟦️.ts` (1/1 -> 2/0)
- `M/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🛠️preparation/🧪️tests/🟦️.ts` (2/1 -> 3/0)
- `M/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🧪️tests/🎟️phase/🟦️.ts` (0/1 -> 1/0)
- `M/🏃️process/🧪️tests/🧭️fixture-placement/🟦️.ts` (0/1 -> 3/0)
- `M/🏃️process/🧭️routing/📥️invocation/🏃️process/⏲️expiry/🧪️tests/🟦️.ts` (0/1 -> 1/0)
- `M/🏃️process/🧭️routing/📥️invocation/🏃️process/🚪️admission/🧪️tests/🟦️.ts` (0/1 -> 1/0)
- `M/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📤️settlement/🟦️.ts` (0/1 -> 1/0)
- `M/🏃️process/🧭️routing/📥️invocation/🏃️process/🪢️lifetime/🧪️tests/🟦️.ts` (2/22 -> 23/0)
- `M/🏗️mesh-engine/🏛️ownership/🎨️attributes/🧪️tests/🟦️.ts` (0/1 -> 5/0)
- `M/📚️compiler/📖️syntax/🌳️module-tree/🧪️tests/🟦️.ts` (6/3 -> 10/0)
- `M/📚️compiler/📖️syntax/🦀️rust/📁️paths/🧪️tests/🟦️.ts` (0/1 -> 3/0)
- `M/📚️compiler/📖️syntax/🦀️rust/🧪️tests/🟦️.ts` (6/1 -> 7/0)
- `M/📡️replication/📡️wire/🎮️command/📥️ingress/🏛️ownership/🧪️tests/🟦️.ts` (2/2 -> 4/0)
- `M/📡️replication/🔗️causal/🔀️transition/🔁️fold/🗂️index/🧪️tests/🧺️set.ts` (0/1 -> 1/0)
- `M/📡️replication/🚪️io/🧪️tests/🏛️ownership/🟦️.ts` (0/1 -> 21/0)
- `M/🔲️pixels/🎨️sampling/↗️affine/🧪️tests/🟦️.ts` (0/1 -> 27/0)
- `M/🔲️pixels/📷️png/📥️decode/🧪️tests/🟦️.ts` (0/1 -> 75/0)
- `M/🔲️pixels/🖊️coverage/🧪️tests/🟦️.ts` (0/1 -> 25/0)
- `M/🔲️pixels/🖼️image/📥️decode/🧪️tests/🟦️.ts` (0/1 -> 87/0)
- `M/🕸️graph/🧪️tests/🪆️record-owner/🟦️.ts` (2/1 -> 3/0)
- `M/🖱️ui/⌨️tui/🧪️tests/⌨️input-decoding/🟦️.ts` (0/1 -> 5/0)
- `M/🖱️ui/🎨️styling/🏗️builder/🐍️python/🧪️tests/🟦️.ts` (5/1 -> 6/0)
- `M/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🎨️styling-resolve/🟦️.ts` (13/1 -> 14/0)
- `M/🖱️ui/🎨️styling/🧪️tests/🧩️suite/📐️geometry/🟦️.ts` (1/2 -> 3/0)
- `M/🖱️ui/🎨️styling/🧪️tests/🧩️suite/📦️asset-delivery/🟦️.ts` (7/5 -> 12/0)
- `M/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🔁️animation-scope/🟦️.ts` (2/1 -> 3/0)
- `M/🖱️ui/🎬️scene/🧪️tests/✂️text-splice/🟦️.test.ts` (0/1 -> 71/0)
- `M/🖱️ui/🧪️tests/📁fixture-placement/🟦️.ts` (0/0 -> 5/0)
- `M/🖱️ui/🧪️tests/📥️checked-values/🟦️.ts` (0/1 -> 4/0)
- `M/🖱️ui/🧪️tests/📦️crate-boundaries/🟦️.ts` (8/51 -> 59/0)
- `M/🖱️ui/🪟️viewport/🧪️tests/🪆️binding/🟦️.ts` (1/2 -> 3/0)
- `M/🖼️canvas/🧪️tests/🧱️ownership/🟦️.ts` (0/1 -> 9/0)
- `M/🗣️dsl/🧪️tests/🧱️ownership/🟦️.ts` (0/1 -> 6/0)
- `M/🗺️surface/🕸️node-graph/📡️scene/🧪️tests/🟦️.ts` (0/1 -> 3/0)
- `M/🚪️io/📝️text/🗿️artifact-reference/🪆️binding/🧪️tests/🏛️ownership/🟦️.ts` (1/2 -> 3/0)
- `M/🚪️io/📦️codec/🧪️tests/🟦️.ts` (0/1 -> 1/0)
- `M/🚪️io/🔢️binary/📥️source/🧪️tests/🎟️funding/🟦️.ts` (0/1 -> 1/0)
- `M/🚪️io/🔢️binary/📥️source/🧪️tests/🟦️.ts` (0/1 -> 51/0)
- `M/🚪️io/🧬️schema/🏛️ownership/🧪️tests/🟦️.ts` (1/2 -> 3/0)
- `M/🚪️io/🪶️sqlite-snapshot/🧪️tests/🏛️ownership/🟦️.ts` (0/1 -> 5/0)
- `M/🛂️manifest/🧪️tests/🧪️fault-notices/🟦️.ts` (0/1 -> 5/0)
- `M/🛠️tool-machine/🧪️tests/🧪️conformance/🟦️.ts` (0/1 -> 33/0)
- `M/🛠️tool-machine/🧪️tests/🧪️gesture-drive-law/🟦️.ts` (0/1 -> 14/0)
- `M/🧪️test/🔌️adapter/🧪️tests/🟦️.ts` (7/3 -> 10/0)
- `M/🧬️schema/✅️validator/🏗️compilation/🧪️tests/🟦️.ts` (7/1 -> 8/0)
- `M/🧬️schema/📄️source/🔗️closure/🧪️tests/🟦️.ts` (0/1 -> 10/0)
- `M/🧬️schema/🧪️tests/🧭️fixture-placement/🟦️.ts` (0/1 -> 2/0)
- `M/🧵️job/⏱️context/📦️owner/🧪️tests/🟦️.ts` (5/1 -> 6/0)
- `M/🧵️job/♻️retirement/📄️payload/🧪️tests/🟦️.ts` (1/3 -> 4/0)
- `M/🧵️job/♻️retirement/🔔️wake/🧪️tests/🟦️.ts` (0/1 -> 1/0)
- `M/🧵️job/👷️worker/🎟️turn/🧪️tests/🟦️.ts` (0/7 -> 7/0)
- `M/🧵️job/👷️worker/🔀️erasure/🧪️tests/🟦️.ts` (3/2 -> 5/0)
- `M/🧵️job/👷️worker/🚪️admission/🏗️preparation/🧪️tests/🟦️.ts` (1/2 -> 3/0)
- `M/🧵️job/📬️outcome/🎟️descriptor/🧪️tests/🟦️.ts` (1/1 -> 2/0)
- `M/🧵️job/🔭️preview/🎟️drive/🧪️tests/🟦️.ts` (0/1 -> 1/0)
- `M/🧵️job/🧪️tests/📏️close-demand/🟦️.ts` (1/4 -> 5/0)
- `OS/🔨️modules/♾️infinite/🌍️world/🏔️terrain/🚦️ownership/🧪️tests/🟦️.test.ts` (0/1 -> 4/0)
- `OS/🔨️modules/🌉️mcp/📡️stdio/📋️protocol/🧪️tests/🟦️.ts` (0/1 -> 2/0)
- `OS/🔨️modules/🌊️flow/🧪️tests/📋️declarations/🟦️.ts` (0/1 -> 1/0)
- `OS/🔨️modules/🏪️store/🕰️history/📐️planning/🎟️admission/🧪️tests/🟦️.ts` (3/1 -> 4/0)
- `OS/🔨️modules/🏪️store/🕰️history/📐️planning/🧪️tests/🟦️.ts` (2/4 -> 6/0)
- `OS/🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🧪️tests/🔬️unit/🟦️.ts` (3/1 -> 4/0)
- `OS/🔨️modules/🏪️store/🧩️composition/🚪️open/🌱️genesis/🧪️tests/🔬️unit/🟦️.ts` (0/1 -> 2/0)
- `OS/🔨️modules/📇️directory/🚪️io/🧱️binary/🔐️descriptor-digest/🧪️tests/🟦️.ts` (6/2 -> 8/0)
- `OS/🔨️modules/📡️spr/📜️history/🛬️decode/📋️records/📦️paged/🧪️tests/🟦️.ts` (0/1 -> 1/0)
- `OS/🔨️modules/📡️spr/📜️history/🛬️decode/🗂️file/📚️frames/🧪️tests/🟦️.ts` (0/1 -> 1/0)
- `OS/🔨️modules/📡️spr/📜️history/🛬️decode/🗂️file/🧪️tests/🟦️.ts` (0/1 -> 1/0)
- `OS/🔨️modules/📡️spr/🧵️channel/🏛️ownership/📥️command-ingress/🧪️tests/🟦️.ts` (1/1 -> 2/0)
- `OS/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎟️native-caller/🧪️tests/🟦️.ts` (5/1 -> 6/0)
- `OS/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/📬️outcome/🧪️tests/🟦️.ts` (3/3 -> 6/0)
- `OS/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/💼️job/📤️output/🧪️tests/🟦️.ts` (2/1 -> 3/0)
- `OS/🔨️modules/🔌️plugin/🎮️command/🎟️lease/🧪️tests/🟦️.ts` (2/1 -> 3/0)
- `OS/🔨️modules/🔌️plugin/🏛️ownership/📥️command-ingress/🧪️tests/🟦️.ts` (1/1 -> 2/0)
- `OS/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🧩️composition/🟦️.ts` (35/3 -> 38/0)
- `OS/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🧱️ports/🟦️.ts` (0/1 -> 1/0)
- `OS/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🏗️startup/🧪️tests/🟦️.ts` (3/1 -> 4/0)
- `OS/🔨️modules/🔌️plugin/🧠️interpreter/🧪️tests/🔌️exports/🟦️.ts` (0/1 -> 4/0)
- `OS/🔨️modules/🔌️plugin/🧩️composition/📨️emission/🌱️genesis/🧪️tests/🟦️.ts` (6/1 -> 7/0)
- `OS/🔨️modules/🔌️plugin/🧬️schema/🪶️sqlite/⚠️refusal/🧪️tests/🟦️.ts` (11/1 -> 12/0)
- `OS/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🧪️tests/🟦️.ts` (41/2 -> 43/0)
- `OS/🧪️tests/📦️crate-boundaries/🟦️.ts` (115/1 -> 116/0)
- `OS/🧪️tests/🧪️attached-document-replacement/🟦️.ts` (0/1 -> 3/0)
- `OS/🧪️tests/🧪️document-port-control-turn/🟦️.ts` (0/1 -> 8/0)
- `OS/🧪️tests/🧪️folder-archive-persistence/🟦️.ts` (0/1 -> 3/0)
- `P/📓️print/📦️artifacts/📖️pdf/🧪️tests/🟦️.ts` (0/1 -> 10/0)
- `P/📓️print/🔨️modules/📊️visualization-gallery/🧪️testing/📏️measurement/📥️pdf/🧪️tests/🟦️.ts` (0/1 -> 2/0)
- `P/📓️print/🔨️modules/📊️visualization-gallery/🧪️testing/📏️measurement/🧪️tests/📏️ownership/🟦️.ts` (0/1 -> 2/0)
- `P/📓️print/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` (0/1 -> 8/0)
- `P/📓️print/🧪️tests/🚪️typed-inference/🟦️.ts` (0/1 -> 7/0)
- `P/🧪️tests/🧭️fixture-placement/🟦️.ts` (0/1 -> 3/0)
- `🧰️framework/🧪️tests/🧱️fixture-ownership/🟦️.ts` (0/1 -> 1/0)
