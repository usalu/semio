# P8yz-b Procedural3d Retained Mounted Source/Static Implementation

Date: 2026-08-26  
Scope: the five leased Procedural3d files, three local language-neutral law fixtures, and this report. No shared Store guard, root script, Cargo manifest/lock, Procedural2d, peer caller, or unrelated concurrent source was edited.

## Result

P8yz-b is audit-ready at the permitted source/static boundary. The Procedural3d whole-buffer Wasm caller is removed. The current raw-ingress census is exactly **9 occurrences: the shared Store guard plus eight untouched peer callers**.

The result is a retained, bounded interactive feature rather than a run-to-completion compatibility wrapper. A fixed four-operation mounted registry owns page/item/byte/output/control credits, operation and generation identity, progress/checkpoint/preview/terminal slots, leases, retry/resume/ACK state, cancellation, and incremental close. Begin, page preflight/admit, seal, poll, output take/resume/retry/ACK, load ACK, cancel, and close are explicit bridge transitions. Rejected credit controls construct no operation or producer and repeated rejection does not consume later admission capacity.

The worker-owned snapshot session accepts only the fixed P3D3 discriminator, rejects P2D2 before semantic allocation, and passes every byte after the discriminator unchanged through the canonical SPK source → anchor → segment/raw-DEFLATE → catalog → value → fixed typed Procedural3d owner route. The principal proof uses non-empty neuron and output-preview widgets, a semantically consistent synapse, non-empty layout for both widgets, recursive neural/dictionary/cluster/generation values, exact synapse row equality and field-delimited digest, exact layout equality, full typed equality, unchanged canonical-ingress digest, and terminal-empty close.

The mutation source owns all fourteen P3 variants, including the P3-only delete-widget-position, plus history edit/meta/cursor, inverse, redo, checkpoint, conflict, recursive child, control, and output authorities. Combined structural depth remains exactly 12. Initial store construction is direct and incremental: it validates and copies the initial snapshot, replays applied mutations across all P3 fields, observes inverse/redo lanes, restores cursor/checkpoint state, builds from the initialized runtime, and retains displaced candidates/values until bounded retirement.

Publication authority binds operation, generation, base revision, and parent revision. The production law drives a non-empty P3 envelope through the real VcsArtifactApp maintenance route, validates authority immediately before candidate construction and atomic swap, proves all-field/all-fourteen semantic equality and digest, explicitly ACKs, and preserves the last valid store for missing/wrong-operation/wrong-generation/wrong-base/wrong-parent candidates. Ordinary populated drop remains fail-loud; cancel, fault, panic, lost lease, interrupted close, stale/ABA handles, and terminal-empty idempotence have executable laws.

## Small Interactive Feature and Independent Oracle

The language-neutral fixture 🔣️p8yz-b-third-party-oracle-laws.json specifies one bounded interaction: move widget source from (1, 2) to (12.5, -8.25) while preserving a two-widget, one-synapse, two-layout-entry P3 graph. Its exact expected semantic result is:

~~~json
{"widgetCount":2,"synapseCount":1,"layoutCount":2,"movedId":"source","x":12.5,"y":-8.25,"synapseId":"source-preview","fromPort":"solid","toPort":""}
~~~

The Rust law exposes only the private, owned Procedural3dSemanticOracle interface. SerdeJsonMoveOracle is test-only, uses the already present serde_json dependency behind that interface, and returns the owned Procedural3dSemanticResult; no third-party type crosses the interface and no dependency or manifest changed. The subject applies the real retained MoveWidget mutation. Subject and oracle compare by exact owned equality and the same field-delimited semantic digest.

The independent Rust oracle was **not run**, because running it requires Cargo and Cargo was forbidden while parallel Rust packets were active. The fixture itself was independently evaluated with standalone Bun and matched its exact expected semantic result. No runtime-oracle claim is made.

## Static Regression Gate

The domain-local static verifier limits the Wasm scan to production and separately limits its mounted scans to the canonical typed snapshot session, envelope snapshot-field authority, and envelope mutation-field authority.

Each mounted production slice fails on whole-hex helpers, ArtifactPack, whole Vec<u8>, RecordValue, decode_pack, decode_document, whole serde JSON decode, direct ArtifactStore::new, generic diff/apply, or generic clone. The same law requires all five canonical layers, P3D3, forbidden P2D2, combined depth 12, the P3-only owner, bounded yield/fuel checks, all fourteen catalog rows, the lifecycle ledger, and the owned no-runtime-dependency oracle declaration.

## Raw Ingress Census: 10 → 9

The accepted P8yz-a report recorded **10** matches: one shared guard and nine remaining peer callers. Procedural3d was one of those nine callers. This packet removes only that P3 caller, so the live census is **9**:

- shared guard: framework Store;
- peer callers: shooting, FEM 2D, FEM 3D, CAD, Puzzle 5D, Puzzle 3D, framework directed DAG, and framework flow VCS.

Neither Procedural2d nor Procedural3d appears in the current match list. No shared guard or peer caller was edited.

Exact command and result:

~~~sh
rg -l --glob '*.rs' 'reject_whole_buffer_artifact_envelope_ingress' | sort
rg -l --glob '*.rs' 'reject_whole_buffer_artifact_envelope_ingress' | wc -l
~~~

Result: the nine paths above; final count 9.

## Verification Executed

### Rust parse/format, diff hygiene, and regions

