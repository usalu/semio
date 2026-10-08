# Runtime Fixture Graph Enforcement Plan — 2026-10-08

## Current Evidence

The root structural schema gate recognizes fixture-owned facets and whole corpus schema definitions, not executable import/resource reachability. Its 68-case/4-test receipt and four guest compilation routes cannot establish runtime fixture exclusion alone. No production source mutation was made in this audit.

A fresh candidate scan across the prior hidden/no-ignore authored roster found 137 possible fixture path expressions across 68 non-test-directory source files. Raw candidate data is retained under `🗑️generated/oct8-boundary-auditor/current-runtime-fixture-edge-candidates.json`. This roster is lexical, includes inline tests, test oracle helper modules, comments and an existing generated published runtime bundle, and must not be described as 137 runtime violations. It also excludes obvious test directories for triage; the permanent graph must instead follow roots and cfg conditions, never infer reachability from path names alone.

Confirmed sampled boundaries:

| Edge | Actual source condition / inbound path | Verdict |
|---|---|---|
| TIFF text mutation fixture `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🚪️io/📝️text/🧬️mutations/🦀️.rs:59` | remove_ifd_test_case is explicitly #[cfg(test)] at57; analogous helpers at74/89/104 need same cfg-owner evaluation. | Preserve explicit test-only edge; not a runtime positive. |
| Infinite DAG actor fixture `OS/infinite/artifacts/dag/vcs/🦀️.rs:634` | opened_dag_test_actor explicitly #[cfg(test)] at632. | Preserve. |
| Canvas mounted input oracle `OS/renderer/engine/elements/Canvas2dHost/oracles/mounted-input/🟦️.tsx:5` | Imports fixture and vitest/ui-react-test; fresh literal inbound search found actual input-contract test import line19 only. | Test helper with currently observed test-only inbound edge; runtime root reachability still needs resolved graph, not directory exclusion. |
| glTF inference 🧪️contract TS modules | Direct fixture imports in helpers outside 🧪️tests directory. | Contract oracle modules; do not report runtime violations unless resolved production root reaches them. |
| Existing OS Dev `📤️distribution/🧶️bundles/🖥️runtime-Dc0wwVhF.js:29717` | Already generated published runtime bundle contains fixture-related text; semantic code/source-map and regeneration comparison required. | Stale bundle candidate, no fresh rebuilt provenance yet. Text occurrence alone does not establish a fixture import. |

No confirmed current production reachable fixture edge is asserted from this lexical scan. No exhaustive zero assertion is justified yet.

## Permanent Guard Ownership and Execution

Implement permanent code in existing domain-owned `📜️script.ts` facade/module infrastructure. Register the callable through existing nx/project/launch routes, using bun and existing test orchestration. Do not add standalone scripts or external runtime dependencies. A ticket-local prototype, if needed, must also be named 📜️script.ts and dispatched by an existing facade. Auditor has executed only read-only shell analysis, not introduced a new script/command registration.

## Rust Source and Resource Proof

1. Enumerate actual production lib/component/binary roots from Cargo target metadata for each supported shipped target/profile. Keep four guest check invocations and component cargo-rustc publication invocations distinct; they have different feature flags/default behavior.
2. Capture exact Cargo selected packages/targets, target triple, profile, rustc cfg values and resolved features for every package. Parse compiler-artifact message `features` and target kind from actual build JSON; retain source hash/manifest/lock identity. Existing artifact collector drops features, so source flags alone are insufficient. Cargo metadata resolve features may reflect a broader selection; bind it to matching command target/package/feature selection and reconcile actual compiler messages.
3. Traverse lib.rs/module declarations, inline modules, #[path], macro-expanded module/resource references and include!/include_str!/include_bytes! after cfg evaluation. cfg(test) and dev-dependencies belong to test graph; preserve them. #[test] functions are omitted from ordinary builds; cfg(any(test,feature="artifact-app-testing")) is runtime reachable if resolved feature is enabled and must be checked. Feature defaults/dependency unification matter; manifest declarations alone cannot show inactive testing feature.
4. Resolve concat!(env!("CARGO_MANIFEST_DIR"),...) and generated include/resource paths with actual build environment. Build scripts can generate code/resources; inspect produced outputs and include paths as graph nodes with owner provenance. Parse actual rustc dep-info as a second compiled-input witness including embedded files; reconcile lexical/expanded graph against it. Unresolved macro/resource expressions must produce a bounded unresolved finding, not silent success.
5. Fail any runtime reachable node or embedded resource under true test/fixture collections. Use taxonomy collection identity/ancestry and canonical physical paths rather than substring "fixture"; genuine fixture-named domain payload reports/types remain valid.

