# Store Relocated Incoming Edits

Applied 219 incoming edits into 15 current extracted owners. Static source work only.

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏃️runtime/📥️lifted/🔁️replay/🦀️.rs`: 4 edits.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/📥️lifted/🏪️store/🦀️.rs`: 4 edits.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🏠️space/📥️lifted/🏪️store/🦀️.rs`: 10 edits.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/📥️lifted/🏪️store/🦀️.rs`: 65 edits.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏃️runtime/📥️lifted/🏪️store/🦀️.rs`: 5 edits.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧱️core/📥️lifted/🏪️store/🦀️.rs`: 29 edits.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🎞️live/📥️lifted/🏪️store/🦀️.rs`: 28 edits.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/👥️space-member/📥️lifted/🏪️store/🦀️.rs`: 6 edits.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧰️test-support/🦀️.rs`: 2 edits.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/📜️wire/📥️lifted/🗄️durable-group/🦀️.rs`: 25 edits.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🗄️stage/📥️lifted/🗄️durable-group/🦀️.rs`: 30 edits.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/👥️space-member/📥️lifted/🏭️operation/🦀️.rs`: 2 edits.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/📥️lifted/🚪️open/🦀️.rs`: 6 edits.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/👥️space-member/📥️lifted/🧾️document/🦀️.rs`: 2 edits.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧱️core/📥️lifted/🧾️document/🦀️.rs`: 1 edits.

A second pass applied44 context-matched edits to 9 current owners, including formerly private extracted method signatures. Five changes remain for explicit structural placement.

## Explicit Structural Placement

Placed new weaker-bound ArtifactStore impl into current engine owner; placed binding issuer capture/transfer into current stage owner; retained current three-store composer ownership and carried new decision/planning fields plus authored Debug implementations.

- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\⚙️engine\📥️lifted\🏪️store\🦀️.rs`
- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\⚙️engine\🗄️stage\📥️lifted\🗄️durable-group\🦀️.rs`
- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\🧩️composition\🗄️durable-group\🦀️.rs`
- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\🦀️.rs`

## New Owner Wiring

Kept bounded durable planning under the extracted stage owner, reused its public planners from the current durable composer, kept the budget under the wire owner, and exposed the original diagnostics owner through one-item publication. Moved the diagnostics ArtifactStore impl into the extracted engine owner.

- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\🎮️one-item\📨️diagnostics\🦀️.rs`
- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\🎮️one-item\📬️publication\🦀️.rs`
- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\⚙️engine\📥️lifted\🏪️store\🦀️.rs`
- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\⚙️engine\📥️lifted\🏪️store\🦀️.rs`
- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\⚙️engine\🗄️stage\📥️lifted\🗄️durable-group\🦀️.rs`
- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\⚙️engine\🗄️stage\📥️lifted\🗄️durable-group\🦀️.rs`
- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\🧩️composition\🗄️durable-group\📜️wire\📥️lifted\🗄️durable-group\🦀️.rs`
- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\🧩️composition\🗄️durable-group\📜️wire\📥️lifted\🗄️durable-group\🦀️.rs`
- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\⚙️engine\🧱️core\📥️lifted\🏪️store\🦀️.rs`
- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\⚙️engine\🧱️core\📥️lifted\🏪️store\🦀️.rs`
- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\🧩️composition\🗄️durable-group\🧵️planning\🦀️.rs`
- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\🧩️composition\🗄️durable-group\🧵️planning\🎒️pack\🦀️.rs`
- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\🧩️composition\🗄️durable-group\🧵️planning\📐️budget\🦀️.rs`
- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\🧩️composition\🗄️durable-group\🧵️planning\📤️source\🦀️.rs`
- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\🧩️composition\🗄️durable-group\🧵️planning\🔗️bound\🦀️.rs`
- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\🧩️composition\🗄️durable-group\🧵️planning\🧷️member\🦀️.rs`
- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\🧩️composition\🗄️durable-group\🧵️planning\🪪️decision\🦀️.rs`
- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\⚙️engine\🗄️stage\📥️lifted\🗄️durable-group\🦀️.rs`
- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\🧩️composition\🗄️durable-group\📜️wire\📥️lifted\🗄️durable-group\🦀️.rs`
- `🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\🧩️composition\🗄️durable-group\📜️wire\📥️lifted\🗄️durable-group\🦀️.rs`
