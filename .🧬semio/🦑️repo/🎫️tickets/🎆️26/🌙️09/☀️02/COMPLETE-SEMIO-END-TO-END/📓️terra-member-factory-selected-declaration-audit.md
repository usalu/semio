# Member Factory Selected-Declaration Audit

## Scope

Read-only pre-implementation audit of the proposed retained-history selected-factory seam. This is not a MemberFactory runtime result and makes no typed-decoder, space, pin, or public transport claim.

## Current Baseline

`MemberFactory` exposes only `create` and `open` [store](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17965). The generated `space_members!` implementation selects an arm using just `(artifact_kind, standard, subset)` and passes the fourth `schema` literal later into `open_member_store` [macro](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18288). `open_member_store` does correctly verify persisted `doc_id`, schema, full dialect, and optional full owner before `P` decoding [store open](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:3139), but its raw slice interface cannot preserve the original retained request on a failed selected-arm admission.

The real Flow child uses `SemioMembers::Flow`, whose exact static coordinate is `s.stdio.semio@v1/flow` with schema `stdio.semio`; `stdio.semio.flow` is a synthetic fixture identity and must not enter the selected table [Semio table](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs:1121).

## Required Seam

Add an associated, static four-field declaration table to `MemberFactory`, generated from the same `space_members!` literals that generate `create` and `open`:

`(artifact_kind, standard, subset, schema)`.

The retained selector must:

- validate the entire bounded table first: nonempty/legal dialect components and schema, no duplicate three-field coordinate, and a table-size bound;
- select only by exact requested three-field dialect, then use the selected static schema for persisted-history semantic identity validation; no caller-supplied schema/table may reach the selector;
- validate history `document_id`, selected schema, full dialect, and exact `OwnerRef` before any typed `P` decode;
- return the exact retained input on invalid table, no matching declaration, semantic mismatch, cancellation, deadline, or stale context. It must not call `take_ready`/transfer its private one-use witness on these paths;
- consume the one-use verified input only after a selected declaration and final context/authority check. The selected static declaration and witness must remain paired through handoff, rather than returning a detached schema string or span.

The static table resolves selection; it is not authorization. Space membership/pin checks and the parent app's `open_child` graph transaction remain independently required.

## Compile Completeness

The trait change must update all existing implementations, not only the macro expansion: `NoMembers` [store](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18133) and the fixture-only direct `ArtifactStore` implementations near [store tests](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:21635). `NoMembers` should declare the empty table and fail closed. The table must remain emitted by the macro itself: duplicating a hand-maintained Semio table would recreate the drift it is meant to prevent.

## Required Neutral Cases

1. Every declared `SemioMembers` coordinate selects its own static schema, including Flow's `stdio.semio`.
2. Same three dialect fields with a persisted different schema is denied before typed decode.
3. Each of the three dialect fields changed independently is denied with the exact input intact.
4. Duplicate/invalid static declaration is rejected before selecting any arm.
5. Correct static coordinate but foreign document, parent, slot, or child id is denied with the exact input intact.
6. Cancellation both before and after generic SPR readiness retains the input; a successful selected handoff is one-use and a second take fails closed.

The native selected-factory law must assert actual typed decoder invocation count is zero for every denial. A schema-only source oracle cannot establish retained-input ownership or one-use transfer.

## Verdict

The provider's proposed static four-field, macro-generated table is the smallest sound replacement for the current three-field dispatch. No current source has landed for this packet at this audit point; runtime acceptance is pending a concrete retained selector, neutral corpus, and native law.

## Current Staged Source — 2026-09-04

The proposed selector is now staged, but remains deliberately unmounted while `MemberFactory` and `space_members!` do not yet define `OPEN_DECLARATIONS`. That is an honest source-only boundary, not a compilable factory claim.

The staged selection machine validates each complete declaration, checks duplicate three-field coordinates over the whole bounded table, and only exposes a selected declaration after the full scan [factory selector](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🦀️.rs:47). Every check, pre/post-fuel handoff, and dictionary transition retains the same `VerifiedMemberHistoryInput`; denial is sticky and only bounded retirement releases it [selector handoff](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🦀️.rs:107). The static selected schema alone reaches `MemberHistoryDictionaryOwner::begin`, so semantic schema comparison still precedes a future typed decoder [semantic handoff](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🦀️.rs:137).

The neutral fixture derives all eighteen rows directly from the current real `SemioMembers` macro literals and correctly records Flow as `s.stdio.semio@v1/flow` plus `stdio.semio` [neutral fixture](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🧫️fixture/🔣️.json:2). Its sixteen table cases cover reverse order, missing/empty/over-capacity, duplicate selected and foreign arms, and late malformed literals; its seven lifecycle traces cover cancellation, operation/generation/clock expiry, one transfer, and exact retirement [fixture lifecycle](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🧫️fixture/🔣️.json:44).

No selection defect was found in the staged state machine. The remaining required implementation is mechanical but material: add the associated static declaration slice to the trait, emit it from every macro arm, supply the empty `NoMembers` slice and fixture direct implementations, mount the module, and bind real native laws. The staged source oracle is not evidence of those facts until its terminal and the mounted native binary execute.