## TypeScript and Browser Proof

1. Enumerate actual app/browser/server/package entry points and export maps, tsconfig/package aliases and Vite plugin virtual entries. Resolve static imports, reexports, literal dynamic import/require, new URL(...,import.meta.url) resources and literal filesystem reads. Bun compiler scan can be reused, but current registryStaticImports filters only import-statement and therefore drops dynamic kinds. Do not treat it as complete.
2. Obtain actual bundler input/metafile/source-map records for built entry targets. Existing Print command guard (`print/...pipeline/🟦️.ts:959–978`) shows repo-owned esbuild graph+exact inputs+forbidden check; packages=external and platform=node limit its closure. Browser Vite/plugin builds need their own resolved inputs with exact alias/export/virtual-module handling.
3. Keep test imports reachable solely through actual test entry/cfg/gated import.meta.vitest paths outside runtime graph. A module path named oracle or tests cannot waive a real runtime import edge. Static dead code containing a resource may be removed by bundler; record both source reachability and actual output/dep-info provenance without confusing them.
4. Resolve literal filesystem resources and dynamic registry asset mounts. Nonliteral imports/resources must use a closed declared owner set whose realized outputs are validated, or fail as unresolved. Never assume compiler success establishes exclusion.

## Publication and Mount Proof

Bind each built module/component/font/shard/Preview2 asset to owner manifest, profile, resolved features, source input digest, output hash and actual artifact marker. Current marker regular-file/path/hash checks establish artifact identity, not source semantics. Verify no fixture source/input/resource graph reaches those outputs. Review actual Vite productionProviders/serveFile declarations, extension mounts, asset catalogs, and mesh placeholder/metabolism sources; then exercise actual browser/server requests and record resolved served file/hash/owner. This prevents a source-only clean scan from masking a stale published bundle.

## Language-neutral Guard Cases

At minimum test: direct runtime TS import/reexport/literal dynamic fixture import; alias and virtual-module fixture edge; runtime new URL/readFile/include_str/include_bytes resource; cfg(test) allowed edge; #[test] inline allowed edge; inactive testing feature allowed; resolved active testing feature refusal; dev-dependency test-only allowance versus normal dependency edge; concat/environment resource resolution; generated resource fixture origin; stale artifact input digest refusal; canonical Unicode path/ancestry distinction; real per-value Fixture-named contract allowed; unresolved computed path refusal; source and actual compiler/bundler roster mismatch refusal. Compare owned graph/parser decisions with existing Bun/esbuild/Cargo actual input receipts as independent evidence. Do not replace actual native behavior laws.

## Acceptance Receipt

A successful result must enumerate every configured shipped runtime root/target/profile and resolved features, resolved source/resource nodes, generated inputs, output hashes, mounts, all unresolved nodes (zero for complete proof), and forbidden reachable edges (zero). Report exact current-source identity and build timestamps. Independently state schema gate PASS, behavioral test PASS, compilation PASS, graph exclusion PASS and actual served artifact proof; none substitutes for another.

## Feature-gated Fixture Resource Example

Actual Plugin `🦀️.rs:7397–7398` declares artifact_app_laws under cfg(any(test, feature="artifact-app-testing")). Its typing_run function line7536 embeds the typing-run fixture with include_str!. This is allowed under actual test cfg, but becomes a genuine runtime embedded fixture if artifact-app-testing is resolved enabled in a shipped package. Current four guest check flags do not request this feature, and plugin defaults are empty, but dependency unification must be independently resolved. This is a concrete positive guard case, not an assertion the current shipped resolved graph enables it. Native Rust candidate triage found cfg/test markers for every sampled Rust source; nearest preceding markers alone do not establish block ownership and must not be used as a permissive parser.

## Actual Manifest Table Classification

Fresh Bun.TOML parse of Cargo.toml files from the authored roster found 131 dependency feature requests for artifact-app-testing, all in dev-dependencies tables (including target-specific dev tables where present), zero normal/build dependency requests for that feature in this roster. Exact raw table ledger: `🗑️generated/oct8-boundary-auditor/artifact-app-testing-manifest-edges.json`. This validates why a naive manifest text search would falsely accuse many artifact components; preserve dev edges. It does not establish resolved runtime feature zero: workspace feature forwarding/root invocation still needs actual resolver/compiler capture. Initial Python tomllib was unavailable; the completed result uses Bun's actual TOML parser.

A second actual Bun.TOML scan of authored [features] tables found only Plugin's own empty artifact-app-testing declaration, no named/default forwarding feature containing artifact-app-testing. Combined manifest source evidence supports test-only requests; this remains distinct from actual build selected/resolved feature proof.
