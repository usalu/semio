# Puzzle 3D Verification

The existing 3D editor includes registry-backed import/export, edit, selection, snapshot and reopen laws. The import-identity corpus and serde_json parser oracle are present in the current source. Their execution is not established by source inspection.

## Attempts

- Previous session handle 18878 was unavailable when polled.
- `repo run @semio-tech/puzzle-3d-rs:test --lang=en --skip-cache` failed during source admission before executing the target: source closure was 16,788,269 bytes against a 16,777,216-byte limit.
- `repo tools exec --cwd=…/🧊️3d cargo -- test -p semio-s-artifact-puzzle-3d --test snapshot-sqlite` initially failed to load the puzzle jobs manifest because its enclosing puzzle workspace lacked `semio-framework-trace`. Added the existing framework trace crate to that workspace. A repeat passed manifest loading but the compiler ingress refused direct Cargo without the repository launcher and authentic enforcing store.
- Calling the package script directly failed because the launcher had not provided `SEMIO_SCRIPT_PROVIDERS`.

## Remaining

Run the repository target after shared source admission is repaired, resolve any subsequent compilation and behavioral failures, and demonstrate actual UI boot, edit, save and reload. No UI or persistence execution is claimed by this audit.

## Added Acceptance Law

The registry-backed unit test `edited_document_survives_export_pack_and_history` creates an object, exports it, parses the result with serde_json, compares every exported field after native pack decoding, then checks undo and redo restore the complete export. It is authored but has not executed. Existing manifest laws cover declared action-to-command mappings; they do not establish browser behavior.

A source scan of 3D `Puzzle3dObject`, `Puzzle3dVortex`, and `Puzzle3dAttraction` struct literals found no missing anchor, label, x, or y fields: five apparent matches use Rust field shorthand. This is structural inspection, not compilation. `repo tools exec --light rustfmt -- --edition 2021 --check` parsed the changed test file but reported formatting differences elsewhere in it; it is not a passing formatting check.

## Canvas Asset Prerequisite

The existing `assets-build` generator contract owned the icon pack but omitted generated Rust shortcode and digest outputs and declared no native consumers. Declared the Canvas runtime-assets Cargo project as consumer and both missing output paths. A portable Cargo fixture models puzzle → canvas → runtime-assets; the existing native-preparation test compares inferred prerequisites with Cargo metadata as an independent oracle. This fixture has not yet run. The 3D dry run has passed source admission and remains in repository graph resolution.

The owned `@semio-tech/puzzle-3d-rs:verify-runtime-lifecycle` native test target selects the editor-surface law via the package router, and its fixture-driven Bun router check passed (1 test, 3 assertions). The native target is queued for capacity. The normal 3D dry run passed source admission, reached dependency graph resolution, and remains active.

The redundant dry run was cancelled while waiting for another process to build dependencies; the actual lifecycle target continues separately after passing source admission.

The actual `nativePreparation` resolver evaluated through registered Bun on the 3D Cargo test closure selects `@semio-tech/assets:build` along with the existing schema, DSL, graph and styling generators. This validates the new transitive contract selection; it does not prove the generator ran or the native test passed.

## Changed Paths

- `✏️s/🔌️plugins/🧩️puzzle/Cargo.toml`: shared jobs workspace trace dependency.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🎛️dispatch-and-operation-laws/🦀️.rs`: lifecycle acceptance law.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json`: owned lifecycle target.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🧪️tests/🔬️rust-package-commands/🟦️.ts` and `🧪️tests/🔣️.json`, `🧪️tests/🟦️.ts`: lifecycle routing and portable route fixture.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: native asset consumer and output ownership.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧫️fixtures/native-preparation/🧫️cases.json`: transitive puzzle asset dependency fixture.

## Browser Lifecycle Scenario

Authored `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧊️3d-lifecycle/🟦️.ts` and `🔣️.json` as a real React time-travel scenario, exposed by `@semio-tech/puzzle-3d-rs:test-browser-lifecycle`. It requests en/de and uses the live palette, add-object dialog, rendered world pointer selection, local-folder fresh-peer readback, reload, undo and redo. TypeScript parser reports no syntax diagnostics; this is not a browser execution. The scenario has not run, and its selectors and archive semantics remain to be proven against a live host. No duplicate activation was started.

The browser scenario was strengthened after independent review: it now identifies the uniquely created object, requires that object be picked on the world canvas, drags its projected moveX handle, asserts a nonzero position change, captures rotation, and compares the edited state across peer/reload; undo targets the pre-transform created state and redo the transformed state. These remain authored assertions pending live execution.

