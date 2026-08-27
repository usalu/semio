# Direct-Leaf Mutation Refactor Control Plane

## Baseline

- Coordinating ticket: `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL`
- Execution baseline: `d03b1fdb6da7c4ea97043e5618d8f4098a43dff7`
- Baseline verified with `git rev-parse HEAD` on 2026-08-27.
- Governing brief: `/Users/ueli/.codex/attachments/e2132cb0-073b-498c-bbf7-780a155ec8b3/pasted-text-1.txt`
- Associated repository goal for this continuation: `🎯singlefilerepo`
- Client: `codex`
- Model: `gpt-5.6-sol`

The checkout was already extensively dirty when this continuation began. Every pre-existing modification is treated as shared work. No modifying Git command is permitted, and no unrelated change may be reverted.

## Immutable Scope

All repository-owned source, schema, codec, command, registry, catalog, fixture, oracle, and test surfaces are in scope except `compose/**`.

`compose/**` is an absolute exclusion:

- It is never inventoried.
- It is never read for mutation discovery or evidence.
- It is never modified.
- The exclusion cannot be overridden by a flag.
- Exclusion behavior is tested only with virtual filesystem fixtures.

Generated/build/cache internals, dependency trees, and ticket evidence are excluded from mutation discovery.

## Target Invariant

Every concrete mutation owns exactly one direct semantic folder and an authoritative direct file leaf:

```text
<owner>/🧬️mutations/<emoji><semantic-verb-noun>/🦀️component.rs
```

Optional child facets may split large concerns, but no simple mutation is required to contain nested mutation/diff/inverse triads. Root mutation components are transparent aggregators containing only mounts, re-exports, wrapped aggregate variants, mechanically derived delegation, leaf-descriptor registry assembly, and structural correspondence tests.

The final mutation-directory, aggregate-variant, descriptor, schema, codec identity, required language representation, catalog entry, and tests form an exact one-to-one correspondence. Sentinels, opaque whole-snapshot fallbacks, generic collection escape hatches, hidden generated implementations, suppressions, allowlists, and architectural exceptions are forbidden.

## Acceptance Contract

The 23 acceptance criteria in the governing brief are binding. Completion additionally requires:

- Direct-owner, root-purity, folder/variant, descriptor, reachability, behavior, codec, wire identity, schema, language, catalog, hidden-generation, sentinel, snapshot-fallback, shared-helper, test-presence, and compose-exclusion policies at high severity.
- Language-agnostic structural tests and third-party parser validation for the AST-aware Rust inspection feature.
- Bun package management, Nx task routing, and all permanent command implementation in the existing `📜️script.ts` hierarchy.
- Registered executable commands in `.vscode/launch.json` in the existing order and grouping.
- Exact executed-command evidence; unexecuted checks are never described as passing.
- Runtime confirmation with `[DEBUG] ` diagnostics for newly introduced runtime behavior, followed by removal of temporary diagnostics before closure.

## Path-Intent Ledger

| Task | Profile | State | Read Set | Write Set | Dependencies |
| --- | --- | --- | --- | --- | --- |
| `SOL-00` | coordinator | active | repository outside `compose/**` | this control plane; later serialized shared files | none |
| `LUNA-CORE-01` | read-only census | active | mutation roots outside `compose/**` | none | baseline |
| `LUNA-CORE-02-04` | read-only foundation audit | active | taxonomy, clean, scaffolding, policies, tests | none | baseline |
| `LUNA-ROOT-PDF-STDIO` | read-only pilot audit | active | PDF 1.7 A/any and their stdio consumers | none | baseline |

No implementation write set will be assigned until the read-only census establishes exact ownership and overlap.

## Evidence Log

### 2026-08-27

