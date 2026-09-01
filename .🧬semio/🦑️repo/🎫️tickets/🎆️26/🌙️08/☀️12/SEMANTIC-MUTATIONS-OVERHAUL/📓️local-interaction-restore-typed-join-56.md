# Local Interaction Restore Typed Join Audit

## Scope and evidence

Read-only audit for runtime task `01a0236e`. No source, schema, renderer, Store, command, or native-test file was changed; no compiler or runtime command was executed.

The typed protocol is already real and strict:

- [`LocalInteractionRestore`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🦀️component.rs:63) has `Full { base, state }` and `Domains { base, domains }`. A domain patch requires all three nullable fields; `null` removes only that map entry. [`apply_cold`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🦀️component.rs:103) rejects a non-identical full authority as `stale-authority`, but explicitly documents that it is not live authority.
- The canonical JSON schema has the same closed `full`/`domains` union and required nullable patch fields at [`$defs.restore`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧬️schema/🔣️local-interaction.schema.json:143). It preserves `DomainSelection.anchorId` where supplied.
- [`LocalInteractionRoot::begin_domain_patch`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🌳️root/🩹️update/🦀️component.rs:107) already owns bounded, cancelable, three-map retained updates. It is not referenced by the Plugin live owner outside protocol tests.
- The capture producer is present: [`LocalInteractionCaptureCursor`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📖️capture/🦀️component.rs:123) serializes an exact `InteractionState` plus full identity; Plugin query start captures the interaction Store snapshot at [`begin_local_interaction_query`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:23912). Therefore “capture producer absent” is false.

## Exact missing live joins

