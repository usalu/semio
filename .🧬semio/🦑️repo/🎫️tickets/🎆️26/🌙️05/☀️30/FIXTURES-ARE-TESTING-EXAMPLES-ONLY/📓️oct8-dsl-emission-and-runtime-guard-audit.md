# DSL Emission and Runtime Guard Audit — 2026-10-08

Read-only source review; no compiler/runtime tests run by this auditor.

## Native DSL Schema Emission

Actual generic DSL `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🦀️.rs:257` exports shape_json_schema; line 329 exports record_spec_json_schema. Existing actual Rust test route `🧪️tests/🔬️json-schema/🦀️.rs:9–91` calls them for primitive/list/tuple/record/flattened shape results. Proc macro derive tests emit Rust TokenStream (`✨️derive/🧪️tests/🦀️.rs:10–13,40`) and parse it with syn; they do not currently emit a JSON Schema contract.

Important semantic bound: Shape::Statements emitter lines 289–299 returns an array whose items contain oneOf variant record schemas, with x-semio-keyword annotation. It does not produce the synthetic fixture's `{statement:{kind:"rect",width:number}|{kind:"text",text:string}}` representation, does not impose one-item cardinality, and record_spec_json_schema has no additionalProperties:false. Merely compiling emitted schema will therefore not establish the existing native required-inline exact-zero-or-two rejection law. Preserve actual native InlineStatement encode/decode/measure/cancellation/cardinality tests separately.

A valid emitted-schema oracle could serialize the actual `record_spec_json_schema(&InlineStatement::__dsl_spec())` result and compare its properties/required/statement items against an independent JSON Schema engine using the emitter's actual representation. This tests emitter behavior; it is not equivalent to existing tagged fixture shape and must not pretend otherwise. A real payload schema reuse avoids promoting toy rect/text types into a domain owner. The root DslValue contract (`🌱️value/🧬️schema/🔣️.json`) genuinely owns recursive JSON values but allows all such toy object shapes, so cannot substitute for exact one-native-variant rejection. Real ValueRefusal result/wire payloads and PendingPatchAcknowledgement leaves have current domain owners but likewise require example changes and test a different domain law.

## Structural Gate Versus Runtime Source Graph

Actual root `📜️script.ts:15119–15125` SchemaScript.check loads inventory diagnostics/placement and optional --fixture-boundary filters to schema-fixture-defines-schema. Discovery root/definitions corpus recognition (`repo/library/discovery/🟦️.ts:3272,3346,3356,3359`) and collection contract facet detection line3518 inspect JSON contract structure and owner path. These are real schema ownership enforcement, not a compiled runtime import graph gate.

`framework/📦️packages/🦀️rust/📜️script.ts:58–61` test-fixture-ownership-source runs `framework/🧪️tests/🧱️fixture-ownership/🟦️.ts`. Current test contains one independent d3 hierarchy oracle for shared pixel protection fixtures. It does not traverse runtime import edges, Cargo feature closure, include_str!/include_bytes!, resources, Vite mount paths, dynamic imports, or generated published bundles. Its name cannot substantiate those exclusions.

The four guest cargo checks establish compilation for their selected manifests/targets/features once actual successful receipts arrive; compiling a runtime that accidentally embeds fixture content is not itself a rejection condition. Runtime fixture exclusion additionally needs a resolved source/resource/feature graph or equivalent artifact inspection. Root JSON fixture-boundary PASS alone cannot prove no runtime fixture imports/resources/features. Prior compiled artifact/mount provenance report remains applicable: source declarations reviewed, rebuilt artifacts and actual resolved features pending.

## Fresh Inline Constant Admission Review

Reparsed current source in prior all-format compile candidate set, excluding current Store/plugin ownership. Raw candidates retained in `🗑️generated/oct8-boundary-auditor/fresh-inline-compile-candidates.txt`. This pass is bounded to that prior source set and direct compile-object syntax, not a claim of a new exhaustive zero census.

Reviewed inline constants do not all represent fixture admissions:

- S fixture-sweep ownership test lines 135–146 calculates observed source hashes, mounts, dependency features/edges and compares actual observations with expected; Ajv const validates observed, not the original fixture. Preserve this source/dependency oracle.
- Dev local-hub contract lines 15,35–53 builds expected process output from example rows then validates real spawned/missing/cancelled child outcomes. Preserve actual child outcome oracle and genuine provider leaf schema validations.
- Print pipeline lines 171 and313 validate generated API/source captions against actual generator/independent MarkdownIt outputs; source roster line976 validates actual bundled source graph input list. These are actual output oracles, not whole fixture shape admissions. Preserve.
- Print native-chart grammar inline constants validate actual schema defaults, real parsed option arrays, and independent allowed domain modes; preserve domain output agreement. Shooting mutation/diff, Drawing raster production output, and Block Cargo role predicates similarly validate genuine payloads/domain declarations, not test collection envelopes.

## Existing Actual Source Graph Enforcement Example

Print `🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts:959–978` implements a genuine resolved graph check: esbuild bundles each configured production entry with metafile:true, then compares exact source input roster and rejects forbidden inputs. Its platform=node, packages=external and external bun configuration bounds coverage to resolved local bundled files. No execution receipt for this route was obtained here; it does not cover Rust Cargo target/feature graphs, other JS entry points, dynamic resource reads, filesystem mounts, or produced plugin components. It demonstrates an existing repo-owned pattern stronger than the root schema structural gate, without implying universal enforcement.

## Current Literal URL Reader Check

Fresh current-source reparse of literal new URL(relative.json,import.meta.url) inputs in prior compile-corpus candidate set now finds only Trinity window-config-ownership test line10's intentional absent retired sibling check. Previously missing Shell schema reader is no longer present in this bounded scan. This excludes computed joins/catalog paths and does not imply all executable readers are closed. Raw current-missing-static-url-json-inputs.json retained under auditor generated folder.
