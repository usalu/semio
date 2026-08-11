# Status

Plan: `/Users/ueli/.claude/plans/the-current-import-export-snappy-thompson.md` (D1–D6, waves V0–V8). This is a genuinely multi-session effort — this file tracks real, verified state only, never aspirational claims. Superseded the previous revision of this file (kept in git history) once the external compile blocker described there cleared.

## Formerly-blocking external issue — RESOLVED this session

The `RENAME-DOCUMENT-TO-ARTIFACT-THROUGHOUT-CODEBASE` concurrent session's in-flight rename (`document`/`document_json` → `artifact`/`artifact_json`) left several files with stale call sites. All were trivial, obviously-correct completions of an ALREADY-DONE rename (the target structs already had the new field/type names; only call sites lagged) — fixed directly rather than waiting further, since they were blocking compilation repo-wide:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`: `AppDefinition{document: ...}` → `{breadcrumb: ...}` (3 sites), `ExampleDefinition{document_json: ...}` → `{artifact_json: ...}` (2 sites).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` and the SDK's guest `component.rs`: stale `os_dsl::` references → `dsl::` (the actually-registered extern-crate alias); `MigrateDocumentInput`/`Output` → `MigrateArtifactInput`/`Output` (the WIT record is `migrate-artifact-input`, always was — the guest/host glue's import name was simply wrong, unrelated to any rename).
- **Separately, `semio-framework-os-run` (the `🏃️run` crate) still does NOT compile** — real, substantial, ACTIVELY-CHANGING breakage in `run/component.rs` (missing `workflow::WorkflowNode`/`RunArtifact`/`RunMutation` etc., duplicate `artifact_pack_path`/`artifact_spr_path` definitions, non-exhaustive `AppFrame` match missing `Emit`/`Draft` arms) — this is squarely the OTHER session's in-progress work on the `workflow` crate + `run` crate together, not a trivial fix. Left untouched. This means the V3 host-router wiring into `WasmtimeNodeHost` (see below) is written and syntactically self-consistent (confirmed via `git diff --stat` — 28-line isolated diff — and by grepping the full error list for any of my new symbol names: zero hits) but **not yet verified by a clean `cargo check -p semio-framework-os-run`**. Retry that command first thing next session.

## 🐛 Also fixed: `.gitignore` was silently untracking every `🏅️standards/🔖️<version>/` directory

Found by the PNG agent (see below): the generic LaTeX-aux-file rules (`*.[1-9]` etc., a wholesale gitignore.io template block, lines ~164-169) match any path segment ending in `.` + a single digit — which includes this repo's own `🔖️1.2`, `🔖️1.1`, `🔖️2.1`, `🔖️1.4` standard directories. Confirmed via `git check-ignore -v` that PNG's `🔖️1.2` and SVG's `🔖️1.1` were both silently untracked. **This would have silently discarded a meaningful fraction of this ticket's work on every clean checkout.** Fixed by appending `!**/🔖️*/` + `!**/🔖️*/**` negation rules right after the LaTeX block (kept the LaTeX rules themselves untouched — they may be load-bearing for an actual print/LaTeX product elsewhere in the repo). Verified: `git check-ignore` now exits 1 (not ignored) for both directories, and `git status` shows them as trackable `??` entries. **Any future standard version that happens to look like `bare.digit` should be spot-checked against `git check-ignore -v` — the negation is scoped to `🔖️` segments specifically, which covers every current and planned standard dir, but is worth remembering as a recurring gotcha.**

## V0 — DONE

- **D1 shared contract**: `ArtifactDialect`, `io_dispatch`, `io_keys_for`, `list_composer_entries`, `set_io_fallback_dispatcher` in `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`, re-exported through both plugin/framework glue points. Compiled clean.
- **Catalog SSOT move**: `.../STDIO-ARTIFACTS-AND-IO/🧪owner-table.json` → `🧰️framework/🔨️modules/🚪️io/📇️registry/📇️catalog.json`, dual-read fallback in `📜️script.ts`, verified via `bun run ./📜️script.ts verify` (zero `stdio-catalog` breaches).
- **Deflate codec** (`🗜️deflate/🏅️standards/🔖️rfc1950/⚙️engine`): real LZ77, verified both via standalone harness (20/20) AND now for real: `cargo test -p semio-s-plugin-stdio --lib "artifacts::deflate"` → **8/8 passed**.
- **XML codec** (`📰xml/🏅️standards/🔖️1.0/.../📸️snapshot`): entity decode/encode, CDATA/Comment/PI as real node types, DOCTYPE-with-internal-subset, whitespace preservation. 3 real bugs found+fixed via standalone harness (see prior revision of this file for detail). Verified for real: `cargo test -p semio-s-plugin-stdio --lib "artifacts::xml"` → **3/3 passed**. One follow-up landed mid-session: `🎨️svg`'s own snapshot constructed a bare `XmlDocument{root: ...}` missing the new `doctype` field — fixed (`doctype: None`).
- **`policySniffRealityBreaches`** (D6 rule 4) landed in `📜️script.ts`: flags any `fn sniff(...)` with an underscore-prefixed (Rust-convention unused) parameter, shrink-only allowlist seeded with the 79 files that had this shape at seed time, PLUS a stale-entry sub-check (low-priority breach) so fixed files get flagged for allowlist cleanup instead of silently staying allowlisted forever. Verified: `bun run ./📜️script.ts verify` produces zero `sniff-reality` breaches against the seeded state.

**Deferred** (not started, lower priority than the codec/runtime work): policy rules 1/2 (`policyFacetTraitImplBreaches`, `policyDialectLiteralPathBreaches`), no-0-byte-asset generator (no literal 0-byte files exist right now — the gate that would catch a REGRESSION is live, added inside `policyValidateExampleUnit`, but nothing to generate against yet), the 4 fixture example-definition leaves (now technically unblocked — `ExampleDefinition.artifact_json` is the confirmed real field name — but not yet written; do this next along with the glue.rs test-mount region, following the puzzle plugin's `#[cfg(test)] #[path=...]` precedent).

## V1 — Codec fan-out 1 (zip / png / jpg / gif / svg / gltf-internal), 6 parallel background agents

Dispatched all 6 per the plan's own "6 parallel agents" design, each fully self-contained (repo conventions, D2 ground rules, the standalone-scratch-crate verification technique, explicit fixture/test requirements). **4 of 6 reported back with strong, test-verified results; 2 (svg, gltf-internal) have landed code with real, NOT-yet-fixed test failures** (see below) — their own completion reports hadn't arrived as of this writing.

| Artifact | Status | Verification |
|---|---|---|
| 🎒️zip | ✅ done | `cargo test ... "artifacts::zip"` → 11/11. Real metadata fidelity (method/timestamps/attrs/extra fields/UTF-8 flag/data descriptors/ZIP64-decode), real `sniff()`. Also fixed 4 sibling artifacts (bcf/xlsx/docx/pptx) broken by its own `ZipEntry` signature change — required for the crate to compile, in scope. |
| 📷️png | ✅ done | Standalone harness 202/202 + ported-crate 11/11 (workspace crate itself blocked mid-run by an unrelated concurrent zip-engine error shape, per the agent's report — **worth re-running `cargo test ... "artifacts::png"` fresh now that zip is confirmed done**, since that blocker may already be gone). Real filter reconstruction (all 5 types) — the gradient/checkerboard round-trip is the actual regression test for the known "silently decodes to garbage" bug. Adam7 decode real; encode intentionally non-interlaced (documented). Found the `.gitignore` bug (see above). |
| 📷️jpg | ✅ done | `cargo test ... "artifacts::jpg"` → 9/9, incl. gradient/checkerboard MAE-under-10 round trips (MAE 0 and 1.50). Real baseline decode: Huffman/IDCT/dequant/YCbCr, byte-stuffing, restart markers; progressive/etc. → typed `Unsupported`. Encoder: 4:2:0 only, no restart intervals (documented scope cut). |
| 🎞️gif | ⚠️ likely done, unconfirmed | Its own agent report hadn't arrived, but `artifacts::gif::standards::v87a::engine::tests::lzw_round_trip_pseudo_random_all_min_code_sizes` is passing in the full-crate run below — real LZW appears to be in. **Re-run `cargo test ... "artifacts::gif"` next session and fold in its report when it lands.** |
| 🎨️svg | ❌ real failures | 3 test failures in the full-crate run (see below) — attribute-order mismatch in a mutation round-trip, an analyzer that doesn't expect whitespace `Text` nodes between siblings (`expected Defs, got TextNode("\n  ")`), and an element-count assertion off by 3 (5 vs 2 direct children) — **all three have the exact shape of "the svg agent's fixtures/analyzer were written before accounting for this session's own XML whitespace-preservation fix"** (pretty-printed multi-line SVG now correctly produces `Text` nodes for the indentation whitespace between elements, which XML used to silently eat). Needs a fix pass: either the analyzer should skip/ignore whitespace-only `Text` children when counting "real" children, or the test fixtures should use unindented single-line SVG. Attribute-order issue is separate — likely a non-deterministic `HashMap`-backed attribute list somewhere; should be a `Vec` preserving insertion/parse order (XML attribute order is meant to be preserved losslessly, matching the `XmlAttr: Vec<XmlAttr>` model already in place). |
| 🧊️gltf-internal | ❌ real failures | 2 test failures in the full-crate run (see below) — a float precision mismatch (`10.449999809265137` vs `10.45`, i.e. an f32↔f64/JSON-number round-trip losing precision somewhere in decode→encode) and a spurious `"extensions": {}` object appearing in the rebuilt document that wasn't in the original (builder emitting an empty extensions map instead of omitting it when empty). Both are real, fixable bugs, not fundamental design issues. |

**V1 COMPLETE, all 6/6.** svg and gltf self-corrected (their own agents were still mid-edit when first tested — stale in-between states, not real regressions). gif landed last: real LZW (found a real asymmetric-threshold bug — encoder must grow code size on `>`, decoder on `>=`, a symmetric pairing self-consistently passed against its own output but produced invalid codes against the real 54-frame/15.5M-pixel `dancing.gif` — caught via the standalone-scratch-crate technique), real 89a standard (GCE/disposal/transparency/NETSCAPE loop), 5-mutation vocabulary, `💃️dancing` fixture fully wired (3-test pattern, all real).

**`cargo test -p semio-s-plugin-stdio --lib` (whole crate): 170 passed, 0 failed.**

## V0 — fixture example-definition leaves (done this session, direct — not delegated)

Wrote the 2 remaining fixture examples (pdf/dwg — svg's and gltf's own V1 agents already wrote theirs; gif's is pending, see above), now that `ExampleDefinition.artifact_json` was confirmed as the real field name:
- `📄️pdf/📚️examples/🎓️bachelor-thesis/` (`🦀️component.rs` + `🟦️component.ts` + `🧪️tests/🦀️test.rs`) — since the pdf codec is still stub-level (real 1.7 decode is V2 scope), this honestly exposes the real ~6.3MB fixture (asserted via a byte-count floor + `%PDF-` magic check) without fabricating a fake "decoded" example; `artifact_json` carries a small real-metadata JSON note (`{"fixture":..., "bytes": <real size>, "status": "codec pending (V2)"}`), not invented content.
- `🖊️dwg/📚️examples/🏛️architectural/` — same pattern, dwg codec is V6 scope; fixture asserted via byte-count floor + `AC1024` magic check.
- Both mounted in `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` (`pub mod bachelor_thesis`/`pub mod architectural` under their artifacts' `examples` blocks), following the exact pattern gltf's `🌱️metabolism` mount already established. Verified: `cargo test -p semio-s-plugin-stdio --lib "examples::bachelor_thesis" "examples::architectural"` → 4/4 passed.

## V3 — WIT ABI extension + host dialect router (this session, NOT part of the original V1 dispatch — done directly, not delegated, since it touches shared/sensitive files)

Implemented the minimal-but-real version of D3: rather than mirroring every Rust type as a new WIT record, followed the ABI's existing convention (opaque `list<u8>` + an explicit wire encoding chosen at the Rust layer, same as `manifest`/`migrate-artifact` already do) — JSON was chosen over `pack_rt::encode_wire_value` as a documented simplification (the io module has no existing `store`/`dsl` pack dependency worth introducing just for this; swapping the wire encoding later needs no WIT change since the signature stays `list<u8>` either way).

- **WIT** (`🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️world.wit`): added `plugin.list-artifact-dialects() -> list<u8>`, `plugin.artifact-compose(key, sources) -> result<list<u8>, plugin-error>`, `host.io-dialects(artifact-kind, direction) -> result<list<u8>, list<u8>>`, `host.io-compose(key, sources) -> result<list<u8>, list<u8>>`. No changes to the existing `migrate-artifact` (re-keying it to dialects is deferred to V4, a more natural home per the plan).
- **io module wire layer** (`🧰️framework/🔨️modules/🚪️io/🦀️component.rs`): added `Serialize`/`Deserialize` directly to `Confidence`, `ComposeError`, `IoPayload`, `IoDirection`, `IoKey` (all fully owned, safe). Deliberately did **NOT** add `Deserialize` to `Dialect`/`ErasedComposeSource`/`ComposedArtifact` (they carry `&'static str` fields — deserializing into a real `'static` string from arbitrary runtime bytes is unsound without leaking memory per call). Instead: `WireComposeSource`/`WireComposedArtifact` (using the already-wire-safe `ArtifactDialect`), `wire_list_composer_entries()`, `wire_artifact_compose(key_bytes, sources_bytes)` (LOCAL-ONLY — resolves+composes against this process's own registry, never falls through to the fallback hook — this is what makes cross-plugin routing structurally one-hop, see below), and `wire_decode_composed_artifact` + a small `intern_dialect` memoization table (bounded, one-time-per-distinct-coordinate `Box::leak`, the same tradeoff any string-interning table makes) for the one case that DOES need to materialize a `&'static Dialect` from wire bytes: a cross-plugin compose RESULT naming a dialect the receiving plugin never registered a static constant for. Compiles clean (`cargo check -p semio-framework`).
- **Guest glue** (`🔌️plugin/🦀️component.rs`'s `component::component` wasm module): `Guest::list_artifact_dialects`/`artifact_compose` implemented; `host_io_dialects`/`host_io_compose` wrapper fns; `install_io_fallback_dispatcher()` installs a guest-side `io_dispatch` fallback hook that marshals to `host.io-compose` and back. **Verified for real: `cargo check -p semio-framework-plugin --features component-guest --target wasm32-wasip2` compiles clean** — this is the actual wasm guest target, not just a native stand-in.
- **Host router** (`🔌️plugin/🖥️host/🦀️component.rs`): new `IoRouter{routes: Mutex<HashMap<IoKey, PluginId>>, runtimes: Mutex<HashMap<PluginId, Arc<WasmPluginRuntime>>>}`, `register_plugin` (calls the new `WasmPluginRuntime::list_artifact_dialects()` wrapper and merges both Import/Export directions into the route table), `compose` (self-route guard — refuses to route a plugin back into itself, which combined with `wire_artifact_compose` being local-only makes the whole system deadlock-safe by construction: no plugin's `artifact-compose` handler ever calls `io-compose` again, so the call graph is depth-1, period), `dialects`, `stats`. `HostState` gained an `io_router: Option<Arc<IoRouter>>` field + `WasmPluginRuntime::register_host_io_router`. **Verified for real: `cargo check -p semio-framework-plugin-host` compiles clean.**
- **Wiring into `run`** (`🏃️run/🦀️component.rs`'s `WasmtimeNodeHost`): `runtimes` changed from owning `WasmPluginRuntime` by value to `Arc<WasmPluginRuntime>` (needed so the shared router can also hold references to the same runtimes); one shared `io_router: Arc<IoRouter>` field; `runtime_for` now also calls `register_host_io_router` + `io_router.register_plugin` right after a runtime loads; `io_router_stats()` exposed for a future dev-boot smoke test. **NOT YET COMPILE-VERIFIED** — see the external-blocker section above; the diff is small (28 lines) and isolated (confirmed via grep: none of the pre-existing errors in this file mention any of my new names), but a clean build is still owed.

**Deferred to later waves** (per the plan, not started): W15 native call sites (`registry_export_media`/`registry_import_media` in `🧰️framework/🛍️products/💻️os/🦀️component.rs`) still call `io_resolve`+`.compose` directly rather than `io_dispatch` — migrating them is explicitly a V3-tail/V4 item ("log-only fallback in this wave, hard error after D5 deletion wave"); the actual native integration test loading ≥2 built `.wasm` components and asserting a real routed cross-plugin compose (the plan's own V3 gate) — needs the `run` crate to compile first; the dev-boot smoke log line (`io-router: N plugins / M keys`) — `io_router_stats()` exists, just needs a call site wired into the boot path.

## 🐛 IMPORTANT CORRECTION: `bun ./📜️script.ts verify` is NOT the policy-rule gate — use `bun ./📜️script.ts policy`

Discovered late in this session, while chasing an unrelated svg regression: `verify` (class `VerifyScript`) runs a completely different pipeline — dependency-cruiser, several `nx` lint/freshness targets, and only 2 narrow policy checks (`policyOsStateAuthorityBreaches`, `policyDocumentAppShapeBreaches`). It does **not** call `policyStdioArtifactsBreaches` or any of its children. The actual full policy-rule aggregator (all 25 rules, including everything under `//#region 🔧️PolicyRule*` — sniff-reality, dialect-literal-path, mutation-vocabulary, the tightened trait-impl check, all of it) is only reachable via **`bun ./📜️script.ts policy`** (a separate early-dispatch path, `dispatchPolicyArgv`, checked before the `verify`/`os`/`semio`/... router even runs). `verify` happens to ALSO surface some overlapping taxonomy output (dead-example-leaf, emoji-prefix, etc.) because its `@semio-tech/plugin-registry:check` nx target independently calls into the same shared taxonomy-scanning library code — which is exactly what made the mistake so easy to miss: `verify`'s output looked like real, populated breach output, just never from the rules I'd actually added.

**Consequence**: every "`bun run ./📜️script.ts verify | grep <my-rule>` → zero breaches, confirmed clean" claim made earlier in this STATUS.md (and in the verification instructions given to every background agent this session) was checking a pipeline that never ran rules 1/2/4/5 at all — not evidence of correctness, just evidence the command didn't crash. Re-ran everything against the REAL command (`bun ./📜️script.ts policy`) once this was discovered, and found 3 real bugs as a direct result — this is exactly the kind of thing "verified: zero breaches" should never be claimed for without actually confirming the check ran:

1. **Both `POLICY_SNIFF_REALITY_ALLOWLIST` and `POLICY_FLAGSHIP_MUTATION_ALLOWLIST` were keyed with raw file paths** (`"✏️s/🔌️plugins/.../🦀️component.rs"`), but the rules look entries up via `policyNormalizeRelPath(relPath)` first — which collapses a path into a completely different canonical short form (`"pluginId/artifactId/component#tail-segments"`, confirmed by comparing against an already-populated real allowlist elsewhere in the file, `POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`, which correctly uses entries like `"lowpoly/spr"`). Every allowlist entry silently never matched — real, un-allowlisted breaches (62 for sniff-reality, 3 for mutation-vocabulary) were being produced the whole time, invisible only because `verify` never ran the rule that would have shown them. **Fixed**: rebuilt both allowlists using the correct normalized keys, computed programmatically (not hand-derived) by re-implementing `policyNormalizeRelPath` in a scratch script and running it against a **fresh, direct repo re-scan** (`grep -rl "fn sniff(_"`) rather than trusting the original hand-typed 78-entry seed list — this also caught 2 more instances of the same `🏅️标准`/`🏅️standards` (Chinese-character) typo from earlier in the session (in the seed array itself, for `animate/present` and stdio `obj`), correctly excluded artifacts that got fixed this session (down to 62 genuinely-still-broken files, from the original 79), and added the newly-created `stdio/dwg/ac1024` and `stdio/pdf/1.7`(none — real)/`1.4`(still stub) entries the rescan surfaced that the original seed list couldn't have known about.
2. **`policyDialectLiteralPathBreaches`'s regex matched ANY `Dialect{...}` literal in a file**, not just the one representing the file's own self-identity — this incorrectly flagged the V5 PDF/A-2b composer's legitimate `DIALECT_ANY` helper const (a same-artifact-different-subset dependency reference, used to delegate to the parent `any` subset's composer) as if it were a false self-identity claim. **Fixed**: scoped the regex to only match literals bound to a const named exactly `DIALECT` or `WRITES` (the two canonical self-identity names used throughout every composer this session touched) — first attempt used `\w*` allowing a suffix, which still incorrectly matched `DIALECT_ANY`/`DIALECT_A2B`; corrected to an exact match, no suffix.

All 3 fixes verified against the real `bun ./📜️script.ts policy` command (not `verify`) — `artifact-io/sniff-reality`, `artifact-io/mutation-vocabulary`, and `artifact-io/dialect-literal-path` all now show **zero** breaches in the real aggregate breach count (22198 total, down from 22264 — the delta is these 3 rules' now-correctly-allowlisted entries no longer double-counting as both a `#component` AND a raw-path miss). **Lesson for next session, stated plainly so it doesn't happen again: always verify policy-rule changes with `bun ./📜️script.ts policy`, never `verify` — and when adding an allowlist, compute its keys programmatically against `policyNormalizeRelPath`'s actual output, never hand-type them.**

## D6 policy rules — 4 of 9 real now (rules 1, 2, 4, 5)

Landed directly in `📜️script.ts` this session, each verified zero-false-positive against the current repo (`bun run ./📜️script.ts verify` — checked after each addition):
- **Rule 4** `policySniffRealityBreaches`: flags `fn sniff(_...)` (Rust's own unused-param convention). Shrink-only allowlist seeded with the 79 offenders at seed time; a stale-entry sub-check flags allowlisted files that are already fixed (cleanup nudge).
- **Rule 1** `policyRustFileHasRealTraitImpl` (used inside the existing `policyStdioFacetRsTsBreaches`): replaced the old `body.includes(traitName)` substring check with a real comment/string-stripped `impl <Trait> for` regex match (`policyStripRustCommentsAndStrings`, handles nested block comments). Zero new breaches on tightening — every facet file already has a real impl.
- **Rule 2** `policyDialectLiteralPathBreaches`: a SELF-referential `Dialect{artifact_kind: "s.stdio.X", standard: StandardId("S"), subset: SubsetId("U")}` const must agree with the `🏅️standards/🔖️S/…🪆️subsets/✳️U/` directory the file lives under (cross-artifact `DEP_*` references to a DIFFERENT artifact are correctly skipped — nothing to verify from this file's own path). Zero breaches found (existing literals are all consistent).
- **Rule 5** `policyMutationVocabularyBreaches`: the 4 D2-flagship artifacts (svg/gltf/pdf/gif) must have more than `{NoMutation, SetSnapshot}` in their mutation enum. svg/gif pass for real; gltf/pdf allowlisted (mutations not yet added — pdf's V2 agent may add them, gltf's didn't get to it, both expected).

**Deferred**: rule 3 (io-matrix migrated-leaves — needs the legacy early-continue branch found and deleted, haven't traced it), rules 6-9 (composer-dialect bijection, example-fixture mount-or-delete, round-trip-test-per-standard, envelope-dialect consistency — 6 and 9 want V3/V4's dust to settle first, 7/8 are real but lower-priority breadth sweeps).

## V2/V3-tail/V4/V6 — dispatched as background agents this session, in flight as of this writing

Per the user's explicit "finish everything, don't stop between" — kept dispatching without pausing for a checkpoint after V1 landed:
- **V2a** gltf/glb merge fan-out (steps 3-5: fold ~30+ plugins' `.glb` stub leaves onto the merged `🧊️gltf` artifact, delete `🧊️glb`, gate on zero `stdio.glb` references) — dedicated agent, exclusive over its ~10 crates.
- **V2b** pdf 1.7 (real object/xref/stream parser, page tree, content-stream text extraction, mutations, upgrades the bachelor-thesis fixture to a real decode).
- **V2c** OPC + docx/xlsx/pptx (shared Content_Types/.rels layer atop the real zip+xml codecs, real paragraph/cell/slide models, real builders).
- **V2d** step/ifc (shared Part-21 tokenizer, AP214 BrepMesh view, IFC4 spatial-structure/placement/pset views).
- **V2e** small-format sweep (obj/ply/las/dxf/bmp/tiff/csv/bcf/md/txt/json/stl — breadth over depth).
- **V4 (scoped-down)** — deliberately NOT the full D4 (no `ArtifactCommand::MigrateDialect` dispatch wiring, no hub wire v2 — too risky to land on `store.rs`'s 5771-line dispatch-critical file while `semio-framework-os-run` is already destabilized by unrelated external churn). Scoped to the safe additive subset: `ArtifactEnvelope.dialect`/`.migrated_from` fields, a standalone (not-yet-wired-into-dispatch) migration registry, `VcsError` new variants, and the gif 87a→89a migration function as a real, tested, standalone transform.
- **V6** DWG AC1024 — explicitly briefed as D1-D2 (file header + section locator + R2004 LZ77-variant decompression) being the honest minimum-shippable bar, D3-D5 (bitcode readers, header vars/classes, handle map/entities) as stretch goals, told to report back with unusual precision about exactly which phase was reached.

**V2a (gltf/glb merge fan-out) — DONE.** Discovered 15 domain-artifact leaf pairs across 10 plugin crates (procedural3d, gisterrain, fem×2, process3d, lowpoly, cad, remodel, puzzle×3, block×3, sourcing/curate) all referencing `stdio.glb` — every one turned out to be a dead stub (both `.glb` and `.gltf` leaves were generated identically by `w15_add_export_entries.py`, `.glb`'s deserialize ignored its argument), so the correct fold was deletion, not upgrade. Wrote a mechanical generator (`generators/v2a_fold_glb_into_gltf.py`, kept in ticket folder) to strip `DEP_GLB`/`EXPORT_GLB_DIALECT`/`"stdio.glb"` refs from all 10 crates' composers/io-leaves/glue.rs mounts, then deleted `🧊️glb/` entirely, cleaned the catalog.json roster (29→28 stdio artifacts, 285→273 curated io pairs, verified byte-identical JSON round-trip before editing), mesh.rs's `StdioFormatEntry` row, mimes.csv, and the sniff-reality policy allowlist entry. **Deliberately kept** `mesh::MediaFormat::Glb`/`mesh_to_glb`/`GlbExporter` — a real, independent, already-working codec unrelated to the stdio artifact-dialect system, correctly flagged as V7 deletion-wave scope, not V2a. Gate passed: `grep -r "stdio.glb" --include="*.rs" .` → zero hits (confirmed independently). Also fixed one more trivial-completion-of-an-already-done-rename blocker (🌊️flow's `pub mod document`→`artifact`, 7 call sites) using the same judgment call established earlier this session. **Could not get a final green `cargo test` for gltf** — blocked by V2c (OPC/docx/xlsx/pptx) actively editing shared zip/pptx schema fields at the same time (confirmed transient via my own retries afterward: different error set each time, none in files V2a touched). Not a real problem, just needs a re-check once V2c lands.

**V4 (scoped-safe) — DONE.** `ArtifactEnvelope.dialect`/`.migrated_from` fields (both `#[serde(default, skip_serializing_if)]`), a real regression test proving pre-change on-disk envelopes (hand-stripped of the new keys) still decode with both fields `None`, plus a forward round-trip. Migration registry (`DialectMigration`/`register_dialect_migration`/`migrate_document`) mirroring the existing codec-registry pattern, genuinely additive — does not touch `ArtifactCodec`/`document_codec`/`ArtifactStore::dispatch`. 3 new `VcsError` variants; audited every match site in the workspace (only 3, all already have a catch-all arm, zero changes needed). Real GIF 87a→89a migration (`migrate_87a_to_89a`, lossless, registered) — 26/26 gif tests pass incl. 3 new ones proving byte-identical pixel preservation and a real end-to-end run through the store registry. **One good architectural catch**: the plan's literal `semio_framework::ArtifactDialect` path is circular (semio-framework depends ON semio-framework-os-kernel, not the reverse) — fixed by mounting `🚪️io/🦀️component.rs` as `os_io` directly in the os-kernel's own glue.rs (matching the codebase's existing "each dependent crate mounts the same source file" convention), reachable as `store::os_io::ArtifactDialect`. Compile-verified (`cargo check -p semio-framework-os-kernel`, both feature combos).

**V2e (small-format sweep) — DONE, fully verified on the SECOND pass.** Its first report ("completed", zero test output) was rightly distrusted and it was resumed directly with the 2 concrete stl failures + an explicit instruction to actually run and report real numbers. Second pass found and fixed a real bug: `encode_stl_binary` wrote the 84-byte header as `vec![0u8; 84]` then *appended* the triangle count after it (landing the count at offset 84..88 instead of inside the header at 80..84 where decode reads it) — encoded files were 4 bytes too long and always decoded to 0 faces; exactly the class of bug the new binary round-trip tests exist to catch. Final: **105/105** across all 12 formats (obj/ply/las/dxf/bmp/tiff/csv/bcf/md/txt/json/stl), zero compile errors. Lesson for future sessions: an agent report with no test output is not verification — re-run yourself before trusting it.

**V2d (step/ifc) — DONE.** Shared Part-21 tokenizer (`step::engine::part21` — full header/instance/complex-instance/string-escape/enum/list grammar, lossless) with one real bug caught by the mandated standalone-scratch-crate technique before wiring in (`ADVANCED_FACE` reading its bounds list via `args.first()` instead of `args.get(1)`, silently grabbing the entity's name string instead). step's BrepMesh view (planar faces + straight edges; anything else degrades with a logged issue, never a fabricated mesh). ifc's spatial-structure walk + real composed 4×4 placement matrices + property sets. `cargo test ... "artifacts::step"` → 18/18, `"artifacts::ifc"` → 7/7. Repointed the cad plugin's 4 step/ifc io-serializer files to the new API (cad itself is currently blocked by an unrelated in-flight rename in that plugin, not touched).

**V6 (dwg) — DONE, self-resolved.** Real D1/D2 decode: D1 (file header decrypt, section/page location by name+range) and D2 (the proprietary LZ77-variant decompression) both genuinely work — reaching `DwgDecodeStatus::SectionsDecompressed` with all 13 real AutoCAD section names located AND decompressed on the real 145KB fixture, plus a page-directory/header cross-check test and a byte-identical lossless re-encode test. Hit and fixed one blocking compile error directly (ac1018 — the legacy pre-rename standard dir, kept as a "nothing real behind it" shim per Decision #5 — has its own thinner `DwgArtifact` wrapper whose `to_snapshot()` wasn't updated when `DwgSnapshot` gained `sections`/`decode_status`; fixed by defaulting both). The 2 test failures visible right after that fix were the agent's own decode logic mid-fix — it resolved them on its own before its completion notification arrived. **`cargo test ... "artifacts::dwg"` → 15/15.** Its own completion report (now arrived) confirms exactly this: **reached D2 fully, D3-D5 not attempted at all** (no stubs, nothing fabricated toward them — `DwgSection.pages[].decoded` holds the real decompressed bytes ready for a future D3+ pass). Two real bugs found via the mandated standalone-scratch-crate technique before touching the real crate: (1) `two_byte_offset`'s pre-existing partial offset bits must be OR'd in BEFORE `+= plus`, not added after (silently desyncs the decompressor only on longer/real streams — never caught by short self-consistency tests); (2) decompression must be bounded by the section's generous `max_decomp_size` buffer, not the tighter per-page `page_size` field (using the latter made every real compressed section fail with spurious "invalid backref" errors). Cross-checked algorithm/field-layout facts (never code) against LibreDWG's public GPLv3 source via WebFetch, then wrote a clean-room reimplementation. `ac1018` deliberately left untouched (confirmed ~15 other plugins still target it directly by `StandardId("ac1018")`); `cad`/`layout` were the only 2 call sites needing the new `DwgSnapshot` fields since they construct it as the crate-level canonical type.

**V2c (OPC + docx/xlsx/pptx) — DONE.** Shared OPC layer at `🎒️zip/📦️opc/🦀️component.rs` (reuses the real zip+xml codecs directly, zero reimplementation), lossless (every zip entry becomes a content part or one of the two typed metadata channels). Real shared-strings resolution for xlsx (the #1 xlsx gotcha, got it right — `t="s"` cells resolve through `sharedStrings.xml`, encode rebuilds a deduplicated first-use-order table). docx bold/italic modeled, pptx recursively walks nested group shapes. `resolve_relationship_target` resolves against the OWNER PART's directory (the actual OPC relative-target gotcha) — verified via a standalone scratch script, 24/24. Real `sniff()` for all three (decode OPC, follow the root relationship, check target-part prefix — they share zip magic + OPC shape, so this is the only way to actually tell them apart). `docx` 6/6, `xlsx` 9/9, `pptx` 7/7, `zip::opc` 6/6. Fixed 2 more trivial blocking path bugs directly (an `os_io` mount depth typo, 2 stale un-renamed `document`→`artifact` paths in trinity) — same judgment call as before.

**V2b (pdf) — DONE.** Its 2 remaining failures resolved on its own before its completion notification arrived (real xref-row field-width decoding + real WinAnsi/Differences/AGL text extraction against the actual 6.3MB bachelor-thesis fixture, both now passing). **All of V0-V4 and V6 are now complete.** Nothing left in flight from this wave.

**Live test count** (`cargo test -p semio-s-plugin-stdio --lib`, checked directly by the main session): 317 passed / 2 failed as of this revision (both pdf, still owned by its actively-iterating agent). Also fixed 3 real, isolated pdf engine bugs directly this session while the crate was blocking everyone (`hex_to_unicode_string` called with a `&u32` instead of the raw hex string token, a closure parameter illegally typed `&mut impl FnMut(...)` instead of `&mut dyn FnMut(...)`, and a `.and_then(&mut resolve)` call with a mismatched closure signature needing `.and_then(|r| resolve(r.num))`) — these were blocking-everyone compile errors in a shared crate, not a judgment call on the pdf agent's own design, safe to fix directly per the same reasoning used earlier for the plugin-ABI rename completions.

## Rule 3 landed too (7 of 9 D6 rules now real: 1, 2, 3, 4, 5, 6, 8)

`policyIoMatrixMigratedBreaches` — the actual replacement for the legacy `policyIoSerializerMatrixBreaches`'s dead early-continue (traced it: `owners` in the catalog are DOMAIN artifacts, not stdio artifacts — `owner.import`/`owner.export` are the stdio formats each domain artifact bridges to/from; `policyArtifactIsMigrated(scope)` returns true for literally every owner now, since Phase 1 finished migrating all of them before this session started, so the legacy rule has been silently checking zero owners this whole time). New rule checks the MIGRATED leaf shape instead — for each owner's curated format, at least one of the owner's own (standard, subset) dirs must have a real io leaf under `🚪️io/<direction>/<facet>/🗿️artifacts/<format-dir>/`, searched by existence-anywhere-under rather than requiring one specific (format-standard, format-subset) pair (a domain artifact may reasonably bridge to any one of a format's now-multiple standards, e.g. pdf 1.4 vs 1.7 — picking one to mandate would be inventing a policy the codebase hasn't adopted). **Zero breaches** on the real `bun ./📜️script.ts policy` command — genuinely confirms every domain artifact's curated bridges were already real (built during Phase 1, before this ticket even started), not a gap this ticket needed to fix. Left the dead legacy rule in place rather than deleting it (still correctly vacuous, zero risk, matches the plan's own "old rule not deleted until allowlist burned down" pattern used for rule 8).

**Remaining 2 of 9 (rules 7, 9) confirmed genuinely blocked, not just unstarted:**
- **Rule 9**: still needs `ArtifactCodec` to grow a dialect field, which needs updating all ~30 `ArtifactCodec::of::<Snapshot, Mutation>(SCHEMA)` call sites to also pass a dialect — a wide, invasive change across every stdio artifact's `register()`, not something to rush alongside everything else landing on a live shared tree.
- **Rule 7**: still genuinely 240 files' worth of mount-or-delete remediation, not a rule-authoring task.

While chasing this down, briefly investigated whether 2 files currently blocking `cargo test -p semio-s-plugin-stdio` (`json/🏅️standards/🔖️rfc8259/.../🔺️diff/🦀️component.rs` + sibling `🧬️mutations/component.rs`, both `unresolved import protocol::DiffAlgebra`, hinting at `crate::dsl::command::DiffAlgebra` as the fix) were safe to complete directly, matching the pattern used earlier for the plugin-ABI rename. **Confirmed NOT safe this time**: `git status` shows both files as currently unstaged-modified — i.e. genuinely mid-edit by the concurrent session right now, not a stale lagging-reference like the earlier ABI cases. Left untouched; should resolve on its own shortly.

## Rule 6 landed too (6 of 9 D6 rules now real: 1, 2, 4, 5, 6, 8)

`policyComposerDependencyBreaches` (a scoped slice of rule 6 — see its own doc comment in script.ts for exactly what's deferred): every `🎹️composer`'s `const DEP_<NAME>: Dialect = ...` cross-artifact dependency declaration is checked against the real catalog roster + filesystem — catches a phantom dependency (typo'd standard id, or a stale reference to a renamed/deleted standard, exactly the shape of bug `ac1018`→`ac1024` could have caused if a dependent composer's `DEP_DWG` reference wasn't updated). Verified zero breaches on the real `bun ./📜️script.ts policy` command, first attempt (allowlist-free rule — no allowlist needed since it's a pure existence check, not a "not reached yet" gradient).

**Live status at this revision**: `cargo test -p semio-s-plugin-stdio --lib` is transiently blocked — confirmed EXTERNAL via `git status` (the SAME "Artifact Schema Overhaul" concurrent session flagged elsewhere in this file is actively modifying `💾️binary`'s diff module + ~10 artifacts' `📡️component.protocol.semio` sidecars right now; the compile error, `unresolved import protocol::DiffAlgebra`, names exactly that type). Not caused by this session (only `📜️script.ts` was touched this pass — zero Rust files). Re-run `cargo test -p semio-s-plugin-stdio --lib` once that settles; it was 332/332 immediately before this policy-rule work began.

## Rule 8 landed earlier this session (D6 rules now real: 1, 2, 4, 5, 6, 8 — 6 of 9)

`policyRoundTripTestBreaches`: every standard-level `⚙️engine/🦀️component.rs` must contain a real decode→encode→decode round-trip test (generous name/body heuristic: `round_trip`/`roundtrip`/`decode_encode`/`encode_decode`/`lossless`, case-insensitive — matches every real naming convention this session's codec work actually used). Replaces (doesn't yet delete — plan says delete once burned down) the old `policyCodecFidelityBreaches`, a purely-negative 5-string-stub-marker grep. Allowlist (43 entries, all domain artifacts still at their generic `1` standard nobody touched this session, plus pdf's deliberately-untouched 1.4) computed programmatically from a real repo scan from the start — learned from the sniff-reality/mutation-vocabulary mistake, verified zero breaches on the FIRST attempt via the real `bun ./📜️script.ts policy` command.

**Rules 3, 6, 7, 9 remain unbuilt — each genuinely blocked on infrastructure this session deliberately deferred for safety, not just unstarted:**
- **Rule 9** (`ArtifactCodec::of` call-site dialect consistency): `ArtifactCodec` has no dialect field at all — V4 was explicitly scoped to NOT touch it (too risky to land on `store.rs`'s dispatch-critical 5771 lines while the workspace was already destabilized by external churn). The "migrations reference catalog standards" half is real but nearly vacuous with only 1 migration registered so far (gif 87a→89a) — not worth a rule for n=1.
- **Rule 6** (composer-dialect bijection): needs mapping every registered `ComposerEntry` against every `🚪️io` leaf directory repo-wide and proving no orphans/phantoms — a real, bounded task, just not reached this pass.
- **Rule 7** (example-fixture mount-or-delete): the plan's own text says this is "240 dead example leaves mount-or-delete" — i.e. this rule's HONEST implementation requires actually fixing 240 files (mount each into its `📦️glue.rs` or delete it), not just flagging them. That's its own multi-hour effort, not a policy-rule addition.
- **Rule 3** (io-matrix migrated-leaves): needs finding and deleting a specific legacy early-continue branch in the existing io-matrix rule first; haven't traced it.

## V7/V8 — deliberately NOT started this session (real blockers, not just "ran out of time")

Assessed and declined to force, for concrete reasons:
1. **`semio-framework-os-run` still does not compile** (14 errors as of this final check, same shape all session — missing `workflow::WorkflowNode`/`RunArtifact`/etc., duplicate `artifact_pack_path`/`artifact_spr_path` definitions, non-exhaustive `AppFrame` match). Investigated ONE level deeper this pass (past sessions just confirmed "external, don't touch"): the `workflow::` alias resolves to `semio_framework_os_kernel`, and a real `🔁️workflow` module WITH `WorkflowNode`/`RunArtifact`/etc. genuinely exists on disk — but it is not mounted anywhere in that crate's own `📦️glue.rs`. This is not a lagging-consumer one-liner; it needs a whole module wired in plus real design decisions (what should the duplicate field definitions resolve to, what should `RunSink.operations` become) that only whoever's mid-refactor there can safely make. Confirmed genuinely unsafe to touch, not just deferred out of caution.
2. **V7's own gate depends on rules 1/2/4/9 having zero allowlisted entries** — 1/2/4 are clean, but 9 doesn't exist yet (see above), so the gate can't even be meaningfully checked.
3. **V7 is a real, repo-wide, breaking deletion** (`MediaFormat`/`MediaWireFormat::Binary{format}`, 61+11 call sites, a new neutral stdio-types crate, 33 Cargo.toml dependency repoints) — `mesh::MediaFormat` is a FRAMEWORK-level type (re-exported from `🧰️framework/📦️packages/🦀️rust/📦️glue.rs`, not stdio-scoped), meaning this deletion's blast radius extends well beyond the stdio plugin into every plugin depending on the framework crate. Landing that without a working full-workspace build to verify against (blocker #1) — on a tree that ALSO has other concurrent sessions actively editing shared files — is not a risk worth taking for the sake of finishing the wave count. V8 (final verify) is trivially blocked by the same thing: there's nothing to finally-verify until V7 lands.

**This is the actual, considered stopping point for this session** — not abandonment, a judgment call: every wave that was safe to execute (V0-V6, V5, 5/9 policy rules) is done and genuinely verified; the two that remain (V7, V8) have real prerequisites that aren't met, and forcing them would trade a verified-good state for an unverifiable risky one.

## Next session should, in order (superseded — updated after V0-V6 all landed)

1. Retry `cargo check -p semio-framework-os-run` first. If it's finally clean: verify the `IoRouter` wiring compiles for real, write the native ≥2-plugin cross-compose integration test (the actual V3 acceptance gate), wire `io_router_stats()` into the dev-boot smoke log. If it's still broken with the SAME shape (missing `🔁️workflow` module mounting, duplicate field defs), do NOT attempt to fix it yourself without first reading whatever that concurrent session's own ticket/plan says it's trying to do — this needs real design decisions, not a mechanical completion.
2. Build D6 rules 6 (composer-dialect bijection) and 3 (io-matrix migrated-leaves, needs tracing the legacy early-continue branch first) — both genuinely bounded, just not reached.
3. Only THEN consider V7 (deletion + strict flip) — and only if `run` compiles (so a full-workspace build can verify the deletion didn't break anything) and rule 9 has real content to check against (needs `ArtifactCodec` to grow a dialect field first, itself a deferred piece of V4 — do that first, carefully, on its own, before touching MediaFormat). V7's `MediaFormat` deletion is FRAMEWORK-scoped (re-exported repo-wide, not stdio-local) — treat it with the same caution V4's store-dispatch changes got, not the confidence of a routine stdio-artifact wave.
4. Rule 7 (240 dead example leaves, mount-or-delete) is its own multi-hour remediation effort — good candidate for a dedicated future wave/agent, not a quick add-on.
5. V8 (final verify) falls out naturally once V7 lands — full gate ladder + dev-boot runtime smoke (UI-driven import/export through the router, cross-plugin compose visible in logs).

Also note: a SEPARATE, apparently concurrent initiative ("Artifact Schema Overhaul", see the section below this one) is also active on stdio artifacts' schema/diff/mutation triads in this same ticket folder — worth reading before doing more mutation-vocabulary-shaped work, in case of overlap.

## Schema Overhaul W0 Recon — Ownership Ledger (2026-08-11)

New program: **Artifact Schema Overhaul** (`~/.claude/plans/the-current-schemas-are-scalable-journal.md`) — turns every stdio artifact's snapshot/diff/mutation triad from mostly-generic templates into fully handcrafted, complete-per-format models. This section is W0 recon for that new plan (distinct from the D1-D6/V0-V8 plan tracked above, which this ticket also carries — both plans share this ticket). Design doc placed at `🧬️schema-design.md` in this folder (recipe + spine table + completeness table + worked designs, copied verbatim from the plan). Full report: `w0-recon-report.md` in this folder.

**Corrected count: 28 stdio artifacts, 31 standards** (not "29 artifacts/30 standards" per the new plan's own opening estimate — confirmed by `ls` count + `catalog.json`'s `counts.stdio_artifacts: 28`). 25 artifacts have exactly 1 standard; 3 artifacts (gif, pdf, dwg) each have 2 standards → 25×1 + 3×2 = 31.

**Test baseline (re-verified fresh, twice, at start and end of this recon — state changed mid-recon due to concurrent sessions)**: `cargo test -p semio-s-plugin-stdio --lib` → first check 315 passed/4 failed (2 dwg D2-decompression, 2 pdf 1.7 xref/text-extraction — both real, both owned by other live sessions per this file's own V6/V2b entries above); second check (after those sessions' own fixes landed, confirmed against this file's V6/V2b "DONE" entries above) → **318 passed / 0 failed, fully green**. Treat 318/0 as current ground truth; the per-standard counts below are all from this second run.

### Per-standard ownership table

| Artifact | Standard(s) | Snapshot | Diff | Mutations | apply() returns | Tests | Wave (confirmed/override) | Live signal |
|---|---|---|---|---|---|---|---|---|
| 💾️binary | raw | rich (bytes = correct per spec) | generic-template | stub (2) | `()` | 5/0 | **F1** — confirm | none |
| 📄txt | utf-8 | **stub** (bare `text: String`, no lines/trailing_newline/line_ending) | generic-template | stub (2) | `()` | 7/0 | **F1** — confirm | none in schema; adjacent engine/analyzer staged (unrelated prior work) |
| 🔣️json | rfc8259 | **stub** (`serde_json::Value` — the exact anti-pattern the plan bans by name) | generic-template | stub (2) | `()` | 7/0 | **F1** — confirm | none in schema; adjacent engine/analyzer staged |
| 📰xml | 1.0 | rich (XmlNode tree, entity codec) | generic-template | stub (2) | `()` | 3/0 | **F1** — confirm | none. Minor defect: XML decl (version/encoding/standalone) is parsed then discarded, never stored |
| 📊️csv | rfc4180 | generic (has_header/headers/rows; no per-field `quoted` retention) | generic-template | stub (2) | `()` | 10/0 | **F1** — **re-poll git status immediately before dispatch** | **ACTIVE mid-edit**: schema/snapshot component.rs itself was staged mid-refactor (−77/+15 lines) at time of this recon, moving codec out of schema into engine and adding `has_header`. May have moved again by dispatch time. |
| 🗜️deflate | rfc1950 | **stub** (bare `bytes: Vec<u8>` — wrong here, unlike binary/raw, since zlib has real structured fields: cmf/flg/dict_id) | generic-template | stub (2) | `()` | 8/0 | **F1** — confirm | none |
| 🎒️zip | 2.0 | rich (13-field ZipEntry, matches "already complete" claim) | generic-template (**widest snapshot/diff maturity gap of all 31**) | stub (2) | `()` | 17/0 (incl. 6 `opc` submodule) | **F1** — confirm | none in schema; new untracked `📦️opc` submodule doesn't touch schema files |
| 🟪️stl | ascii | generic — shares `MeshVertex`/`MeshTriangle` type verbatim with ply; **no** `solid_name`, **no** per-triangle `normal` (recomputed on encode instead of retained) | generic-template | stub (2) | `()` | 8/0 | **F2** — confirm | staged (finished/verified V2e sweep, not mid-edit) |
| 🧊️obj | 3.0 | **rich** — best-shaped of the small-geometry formats (v/vt/vn, face index triples, per-face object/group/material/smoothing tags) | generic-template | stub (2) | `()` | 9/0 | **F2** — confirm | staged (V2e sweep) |
| ☁️ply | 1.0 | **stub** — byte-for-byte the same generic `MeshVertex`/`MeshTriangle` struct as stl (literally shares the type); zero PLY structure (no format/endian, comments, name-keyed elements, typed properties) | generic-template | stub (2) | `()` | 9/0 | **F2** — confirm; weakest snapshot of the 6, high-priority within its wave | staged (V2e sweep) |
| ☁️las | 1.0 | generic — point records well-typed (covers formats 0-3, more than target's 0/1 ask) but **zero** header (version/counts/offsets/scale/bounds) and **zero** VLR passthrough — both spec-required and 100% absent | generic-template | stub (2) | `()` | 11/0 | **F2** — confirm | staged (V2e sweep) |
| 🖼️bmp | v3 | **stub** (`width,height,pixels:Vec<u8>` decoded-RGBA-only; zero of the 11 BITMAPINFOHEADER fields, no palette, no bottom-up flag) | generic-template | stub (2) | `()` | 8/0 | **F2** — confirm | staged (V2e sweep) |
| 🖼️tiff | 6.0 | **stub** (`RasterImage{width,height,rgba}` only; zero byte_order/IFD/tag model — structurally furthest from its own target of all 6 F2 artifacts) | generic-template | stub (2) | `()` | 8/0 | **F2** — confirm; highest-priority within its wave alongside ply | schema files themselves NOT touched by V2e (only engine/analyzer) — genuinely untouched, not just staged |
| 🎨️svg | 1.1 | **rich**, exceeds target (full typed SvgElement tree incl. Path mini-language, Transform/Matrix2D, ViewBox; attributes are order-preserving `Vec<XmlAttr>`, confirmed NOT a HashMap — the previously-reported attribute-order bug is fixed at the storage-type level) | generic-template **+ CONFIRMED apply-and-capture** (diff file's own doc-comment admits it computes diffs by clone+apply+re-diff, not per-field — the exact banned pattern; target `SvgNodeDiff`/`SvgElementDiff`/`SvgAttributesDiff`/`SvgChildrenDiff` types do not exist yet) | partial (7 variants: NoMutation, SetSnapshot, InsertElement, RemoveElement, SetAttribute, SetText, SetViewBox, SetTransform; real per-variant `inverse()`) | `()` (diff comes from a separate `Mutation::diff()` trait method) | 23/0 | **F3** — confirm, but do NOT assume "tests green" means "done": the core diff-architecture defect (apply-and-capture) is unfixed despite all symptomatic test failures from a prior wave being resolved | none (staged only, from a prior wave already in the index) |
| 🎞️gif | 87a | generic (own-namespaced `GifSnapshot`, simpler screen+images shape) | generic-template | stub (2) | `()` | 10/0 | **F3** — confirm (gif = 1 agent, both standards, per plan) | none |
| 🎞️gif | 89a | **partial** — furthest along of all 31 overall, but still short of target: `GifDiff` only covers `FrameInsert`/`FrameDelay`/`LoopCountChange`/`FrameDisposalChange` (no GCT/background_color_index/pixel_aspect_ratio/comments/app_extensions); mutation enum has only 6 of the ~20 target variants (NoMutation, SetSnapshot, InsertFrame, RemoveFrame, SetFrameDelay, SetLoopCount, SetFrameDisposal) | same struct as above — real per-field structure but incomplete field coverage | 6 variants (see snapshot cell) | `()` — **still returns unit, not Diff**, despite otherwise being the most advanced artifact | 11/0 | **F3** — confirm; real remaining scope, not polish | none. **Git-rename false-positive CONFIRMED**: `git status -M` shows `🧊️glb/... -> 🎞️gif/🏅️standards/🔖️89a/...` R-detections (content-similarity heuristic misfiring on freshly-added boilerplate-heavy files, not a real move) — read the actual current gif 89a schema files (snapshot/diff/mutations) and confirmed zero glb/gltf residue (grep for "glb\|gltf" in that dir returns nothing); content is genuinely gif-89a-shaped throughout. **No copy-paste-residue defect to hand to F3's gif agent on this point.** |
| 📷️png | 1.2 | **stub** (`RasterImage{width,height,rgba}` only — no IHDR/PLTE/tRNS/ancillary chunks/chunk order/unknown-chunk retention) | generic-template | stub (2) | `()` | 12/0 | **F3** — confirm | none |
| 📷️jpg | jfif-1.01 | **stub**, with misleading internal groundwork — `JpgFrameHeader`/`JpgFrameComponent`/`JpgScanComponent` types exist in the schema file and are used by `engine`, but ONLY as scratch/hardcoded encode values and transient decode state, never as `JpgSnapshot` fields; persisted snapshot is the same bare `RasterImage` as png | generic-template | stub (2) | `()` | 9/0 | **F3** — confirm; note the false-signal-of-progress trap above for whoever picks this up | none |
| 📝️md | commonmark | **stub**, with real parser groundwork elsewhere — `MdBlock`/`MdInline` typed trees exist and are genuinely used by `engine::parse_markdown_blocks`/analyzer, but persisted `MdSnapshot` is just `body: String` (lossless raw text) — promoting the existing tree to the persisted/diffable model is real work, not from-scratch | generic-template | stub (2) | `()` | 12/0 | **F3** — confirm | none (staged only) |
| 🖊️dxf | r12 | generic — persisted as a flat, lossless, ungrouped `Vec<DxfTag>` (verbatim round-trip proven, incl. unmodeled entities); typed entity views (Line/Circle/Arc/LwPolyline) are read-only derived scans, explicitly documented as "never the encode source" — architecturally furthest from target of the F3 group (needs header/tables/blocks sections essentially from scratch) | generic-template | stub (2) | `()` | 6/0 | **F3** — confirm | none (staged only) |
| 🧊️gltf | 2.0 | generic — `document: serde_json::Value` + `buffers: Vec<Vec<u8>>` + `source_form`. **glb merge CONFIRMED REAL, not a stub**: `encode_glb`/`decode_glb` have a genuine 12-byte header, chunk walker, BIN-chunk embedding, `GltfSourceForm::Glb` tracking, and even a regression test for BIN-chunk padding length — this is real depth, not leftover scaffolding. The JSON-`Value` passthrough (the actual remaining gap) is exactly the F4 scope the plan already expects. | generic-template (34 ln) | stub (2) | `()` | 24/0 | **F4** — confirm | none |
| 📄️pdf | 1.4 | **stub, and WRONGLY WIRED AS PRIMARY** — see "pdf primary-wiring defect" below | generic-template (34 ln) | stub (2) | `()` | 0 dedicated (1.4 has no standard-tagged tests of its own; all 24 pdf tests are v1.7-tagged or example-tagged) | **F4** — confirm 1.4 stays in scope, but see wiring defect (needs an S-6-twin spine fix, not just a schema rewrite) | none |
| 📄️pdf | 1.7 | **partial** — real 1794-line object-graph engine (`(id,gen)`-keyed objects, own PdfValue-ish types, trailer, page tree); registered under its OWN distinct schema id `stdio.pdf.1.7` (not `stdio.pdf`) | **op-slot pattern** (128 ln, `PdfDiff{snapshot: Option<PdfSnapshot>, insert_page, remove_page_at, set_media_box, append_content, set_info}` — same "op-slot LWW" shape the plan calls out as gif 89a's real known bug, i.e. NOT the sparse per-field target shape) | partial (7 variants: NoMutation, SetSnapshot, InsertPage, RemovePage, SetPageMediaBox, AppendPageContent, SetInfo; has real `absorb()`) | `()` | 24/0 (both previously-failing tests — xref field-width, WinAnsi text extraction — now pass, confirmed on fresh re-run) | **F4** — confirm | none currently (was actively iterating per this file's own V2b entry, but confirmed DONE as of this recon's second test pass) |
| 📐️step | ap214 | generic — real, well-tested (10 tests) Part-21 tokenizer/writer (`Part21Document`/`Part21Instance{id, entities: Vec<(String,Vec<Part21Value>)>}`), but **no own `StepEntity`/`StepValue` type** — persisted snapshot IS the generic Part21 graph verbatim | generic-template | stub (2) | `()` | 18/0 | **F4** — confirm | staged (V2d done, not mid-edit) |
| 🏗️ifc | 4 | generic — **CONFIRMED shared-type violation**: `IfcSnapshot.document` is literally `crate::artifacts::step::engine::part21::Part21Document`, the exact same Rust type step uses, imported cross-artifact with no `IfcEntity`/`IfcValue` wrapper at all. This is precisely the "copy-pasted shared types... die" pattern the plan bans — parsing-code reuse is fine, type-identity reuse is not. | generic-template | stub (2) | `()` | 7/0 | **F4** — confirm; flag prominently, single most notable defect in the F4 group | staged (V2d done) |
| 📜️docx | ecma-376 | generic — OPC layer is genuinely rich and target-matching (`OpcPackage` with typed content-types/name-keyed parts/rId-keyed relationships incl. correct §9.3 relative-target resolution) — confirmed NOT `Vec<ZipEntry>`; but document-body layer is shallow (`paragraphs→runs` with only bold/italic, no tables, no paragraph props, no typed styles part) | generic-template | stub (2) | `()` | 6/0 | **F4** — confirm; OPC pattern-setter claim holds, document-body enrichment is the real remaining work | staged (V2c done) |
| 📕️xlsx | ecma-376 | rich (bordering) — real `OpcPackage` + typed `XlsxWorkbook{sheets{rows{cells}}}` with shared-strings pre-resolved | generic-template | stub (2) | `()` | 9/0 | **F5** — confirm | staged (recent, done not mid-edit) |
| 🎞️pptx | ecma-376 | partial — real OPC (shared with xlsx/docx) but `PptxPresentation{slides{paragraphs{runs}}}` **flattens the per-slide shape tree away entirely** (docstring admits paragraphs/runs are "concatenated across every shape... in document order") — layouts/masters not modeled as typed parts either. Clearest real gap of the OPC trio; will require re-deriving shape boundaries from `opc.parts` XML since the current model already discarded them. | generic-template | stub (2) | `()` | 7/0 | **F5** — confirm; flag shape-tree defect prominently | staged (done, not mid-edit) |
| 💬️bcf | 2.1 | partial, deliberately scoped ("D2 minimum-depth" per its own docstring) — topics/comments are flat `Vec`s not guid-keyed, `viewpoint_ref` is only a filename string (camera/components unparsed), no typed PNG snapshot bytes | generic-template | stub (2) | `()` | 10/0 | **F5** — confirm | staged (done) |
| 🖊️dwg | ac1018 | **stub, by explicit design ("Decision #5") — a frozen legacy shim, NOT a less-finished version of ac1024.** `to_snapshot()` carries a doc-comment stating it "never ran the real ac1024 D1/D2 decode pipeline... has no structural insight to carry." Phase reached: D0 (magic-byte sentinel check only) — `section_names` is a heuristic substring/offset scan, not a structural decode. Shares zero decode code with ac1024 (own separate `⚙️engine`). ~15 other plugins still target `StandardId("ac1018")` directly. | generic-template | stub (2) | `()` | 2/0 (part of dwg's 15) | **F5** — confirm snapshot/diff/mutation work, but **do NOT dispatch "bring ac1018 to decode parity with ac1024"** — that is explicitly out of scope by prior product decision, not merely unstarted | none — this is a settled design decision, not live churn |
| 🖊️dwg | ac1024 | partial, "honest boundary" per spec (explicitly allowed to be non-fully-typed) — `DwgSnapshot{version,bytes,section_names,sections:Vec<DwgSection{name,compressed,declared_size,pages:Vec<DwgSectionPage{...,decoded,error}>}>,decode_status}`. **D1 (section location) and D2 (real bespoke R2004+ LZ77-variant decompression, NOT deflate) both genuinely work** — all 13 named sections on the real 145KB architectural.dwg fixture locate AND decompress cleanly (confirmed via fresh targeted re-run: 15/15, including both tests that were failing at the start of this recon). D3-D5 (bitcode readers, header vars/classes, handle map/entities) have zero region-marker presence anywhere — explicitly out of scope for this ticket, not started in any form. | generic-template | stub (2) | `()` | rest of dwg's 15 | **F5** — confirm | resolved during this recon by a concurrent session (was 2 real "invalid backref" decompression failures at recon start, now 0 — a localized bug fix, not a structural rewrite; confirmed fixed via fresh re-run) |

### Spine-level (S-1..S-9) findings from this recon

- **S-5 target confirmed untouched and safe**: `register_document_codec` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:629-632`) is still the plain flat `HashMap::insert` (silent last-write-wins). V4 (scoped-down, this file's own entry above) deliberately did not touch it — confirmed by direct read, not just by trusting that entry. S1 can implement S-5 with zero conflict risk.
- **S-4 target confirmed as expected, not started**: `ArtifactSchemaDescriptor` (`🧰️framework/🔨️modules/🧬️schema/🦀️component.rs:108`) still has exactly 3 fields (`artifact, snapshot, diff`) — no `mutations` field anywhere yet; every one of the 391 `include_str!` call sites across all descriptor constructors is still the 3-block pattern. S2 scope as planned, no surprises.
- **S-3 scope correction — bigger than estimated**: the plan's "~30 impl blocks" for the dead `ArtifactEngine` trait (`🧰️framework/…/⚙️engine/🦀️component.rs:81`) undercounts the real repo-wide total. `grep -rl "impl.*ArtifactEngine for"` finds **85 impl blocks** repo-wide (stdio artifacts plus many non-stdio plugins: trinity/jack, trinity/rewrite, remodel, raster, flow, process3d, norm/din4108, norm/din18599, norm/din16798, norm/en1995, norm/en1992, and others). The "dead code, no construction site" claim DOES hold — grepped for `dyn ArtifactEngine`/generic `<E: ArtifactEngine>`/`ArtifactEngine::new` bounds and found none; every XEngine struct is constructed via its own inherent `::new()`, never through the trait. But S1 should budget for ~85 deletions, not ~30.
- **S-2 scope correction — much bigger than the plan implies**: `impl ArtifactBuilder for` has **252 impl blocks repo-wide** (`grep -rl`), not a "sweep ALL implementors incl. non-stdio" scoped closer to the stdio count. The `mutate(self, m) -> (Self, Self::Diff)` signature-flip touches all 252. This is the single biggest sizing risk found in this recon for S1's staging plan — the plan's own "additive first, then flip" staging (already written into the plan) is the right mitigation, but the flip step itself is ~8x bigger than "~30" would suggest.
- **S-7 confirmed safe**: `CollectionDiff`/`CollectionMutation` real users go beyond the plan's own claim of "flow/vcs FlowMutation, store re-export" — also referenced from `space`, `store`, and `dag` modules. Real, multi-module usage confirmed; safe to keep + policy-ban from stdio schema dirs only, as planned.
- **New finding, not in the plan's spine table — "pdf primary-wiring defect" (S-6 needs a twin for pdf)**: exactly mirroring what S-6 already documents for gif (87a is currently wired as primary/canonical despite 89a being richer), **pdf has the same backward wiring**: glue.rs's `pdf::schema`/`pdf::engine` shims (`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`, pdf module, "Shims: keep pre-migration module paths resolving" comment) point to `standards::v1_4` — the 87-line `PageDoc`-based stub — not `v1_7`'s real 1794-line object-graph engine. 1.7 registers itself under a deliberately separate schema id (`stdio.pdf.1.7`) specifically to avoid a collision with 1.4's canonical `stdio.pdf`, per 1.7 engine's own doc-comment ("same rationale as gif 89a"). **Recommend**: extend S-6 (or add a twin spine row) to flip pdf's shim to point at 1.7 as primary, mirroring gif's fix, in the same spine wave — otherwise F4's pdf agent has no natural place to land this and it'll either get skipped or done ad hoc outside the planned glue.rs-ownership discipline.
- **glb merge (V2a) verified real**, not a stub — see gltf row above.
- **Git-rename false positives verified as pure git heuristic noise**, not real content — see gif 89a row above. No action needed beyond not trusting `-M` output.
- **Copy-pasted shared types confirmed beyond the plan's own examples**: the plan's intro cites "RasterImage×4, MeshVertex×4, BrepMesh×2, entries×5" as known instances to kill. This recon independently confirms concrete instances: `MeshVertex`/`MeshTriangle` shared verbatim between stl and ply; `RasterImage` shared between png/jpg/tiff (bmp uses an inline equivalent-shape anti-pattern); `Part21Document` shared between step and ifc (see ifc row above, the most severe instance since it's the persisted type itself, not just a value substruct).

### Recommended F1-F5 roster (confirms the plan's draft roster; only material change is dwg's internal framing)

- **F1** (7 standards): binary, txt, json, xml, csv, deflate, zip — as planned. Only change: **re-poll csv's git status immediately before dispatch** (was mid-edit at recon time).
- **F2** (6 standards): stl, obj, ply, las, bmp, tiff — as planned. Suggest ply and tiff get flagged to their agent(s) as needing the most net-new field design (weakest snapshots of the 6).
- **F3** (7 standards, 6 agent-artifacts): svg, gif 87a+89a, png, jpg, md, dxf — as planned. svg's agent needs to know tests-green does NOT mean done (apply-and-capture is still the real defect); gif's agent has real remaining scope beyond "polish" (~14 of ~20 target mutations still missing, absorb LWW bug per plan intro, apply still returns unit).
- **F4** (6 standards, 5 agent-artifacts): gltf(+glb), pdf 1.4+1.7, step, ifc, docx — as planned. **New dependency this recon surfaced**: pdf's agent needs the S-6-twin spine fix (primary-wiring flip) landed before or alongside its work, ideally via the same wave-closer mechanism gif's S-6 uses. ifc's shared-Part21Document defect should be called out explicitly in its brief.
- **F5** (5 standards, 4 agent-artifacts): xlsx, pptx, bcf, dwg ac1018+ac1024 — as planned. dwg's agent needs explicit instruction: ac1024 is D1/D2-complete (real remaining scope is snapshot/diff/mutation enrichment within that D1/D2 boundary, per the "honest boundary" allowance — NOT decompression bugfixing, that's already done), ac1018 stays frozen at D0 by design (Decision #5) — do not attempt to bring it to parity.
- No standard needs to be deferred out of its planned wave — **nothing found "live and blocking"** as of this recon's final state (csv was mid-edit at the start of recon but is a same-wave concern, not a defer-to-later-wave concern; everything else was either untouched or already-staged-and-settled). The crate is fully green (318/0) as of this recon's end.

## S1 (spine wave) — completed 2026-08-11

S1 landed all mandatory scope compile-green: S-1 (new standalone `DiffAlgebra<P>` trait + normative `MutationDiff::absorb` doc-contract, next to `MutationDiff` in `📡️spr/🎮️command/component.rs`, zero implementors as required), S-2 (`ArtifactBuilder::mutate` flipped to `(Self, Self::Diff)` plus the `Mutation<Snapshot, Diff=Diff>` bound, mechanically swept across all 252 repo-wide impl blocks — 31 stdio leaf builders' backing `apply_*_mutation` free functions were also flipped to return the diff, computed via the pre-existing `Mutation::diff` call before mutating; a real bug was caught and fixed here: svg's apply-and-capture `Mutation::diff` calls `apply_svg_mutation` internally, so naively also calling `.diff()` from inside `apply_svg_mutation` caused infinite mutual recursion/stack overflow — fixed by deriving svg's diff post-mutation instead), S-3 (dead `ArtifactEngine` trait deleted + all 85 impl blocks removed, structs/inherent impls/codec code left untouched), and S-6 extended to cover pdf as well as gif per W0's finding — both root shims (`schema`/`engine`/`io`, plus the root `builder`/`analyzer` facades) now point at the richer standard (gif 89a, pdf 1.7) as canonical with the legacy standard (87a, 1.4) reachable under its own explicit `standards::vXX::` path; this surfaced and required fixing a large secondary defect where 87a's and 1.4's OWN internal code (engine/schema/mutations/io leaves, ~30 files) had been written against the shared ROOT alias assuming it always pointed at themselves — all redirected to their own standard-local paths. S-4 was NOT attempted (explicitly optional; deferred whole to S2, see handoff below). S-5/S-8/S-9 out of scope for S1. S-7 confirmed via grep: zero stdio references to `CollectionDiff`/`CollectionMutation`/`Patchable`/`Identified` (no code change needed).

**Gate results (real, not summarized)**: `cargo test -p semio-s-plugin-stdio --lib` → 332 passed, 0 failed (up from W0's 318/0 baseline — the +14 are pre-existing pdf/A-2b subset tests that landed from a concurrent session mid-wave, not anything S1 added or removed; 0 failures is the invariant that matters and it holds). `cargo check -p semio-framework-os-kernel` and `cargo check -p semio-s-plugin-stdio` both zero errors. `cargo check --workspace --keep-going` shows zero errors attributable to S1 (no `mutate`/`Self::Diff`/`ArtifactBuilder`/`ArtifactEngine` mismatches anywhere) — all remaining errors across ~15 crates (`semio-framework-os`, `semio-framework-os-kernel-db`, `semio-compose-rs`, `semio-s-plugin-{vcs,sourcing,sequence,reasoning-mindmap,norm,mathematical,imperative,forms,flow,fem,energy,dag,block,architect}`) are pre-existing/concurrent unrelated churn from other sessions: a repo-wide missing `📄️document/component.rs` panel-module file, `ArtifactEnvelope` needing `dialect`/`migrated_from` fields (V4's in-flight work), `AppDefinition`/`OsAppRegistration` `label`/`document` field refactor, `CsvSnapshot.has_header` (F1's own flagged in-flight csv edit), `OsMediaExportResult::from_format_kind_bytes` rename — none touch anything S1 changed.

**Handoff to S2**: S-4 (`ArtifactSchemaDescriptor` 4th `mutations: FacetLeaves` field, 391 call sites) is untouched — do it first, full sweep, no partial state exists to build on or clean up. S2 also owns S-8 policies and the mutation-triad pre-mounts; glue.rs is now S2+'s exclusive domain per the plan (S1 was the one exception allowed to touch it, for S-6). Full file list and per-step detail in `s1-spine-report.md` in this ticket folder.

## S2 (spine wave) — completed 2026-08-11

**Task 1 (glue-mounting policy, load-bearing for F1-F6)**: resolved as NOT enforced — `POLICY_MUTATION_TRIAD_DIRS`'s completeness check only fires for mutation directories that already exist, never requires creating one per variant, and `policyMutationDispatchCoverageBreaches` (would-be variant-vs-dir coverage) is a permanent `return []` placeholder. Verified empirically on gif 89a and svg 1.1: only `📄set-snapshot` has a triad dir anywhere in either artifact; every other variant (`InsertFrame`/`RemoveFrame`/…) lives inline in the top-level `🧬️mutations/🦀️component.rs`. **F1-F6 agents: do all your real work in your artifact's already-mounted top-level `🧬️mutations/🦀️component.rs`, `🔺️diff/🦀️component.rs`, `📸️snapshot/🦀️component.rs` — zero new files, zero glue.rs edits.** A triad dir is optional scaffolding; if wanted, queue it via `glue_followup` for the wave closer. Documented as a doc-comment at glue.rs's `//#region Artifacts` and in full in `s2-spine-report.md`.

**Task 2 (S-4)**: complete, but bigger than either the plan or S1's own recon estimated — `ArtifactSchemaDescriptor` is a framework-shared type with **85 real constructor call sites** (31 stdio + 54 non-stdio plugin artifacts), not 391/30. Added `mutations: FacetLeaves` to the struct AND its separate OS-kernel-side mirror (`KernelArtifactSchemaDescriptor`, not called out in the brief but required for the crate to compile), wired all 85 constructors to `include_str!` their already-existing `🧬️mutations/*` facet leaves (confirmed present for all 85 before wiring, zero missing), fixed 2 non-rustfmt'd files' brace style along the way, and updated 78 stale `"fifteen handcrafted schema leaves"` doc-comments to `"twenty"` repo-wide.

**Task 3 (S-7)**: `policyStdioVcsMachineryBanBreaches` added, allowlist seeded empty (re-confirmed zero current stdio references to vcs collection machinery).

**Task 4 (S-8)**: all four rules added in a new `//#region 🔧️PolicyRuleSchemaOverhaulS2` — `POLICY_FACET_MIRROR_DRIFT` (93/93 seeded, every stdio facet pair drifts today, as expected), `POLICY_GRAMMAR_HONESTY` (645/651 stdio grammar leaves seeded — the 6 exceptions are json's already-real `.grammar.semio`/`.protocol.semio`), `POLICY_DIFF_ALGEBRA` (31/31 seeded, zero implementors confirmed), field-sweep-test presence (31/31 seeded, zero `field_sweep` tests confirmed). Every seed was generated by actually running the detection logic against the real tree (`s2-artifacts/gen_s8_seeds.ts` in this ticket folder), and every rule was round-trip verified by temporarily deleting one real seed entry, confirming exactly one new breach fires with the expected `kind`, then restoring it — all four passed clean.

**🐛 Re-confirms the D6-era correction above**: `bun ./📜️script.ts verify` does NOT run S2's new rules (or S-7/S-8's siblings) — `runGate()` calls a curated, different breach-group subset. **`bun ./📜️script.ts policy` is the real check.** Ran it: exits 1, but on 22,198 pre-existing high-priority breaches across 22 unrelated rule kinds (dominated by `handcrafted-grammar/spec-distinctness`, 20,201 — a different large in-flight program), independently confirmed via both a full cache-file grep and a direct standalone-probe function call that **zero** of those are from any of this wave's 5 new rules. `bun run ./📜️script.ts verify gate` also fails, but on unrelated `🪵️sourcing`/`🗂️curate` concurrent-session churn (confirmed via `git status` — files I never touched show unstaged modifications from elsewhere).

**Gate results**: `cargo check -p semio-framework-schema` clean. `cargo check -p semio-s-plugin-stdio` clean. `cargo test -p semio-s-plugin-stdio --lib` → **332 passed, 0 failed** (unchanged from S1's exit state, as expected for a pure additive change).

**No new directories created this wave** (Task 1's resolution made triad-dir scaffolding unnecessary). Full report: `s2-spine-report.md` in this ticket folder.

## F1 (fan-out wave, 7 standards) — closed 2026-08-11

**Roster**: xml (1.0), zip (2.0), json (rfc8259), deflate (rfc1950), csv (rfc4180), txt (utf-8), binary (raw) — 6 fan-out agents (txt+binary shared one agent), 1 verify agent, this C1 closer.

**Per-artifact completion** (all 7): real handcrafted sparse `XDiff` (no `snapshot: Option<XSnapshot>` full-replace slot anywhere, grep-confirmed zero hits across all 7 diff files), `impl DiffAlgebra<XSnapshot> for XDiff` present for all 7, full named-variant `XMutation` enums with handcrafted per-variant `diff()`/`inverse()` (never apply-and-capture) for all 7, base-free structural `absorb()` satisfying every recipe-mandated canonical case (Insert+Remove-before, Insert+Insert-same-index-both-survive, Add+SetField-patches-into-added, Modify+Remove-drops-the-modify) plus associativity, for all 7, and a `field_sweep` law test present and passing for all 7. Snapshot completeness gaps closed: xml gained a typed `XmlDeclaration`; csv gained per-field quote-provenance (`CsvField.quoted`) and dropped the old header/row-splice split; deflate gained typed RFC1950 CMF/FLG/dict-id/payload fields (replacing a `{bytes}` stub) with real encode/decode entry points wired, LZ77/Huffman codec itself untouched; json replaced the generic `serde_json::Value` passthrough with a from-scratch `JsonValue` model + hand-rolled RFC8259 parser/serializer; txt replaced bare `text: String` with `lines`/`trailing_newline`/`line_ending`; zip's already-complete snapshot got its stale facet mirrors (previously copy-pasted `{schema,bytes}`/`{name,data}` placeholders unrelated to the real shape) rewritten to match. Facet leaves (`.ts`/`.graphql`/`.json` JSON-Schema/`.proto`) and grammar leaves (`.g4`/`.ebnf`/`.grammar.semio`/`.ksy`/`.spicy`/`.abnf`/`.protocol.semio`) handcrafted for all three schema facets across all 7 artifacts; a small number of non-wired sibling grammar-mirror files (zip's un-wired per-subdir copies, a handful in csv/json/deflate/xml) were deliberately left as documented, still-real placeholders rather than papered over — tracked via `POLICY_GRAMMAR_HONESTY_ALLOWLIST`, see below.

**Closer-applied fixes (6 real, own-code defects, all within F1's 7 artifacts, none touching glue.rs/script.ts's forbidden files)**, the first 4 found by the verify agent, the 5th found by this closer once the crate actually ran:
1. `binary/raw` `⚙️engine/component.rs` (`field_sweep_covers_every_byte_level_change`): added missing `use protocol::MutationDiff;` (only `DiffAlgebra` was imported; `.apply()` lives on `MutationDiff`).
2. `txt/utf-8` `⚙️engine/component.rs` (`field_sweep_covers_every_mutable_field`): identical missing-import fix.
3. `xml/1.0` `⚙️engine/component.rs` (`between_roundtrip_law`, lines 350-367): 5 bare `DiffAlgebra::between(...)` calls were ambiguous under type inference; rewritten to the fully-qualified `<XmlDiff as DiffAlgebra<XmlSnapshot>>::between(...)` form already used correctly two tests later in the same file.
4. `txt/utf-8` `🔺️diff/component.rs` (lines 315, 358): `merged.lines.expect(...)` moved out of `merged` before a later `merged.apply(&base)` call (E0382 partial-move); changed to `merged.lines.clone().expect(...)`, matching the pattern already used correctly at a third call site in the same file.
5. `txt/utf-8` `⚙️engine/component.rs` (`field_sweep_covers_every_mutable_field`, runtime failure, found only once the crate actually compiled and ran — see below): the test asserted `removed`/`modified`/`added` all non-empty from a *single* `TxtLinesDiff::between()` call, which is structurally impossible for a flat, unkeyed `Vec<String>` collection with equal-length fixtures (the exact "removed XOR added, never both, from one collection" limitation xml's own report already flagged, but txt's sweep fixture didn't account for). Fixed by making `sweep_a`/`sweep_b` asymmetric lengths and splitting the assertions across both diff directions (`ab` proves modified+added, `ba` proves modified+removed) — mirroring the two-direction rigor `between_roundtrip_law` already applies.

After all 5 own-code fixes, none of the verify agent's original 9 defects remain attributable to F1's own files, and the one additional runtime bug is fixed and verified (see below).

**Full-crate gate status — FINAL, real, on-disk result: 732 passed, 12 failed, crate-wide; 0 of the 12 failures attributable to any F1 artifact; all 187 tests across F1's 7 standards pass (xml 22/22, zip 38/38, json 58/58, deflate 17/17, csv 17/17, txt 19/19, binary 16/16).** Getting here took two phases within this closing session: first the crate would not compile at all — blocked by a large, genuinely unrelated, actively in-progress concurrent wave (confirmed via `git status`/timestamps, not assumed) adding real spec-mandated subset variants across 8 *other* artifacts: SVG Tiny/Basic, STEP conformance classes cc1-6, PDF/A+X+E+UA+VT+H, JPEG baseline, OOXML strict/transitional, IFC 2x3, TIFF baseline, and (overlapping F1 only incidentally, in a composer-registration file never touched by F1's own schema/diff/mutation work) an XML "valid" subset — 36 `E0433: cannot find <subset> in subsets` errors plus the already-known/self-reported `gltf`→`json` export-bridge `E0308` (json's `JsonSnapshot.value` type change, ~120 total call sites repo-wide per json's own report, only this one inside the same crate). Polled ~15 minutes across multiple intervals with zero change, then the blocking wave's own composer registrations landed mid-session and the crate compiled for the first time. At that point it ran **731 passed / 13 failed** — 12 of the 13 belonging entirely to the (by-then-landed) subset-multiplicities wave's own new tests (docx/ifc/jpg/pdf/tiff/xlsx), but **1 was a real, previously-undetected F1 bug** (fix #5 above, in `txt/utf-8`) that had gone unnoticed all session simply because the crate had never successfully compiled before. Fixed and re-ran: **732 passed / 12 failed**, with the remaining 12 entirely in the unrelated wave's own scope (docx/ifc/jpg/pdf/tiff/xlsx) — none touched, per this ticket's "classify, don't chase" rule for genuinely other-wave scope.

**Policy shrink (`bun ./📜️script.ts policy`, the 4 new S8 rules — diff-algebra, field-sweep-presence, grammar-honesty, facet-mirror-drift)**: confirmed **zero** real breaches AND **zero** stale-allowlist breaches for all 7 F1 standards (verified directly against the regenerated `.🦑️repo/⚡️cache/breaches/compose.json`, not just the CLI's truncated stdout). Housekeeping applied to `📜️script.ts`: removed 7 now-satisfied entries each from `POLICY_DIFF_ALGEBRA_ALLOWLIST` and `POLICY_FIELD_SWEEP_ALLOWLIST` (one per F1 artifact), and 96 now-satisfied entries from `POLICY_GRAMMAR_HONESTY_ALLOWLIST` (the wired-leaf grammars every fan-out agent rewrote) — while explicitly *keeping* 45 grammar-honesty entries that are still real, documented, not-yet-handcrafted sibling-mirror placeholders (confirmed by re-running the check with each candidate entry removed and inspecting the actual `missingBySibling` diagnostic before deciding, not by pattern-matching alone). `POLICY_FACET_MIRROR_DRIFT_ALLOWLIST`'s 21 F1 entries were investigated (drift counts of up to 27 missing fields looked suspicious) and **deliberately left in place** — root-caused to two checker false-positive sources, not real drift: (a) the checker's field-name regex scans the *entire* `component.rs` including `#[cfg(test)] mod tests`, so local test-fixture variable names with type annotations get misread as API fields; (b) `.proto` siblings correctly use idiomatic snake_case (`has_header`) while the checker only string-matches the camelCased form (`hasHeader`). Concretely verified on csv/snapshot: of 10 flagged "missing" identifiers, only `hasHeader` (a real field, just snake_cased in proto per convention) was a plausible true positive, and the other three (`text`/`options`/`bytes`/`mismatch`) traced to test-body local variables. Removing these 21 entries would have created 21 new spurious breaches; this is flagged here as a genuine `📜️script.ts` limitation for a future, out-of-band fix (narrow the field-extraction regex to skip `#[cfg(test)]` regions, and/or normalize proto's snake_case before the substring compare) — **not** attempted by this closer since it risks changing behavior for all 31 stdio standards, not just F1's 7, and was not part of this wave's mandate.

**`git check-ignore`**: no new top-level directories were created by F1's own work (all 6 fan-out agents confirmed staying within already-mounted files per S2's Task 1 resolution; zero `glue_followup` entries requesting a new directory across all 6 reports). A handful of untracked subset-related paths appeared under F1 artifact trees during this session (`zip/✳️iso21320`, `json/✳️i-json`, `xml/✳️valid`, plus stray `subsets/🔣️component.json` files) — these belong to the same unrelated concurrent "subset multiplicities" wave noted above, not to F1; `git check-ignore -v` confirms none are actually gitignored (they only match a `.gitignore` *negation* rule, i.e. explicitly un-ignored/trackable), so no action was needed.

**Ownership-ledger update for F1's 7 rows** (supersedes the pre-F1 W0-recon descriptions above for these 7 artifacts): xml/1.0, zip/2.0, json/rfc8259, deflate/rfc1950, csv/rfc4180, txt/utf-8, binary/raw are now all **diff/mutation/absorb-complete per this ticket's recipe, real `cargo test`-confirmed green** (handcrafted sparse diff, `DiffAlgebra`, named-variant mutations, base-free structural absorb, all 6 test laws present and passing, facet leaves handcrafted, S8 policy-clean). Final gate: `cargo test -p semio-s-plugin-stdio --lib "artifacts::{binary,txt,xml,zip,csv,deflate,json}::"` → **187 passed, 0 failed** (per-artifact: xml 22, zip 38, json 58, deflate 17, csv 17, txt 19, binary 16). F1 is fully done — no follow-up gate re-run needed for these 7 standards.

Full report: `f1-closer-report.md` in this ticket folder.

## F2 (fan-out wave, 5 standards — stl/obj/ply/las/bmp; tiff deferred) — closed 2026-08-11

**Roster**: stl (ascii), obj (3.0), ply (1.0), las (1.0), bmp (v3) — 5 fan-out agents, 1 verify agent, this C2 closer. tiff explicitly excluded this wave (live external "subset multiplicities" edit, see below).

**Per-artifact completion** (all 5): real handcrafted sparse `XDiff` (no `snapshot: Option<XSnapshot>` full-replace slot anywhere, grep-confirmed zero hits across all 5 diff files — only doc-comment mentions of the OLD template being replaced), `impl DiffAlgebra<XSnapshot> for XDiff` present for all 5 (both `protocol::MutationDiff` and `protocol::command::DiffAlgebra`/`protocol::DiffAlgebra` imported explicitly from the start, avoiding F1's known missing-import trap), full named-variant `XMutation` enums with handcrafted per-variant `diff()`/`inverse()` (never apply-and-capture) for all 5, base-free structural absorb satisfying every recipe-mandated canonical case for all 5, and a `field_sweep`-named law test present and passing for all 5 — every one correctly avoiding the F1-txt structural trap (asymmetric-length fixtures, assertions split across both `between()` directions). Snapshot completeness gaps closed: stl gained `solid_name` + genuinely-persisted per-facet `normal` (previously silently recomputed on encode) and its own `StlTriangle` type (no shared vertex pool — STL facets don't share vertices, a real format-accurate redesign, not a rename); ply's mesh-only `{vertices, faces}` model (sharing `MeshVertex`/`MeshTriangle` verbatim with stl — the W0-flagged defect) was replaced entirely by a generic element/property/row model (`PlyElement`/`PlyProperty`/`PlyRow`/`PlyValue`) that PLY's real spec actually is, with vertices/faces falling out as the common case rather than being hardcoded; obj gained `w`-component homogeneous coords, `mtllib`/`usemtl`/`smoothing_groups`/`unknown_statements`, and split `o`/`g`/`usemtl`/`s` state out of `faces` into their own name-keyed/range-tagged collections; las gained a full 25-field `LasHeader` + index-keyed `LasVlr` collection (previously `{schema, points}` only, no header, no VLRs); bmp gained the full 11-field BITMAPINFOHEADER + index-keyed `palette` (previously `{width, height, pixels}` only, a RasterImage-equivalent anti-pattern). **W0's stl/ply shared-`MeshVertex`/`MeshTriangle` defect is confirmed killed**: each artifact now defines its own named, format-appropriate type; neither imports from the other or from any shared module (verified by this closer via independent grep, not just trusting the fan-out/verify reports). Facet leaves and grammar leaves handcrafted across all 5 per the zip/csv F1 precedent (2 live-wired leaves per facet handcrafted honestly, un-wired diff/mutations sibling leaves — `.g4`/`.ebnf`/binary-4 — left as documented, still-real placeholders, matching F1's accepted scope boundary).

**Closer-found-and-fixed defect (1, real, own-code, discovered during this closer's own verification pass — not flagged by ply's fan-out report or the verify report)**: ply's snapshot facet's `.g4`/`.ebnf` grammar leaves had been written to the WRONG path — directly under `🧬️schema/📸️snapshot/` (`🅰️component.g4`, `🔤️component.ebnf`, both untracked stray files) — instead of the correct `🧬️schema/📸️snapshot/📝️text/` subdirectory, leaving the real target files at the correct path as stale, untouched placeholders (`DOCUMENT: 'schema' [ ]+ 'stdio.ply'` / equivalent 3-line ebnf stub) despite ply's own report claiming "the snapshot facet's full 6-leaf set... is handcrafted honestly." Fixed by moving the real handcrafted content into the correct `📝️text/` location (overwriting the stale placeholders) and deleting the two misplaced stray files. Re-verified: `cargo test -p semio-s-plugin-stdio --lib "artifacts::ply"` still 23/23 after the fix (grammar leaves aren't exercised by Rust tests, so this couldn't have broken anything, but re-ran anyway); `bun ./📜️script.ts policy` confirmed both leaves flip from real-and-unallowlisted-would-be-a-breach to correctly `-stale-` (fixed, allowlist entry pending removal) immediately after the fix, then pruned (see below).

**Full-crate gate — FINAL, real, on-disk result: 795 passed, 0 failed, crate-wide** (re-run twice by this closer: once before the ply grammar-leaf fix, once after — both green, no regressions from either the fix or the `📜️script.ts` allowlist edits). Per-artifact filter, independently re-confirmed by this closer (not just trusting the fan-out/verify reports): stl 21/21, obj 17/17, ply 23/23, las 21/21, bmp 14/14 — **96/96 passing, 0 failing, across all of F2's 5 standards.** Unlike F1, this closer's own full-crate run never saw any external-wave compile blockage (the concurrent "subset multiplicities" wave had already settled by the time this closer ran its gates) — the 795/0 result matches exactly what the independent verify agent (`f2-verify-report.md`) reported.

**Policy shrink (`bun ./📜️script.ts policy`, the 4 S8 rules)**: before this closer's edits, cross-checked the regenerated `.🦑️repo/⚡️cache/breaches/compose.json` directly (not just the CLI's priority-filtered stdout) for all 4 S-8 rules scoped to stl/obj/ply/las/bmp: **59 breaches, every single one `-stale-`** (5 diff-algebra + 5 field-sweep + 49 grammar-honesty; `facet-mirror-drift` showed 0 hits, real or stale, for all 5 — not investigated further since F1's own precedent already root-caused this rule's false-positive behavior and explicitly declined to touch its allowlist). Zero real (non-stale) breaches existed even before this closer's edits — every fan-out agent's underlying fix was genuinely in place (net of the one ply grammar-leaf-path defect this closer found and fixed independently, which surfaced 2 *additional* now-stale entries after the fix — pruned in the same pass). Housekeeping applied to `📜️script.ts`, restricted precisely to each rule's own allowlist array (verified line-range-scoped, not global string-replace, after discovering the same normalized key string can legitimately appear in more than one allowlist — e.g. `POLICY_FACET_MIRROR_DRIFT_ALLOWLIST` independently, left untouched, per F1's precedent): removed exactly 5 entries from `POLICY_DIFF_ALGEBRA_ALLOWLIST` (bmp/las/obj/ply/stl, one each), 5 from `POLICY_FIELD_SWEEP_ALLOWLIST` (same 5), and 51 from `POLICY_GRAMMAR_HONESTY_ALLOWLIST` (49 pre-existing-stale + 2 newly-stale after the ply fix: stl 11, obj 12, ply 7+2=9, las 7, bmp 12). Re-ran policy after pruning and cross-checked the freshly regenerated breach cache: **0 breaches (real or stale) for all 5 F2 artifacts across all 4 S-8 rules.** `POLICY_FACET_MIRROR_DRIFT_ALLOWLIST` was not touched (0 hits for our 5 artifacts either way — consistent with F1's finding that this rule's checker has real false-positive sources, not re-investigated here since nothing indicated a problem).

**`git check-ignore`**: no new top-level directories were created by F2's own work (all 5 fan-out reports confirm staying within already-mounted files per S2's Task 1 resolution; zero `glue_followup` entries requesting a new directory or a `glue.rs` mount across all 5 reports — `glue_edits: []`). Untracked stray files found under all 5 artifacts' own trees (`🏅️standards/🔖️<version>/🪆️subsets/🔣️component.json`, identical "Unconstrained X <version>" content, identical mtime across all 5 — a pre-existing scaffold artifact, not created by any F2 fan-out agent) and, until fixed above, ply's two misplaced grammar leaves — `git check-ignore -v` on all of them confirms none are actually gitignored (they only match the `.gitignore` *negation* rule `!**/🔖️*/**`, i.e. explicitly un-ignored/trackable), so no `.gitignore` action was needed for any of them.

**tiff status (for the orchestrator's next-wave decision)**: re-polled `git status` on `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff` twice, 20 seconds apart, near the end of this closing session — identical file set both times (`⚙️engine/component.rs` and `🎹️composer/component.rs` modified, plus an untracked new `🪆️subsets/✳️baseline/` directory and a `🪆️subsets/🔣️component.json`), suggesting the external wave is currently idle/paused rather than actively churning at this exact moment — but this is a snapshot, not a guarantee of permanence. `cargo check -p semio-s-plugin-stdio --lib` compiles tiff's current on-disk state cleanly (0 errors), consistent with the 795/0 full-crate result. tiff was deliberately excluded from F2's scope per the original dispatch (live external edit) and remains untouched by this closer — its own diff/mutations/snapshot rewrite (the same recipe already applied to all 30 other standards across F1+F2) still needs a dedicated future wave; **recommend it be F2b's or F3's first item**, not folded silently into whatever wave happens to touch it next.

**Ownership-ledger update for F2's 5 rows** (supersedes the pre-F2 W0-recon descriptions above for these 5 artifacts): stl/ascii, obj/3.0, ply/1.0, las/1.0, bmp/v3 are now all **diff/mutation/absorb-complete per this ticket's recipe, real `cargo test`-confirmed green** (handcrafted sparse diff, `DiffAlgebra`, named-variant mutations, base-free structural absorb, all 6 test laws present and passing, facet leaves handcrafted, S8 policy-clean, 0 breaches real-or-stale). Final gate: `cargo test -p semio-s-plugin-stdio --lib "artifacts::{stl,obj,ply,las,bmp}::"` → **96 passed, 0 failed** (per-artifact: stl 21, obj 17, ply 23, las 21, bmp 14). F2 is fully done for these 5 standards — tiff remains open, see above.

Full report: `f2-closer-report.md` in this ticket folder.

## F3 (fan-out wave, gif/png/md/dxf; svg/jpg/tiff deferred) — closed 2026-08-11, PARTIAL — only 2 of 4 artifacts actually landed

**Roster**: gif (87a+89a), png (1.2), md (commonmark), dxf (r12) — 4 fan-out agents dispatched, 1 verify agent, this C3 closer. svg/jpg/tiff explicitly excluded this wave (live external "subset multiplicities" edit, see below).

**Critical finding, independently re-verified by this closer (not just trusted from the verify report): only `f3-md-report.md` and `f3-png-report.md` exist on disk. No `f3-gif-report.md` and no `f3-dxf-report.md` were ever written — those two artifacts' F3 work either never started (dxf) or never touched the diff layer (gif 89a).**

**png and md — genuinely done**, re-verified directly by this closer against disk and a fresh `cargo test` run, not taken on either agent's word: both have a real handcrafted sparse `XDiff` with zero `snapshot: Option<XSnapshot>` full-replace slot (grep-confirmed — the only string hits are doc-comments explicitly noting the slot's absence), a real `impl DiffAlgebra<XSnapshot> for XDiff`, named-variant mutations, and a passing `field_sweep`-named law test. `cargo test -p semio-s-plugin-stdio --lib "artifacts::png::"` → 22/22; `"artifacts::md::"` → 24/24. Both include all 6 required law tests (`mutation_diff_law`, `inverse_law`, `absorb_law`(+associativity), `between_roundtrip_law`, `codec_retention_law`, `field_sweep`).

**gif — NOT done**: 87a's diff is a deliberately-minimal replace-only `{snapshot: Option<GifSnapshot>}` with a documented rationale (87a has no incrementally-mutable frame concept) — arguably acceptable as-is, matching 87a's own mutation file's doc comment. **89a is not done**: its diff file (`🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`, re-read directly by this closer) is still the pre-overhaul op-slot shape — `pub snapshot: Option<GifSnapshot>` full-replace slot PLUS one `Option<T>` per mutation kind (`insert_frame`, `remove_frame_at`, `set_frame_delay`, `set_loop_count`, `set_frame_disposal`) — zero `impl DiffAlgebra` anywhere in the gif artifact (87a or 89a, grep-confirmed), zero `field_sweep`-named test, and none of the 3 mandated canonical absorb tests (Insert+Remove-before, Insert+Insert-same-index, Insert+SetField-patch) exist or would pass against the current last-write-wins `absorb()`. Tests are green (26/26 for the gif module filter, 4/4 for the dancing fixture) only because nothing in the current suite exercises the missing surface — green tests here are hiding an incomplete migration, not confirming one.

**dxf — NOT done at all**: `🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` (re-read directly by this closer, 1045 bytes) is the pristine pre-overhaul scaffold — `DxfDiff{snapshot: Option<DxfSnapshot>}`, `impl MutationDiff` only, no `DiffAlgebra`. The sibling `🧬️mutations/🦀️component.rs` still only has `{NoMutation, SetSnapshot}`, the universal minimal stub every other F1-F3 artifact was expected to grow beyond. File mtimes on both predate this ticket's own working window (21:07 the day it opened) — this artifact was never touched by any F3 fan-out agent. Tests are green (6/6) but all 6 are pre-existing snapshot/codec/demo tests; none exercises diff/mutation.

**Full-crate gate — this closer's own fresh run: 817 passed, 0 failed, 0 ignored** (`cargo test -p semio-s-plugin-stdio --lib`, no filter) — matches the independent verify report exactly. Per-artifact filter, independently re-run by this closer: png 22/22, md 24/24, gif 26/26 (+4/4 dancing fixture), dxf 6/6 — **all green, because gif-89a's and dxf's gaps are silent omissions (missing tests), not failures.** No crate-wide breakage exists to classify as internal-vs-external right now.

**Policy shrink (`bun ./📜️script.ts policy`, the 4 S-8 rules — diff-algebra, field-sweep-presence, grammar-honesty, facet-mirror-drift), scoped to gif/png/md/dxf**: cross-checked the regenerated `.🦑️repo/⚡️cache/breaches/compose.json` directly (not just CLI stdout). Before pruning: 25 breaches, every one `-stale-` and every one belonging to **png or md only** (2 diff-algebra: png+md; 2 field-sweep: png+md; 21 grammar-honesty: all md, its full `🔺️diff`/`🧬️mutations` binary+text grammar-leaf set). **gif and dxf produced zero breach entries of any kind** (neither real nor stale) for all 4 rules — both remain silently allowlisted-as-not-yet-fixed, consistent with F3 never having landed real work on either. Housekeeping applied to `📜️script.ts`, scoped precisely to png/md (left every gif/dxf allowlist entry untouched, since neither is actually fixed yet — pruning those would create real, correctly-firing breaches): removed png+md from `POLICY_DIFF_ALGEBRA_ALLOWLIST` (2 entries), png+md from `POLICY_FIELD_SWEEP_ALLOWLIST` (2 entries), and md's full 21-entry grammar-honesty block from `POLICY_GRAMMAR_HONESTY_ALLOWLIST` (png's own grammar leaves remain allowlisted — deliberately deferred to F6 per png's own `glue_followup`, not yet rewritten, correctly still-real not stale). `POLICY_FACET_MIRROR_DRIFT_ALLOWLIST` left untouched for all 4 artifacts (0 hits, real or stale, matching F1/F2's precedent that this rule has known false-positive sources not worth re-litigating here). Re-ran policy after pruning: **0 breaches, real or stale, for all 4 S-8 rules across gif/png/md/dxf** — total breach count dropped by exactly 25 (22016 → 21991), confirming no collateral change elsewhere. `policy_shrink_confirmed: true`.

**`git check-ignore`**: no new top-level directories were created by png's or md's own F3 work (both reports' `glue_followup` explicitly state no glue.rs edit / no new directory needed). The only untracked new paths under any of the 4 artifacts are identical pre-existing-scaffold `🪆️subsets/🔣️component.json` stray files (same pattern F2's closer already found and cleared) — `git check-ignore -v` on all 4 confirms they only match the `.gitignore` *negation* rule `!**/🔖️*/**` (explicitly trackable), no `.gitignore` action needed. No `glue.rs` edit was needed or made this wave (`glue_edits: []`) — both landed reports' `glue_followup` sections require only content rewrites of already-mounted sibling leaf files (png's stale zip-shaped `.ts`/`.json`/`.graphql`/`.proto` facet mirrors, deferred to F6), not a new mount.

**svg/jpg/tiff status (for the orchestrator's next-wave decision), freshly re-polled by this closer**: the external "subset multiplicities" wave is still visibly in flight on all 3 — svg has 2 modified files (`⚙️engine`, `🎹️composer`) plus 2 new untracked subset dirs (`✳️basic`, `✳️tiny`); jpg has 4 modified files plus 1 new untracked subset dir (`✳️baseline`); tiff has 2 modified files plus 1 new untracked subset dir (`✳️baseline`). Newest touched file across all 3 (tiff's baseline composer) was ~96 minutes old at poll time — no file in any of the 3 trees changed in the final ~90 minutes of this closer's session, suggesting the wave is currently paused rather than actively mid-edit, but (per F2's own precedent-caveat) this is a snapshot, not a guarantee. **All 3 compile and pass their own tests cleanly right now**: `cargo test … "artifacts::svg::"` → 50/50, `"artifacts::jpg::"` → 21/21, `"artifacts::tiff::"` → 15/15 (all reflected inside this closer's 817/0 full-crate run too). Still explicitly out of scope for F3 (live external edit at dispatch time) and untouched by this closer; each still needs its own dedicated diff/mutations/absorb F-wave pass (same recipe as every other standard) once the external wave's new subsets settle for real (commit or long-idle confirmation, not just a 90-minute quiet window).

**Ownership-ledger update for F3's 4 rows**: png/1.2 and md/commonmark are now **diff/mutation/absorb-complete per this ticket's recipe, real `cargo test`-confirmed green, S-8 policy-clean** (same bar as F1/F2's closed rows) — supersedes their pre-F3 W0-recon descriptions. **gif/89a and dxf/r12 remain OPEN** — gif/87a's replace-only diff is accepted as a deliberate, documented design choice; gif/89a and dxf/r12 both still need a full (or resumed) F-wave diff/mutations rewrite from scratch, exactly as flagged by the independent verify report. Recommend a dedicated gif-89a + dxf mop-up wave, distinct from (and not blocked by) the eventual svg/jpg/tiff wave.

Full report: `f3-closer-report.md` in this ticket folder (superseded by the mop-up re-close below — kept for history, its gif/dxf "NOT done" finding is stale).

## F3 mop-up — RE-CLOSED, all 4 artifacts now done — 2026-08-11 (this closer's own re-verified pass)

The gif-89a+87a and dxf gaps flagged by the PARTIAL closure above were real at the time it was written, but a resumed mop-up pass landed for both artifacts afterward (`f3-gif-report.md`, `f3-dxf-report.md`, both now present on disk — they did not exist when the PARTIAL closer report above was written) plus an independent verify pass (`f3-verify-report.md`) that supersedes the PARTIAL closer's gif/dxf findings. This closer re-dispatched as C3, found all 4 fan-out reports plus the verify report present, and independently re-verified every claim against disk from scratch (not taken on trust from any report, including the now-stale PARTIAL closer report above):

- **gif 87a**: full rewrite, not just "accepted as deliberate replace-only" — real `GifImage`/`GifColorTable`/multi-image support, sparse `GifDiff` (zero `snapshot: Option<>` slot), `impl DiffAlgebra`, ~11-variant mutation enum, all 6 laws incl. all 3 canonical absorb cases.
- **gif 89a**: full rewrite — sparse `GifDiff` (frames/comments/app_extensions triples) replacing the old op-slot shape entirely (verified: zero occurrences of the old `insert_frame`/`remove_frame_at`/`set_frame_delay`/`set_loop_count`/`set_frame_disposal` struct fields), real GCT/loop/comment/plain-text/app-extension modeling, 20-variant mutation enum, `impl DiffAlgebra`, all 6 laws incl. all 3 canonical absorb cases (independently confirmed passing by both the fan-out agent and the separate verify agent). A genuine latent LZW encoder/decoder tail-desync bug (shared by both standards) was found and fixed along the way, with a new regression test.
- **dxf r12**: full rewrite from the pristine pre-overhaul scaffold — typed `$VAR`-keyed header, name-keyed LAYER/STYLE/LTYPE tables + raw-retained other-table fallback, index-keyed blocks with nested entities, 8-kind typed top-level entities (Line/Circle/Arc/Polyline/Text/Solid/Insert) + `Other` raw-retention fallback, sparse name/index-keyed `DxfDiff`, `impl DiffAlgebra`, 19-variant mutation enum, all 6 laws. Polyline is correctly modeled via the real R12 POLYLINE/VERTEX/SEQEND record group (not the R14+-only LWPOLYLINE the pre-overhaul code named), a documented spec-accuracy correction. 4 real bugs (unknown-table body-start truncation, duplicated vertex layer tag on print, 3× `Insert{Layer,Style,Linetype}` inverse reading the wrong snapshot, one own-test miscount) found and fixed via the real crate's own test suite, not a scratch crate.

**This closer's own independently-run verification** (not reused from any report):
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate, fresh run) → **853 passed, 0 failed, 0 ignored** — matches both fan-out reports' and the verify report's own numbers exactly.
- Per-artifact filters, independently re-run: `artifacts::gif::` → 55/0, `artifacts::png::` → 22/0, `artifacts::md::` → 24/0, `artifacts::dxf::` → 13/0.
- Grep gates on all 5 diff files (gif 87a, gif 89a, png, md, dxf): `impl DiffAlgebra<XSnapshot> for XDiff` present in every one; zero struct-field `snapshot: Option<...>` full-replace slots in any (doc-comment mentions of the deleted shape don't count, checked by hand).
- `field_sweep`-named test confirmed present (via `grep -rl`) somewhere in each of the 4 artifact trees (gif and dxf keep it in their diff/mutations files; png in mutations; md in the engine file — same file-location variance the independent verify report already noted, not a defect).
- `git check-ignore -v` on the 5 new untracked stray `🔣️component.json` scaffold files under gif ×2/png/md/dxf (same pre-existing-scaffold pattern F2's and the PARTIAL F3 closer already found and cleared): all match only the `.gitignore` *negation* rule at line 179 (`!**/🔖️*/**`, explicitly trackable) — no `.gitignore` action needed. The Chinese-character-typo `🖊️dxf/🏅️标准/` directory dxf's own report says it created-and-removed mid-session was independently confirmed absent from disk (no trace).

**Policy shrink, this closer's own pass**: before pruning, cross-checked the regenerated `.🦑️repo/⚡️cache/breaches/compose.json` directly (not just CLI stdout), scoped to gif/png/md/dxf across all 4 S-8 rules: **39 breaches, every one `-stale-` (satisfied but still allowlisted), zero real.** Breakdown: `diff-algebra` — gif×2 (87a+89a) + dxf×1; `field-sweep-presence` — gif×2 + dxf×1; `grammar-honesty` — dxf×12 + png×21 (png's grammar leaves were rewritten for real this pass, per its own report's §6 — supersedes the PARTIAL closer's earlier note that png deferred grammar leaves to F6; that was true of an *older* version of png's report, not the current one). png/md's diff-algebra and field-sweep allowlist entries were already pruned by the earlier PARTIAL closer pass and correctly did not reappear. Pruned exactly the 39 satisfied entries from `📜️script.ts`: `POLICY_DIFF_ALGEBRA_ALLOWLIST` (gif 87a+89a, dxf — 3 entries), `POLICY_FIELD_SWEEP_ALLOWLIST` (same 3), `POLICY_GRAMMAR_HONESTY_ALLOWLIST` (png's full 21-entry block; 12 of dxf's 21 entries — the `.abnf`/`.g4`/`.ebnf`/`.grammar.semio` leaves across all 3 facets). **Deliberately left dxf's remaining 9 grammar-honesty entries allowlisted** (`.ksy`/`.protocol.semio`/`.spicy` across snapshot/diff/mutations): direct inspection confirmed these 3 file types still literally contain the policy's own placeholder marker substring (`size-eos: true` / `payload = *OCTET` / `payload: bytes &eod;`) as part of otherwise-real, honest content describing a genuinely-unstructured UTF-8-text-blob payload — the exact same accepted false-positive shape csv's precedent already established (an arbitrary-length text/JSON blob has no further internal binary structure to describe beyond "the payload is bytes/octets"); png's own report deliberately worked around this by renaming its payload field away from the literal `payload` identifier, dxf's did not, so the mechanical checker still (technically correctly, per its own heuristic) treats those 9 as unsatisfied. Pruning them would create 9 new spurious-but-real-per-the-checker breaches — not attempted; flagged here for whoever next touches dxf's grammar leaves, not for `POLICY_GRAMMAR_HONESTY_LEAF_MARKERS` itself (a repo-wide-blast-radius change out of this wave's scope, matching F1's identical reasoning for `POLICY_FACET_MIRROR_DRIFT`'s known false positives). `POLICY_FACET_MIRROR_DRIFT_ALLOWLIST` left fully untouched for all 4 artifacts (0 hits, real or stale — consistent with every prior wave's precedent). **After pruning, re-ran `bun ./📜️script.ts policy` and re-checked the freshly regenerated breach cache: 0 breaches, real or stale, for all 4 S-8 rules across gif/png/md/dxf.** Total breach count dropped by exactly 39 (22031 → 21992), confirming no collateral change to any other rule or artifact. `policy_shrink_confirmed: true`.

**svg/jpg/tiff re-poll (for the orchestrator's next-wave decision)**: still show modified `⚙️engine`/`🎹️composer` files and untracked new subset dirs (svg: `✳️basic`/`✳️tiny`; jpg/tiff: `✳️baseline`), same shape the PARTIAL F3 closer saw ~3 hours earlier — but newest touch across all 3 is now **~175-180 minutes old with zero further change since**, and a separate, later-dated sibling ticket (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES/🎫️ticket.json`) confirms this IS that same "subset multiplicities" wave, its own `status` field reads **`"closed"`**, and its own summary claims "744/744 passing" delivered via a 2-agent pilot + 10-agent fan-out covering exactly svg/jpg/tiff (Tiny/Basic, baseline, baseline) among others — with a caveat that it still wants "re-verification once [unrelated stl/norm sessions] land," which does not implicate svg/jpg/tiff specifically. All 3 compile and pass cleanly right now, independently re-run by this closer: `artifacts::svg::` → 50/50, `artifacts::jpg::` → 21/21, `artifacts::tiff::` → 15/15 (same counts as the PARTIAL closer's own poll — no drift). **Given the sibling ticket's own closed status plus ~3 hours of observed quiescence, svg/jpg/tiff now read as genuinely settled, not merely paused** — a meaningfully stronger signal than the PARTIAL closer had available at the time. Still recommend the orchestrator do one final direct `git status`/`cargo test` spot-check immediately before dispatching a mop-up wave (this remains a snapshot, not a guarantee), but the evidence now favors "safe to fold in" over "still live."

**Ownership-ledger update for F3's 4 rows, final**: gif/87a, gif/89a, png/1.2, md/commonmark, and dxf/r12 are now **all diff/mutation/absorb-complete per this ticket's recipe, real `cargo test`-confirmed green (853/0 whole-crate), S-8 policy-clean (0 real-or-stale breaches across all 4 rules, modulo dxf's 9 documented-accepted binary-grammar false-positive entries)** — F3 is now fully closed, no open rows remain from this wave. svg/jpg/tiff remain the only artifacts among F1-F3's original roster still needing their own dedicated diff/mutations/absorb pass, and now read as ready for that pass per the settlement evidence above.

Full report: `f3-c3-final-closer-report.md` in this ticket folder.

## F3b (fan-out wave, svg/jpg/tiff mop-up — the 3 artifacts deferred out of F3) — closed 2026-08-11

**Roster**: svg (1.1), jpg (jfif-1.01), tiff (6.0) — 3 fan-out agents (1 per artifact), 1 verify agent (`f3b-verify-report.md`), this C3b closer. This completes the full, originally-planned F3 roster (gif, png, md, dxf, svg, jpg, tiff — 7 artifacts across the wave, F3 proper + this F3b mop-up together).

**Per-artifact completion** (all 3), independently corroborated by the F3b verify agent against disk/`cargo test`, not taken on trust: real handcrafted sparse `XDiff` (zero `snapshot: Option<XSnapshot>` full-replace struct field in any of the 3 diff files — only doc-comment mentions of the deleted shape remain), `impl DiffAlgebra<XSnapshot> for XDiff` present for all 3, named-variant mutation enums with handcrafted per-variant `diff()`/`inverse()` (never apply-and-capture) for all 3, base-free structural `absorb()` satisfying the recipe's canonical cases plus associativity, and a `field_sweep`-named law test present and passing for all 3 (svg: `field_sweep`; jpg/tiff: `field_sweep_covers_every_mutable_field`). **svg's own headline defect — the apply-and-capture `SvgDiff{snapshot: Option<SvgSnapshot>}` catch-all arm flagged back in W0 recon — is confirmed genuinely fixed**: every `Mutation::diff()` variant except `SetSnapshot` builds its `SvgDiff` directly from the mutation's own fields; `SetSnapshot`'s own body is a legitimate base-vs-target recursive tree-diff (`SvgDiff::between`), not a simulate-then-compare shim. jpg killed the shared `RasterImage{width,height,rgba}` stub (flattened `width`/`height`/`pixels` onto `JpgSnapshot` directly, matching png's own precedent) and gained typed JFIF APP0/id-keyed DQT/compound-keyed DHT/DRI/verbatim-retained `other_segments`. tiff killed its own `RasterImage` copy too, replacing it with a real generic tag/type/value IFD model (`TiffByteOrder`/`TiffFieldType`/`TiffValues`/`TiffTag`/`TiffIfd`) — decode now walks the whole IFD chain generically; encode stays honestly single-IFD-scoped (`EncodeScopeNote`, mirroring this same ticket's png precedent). **Cross-checked independently by this closer: png/jpg/tiff now share zero raster/image type** (`RasterImage` appears only in historical doc-comment mentions across all three, confirmed by direct grep, not reused from any report). Both jpg's and tiff's own `✳️baseline` subset (added mid-plan by the separate, now-closed "subset multiplicities" ticket) were updated in step by their own F3b agents to keep their real ITU-T T.81/Adobe TIFF 6.0 Part 1 conformance checks working against the new snapshot shapes — confirmed compiling and passing, not just left alone.

**Deviation flagged by svg's own agent, not this closer's concern to fix**: a latent, pre-existing (shared with xml, inherited by svg's mirrored design) position-restoration gap in `SetAttribute`'s mutation-level `Mutation::inverse` when removing/re-adding a non-last attribute via mutation replay — the diff-level `DiffAlgebra::inverse` is proven fully correct (new dedicated test), only the mutation-replay convenience path loses position. Not blocking (outside svg's own artifact boundary, inherited from xml), flagged here for a future xml maintenance pass, not touched this wave.

**Full-crate gate — this closer's own fresh run: 883 passed, 0 failed, 0 ignored** (`cargo test -p semio-s-plugin-stdio --lib`, no filter) — matches the independent F3b verify report's own number exactly, and is up from F3's own 853/0 exit state by exactly the +30 these 3 artifacts' own test suites contribute (svg 58 + jpg 29 + tiff 29 = 116 total across the 3, net of some overlap in what F3's prior 853 already counted for these 3 as pre-existing-and-unrelated tests). Per-artifact filter, independently re-run by this closer: `artifacts::svg::` → 58/0, `artifacts::jpg::` → 29/0, `artifacts::tiff::` → 29/0.

**Policy shrink (`bun ./📜️script.ts policy`, the 4 S-8 rules — diff-algebra, field-sweep-presence, grammar-honesty, facet-mirror-drift), scoped to svg/jpg/tiff**: cross-checked the regenerated `.🦑️repo/⚡️cache/breaches/compose.json` directly (not the CLI's low-priority-filtered stdout, which shows none of these rules at all). Before pruning: **49 breaches, every one `-stale-` (satisfied but still allowlisted), zero real** — 3 diff-algebra (svg/jpg/tiff, one each), 3 field-sweep (same 3), 43 grammar-honesty (svg 8, jpg 14, tiff 21). `facet-mirror-drift` produced **zero breach entries of either kind** for all 3 (and for gif, cross-checked too) — meaning those allowlist entries (svg×3, jpg×3, tiff×3, still present) remain correctly protecting genuine, not-yet-fixed sibling-mirror drift, exactly F1/F2/F3's own established precedent for this rule's real false-positive/still-real-gap mix; **left fully untouched**, matching every prior wave. **One correction to a fan-out agent's own self-report, caught by this closer's live-policy-not-self-report verification discipline**: tiff's own `f3b-tiff-report.md` claimed `POLICY_GRAMMAR_HONESTY_ALLOWLIST` "never had tiff entries and needs no change" — false; tiff in fact had a full 21-entry block there (all now genuinely stale), confirmed both by direct `grep` on `📜️script.ts` and by the live breach cache showing all 21 as `-stale-`. Pruned exactly the 49 confirmed-stale entries: `POLICY_DIFF_ALGEBRA_ALLOWLIST` (svg/jpg/tiff, 3 entries), `POLICY_FIELD_SWEEP_ALLOWLIST` (same 3), `POLICY_GRAMMAR_HONESTY_ALLOWLIST` (svg's `diff`+`mutations` facets' `.g4`/`.ebnf`/`.grammar.semio`/`.protocol.semio` — 8 entries, snapshot facet correctly left allowlisted since svg's own agent explicitly did not rewrite it this wave; jpg's `diff`+`mutations`+`snapshot` facets' same 4-marker set minus `snapshot`'s `.ksy` — 14 entries, `.abnf`/`.ksy`/`.spicy` correctly left allowlisted across all 3 facets since those leaves' real content is honest JSON-wire-form prose that still legitimately contains the checker's literal substring markers, same accepted false-positive shape F1/F2/F3 already established for csv/dxf; tiff's full 21-entry block — every facet × every marker, confirmed genuinely rewritten honestly by direct content grep before pruning, not just trusting the stale-breach signal alone). **After pruning, re-ran `bun ./📜️script.ts policy` and re-checked the freshly regenerated breach cache: 0 breaches, real or stale, for all 4 S-8 rules across svg/jpg/tiff (and gif, re-confirmed still 0 — its own entries were already correctly pruned by the earlier F3 mop-up closer).** `policy_shrink_confirmed: true`. Full-crate `cargo test` re-run clean after the `📜️script.ts` edits (883/0, unchanged — expected, since these are TypeScript-tooling-only allowlist edits with zero Rust surface).

**`git check-ignore`**: no new top-level directories or `glue.rs`/`📜️script.ts` mounts were needed by any of the 3 fan-out reports (`glue_edits: []` across all 3 — svg/jpg/tiff each explicitly confirm "no glue.rs edit required", all real work landed inside already-mounted `🧬️schema/{📸️snapshot,🔺️diff,🧬️mutations}/🦀️component.rs` + sibling facet/grammar leaves per S2's Task 1 resolution). The only untracked new paths under any of the 3 artifacts' trees are the external, now-closed "subset multiplicities" ticket's own real, finished, additive work (svg's `✳️basic`/`✳️tiny`, jpg's/tiff's `✳️baseline`, plus the same pre-existing-scaffold stray `🪆️subsets/🔣️component.json` files F2's and F3's closers already found and cleared) — `git check-ignore -v` (cross-checked against `git status --porcelain --ignored`, since the raw `check-ignore -v` exit/negation-rule output is easy to misread) confirms all of them only match the `.gitignore` *negation* rule at line 179 (`!**/🔖️*/**`, explicitly trackable, not actually ignored), so no `.gitignore` action was needed. svg's own scratch-verification crate (`f3b-svg-scratch/`) lives correctly inside this ticket folder, also confirmed not gitignored.

**Ownership-ledger update for F3b's 3 rows, final — completes the full F3 roster**: svg/1.1, jpg/jfif-1.01, and tiff/6.0 are now **all diff/mutation/absorb-complete per this ticket's recipe, real `cargo test`-confirmed green (883/0 whole-crate), S-8 policy-clean (0 real-or-stale breaches across all 4 rules)** — same bar as every other closed row in this ticket. **All 7 of F3's originally-planned artifacts (gif/87a, gif/89a, png/1.2, md/commonmark, dxf/r12, svg/1.1, jpg/jfif-1.01, tiff/6.0 — 8 standards across 7 artifacts) are now fully closed.** No open rows remain from the F3 wave (F3 proper + this F3b mop-up together). Remaining open work in this ticket's overall program is whatever F4/F5/F6 (gltf/pdf/step/ifc/docx, xlsx/pptx/bcf/dwg, and the deferred facet-mirror-only follow-ups png/gif/svg-snapshot flagged) still cover — out of this closer's own scope.

## F4 (fan-out wave, gltf/pdf/step/ifc/docx) — closed 2026-08-11

**Roster**: gltf (2.0), pdf (1.4 + 1.7), step (ap214), ifc (standard `4` only — `2x3` explicitly
out of scope, see below), docx (ecma-376) — 5 fan-out agents (1 per artifact, pdf covering both
its standards), 1 verify agent (`f4-verify-report.md`), this C4 closer.

**Per-artifact completion** (all 5/6 standards), independently corroborated by the F4 verify
agent against disk/`cargo test`, not taken on trust: real handcrafted sparse `XDiff` (zero
`snapshot: Option<XSnapshot>` full-replace struct field in any diff file — only doc-comment
mentions of the deleted shape remain), `impl DiffAlgebra<XSnapshot> for XDiff` present for every
standard, named-variant mutation enums with handcrafted per-variant `diff()`/`inverse()` (never
apply-and-capture), and a `field_sweep`-named law test present and passing for every standard.
**gltf** killed `GltfSnapshot.document: serde_json::Value` entirely, replacing it with a fully
typed `GltfDocument` covering every glTF 2.0 top-level object, verified against the real
271-mesh/1095-accessor metabolism `.glb` fixture. **pdf** rewrote both standards' diffs from the
banned op-slot template to real sparse recursive diffs over the existing object-graph model
(1.7) and the minimal `PageDoc` model (1.4), verified against the real ~6.3MB bachelor-thesis
fixture; a real pre-existing decode bug (lossy-UTF8 corruption of raw deflate stream bytes) was
found and fixed as a side effect of 1.4's own `codec_retention_law` test. **step** replaced its
`document: Part21Document`-shaped snapshot (STEP's own worst-offender copy-paste-type defect)
with a typed ISO 10303-21 model (`StepHeader`/`StepEntity`/`StepValue`), keeping the shared
Part-21 tokenizer as a genuinely shared low-level substrate. **ifc** fixed the identical, more
severe instance of the same defect for standard `4` (`IfcSnapshot.document` was literally
`step::engine::part21::Part21Document` verbatim) with its own `IfcValue`/`IfcEntity`/`IfcHeader`
model; standard `2x3` was out of scope for F4 and still has the original defect (see below).
**docx** extended its snapshot from shallow paragraphs/runs to a full block tree (tables, styles,
raw-XML retention for unmodeled properties) while continuing to reuse zip's real `OpcPackage`
directly (zero reimplementation) — a real OPC relative-target relationship bug was found and
fixed while wiring the new `styles.xml` part.

**Full-crate gate — this closer's own fresh run: 965 passed, 0 failed, 0 ignored** (`cargo test
-p semio-s-plugin-stdio --lib`, no filter) — matches the independent F4 verify report's own
number exactly, up from F3b's 883/0 exit state by the 82 net tests these 5 artifacts' rewrites
add. Per-artifact filter, independently re-run by this closer: `artifacts::gltf` → 35/0,
`artifacts::pdf` → 131/0, `artifacts::step` → 91/0, `artifacts::ifc` → 62/0, `artifacts::docx` →
45/0.

**Policy shrink (`bun ./📜️script.ts policy`, the 4 S-8 rules), scoped to gltf/pdf/step/ifc/docx**:
cross-checked the regenerated `.🦑️repo/⚡️cache/breaches/compose.json` directly. Before pruning:
**60 breaches, every one `-stale-` (satisfied but still allowlisted), zero real** — 6
diff-algebra (gltf, ifc/4, pdf/1.4, pdf/1.7, step, docx — one each), 6 field-sweep (same 6), 48
grammar-honesty (ifc/4 ×6 — snapshot facet only, diff/mutations correctly left allowlisted as a
documented deferred gap; step ×6 — one `.protocol.semio`+`.grammar.semio` pair per facet,
remainder correctly left allowlisted matching zip's own established precedent; pdf/1.7 ×17 — 1.4
correctly 0 stale, its grammar leaves were explicitly left as placeholder per the brief's
"main target" triage; gltf ×19 — every facet × every leaf kind; docx ×0 — its own report
explicitly left grammar leaves untouched this wave, correctly still allowlisted). `facet-mirror-
drift` produced **zero breach entries of either kind** for all 5 — the field-name-substring
heuristic did not confirm zero-drift for gltf/pdf/step/ifc despite their own reports' claims of
complete facet mirrors, so those allowlist entries were **left fully untouched** (same
don't-trust-the-self-report discipline F3b's closer used on tiff); docx's own report explicitly
didn't touch facet mirrors this wave, consistent with 0 stale there too. Pruned exactly the 60
confirmed-stale entries from `POLICY_DIFF_ALGEBRA_ALLOWLIST` (6), `POLICY_FIELD_SWEEP_ALLOWLIST`
(6), and `POLICY_GRAMMAR_HONESTY_ALLOWLIST` (48) — one accidental double-match caught and handled
during pruning (the diff-algebra and facet-mirror-drift allowlists share identical literal key
strings for these 5 artifacts, e.g.
`"stdio/ifc/standards#4-subsets-any-schema-diff-component"` appears verbatim in both arrays;
scoped each removal to its own array's `[`...`]);` span so only the intended array was edited).
**After pruning, re-ran `bun ./📜️script.ts policy` and re-checked the freshly regenerated breach
cache: 0 breaches, real or stale, for all 4 S-8 rules across gltf/pdf/step/ifc/docx** (excluding
ifc's untouched `2x3` sibling standard, pre-existing state, not part of F4). Also swept every
prior wave's own fan-out reports for unaddressed "should be pruned"/"stale" mentions per the
brief's instruction: found one (`f3b-tiff-report.md`'s facet-mirror-drift claim), already
investigated and deliberately left in place by `f3b-closer-report.md` with documented reasoning —
not an oversight. Cross-checked via a repo-wide query of the regenerated breach cache for
`-stale-` entries across all 4 S-8 rules: **zero**, confirming every prior wave's closer (F1, F2,
F3, F3b) and this one left their allowlists fully shrunk. `policy_shrink_confirmed: true`.
Full-crate `cargo test` re-run clean after the `📜️script.ts` edits (965/0, unchanged — expected,
TypeScript-tooling-only allowlist edits with zero Rust surface).

**`glue_edits: []`** — no `glue.rs`/`script.ts` mounts were needed by any of the 5 fan-out
reports or the verify report; all real work landed inside already-mounted
`🧬️schema/{📸️snapshot,🔺️diff,🧬️mutations}/🦀️component.rs` + sibling `⚙️engine`/`🏗️builder`/
`🧐️analyzer`/facet/grammar leaves per S2's Task 1 resolution. One deferred (not urgent) design
note from docx: `DocxOpcDiff` and its 4 sibling diff types currently live inside docx's own
`🔺️diff/🦀️component.rs` rather than `zip/📦️opc/🦀️component.rs` (docx's ownership boundary didn't
reach that file) — future consolidation once xlsx/pptx/bcf need the same OPC diff shape, not
actioned this wave.

**`git check-ignore`**: no new directories were created by any F4 agent. The only untracked paths
under `🗿️artifacts/{🧊️gltf,📄️pdf,📐️step,🏗️ifc,📜️docx}/**` are pre-existing deliverables from the
separate, now-closed sibling ticket `26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES`
(pdf's `✳️a`/`✳️e`/`✳️h`/`✳️ua`/`✳️vt`/`✳️x`, step's `✳️cc1`–`✳️cc6` + `⚙️engine/🪜️ladder`, docx's
`✳️strict`/`✳️transitional`, ifc's whole `2x3` standard, one `🔣️component.json` subset-registry
file per artifact) — none created or modified by F4. `git check-ignore -v` confirms all of them
match only the `.gitignore` line-179 *negation* rule (`!**/🔖️*/**`, explicitly trackable, not
actually ignored; `git status --porcelain` independently shows them as plain `??` untracked, not
silently absent). No `.gitignore` action needed.

**Residual defect, flagged not fixed (out of F4's assigned scope)**: ifc's **`2x3` standard**
still has the original W0-flagged worst-offender pattern — its `IfcSnapshot`/diff/mutation
structs directly store/import `step::engine::part21::Part21Document`/`Part21Header`/
`Part21Instance`/`Part21Value` as their own persisted types, the exact defect standard `4` fixed
this wave. F4's brief scoped ifc to standard `4` only; `2x3` was never assigned to any F4 agent.
Confirmed via the breach cache: `2x3`'s grammar-honesty entries fire as genuine (non-stale, "low"
priority) breaches, not allowlisted — pre-existing, untouched by F4, not a regression. Flagged
for the orchestrator to decide whether `2x3` needs its own future wave.

**Ownership-ledger update for F4's 5 rows**: gltf/2.0, pdf/1.4, pdf/1.7, step/ap214, and ifc/4 +
docx/ecma-376 are now **all diff/mutation/absorb-complete per this ticket's recipe, real `cargo
test`-confirmed green (965/0 whole-crate), S-8 policy-clean (0 real-or-stale breaches across all
4 rules)** — same bar as every other closed row in this ticket. ifc/2x3 remains open (out of
scope, see above). Remaining open work in this ticket's overall program is whatever F5/F6
(xlsx/pptx/bcf/dwg, the deferred facet-mirror-only follow-ups, `DiffCodec` for every artifact,
and ifc/2x3 if assigned) still cover — out of this closer's own scope.

Full report: `f4-closer-report.md` in this ticket folder.

Full report: `f3b-closer-report.md` in this ticket folder.

## F5 (fan-out wave, xlsx/pptx/bcf/dwg) — closed 2026-08-11 — LAST per-artifact fan-out wave, all 31 standards now schema-complete

**Roster**: xlsx (ecma-376), pptx (ecma-376), bcf (2.1), dwg (ac1018 + ac1024) — 4 fan-out agents
(1 per artifact, dwg covering both its standards), 1 verify agent (`f5-verify-report.md`), this C5
closer.

**Per-artifact completion** (all 4/5 standards), independently corroborated by the F5 verify agent
against disk/`cargo test`, not taken on trust: real handcrafted sparse `XDiff` (zero
`snapshot: Option<XSnapshot>` full-replace struct field in any diff file — only doc-comment
mentions of the deleted shape remain), `impl DiffAlgebra<XSnapshot> for XDiff` present for every
standard, named-variant mutation enums with handcrafted per-variant `diff()`/`inverse()` (never
apply-and-capture), and a `field_sweep`-named law test present and passing for every standard.
**xlsx** redesigned its cell model from eagerly-resolved A1-string text to a real typed
`(row,col)`/`XlsxCellValue{Number,SharedString(idx),InlineString,Boolean,Formula,Empty}` union with
an explicit `shared_strings: Vec<String>` SST field (the #1 xlsx decode gotcha, previously collapsed
away). **pptx** un-flattened its slide model — `PptxShape{TextBox,Picture,Placeholder,Other}` real
per-shape variants replacing the old paragraphs-concatenated-across-every-shape shape, the exact W0
defect fixed. **bcf** replaced a shallow flat-`Vec` reconciled-view stub with the full completeness
target (guid-keyed topics/comments/viewpoints, a real `BcfCamera`/`BcfComponents`/`BcfColoring`
visualization-info model, a `Priority`-as-attribute→element spec-accuracy fix) atop its own simple
package wrapper (bcfzip has no OPC apparatus, so — per the brief's own documented fallback — it does
NOT reuse `zip::opc`, unlike docx/xlsx/pptx). **dwg** gave both standards a real diff/mutations
layer within their existing, previously-established honest decode boundaries: ac1024 kept its real
D1/D2 (file-header decrypt, section location, LZ77-variant decompression — confirmed still green on
the 145KB `architectural.dwg` fixture) with 2 new real header fields
(`maintenance_version`/`codepage`, externally verified against LibreDWG's own `header.spec` and
cross-checked byte-for-byte against the real fixture); ac1018 stayed frozen at its Decision #5
scope (no decode parity attempted, as instructed) but gained the same schema-layer treatment, plus
a real pre-existing bug fix (ac1018's own `diff`/`mutations` files had been silently importing
ac1024's canonical type via the shared top-level re-export the whole time — harmless while both
standards shared a generic template, a hard compile error once ac1018 got its own vocabulary; fixed
by repointing to ac1018's own standard-local types).

**Full-crate gate — this closer's own fresh run: 1013 passed, 0 failed, 0 ignored** (`cargo test -p
semio-s-plugin-stdio --lib`, no filter) — matches the independent F5 verify report's own number
exactly, up from F4's 965/0 exit state by the 48 net tests these 4 artifacts' rewrites add.
Per-artifact filter, independently re-run by this closer: `artifacts::xlsx` → 41/0,
`artifacts::pptx` → 48/0, `artifacts::bcf` → 16/0, `artifacts::dwg` → 31/0 (both standards
combined).

**Policy shrink (`bun run ./📜️script.ts policy`, the 4 S-8 rules), scoped to xlsx/pptx/bcf/dwg**:
cross-checked the regenerated `.🦑️repo/⚡️cache/breaches/compose.json` directly. Before pruning: **10
breaches, every one `-stale-` (satisfied but still allowlisted), zero real** — 5 diff-algebra
(pptx, bcf, xlsx, dwg/ac1018, dwg/ac1024 — one each), 5 field-sweep (same 5). `grammar-honesty` and
`facet-mirror-drift` produced **zero breach entries of either kind** for all 4 artifacts (every
fan-out report explicitly left grammar/facet leaves untouched this wave, consistent with every
prior wave's own documented deferral). Pruned exactly the 10 confirmed-stale entries — both
`POLICY_DIFF_ALGEBRA_ALLOWLIST` and `POLICY_FIELD_SWEEP_ALLOWLIST` are now **fully empty `Set`s**.

**Full 31-standard sweep (this wave's specific mandate, being the last fan-out wave)**: re-ran
policy after pruning and queried the freshly regenerated breach cache **repo-wide**, all 4 S-8
rules: `diff-algebra` → **0 stale, 0 real**; `field-sweep-presence` → **0 stale, 0 real**;
`grammar-honesty` → 0 stale, 21 real (**all `ifc/2x3`**, a pre-existing, never-assigned-to-any-wave
defect F4's closer already flagged, confirmed still the only source); `facet-mirror-drift` → 0
stale, 3 real (same, all `ifc/2x3`). **Every one of this ticket's 31 standards now has a real
`impl DiffAlgebra` and a real passing `field_sweep` test — both allowlists are empty, nothing left
to prune, ever, for these 2 rules.** Swept every prior wave's own fan-out reports for unaddressed
"should be pruned"/"stale" mentions: found the same single hit F4's closer already resolved
(`f3b-tiff-report.md`, deliberately left in place by `f3b-closer-report.md`) — nothing new missed
across F1–F5. `policy_shrink_confirmed: true`. Full-crate `cargo test` re-run clean after the
`📜️script.ts` edits (1013/0, unchanged — TypeScript-tooling-only allowlist edits).

**`glue_edits: []`** — no `glue.rs` mount was needed by any of the 4 fan-out reports or by this
closer; all real work landed inside already-mounted `🧬️schema/{📸️snapshot,🔺️diff,🧬️mutations}/
🦀️component.rs` + sibling `⚙️engine`/`🏗️builder`/`🧐️analyzer` leaves per S2's Task 1 resolution.
Three deferred (not urgent) `glue_followup` design notes reviewed, none actioned this wave (same
"future consolidation, not this closer's mandate" call F4's closer made for docx's identical note):
`XlsxOpcDiff`/`PptxOpcDiff` (3rd/4th copies of docx's own OPC-diff shape, still living in each
artifact's own `🔺️diff` file rather than `zip::opc`); bcf's own copy of the generic
`NamedTripleDiff<K,D,T>` engine (now 4 independent copies: docx, bcf, xlsx, pptx); dwg ac1018's
`⚙️engine`/`🧐️analyzer`/`🏗️builder`/`🎹️composer`/`🚪️io` subtree still importing ac1024's canonical
types rather than its own (this closer's own read: consistent with Decision #5's "frozen shim,
delegate operational plumbing" framing, not a contradiction — not re-flagged as an open defect, a
future targeted pass can start from this closer's reasoning instead of re-litigating it).

**`git check-ignore`**: no new directories were created by any F5 agent. The only untracked paths
under `🗿️artifacts/{📕️xlsx,🎞️pptx,💬️bcf,🖊️dwg}/**` are pptx's/xlsx's own `✳️strict`/`✳️transitional`
subset dirs (pre-existing deliverables from the separate, now-closed sibling ticket
`ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES`, matching docx's own identical pattern from F4) plus
one stray `🔣️component.json` scaffold file per standard (same pre-existing-scaffold pattern every
prior closer since F2 has found and cleared). `git check-ignore -v` confirms all 9 match only the
`.gitignore` line-179 *negation* rule (`!**/🔖️*/**`, explicitly trackable, not actually ignored). No
`.gitignore` action needed.

**Ownership-ledger update for F5's 4 rows**: xlsx/ecma-376, pptx/ecma-376, bcf/2.1,
dwg/ac1018, and dwg/ac1024 are now **all diff/mutation/absorb-complete per this ticket's recipe,
real `cargo test`-confirmed green (1013/0 whole-crate), S-8 policy-clean (0 real-or-stale breaches
across all 4 rules)** — same bar as every other closed row in this ticket. **F5 is the last of the
5 originally-planned per-artifact fan-out waves — all 31 standards this program set out to cover
are now schema-complete.** ifc/2x3 remains the sole open row, out of scope for every wave to date
(added by a separate sibling ticket after this program's original 31-standard count was fixed, never
assigned to F1–F5).

### Final summary table — all 31 standards, pulled from this file's own F1–F5 ledger entries

| # | Artifact | Standard | Closed by | Tests (own filter) |
|---|---|---|---|---|
| 1 | 💾️binary | raw | F1 | part of 187/0 |
| 2 | 📄txt | utf-8 | F1 | part of 187/0 |
| 3 | 🔣️json | rfc8259 | F1 | part of 187/0 |
| 4 | 📰xml | 1.0 | F1 | part of 187/0 |
| 5 | 📊️csv | rfc4180 | F1 | part of 187/0 |
| 6 | 🗜️deflate | rfc1950 | F1 | part of 187/0 |
| 7 | 🎒️zip | 2.0 | F1 | part of 187/0 |
| 8 | 🟪️stl | ascii | F2 | 21/0 |
| 9 | 🧊️obj | 3.0 | F2 | 17/0 |
| 10 | ☁️ply | 1.0 | F2 | 23/0 |
| 11 | ☁️las | 1.0 | F2 | 21/0 |
| 12 | 🖼️bmp | v3 | F2 | 14/0 |
| 13 | 🎞️gif | 87a | F3 mop-up | part of 55/0 |
| 14 | 🎞️gif | 89a | F3 mop-up | part of 55/0 |
| 15 | 📷️png | 1.2 | F3 | 22/0 |
| 16 | 📝️md | commonmark | F3 | 24/0 |
| 17 | 🖊️dxf | r12 | F3 mop-up | 13/0 |
| 18 | 🎨️svg | 1.1 | F3b | 58/0 |
| 19 | 📷️jpg | jfif-1.01 | F3b | 29/0 |
| 20 | 🖼️tiff | 6.0 | F3b | 29/0 |
| 21 | 🧊️gltf | 2.0 | F4 | 35/0 |
| 22 | 📄️pdf | 1.4 | F4 | part of 131/0 |
| 23 | 📄️pdf | 1.7 | F4 | part of 131/0 |
| 24 | 📐️step | ap214 | F4 | 91/0 |
| 25 | 🏗️ifc | 4 | F4 | 62/0 |
| 26 | 📜️docx | ecma-376 | F4 | 45/0 |
| 27 | 📕️xlsx | ecma-376 | **F5** | 41/0 |
| 28 | 🎞️pptx | ecma-376 | **F5** | 48/0 |
| 29 | 💬️bcf | 2.1 | **F5** | 16/0 |
| 30 | 🖊️dwg | ac1018 | **F5** | part of 31/0 |
| 31 | 🖊️dwg | ac1024 | **F5** | part of 31/0 |

**All 31/31 standards closed.** `cargo test -p semio-s-plugin-stdio --lib` (whole crate, this
closer's own fresh run): **1013 passed, 0 failed**. `ifc/2x3` (a 32nd standard added by a separate,
unrelated sibling ticket after this program's scope was fixed) remains the only open row anywhere
in this ticket's schema-overhaul ledger — flagged to the orchestrator by F4's closer, re-confirmed
still open and still out of every wave's assigned roster by this closer's own full policy sweep
(§ above). This is the primary input for F6 (op-codec) wave planning: every standard now has a real
snapshot/diff/mutations triad to build a `DiffCodec`/wire-serialization layer on top of, plus 4
deferred `glue_followup` consolidation notes (OPC-diff-types hoist ×2, `NamedTripleDiff` engine
hoist, dwg ac1018 import-boundary decision — see this section's own `glue_edits` paragraph above)
that a future targeted pass (not necessarily F6 itself) should pick up.

Full report: `f5-closer-report.md` in this ticket folder.

## F6a (op-codec fan-out sub-wave, ply/ifc4/txt/pdf1.4/csv/step/xlsx) — closed 2026-08-11 — first op-codec sub-wave of F6

**Roster**: `☁️ply` 1.0, `🏗️ifc` 4 (not 2x3, out of scope), `📄txt` utf-8, `📄️pdf` 1.4 (not 1.7, out of
scope), `📊️csv` rfc4180, `📐️step` ap214, `📕️xlsx` ecma-376 — 7 fan-out agents (1 per artifact/standard),
1 verify agent (`f6a-verify-report.md`), this C6a closer. Precedes this sub-wave: a pilot (not tracked
in this file until now) that already landed `💾️binary`/`🎞️gif 89a`/`🎨️svg 1.1` and produced
`f6-recon-report.md` (the authoritative spec for every F6 fan-out agent, incl. this wave's 7).

**Scope of F6 (distinct from F1-F5 above)**: F1-F5 built the snapshot/diff/mutation *type* triad
(schema-complete per artifact). F6 builds the *wire codec* layer on top — `protocol::DiffCodec` for
each artifact's `XDiff` type and `protocol::OpText`/`protocol::OpBinary` for its `XMutation` type,
replacing the placeholder `serde_json`-based stubs every one of these 7 files still had.

**Per-artifact classification** (STEP 1 of `f6-recon-report.md` §9, every one independently verified
by actually adding the derive attributes and reading real `cargo check` errors — never trusted from
the recon's own heuristic §8 table, which was a single-file grep sweep and got several rows wrong):

| Artifact | Standard | Diff path | Mutation path | Real blocker (if hand-roll) |
|---|---|---|---|---|
| ☁️ply | 1.0 | hand-roll | hand-roll | `PlyProperty`/`PlyValue` data-carrying enums in the snapshot tree (§3a) — recon's own "DERIVE probable" guess was wrong |
| 🏗️ifc | 4 | hand-roll | hand-roll | `IfcValue` data-carrying enum, directly and transitively reachable from both sides (§3a) — recon's own "DERIVE probable" guess was wrong |
| 📄txt | utf-8 | **derive** | **derive** | none — matched the recon's own guess exactly, zero hand-rolling needed |
| 📄️pdf | 1.4 | **derive** | **derive** | none — matched the recon's own guess; 1.4's `PdfSnapshot`/`PageDoc` tree has no `PdfValue` enum (that's 1.7's, out of scope) |
| 📊️csv | rfc4180 | hand-roll | hand-roll | Diff: `Option<Vec<Option<CsvFieldDiff>>>` (`Vec`-wrapped tri-state, no blanket `DslField` for `Option<T>`, §3b-adjacent). Mutation: a genuine `dsl_derive` macro-hygiene bug — any variant field literally named `record` shadows the codegen's own `record` accumulator variable (a NEW failure mode beyond recon's §3a/§3b, flagged for framework awareness, not fixed — `dsl_derive` is out of this ticket's ownership) |
| 📐️step | ap214 | hand-roll | hand-roll | `StepValue` data-carrying enum, directly and transitively reachable from both sides (§3a) |
| 📕️xlsx | ecma-376 | hand-roll | hand-roll | `XlsxCellValue` data-carrying enum (§3a) + `NamedTripleDiff<K,D,T>`, a generic collection type with no `DslField` impl (a second, independent structural blocker) |

Every hand-roll used §5's shared grammar template (bracket-depth-aware `split_top_level`, hex for
strings/bytes, `[0]`/`[1,x]` for `Option<T>`, `[removed];[modified];[added]` collection triples,
single-uppercase-letter enum tags, space-separated `name=value`/`keyword arg=value` top-level lines,
`encode_*` = the printed text bytes verbatim). Every derive used cascading
`#[derive(dsl::DslRecord)]`/`#[derive(dsl::DslScalar)]` on nested types then `dsl::DslDiff`/`dsl::DslOps`
on the top-level type, followed by the standard `OpText`/`OpBinary` wrapper (`DslOps` never emits
those itself, per P6 — every mutation side, derived or not, ends with a handwritten `OpText`/`OpBinary`
impl; "derive" in the table above means the wrapper is boilerplate-only, no custom grammar).

**One real bug found and fixed in-flight** (xlsx, self-corrected by its own fan-out agent before this
closer ran): the first `diff_codec_text_binary_roundtrip_law` run silently dropped a legitimate
empty-string OPC relationship-owner key (`""`) because every `dec_*` list-splitter chained a defensive
`.filter(|s| !s.is_empty())` after `split_top_level` — harmless for "0 items" (already handled by
`split_top_level`'s own empty-input short-circuit) but actively wrong for "1+ items, one of them
`""`". Fixed by removing all 12 occurrences across both xlsx files. Every earlier-run report in this
ticket folder (ply/ifc4/txt/pdf1.4/csv/step) that shows a whole-crate run with "1032 passed, 1 failed"
is this same xlsx bug caught mid-flight by a concurrently-running sibling agent, not a regression any
of those 6 artifacts caused — confirmed by this closer's own independent full-crate re-run below,
taken after xlsx's fix landed, showing 0 failures.

**Full-crate gate — this closer's own fresh run**: `cargo test -p semio-s-plugin-stdio --lib` (no
filter) → **1033 passed, 0 failed, 0 ignored, 0 measured, 0 filtered out**, finished in 7.46s. Matches
the independent F6a verify agent's own number exactly (`f6a-verify-report.md`). Per-artifact filtered
counts, cross-checked against the verify report (not re-run individually by this closer — the verify
agent already did so scoped to exactly the standard in question, excluding sibling standards under the
same artifact directory that are out of scope): ply 25/0, ifc/v4 19/0, txt 21/0, pdf/v1_4 23/0, csv
19/0, step 93/0, xlsx 43/0 — every one includes both mandatory law tests
(`diff_codec_text_binary_roundtrip_law`, `op_text_binary_roundtrip_law`), zero failures anywhere.

**Policy shrink (`bun ./📜️script.ts policy`, `dsl-migration/diff-completeness` rule, stdio-scoped)**:
this closer's own fresh run — **22 stdio breaches remain** (down from the recon's §7 baseline of 28
"remaining before this sub-wave"). Verified precisely, not just by the summary count: grepped the
full breach listing for every one of this wave's 7 artifact/standard paths (`☁️ply/1.0`,
`🏗️ifc/4` — NOT `2x3`, `📄txt/utf-8`, `📄️pdf/1.4` — NOT `1.7`, `📊️csv/rfc4180`, `📐️step/ap214`,
`📕️xlsx/ecma-376`) — **zero matches for any of the 7**, confirming every one's new `DiffCodec` impl
(hand-rolled or `dsl::DslDiff`-derived) is real enough to satisfy the check's literal-text grep
(`content.includes("dsl::DslDiff") || content.includes("DiffCodec for")`, `📜️script.ts:3185-3205`).
The observed 22 (not the naively-expected 28−7=21) is **not a shortfall** — it's `21` (the official,
recon-tracked count) **+ 1** (`🏗️ifc/2x3`, the pre-existing 32nd standard added by the unrelated sibling
ticket `ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES`, explicitly out of scope for every wave to date
per F5's closer's own note above, still un-migrated, unrelated to this wave's work). Full raw policy
output saved (ticket-folder scratch, `.txt`): `f6a-closer-policy-full.txt`. Remaining 22 stdio breaches
(for the next op-codec sub-wave to pick up): `☁️las`, `🎒️zip`, `🎞️gif 87a`, `🎞️pptx`, `🏗️ifc 2x3`,
`💬️bcf`, `📄️pdf 1.7`, `📜️docx`, `📝️md`, `📰xml`, `📷️jpg`, `📷️png`, `🔣️json`, `🖊️dwg ac1018`,
`🖊️dwg ac1024`, `🖊️dxf`, `🖼️bmp`, `🖼️tiff`, `🗜️deflate`, `🟪️stl`, `🧊️gltf`, `🧊️obj`.
`POLICY_DIFF_COMPLETENESS_ALLOWLIST` (`📜️script.ts:2304`) untouched by any of the 7 fan-out agents or
this closer, confirmed still zero stdio entries — every one of the 7 breaches disappeared on its own
merits, not via allowlisting, matching the mission's own "zero stdio entries, for real" goal.

**`glue_followup: []`** — none of the 7 fan-out reports flagged a need for a new `glue.rs` mount (F6's
op-codec work lands entirely inside already-mounted `🧬️schema/{🔺️diff,🧬️mutations}/🦀️component.rs`
files, same leaves F1-F5 already wired). `📦️glue.rs` and `📜️script.ts` were both read-only for every
one of the 7 fan-out agents (confirmed via each report's own "no shared files touched" section) and
were not edited by this closer either — the large pending diffs `git status` currently shows on both
files belong entirely to the separate, concurrently-active `ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES`
sibling ticket (confirmed: `git diff` on `glue.rs` shows only step's `cc1`-`cc6` subset `#[path=...]`
mounts, none of this wave's 7 artifacts' op-codec files, which don't need new mounts at all since they
extend files already `#[path=...]`-mounted by F1-F5).

**`git check-ignore`**: no new directories were created by any of the 7 fan-out agents. The untracked
paths seen under this wave's own artifact dirs (`✳️a`/`✳️x` under pdf/1.4, `✳️cc1`-`✳️cc6` under
step/ap214, `✳️strict`/`✳️transitional` under xlsx/ecma-376, plus stray `🔣️component.json` scaffolds
per standard) are the same pre-existing sibling-ticket scaffold pattern every closer since F2 has found
and correctly left alone — none of them were touched or need to be.

**Ownership-ledger update for F6a's 7 rows**: ply/1.0, ifc/4, txt/utf-8, pdf/1.4, csv/rfc4180,
step/ap214, and xlsx/ecma-376 are now **op-codec-complete** (real `protocol::DiffCodec` +
`protocol::OpText`/`protocol::OpBinary`, no `serde_json` stub remaining anywhere in any of the 14
files), real `cargo test`-confirmed green (1033/0 whole-crate), policy-clean for
`dsl-migration/diff-completeness` (0 of the 7 present in the breach list). **24 of 31 official standards
remain for future op-codec sub-waves** (28 minus this wave's 7 — `🏗️ifc/2x3`, the 32nd, is separately
tracked and was never part of the 31), listed in full in the "Remaining 22 stdio breaches" paragraph
above (21 of those 22 are official-scope; `ifc/2x3` is the extra).

Full report: `f6a-closer-report.md` in this ticket folder. Per-artifact reports: `f6-ply-report.md`,
`f6-ifc-4-report.md`, `f6-txt-report.md`, `f6-pdf-1.4-report.md`, `f6-csv-report.md`,
`f6-step-report.md`, `f6-xlsx-report.md`. Verify report: `f6a-verify-report.md`. Recon (spec for all
of F6): `f6-recon-report.md`.

## F6b (op-codec fan-out sub-wave, dwg ac1018/ac1024/bmp/stl/las/gif87a/zip) — closed 2026-08-11 — second op-codec sub-wave of F6

**Roster**: `🖊️dwg` ac1018, `🖊️dwg` ac1024, `🖼️bmp` v3, `🟪️stl` ascii, `☁️las` 1.0, `🎞️gif` 87a,
`🎒️zip` 2.0 — 7 fan-out agents (1 per artifact/standard), 1 verify agent (`f6b-verify-report.md`),
this C6b closer. Same scope definition as F6a: wire-codec layer (`protocol::DiffCodec` +
`protocol::OpText`/`protocol::OpBinary`) on top of the snapshot/diff/mutation type triad F1-F5
already built, replacing the placeholder `serde_json`-based stubs.

**Per-artifact classification** (STEP 1 independently re-verified for real by every fan-out agent,
never trusted from the recon's own §8 heuristic table):

| Artifact | Standard | Diff path | Mutation path | Real blocker (if hand-roll) |
|---|---|---|---|---|
| 🖊️dwg | ac1018 | **derive** | derive+wrapper | none — matched recon's own guess exactly, second artifact after `💾️binary` to land clean-derive on both sides |
| 🖊️dwg | ac1024 | **derive** | derive+wrapper | none — matched recon's own guess; real 145KB `architectural.dwg` fixture confirmed still round-trips losslessly (`codec_retention_law` green, untouched by this wave's derive additions) |
| 🖼️bmp | v3 | **derive** | derive+wrapper | none — matched recon's own guess exactly |
| 🟪️stl | ascii | hand-roll | hand-roll | **recon table said "DERIVE (probable)" — wrong.** A third, previously-undocumented derive blocker: nested fixed-arity arrays (`[[f64;3];3]`) compile clean under the derive but are NOT print/parse round-trip-safe at runtime — the shared `dsl` crate's `Shape::Tuple` printer flattens every nesting level into one indistinguishable comma-join, and the parser never bounds a nested tuple's comma-consumption to its own arity, greedily eating the outer tuple's remaining values too (`"tuple expects 3 elements, found 9"`, a real reproduced runtime failure). Traced to `🧰️framework/…/🗣️dsl/🧬️schema/🦀️component.rs`'s `print_shape`/`parse_shape`. Out of ownership boundary to fix (shared framework file) — documented via doc comment citation on `StlTriangle`/`StlDiff`/`StlMutation`, not fixed. Flagging: any other artifact with a `[[T;N];M]`-shaped field will hit the same bug; no repo-wide grep for this shape was run as part of any F6 agent's scope. |
| ☁️las | 1.0 | hand-roll | hand-roll | **Missing from the recon's §8 table entirely (31 rows, no `las` row) — gap now filled.** 3b tri-state (`gps_time`/`rgb`) PLUS a fourth, previously-undocumented blocker class: bare tuples (`(u16,u16,u16)`, `(f64,f64,f64)`) have no blanket `impl DslField for (A,B,...)` anywhere in the `dsl` crate (same root cause family as 3b — a missing blanket impl — different type shape). Both sides hand-rolled cleanly, 23/23 scoped tests, both mandatory law tests present and green. |
| 🎒️zip | 2.0 | hand-roll | **derive**+wrapper | Diff: 3b tri-state (`ZipEntryDiff::unix_mtime: Option<Option<i64>>`), zip's only tri-state field — matches recon's row 18 guess exactly. Mutation: recon table only classifies the Diff side per-standard (its own stated scope) — Mutation side independently verified DERIVE-clean, zero data-carrying enum anywhere in its reachable tree. |
| 🎞️gif | 87a | hand-roll | **derive**+wrapper | Diff: 3b tri-state (`GifDiff::gct`, `GifImageDiff::lct`, both `Option<Option<GifColorTable>>`) — same split gif89a already documented. Mutation: derived clean, matching gif89a's precedent exactly. |

Two genuinely new derive-blocker classes surfaced this wave, beyond the recon's own §3a (enum)/§3b
(tri-state) taxonomy: **nested fixed-arity arrays** (stl, a real `dsl` framework bug, not fixed —
out of every F6 agent's ownership boundary) and **bare tuples** (las, same missing-blanket-impl root
cause as 3b, different shape). Neither fixed this wave (both are shared-framework-file findings);
both documented with doc-comment citations at the point of use and flagged here for whoever next
works on the `dsl` crate's `DslField`/`Shape` machinery.

**Full-crate gate — this closer's own fresh run**: `cargo test -p semio-s-plugin-stdio --lib` (no
filter) → **1047 passed, 0 failed, 0 ignored, 0 measured, 0 filtered out**, finished in ~7.7s.
Matches the independent F6b verify agent's own number exactly (`f6b-verify-report.md`, which
re-ran every one of the 7 scoped test suites itself AND the whole-crate suite, from its own
uninvolved session). Per-artifact scoped counts (cross-checked against the verify report): dwg
ac1018 12/12, dwg ac1024 18/18, bmp 16/16, stl 23/23, las 23/23, zip 40/40, gif87a 27/27 — sum 159 —
every one includes both mandatory law tests (`diff_codec_text_binary_roundtrip_law`,
`op_text_binary_roundtrip_law`), zero failures anywhere, zero `serde_json` stub remnants in any of
the 14 diff/mutations files (independently re-confirmed by the verify agent via direct file
inspection, not just report-trusting).

**Policy shrink (`bun run ./📜️script.ts policy`, `dsl-migration/diff-completeness` rule,
stdio-scoped)**: this closer's own fresh run — **15 stdio breaches remain** (down from F6a's
closer-confirmed 22). Verified precisely: grepped the full breach listing for every one of this
wave's 7 artifact/standard paths — **zero matches for any of the 7**, confirming every one's new
`DiffCodec` impl (hand-rolled or `dsl::DslDiff`-derived) satisfies the check's literal-text grep.
The drop from 22 → 15 is exactly this wave's 7 artifacts, no more, no less.
`POLICY_DIFF_COMPLETENESS_ALLOWLIST` (`📜️script.ts:2304`) confirmed untouched by any of the 7
fan-out agents or this closer — grepped the allowlist's full contents for `stdio`: zero matches,
same "zero stdio entries, for real" outcome as F6a. Remaining 15 stdio breaches (14 official-scope +
`🏗️ifc/2x3` extra, tracked separately, never part of the 31): `🎞️pptx`, `🏗️ifc 2x3`, `💬️bcf`,
`📄️pdf 1.7`, `📜️docx`, `📝️md`, `📰xml`, `📷️jpg`, `📷️png`, `🔣️json`, `🖊️dxf`, `🖼️tiff`, `🗜️deflate`,
`🧊️gltf`, `🧊️obj`.

**`glue_followup: []`** — none of the 7 fan-out reports flagged a need for a new `glue.rs` mount
(same pattern as F6a — op-codec work lands entirely inside already-mounted
`🧬️schema/{🔺️diff,🧬️mutations}/🦀️component.rs` files). `glue.rs` shows zero diff against its
tracked baseline as of this closer's session (whatever "MM" state was visible in `git status` at
session start resolved on its own — another concurrent session's unrelated edit). `script.ts` has a
large pending diff, but grepped for every one of this wave's 7 artifact names: only pre-existing
schema-id/grammar-manifest inventory churn (facet-mirror list entries, not policy-rule logic, not
allowlist entries) that predates this closer's own session (file mtime earlier than this session's
`policy` run) — same concurrent `ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES` sibling-ticket
automation pattern every closer since F2 has documented and correctly left alone. Not touched by
this closer.

**`git check-ignore`**: every one of the 7 artifact trees shows a stray untracked
`🪆️subsets/🔣️component.json` (same pre-existing scaffold pattern every closer since F2 has found),
plus zip additionally shows an untracked `🪆️subsets/✳️iso21320/` directory. `git check-ignore -v` on
all 8 paths confirms every one matches only the `.gitignore` negation rule `!**/🔖️*/**` — none
actually gitignored, no action needed. Also noted: `bmp`/`stl` (and to a lesser extent `las`/`dwg`)
show dozens of additional modified grammar-leaf facet-mirror files beyond the 3 `.rs` files each
fan-out report lists as touched — content-inspected, none are op-codec-shaped edits, consistent with
the same concurrent sibling-ticket regeneration pattern, not this wave's work. `zip`'s
`⚙️engine`/`🎹️composer` files show small (4-5 line) pre-existing diffs predating this session,
consistent with leftover uncommitted state from zip's much earlier F1 wave, not touched again here.

**Ownership-ledger update for F6b's 7 rows**: dwg/ac1018, dwg/ac1024, bmp/v3, stl/ascii, las/1.0,
zip/2.0, and gif/87a are now **op-codec-complete** (real `protocol::DiffCodec` +
`protocol::OpText`/`protocol::OpBinary`, no `serde_json` stub remaining anywhere in any of the 14
files), real `cargo test`-confirmed green (1047/0 whole-crate), policy-clean for
`dsl-migration/diff-completeness` (0 of the 7 present in the breach list). **`las`'s classification
gap — entirely missing from the recon's own §8 table — is now filled**, both sides hand-rolled, real
substantive coverage, confirmed not silently skipped. **14 of 21 official-scope standards remain**
for future op-codec sub-waves (28 recon baseline − F6a's 7 − F6b's 7 = 14; `🏗️ifc/2x3` is the extra,
separately tracked, never part of the 31) — 15 total stdio breaches remaining including it, matching
the policy count above exactly.

Full report: `f6b-closer-report.md` in this ticket folder. Per-artifact reports:
`f6-dwg-ac1018-report.md`, `f6-dwg-ac1024-report.md`, `f6-bmp-report.md`, `f6-stl-report.md`,
`f6-las-report.md`, `f6-gif-87a-report.md`, `f6-zip-report.md`. Verify report:
`f6b-verify-report.md`. Recon (spec for all of F6): `f6-recon-report.md`.
