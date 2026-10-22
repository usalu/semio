# Dashboard Feedback And Settings

Updated 2026-10-21T19:16:04.549Z. Source implementation and static DATA evidence only; native and Bun runtime remain unrun.

## Source Gaps And Implemented Contract

The previous terminal loop discarded present/copy errors and converted wait failure into an empty event list; entry/cleanup failures were silent. The actual terminal lifecycle now uses the same generic TerminalBackend session and view loop as the new native laws. Enter/present/wait failures exit after exactly one cleanup attempt and preserve both primary and cleanup causes. Before language selection the rendered failure includes both declared languages. Clipboard failure is recoverable. Native tasks remain daemon-owned when the view exits.

Local daemon commands now record typed operation failures with English/German operation labels and a translated disconnected condition. Wire failures retain their existing localized notice and also enter bounded ephemeral local history. The overview and prefix ! open Local diagnostics; wrapped records are inspectable and Enter copies the full bounded record. History retains64 events and at most8192 UTF-8 bytes per detail; it is never persisted or shared.

Settings now opens a local keyboard draft from its prefix row or Edit key bindings. Individual canonical aliases can be edited/added/removed, and one action or all keyboard defaults can be restored. Drafts use the existing Keymap validator and publish only deliberate valid differences. Cancel/unchanged publishes no event. Saves use the existing append_cancellable preference command, expose cancellation, and adopt only an actual successful completion; cancellation requested after a successful durable commit does not claim rollback. Other preference fields and local/workspace scope remain unchanged.

## Schema First And Static Checks

Both new closed schemas/fixtures were authored before production changes. Feedback has eight terminal primary/cleanup rows, including Detach-only and present+Detach+Leave ordered cleanup, bounded-history arithmetic and both locale labels. Editor has seven save/cancel/refusal/unchanged/alias rows. Existing Ajv validated both fixtures and rejected additional properties. Independent Node SQLite queries computed all fifteen case outcomes correctly. This was pure DATA analysis, without product runtime imports. All three changed TypeScript files parse with zero diagnostics. The two neutral Bun leaves are literal imports in the existing dashboard controls suite, reached by the registered test-windows route; Source0 roster/project/package files were not edited.

## Native Laws Pending Actual Compiler And Bodies

- `tests::terminal_backend_failures_preserve_primary_and_cleanup_from_portable_cases`
- `tests::unresolved_terminal_failure_reports_both_declared_languages_without_a_default`
- `tests::clipboard_failure_is_recoverable_and_diagnostics_remain_inspectable_and_copyable`
- `tests::local_control_failures_are_typed_translated_and_retained_under_a_bounded_history`
- `tests::settings_keymap_drafts_match_portable_save_cancel_and_refusal_cases`
- `tests::settings_prefix_is_editable_by_real_keys_and_cancel_never_queues_an_event`
- `tests::keymap_alias_reset_and_durable_save_results_preserve_other_preferences`

The native tests exercise actual TerminalBackend/run_view/terminal_session, real dashboard key routing, typed failed send, diagnostic copy effects, existing keymap validation and preference completion/cancellation projection. The durable writer is the existing production append_cancellable port; these UI laws inject completion messages without host journal writes. Existing preference journal laws remain separate.

Preferred genuine scoped route (not launched):

```text
repo cargo --store=shared --configuration=prototype -- test --manifest-path Cargo.toml --locked -p semio-framework-repo-dashboard-terminal --lib -- --nocapture --test-threads=1
```

The terminal manifest declares Root workspace ownership and already enables native TUI backend. Registered test-windows additionally owns the Bun controls/neutral laws and wider window acceptance; a direct library pass will remain partial evidence.

## Scope, Source Extent And Freeze

Exactly20 authorized paths are saved (12 existing +8 new); maximum absolute path146 UTF-16 code units. No project/package/Source0 roster/framework writes, external runtime dependency, producer, IDE launch or Git mutation. Low final bounded source review accepted the core, neutral cases, and actual-connection Detach guard; compiler/runtime remain unrun.

A bounded literal Rust mount census finds18 files/291374 raw bytes including all cfg(test) mounts, versus saved pre-edit16-file261766 bytes. Production-only physical union is190126 bytes. The terminal manifest declares262144 source bytes/32 files/4000 active lines. This static raw union is a potential admission frontier; actual guard classification is not assumed and no cap was changed. Parent was notified before runtime release.

## Final Concrete Review Correction