- Read the governing brief completely: 1,389 lines.
- Confirmed active Codex goal for this request.
- Confirmed repository MCP was not exposed through the host tool registry, then connected directly to the configured repository MCP stdio server.
- Read `repo://goals`; selected `🎯singlefilerepo` as the closest file-leaf taxonomy goal.
- Read `repo://tickets`; confirmed `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL` already covers the task.
- `ticket_reopen` reported `ticket is already open`; the existing open ticket remains the coordinating ticket.
- Verified execution baseline equals the brief's pinned SHA.
- Started three read-only audit lanes. No source changes were made before the audits began.
- Ran `bun nx run @semio-tech/repo-lib:test-quick` as the baseline check. The command exited `1`: the underlying `bun test ./🧪️index.test.ts` exceeded its existing 30,000 ms quick-test budget and was killed. Tests shown before termination passed, but the suite is not a passing baseline and no full-suite claim is made.
- Foundation census found 145 mutation-root Rust components, 1,559 nested mutation Rust components, 1,554 nested diff Rust components, 1,554 nested inverse Rust components, 226 direct codec components, and only 17 direct semantic components. Sixteen direct semantic components are glTF leaves; the other is OS `change-merge-policy`.
- Foundation audit identified the shared implementation seams: root `📜️script.ts`, taxonomy JSON, discovery, normalization, repository-library tests, root Nx targets, and launch configuration. Current mutation discovery is hard-coded to `✏️s`, direct leaves are invisible to implementation checks, emoji/dispatch findings are below high severity, and the legacy triad remains mandatory in policy and taxonomy configuration.
- PDF/STDIO pilot audit found PDF/A 1.7 has 16 inline variants and no leaf directories, while PDF 1.7 Any has 18 inline variants and only a non-authoritative legacy `set-snapshot` triad. Both roots centralize payload/apply/diff/inverse/tests; Any additionally centralizes mutation-specific codecs and cross-subset conformance support.
- PDF pilot parity is already inconsistent: TypeScript, GraphQL, JSON Schema, Proto, and text surfaces omit `MovePage`, `SetPageContent`, and `SetPageRotation`; Rust binary handles tags 0–17 while its specification says 0–14. These roots require semantic extraction and coordinated greenfield wire renumbering, not a blind path move.
- PDF target vocabulary follows the governing brief exactly: remove `NoMutation`; remove generic `SetSnapshot` or explicitly review it as `replace-snapshot`; use semantic leaves such as `change-document-info`, `change-object-value`, and `replace-page-content`.
- Exhaustive visible-file census found 157 mutation roots: 145 schema roots and 12 IO roots across 33 plugins plus one framework config root. The roots contain 1,569 candidate concrete directories and 226 codec-infrastructure directories.
- Of the 1,569 candidate directories, 17 have direct components and 1,552 do not. Of the missing-direct set, 1,545 have a complete legacy triad, five have only a nested mutation component, and two OS identity folders have no implementation file.
- Root components are not transparent: 141 of 145 contain `match`, 144 contain `KINDS`, 145 contain root tests, 97 contain codec functions, and 83 contain apply/diff/inverse/transform/codec behavior. Lexical enumeration found 2,165 aggregate variants and 51 folder/variant count mismatches; AST confirmation remains mandatory.
- Thirty roots have no candidate mutation folders but contain 269 inline variants. Seventy-eight STDIO roots declare both `NoMutation` and `SetSnapshot`; 48 `set-snapshot` folders remain nested and 30 snapshot fallbacks exist only centrally. `CollectionMutation` root behavior is concentrated in flow.
- The conversion order is frozen: OS config; glTF classification; PDF 1.7 pilots; remaining central-only roots base-before-subset; remaining STDIO sentinel/fallback roots; dependency-first leaf-authoritative roots; large architect/norm/block/puzzle lanes; IO codec roots after descriptors stabilize.
- Replaced the legacy taxonomy contract with direct mutation ownership: a direct `🦀️component.rs` is mandatory, while `🔺️diff`, `↩️inverse`, `🧩️plan`, `📝️text`, and `💾️binary` are optional facets. Removed the mandatory triad/composite ownership fields.
- Added the shared Rust structural inspector and virtual-source exclusion API. Its direct-taxonomy suite passed `4/4`; the combined taxonomy load/validation/direct suite passed `51/51`; `git diff --check` passed. The independent parser oracle is the installed nightly `rustc -Zunpretty=ast-tree`. No real `compose/**` path was inspected.
- Registered `clean taxonomy inventory|plan|apply|verify --kind mutation` and `new mutation` through the root script, Nx project targets, and VS Code launch configurations. Apply refuses unresolved or partial semantic rewrites, and scaffolding is idempotent without overwriting an existing owner leaf.
- Ran `bun nx run workspace:clean-taxonomy-inventory -- --kind mutation`: exit `0`, `roots=157`, `records=2344`, `violations=2224`, Nx success. The larger record count intentionally reflects the union of physical folder identities and aggregate variants while drift remains.
- Ran `bun nx run @semio-tech/repo-lib:test-quick -- -t 'direct mutation ownership'`: exit `0`; `5` passed, `247` filtered, `0` failed, `20` expectations; Nx success. The suite covers high-severity legacy nesting/root behavior/sentinel/snapshot violations, accepted direct leaves and optional facets, virtual compose exclusion, idempotent non-overwriting scaffolding, stable inventory JSON validated with Ajv, and unresolved-plan refusal.
- Reworked the frozen OS-config shape policy to discover semantic mutations from their direct owner component and to reject a missing direct leaf; legacy triad terminology and taxonomy access were removed.
- Added the language-neutral direct-mutation descriptor contract at `🦑️repo/📚️library/🔣️mutation-descriptor.schema.json`. Taxonomy now declares its JSON file kind; every direct leaf owns `🔣️component.json` with owner, semantic kind, display name, emoji, aggregate variant, payload schema, optional text opcode/binary tag, invertibility, diff participation, outcome classes, atomic/composite classification, and required language surfaces.
- Updated the OS plugin-registry validator to consume the direct component/descriptor/optional-facet taxonomy contract and removed its root-level mutation fallback, legacy triad/composite reconstruction, and compatibility wording.
- Registered all 17 required direct-mutation structural policy kinds at high severity. Descriptor validation and parity now cover folder/Rust identity, duplicate kinds/opcodes/tags, completed classifications, TypeScript, GraphQL, protobuf, JSON Schema, text, binary, and subset oracle/catalog counterparts. The compose policy is structural: the central skip set and repository-root derivation exclude `compose/**` before traversal, with only virtual fixture coverage.
- Removed mutation semantic-vocabulary suppressions, the root-total-kind exception, and the glTF outcome/message exception. Repository-owned mutation/command scanning is taxonomy-rooted and no mutation architecture may bypass direct semantics or the frozen outcome codes.
- First combined descriptor/policy regression attempt exited `1`: the scaffolder used the unstemmed taxonomy JSON filename `🔣️.json`, while the direct descriptor contract requires `🔣️component.json`. Corrected it to use `canonicalStemmedFilenameForKind`.
- Re-ran `bun nx run @semio-tech/repo-lib:test-quick -- -t 'direct mutation ownership|direct mutation taxonomy'`: exit `0`; `10` passed, `243` filtered, `0` failed, `57` expectations; Nx success. Ajv independently validates the language-neutral descriptor while nightly `rustc` remains the independent Rust parser oracle.
- Relocated PDF 1.7 Any conformance support from the mutation aggregate to `schema/🏅️conformance-support/🦀️component.rs`, mounted it at the schema owner, and rewired A/E/H/UA/VT/X production and exhaustive-test imports. Function bodies remain byte-equivalent after de-indentation; no legacy import remains; `git diff --check` passed. The full STDIO lib target compiled and entered 5,797 tests, then encountered five unrelated Binary/BCF/BMP failures after 183 passes; no PDF support failure appeared. A subset-filtered retry compiled but exceeded the existing 30-second nextest budget before a result, so no passing PDF runtime claim is made.
- Resolved the descriptor/payload-schema filename collision. The language-neutral descriptor remains `🔣️component.json`; a direct JSON payload schema is `🔣️payload.schema.json`. The rejected `🧾️component.schema.json` form violates taxonomy because `🧾️` is the JSON Lines file kind. Existing repository schemas independently establish the `🔣️*.schema.json` convention.
- Extended the scaffolder regression to request a JSON Schema, assert the descriptor reference and required-language surface, and compile/execute the generated draft-07 schema with Ajv. The first run correctly failed because the generated draft-2020-12 metaschema was unavailable to the installed Ajv entry point; standardizing the generated schema on the repository-supported draft-07 contract made the rerun pass: `5` passed, `248` filtered, `0` failed, `38` expectations; Nx success.
- Published the ticket-scoped mutation inventory at `📊️taxonomy-inventory/🔣️.json`: exit `0`, `roots=157`, `records=2344`, `violations=2237`; Nx success. This snapshot was taken while glTF and Trinity conversions were partially written, so its violation total is coordination evidence rather than a progress comparison. Root count and record count remain stable.
- OS config cutover now has five direct Rust leaves and descriptors for set-default-app, clear-default-app, change-merge-policy, sign-in, and sign-out; the root is a transparent tagged aggregate/delegator, and all 15 legacy Rust triad implementation leaves plus six identity TypeScript triad leaves are removed. Agent validation: uncached framework OS quick suite `208` passed; plugin-host opening-config suite `34` passed, `0` failed; plugin-host Rust check succeeded; Ajv descriptor/root/catalog parity `5/5` with zero unclassified values; scoped `git diff --check` clean; zero `[DEBUG]` logs.
- Independent policy review of that OS result found six remaining high-severity TypeScript parity breaches: the root TypeScript aggregate exists but only sign-in/sign-out had direct TypeScript leaves. The OS owner lane was returned to add direct TypeScript identity for the other three mutations and must demonstrate zero scoped policy violations before this root is accepted.
- Removed Ajv from the production command router. The repository-owned Draft-07 subset validator covers object/property/required/additional-property, scalar type/const/enum/range/pattern, array cardinality/uniqueness/items/contains, and anyOf semantics; mutation descriptors load their schema-first contract and validate through that dependency-free interface. Ajv remains a test-only oracle and agrees on accepted, negative-tag, missing-Rust-surface, and additional-property cases.
- The descriptor/scaffolder test was red first because the new internal validator export did not exist. After implementation, the direct ownership/scaffolder selection passed `6`, filtered `247`, failed `0`, with `30` expectations; Nx success. The pre-existing retained-command checkpoint oracle was moved onto the same internal subset (including maximum/maxItems); `bun ./📜️script.ts verify interactivity tool-jobs --self-test` reported `486 clean`.
- Tightened descriptor completion at the schema boundary: `unclassified` is no longer a legal invertibility or diff-participation value, and outcome classes require at least one entry. The scaffolder now emits atomic `explicit-mutation` / `detect` / `[applied]` semantics rather than an immediately failing placeholder. The regression was red on the old empty outcome, then passed `1`, filtered `252`, failed `0`, with `12` expectations; Ajv and the internal validator also agree that empty outcomes and `unclassified` are invalid.
- OS TypeScript parity correction completed: all five leaves now have direct TypeScript components, the root TypeScript aggregate covers all five, and all five descriptors require Rust plus TypeScript. Uncached framework OS quick suite passed `211` tests across three files. Fresh repository structural policy filtered to the OS mutation root returned zero breaches; Ajv validated `5/5`, with five TypeScript leaves, zero unclassified values, and zero empty outcome arrays.
- Patched the TypeScript OS and Rust host Nx named inputs to cover the external config/plugin sources they compile. A normal cache probe executed; a temporary `[DEBUG]` comment under the config root caused a second normal cache miss; after removing it, the next normal run matched the cache. The focused host retry encountered the shared Cargo build lock while other conversion lanes compiled; earlier uncached host coverage remains `34` passed and host check succeeded.
- Attempted `bun nx run @semio-tech/repo-lib:lint`; its configured target recursively invoked the same Nx target. The repeating process was interrupted without a lint result. No passing lint claim is made; targeted repository-library tests remain the executed validation for the foundation.
- Re-ran the complete direct-foundation selection after schema completion: `bun nx run @semio-tech/repo-lib:test-quick -- -t 'direct mutation ownership|direct mutation taxonomy'` passed `10`, filtered `243`, failed `0`, with `64` expectations; Nx success.
- Trinity JACK 1 Any static/oracle cutover reached exact `8/8` direct Rust owners, descriptors, payload schemas, TypeScript, GraphQL, protobuf, text, and binary counterparts; nested mutation implementation leaves `0`; singular `::mutation` compatibility aliases/mounts `0`; scoped structural breaches `0/17`. Ajv validated `8/8` descriptors and compiled `8/8` draft-07 payload schemas; pinned nightly rustc parsed `8/8` direct Rust owners; scoped `git diff --check` clean. Runtime compilation is still running behind a concurrent procedural Cargo build lock, so no Trinity runtime-pass claim is made yet.
- glTF coordinator gate progressed from `365` breaches (retained generation markers obscured 120 semantic/behavior blocks) to `124`, all schema parity: four direct protobuf leaves and 120 root GraphQL aggregate identities remain. Behavior ownership, wire identity, and codec ownership are now clean; the glTF owner lane is correcting the remaining schema surface.
- PDF 1.7/A reached 14 direct Rust leaves, 14 completed descriptors, and zero legacy implementation triads. The first coordinator gate found five issues: two JavaScript acronym descriptor/variant mismatches, their folder/variant mismatch, and root hidden-generation/root-purity findings from a structural-test `include_str!`. Canonicalizing the two types to `InsertJavascriptAction` / `RemoveJavascriptAction` and switching the structural catalog test to a visible runtime filesystem read reduced the scoped gate to zero violations across all 17 policy classes. Runtime/oracle closure remains in progress.
- glTF schema correction reduced its remaining `124` findings to zero: the root GraphQL aggregate now visibly assembles all 120 exact semantic identities and the four renamed extension protobuf leaves expose their add/remove required/used identities. OS config, Trinity JACK, PDF 1.7/A, and glTF 2.0 Any all return zero scoped structural findings.
- Live post-pilot census: `157` roots, `2338` records; `147` direct, `1428` legacy, `763` central-only, zero missing-state records. Global structural backlog is `2048`: codec ownership `45`, direct owner `1428`, folder/variant `90`, generic snapshot fallback `77`, hidden generation `114`, sentinel `77`, root purity `140`, shared-helper purity `77`. This is an executed progress checkpoint, not final verification.
- Four Cargo validation commands (procedural, two STDIO profiles, and Trinity) serialized behind the procedural holder of `target/debug/.cargo-lock`, leaving all conversion lanes queued for 10–18 minutes. Coordinator interrupted the three conversion agents and sent SIGINT only to their exact queued STDIO/Trinity nextest+cargo PIDs; the unrelated active procedural build was not touched. Runtime validation for glTF, Trinity JACK, and PDF/A remains explicitly deferred and must be rerun after the shared lock clears.
- Started the next disjoint production wave: Trinity Rewrite 1 Any (7 legacy records), PDF 1.7 Any (central inline + legacy/wire parity pilot), and four STDIO textual/base roots (JSON RFC8259 Any, XML 1.0 Any, SVG 1.1 Any, TXT UTF-8 Any). Write sets are disjoint; all retain the same direct descriptor/surface/catalog/test and zero-policy gates.
- Trinity Rewrite reached seven direct semantic owners and zero scoped findings across all 17 structural classes. Its combined JACK+Rewrite Rust validation is running; runtime success is not yet claimed.
- Four STDIO textual/base roots reached zero scoped findings independently: JSON RFC8259 Any `5` semantic leaves, XML 1.0 Any `6`, SVG 1.1 Any `9`, and TXT UTF-8 Any `5` (`25` total). Their combined validation also covers the deferred glTF runtime proof and is running; runtime success is not yet claimed.
- PDF 1.7 Any reached 16 direct semantic leaves and zero scoped findings. A rejected intermediate relocation produced `96` parity failures because whole codecs moved behind compatibility glue and schema roots became identity-free. The corrected shape restores canonical root text/binary components as framing plus visible 16-entry registries with no match dispatch, gives all 16 leaves direct text parse/print and binary encode/decode facets, and assembles all 16 identities in root GraphQL/protobuf. Generic set-snapshot is removed. Ajv/nightly/runtime closure remains in progress.
- Trinity Rewrite 1 Any cutover completed exact `7/7` direct Rust owners, descriptors, payload schemas, TypeScript, GraphQL, protobuf, text, and binary surfaces; nested implementation owners `0`, singular `::mutation` routes `0`, root behavior markers `0`, source `[DEBUG]` markers `0`, and the coordinator gate reports `0/17` scoped policy breaches. The test-only Ajv oracle validated `7/7` descriptors against `🔣️mutation-descriptor.schema.json`, compiled `7/7` draft-07 payload schemas, and matched `7/7` oracle catalog vectors; pinned nightly `rustc -Zunpretty=ast-tree` parsed `7/7` direct owners; scoped `rustfmt --check` and `git diff --check` passed. The single combined `bun nx run @semio-tech/trinity-plugin:test-quick` attempt exited `1` before Trinity runtime because `semio-s-plugin-stdio` had `116` concurrent compile errors across glTF/TXT/JSON/XML/SVG/PDF; no diagnostic named JACK or Rewrite. `bun nx show project @semio-tech/trinity-plugin --json` confirmed that all registered test targets use the same crate and STDIO dependency, so no narrower Nx-routed runtime gate exists. No temporary panic was introduced because compilation never reached the Trinity crate, and no runtime-pass claim is made.