The world-pick check clears selection with actual background pointer clicks and waits for an empty selection before each real pointer attempt, then requires the created ID to appear. Pick and clear deadlines are fixture inputs. Renderer source has no object screen-coordinate stamp; the scenario uses a bounded canvas grid. Selector, background clearing and timing behavior remain unverified until the browser run.

## Supported Cargo Gate

`repo cargo --store=shared --configuration=prototype --parallelism=1 -- test --manifest-path <3d>/Cargo.toml -p semio-s-artifact-puzzle-3d-editor --test editor-surface edited_document_survives_export_pack_and_history -- --nocapture` passed launcher source admission and planning, refreshed the 3D Cargo.lock, then exited 1 at Rust source admission with `Missing or ambiguous Rust module: icon_pack_digest`. The asset generator had not run; no compiler or native test result was produced. The normal lifecycle target remains attached behind the dependency build and declares that generator as a native prerequisite.

The genuine `repo run @semio-tech/assets:build --lang=en` is active as session 2319 and passed source admission. The pre-dependency recovery command router admits only its fixed frontend, maintenance and reset owners; the assets script requires an issued parent invocation, so no direct standalone script execution has been used. The owner must run before supported Cargo source admission can read the generated digest.

## Inline Import Capacity

The inline object branch bypassed the text import byte cap. Added allocation-free `bounded_json_len` to pack-json and use it before cloning in 2D and 3D importers. Portable fixtures cover exact boundaries with ASCII, escaped quotes, newline, and multibyte text; the boundary laws use serde_json as the independent serializer. The 3D import acceptance route now runs the new budget law and declares native test scope. Supported `repo cargo --store=shared --configuration=prototype --parallelism=1 -- test --manifest-path 🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust/Cargo.toml -p semio-framework-pack-json --test bounded-json-length -- --nocapture` exited 0: 2 tests passed, including serde vectors and the parser depth boundary. The 2D/3D caller tests have not run because Canvas icon digest generation is pending.

The 3D manifest advertises Set Active Example without action arguments; its retained work maps absent exampleId to the empty snapshot. The scenario requires a zero-instance verdict before adding. Confirmation/dispatch in the generic palette remains unverified. The empty document creates a default kind without a representation; source maps its unresolved mesh to `box`, the React host builds BoxGeometry, and the instance node exposes pick raycasting. This is source evidence only, not a browser verdict or proof of external mesh loading.

## Asset Renderer Compute-Only Check

Registered Bun evaluated the genuine `renderCatalogArtifacts("all")` twice without writing or publishing outputs. Both renders were byte-identical across 260 outputs. The icon pack parsed as version 1 with 250 plain and 29 themed entries; it measured 853,838 bytes. An independent Node crypto SHA-256 over the pack matched both the JSON digest and generated Rust constants: `d350329a912b3904c8fc32b4c1ef4c01a6d28508c194f4679013243769307a6a`. This verifies computation, not asset publication or native compilation.

Compute-only command: `repo tools exec --light bun -- -e` importing `renderCatalogArtifacts` from `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🏗️builder/📽️projection/🟦️.ts`, calling `renderCatalogArtifacts("all")` twice and asserting deep equality, Node SHA-256, byte length, JSON and Rust digest agreement, version, and nonempty themed/plain entries. It exited 0; no files were written.

A single 3D React prototype activation was started as `repo --tool=bazel run @semio-tech/framework-os-dev:activate-puzzle3d-react-prototype --lang=en` with process-local `SEMIO_JS_INSTALL=closure`; session 1948 is waiting for capacity. This is not yet a serve or browser verdict.

The normal lifecycle session 2493 was deliberately interrupted to free capacity for browser activations while it was still waiting for the shared build; terminal exit was 1 and no test ran. Fleet had associated PIDs 46152 and 4520; both reported ESRCH afterward and neither appeared in capacity or lease status. The lifecycle acceptance remains to be rerun after genuine assets publication. Assets producer 2319 and 3D activation 1948 were preserved.

Activation semantics: `activate-puzzle3d-react-prototype` builds the runtime image and exits; it does not itself serve port 6013. After successful activation, the owned `test-browser-lifecycle` scenario should run through the time-travel runner, whose `ensureDevServe` starts or reuses the real serve. No separate concurrent dev target has been started. Session 1948 remains live in dependency preparation.


Genuine asset build session 2319 terminated with exit 1: dependency build lease wait exceeded its declared maximum in concurrent storage. It did not reach publication; generated Canvas icon assets remain unproven. Activation session 1948 remained live awaiting the shared dependency build at this observation. No retry was started.
