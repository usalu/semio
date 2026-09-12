# Retained Document Hydration Extraction

## Implemented Boundary

The common Store-owned boundary is `RetainedPersistedDocumentHydration<P, Mutation>` at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs`. It accepts a semantically validated `HistoryLog`, exact document schema/id/dialect/owner identity, the canonical `DocumentStoreOwners<P, Mutation>` catalog, operation/generation/expiry authority, and either a retained typed initial snapshot or its persisted Pack bytes.

Its output modes are explicit:

- `Store { generation }` moves the hydrated envelope, `ArtifactStoreInitializationRuntime` and complete owner catalog into one boxed `ArtifactStore`. The retained member opener consumes this mode.
- `Envelope` closes the validation/cursor runtime and unused owner catalog to terminal empty, then transfers only the hydrated envelope into the app's existing document initialization job. The retained parent archive loader consumes this mode, so domain-specific initializer behavior remains authoritative.

The public catalog and exact installer were renamed repository-wide to `DocumentStoreOwners` and `install_document_store_owners_exact`. The old member-level names have no alias. The full 87-source ledger is in `document-store-owner-catalog-rename-ledger.md`. Member-specific factories still use `MemberStoreOwner::member_store_owners()` because those factories remain the closed member-loader boundary.

## Retained Phases

The cursor retains separate owners for Pack bytes, typed initial snapshot, validation projection, decoded history, expected document identity, owner stamp, schema, envelope, initialization runtime, owner catalog, pending typed edit, pending durable messages and the currently active retirement.

One step advances one bounded transition: Pack byte scan; Pack decode; identity validation; begin one edit; decode one forward operation; decode one inverse operation; decode one metadata item; publish one edit; hydrate one change, checkpoint, alternative, conflict or composition pin; validation replay of one operation; displaced validation retirement; cursor replay of one operation; seed one applied or redo id; retire history; retire Pack bytes; close envelope-mode runtime and catalog; or terminal handoff.

Every capacity refusal places the rejected owner into the retained close ladder. Cancellation, expiry, stale authority and malformed typed operations preserve their primary diagnostic while bounded cleanup drains pending edit/messages, validation projection, envelope, runtime, history, Pack, identity strings and owner catalog. `Drop` fails closed unless the output transferred or all owners reached terminal empty.

## Pack Scan and Decode Grant

`from_pack` first scans at most `OWNED_SCHEMA_DECODE_PAGE_BYTES` and no more than the supplied fuel on each call. The decode count stays zero throughout that scan and across the phase-change yield. Once the complete admitted extent has been scanned, one later maintenance opportunity calls `P::decode_pack` exactly once and yields before any history phase.

The typed Pack codec itself is one call rather than an incremental codec cursor. Its input is already bounded by archive/member admission and has been charged byte-for-byte before invocation. There is no whole-history parser in that transition: SPR framing, dictionary construction, semantic graph validation and auxiliary owner transfer run in `RetainedHistoryDecode`; typed mutations, metadata, validation replay and cursor replay then run item-by-item in this common cursor. This is the exact proof boundary and limitation for the current design.

## Production Integration

`InitialMemberStoreOpen` preserves the retained snapshot opener, request/history witness, verified byte copy and raw `RetainedHistoryDecode`. After raw decode it transfers the typed initial snapshot, decoded history and verified identity into the common cursor in `Store` mode. A ready boxed Store then enters the existing retained source-witness retirement and closed-member handoff.

`ActiveDocumentArchiveLoad` preserves admission, poll, cancel and terminal acknowledgement. Parent raw history still enters `RetainedHistoryDecode`. `HydrateParent` moves the parent Pack, decoded history, exact root identity and `A::build_document_store_owners` into the common cursor in `Envelope` mode, drives it under the archive scheduler's bounded maintenance grant, and passes the resulting envelope into `begin_persisted_document_store_replacement`. The existing member roster, closure validation, atomic parent/member/content/coordinator/window publication, stale-authority checks and bounded displaced-owner retirement remain the publication boundary.

## Language-Neutral and Native Evidence

The recursive JSON corpus describes complete parent/branch/leaf histories, topology rejection, cancellation and stale authority. The TypeScript oracle validates it with the third-party Ajv schema validator and independently checks exact owner graph reachability and acyclicity with Graphlib. Strict TypeScript also passes. The native fixture persists real edits in all three documents and verifies parent revision 7, branch value 29 and leaf value 53 after publication. Its malformed parent and branch histories contain absent edit references after valid prefixes and must preserve the previous archive byte-for-byte.

- `🗑️generated/recursive-replacement-oracle-10.log`: five owner tuples, eight recursive atomicity vectors and three complete history graphs passed Ajv, Graphlib and strict TypeScript.
- `🗑️generated/recursive-archive-history-native-3.log`: Store/kernel compiled the retained raw decoder and passed 2 selected retained-history laws, 1,084 filtered, Nx exit 0 on the normal configured stack.
- `🗑️generated/recursive-replacement-suite-native-49.log`: 5 selected recursive laws passed with nonempty parent/branch/leaf histories and malformed-after-valid-prefix preservation.
- `🗑️generated/recursive-archive-history-native-51.log`: Store/kernel compiled the common cursor and catalog rename; 2 selected retained-history laws passed, 1,084 filtered, Nx exit 0 in 27.2 seconds.
- `🗑️generated/recursive-replacement-suite-native-52.log`: the full plugin and all 87 catalog consumers compiled; all 5 selected recursive laws passed, 684 filtered, Nx exit 0 on the normal configured stack. This covers both cursor output modes and the production archive/replacement path.

A new focused Store law, `retained_history_decode_persisted_pack_scans_before_one_typed_decode_and_cancels_terminal_empty`, is source-ready. It grants one Pack byte per maintenance opportunity, proves decode remains at zero until the complete admitted extent and an intervening yield, permits exactly one typed decode, cancels before envelope construction, verifies zero-budget preservation, then requires one-item/one-byte cleanup and exactly one typed initial-snapshot retirement before terminal empty. Its registered native run is queued after the current coordinated Cargo batch; no pass is claimed yet.