## Energy Model Direct-Leaf Checkpoint

- Converted `s.energy.model` 1/Any `replace-model` from a nested implementation triad to one direct semantic owner with descriptor, payload schema, TypeScript, GraphQL, protobuf, text, binary, diff, inverse, and leaf-owned tests.
- Preserved the public language-neutral report bridge, committed vector, inverse/absorb laws, and existing text/binary wire bridge.
- Scoped 17-rule structural policy: `0`; Ajv + internal descriptor parity: clean; payload Draft-07: clean; nightly AST: clean; rustfmt/diff/debug/stale-route scans: clean.
- Registered energy runtime remained blocked before energy execution by the shared STDIO dependency, now reduced to 31 compile errors from the prior 116. The 29 textual-root consumers and two PDF approved-verb failures were assigned to their exact owners.

## Second Live Conversion Census

- Executed the compose-excluding live inventory while Writer/Imperative/Space and PDF/E were mid-cutover: `157` roots, `2326` records, `220` direct, `1403` legacy, `703` central-only.
- This is a coordination snapshot, not an acceptance gate. The in-progress direct leaves temporarily expose `104` schema-parity and `80` language-parity findings; total structural findings are `2163` until those owners finish their surfaces.
- Relative to the first live checkpoint, direct records increased from `147` to `220`, legacy records fell from `1428` to `1403`, and central-only records fell from `763` to `703`.
- Textual/base closure completed for JSON RFC8259 Any (`5`), XML 1.0 Any (`6`), SVG 1.1 Any (`9`), and TXT UTF-8 Any (`5`): `25` direct owners with completed descriptors/payload schemas and `150` direct required-language surfaces. The ticket-scoped Ajv/catalog/surface/root-codec validator reports `descriptors=25 payloads=25 catalogs=4 surfaces=150 rootCodecs=10 errors=0`; the coordinator gate remains `0/17` for each prefix; mutation-root scans report zero sentinels/snapshot fallbacks and zero nested `🦠️mutation` owners.
- Direct consumer closure restored glTF's generic apply entry point, converted all four Any exhaustive subject adapters to exact direct leaf rosters, and updated only the authorized I-JSON lowering/expectations. The executable stale-constructor scan is `0`. The shared `cargo check -p semio-s-plugin-stdio --lib --no-default-features` exited `0` in `9m 13s` with `394` warnings and no errors; this is the compiled proof for glTF plus the four textual roots.
- Canonical root grammar/protocol assets now match the generic Rust registry framing and visibly enumerate every descriptor identity/tag; SVG EBNF/ANTLR and TXT's committed direct binary fixture were aligned. Final scoped `git diff --check` passed and source `[DEBUG]` count is zero. A focused registered Nx runtime retry was blocked before Nx/Cargo by unrelated invalid `assets-build` generator-contract state; its transcript is preserved and no runtime-pass claim is made from that blocked attempt.

