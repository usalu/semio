# Retained OPC Cursor and XLSX Authority Audit — 2026-10-03

## Scope

This is a read-only source audit of the retained OPC copy failure reported in `🗑️generated/retained-opc-native-current.log`, together with a focused XLSX grid and revision-authority check. No production source was changed and no test or compile command was run by this audit.

## P0 — Retained OPC copy has a deterministic zero-progress loop

The native log records that `opc::component::retained::tests::retained_opc_copy_and_materialization_preserve_package_authority` panics at the termination guard in `📦️opc/🧬️retained/🧪️tests/🔬️unit/🦀️.rs:97`; the run is one failure and zero passes at log lines 7920–7939. This is not a ZIP parsing, materialization, or test-timeout failure.

The fixture deliberately uses a small copy grant:

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️opc/🧬️retained/🧫️fixtures/🔣️.json:61-65` supplies five items, 64 copy bytes, 4096 retained-capacity bytes, and depth 64.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️opc/🧬️retained/🧪️tests/🔬️unit/🦀️.rs:85-101` passes that grant to the cursor and limits the loop to 99,999 turns.

The exact state transition that cannot complete is in the generic paged-list retained clone driver:

1. `RetainedOpcPackage.parts` is `PagedList<RetainedOpcPart, _>` at `📦️opc/🧬️retained/🦀️.rs:14-19`; `RetainedOpcPart` owns two retained UTF-8 carriers and one retained byte carrier at `:21-27`.
2. Once that child has been cloned and its scaffold is terminal, `PagedListCursor::advance` refuses the owner hand-off whenever the remaining copy budget is below `size_of::<T>()` (`🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🦀️.rs:181-182`). It returns `Progress(used)` without changing `child_value`, `child`, or `index`.
3. The only placement method also rejects the same grant and charges the full inline owner size as `placed_bytes` (`🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs:377-385`). The driver calls it at `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🦀️.rs:184-191`.

`RetainedOpcPart` exceeds 64 bytes on the native 64-bit target: `PagedList` owns a `Vec` plus three `usize` counters (`📋️list/🦀️.rs:119-125`), `PagedUtf8` adds a `usize` to that carrier (`📦️paged/🦀️.rs:159-163`), and `PagedBytes` wraps a `PagedList` (`:39-42`). The part therefore has two large text headers plus the byte header. The same structural condition can recur in the relationship and metadata lists (`📦️opc/🧬️retained/🦀️.rs:29-34`, `:43-50`) whenever their inline entry exceeds the caller's copy grant.

Because all deep payload cloning has already completed, each subsequent call reaches the unchanged placement branch and reports zero work forever. Raising `maximumCopyBytes` in the fixture would hide this general liveness defect; it would not make arbitrary retained list elements usable under a small positive copy budget.

### Recommended production repair

Add an explicit **retained owner-adoption** operation to `PagedList` and use it only from the retained-clone driver.

It should require a reserved slot and one item of work, then move the completed `T` owner into that slot without consuming `maximum_copy_bytes` and without reporting `copied_bytes`. The copy budget has already paid for the child's deep duplicated data; this final operation merely transfers the completed ownership handle into its pre-reserved destination. Keep `PagedList::place_reserved` unchanged for the UI paths that intentionally meter its physical inline placement; it is also used through `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs:328-329` and document assembly.

Then replace the precondition and `place_reserved` call at `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🦀️.rs:181-190` with the adoption operation and emit `{ copied_items: 1, copied_bytes: 0 }`. This is a framework repair, not an OPC special case. It preserves the retained capacity reservation at `:137-153` and preserves the item budget as the bound for each atomic owner hand-off.

If the retained-clone contract instead intends `maximum_copy_bytes` to cover every physical header move, the cursor cannot promise progress for arbitrary `T` under a smaller positive byte grant. The principled alternative would be a paged storage representation whose insertion hand-off moves a fixed-size handle. Do not silently make the current check conditional on the OPC fixture or increase the fixture budget.

Required regression coverage:

- Add a framework retained `PagedList<T>` law where `size_of::<T>() > maximum_copy_bytes > 0`, capacity has already been admitted, and the cursor completes, can be taken, and closes terminal-empty.
- Keep the native retained-OPC fixture at 64 bytes and require its copy, cancellation close, materialization, and final retirement to terminate. It is the integration regression for the real composite owner.
- Compare the materialized result with the existing conventional `OpcPackage` oracle as the current test does at `📦️opc/🧬️retained/🧪️tests/🔬️unit/🦀️.rs:103-105`.

## P1 — Copy and retirement budgets are conflated in composite retained cursors

`RetainedCloneProgress` distinguishes `copied_bytes` from `retained_capacity_bytes` (`🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs:112-142`), and direct cursor closing receives a distinct `maximum_bytes` parameter (`:175-181`). During `advance`, however, composite cursors use `maximum_copy_bytes` as the scaffold-retirement byte grant and re-label released bytes as `copied_bytes`.

