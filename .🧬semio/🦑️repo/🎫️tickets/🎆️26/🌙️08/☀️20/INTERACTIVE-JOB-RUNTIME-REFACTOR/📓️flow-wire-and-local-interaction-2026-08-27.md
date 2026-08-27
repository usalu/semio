# Flow Binary Wire and Local Interaction Boundary

## Exact Binary Wire

The former Flow HostOnly cursor recognized a JSON command tuple, which is not the `app_commands!` operation wire. The real wire is format byte `1`, unsigned variant ordinal, then the canonical pack record body. Fields use zero-based declaration ids. Symbols are byte-sorted and interned when their UTF-8 length is at most 128 bytes or their value occurs more than once.

The shared pack scalar witness now retains an exact immutable `Arc<R>` and captures its typed projection once on the first granted step, rather than accepting a different record view each turn. Private string borrows remain pinned to that Arc and are cleared before root transfer. Scalar values are copied once, including when the source exposes atomic metadata. It compares at most one source or input byte per call, retains string comparison state for long common prefixes, verifies actual symbol ordering/references and canonical scalar bits, and latches errors until close. Close transfers the exact root back to its domain owner; nonterminal drop is guarded. No whole text clone, map, raw allocation, parser, or JSON stand-in is used by this cursor.

Flow's six HostOnly routes now use that witness over their exact retained payload root. This corrects their wire validation only: their whole host evaluation and final payload cleanup remain unfinished and are not certified by the wire tests. `setGraphParameter` still needs its dedicated app-owned retained job and registration; the shared exact payload exists but is not a working live Flow route yet.

## Validation

- Canonical kernel source target: nine binary fixture cases and four strict hostile fixtures passed in `🧪️pack-scalar-wire-source-r3-2026-08-27.txt`. Existing third-party LEB128 and IEEE754 codecs supply independent primitive output. NaN is explicitly normalized to the domain's canonical quiet-NaN bits; the third-party library's noncanonical NaN is checked semantically, not falsely treated as identical bytes.
- Canonical Flow source target: six actual command shapes and three hostile fixtures passed in `🧪️flow-real-wire-source-2026-08-27.txt`. The source test pins each ordinal to the real macro catalogue. This run also passed four Artifact recipe/Immer cases with a 4,800-byte semantic label, Config grant fixtures including one-byte preparation, shared parameter/label/canonical/identity fixtures.
- Four native pack tests are authored: actual pack-codec parity at grants 1/64/4096, exact final-root cancellation/worker transfer, malformed-input fault latching, and one-time capture of an atomic source. They have not run. The expanded source capture fixture passed in `🧪️pack-scalar-wire-source-r4-2026-08-27.txt`.
- One native app test is authored for real `FlowCommand::encode_op` across all six routes, including a 16,376-byte semantic identifier whose encoded command exactly fills the existing 16,384-byte raw admission envelope, partitions, cancellation and malformed input. It has not run. The larger generic scalar fixture remains above that app envelope and does not imply wider app admission.
- The coordinator reports the full neural native suite now passes 43/43. This supersedes the earlier historical 42-test result; no Flow lifecycle native pass has occurred.

## Native Local Interaction Finding

The real native owner is `VcsArtifactApp::interaction_store`, a persisted-local `ConfigStore<InteractionState, InteractionConfigMutation>`, plus the ephemeral-local `interaction_hover` map. The inherent `interaction_state` method combines them but clones the entire state; it is not a bounded transport reader. `PluginApp` exposes `ephemeral_snapshot`, whose interaction bytes deliberately contain only declared broadcast domains assembled by `assemble_presence_interaction`. That is not a full local selection snapshot and must not be substituted for one.

Native write authority already exists through the six framework interaction actions. `interactionSelect` parses exact targets and merge mode, computes selection, then validates and persists `InteractionConfigMutation::SetState` in the Interaction history lane. The current implementation clones state and topology and replaces the whole interaction state, so it is not a proven retained large-selection restore path. There is no identified full-local snapshot/replace transport API in the inspected PluginApp contract or renderer bridge. `pushPresence` is a remote peer update, not this local authority.

The smallest honest next seam is a registered local interaction query whose immutable captured root is emitted pagewise with generation/identity, paired with a typed local selection command through existing framework interaction authority. A renderer-only field or presence broadcast cannot supply exact tutorial save/restore semantics. The query/command must preserve local-only domains, selection anchor/mode/granularity, and cancellation/root retirement; the current whole-state action requires a retained recipe before earning large-state interaction credit.

## Tooling

`semio-framework-os-flow-core:check --args=--tests` now awaits Cargo check against the real family manifest. All seven core-test extension routers and the Flow app test router await budgeted testing, resolve the test level and forward exact filters. The authoritative launch seed and generated launch entries include core check and scalar source gates. The coordinator/peer compiler lease remains exclusive; no Cargo process was launched for this source packet.