## Small Schema Direct 01

- Writer 1 Any, Imperative 1 Any, and S Space 1 Any each reached exact `4/4` direct Rust owners, completed descriptors, payload schemas, root-supported language/codec surfaces, catalog kinds, and catalog vectors. Their three behavior-heavy mutation roots are now transparent aggregates over sibling schema `⚙️operations` owners; Writer IO and Space Home were untouched.
- Fresh scoped all-17 policy results are Writer `0`, Imperative `0`, and S Space `0`. Ajv and the dependency-free internal validator agreed on all `48` positive/negative cases; pinned nightly rustc parsed `18/18` owners/aggregates/operations files; scoped rustfmt and stale-route/debug scans passed.
- Cargo/runtime validation is explicitly deferred while the coordinator's shared STDIO gate remains active. Exact evidence and the deferred-runtime boundary are recorded in `📓️small-schema-direct-01.md`.
## Demonstrator Playground Direct Leaf Cutover

- Converted the single `✒️change-schema` mutation from a nested payload facade to one authoritative direct folder with Rust behavior, descriptor, payload schema, TypeScript, GraphQL, protobuf, text, and binary leaves.
- Reduced the root to a transparent aggregate plus visible language-neutral/text/binary assemblies; updated glue and editor consumers to the direct payload path.
- Independent static gates: scoped mutation policy 0/17, Ajv descriptor/payload errors 0, nightly AST 8/8, rustfmt clean.
- Evidence: `📓️demonstrator-playground-direct-leaf-cutover.md`, `🧪️demonstrator-playground-ajv.log`, and `🧪️demonstrator-playground-nightly-ast.log`.
## Shared STDIO Compiler Closure

- Serialized gate: `cargo check -p semio-s-plugin-stdio --lib --no-default-features`.
- Result: exit 0 after 9m13s, 394 warnings, zero errors.
- This closes the earlier 116-error baseline and 31-error intermediate checkpoint across glTF dispatch, JSON/XML/SVG/TXT direct consumers, i-json lowering, and PDF direct roots.
- Transcript: `🧪️stdio-shared-check-after-direct-consumer-repair.log`.
- A later focused registered-Nx attempt was blocked before Nx/Cargo by unrelated live taxonomy drift (`assets-build` output roots and a missing tracked assets readme); it does not invalidate the completed shared library check.
## VCS Direct Schema Leaf Cutover

