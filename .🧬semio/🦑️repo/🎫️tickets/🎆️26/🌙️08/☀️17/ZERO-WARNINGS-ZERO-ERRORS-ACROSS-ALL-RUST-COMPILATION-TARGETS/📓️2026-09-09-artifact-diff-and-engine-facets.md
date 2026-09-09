# Artifact Diff and Engine Facets

EN1998 fixture laws now assert that serialized artifact diffs omit selectedCheckIndex, instead of accessing a field no longer present in the schema. Puzzle2D fill code uses the current app engine namespace. Removed obsolete GIS view-layer merging and Animate presence/config application from artifact diff codecs, matching their current schema definitions. No fields were added back to artifact state.

54 Rust sources parsed. Full compiler/runtime verification pending.

- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-structural-system/🧪️tests/🏗️switches-6a829c/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗼️change-tower-mass-t/🧪️tests/🗼️raises-tower-1830cb/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️change-en-a-gr/🧪️tests/🏎️raises-en-a-gr-to-0-25/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧂️change-wall-soil-gamma-kn-m3/🧪️tests/🧂️raises-wall-soil-a32fb4/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓️change-tank-mass-t/🧪️tests/⚓️raises-tank-mass-b9822d/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️change-tower-is-chimney/🧪️tests/🏭️turns-tower-is-4d9ad2/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️change-k-soil/🧪️tests/🌱️raises-k-soil-to-262500-0/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥁️change-tank-radius-m/🧪️tests/🥁️raises-tank-a43268/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️change-retrofit-ed-kn/🧪️tests/📥️raises-retrofit-5e417c/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏋️change-wall-h-rd-kn/🧪️tests/🏋️raises-wall-h-rd-904a71/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-period-ratio/🧪️tests/⏱️raises-period-203258/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫨️change-seismic-zone/🧪️tests/🫨️raises-seismic-75dfb7/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌈️change-en-spectrum-type/🧪️tests/🌈️switches-en-401c6b/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️change-annex/🧪️tests/🌍️switches-annex-to-en/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭕️change-silo-radius-m/🧪️tests/⭕️raises-silo-90e025/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️change-v-rd-kn/🧪️tests/🛡️raises-v-rd-kn-to-925-0/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️change-importance-class/🧪️tests/🏛️switches-4ad1d6/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️change-en-ground-type/🧪️tests/🗺️switches-en-f84fe4/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✖️change-retrofit-gamma-el/🧪️tests/✖️raises-retrofit-aac27e/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️change-foundation-area-m2/🧪️tests/🔲️raises-foundation-7d34b0/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️change-retrofit-limit-state/🧪️tests/🚦️switches-69a8e9/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-wall-phi-deg/🧪️tests/📐️raises-wall-phi-d14b2b/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️change-ground-type/🧪️tests/🪨️switches-ground-50fde9/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👇️change-foundation-p-rd-kpa/🧪️tests/👇️raises-foundation-ee8ae9/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️change-silo-q-nominal/🧪️tests/📊️raises-silo-q-8fac52/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦾️change-tower-m-rd-knm/🧪️tests/🦾️raises-tower-m-e2bec9/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️change-drift-mm/🧪️tests/↔️raises-drift-mm-to-33-5/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛢️change-tank-height-m/🧪️tests/🛢️raises-tank-bd4308/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↕️change-height-m/🧪️tests/↕️raises-height-m-to-18-75/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗜️change-silo-n-rd-kn/🧪️tests/🗜️raises-silo-n-rd-0b359e/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️change-bridge-v-rd-kn/🧪️tests/🌉️raises-bridge-v-cab0cd/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️change-k-foundation/🧪️tests/🌀️raises-k-542113/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️change-silo-v-rd-kn/🧪️tests/🚧️raises-silo-v-rd-941256/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➡️change-foundation-h-ed-kn/🧪️tests/➡️raises-foundation-3a1660/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔰️change-tank-v-rd-kn/🧪️tests/🔰️raises-tank-v-rd-8af8f4/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️change-retrofit-knowledge-level/🧪️tests/🎓️switches-9d6e47/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉️change-silo-v-ed-kn/🧪️tests/📉️raises-silo-v-ed-90bfaa/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️change-tower-q-nominal/🧪️tests/💨️raises-tower-q-f7fa63/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️change-wall-height-m/🧪️tests/🧱️raises-wall-2ab8ac/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌾️change-silo-height-m/🧪️tests/🌾️raises-silo-98db8b/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️change-retrofit-rk-kn/🧪️tests/💪️raises-retrofit-16e455/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛑️change-bearing-d-rd-mm/🧪️tests/🛑️raises-bearing-d-7b8d84/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↪️change-tower-m-ed-knm/🧪️tests/↪️raises-tower-m-ed-17f812/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲️change-foundation-h-rd-kn/🧪️tests/🧲️raises-foundation-7fde4b/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕐️change-t1-s/🧪️tests/🕐️raises-t1-s-to-0-75/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️change-multiple-resisting-systems/🧪️tests/🕸️turns-multiple-d86959/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-wall-r/🧪️tests/🔢️raises-wall-r-to-2-25/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️change-mass-t/🧪️tests/⚖️raises-mass-t-to-812-5/🦀️.rs
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️change-bearing-d-ed-mm/🧪️tests/🎯️raises-bearing-d-217e26/🦀️.rs
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️set-fill-count/🦀️.rs
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️set-fill-count/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs
- ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/📝️text/🦀️.rs

Skipped concurrent changes:

