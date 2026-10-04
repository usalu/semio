# Current Non-Stdio Owner Native Hook Inventory

## Scope and Priority

Read-only fresh source audit. No Cargo, runtime execution, source edits or independent passing-test claims. The149 planning candidates are discovery scope, not executable coverage. Existing reports were used to locate owners, then current source was inspected.

Three requested actual persisted owners remain missing the trait/capability/native hook mount:

- Framework FlowHostSnapshot: `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow`; ordinary parent Pack in `🌿️vcs/🦀️.rs:582`, tests capability law128. Adjacent `🧬️schema/📸️snapshot/🪶️sqlite/{🛫️projection,🛬️reconstruction}/🦀️.rs` remain drafts, with no production ArtifactSqliteSnapshot impl or Pack opt-in found. Classification: no mounted hook; semantic projection/reconstruction draft exists, not a complete controlled hook.
- Framework DagSnapshot: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag`; ordinary parent Pack `🌿️vcs/🦀️.rs:774`, tests capability law135. Adjacent handwritten SQL producers remain drafts; no trait/Pack capability/native hook mount found. Classification: no mounted hook.
- RewritingSnapshot: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting`; ordinary Pack `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:50`, capability test103. No ArtifactSqliteSnapshot impl/Pack opt-in found. Classification: no mounted hook. Do not parse its three JSON-named authored String fields into a new tree domain; bindings are actual six-variant graph PropertyValue, layout points raw binary64.

These owner gaps precede strict UnsupportedOwner defaults: their bare capability is absent, rather than an opted-in provider successfully using the default methods. Process3d is separately assigned and excluded here.

## Fresh Mounted Hook Evidence

All following source rows declare both decode_sqlite_snapshot_native and encode_sqlite_snapshot_native. They use controlled native record bridges/typed producers, not merely an ordinary codec before/after guard. Classification is actual controlled hook **mounted in source**, not compile/runtime/cumulative-allocation proof.

### Framework Space

Provider `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`: trait13, encode15, decode19.

Mount/Pack evidence: `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🦀️.rs:11`; `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🦀️.rs:728`; `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:6`.

### Framework Collection

Provider `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`: trait14, encode16, decode20.

Mount/Pack evidence: `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🦀️.rs:6`; `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🦀️.rs:228`; `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:6`.

### Framework History

Provider `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`: trait23, encode25, decode26.

Mount/Pack evidence: Store root `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:25103` mount and `:25265` Pack opt-in.

### Workflow

Provider `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`: trait41, decode43, encode46.

Mount/Pack evidence: `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🦀️.rs:11`; `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🦀️.rs:1303`.

### Plugin Flow

Provider `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`: trait5, decode6, encode7.

Mount/Pack evidence: `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:87`; `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:105`.

### Plugin DAG

Provider `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`: trait25, decode57, encode72.

Mount/Pack evidence: `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🦀️.rs:23`; `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:185`.

### Semio Graph

Provider `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`: trait14, encode17, decode19.

Mount/Pack evidence: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/🦀️.rs:514`; `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/🦀️.rs:636`.

### Puzzle2d

Provider `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`: trait180, encode182, decode188, trait197, decode204, encode212.

Mount/Pack evidence: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:10`; `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:87`; `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:731`.

### Puzzle3d

Provider `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`: trait406, decode408, encode423, trait880, decode888, encode891.

Mount/Pack evidence: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:80`; `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:110`; `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:1027`.

### Puzzle5d

Provider `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`: trait45, decode47, encode48, trait148, decode152, encode153.

Mount/Pack evidence: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:7`; `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:109`; `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:713`.

### Jack

Provider `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`: trait416, decode425, encode443.

Mount/Pack evidence: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️.rs:872`; `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs:123`.

### Layout

Provider `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`: trait81, decode86, encode90.

Mount/Pack evidence: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:107`; `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:139`.

### Raster

Provider `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`: trait134, decode139, encode145.

Mount/Pack evidence: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:11`; `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:91`.

### Block3d

Provider `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`: trait332, decode334, encode349.

Mount/Pack evidence: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:81`; `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:123`.

### Block5d

Provider `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`: trait376, decode378, encode393.

Mount/Pack evidence: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:102`; `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:146`.

## Limits of Classification

Semio Graph is the actual declared graph subset found in the inventory; no separate plugin directory named graph exists. Plugin DAG and Framework Infinite DAG are distinct owners. Workflow is the OS WorkflowSnapshot, with RunArtifact independently owned. Puzzle Play wrappers delegate actual native hooks to typed owners, and do not establish separate semantic models.

Raster uses an owner-authored flat RasterNativeDocument and Jack uses its existing JackPackRecord, with explicit controlled conversion back to actual retained models. These transient native records are not SQL opaque payload columns. Schema fidelity and exact raw words still need each owner's genuine Native laws and independently queried physical file evidence; the hook inventory does not establish that proof. Existing recursive controlled types, partial retirement, schema and provider indexes can still contain gaps below a mounted hook, so method presence must not be used as full completion.

Store's actual erased import invokes encode_sqlite_snapshot_native directly after reconstruction/subset validation (`🏪️store/🦀️.rs:10931`), so an absent optional preflight override alone is not a missing native output hook. Existing strict decode/encode defaults remain refusal authorities for unimplemented owners. No new fallback or opaque carrier was authored in this audit. No currentmatching runtime receipts were verified, therefore no passing-test status is assigned to any row.