Low identified a failed Detach being recorded after the last frame and then discarded. The final source returns typed Detach as the primary failure when the view succeeded, or as ordered cleanup behind an existing present/wait failure. Leave appends its own failure afterward. The actual lifecycle has at most two cleanup causes and never masks the original primary. Neutral and native cases now include this three-cause boundary. Both closed fixtures still validate/reject extra fields, all eight feedback SQL outcomes and seven editor SQL outcomes match, and TypeScript parse diagnostics remain zero.

## Final Guard And Admission Trace

Root approved requesting Detach only while dash.online() owns an actual connection. The recording native fixtures explicitly set online_for_test=true. The same backend law also proves immediate non-online exit performs no Detach and does not invent a failure.

The actual byte consumer is `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts` lines27–28: it takes the minimum global/manifest limits and cumulatively charges physical byteLength before source cfg selection. `📥️source/🟦️.ts` lines23–27 retains full raw bytes even when selected active text is smaller. `🕸️units/🟦️.ts` line15 classifies Cargo mode=test as kind=test; admission line22 asks actual compiler cfg with --test. Therefore cfg(test) is not a byte exemption for the proposed library test. The18 literal active Rust mounts alone total291374 bytes, over262144; this is a source-proven admission frontier (not a run/whole claim). No cap, cfg waiver, skipped law or producer was introduced. Merely moving tests within the same crate cannot lower the union. Parent received the concrete frontier for a coherent semantic owner extraction; no extra path is authorized yet.

| Exact authored path | Bytes | SHA256 |
| --- | ---: | --- |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🦀️.rs` | 8940 | `68d388af561d5893403f5c3979700c55adcf0da23cd7182be555946c74722291` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🌐️language-choice/🦀️.rs` | 6439 | `190850235761ce87c91f32ad658d1824015417e21ecc693d562da6a222514f6b` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🪟️windows/🦀️.rs` | 27387 | `3b898678be41d5aa3e47832c697c667e7c2a4c1545c11c0cb6ea7ec612fd6d8e` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📋️panes/🦀️.rs` | 18146 | `072c93e2cf09460723b67ea2a3a56c163d1f3532a9a46868660ef90c8e771f40` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/⌨️controls/🦀️.rs` | 44618 | `4b33a2677b7dcce61875098ee9f4fdc2b10b70f92c50269fd8ab2d74a393bea3` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📡️sessions/🦀️.rs` | 25991 | `b0a582d3804d633b3ba3b6f27449a8daf9327814289a06f99bc5c51ae24554e5` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📂️sources/🦀️.rs` | 37682 | `75222c8039a8467bc59287d1607ddb47026f20dda62de62157dc97b0be48200b` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🌐️labels/🦀️.rs` | 43215 | `e8a48048bce03305e1c6a0f1f6b779ad052aefa6da8deadf9748ddc1bdebc0cc` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🧪️tests/🔬️unit/🦀️.rs` | 17321 | `ddadf50c79a7f5d087fda083ee43014d7e62f49684812cfcb4f7041efc7518e8` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⚙️preferences/⌨️keymap/🔣️.json` | 8092 | `0db747257887509e59cbc20450a0fcb199e55bfed35644a210a8091425e8a269` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/⌨️controls/🟦️.ts` | 10039 | `901043877c33b6c270195ad689276cc2c6418690b02e1588b1a039a3722dcd80` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/README.md` | 41799 | `67bff23d6b3928acf5abdd14ba65feb0c76606d4ab6e8a8bd15dd69298c8b313` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📣️feedback/🦀️.rs` | 4130 | `054a7a5719357ceca9f6815d8f69c107b2f30065b1e099a20c8fc5c16e96e38c` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/⚙️settings/⌨️keymap/🦀️.rs` | 4865 | `adad2c666dbcf27dc2f4be2398dd22dc205cd77cf7e03d7cb6db60bba7a0f6f6` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📣️feedback/🧬️schema/🔣️.json` | 2852 | `af5da357176d22e79ed708b4f2c38eab09c422e2f07e435280171162e813e1d1` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/⚙️settings/⌨️keymap/🧬️schema/🔣️.json` | 1710 | `152391416ce6a680206f8bbf0df2da4b51aa68b50a3983a64584182dc7578900` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📣️feedback/🧪️tests/🧫️fixtures/🔣️.json` | 2193 | `ba774538976df8cd417096a39e8c9ccd7eeea5c32fa2f14c64b6a72fdc62aee9` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/⚙️settings/⌨️keymap/🧪️tests/🧫️fixtures/🔣️.json` | 1281 | `b9b34e4f3aa4b0344d4830ec1d4ab1b792129a8ab799f5f08eeb2d37c70bd1f8` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📣️feedback/🧪️tests/🟦️.ts` | 1611 | `00062f561088ff889235aa701da870900b6d501a657eea4cbdac7c8f90d6bb0f` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/⚙️settings/⌨️keymap/🧪️tests/🟦️.ts` | 1546 | `14214544837216e0784050051f8ec2fa630ff9834e7f781e9d8c01e5afaf26db` |