1. The Plugin local-interaction root mounts authority, live query, capture, query, retirement, and only `set-state`; it does not mount a restore owner or leaf ([`component.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🦀️component.rs:3)).
2. `InteractionConfigMutation` is a single `SetState` replacement variant ([Plugin main](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:9742)); its direct leaf deliberately has only cold replacement semantics ([leaf](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🧬️mutations/🔁️set-state/🦀️.rs:15)). A restore must not be expressed as this unguarded replacement, because it needs identity/topology validation and retained candidate close.
3. The inspected local-interaction fields retain a query owner ([main](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:18575)), but no inspected live restore route joins `LocalInteractionRestore` to `begin_domain_patch`. This is a missing concrete command/reducer/publication join, not proof that another operation slot, token family, or cancellation protocol is needed. Existing typed-operation ownership must be audited and reused for the exact restore input, candidate, and close path.
4. The special local-interaction AppChannel variant is currently query-only: tag `29`, a 142-byte payload, and `Read`/`Acknowledge`/`Cancel` decoding ([channel](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs:1252)). This does not imply a new restore frame. The same channel already has ordinary `AppCommand::Command { seq, command, view_state }` ([declaration](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs:1455)); the mounted dispatch decodes manifest actions/commands or calls `handle_command_frame`, and returns existing invocation or fault frames ([dispatch](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:30874)). The missing evidence is an exact restore declaration, strict decoder, concrete retained factory, and host caller through that ordinary admission path. `CommandText` remains explicitly unsupported, so it is not an alternate route.
5. Current ordinary interaction action vocabulary is exactly six verbs: select, hover, clear selection, select all, set mode, set granularity ([main](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:18636)). `interactionSelect`/`interactionHover` accept JSON text for `targets`, not comma splitting ([parser](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:401)). No restore action/ordinary command exists. A comma in an ID remains literal protocol data; it must never be split by a restore bridge.
6. The query capture/publication is source-mounted ([query start](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:23912), [publication](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:23949)); this audit has not executed it and makes no completeness claim. Tutorial restore still needs the ordinary-command caller and typed live acceptance join; query publication alone does not confer mutation authority.
7. Tutorial authored cold composition exists ([tutorial module](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🎬️tutorial/🏠️local-interaction/🟦️component.ts:16)) but is not mounted as a live producer. Meanwhile the manifest model uses typed `interactionSelection` and `selection { domainId, granularity, ids }` ([manifest](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts:675)), whereas ShellHelpers still captures/applies opaque `selectionJson` ([capture](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx:2083), [full apply](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx:2117), [delta](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx:2195)). The renderer must eventually consume/send the typed restore route, rather than translate it through opaque selection JSON.

## Required neutral acceptance matrix for the runtime packet

| Case | Required result |
| --- | --- |
| Full restore, exact base | Replaces all three maps only after exact app/generation/interaction/document/topology identity validation. |
| Domains patch with all fields `null` | Removes that domain from all three maps and leaves every unrelated domain unchanged. |
| Declared non-broadcast domain | Restore remains local/persisted interaction state; outbound presence remains filtered by existing `SelectionSpec.broadcast`, not silently dropped from restore. |
| Selection anchor | `anchorId`, granularity, and ordered IDs survive typed decode/restore unchanged. |
| Comma ID | An ID such as `"node,a"` is one exact ID through schema and restore; no command/UI layer splits it. |
| Stale base or topology revision | Reject before Store write; retained candidate is closed, and state/generation remain unchanged. |
| Cancellation at every retained phase | No partially installed root; original Store/root remains authoritative and all captured input/read owners reach terminal close. |
| Unknown/unregistered domain or invalid granularity/ID | Schema may accept syntactically valid data, but live topology/registry validation must reject before commit; this is distinct from cold composition. |

## Suggested bounded integration footprint

Define the exact restore semantic command from the existing `LocalInteractionRestore` schema and mount its concrete retained decoder/reducer/publication authority through ordinary command admission. The current [`dispatch_typed_command_inner`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:22285) checks live instance, admitted command identity, complete app-owned proof, and capacity. [`start_typed_command_operation`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:22322) captures exact Store roots/revisions and obtains an existing keyed cancellation lease. These are reuse candidates, not proof that the restore-specific input, Interaction freshness, or publication lanes are already admitted. The runtime owner must choose and test that exact join without granting authority from the query, a command name, or a default factory.

Reuse `LocalInteractionRootPatch`/`LocalInteractionRootUpdate` for retained three-map candidates and the existing operation cancellation/result/retirement machinery. Any required mutation payload must be a schema-owned semantic direct leaf with real metadata, not an alias authorizing unvalidated replacement. `SetInteractionState` and `apply_cold` must not stand in for live identity/topology validation or bounded candidate ownership. No new AppChannel tag, token family, parallel ABI, or cold fallback is proposed by this audit. A full-restore retained traversal and atomic all-domain publication still need their own concrete ownership proof; the per-domain cursor alone does not establish them.

The renderer/tutorial follow-up is separately required to replace the stale `selectionJson` bridge with the typed manifest domain data. It is not evidence that protocol capture is missing.

## Read-only endpoint hashes

| Endpoint | SHA-256 |
| --- | --- |
| Plugin main | `2ad816977def25ded3175c87c0f7d03344f1bae57549689b17388adf871736ca` |
| Plugin interaction mount | `6f85aaf9bcf698b48c52744edd57efe3d8692907efbb1aa5ba55b7da0b9542bb` |
| Capture cursor | `e14b556c32a7ec32f424e156b738f0e478f2e913982ee97f4cbe2448e14fb8b1` |
| Set-state leaf | `9459f322c4f679aec523bf1eacb89b953a53448312ccc276d5276be4394a263c` |
| Protocol local-interaction Rust | `7d442ab45f5499b6520ea38ddaff866da4cb9d4118755c8c0af22cbfaea55bc6` |
| Protocol local-interaction schema | `9b200e30396f6637f08b6b3a7d5017eac938a8edc88258ade34e907e5a87348e` |
| Retained root update | `d4f9c7d2259f7962a3206f268fd64c3b5c399aba0ee8238eaf2d251025e6ccdc` |
| AppChannel | `6085646b9878ef5457e25ff1a5fdf5e6883b7fbfc0ec1879679893e3df17f9ae` |
| ShellHelpers bridge | `0962a01fc34439decc09d0322485dc8a403b7cabef6dfe88114505db4806de1f` |
| Tutorial local-interaction composition | `eee4c3f41d2c54bf8415825427b816338dcc1f3771fb3a629e47713c22b43475` |
| Tutorial manifest types | `0c53289933fa4da3891afea84e2eb2eff34af6d68205736a0e563b7daf9fd353` |

No runtime behavior was executed or claimed by this audit.
