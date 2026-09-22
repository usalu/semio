# PZ2 — `🧩️puzzle` describes inside the 8 G fuel budget, catalog diagnostics 0

Slice PZ2 of ticket 26/09/18, session 8 (2026-09-22 12:3x →). Predecessors:
`📓️pz1-catalog-zero-diagnostics.md` (the deferrable example body), `📓️ce3-four-mcp-gates-green.md`
§5 (the fuel cliff), `📓️ce1-client-e2e-pinning-and-puzzle-bound.md` §4.
Every number below comes from a capture in `🗑️generated/pz2-*`.

## 0. Headline

| | |
| --- | --- |
| **The catalog diagnostic is GONE** | `semio-os-mcp audit`: **0** `skipping plugin` lines, **60** descriptors inspected (was 1 / 59). `client-e2e`: `PASS os: capability catalog health — catalog compiled with zero skips and zero duplicate ids` |
| `🧩️puzzle`'s `describe` | **rc=0 at 5 793 699 458 fuel** — 72 % of the untouched 8 G budget, 1 314 026 ms (was: 8 G **exhausted** at 1 946 890 ms) |
| its committed descriptor | `🔣️.json` **4 803 294 → 985 501 B**; pack **4 295 257 → 244 927 B** |
| `DESCRIBE_FUEL_BUDGET` | **untouched at `8_000_000_000`** |
| where the fuel went | **measured natively, for the first time** (§2): not the example BODIES (PZ1 already deferred those) but three `ActionArgDef::select` option lists — 70 short strings — that derived their rows by parsing 403 991 B of authored DSL on the `AppDefinition` path |
| `create_puzzle5d_app()` | **338 ms → 2 ms** cold (native); 2d **157 → 1**; 3d **231 → 4** |
| the whole native describe path | **985 ms → 416 ms**, **−58 %**, medians of four interleaved cold runs each (§2.4); building all six `AppDefinition`s went 726 ms → **7 ms** |
| `capability-audit-check` findings | 29 → **1** (CA1's describes landed in the same window; the survivor is `norm`'s, not this slice's) |
| `client-e2e` | **35 / 37** (was 36/38): red 1 closed; `inference_run` is the only red family left and on the current `🀄️wfc` build it now HANGS past 240 s rather than refusing (§5.1) |
| a red the new laws found | a bare `addPartKind` — what an agent sends — added a part of the undeclared literal kind `"Part"`. Root-fixed, §3.2 |

## 1. Inherited state, verified not assumed

The brief asked whether the STAGED puzzle component contains PZ1's `puzzle5d_part_kind_options()`
fix. **It does**, and both halves of that were checked rather than assumed:

| | |
| --- | --- |
| staged `dist/component-dev/semio_s_plugin_puzzle.wasm` | **2026-09-22 02:44:48**, 119 583 124 B |
| PZ1's source edit | 2026-09-21, and the fix is in the working tree now — `🖐️5d/…/✏️editor/🦀️.rs:9683` unions `concrete-forest` + `nakagin` only, with PZ1's two reasons in its docstring |
| CE2's describe of that component | 04:13 → 04:51, **rc=1 `fuel exhausted` at 8 G / 1 946 890 ms** |

So CE3 §5's reading is confirmed and sharpened: **the fuel cliff survived the deferred example
bodies AND survived PZ1's `capsule-dream` fix.** Neither was wrong; both were incomplete.

Eight puzzle source files are newer than that component (peers' work landed after 02:44: CA1's
`action_destructive`/`action_audience` rows, and a peer's 2d `LazyLock` + 5d
`PUZZLE5D_EXAMPLE_OPERATIONS` deletions). The describe this slice queues rebuilds the component, so
it will carry all of it.

## 2. Native measurement — where the fuel goes

### 2.1 Instrument

`📜️ce2-profile-describe.sh` profiles the OWNED (wasm) describe, whose native stacks are wasmtime
frames — it cannot attribute guest work to guest functions. So the measurement was taken one level
down instead, on a NATIVE build of the same code: `pz2_describe_profile` /
`pz2_describe_total` / `pz2_describe_total_with_example_documents` in
`✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️surface/🦀️.rs`, which run exactly what the guest's
`__semio_describe_component` runs — `__semio_install_plugin_bundle()` → `plugin_manifest()` →
`describe::describe_plugin()` — and time each phase.

`CARGO_TARGET_DIR=…/⚡️cache/cargo/target-pz2`, `CARGO_INCREMENTAL=0`, no wasm32 build, **no fleet
mutex**. Captures: `🗑️generated/pz2-native-total-compare.txt` and `pz2-native-profile-after.txt`.

⚠️ **Capture honesty.** `🗑️generated` was wiped in the ~13:15 outage, taking this slice's first
round of captures with it. §2.4 and the post-fix breakdown were **re-run at 17:10** and are
captured. The PRE-fix per-phase numbers in §2.2 cannot be re-run without reverting §3, so they are
quoted as they were read in session, and the ONE of them the whole argument rests on —
`create_puzzle5d_app()` 338 ms cold vs ~1 ms warm — is independently corroborated by the after
numbers (1/4/2 ms cold, §2.4) and by the 985 ms ↔ 416 ms totals, both captured.

### 2.2 The first profile (12:41, load ≈ 40 — capture lost to the 13:15 wipe, see the note above)

| phase | ms | what it is |
| --- | ---: | --- |
| `example-5d-capsule-dream-document-json` | **4 610** | 3 035 200 B of DSL → 3 577 295 B of JSON — and **nothing on the describe path calls it any more** (PZ1's fix), which is why it is quoted here only for scale |
| `example-5d-nakagin-document-json` | 269 | 202 710 B of JSON out |
| `example-3d-nakagin-document-json` | 143 | 158 489 B |
| `example-2d-nakagin-document-json` | 123 | 124 280 B |
| `create-puzzle2d-app` | 157 | |
| `create-puzzle3d-app` | 231 | |
| `create-puzzle5d-app` | 338 | |
| the three viewers | 0 | |
| `plugin()` (statics warm) | 591 | three `declare_artifact`s, six mutation rosters, `try_build` preflight |
| `install-plugin-bundle` | 408 | |
| `plugin_manifest` | 453 | |
| `describe_plugin` | 46 | → **descriptor 244 725 B** |

Two readings fall straight out of that table and neither was in any previous report:

1. **The committed `🔣️.json` is 4 803 294 B and the descriptor this tree emits is 244 725 B** — a
   **19.6×** drop. PZ1's deferral IS landed and IS working; the committed file is simply the
   2026-09-19 one, because no describe has ever finished since.
2. **`create_puzzle5d_app()` cost 338 ms cold and 1 ms with the example statics warm**
   (`create-puzzle5d-app-warm`, `pz2-native-profile-2.txt`). The structural AppDefinition assembly —
   six apps, ~250 actions, every arg form, every window kind — is **~1 ms**. *All* of the per-app
   cost was one `ActionArgDef::select`.

### 2.3 The three selects

`pz2-native-profile-after.txt`'s `pz2-options` rows walk every `ArgSchema::String { options }` of the three editor
`AppDefinition`s. Exactly three option sets are example-derived, and they are small:

| app | action.arg | options | derived from |
| --- | --- | ---: | --- |
| `puzzle2d` | `addNode.kind` | **43** | `CONCRETE_FOREST_EXAMPLE_JSON` + `NAKAGIN_EXAMPLE_JSON`, re-parsed to `serde_json::Value` |
| `puzzle3d` | `addObjectKind.objectKind` | **14** | `CONCRETE_FOREST_EXAMPLE_FIXTURE` + `NAKAGIN_EXAMPLE_FIXTURE` (typed `Puzzle3dFixture`) |
| `puzzle5d` | `addPartKind.partKind` | **13** | `CONCRETE_FOREST_EXAMPLE_DOCUMENT` + `NAKAGIN_EXAMPLE_DOCUMENT` (typed `Puzzle5dDocument`) |

**70 short strings, in which every label equals its id**, bought by parsing 403 991 B of authored
DSL, re-serialising it to JSON and deserialising that into six typed/loose documents — on the
`AppDefinition` path, which is the `describe()` path and also every actor boot in a live shell.
That is the cliff PZ1's fix uncovered by removing the bigger one in front of it.

### 2.4 Before/after, measured the same way

`pz2_describe_total` runs the guest's own sequence in a process where nothing has warmed a static;
`pz2_describe_total_with_example_documents` forces the same six example documents first, which is
precisely what the old selects did while `plugin()` ran. Four interleaved runs of each, each in its
own process via `-- --exact <name>` (`pz2-native-total-compare.txt`, 17:10, load ≈ 35):

| | run 1 | run 2 | run 3 | run 4 | median |
| --- | ---: | ---: | ---: | ---: | ---: |
| with the example documents (pre-PZ2 shape) | 1 099 | 871 | 577 | 1 185 | **985 ms** |
| this slice's tree | 408 | 424 | 373 | 470 | **416 ms** |

**−58 %**, and the `force-example-documents` phase alone (164–540 ms) is the whole difference.
The post-fix breakdown in the same window (`pz2-native-profile-after.txt`) shows why there is
nothing left to shave on the puzzle side of it:

| phase | ms |
| --- | ---: |
| `create-puzzle2d-app` (cold) | **1** |
| `create-puzzle3d-app` (cold) | **4** |
| `create-puzzle5d-app` (cold) | **2** |
| the three viewers | 0 |
| `install-plugin-bundle` | 260 |
| `plugin-manifest` | 223 |
| `describe-plugin` | 30–160 → **descriptor 244 725 B** |

Building all six `AppDefinition`s is now **7 ms**, against 726 ms before (§2.2), and not one example
document is materialised on the path. Applied to a run that exhausted 8 G, −58 % predicts ≈ 3.4 G;
the real run came in at **5.79 G** (§4), which says the guest charges relatively more for the
allocation-heavy manifest clone than a native debug build does — the honest reading of the gap
between the two numbers, and the reason §7.5 names that clone as the next lever.

What is left, and who owns it: `plugin_manifest` is roughly half of what remains, and it is
`Plugin::manifest` = `self.manifest.clone()`
(`🧰️framework/…/🔌️plugin/🦀️.rs:32309`) — a deep clone of all six `AppDefinition`s so that
`plugin_descriptor` can mutate one field of it. That is a framework-owned win of roughly the same
size as this slice's, and it is **not** taken here: `🔌️plugin` is FP10's file this session.

## 3. Root fixes in `🧩️puzzle`'s source

Each editor now declares the shipped examples' kind rows as an authored `const`, and the select is
built from it. The derivation is not deleted — it moves into a LAW that pays the parse once, in a
test, where a slow parse costs nothing.

| file | change |
| --- | --- |
| `🗿️artifacts/◻️2d/…/✏️editor/🦀️.rs` | `PUZZLE2D_SHIPPED_NODE_KINDS` (43 rows); `puzzle2d_node_kind_options()` maps it |
| `🗿️artifacts/🧊️3d/…/✏️editor/🦀️.rs` | `PUZZLE3D_SHIPPED_OBJECT_KINDS` (14 rows); `puzzle3d_object_kind_options()` maps it |
| `🗿️artifacts/🖐️5d/…/✏️editor/🦀️.rs` | `PUZZLE5D_SHIPPED_PART_KINDS` (13 rows); `puzzle5d_part_kind_options()` maps it |
| each editor's `🧪️tests/🔬️unit/🦀️.rs` | `shipped_{node,object,part}_kinds_are_the_two_examples_own_catalog_rows` — derives the list from the example documents exactly as the select used to and asserts equality, so the authored rows can never silently drift |

The per-artifact `…_OPTIONS_MAX = 64` caps stay and are still applied (`.take(MAX)`), in the const
path and in the law, so the bound the old code carried is not lost.

`cargo check -p semio-s-artifact-puzzle-{2d,3d,5d} --features component-app-assembly --all-targets`
→ **rc=0**, 27/148/3 warnings (capture lost to the 13:15 wipe; superseded by §3.1's test runs, which compile the same targets); the warnings are the proof a real
type-check ran, and `--all-targets` is what compiles the three new laws.

### 3.1 The laws, RUN

`CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-pz2 cargo test -p semio-s-artifact-puzzle-<n>
--features component-app-assembly --lib`, at load 23:

| crate | filter | result | capture |
| --- | --- | --- | --- |
| `semio-s-artifact-puzzle-5d` | `kinds part_kind brush` | **38 passed, 0 failed**, 1.20 s | `pz2-law-5d.txt` |
| `semio-s-artifact-puzzle-3d` | `kinds object_kind` | **24 passed, 0 failed**, 0.12 s | `pz2-law-3d.txt` |
| `semio-s-artifact-puzzle-2d` | `kinds node_kind` | **46 passed, 0 failed**, 0.05 s | `pz2-law-2d.txt` |

### 3.2 A red the laws found, and its root fix

The FIRST 5d run was **33 passed / 1 failed**:
`add_part_kind_materializes_the_declared_kind_default` read back `Some("Part")` where the declared
default is `Some("Hexagonal Cut Concrete Forest Left")`.

**It is not §3's doing, and that is provable rather than asserted**: the test's own guard
`assert!(expected_default != "Part")` PASSED, i.e. `puzzle5d_part_kind_options()` still yields the
same first row it always did (the const was generated from the pre-change derivation's own output,
the run whose `pz2-options` rows §2.3 quotes); and the value that came back is produced by
`Puzzle5dAddBrushPartWork::owned_part_kind`, which never consults the option list at all.

The root is one literal: `owned_part_kind` fell back to the string `"Part"` — exactly the literal
the select stopped hardcoding — whenever the dispatch carried no `partKind`, which is what a bare
`addPartKind` from an agent looks like. So the MCP-facing arg form added a part of a kind no catalog
declares. Fixed to fall back to **this tool's own declared default**
(`addPartKind` → the catalog default; `addBrushPart`, whose select is a one-row `"Part"`, stays
byte-identical), and the run is now 38/38 green.

## 4. The describe — `🧩️puzzle` finishes at **5.79 G of the 8 G budget**

**It landed, and the budget was never touched.** The run was not mine: the peer "Semio-tech play"
session held the fleet wasm mutex at stamp `…110250` and described `🧩️puzzle` as part of its own
activation batch at **15:11 → 15:34**, from a tree that already carried §3's three consts. Its
capture is
`🎫️tickets/🎆️26/🌙️09/☀️19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP/🗑️generated/activation/steps/describe-puzzle-151158.txt`.

| | CE2's run, 2026-09-22 04:13 (pre-PZ2 source) | the 15:11 run (post-PZ2 source) |
| --- | ---: | ---: |
| component | 119 510 521 B | 120 342 649 B |
| final fuel | **8 000 000 000 — exhausted** | **5 793 699 458** |
| guest wall | 1 946 890 ms, then `fuel exhausted` | 1 314 026 ms, then **rc=0** |
| `🔣️.json` | 4 803 294 B (2026-09-19, stale) | **985 501 B** |
| `🛂️.descriptor.semio` | 4 295 257 B (stale) | **244 927 B** |
| whole `describe` verb | rc=1 | **22 m 29 s, rc=0** |

**72 % of `DESCRIBE_FUEL_BUDGET`, 2.2 G of headroom**, and the emitted pack (244 927 B) agrees with
the 244 725 B this slice measured natively (§2.2) to within the hashes the emitter patches in
afterwards. Fuel fell by **at least 27 %** — "at least", because the pre-PZ2 number is a floor, not
a total: that run was killed at 8 G and nobody knows what it would have needed.

The committed descriptor was read back and it carries exactly what §3 authored — this is the check
that the fix, and not something else, is what landed:

| | |
| --- | --- |
| `s.puzzle.puzzle2d@1/*#editor` `addNode.kind` | **43** options, first `Hexagonal Cut Concrete Forest Left` |
| `s.puzzle.puzzle3d@1/*#editor` `addObjectKind.objectKind` | **14**, first two the two `Hexagonal Cut Concrete Forest` rows |
| `s.puzzle.puzzle5d@1/*#editor` `addPartKind.partKind` | **13**, first `Hexagonal Cut Concrete Forest Left` — and **no catalog UUID**, PZ1's law holding in the shipped artifact |
| all **7** example rows | `artifactJson` length **0** — deferred, PZ1's channel working end to end |
| `assets` | **7** declared example bodies, by their AUTHORED DSL size: `capsule-dream.dsl.semio` **3 035 200 B**, `puzzle5d/nakagin` 168 355, `puzzle3d/nakagin` 128 755, `puzzle2d/nakagin` 93 779, three small `concrete-forest` rows |

And the three authored lists were diffed against the shipped descriptor element by element, id and
`label.native.en` together, not just counted:

```
PUZZLE2D_SHIPPED_NODE_KINDS:   const=43 shipped=43 identical=True
PUZZLE3D_SHIPPED_OBJECT_KINDS: const=14 shipped=14 identical=True
PUZZLE5D_SHIPPED_PART_KINDS:   const=13 shipped=13 identical=True
```

That is the strongest statement this slice can make without the laws having run (§7.4): the guest
that produced the committed descriptor built these selects from §3's consts, and the option sets
the agent catalog now publishes are exactly the ones the derivation used to produce.

**PZ2's own mutex ticket was cancelled rather than spent.** It was queued behind `ca1` and would
have re-described an already-current `🧩️puzzle` for ~22 minutes of exclusive fleet time; the
descriptor on disk is byte-for-byte what this slice's source produces, so the run would have bought
nothing. `kill -9` on pid 30246 and its queue file, nothing else — `ca1`'s hold was not touched.

## 5. `client-e2e`

Baseline on THIS tree, against CE3's live serve **6196** (200) and hub **7621** (`readyz` 200) —
neither restarted, neither is mine: `bun ./📜️script.ts client-e2e` from
`🌉️mcp/📦️packages/🟦️typescript` → **36 / 38**, same two reds as CE3's
(13:05; that capture was lost to the wipe — the post-describe runs below are the ones on disk).

### 5.1 Red 1 — `os: capability catalog health`

`2 diagnostic(s)`, both the same line emitted once per registry compile: `skipping plugin 'puzzle':
🔣️.json did not decode as a PackageDescriptor: missing field 'artifactSchema' at line 1 column
6451`. This is §4's describe and nothing else, and after it:

```
PASS  os: capability catalog health — catalog compiled with zero skips and zero duplicate ids
```

(`pz2-client-e2e-{2,3}.txt`). **Red 1 is closed.**

The two post-describe runs did not produce a clean tally, and the reason is the machine, not the
product — both ran while the fleet drove the 1-minute load to **267–300** with swap at 8.4/9.2 GB:

| run | tally | what happened |
| --- | --- | --- |
| run at ~16:05 (capture lost) | no tally | catalog health **PASS**; the journey then threw at `action_prepare`: `tools/call did not answer within 240000ms` |
| run at ~16:10 (capture lost) | **34 / 37** | catalog health **PASS**; `inference_run` timed out at 240 s instead of answering its usual refusal, taking `inference job progress` and `notifications/cancelled` down with it |

| `pz2-client-e2e-4.txt` | **35 / 37** | the clean run, at load 23: catalog health **PASS**, every mutation/undo/redo/transaction row **PASS**; `inference_run` times out at 240 s and `inference job progress` cascades from it |

**35 / 37, and the denominator moved for a reason worth naming.** `inference_run` no longer answers
CE3's fast refusal (`bitmap-inference-wire-decode:missing field 'snapshot'`) — it **does not answer
at all inside the journey's 240 s budget**, so no `jobId` is minted and the `inference cancel` step
never runs (38 → 37). That is CE1's 2026-09-20 shape returning ("did not answer within 900 000 ms"),
and it arrived with `🀄️wfc`'s rebuild: the pinned component is now
`semio_s_plugin_wfc.wasm` **129 198 650 B, sha256 5981dfb5eb73…** where CE3 measured 128 436 995 B /
`7544e0b01fba…`, and the freshness step passes on both. So the red moved from "the guest refuses a
payload it cannot decode" to "the guest does not come back", under `🀄️wfc`, between CE3's run and
this one. Not this slice's code and not this slice's plugin — but it is what stands between the
journey and 38/38 today, and §5.2's design gap sits behind it.

### 5.2 Red 2 — `os: inference_run` is a DESIGN gap, and this slice adds one fact to it

CE3 §3.3 located the refusal: `job.infer.dispatch: tool factory 'semio.infer' rejected
's.wfc.bitmap.solve': bitmap-inference-wire-decode:missing field 'snapshot'`, and named the gap as
"no payload contract is published anywhere an agent can read it". Reading the whole route confirms
that and sharpens it in two ways — **and this slice deliberately did NOT make the row green**,
because every way to do that today is either a fake payload in the gate or a half-landed
declaration change across all 59 committed descriptors:

1. **The contract exists in the guest and is simply not published.** `BitmapInferenceJobFactory`
   already declares `fn payload_schema_id(&self) -> &str { BITMAP_INFERENCE_PAYLOAD_SCHEMA }` =
   **`"s.wfc.bitmap.inference.request.v1"`**
   (`🖼️bitmap/…/💡️inferences/🦀️.rs:45`, `:704`). The type that CROSSES into the descriptor,
   `ArtifactInferenceDescriptor` (`🧬️schema/⚛️component/🦀️.rs:444`), is `{ id, inference:
   FacetLeaves }` — there is no field for it. So the publication gap is one declaration field wide,
   not a missing contract.
2. **Even published, a schema id would not make the row green**, and that is the deeper half:
   `InferCommand` (`🌉️mcp/🔀️dispatch/…`) carries `plugin_id`, `artifact_kind`,
   `inference_schema`, `revision`, `generation`, `cancellation_id`, `work_units`,
   `canonical_payload` — **and no artifact binding at all** (`INFERENCE_ROUTED_INSTANCE = 0`,
   `💡️inference/🦀️.rs:1718`). `inference_run`'s tool schema has no `artifactId` either. So the
   guest has no document to read a `snapshot` from, and a generic MCP client has no way to
   synthesise 4 096 bitmap cells. An agent asking "solve this bitmap" needs `inference_run` to
   name an artifact and the guest to build its own canonical request from that artifact's
   document — which is a packet (gateway tool schema + plugin SDK entry point + one plugin), not a
   red-fix.

Adding a payload field to `ArtifactInferenceDescriptor` would also mean re-describing every plugin
before `inference_list` could answer it for more than `🀄️wfc`, which is exactly the
2-diagnostics-to-59 trade PZ1 §2.1 measured and refused.

## 6. `capability-audit-check` / catalog diagnostics

Baseline on THIS tree (`bun ./📜️script.ts capability-audit-check` from `🌉️mcp/📦️packages/🦀️rust`,
the staged `semio-os-mcp`, 13:10 — capture lost to the 13:15 wipe, and identical to CE3's own `ce3-audit-{1,2}.txt` which survive):

```
semio-os-mcp audit: 29 finding(s) over 59 descriptor(s) under /Users/ueli/Documents/semio
[mcp registry] skipping plugin `puzzle`: … 🔣️.json did not decode as a PackageDescriptor:
  missing field `artifactSchema` at line 1 column 6451
```

**29 findings / 1 catalog diagnostic**, identical to CE3's two runs — this slice changed no
gateway code and no committed descriptor before the describe, so an unchanged number is the right
result here.

**After the describe** (`pz2-audit-after.txt`, 16:0x, same command, same staged binary):

```
semio-os-mcp audit: 1 finding(s) over 60 descriptor(s) under /Users/ueli/Documents/semio
```

| metric | CE3 | PZ2 | delta |
| --- | ---: | ---: | ---: |
| `[mcp registry] skipping plugin …` | 1 | **0** | **−1 — the target** |
| descriptors the audit decodes and inspects | 59 | **60** | +1 |
| audit findings | 29 | **1** | −28 (CA1's declarations, not this slice's) |

`grep -c "skipping plugin" pz2-audit-after.txt` → **0**. The single surviving finding is
`norm.s.norm.din4108@1/*#editor.setSnapshot … WhenDestructive never fires`, which is CA1's
remaining source work.

## 7. Honest gaps

1. **The winning describe was not run by this slice** (§4). The peer "Semio-tech play" session's
   activation batch ran it at 15:11 from a tree that already carried §3, and PZ2's own queued
   mutex ticket was then cancelled rather than spent on a 22-minute re-run of an already-current
   descriptor. The fuel number is therefore read from THAT capture, named in full in §4, not from
   one of mine. What is mine and verified is the descriptor's content (§4's table) and the native
   before/after (§2.4).
2. **The pre-PZ2 fuel total is unknown.** 8 G is a floor: CE2's run was killed at the cap, so
   "fuel fell by at least 27 %" is the honest statement, not "by 27 %".
3. **`client-e2e` is 35/37, not 38/38.** Red 1 is closed; what is left is `inference_run`, which on
   the current `🀄️wfc` build does not answer inside 240 s at all (§5.1's table) — a regression under
   somebody else's plugin between CE3's run and this one — with §5.2's design gap behind it. No
   payload was hand-written into the gate to buy the row, and the gate's assertions were not
   touched.
4. **The staged `🧩️puzzle` component is now one commit behind its source.** §9's `owned_part_kind`
   fix landed AFTER the 15:11 describe. It changes runtime behaviour only — no manifest, no
   declaration — so the committed descriptor stays correct and no re-describe is owed; the next
   `component dev/release` picks it up.
5. **The remaining half of the describe cost is framework-owned.** `Plugin::manifest` is
   `self.manifest.clone()` (`🧰️framework/…/🔌️plugin/🦀️.rs:32309`), a deep clone of six
   `AppDefinition`s so `plugin_descriptor` can mutate one field of the copy: ≈ 400 ms of the 750 ms
   that is left, and allocation-heavy work is exactly what the owned guest charges most for. With
   2.2 G of headroom nothing forces it now; it is the next lever if another artifact lands under
   `🧩️puzzle`. Not taken here — `🔌️plugin` is FP10's file this session.
6. **A deferred example body is declared, never fetched.** PZ1 §6.5's gap stands: the seven
   `assets` rows in §4's table carry each example's authored DSL size and SHA-256 and nothing in
   the tree resolves an example body from `assets` yet.

## 8. Files changed

| file | change |
| --- | --- |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | `PUZZLE2D_SHIPPED_NODE_KINDS` (43 authored rows) + `puzzle2d_node_kind_options()` maps it |
| `…/◻️2d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | `shipped_node_kinds_are_the_two_examples_own_catalog_rows` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/✏️editor/🦀️.rs` | `PUZZLE3D_SHIPPED_OBJECT_KINDS` (14) + `puzzle3d_object_kind_options()` maps it |
| `…/🧊️3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | `shipped_object_kinds_are_the_two_examples_own_catalog_rows` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/…/✏️editor/🦀️.rs` | `PUZZLE5D_SHIPPED_PART_KINDS` (13) + `puzzle5d_part_kind_options()` maps it; `puzzle5d_part_kind_rows` MOVED out of the lib (the `cargo check` warning `function 'puzzle5d_part_kind_rows' is never used` is how that was caught — it now exists only where it is used, as the law's own derivation) |
| `…/🖐️5d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | `puzzle5d_part_kind_rows` + `shipped_part_kinds_are_the_two_named_examples_own_catalog_rows` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/…/✏️editor/🦀️.rs` (`Puzzle5dAddBrushPartWork::owned_part_kind`) | §3.2 — a bare `addPartKind` no longer adds a part of the undeclared literal kind `"Part"`; each tool falls back to its OWN declared select default |
| `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️surface/🦀️.rs` | **unchanged at close** — the native describe profiler lived here while §2 was measured and was lifted back out into `🐍️pz2-describe-profile.rs` (ticket folder, not compiled) once it had done its job: it asserts nothing, so it is not a law and does not belong in the crate. Its header says exactly where to paste it and what to run. |

Ticket files: `🐍️pz2-describe-profile.rs` (the §2 profiler), `📜️pz2-describe-puzzle.sh` (ordered-mutex hold + ONE automatic re-queue when the hold
died in <120 s, i.e. on a peer's compile break — PZ1 lost a 98-minute hold to exactly that),
`📜️pz2-describe-once.sh` (the describe body + ledger row), this report.
Captures on disk: `🗑️generated/pz2-{native-total-compare,native-profile-after,law-2d,law-3d,law-5d,
audit-after,client-e2e-4,describe-ledger}.txt`, plus the peer capture named in §4. The ~13:15
outage wiped `🗑️generated`; §2's totals, the audit and `client-e2e` were re-run at 17:10 rather
than quoted from memory, and every number that could not be re-run is marked where it appears.

No framework file, no `🌎️hub` file and no other plugin was touched.