## Proposed Semantic Extraction — Not Yet Authorized Or Edited

The real source-browser/document production extent is29106 bytes:17160 in its current root,8011 management,3935 history. Its20522 bytes of nine existing UI laws remain in the terminal owner. A production-only source-view extraction by itself leaves262268 bytes before any adapter, so that single move cannot honestly fit262144. The proposal adds three coherent first-party production owners.

- `semio-framework-repo-dashboard-source-view`: source worker scheduling/operation/cancel/pending custody; browser/document/management/history interaction and rendering. It receives an immutable root/keymap/locale/waker context and a `SourceWindowPort` that borrows the actual existing Files/Document model fields and node identities. It never imports terminal. Typed effects carry app window open/focus/close/remount, notices/failures, and engine signals; terminal applies effects after the controller call returns, preventing reentrant aliasing. A terminal adapter borrows controller and windows separately, with no captured mutable Dashboard, unsafe alias or copied window model. The existing9 source UI laws stay at their current terminal module/name and continue to drive actual dashboard interactions.
- `semio-framework-repo-dashboard-feedback`: the sole existing feedback/session RS becomes its crate root. It owns failure values, bounded local history, ordered terminal cleanup and the genuine generic TerminalBackend session. Dependencies: labels, preferences language declaration, locale and TUI terminal. No source-view/editor/terminal-app edge.
- `semio-framework-repo-dashboard-keymap-editor`: the sole existing editor RS becomes its crate root. It owns local draft commands, validation/difference projection and action rows. Dependencies: preferences/keymap and labels. No source-view/feedback/terminal-app edge.

Source-view dependencies are filesystem-workspace, labels, preferences, feedback, locale and the existing first-party TUI core/frame/engine/widgets/terminal. Terminal gains the3 actual normal dependencies. Existing module mounts for feedback/editor are removed and callers use actual crate names directly; no old reexport/alias compatibility layer. Their implementations remain single physical RS files. Every7 new terminal law remains in terminal. No external runtime dependency.

Per-crate admission is proven in the actual consumer: each graph unit gets a new seen/sourceBytes map and starts at its own unit.entry; the Rust closure follows authored mod/include inputs, not extern/use dependency crate sources. Dependencies have separate graph units and budgets. A real owner extraction therefore removes those raw bytes from terminal rather than merely hiding the same physical input behind another module.

Schema-first source-view DTO/effect fixtures and independent SQLite oracle must precede actor edits. Preserve every existing cancellation, generation, committed-after-cancel, worker disconnect and quantum settlement assertion. New owner caps will be chosen from the actual final physical/active closure with Low review; existing terminal cap is unchanged. Genuine owned Cargo lock resolution follows actual declarations, never a handcrafted lock or receipt.

Exact proposed new/changed ownership paths:

### sourceView

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📂️workspace/🖥️view/🦀️.rs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📂️workspace/🖥️view/🎮️management/🦀️.rs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📂️workspace/🖥️view/🕰️history/🦀️.rs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📂️workspace/🖥️view/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📂️workspace/🖥️view/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📂️workspace/🖥️view/🧪️tests/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📂️workspace/🖥️view/🧪️tests/🟦️.ts`

### models

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📣️feedback/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/⚙️settings/⌨️keymap/📦️packages/🦀️rust/Cargo.toml`

### manifests

- `Cargo.toml`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📦️packages/🦀️rust/Cargo.toml`

### genuineLocks

- `Cargo.lock`
- `🧰️framework/Cargo.lock`

### removedProduction

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📂️sources/🎮️management/🦀️.rs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📂️sources/🕰️history/🦀️.rs`

Existing20 paths remain the app integration/test scope; extra exact terminal test leaves will be listed only if the bounded caller census establishes a required edit. No expanded source path has been written. Proposed maximum absolute path147 UTF-16 units.

## Saved Three-Owner Integration And Static Freeze