- Converted six VCS schema mutations to direct owners with descriptors, payload/wire schemas, TypeScript, GraphQL, and protobuf leaves.
- Preserved existing IO mutation codec ownership for the later IO batch; only schema payload mounts and references changed.
- Moved cross-mutation protocol-dispatch helpers and store laws to schema operations; the aggregate now exposes only direct wrappers and structural correspondence tests.
- Existence-checked structural query 0/17; Ajv/internal descriptor agreements 12/12; committed wire/payload fixtures 6/6; malformed payload rejection 6/6; nightly AST 20/20; TypeScript roster and rustfmt/diff checks clean.
- Evidence: `📓️vcs-direct-leaf-cutover.md` and `🧪️vcs-direct-*` logs. Runtime compilation remains queued.

## Independent Acceptance Gate Corrections

- PDF 1.7 UA now has 11 direct owners. The coordinator existence-checked the exact `📄️pdf/.../✳️ua/🧬️schema/🧬️mutations` root and observed zero findings across the 17 structural policies; transcript `🧪️pdf-ua-independent-policy.log`.
- The registered Demonstrator quick suite exceeded the existing 1,200,000 ms warm-build limit while compiling the shared STDIO test dependency. The Nx target failed before assertions; no runtime pass is claimed. Transcript `🧪️demonstrator-playground-test-quick.log`. Process sampling showed active rustc code generation, not proof of a shared target lock. Unrelated builds were left untouched.
- A previous manual scoped query used a nonexistent PDF path and returned an empty result. The later existence-checked acceptance pass supersedes that false-negative result. The test-first shared verifier correction now rejects missing, opaque, escaped, and symlinked selected roots before source access. The red test reproduced the false green; the full focused foundation selection then passed 11 tests, 87 expectations, zero failures. Evidence: `📓️mutation-scope-fail-closed.md`.
- Active disjoint execution lanes: PDF 1.7 UA closure and E/H codec mounts, then VT/X; PNG/JPG/BMP/TIFF base roots; Imperative/Space regression repairs plus GIS/Curate/Home closure, then Sequence. Sequence has been researched but not edited by the coordinator; leaf detection ownership is required rather than moving central planner branches to shared operations.

## Dependency-First Small Direct 01

- GIS/Gisterrain 1 Any (2), Sourcing/Curate 1 Any (3), and Space/Home 1 Any (1) now have six authoritative direct Rust owners and 48 direct descriptor/payload/language/codec files. Root components are transparent aggregates; shared bridges/laws live in sibling schema operations. Only required direct-path consumers changed, including nine Curate IO constructions; IO ownership was preserved.
- Existence-checked scoped policy returns GIS 0/17, Curate 0/17, Home 0/17. Ajv/internal schema agreement is 36/36 (12 valid +24 invalid); direct/variant/catalog/vector parity is 2/2/2/2, 3/3/3/3, and 1/1/1/1. Nightly AST parsed 12/12 sources and agreed on six variants plus six MutationKind/SEMANTICS implementations. Nine TypeScript surfaces parsed; scoped rustfmt/diff/stale-route/debug checks passed.
- Prior-batch acceptance repairs are complete: the three Writer/Imperative/Space singular-module assertions no longer falsely match `::mutations`; Space has its explicit four-leaf root payload-schema aggregate; Imperative wire records/conversions are direct text-leaf owned and its codec root has four wrapped wire variants with zero match arms. Imperative's five changed codec sources passed nightly AST parsing. Final scoped policy is Writer 0/17, Imperative 0/17, Space 0/17.
- No Cargo was started while the shared STDIO test codegen gate was active. Runtime remains pending; the inline `[DEBUG]` schema-oracle probe is preserved in `📓️dependency-first-small-direct-01.md` and no temporary debug source was left behind.
- Coordinator independently rechecked 13 exact roots with the fail-closed verifier: PDF Any/A/E/H, Writer, Imperative, Space, Energy, Demonstrator, VCS, GIS/Gisterrain, Sourcing/Curate, and Space/Home each returned zero findings. `🧪️small-schema-independent-policy-recheck.log` supersedes the earlier Imperative/Space regression results.

## Third Live Conversion Census

- Registered Nx inventory completed with 157 roots and 2314 records: 330 direct, 1378 legacy, and 606 central-only. The stronger current policy reported 2070 findings while raster and Sequence leaves were mid-cutover.
- This live checkpoint is not final acceptance. It includes eight incomplete descriptors, eight wire-identity findings, and codec-name vocabulary false positives being corrected through language-neutral regression tests. The stronger root/selected-scope gates also expose 14 reachability findings that older policy runs missed.
- Transcript: `🧪️mutation-inventory-third-checkpoint-rerun.log`; canonical inventory: `📊️taxonomy-inventory/🔣️.json`.
- Independent PDF 1.7 VT and X checks both returned zero findings; their 18 and 14 direct-owner closure packets are recorded separately. PDF 1.4 is reserved for read-only audit while the next shared runtime gate is prepared.
- The coordinator identified unfinished scaffold, derive, and inventory-contract requirements in the initial foundation. They remain mandatory work, recorded in `📓️foundation-acceptance-open-items.md`; no foundation-complete or monorepo-complete claim is made.

## Raster Base Direct Compile-Readiness Checkpoint

- PNG 1.2 Any (15), JPG JFIF-1.01 Any (10), BMP V3 Any (5), and TIFF 6.0 Any (6) now contain 36 direct payload/behavior owners and 360 required direct descriptor, payload, language, codec, and test files. Each leaf uses an explicit payload struct and semantic inverse operations, without an arbitrary-diff Restore carrier. Generic apply/inverse handling lives in each nearest schema operations module; canonical codecs retain exact direct-module registries.
- All four roots independently returned zero findings under the coordinator's 17-class structural gate. Ajv validated 36 descriptors, 36 payload schemas, and 36 authored vectors; 144 internal/Ajv positive/negative checks agreed. Nightly AST parsing passed PNG 67/67, JPG 47/47, BMP 27/27, and TIFF 31/31 (172 total), including the exhaustive adapters and independent oracle modules. Scoped rustfmt and git diff checks passed. Production stale constructors and scoped debug markers are zero.
- Removed the four obsolete snapshot payload trees and then their 48 proven-empty directories. Original payload files remain recoverable from the ticket baseline. The Any test oracles now use explicit codec round-trip entry points rather than sentinel mutation specs. JPG/TIFF subset mutation roots remain untouched.
- The source freeze preceded the final ticket-only grammar checker. That checker exposed nine remaining grammar lines across JPG/BMP/TIFF: each canonical grammar still has its no-mutation and set-snapshot productions and their root alternatives. Runtime Rust codecs do not accept those sentinels, so these are real static grammar parity corrections, queued for the coordinator's source-consumed release; no full raster closure claim is made.
- No Cargo or Nx build was started in the raster lane. The coordinator owns the supported-budget Demonstrator retry and shared STDIO compile. Authored direct semantic/codec tests have not yet executed in this batch. Exact rosters, files, commands, parse transcripts, and remaining grammar findings are in `🧪️raster-base-direct/📓️closure.md`.

