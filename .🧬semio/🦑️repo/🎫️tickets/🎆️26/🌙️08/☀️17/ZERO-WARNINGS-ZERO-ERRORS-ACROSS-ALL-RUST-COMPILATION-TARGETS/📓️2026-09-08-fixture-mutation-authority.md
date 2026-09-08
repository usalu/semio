# Fixture Mutation Source Authority

Native604 reported 14 source-authority errors in three OS fixture mutation families. Their canonical descriptors and payload schemas already exist under each test's mutation collection, but the Rust files were placed in flattened sibling directories and parent modules still point there. The planned fix restores the Rust sources beside their existing descriptors and updates only physical module paths. Rust module names and mutation behavior stay the same. No macro authority exception is needed.

The audit verified unique descriptor ownership, matching payload type names, existing payload schemas and absent Rust destinations for 11 leaves and 3 aggregates. No source changes were made by this audit.

- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/📔️registry-mutations/🦀️.rs -> 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/📔️registry/🧬️mutations/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/📔️registry-mutations-rename-mini/🦀️.rs -> 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/📔️registry/🧬️mutations/📛️rename-mini/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations/🦀️.rs -> 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-add-counter/🦀️.rs -> 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/➕️add-counter/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-add-counter-twice/🦀️.rs -> 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/✌️add-counter-twice/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-4-add-counter-four-times/🦀️.rs -> 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/4️⃣add-counter-four-times/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-add-counter-then-notify-foreign/🦀️.rs -> 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/🌐️add-counter-then-notify-foreign/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-add-counter-sequence/🦀️.rs -> 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/🔢️add-counter-sequence/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations/🦀️.rs -> 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-counter/🦀️.rs -> 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/➕️add-counter/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-missing-counter/🦀️.rs -> 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/🚫️add-missing-counter/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-observed-counter/🦀️.rs -> 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/👁️add-observed-counter/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-rejected-counter/🦀️.rs -> 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/⛔️add-rejected-counter/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-unchecked-counter/🦀️.rs -> 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/🐛️add-unchecked-counter/🦀️.rs

## Pass 617

Restored 14 Rust sources to canonical descriptor-owned locations and updated 3 parent mounts. All 17 proposed Rust files passed the Rust 2021 parser before publication. Existing inline descriptor includes were rebased to the same JSON files. Destination files were created exclusively; complete source snapshots were checked before editing and before removing the old copies. 14 old Rust files were removed; 0 concurrently changed originals were retained for review. Existing descriptors, schemas, module names, mutation bodies and wire behavior were preserved. Fresh compiler macro expansion and runtime laws are still required.

- Created: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/📔️registry/🧬️mutations/📛️rename-mini/🦀️.rs
- Created: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/📔️registry/🧬️mutations/🦀️.rs
- Created: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/➕️add-counter/🦀️.rs
- Created: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/✌️add-counter-twice/🦀️.rs
- Created: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/4️⃣add-counter-four-times/🦀️.rs
- Created: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/🌐️add-counter-then-notify-foreign/🦀️.rs
- Created: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/🔢️add-counter-sequence/🦀️.rs
- Created: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🧬️mutations/🦀️.rs
- Created: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/➕️add-counter/🦀️.rs
- Created: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/🚫️add-missing-counter/🦀️.rs
- Created: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/👁️add-observed-counter/🦀️.rs
- Created: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/⛔️add-rejected-counter/🦀️.rs
- Created: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/🐛️add-unchecked-counter/🦀️.rs
- Created: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/🦀️.rs
- Updated: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/📔️registry/🦀️.rs
- Updated: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/🦀️.rs
- Updated: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🦀️.rs
- Removed: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/📔️registry-mutations-rename-mini/🦀️.rs
- Removed: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/📔️registry-mutations/🦀️.rs
- Removed: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-add-counter/🦀️.rs
- Removed: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-add-counter-twice/🦀️.rs
- Removed: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-4-add-counter-four-times/🦀️.rs
- Removed: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-add-counter-then-notify-foreign/🦀️.rs
- Removed: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations-add-counter-sequence/🦀️.rs
- Removed: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws-mutations/🦀️.rs
- Removed: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-counter/🦀️.rs
- Removed: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-missing-counter/🦀️.rs
- Removed: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-observed-counter/🦀️.rs
- Removed: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-rejected-counter/🦀️.rs
- Removed: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations-add-unchecked-counter/🦀️.rs
- Removed: 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws-mutations/🦀️.rs

Pass 619 confirmed all 14 physical module mounts and 12 literal includes in the relocated fixture families resolve. All 14 old Rust locations are absent; all 11 descriptor owners exactly match the new leaf directories. Nine existing kernel laws covering descriptor registration, counter codecs/algebra/inversion and crate mounts, plus the Playbook JSON oracle law, were added to the runtime selections. The current ticket inventory is 339 laws in 58 groups. Their execution remains pending.

The current Mutations derive was also inspected for the missing registry helper: it generates `register_<snake-case aggregate>_descriptors`, so MiniMutation produces the exact `register_mini_mutation_descriptors` name already used by the tests. Its absence in native604 follows the rejected aggregate expansion; no replacement registration helper or compatibility wrapper was added.