This saved implementation supersedes the earlier unedited proposal above. All three physical production owners are now wired into the Root workspace; terminal has no old feedback/editor module mount or management/history production mount. No Framework declaration was changed. The two old terminal child Rust files were removed after their sole bodies moved to source-view. The borrowed port owns no Dashboard or copied window model; controller effects are applied after the borrow ends, preserving remount before pending worker start. The testkit is an explicit observed-worker fixture port used by the unchanged nine terminal UI laws; it does not hide or move those laws.

The eight neutral settlement cases were validated with Ajv and independent Node SQLite DATA queries. Production completion/load guards use the same actual predicates checked by the source-view native fixture law. This is static/DATA evidence only. No Bun/native producer or compiler was launched by this slice.

| Owning unit | Raw admitted literal bytes | Files | Raw source lines | Declared byte bound |
|---|---:|---:|---:|---:|
| terminal | 262032 | 14 | 3938 | 262144 (unchanged) |
| source-view | 38111 | 4 | 380 | 65536 |
| feedback | 4114 | 1 | 55 | 16384 |
| keymap-editor | 4849 | 1 | 71 | 16384 |

The census follows every literal path module and include in the exact owning root. All terminal Rust raw bytes are charged, including cfg(test) bodies; no include_str/include_bytes DATA is present there. Its fixture_text calls are genuine runtime reads: the canonical fixture-reader macro expands to text_in(env!(CARGO_MANIFEST_DIR),relative), which performs std::fs::read_to_string. They are not compile-embedded data. Source-view has three Rust files plus its actual3685-byte include_str fixture, included in38111. Raw lines conservatively precede cfg selection. The genuine native admission/compiler remains authoritative; no admission pass is inferred from this static census. Terminal has112 bytes of current headroom; no cap or test was weakened to create it. Maximum authored absolute path147 UTF-16 units.

All original nine source_* laws remain in sources::tests; all seven feedback/settings laws remain in the terminal unit tests. One new owner law is tests::source_settlement_matches_portable_owner_generation_and_cancellation_cases in source-view and must be run explicitly (dependency compilation does not execute it). Required runtime validation is a genuine locked unfiltered --lib selection of terminal and source-view (plus feedback/editor owners as desired), after actual owned Root lock resolution and full tuple audit. Existing Source controls oracle literally imports all three neutral leaves.

Manifest freeze consists of Root Cargo.toml, the existing terminal manifest, and the three new manifests. Terminal gains dashboard_source_view/dashboard_feedback/dashboard_keymap_editor normal edges and source-view testkit dev selection. Source-view uses existing filesystem-workspace, labels, preferences, feedback, locale, TUI core/frame/engine/widgets/terminal. Feedback uses labels/preferences/locale/TUI terminal. Editor uses labels/preferences. No dependency returns to terminal and no external runtime library was introduced. Genuine lock generation is delegated to High only after parent release; no lock has been hand-edited.

Current scoped source/manifest SHA-256 inventory (Root-owned windows/controls included as shared attribution):