Examples include:

- paged-list child closing at `📋️paged-list/🦀️.rs:162-168`;
- paged byte/text/map wrappers at `📦️paged/🦀️.rs:12-16`, `:53-55`, `:165-167`, and `:284-286`;
- vector, option, and box cursor scaffolds at `🧬️retained-clone/🦀️.rs:482-490`, `:672-679`, and `:798-805`;
- generated struct-field drain code at `✨️derive/🧬️retained-clone/🦀️.rs:171-187`.

This can produce another zero-progress state. A paged list retirement returns `BudgetExhausted` when an empty page costs more than `maximum_bytes` (`📋️paged-list/🦀️.rs:32-40`). `CursorStack` then converts that to `Pending { released_items: 0, released_bytes: 0 }` while the retirement remains live (`♻️retirement/🦀️.rs:271-293`). The paged wrappers map that result to zero retained-clone progress (`📦️paged/🦀️.rs:12-16`), so repeatedly calling `advance` with the same copy grant cannot resolve it. The existing nonconforming-child law asserts refusal for an over-budget retirement at `📋️paged-list/🧪️tests/🔬️unit/🦀️.rs:368-398`, so this accounting boundary is intentional today but does not provide a liveness guarantee for a page whose retirement budget is larger than its copy budget.

The concrete OPC fixture reaches the owner-placement deadlock first. I did not establish that its completed child scaffolds retain a page that triggers this second condition. It remains a framework-level liveness and accounting defect for composite cursors.

### Recommended production repair

Give retirement work an explicit budget and progress channel, or explicitly define and name `maximum_copy_bytes` as a combined copy-and-retirement work budget everywhere. The former is clearer: pass the remaining retirement-byte grant to `close_step`, record released bytes separately from `copied_bytes`, and admit them against that separate grant. Apply it uniformly in the generic drivers and derive macro. A zero-byte `Pending` caused by a positive but insufficient fixed-page release must be exposed as an unsatisfiable work limit or be made splittable; it must never be returned as repeatable successful `Progress`.

Add a law with a retained child whose completed scaffold needs a page release larger than the copy budget but no larger than the retirement budget. It must either finish under the stated retirement grant or return a deterministic refusal, then close terminal-empty under its cancellation/retirement grant.

## DOCX Store ownership remains conventional

The retained OPC types are additive only. `RetainedOpcPackage::try_from_package` and `materialize_package` are defined in `📦️opc/🧬️retained/🦀️.rs:143-183`, but no DOCX Store source references `RetainedOpcPackage`.

The active DOCX snapshot still owns conventional values:

- `DocxSnapshot.opc: OpcPackage` at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:148-159`;
- `DocxSnapshot.xml_parts: Vec<DocxXmlPart>` at `:155-158`;
- `DocxSnapshot::from_parts` accepts and stores those same conventional owners at `:167-174`.

Therefore this retained OPC work has not migrated the production DOCX Store or its XML ownership. This conclusion is static-source evidence only; it does not claim a DOCX runtime verification.

## XLSX sparse grid and vacancy revision authority

The XLSX editor and viewer build sparse `BTreeMap<(u32, u32), &XlsxCell>` projections (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:52-55`; `👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs:39-41`). Grid bounds are derived from observed occupied cells and include one spare row and column where legal (`✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:23-36`). The viewer supplies read-only cells (`👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs:57-67`). These are presentation projections, not mutation authority.

The editor derives authority from the canonical `XlsxSnapshot` XML:

- it obtains a canonical worksheet address before rendering (`✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:52-55`);
- occupied cells receive their canonical cell revision, while vacancies receive the worksheet revision (`:76-84`);
- command emission recreates a canonical cell or vacancy address from the snapshot, validates the supplied revision, and then emits `SetCell` or `InsertCell` (`✏️editor/🦀️.rs:272-291`);
- address resolution recalculates the revision from the XML node and rejects stale cell, worksheet, or vacancy addresses (`🧬️schema/🧬️mutations/🧭️cell-address/🦀️.rs:331-388`).

There is no projection-authority defect in this path. One policy detail should be documented by a law: a vacancy revision is the `sheetData` revision (`🧭️cell-address/🦀️.rs:290-308`), and that revision is calculated over the canonical address scope (`:276-287`). Consequently any relevant XML change inside that worksheet invalidates every outstanding vacancy draft, including a non-structural cell-value change. This is conservative and race-safe; it is broader than an “only structural changes stale vacancies” policy. Keep it if that is intended, or redesign the revision scope deliberately rather than letting the editor projection decide it.

## Evidence limits

The existing native log is evidence of the retained OPC test failure. This audit did not rerun the test, compile Cargo, modify source, or verify runtime behavior outside that recorded result.
