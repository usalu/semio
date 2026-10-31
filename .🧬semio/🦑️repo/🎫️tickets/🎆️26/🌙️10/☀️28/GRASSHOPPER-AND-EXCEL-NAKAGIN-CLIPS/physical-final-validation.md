# Final Physical and Structural Validation

The corrected current reconstruction passes 187 physical and structural assertions. This includes the newly confirmed panel-corner defect: exact flat-panel corners and lining-quarter vertices are preserved while the circular aperture retains its original N-angle loop. Every tested material patch remains closed, positively oriented and manifold. Final renderer, integrated images and native scenes have their own acceptance gates.

| Domain | Assertions | Result |
|---|---:|---|
| shared | 59 | Pass |
| 🚶️circulation | 18 | Pass |
| 📦️casing | 22 | Pass |
| 🧱️panel | 3 | Pass |
| 💊️detail | 9 | Pass |
| 🪟️window | 2 | Pass |
| 🧭️poses | 2 | Pass |
| 📍️survey | 6 | Pass |
| 🪟️unit | 8 | Pass |
| 🔧️mounts | 4 | Pass |
| 💡️light | 2 | Pass |
| 📏️dimensions | 13 | Pass |
| 🗼️core | 12 | Pass |
| 🌉️bridge | 7 | Pass |
| 🏬️podium | 10 | Pass |
| 🏭️roof | 10 | Pass |

The physical run passes all 135 assertions with its entire broad model-directory map unchanged between the original endpoints. Hardened acceptance verifies all eleven expected names/counts and zero errors/failures. Later integration changed five exactly named renderer inputs, which none of these physical tests consumes; current physical dependency maps still match both tested endpoints. The strict rejected recheck and independent scope audit are recorded in physical-verification-acceptance.md. The structural 52 runner also has unchanged endpoint maps, and all overlapping model/data/test hashes agree.

Canonical physical manifest SHA256: `4307178a632167490a69cf730bb869333bb79ca47d63b79a7972f6206f4820af`. Repository-relative map keys use forward slashes on every platform; the original raw run record is preserved unchanged.

The prior 184-assertion snapshot is retained in physical-validation-before-panel.md as superseded chronology. No old successful result was promoted across the panel repair.

Independent AJV checks accept the current authored model and reject 29 shared, 5 casing, 4 window-unit, 3 mount, 4 pose, 3 panel, 12 bridge, 8 podium/sign, 6 roof and 9 fixture variants. All 1397 source-bearing scalar leaves retain registered provenance. The acceptance predicate/schema independently reject incomplete, duplicate, empty, wrong-count and failed domain records.

## Physical Input Map

