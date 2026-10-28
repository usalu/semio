# Independent Native Template And Host Source Audit

Read-only source checkpoint on 2026-10-21. No production files changed. Repo command-route/compiler limitations are inherited from the parent ticket. No compiler ingress bypass, native execution, heap witness or scheduler proof is claimed.

## Findings

No additional confirmed P1/P2 source defect was established in the inspected selected-template receiver/planner and borrowed publication wire. This is a limited source audit, not native type checking or completion acceptance. The documented `template.semantic-admission-unavailable` refusal is explicit incomplete functionality and is not reported as a newly discovered defect.

## Inspected Custody And Publication

`apply/work.rs:168-184` moves the addressed original application into retained ownership and leaves the same command variant containing Null. The work checks original cancellation before receiving and each preparation step. Receiver source/output/pending/value owners remain explicit across its retained phases; the source envelope is retired before the typed result is transferred.

`apply/plan.rs:198-244` quotes actual contiguous index, mutation, collection and carrier allocations before its planning step. Large source indexing and merge ordering are persistent. Selected entry/identity limits are release-derived, rather than graph-size limits. Its constant-size metadata quotation remains conservative source reasoning, not measured receipts. Whole index allocation can legitimately be denied by a smaller physical capacity loan; repeated fuel alone cannot fund it.

`apply/work.rs:59-160, 302-346` retains planner buffers, mutations, scalar envelopes, original application and snapshot owner for controlled close. Codec close precedes source close. `apply/work.rs:266-280` probes the preinitialized catalog and explicitly refuses unavailable catalog authority.

`apply/🦀️.rs:389-417` restricts generated batches to distinct final append insertions and distinct writes to original fields. This makes captured source preimages sufficient for the selected-template batch shape. It does not implement a general progressive inverse overlay.

Shared `🔎️details/wire.rs:124-157, 316-344` checks source-sensitive prior/current mutation pairs before any codec wire starts. Its borrowed inverse selectors address the captured header/entity/argument/component source; seek progresses per entity. Codec receipts and refusal receipts are debited through the original context (`wire.rs:378-420`), with original cancellation checked between bounded events. The codec source mutation slice is pinned by pointer/length identity; callers still own the requirement to keep its contents stable.

The host wire source intentionally skips a missing original entity or missing original argument inverse (`wire.rs:359-369`); downstream Store progressive validation remains necessary authority for refusing invalid no-op/source mismatches. This inspection did not establish a publication bypass through that behavior.

## Prerequisites For The Semantic Gate

Replacing `apply/work.rs:289-295` requires real original-owned retained semantic receiving and validation of the complete resulting graph, including scalar domains, WHERE/UNIQUE/reference constraints and contextual validity. The result must be complete and valid before handing over the existing owned mutation vector. Keep original cancellation, captured base revision and retained source authority through Store admission. Do not use a cold evaluator wrapped in progress ticks, an unrelated authority grant, a synthetic context or a second graph DTO.

Native compilation and native original host/store execution remain required. Written template goldens, close tests, neutral preimage vectors and third-party fixture oracles do not prove the runtime registry, physical receipt accounting, base-revision rejection, checkpoint behavior or scheduler fairness. Existing host synchronous admission/domain frontiers described in host-wire-progress.md remain open; this audit did not reinterpret them as completed work.

## Small GLB Checkpoint

`🚪️io/native_gltf.rs:148-185` computes the sign of the represented binary64 3x3 determinant using six exact products and separate fixed positive/negative limb accumulators. The finite binary64 exponent/product range fits the 100-limb accumulation; zero/subnormal terms and negative zero do not change the sign. `native_gltf.rs:188-189` mounts the shared-case native orientation test module.

`🚪️io/native_exchange.rs:109-120` checks aligned bounded chunk extents, required JSON/BIN order, duplicate known chunks after BIN and zero BIN padding while allowing unknown extension chunks. Shared framing fixtures are included by native_exchange.rs tests and native_orientation.rs. No additional confirmed GLB defect was established in this small checkpoint. The cold whole-buffer pipeline and unexecuted native tests remain limitations; the earlier executed TypeScript/oracle results belong to gltf-framing-implementation.md.