## Raster Compiler Repair and glTF Opaque-Diff Audit

- The supported-budget Demonstrator retry reached STDIO compilation and reported six E0624 visibility errors, with no test assertions reached. Source inspection identified the three private helper pairs: PngTextChunkDiff::between/is_empty, JpgQuantTableDiff::between/is_empty, and JpgHuffmanTableDiff::between/is_empty. Their live concurrent repair exposes only the nearest schema scope through `pub(in super::super)` and was preserved by the raster lane.
- After the coordinator released the source freeze, the raster lane removed the nine stale canonical grammar lines from JPG/BMP/TIFF. The final 36-leaf grammar/tag/module-registry/catalog/direct-file checker now reports zero errors; all four roots have zero stale/nested/debug counts. Scoped git diff check remains clean. No agent Cargo build was launched.
- The bounded read-only glTF audit confirms 120/120 public mutation types carry `Restore(GltfDiff)` and all 120 inverses construct it. There are no direct Rust text/binary codec facets; all 120 descriptors declare null opcodes/tags despite the public aggregate codecs. All 120 raw payload schemas omit the Rust phase/value wrapper; Ajv independently accepts the raw change-node-name payload but rejects the actual Rust Apply/Restore payload shapes. The Rust aggregate also uses camelCase discriminators while the schema uses kebab-case.
- glTF retains 130 per-leaf Rust contract/scenario files, but its direct leaf sources mount none; many files still import deleted triad APIs. All 120 descriptors claim detect while no direct leaf has an explicit detection method. Read-only scoped Git retrieval located 116 old inverse sources: 45 return GltfDiff, while 71 provide narrower typed restoration evidence (not automatically correct). The audit packet gives exact paths/types and dependency-ordered inverse-plan remediation, including typed collection/reference restoration. No glTF source was edited. Evidence: `🧪️gltf-opaque-diff-audit/📓️audit.md` and `🔣️audit.json`.

## Sequence Direct-Leaf Cutover

- Sequence 1 Any now has eight direct payload owners and 56 direct Rust/descriptor/payload/wire/TypeScript/GraphQL/protobuf files. Seven leaf-owned detectors feed a generic ordered assembly; duplicate-step is explicitly apply-only. Sixteen single-mutation law tests moved to their leaves; eight catalog scenarios resolve to committed vectors. Existing schema-external IO codecs were preserved with one direct builder-path repair.
- Existence-checked policy is 0/17 (also independently confirmed by coordinator). Ajv/internal parity is 48/48; aggregate wire checks are 18 valid plus eight invalid; catalog/variant/direct/vector parity is 8/8/8/8. Lodash oracle and an isolated rustc execution of unchanged production detector bodies agree on two language-neutral fixtures and 10 ordered mutations. Nightly AST parsed26/26 and agreed on eight variants plus eight MutationKind/SEMANTICS facts; nine TypeScript sources parsed; scoped rustfmt/diff/stale-route/debug checks are clean.
- No Cargo was launched during shared STDIO test codegen. The isolated detector probe does not claim full plugin trait/store/codec runtime. Temporary [DEBUG] output is preserved in the ticket and removed from the probe source. Exact scope, limitations, and commands: `📓️sequence-direct-leaf-cutover.md`, `📓️sequence-direct-verification-commands.md`, and `🧪️sequence-direct-*` evidence.

## Foundation Completeness Read-Only Audit

- Current source confirms incomplete new-mutation surfaces/trait/tests, null requested binary identity, no post-scaffold gate, and absent mutation launch registration. An executed in-memory call additionally reproduced enum attributes attaching to a newly inserted module. Built derive source validates tuple shape and SEMANTICS kind/verb only, with no From/direct-owner/wire descriptor contract; its compiled glue source and component copy are separate authorities.
- Inventory still has empty command/editor/viewer data, unassigned agents, root-local heuristic consumer lists, and metadata-only source digests. An executed plan probe returned zero unresolved items for a direct-shaped record carrying a behavior-ownership violation. The apply path's fresh-policy/source-digest check is absent by code inspection; mutating apply was not run.
- Exact bounded test-first closure packets, write sets, contract-freeze/fan-out boundaries, commands, and evidence: `📓️foundation-completeness-audit.md` and `🧪️foundation-completeness-probe.log`. No production/shared API/root-script/policy/test source changed during this audit; no Cargo or real compose access occurred.

## Semantic Acceptance Reset and Current Write Lanes

- Earlier direct-shaped JSON/XML/SVG/TXT and glTF roots are unaccepted: independent audits prove 145 arbitrary aggregate-diff carriers, whole-enum codec bypasses, mismatched wire/schema shapes and non-executed or surrogate inverse tests. The exact bounded packets are `📓️textual-inverse-carrier-audit.md` and `🧪️gltf-opaque-diff-audit/📓️audit.md`. PDF 1.7 X also has a separately observed whole-enum serde root codec; all PDF 1.7 codec delegation requires follow-up review.
- Root ownership remains shared policy/AST and integration. A test-first transitive aggregate-input guard now detects the carrier defect even through aliases and nested payload types. Live 29-root rerun found 143 carriers while TXT repair was already underway; see `📓️mutation-input-carrier-enforcement.md`.
- `/root/terra_txt_closure` (explicit Terra High) owns only the five related TXT Any direct leaves, their real wire/mirror/consumer closure and production-inverse tests. Other textual roots and glTF remain queued for independently verified repair. `/root/terra_terminality` (explicit Terra High) owns only the foundation plan/apply terminality packet and byte-sensitive live inventory digest, not root policy or scaffolding/derive contracts. `/root/luna_census` continues the already assigned PDF 1.4 Any/A/X nine-leaf domain cutover, with its declared apply-only behavior and executable leaf codec registries.
- The four-slot physical limit is enforced. New write lanes use the requested Terra execution model, and independent completed audit packets are preserved. Shared root-script sections/test regions have disjoint writers; no agent may modify the root's current direct-policy/AST section. The next full Cargo run is serialized after stable source checkpoints; no passing runtime result is inferred from compilation or parser evidence.

## Current Stable Source and Foundation Checkpoint