~~~sh
rustfmt --edition 2021 --check '✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs' '✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs' '✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs' '✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs'
git diff HEAD --check -- '✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs' '✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs' '✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs' '✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs' '✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🧪️tests/mutate-procedural-3d-1/component.feature'
for file in '✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs' '✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs' '✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs' '✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs'
do
  a=$(rg -c '^\s*//#region' "$file")
  b=$(rg -c '^\s*//#endregion' "$file")
  test "$a" = "$b" || exit 1
done
~~~

Exact result: PASS rustfmt=4 diff-check=5 region-balance=4.

rustfmt --check proves Rust parse/format shape only; it is not reported as a typecheck or runtime test.

### Language-neutral fixtures and exact semantic result

~~~sh
bun -e 'const fs=require("fs"),p=process.argv.slice(1).map(x=>JSON.parse(fs.readFileSync(x,"utf8"))),[l,o,r]=p,e=r.expected,w=new Set(r.input.widgets.map(x=>x.id)),s=r.input.synapses[0];if(l.discriminator!=="P3D3"||!l.forbiddenDiscriminators.includes("P2D2")||l.route.length!==7||l.interaction.monolithicCompatibilityWrapper||o.mutationOwners.length!==14||o.combinedDepth!==12||!w.has(s.from)||!w.has(s.to)||e.widgetCount!==2||e.synapseCount!==1||e.layoutCount!==2||e.x!==12.5||e.y!==-8.25||r.oracle.ownedInterface!=="Procedural3dSemanticOracle"||r.oracle.runtimeDependency)throw Error("P8yz-b fixture law");console.log(JSON.stringify({status:"PASS",discriminator:l.discriminator,routeLayers:l.route.length,mutationOwners:o.mutationOwners.length,combinedDepth:o.combinedDepth,semanticResult:e,oracle:r.oracle}))' '✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🔣️p8yz-b-retained-mounted-laws.json' '✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🔣️p8yz-b-owner-catalog-laws.json' '✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🔣️p8yz-b-third-party-oracle-laws.json'
~~~

Exact result:

~~~json
{"status":"PASS","discriminator":"P3D3","routeLayers":7,"mutationOwners":14,"combinedDepth":12,"semanticResult":{"widgetCount":2,"synapseCount":1,"layoutCount":2,"movedId":"source","x":12.5,"y":-8.25,"synapseId":"source-preview","fromPort":"solid","toPort":""},"oracle":{"library":"serde_json","scope":"test-only-existing-dependency","ownedInterface":"Procedural3dSemanticOracle","runtimeDependency":false}}
~~~

### Mounted production source slices

~~~sh
bun -e '<scan mounted canonical snapshot production for 14 forbidden whole-route patterns and require all five retained layers>' '.../snapshot/💾️binary/🦀️component.rs'
bun -e '<scan both envelope field production slices for the same 14 forbidden patterns and require eight P3 discriminator/depth/yield/initializer/publication tokens>' '.../mutations/💾️binary/🦀️component.rs'
~~~

Exact results:

~~~json
{"status":"PASS","scope":"mounted-snapshot","forbidden":14,"layers":5}
{"status":"PASS","scope":"mounted-envelope-fields","productionSlices":2,"forbidden":14,"required":8}
~~~

### Stale adaptation scan

~~~sh
rg -n 'p8yz-a|two_dimensional|clear-widget-layout\.3d-only|procedural3d-mo$' <the P3 Wasm, snapshot, and mutation files> || true
~~~

Result: no matches.

## Deferred Runtime Gates

No Cargo, Nx, Wasm, browser, or timing command was run. The exact final commands after the parallel Rust-source embargo lifts are:

~~~sh
cargo test -p semio-s-plugin-procedural small_move_widget_feature_matches_the_test_only_third_party_oracle -- --nocapture
cargo test -p semio-s-plugin-procedural domain_local_static_verifier_rejects_raw_routes_and_proves_three_dimensional_coverage -- --nocapture
cargo test -p semio-s-plugin-procedural --lib
cargo check -p semio-s-plugin-procedural --target wasm32-unknown-unknown
~~~

The native test run must cover the retained snapshot equality/digest, all-fourteen decode/replay, zero/max/+1 and repeated-control credits, insufficient fuel/deadline, cancel/fault/panic/interrupted close, stale/ABA/wrong/lost handles, publication failure matrix, real VCS maintenance replacement, explicit ACK, and incremental retirement. The Wasm target check is required because the public bridge is target-gated. Browser and watchdog/timing matrices remain coordinator-owned final gates.

## Changed Files

Production and existing language-neutral feature:

1. ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs
2. ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs
3. ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs
4. ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs
5. ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🧪️tests/mutate-procedural-3d-1/component.feature

New local language-neutral fixtures:

6. ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🔣️p8yz-b-retained-mounted-laws.json
7. ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🔣️p8yz-b-owner-catalog-laws.json
8. ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🔣️p8yz-b-third-party-oracle-laws.json

Ticket report:

9. .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/EVERY-TOOL-INTERACTIVE-JOB-MIGRATION/📓️codex-p8yz-b-procedural3d-retained-mounted-source-static-implementation-2026-08-26.md

Existing ticket-local P8yz-b scratch files were retained. Ticket close/reopen was not attempted because repository MCP ticket tools were unavailable to this agent and coordinator ownership was explicit.