| Path | Bytes | SHA-256 |
|---|---:|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🦀️.rs` | 8999 | `9d72652e7f77e27e831942d314205ec1fecd50c45424645c4ecfcbf22a6f8331` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🌐️language-choice/🦀️.rs` | 6460 | `3988c0c95e2d2c0a75bf39ba19c75fe247b94422e9de1fe7708de81447e758ee` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🪟️windows/🦀️.rs` | 27070 | `b23c780d1f67b95a8b82ef247f7dfc30d5bc32c4157daa16fa0c681e6734a255` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📋️panes/🦀️.rs` | 18159 | `97b2715e366ea3a435907ca155219f34cc153051025f53de1e235e6083b76f19` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/⌨️controls/🦀️.rs` | 44638 | `5dda1a9a347f90aaf3d2a41aad3b58f70db9b1efab4b37d013230a9c8ca0038e` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📡️sessions/🦀️.rs` | 25994 | `5daa81423d18a16318f3ead20ad19d2bcc10ecf250ee363309fc5677b6c0106f` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📂️sources/🦀️.rs` | 29436 | `a7245efc3108081a8e73ff0a4e7172f6b0a6d92be9efdc6c894753c01f1c6f2d` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🌐️labels/🦀️.rs` | 43215 | `e8a48048bce03305e1c6a0f1f6b779ad052aefa6da8deadf9748ddc1bdebc0cc` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🧪️tests/🔬️unit/🦀️.rs` | 17367 | `2d63dba2e8b91ecdd2b23b94b7ba22822a92790896406cc02b8fde276f257d3e` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⚙️preferences/⌨️keymap/🔣️.json` | 8092 | `0db747257887509e59cbc20450a0fcb199e55bfed35644a210a8091425e8a269` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/⌨️controls/🟦️.ts` | 10108 | `8c7e052ca621f4379244ab919571b594aa082ea87fa3f0e11bbb2314ae6ad8e6` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/README.md` | 41799 | `67bff23d6b3928acf5abdd14ba65feb0c76606d4ab6e8a8bd15dd69298c8b313` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📂️workspace/🖥️view/🦀️.rs` | 23664 | `d61c57780a45fcbb686af664a6ed44ebef7365c32c8069a443f6c03b75643c2f` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📂️workspace/🖥️view/🎮️management/🦀️.rs` | 8021 | `551bb7b7236ecbc79fa85810f2f777b39cb2e83af42ec3762d6a91c9fa0388ef` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📂️workspace/🖥️view/🕰️history/🦀️.rs` | 3769 | `b786df892a010746f81b9b13445c435b5c3be859f25250a5e74db262b3c2f218` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📂️workspace/🖥️view/📦️packages/🦀️rust/Cargo.toml` | 2015 | `90457e3ac36b6979594c2d89d1c661116f35ca27f821be59fb6de46c04dac3c9` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📂️workspace/🖥️view/🧬️schema/🔣️.json` | 1886 | `3453861d06c823fb2386d25e43b4d04f4289d810e84ce10f5c436d93738b07e0` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📂️workspace/🖥️view/🧪️tests/🧫️fixtures/🔣️.json` | 2657 | `2e2e6572cf7bd1d5c44af949c7fc39a91815fe22a4c4c42f2f4b904502125eee` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📂️workspace/🖥️view/🧪️tests/🟦️.ts` | 1551 | `333d5825c55d1ea766d482e6c65fffec6aa430c87e1d1644b713bcd62db7a2ca` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📣️feedback/🦀️.rs` | 4114 | `0b894d28563cba571672be6401d8c348930c5a7abc10be86649d7c4464d597ec` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📣️feedback/📦️packages/🦀️rust/Cargo.toml` | 1011 | `2841ffdc45d8a94643feedcb2477f9e420915758705ebdfa1688216bc4fb19b0` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📣️feedback/🧬️schema/🔣️.json` | 2852 | `af5da357176d22e79ed708b4f2c38eab09c422e2f07e435280171162e813e1d1` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📣️feedback/🧪️tests/🧫️fixtures/🔣️.json` | 2193 | `ba774538976df8cd417096a39e8c9ccd7eeea5c32fa2f14c64b6a72fdc62aee9` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📣️feedback/🧪️tests/🟦️.ts` | 1611 | `00062f561088ff889235aa701da870900b6d501a657eea4cbdac7c8f90d6bb0f` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/⚙️settings/⌨️keymap/🦀️.rs` | 4849 | `b8ce83dc976ae62e2a6f28f95172c93843bcc186000627413e4c9efd527b25ef` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/⚙️settings/⌨️keymap/📦️packages/🦀️rust/Cargo.toml` | 725 | `c919e78a1241abc67c7852b024ecde88efd19c6df4a0a8a9d02c880a663951d3` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/⚙️settings/⌨️keymap/🧬️schema/🔣️.json` | 1710 | `152391416ce6a680206f8bbf0df2da4b51aa68b50a3983a64584182dc7578900` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/⚙️settings/⌨️keymap/🧪️tests/🧫️fixtures/🔣️.json` | 1281 | `b9b34e4f3aa4b0344d4830ec1d4ab1b792129a8ab799f5f08eeb2d37c70bd1f8` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/⚙️settings/⌨️keymap/🧪️tests/🟦️.ts` | 1546 | `14214544837216e0784050051f8ec2fa630ff9834e7f781e9d8c01e5afaf26db` |
| `Cargo.toml` | 123348 | `e1bed39c09d7c5b49848f6ce95afc3b4df24a669f736edd966fbe2ba7a39b8c3` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📦️packages/🦀️rust/Cargo.toml` | 4269 | `da48ab06ff568524203ab1ac2f188474b1bca140a8b4873c1116be06fe53eceb` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📂️sources/🎮️management/🦀️.rs` | removed | absent |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📂️sources/🕰️history/🦀️.rs` | removed | absent |
