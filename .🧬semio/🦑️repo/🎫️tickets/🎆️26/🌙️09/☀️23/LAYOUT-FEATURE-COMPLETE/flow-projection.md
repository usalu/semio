# Canonical Paged Frame Projection

## Result

The schema-first native and TypeScript FlowProjection prepares scalar metadata for every local and per-physical-page inherited text frame. It retains original page/frame/template/override/story addresses, resolves exact story and next ordinals, applies bounds/visible/locked overrides and exposes every independent head. Local edges span physical pages; inherited edges stay within their own physical page instance. Hidden frames retain their thread topology; painting and original layer visibility remain consumer decisions. Original stories, IDs, frames, fonts, styles and document owners are borrowed rather than cloned.

The sibling FlowFrameSource/advance_source seam consumes prepared rows without flattening them. Source-qualified direct ordinal(page,parent,frame) lookup uses paged numeric page ranges and original raw frame slots, including non-text source slots. It does not scan or copy IDs.

## Owned Preparation And Retirement

Five framework NumericIndex owners retain hash buckets, collision entries, frame rows, source locations and page ranges. UTF-8 FNV32 indexing advances one Unicode scalar per admitted turn; exact collision resolution compares one borrowed scalar per turn. No hash collision rejects valid input. The shared fixture includes a real collision between story-erfyto-ww8 and story-1kcwvnf-1jvb. Preparation handles at most one source scalar/header or structurally bounded scalar-index operation, or reserves one original fixed metadata block. No whole graph sorting or arbitrary text/frame array copy occurs.

The TypeScript scratch NumericIndex previously retained one unbounded nodes Array. It now uses fixed16-way numeric metadata pages with separately admitted reserveOne/setReserved, one original block per capacity turn, explicit copy/release close demands and generation reuse. Its fixed capacity receipts are logical scalar custody budgets; they do not claim observable VM allocator header sizes or GC timing. Native retains the original PagedList-backed index. Its arbitrary1048576 stored-key cap was replaced with the natural usize address bound, so valid Layout models are not rejected merely by that unrelated infrastructure limit.

FlowProjection implements native controlled RetireOwned through priced deferred fields over the original five index owners. There is no synchronous whole-index drop in that path. The standard original paged retirement cursor provides exact next_demand, cursor births and physical release quotes. The explicit close path additionally exposes next_copy_bytes/nextCopyBytes and next_release_bytes/nextReleaseBytes; close_one/closeOne requires independent item, copy and release grants. Native copy demand is the original fixed Node width; TypeScript uses its declared fixed scalar-node capacity. Refused body/release grants do not advance. The shared original NumericIndex also exposes its funded close operation.

## Native API

scene::flow::projection::FlowProjection::new(); advance(&LayoutSnapshot,FlowProjectionGrant{maximum_source_items:u32,maximum_capacity_bytes:usize})->Result<FlowProjectionStep,static str>. The ready source_view(&snapshot) lends FlowProjectionView implementing FlowFrameSource. len(), is_head(ordinal), ordinal(page,parent,frame) and address(&snapshot,ordinal) expose scalar rows. FlowFrameAddress contains page_index,frame_index,parent_index,override_index,story_index,next,predecessor,head,visible,locked. The next_advance_capacity_bytes(&snapshot) query quotes the exact next original metadata block without granting or reserving it;0 means the next turn allocates none. Consumers must use their original retained grant and debit last_receipt.

Cancellation clears source loans and pending fixed state. next_copy_bytes(), next_release_bytes(), close_one(maximum_items,maximum_copy_bytes,maximum_release_bytes), terminal_is_empty() drain explicit ownership. The retained source must remain immutable and at the same native address throughout preparation/use. TypeScript exports the corresponding camelCase API and original canonical Binary64 model. Both owned public package facades export the projection and its source types.

## Test-First Evidence