- PDF1.4 Any/A/X now contains nine direct leaves and independently passes the exact three-root structural check. The lane also recorded schema/oracle/parser evidence, including a real five-file TypeScript syntax failure and its corrected8/8 parser result. Full compiled laws remain pending. Packet: `📓️pdf-1-4-direct-cutover.md`.
- The terminality packet is accepted after independent16-test/212-expectation Nx verification. Root-roster and source-byte fingerprints, stable inventory retry, fresh apply verification and cancellation guards are present; no production apply was run. Evidence: `📓️shared-foundation-integration-review.md`.
- The serialized registered STDIO quick test target is running with `--no-default-features`, supported build/test budgets and frozen fleet Rust inputs. Transcript: `🧪️stdio-direct-mutation-runtime-checkpoint.log`. This compiles the STDIO test library; the earlier successful library-only check did not prove it.
- TXT's five direct payloads and generic leaf-callback registries are structurally repaired, but freeze-time review found remaining line-content/native-carrier representability and strict unknown-field parity defects. Corrections are queued behind the runtime checkpoint; no final TXT semantic acceptance is claimed. Evidence: `📓️txt-utf-8-any-semantic-closure.md`.
- Current explicit Terra High lanes are `/root/terra_txt_closure` (TXT follow-up), `/root/terra_terminality` (now source-aware root codec-ownership enforcement), and `/root/terra_inventory` (enriched source/consumer/assignment inventory). Their root-script/test regions are disjoint. The PDF executor has completed its bounded packet and is idle; the four-active-agent limit remains respected. Scaffold and mandatory descriptor/derive propagation remain open and unassigned for a later bounded wave.

## Post-Review Checkpoint

- The earlier production-root structural counts precede the concurrent canonical filename transition to emoji-only primary names. They are historical evidence, not current global acceptance.
- TXT's source hold was released after the running STDIO compiler had emitted fresh dependency files. The corrected TXT sources independently compile and pass28 actual-source runtime tests in the retained ticket harness, including432 native-carrier cases and the one-line removal regression. Full STDIO acceptance is still pending; the currently running invocation uses the older source boundary.
- FND-CODEC-OWNERSHIP-02 is accepted through the16-case independent Rust type-solver/parser/policy replay and a registered Nx test with65 expectations. Production root codecs in other owners remain to be repaired.
- FND-INVENTORY-02 is not accepted. Independent decoy/mount/provenance regressions failed and the executor is replacing name guessing with source module resolution. It exclusively owns MutationTaxonomyWorkflow, its consumer regression block, neutral consumer fixtures, and a new narrow resolver inspector if required.
- `/root/luna_descriptor_contract` is read-only, auditing the full mandatory descriptor/derive fan-out. The coordinator owns the eventual contract freeze.
- `/root/terra_terminality` now owns FND-SCAFFOLD-TRANSACTION-03: only MutationScaffolding/new-mutation routing, scaffold regression tests and dedicated neutral fixtures. Its purpose is safe source preparation/publication and attributed-enum insertion, not full semantic scaffold closure. It must not change the schema/traits/derive or invent mutation behavior.
- `/root/terra_txt_closure` has finished its bounded TXT source changes and will not start another owner without assignment. Root alone continues Cargo integration. The four-active-agent cap and disjoint shared-file subregions remain in force.

## Corrected Foundation and Metadata Type Wave

- FND-SCAFFOLD-TRANSACTION-03 passed independent4-case adversarial replay and the registered2-test/56-assertion gate after review corrections. Full descriptor-first templates, executable generated laws, schema/language closure and post-scaffold structural verification are still not complete. Source and evidence: `📓️foundation-second-independent-review.md`.
- `/root/terra_terminality` has released scaffold ownership and now owns FND-METADATA-TYPES-03: exact full descriptor types/enums, validation, explicit SPR facade reexports and neutral/compiler-backed tests only. No mandatory trait/derive/registry or production leaf cutover is included in this stage.
- `/root/terra_inventory` has released MutationTaxonomyWorkflow and now owns FND-REACHABILITY-03: exact source-resolved direct public mounts and wrapped-leaf/folder bijection in the root policy, the narrow discovery facts needed by it, and dedicated neutral/compiler-backed regression tests. It may not edit scaffold, core metadata, derive or production owners.
- The bounded inventory graph corrections have executor compiler-backed evidence; root's final replay is pending after unrelated transient taxonomy generator outputs became missing. The root retained the loader failures and did not modify or suppress the other work. Global unsupported imports and non-mutation unresolved-evidence classification remain follow-up work, not a completed census.
- `/root/luna_descriptor_contract` remains read-only, now auditing actual Rust mutation implementations and derives outside mutation facets. Root recorded its source-provenance recommendation and independently proved `Span::local_file()` under normal/remapped compilation; mandatory derive ownership is not yet implemented.
- The old STDIO build expired at its60-minute budget with no tests reached. A fresh registered library-only, single-build-job gate is running, with all nextest metadata retained under the ticket. It must not be confused with the old gate or the independently passing TXT-only runtime harness.

## Const Contract and Runtime Failure Checkpoint

- FND-METADATA-TYPES-03/CONST-04 are independently accepted as bounded metadata foundations: exact14-field type, schema-equivalent const validation,20 neutral roster vectors,5 unchanged actual-source tests, expected compile-time rejection diagnostics, and the registered real-kernel5-test selection all pass. Roster scope is the common mutation-root; every descriptor retains its distinct full leaf owner. No defaults or optional metadata registration were added.
- FND-REACHABILITY-03 and the coordinator's final integration vectors pass the registered1-test/20-assertion gate. Three canonical/semantic-alias/child-facet positive sources compile under Rust2021. Restricted aliases, duplicate childmounts and escaped childpaths now have explicit negative fixtures. Per-vector compiler source/log retention is corrected. Other structural policy gaps remain open.
- The fresh STDIO library gate reached runtime and failed6 of164 executed tests before fail-fast. A bounded reuse of the same executable without fail-fast exposed70 assertion failures and11 per-test timeouts before its180-second total budget ended at or after completed index2324/5996. The binary hash remained unchanged. No complete-suite pass is claimed; detailed failed-owner queues are in `📓️stdio-runtime-failure-triage.md`.
- The outside-facet audit includes80 runtime-state mutation implementations and additional workflow/space/store/DAG/plugin/generic candidates. Presence/configuration/session state and synthetic public codecs are not exemptions. The lexical census remains qualified, not exhaustive. Evidence: `📓️outside-facet-mutation-audit.md`.
- Current writer `/root/terra_inventory` owns FND-DERIVE-ATTRS-05 only: strict mutation/composite attribute parsing in both existing derive mirrors and fixture-driven actual-parser tests, with test-only dependency updates. It has released reachability/discovery policy writes. Cached-artifact standalone attempts failed; permanent real-crate tests are being completed for serialized root execution.
- Current writer `/root/terra_terminality` owns FND-TAXONOMY-AUTHORITY-06 only: root `📋️project.json` `metadata.semio.taxonomy`, its schema/fixtures and the narrow TypeScript loader path authority. The frozen locator eliminates the current hardcoded taxonomy-location fallback before metadata source proof is implemented.
- Read-only `/root/luna_descriptor_contract` is diagnosing all nine PDF1.4 direct law failures. Root retains coordination, architecture freeze and independent integration. Core descriptor sources are released and stable; no Cargo gate is currently active after the successful kernel selection.
- Canonical filename cutover, mandatory trait/derive/registry propagation, every remaining owner/surface conversion, fixture/behavior repairs, exhaustive census and all final gates remain required. The goal and repository ticket remain active.