| Repository Path | Semantic SHA256 |
|---|---|
| 🎓️teaching/🏛️architecture/🏢️nakagin/🐍️.py | `87255d8965e795fc9c5a00fed16fb8de67184b94552a72006f5d4959cdbaa084` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🔣️.json | `a497edd3b5fd9392889177f0b226f6e5becdb9b3cba26b48a20159bf680e83cf` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🌉️bridge/🔣️.json | `3bab3a7f32b6baaaedbf792238f87da0b9a19b0dc9a1cc9fd84fe02ce78587d3` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🏬️podium/🔣️.json | `7bf165aa969ad207d654ce6776772f33e1ff0491cc06a102a05f354cdff5af53` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🏭️roof/🔣️.json | `17a0476703eaf3cb1f703442f788010897c39ee98628f2517b47a31f0eca6648` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/💊️detail/🔣️.json | `f09a0314cbea08233d5a2f1dfdcb33f9e48244143a5844465b4b5adc25e6e281` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/💡️light/🔣️.json | `94c0448475d12df90f87353eca9dae4b1cdb88ab1ae995683959be751bf0bf4c` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/📍️survey/🔣️.json | `f21057b8d0e9199eaf4cbb458331f4add5574e4abdf61fdcd2691411b9dad95c` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/📏️dimensions/🔣️.json | `73d54f96e23e772602a563b8c7a14d135887062d1eb81bba998bdadbc6c6dbe9` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/📦️casing/🔣️.json | `1c8f6ea45360a28dfac974a7068d4eff299bc1ad52f9648ac1d9544989383014` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🔣️.json | `3a7a00e0dd6a4af54a2783f9533d04d335506715d8f0b50e2a4093d00fe72d8f` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🔧️mounts/🔣️.json | `14a45e4f2fdd914ea136ed7924bf58ace23df7d2f27951c8e2cffda46ee3d513` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🔧️mounts/🔣️.schema.json | `81c7fb905989e480e57514a34676370a55658706e63196e0f2b6b90a59f20ad7` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🗼️core/🔣️.json | `bd2a8f72d060228ab66f44e864cc290d6a0e97e315a11c6064d027409a5bb33a` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🚶️circulation/🔣️.json | `e2b8fa628969c0f7498361139b34085c871333e99c07b2aa9ee7167301bf2ff7` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🧭️poses/🔣️.json | `6bda1618bdee9725fff1ab8503c84e7a9334b11d6ef19e6393c119bee3c7a1f0` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🧭️poses/🔣️.schema.json | `30c8e5dab9992c3a61a833b0e9f425a7c92d9b8fded23a8e395205d9e64a08b7` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🧱️panel/🔣️.json | `76035c987e4336d06fc2ae26273f09cf6ac6a03c854c244e29540913b9a5c4af` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🧱️panel/🔣️.schema.json | `3b5e6c7fddb28f478c9e898447e282af2e39db0169761620e3d3b6d063ac8c9d` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🪟️unit/🔣️.json | `71d79d1b40d2877955b84e50924d576d34ca3c6b606d889b1b123595cf633cb7` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🪟️window/🔣️.json | `b2e695b81348dd23ea2f46da0199116a074e9d5552ef364575161178b341ae90` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🌉️bridge/🐍️.py | `b77918f38b342cdf208f17f4b3acc651e5a898f528196cdce82c6985c5b8e5c1` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🌉️bridge/🟨️.cjs | `fbe69b2ea436533f2c6d2890967771630a3d19a6cf52a7ab7091113270895128` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🏬️podium/🐍️.py | `c9472d0bcfd5020509e629593b476298b90129219b29c7a4afe4bcaa70840f37` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🏬️podium/🟨️.cjs | `17566f21af14fa19f24f97110d4c08854cd47eb08c2260922651267ac385e8c2` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🏭️roof/🐍️.py | `1ce5669b9e78c0b9a71b6ba0eaf2c727953a0ff46195c3d8ba881b3347c5c398` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🏭️roof/🟨️.cjs | `f43f2e42f810bc4fab082146722d5bbe21f04c7d68affb1c21cb4a6f7d24b8f7` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🐍️.py | `51b69702a340255ff6bc546185862a95e1d826f7f6fb83ca0b69590db2ee9373` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/💊️detail/🐍️.py | `e093498512bd48e51acf9988523a662a90792cabcd37b6dde80cf25a97241ce9` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/💊️detail/🟨️.cjs | `28ba6d97ba3e516f3e275f30c05d36b67c8efefda6542d3d22f328bfdba37214` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/💡️light/🐍️.py | `bc43166644abf45e475da3b7cffc1dbb495bdf20d2ccee77e56d730345f560a3` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/📍️survey/🐍️.py | `01c7181a5258889659a109351454d843e043d39d9f96b797b3f4efffa3f24efb` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/📏️dimensions/🐍️.py | `a9fd75f7f0bccea4ae108ae61c226c861efa22635f071d95aeb2a38f5d531451` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/📦️casing/🐍️.py | `c428be30e1e3b863556f4380b4009380cd377d5aad18ad7aab0ac9704587d753` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/📦️casing/🟨️.cjs | `8d93429fb31b6c56d65355e92c216d4d6c5446a539455cd3f35e5a89c59e5e6d` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🔧️mounts/🐍️.py | `33061805e0452ea8269b4e750789fe14077da7b89cb5bdf5bde401730ce9902c` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🔧️mounts/🟨️.cjs | `ff85310f7ea6fd0bd84a7a421bafe5eae555017a9efcd6842ea8520cec4ff383` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🗼️core/🐍️.py | `080f19a2f7bc1777597d46445313a48d7c85b2861414becde448a297e96ffcb8` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🚶️circulation/🐍️.py | `e50dc411966e2e2e962352c4c51140e4f7600bdf0b44773ebfe7af70e71c9a75` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🟨️.cjs | `ebcc18f28a105bf00c453278021f61d8c60a7d566e0fe78f973e0b813e0c8989` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🧭️poses/🐍️.py | `607578233e6f8db36233791916cea13d1805f265ed44c5144f7559945ffac316` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🧭️poses/🟨️.cjs | `231996b08afde43e1e3f6b01d95797877d82862d3303b7fd6616bb1198bc53bc` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🧱️panel/🐍️.py | `477002df211bea10557655ee38ac6d8c82270b0ed1fa4434eb4868a56ecc1451` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🧱️panel/🟨️.cjs | `1589136f3e702f130ee8460fc4c3059c3b8e7a9673b9ea266d74e4cf76a76c7a` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🪟️unit/🐍️.py | `617049af9fae1276a136a8a8bfd325f6f7afcf9a843c6b6424304daca8c18a8b` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🪟️unit/🟨️.cjs | `12c66c387aadad227d22aed5c202883ba10edad47b13d29abb17044908f217ec` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🪟️window/🐍️.py | `593e464fef8ba580e7d99b47ccf1d8e38872c62bbb4e3a7a2f3c7b8a509c126c` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧬️schema/🔣️.schema.json | `50992761d155bd4953b829281a632c3e5eb6bbf0907f21445e7eafdde5032850` |