Neutral source fixtures and tests first exposed the missing FlowFrameSource/advanceSource API. Neutral projection grant schema and page-instance/long-Unicode input cases were authored before production. Independent test-only fast-json-patch applies RFC6902 override patches; eager Map topology and lodash counts produce the durable expected frames, addresses and heads. Actual tests also admit every valid canonical document through the production graph validator.

Actual TypeScript projection execution passes 5 portable documents across 3 grants, 4 cancellation cuts, 7 explicit graph refusals and zero-capacity retry. The 5 documents cover page instances/overrides, empty metadata, long Unicode IDs exact hash collisions and interspersed non-text source slots.112846 funded small-case turns compare every allocation receipt to its pre-turn quote. Refusal cases cover missing stories/targets/parents, cross-story edges, forks, cycles and duplicate identity.

The neutral wide parameters generate 40 pages with 80 local frames each plus 2 inherited frames per page: 3280 projected frames and 41 independent heads.229681 preparation turns match all eager reference rows/addresses. All 3,280 rows then feed the actual TextFlowCursor once through their threads. Combined cancellation/retirement laws exercise14825 close steps.

The shared scratch-index law passes 20 portable operations against test-only lodash/Map output and Ajv2020 schema admission, 4,097 retained keys, 277 fixed-block admissions, 4,374 close turns and 597,248 declared scalar capacity bytes returned. Below-copy and below-release grants are refused independently.

Combined public Layout facade, projection, scratch and Drawing test closure: 404 actual TypeScript source files, 0 strict diagnostics with installed node+bun ambient types. Targeted projection closure 224 files and index closure 172 files also have0 diagnostics. Actual Drawing image supplier regression remains10cases/60cuts/49small-events plus40texts/12images/141paths/4377segments/4097-character style.

Five native projection/index production/test files are Rust-grammar clean, maximum path 160. Native shared projection, collision, wide flow, refusal, cancellation, manual body/release and controlled RetireOwned laws are authored and mounted. Native compilation/tests remain unrun because the cached repo launcher is stale/unavailable. Grammar parsing is not a compile or execution claim. End-to-end interactive/export publication remains the integrating consumers' scope.

## Files

New source owner: Layout scene/🧵️flow/📇️projection/{🔣️.json,🦀️.rs,🟦️.ts,🧪️tests/cases.json,🧪️tests/🦀️.rs,🧪️tests/🟦️.ts,🧪️tests/oracle.mjs,🧪️tests/oracle.d.mts,🧪️tests/run.ts}. Flow native root mounts the module. Public Layout TypeScript root and explicit projection leaf mount exports.

Framework files: value/🗂️ordered/🔢️numeric/🧮️scratch/{🦀️.rs,🟦️.ts,📚️pages/🟦️.ts,🧬️schema/🔣️.json,🧪️tests/📜️fixtures/🔣️.json,🧪️tests/🦀️.rs,🧪️tests/🟦️.ts,🧪️tests/oracle.mjs,🧪️tests/oracle.d.mts,🧪️tests/run.ts}.

Retained verification inputs: flow-source-check.mjs,flow-projection-check.mjs,numeric-scratch-check.mjs. Each removes its generated outputs. Native/TypeScript Drawing image borrowing and laws were also updated as documented in drawing-planning.md. No build manifests, dependencies, AGENTS files or git state were changed.

## Precomputed Head Ordinals

FlowFrameAddress.head is written once per row during the existing bounded topology traversal. Consumers can address their story/head cache directly, with no predecessor walk or new allocation pass. The independent oracle and all expected address rows include it. The wide law traverses 3,280 regions from 41 heads using the real source-backed TextFlowCursor.

Final public verification also includes all durable run.ts entries:404 source files,0 strict diagnostics. The flow-source closure now includes the public projection facade and passes at253 files. The ticket verification input exports verifyFlowProjectionPublicTypes for this exact closure. The source ordinal/head API and controlled RetireOwned integration were handed to core; end-to-end renderer publication is not claimed by these source laws.