## Source Authority and Registry Follow-Up

- FND-DERIVE-ATTRS-05 independently passed its real registered crate gate: two compiled tests, ten neutral rows, zero skipped. The first shell-filter forwarding failure is retained separately. Both mirrors contain the checks, but only the glue-source crate is compiled. Its old cached-dependency diagnostic is corrected in the evidence report.
- `/root/terra_inventory` now owns FND-SOURCE-AUTHORITY-07: private compile-time source/workspace/taxonomy/owner proof in both derive mirrors, dedicated neutral fixtures/tests and a private compile-time JSON boundary. Public trait, aggregate derive and registry changes remain outside this packet.
- `/root/terra_terminality` retains FND-TAXONOMY-AUTHORITY-06 for root-review corrections: exact marker-pair validation, no-follow root ancestry and positive child-start discovery. Its initial focused21-assertion green is not accepted as proof of those additional cases.
- `/root/luna_descriptor_contract` completed the mutation-versus-inference registry fan-out audit. Inference has a separate registry; full mutation metadata can become mandatory without synthetic inference metadata. The read-only lane now checks actual payload/source proof ownership, including optional child facets.
- Source-path preflight now has four passing compiler cases. Raw compiler source paths can legitimately be relative and contain parent segments; strict manifest locator rules cannot be reused blindly. The ADR records this boundary and retained runtime output.
- No Cargo gate is active at this checkpoint. The coordinator continues independent integration and records current failures rather than treating structural-only historical passes as semantic acceptance.

## Locator Acceptance and Active Repair Lanes

- FND-TAXONOMY-AUTHORITY-06 independently passed3 registered tests/35 assertions; root/start ancestry, raw path rejection, exact markers and malformed nested anchors are covered. Retained root evidence: `🧪️taxonomy-authority-root.log` and `🧪️taxonomy-authority-root-artifacts`. Source regions are released.
- `/root/terra_terminality` now owns PDF14-LAWS-02: only the nine direct PDF1.4 law-test sections and dedicated validation evidence, without production geometry/behavior/fixture changes or filename renames. The aim is to execute the inverse/codec assertions hidden behind the first JSON representation failure.
- `/root/terra_inventory` retains FND-SOURCE-AUTHORITY-07 after independent compile/review rejection. The real crate failed E0382 before tests; source path checking, fixture authority, environment-neutral artifact placement and nested marker behavior require correction. No public metadata/registry cutover is present yet.
- The coordinator's refined9-case compiler/runtime test-presence preflight found7 policy false negatives. Empty directories/modules, unmounted files, disabled/ignored laws and nested-function decoys cannot serve as acceptance evidence. The bounded policy repair remains queued.
- Read-only Luna's legacy Note/FEM/DAG examples do not justify restoring the removed mutation triad facet. The final metadata source authority continues to use the manifest locator and actual descriptor JSON. Current public trait/registry consumers and source-proof scope are being frozen without compatibility defaults.

## Source Proof Acceptance and Executable-Test Repair

- FND-SOURCE-AUTHORITY-07 independently passed8 actual-source filesystem cases and the real derive crate's3 tests, including18 source-authority rows. The final file-parent regression rejects invalid traversal before normalization; source bytes remained unchanged. This is private source/owner proof only; the full JSON contract, public derive and mandatory trait/registry remain open.
- PDF14-LAWS-02 released nine test-only assertion repairs. The registered STDIO library gate is running with no default features and one build job (`🧪️pdf14-laws-registered.log`). The fixture/source checkpoint is not substituted for runtime acceptance.
- `/root/terra_terminality` now owns FND-TEST-PRESENCE-08 in the TypeScript policy/inspector and neutral tests only. The seven independently proven false negatives remain unaccepted until compiler-backed and registered replay.
- `/root/luna_descriptor_contract` is auditing direct Mutation implementations and generic/aggregate metadata enforcement before the coordinator freezes the mandatory trait transaction. The one-leaf glTF local inverse design is recorded but not implemented or accepted.

## Retained State After Tool-Context Reset

- The first PDF registered invocation lost its process/session without a final test summary or exit record. Its442-byte startup transcript remains `🧪️pdf14-laws-registered.log`; no pass or build-completion claim is made. Root started the identical registered selection again with a separate artifact directory.
- FND-LEAF-JSON-08 prepared25 schema-first vectors and an Ajv oracle while derive sources were held for compiler consumption. Integer spelling and duplicate raw-key handling are explicit boundaries. Its private Rust implementation is still pending verification.
- The live-agent context reset removed the prior test-presence and read-only audit lanes. Their work was reassigned, without overlapping live writers, to `/root/terra_test_presence` and `/root/luna_trait_contract`. `/root/terra_inventory` retains the JSON parser packet.
- The coordinator identified the lower replication ownership of the base Mutation trait and recorded the dependency constraint in `📓️mandatory-leaf-descriptor-contract.md`. Full mandatory propagation must include that contract without a reverse OS dependency.
- The glTF single-node-name inverse follow-up is retained in `📓️gltf-node-name-local-inverse-plan.md`; it is a design packet, not runtime or conversion acceptance.

The second PDF invocation reached actual STDIO test-crate compilation after cached derive/kernel dependencies. The JSON-parser lane's derive-source hold was then released. Separately, the coordinator's five-case trait prototype passed expected compiler/runtime outcomes; exact limitations and the required source-policy boundary are recorded in `📓️mandatory-metadata-trait-preflight.md`.

The parsed test-presence implementation passed the original nine independent cases but failed all seven new compiler-backed review vectors (five false positives, two false negatives). The agent retains its disjoint source ownership for the corrections. Evidence and exact cfg/ignore/module-path failures: `📓️test-presence-policy-preflight.md`.