Exactly excluded from this physical gate (each has its own rendering validation):

- `🎓️teaching/🏛️architecture/🏢️nakagin/🎨️visuals/🐍️.py`
- `🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/👁️occlusion/🔣️.json`
- `🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/👁️occlusion/🐍️.py`
- `🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🎨️visuals/🐍️.py`
- `🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🎨️visuals/🔣️.json`

## Structural Input Map

| Repository Path | Semantic SHA256 |
|---|---|
| .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/validate-structural-domains.py | `72f86a0dd88997246a4308549dcaf641c2fa56d2795cee3b01b6e5dd6af74a89` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🐍️.py | `87255d8965e795fc9c5a00fed16fb8de67184b94552a72006f5d4959cdbaa084` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🔣️.json | `a497edd3b5fd9392889177f0b226f6e5becdb9b3cba26b48a20159bf680e83cf` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🌉️bridge/🔣️.json | `3bab3a7f32b6baaaedbf792238f87da0b9a19b0dc9a1cc9fd84fe02ce78587d3` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🏬️podium/🔣️.json | `7bf165aa969ad207d654ce6776772f33e1ff0491cc06a102a05f354cdff5af53` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🏭️roof/🔣️.json | `17a0476703eaf3cb1f703442f788010897c39ee98628f2517b47a31f0eca6648` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/📏️dimensions/🔣️.json | `73d54f96e23e772602a563b8c7a14d135887062d1eb81bba998bdadbc6c6dbe9` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🔣️.json | `3a7a00e0dd6a4af54a2783f9533d04d335506715d8f0b50e2a4093d00fe72d8f` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🗼️core/🔣️.json | `bd2a8f72d060228ab66f44e864cc290d6a0e97e315a11c6064d027409a5bb33a` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🌉️bridge/🐍️.py | `b77918f38b342cdf208f17f4b3acc651e5a898f528196cdce82c6985c5b8e5c1` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🏬️podium/🐍️.py | `c9472d0bcfd5020509e629593b476298b90129219b29c7a4afe4bcaa70840f37` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🏭️roof/🐍️.py | `1ce5669b9e78c0b9a71b6ba0eaf2c727953a0ff46195c3d8ba881b3347c5c398` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🐍️.py | `51b69702a340255ff6bc546185862a95e1d826f7f6fb83ca0b69590db2ee9373` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/📏️dimensions/🐍️.py | `a9fd75f7f0bccea4ae108ae61c226c861efa22635f071d95aeb2a38f5d531451` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🗼️core/🐍️.py | `080f19a2f7bc1777597d46445313a48d7c85b2861414becde448a297e96ffcb8` |
| 🎓️teaching/🏛️architecture/🏢️nakagin/🧬️schema/🔣️.schema.json | `50992761d155bd4953b829281a632c3e5eb6bbf0907f21445e7eafdde5032850` |

Python AST, canonical JSON and normalized text identify the implementation tested. These fingerprints do not establish unsupported historical measurements; source fidelity and inferred details remain documented in final-architecture-gate.md and the domain reports.

## Final Renderer-Repair Scope Recheck

The exact unconsumed renderer contract now declares seven files after independent rereading of all eleven physical domains. The two additional occlusion schema/CJS inputs were absent from the original captured map; no additional captured dependency was removed. Under the same teaching Python3.12 runtime, the current135-proof recheck passes and retains canonical consumed hash4307178a632167490a69cf730bb869333bb79ca47d63b79a7972f6206f4820af. The physical model and all captured physical tests remain unchanged. See physical-verification-acceptance.md for the rejected cross-version AST preflight and complete scope chronology.
