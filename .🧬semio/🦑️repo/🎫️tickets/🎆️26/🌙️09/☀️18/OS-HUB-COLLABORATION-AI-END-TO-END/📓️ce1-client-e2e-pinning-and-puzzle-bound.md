# CE1 — `client-e2e` determinism, the puzzle bound, and the rest of the descriptor sweep

Slice owner: CE1, session 5e (2026-09-20 22:16 →). Spec: `📓️a3b-descriptor-sweep.md` (whole),
`📓️wr4-typed-command-dispatch-and-gates.md` §5.3/§5.4, `📓️ex1-extension-exports-and-describe-cliff.md` §3.

Three items:
1. `client-e2e` determinism — pin every dispatch step to an explicit artifact kind + verb, stage
   the component the gate needs, and REFUSE up front by name when it is missing or stale.
2. `🧩️puzzle` under the 4 MiB bound — lazy example sources in the framework.
3. The remaining non-current descriptors, cheapest first, with census + catalog diagnostics after.

Machine at 22:16: load 43.8, 1 `rustc`, fleet wasm mutex free, 44 GiB free.

---

## 0. Headline — measured only

1. **`client-e2e` 13/17 → 34/36** (`🗑️generated/ce1-client-e2e-final.txt`, 23:36, **32 s** on warm
   components; two reds, both named: the catalog's `puzzle`+`stdio` diagnostics, and `wfc`'s own
   `job.explicit-state-machine-required` refusal of its inference).
   The denominator grew because the journey no longer *returns* at
   `action_prepare`: eighteen rows — subscribe, invoke, the resource-updated push, unsubscribe,
   snapshot, live head, undo, redo, transaction begin/rollback, inference and the job rows — RAN for
   the first time in this gate's life. §1 names each of the four old reds.
2. **The gate no longer measures the BM25 weather.** Every dispatch row is pinned to
   `note.s.note.note@1/*#editor.addBlock` on `s.note.note`, and a new FIRST step refuses by name
   when a pinned component is missing or is not the build its committed descriptor describes (§2).
   It found a real one on its first run: `🗒️note`'s staged component was **newer than its
   descriptor** since WR4's 17:38 rebuild, and nothing in the tree said so.
3. **A product defect the pin exposed and this slice fixed**: `history_undo` left the revision stamp
   unchanged, because `RevisionStamp.head_edit_id` read the newest history row and an undo clears
   `applied` rather than removing the row. `expectedRevision` is the only revision oracle an agent
   has for a plugin-owned artifact, so an optimistic-concurrency client would have committed over
   somebody's undo without a `REVISION_CONFLICT` (§2b). One line at
   `🌉️mcp/🏠️workspace/🦀️.rs:1793`; undo and redo are green.
4. **`capabilities_search` keeps its own step, as a RANKING step** over the compiled catalog alone —
   ordered, filtered, unique ids, all `audience=agent`, pinned verb reachable (§3).
5. **The inference route was ABI-broken by component AGE, and a rebuild proved it.** `🌍️gis` (09-17)
   and `🀄️wfc` (09-18) both answered `inference instantiate: wasmtime: failed to convert function to
   given type`; after `🀄️wfc` was re-described on today's SDK that refusal is GONE and the guest
   answers a precise declaration fault instead — `job kind "semio.infer" has no admitted explicit
   bounded state machine` (§3b). The gate's last non-catalog red is now a named plugin defect.
   `job_get` and `job_cancel` run for real against that refused job.
6. **`🧩️puzzle` is NOT under the bound and the lazy `ExampleSource` was NOT landed** — §4 is a
   measured design finding, not a fix: the change the brief asks for is a WIRE-SHAPE change to
   `ExampleDefinition`, which regenerates the WIT bindings of every component in the tree.
7. **Catalog diagnostics stay at 2** (`puzzle`, `stdio`) — neither is reachable from this slice. But
   the **15 remaining stale descriptors were all swept** (§5, every one rc=0), taking descriptors
   regenerated today from **42 → 58 of 60**, and with them: capability descriptions **219 → 410**,
   declared `destructive` **44 → 82**, undeclared gesture routes **26 → 5**, audit findings
   **111 → 46** (§6).

## 1. `client-e2e` — the four reds, named

Baseline re-run at 22:17 (`bun ./📜️script.ts client-e2e` from `🌉️mcp/📦️packages/🟦️typescript`,
capture `🗑️generated/ce1-client-e2e-baseline.txt`): **13/17**, identical to A3b §6. The four:

| # | step | what it actually said |
| --- | --- | --- |
| 1 | `os: capability catalog health` | `4 diagnostic(s)` — `skipping plugin `puzzle`: … 🔣️.json did not decode as a PackageDescriptor` and `stdio: NotFound`, each counted twice (catalog-load + routing). |
| 2 | `os: artifact_create (a real plugin artifact kind)` | `kind=s.energy.model: NOT_FOUND — plugin `energy`'s compiled wasm is missing`. |
| 3 | `os: artifact_export` | `PLUGIN_UNAVAILABLE` on `mcp-client-e2e-…` — a CONSEQUENCE of red 2: the export step fell back to the gateway's own `os.agent.probe/v1` artifact, which by construction belongs to none of the 33 registered plugins. It was never a measurement of anything. |
| 4 | `os: action_prepare` | `energy.s.energy.model@1/*#editor.set-cell input={}` → the same `NOT_FOUND`. The journey `return`s here, which is why the denominator is 17 and not ~32. |

Reds 2–4 all descend from one fact: the journey took **hit 0 of a fuzzy
`capabilities_search({query:"set", kind:["mutation"]})`**. `capabilities_describe` on that hit
PASSED — the catalog types `energy` correctly — so nothing about the dispatch lane was measured.
Between WR4's 18:00 run and A3b's 22:0x run the same query's hit 0 moved `animate` → `energy` purely
because 20 more descriptors became decodable. The gate measured the BM25 weather.

## 2. The pin: explicit kind + verb, staged component, up-front refusal

All in `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts`.

**(a) Three constants, one plugin.** `CLIENT_E2E_PINNED_PLUGIN_ID = "note"`,
`CLIENT_E2E_PINNED_ARTIFACT_KIND = "s.note.note"`,
`CLIENT_E2E_PINNED_CAPABILITY_ID = "note.s.note.note@1/*#editor.addBlock"` — the exact guest and verb
WR4 §5.1 proved the two-phase prepare/apply law against, and the smallest of the rebuilt components
(64 MB vs `🌍️gis`'s 202 MB, the one that wedged an earlier run behind the 240 s wall). One plugin
per journey is also one cold component compile per journey, which is what WR4 §5.4's revert was
about.

**(b) `verifyStagedPluginComponent(repoRoot, pluginId)`** — a new exported function, and a new step
placed immediately after the channel pin, i.e. before the gate spends one request on a guest. It
reads the generated plugin registry for the plugin's `cratePath`/`wasmOut`, resolves the component
under `.🧬semio/🦑️repo/⚡️cache/cargo/target/wasm32-wasip2/{wasm-dev,wasm-release}` (the TypeScript
twin of the gateway's own `PLUGIN_WASM_TARGET_DIR`/`PLUGIN_WASM_PROFILE_DIRS`), and compares its
SHA-256 against the committed descriptor's `hashes.wasmSha256`. Three named refusals, each carrying
the command that fixes it; the journey returns on any of them.

It found a real defect on its first run — `note`'s staged component is **not** the build its
committed descriptor describes:

```
note: the staged wasm-dev/semio_s_plugin_note.wasm (64459968 B, sha256 46e12c0dfb01…) is NOT the
build its committed descriptor describes (63b37c41401a…) — the catalog would type this verb from a
descriptor that no longer describes the guest that runs it; re-describe it: …
energy: no compiled component is staged (tried …/wasm-dev/semio_s_plugin_energy.wasm, …)
animate: wasm-dev/semio_s_plugin_animate.wasm 104416649 B, sha256 ac2a2ed33b37… matches
```

i.e. WR4's 17:38 rebuild of `note`/`draw` left both components **newer than their descriptors**, and
nothing in the tree said so. `animate` matches because A3b re-described it at 20:33.

**(c) The dispatch rows.** `capabilities_describe`, `artifact_create (a real plugin artifact kind)`,
`artifact_export`, `action_prepare`, the saga member and all four `headRevision` re-reads now name
the pinned capability. `capabilities_describe` additionally asserts the catalog types it against the
pinned artifact kind, so a descriptor that retyped the verb is a named red rather than a silent
retarget. The `artifact_export` fallback to the gateway's own probe artifact is **removed** — a
failed typed create now returns instead of manufacturing red 3.


**(d) `🗒️note` was re-described** (`📜️a3-describe.sh 🗒️note`, 22:25 → 22:34, **546 s, rc=0**,
`🔣️.json` 197 877 → 197 877 B, pack 55 508 → **55 501** B) so the descriptor and the staged
component are one build again. The preflight then reads
`note: wasm-dev/semio_s_plugin_note.wasm 64 528 261 B, sha256 7cc1e0ed1125… matches`.

### 2b. What the pin then exposed — and two product defects it made visible

First pinned run (22:34 → 22:38, `🗑️generated/ce1-client-e2e-pinned1.txt`): **30/35**. The
denominator went 17 → 35 because the journey no longer returns at `action_prepare`: subscribe,
invoke, the resource-updated push, unsubscribe, snapshot, live head, undo, redo, transaction
begin/rollback, inference and the job rows all RAN for the first time. Three of the four original
reds are gone; two new ones appeared, both real:

1. **`artifact_snapshot` — `no such artifact: note`.** The step followed
   `revisionAfter.artifactId`, and in the headless lane that field is the **plugin id**:
   `🌉️mcp/🏠️workspace/🦀️.rs:1793` stamps `self.entry.plugin_id`. `action_prepare`/`action_invoke`
   drive the plugin's own live session document, while `artifact_create` persists a *separate*
   folder artifact seeded from it — so the step was snapshotting an id that names neither. Fixed in
   the gate by snapshotting the artifact the journey created, by the id it created it with, and the
   "the mutation landed" statement is left where it belongs, on `os: live head advanced` (a fresh
   `ReadHistory` against the document the mutation actually went to). The identity split itself is
   §8 gap 1 — it is the headless twin of WR4 §4 and bigger than this slice.
2. **`history_undo` did not move the revision stamp** — and this one is a product defect, fixed
   here. `HistoryPatch.upserts` is ordered newest-first and an undo does **not** remove its row, it
   clears `HistoryEntry.applied`; the stamp read `upserts.first()`, so a real `addBlock`, its undo
   and its redo all stamped `note@transaction:txn_…/1`. `expectedRevision` is the ONLY revision
   oracle an agent has for a plugin-owned artifact (the gateway's own `headRevision` docstring says
   so, and the shipped `history-undo` prompt tells agents to "confirm the result rather than
   assuming it"), so an optimistic-concurrency client would have committed over somebody's undo
   without ever seeing a `REVISION_CONFLICT`. **Fix:** `head_edit_id` is now the newest **applied**
   entry (`🏠️workspace/🦀️.rs:1793`). Measured after a rebuild of the binary (100 s, rc=0):

   ```
   PASS os: history_undo (mutation reverted) — members=1 warnings=none head note@/1 (baseline note@/0)
   PASS os: history_redo (mutation restored) — members=1 warnings=none head note@transaction:txn_5d4…/1
   ```

   The gate also stopped reading `members` as a success signal: `ActionAdapter::fan_out` returns
   `undo.members.len()` whether or not a member failed — per-member failures travel in `warnings`,
   and only an all-member failure is a tool error — so a fan-out where every member warned used to
   pass. Both rows now assert `warnings` is empty, and print it.

## 3. `capabilities_search` ranking, as its own catalog-only step

`capabilities_search` is no longer what picks the dispatch target. It has a step of its own, and it
is a **ranking** step read over the compiled catalog alone — no guest, no component, no artifact.
`capabilitySearchRankingVerdict(hits, mustReach, filteredArtifactKind)` (exported, next to the
preflight) asserts four properties of one reply to
`{query:"add a block to the note", kind:["mutation"], artifactKind:"s.note.note"}`:

| property | why a ranking owes it |
| --- | --- |
| scores monotonically non-increasing | a ranking that is not ordered is not a ranking |
| every `capabilityId` unique | two catalog rows carrying one id is the duplicate-id defect the catalog health line counts |
| every hit `audience=agent` | the catalog is an agent PROJECTION; a hit outside it is a leak |
| every hit's `artifactKind` equals the filter | the filter is honored rather than decorative |
| the pinned verb is present | the verb a client would look for is reachable by search at all |

Measured (both pinned runs, identical):

```
PASS os: capabilities_search ranking properties — 18 mutation hit(s) of `s.note.note`,
scores 6.824600030530695…0.05329356174636807 monotonically non-increasing, ids unique,
all audience=agent, `note.s.note.note@1/*#editor.addBlock` at rank 0
```

The journey's earlier free-text `capabilities_search` step (`query:"edit the document"`, no filter)
is untouched and still asserts only that the catalog answers a natural-language query at all.

### 3b. The inference target was the same defect, one tool along

`inference_list` returns the UNION of every installed plugin's declared roster and the journey took
`declared[0]` — which is `🌍️gis`, whose guest answered
`inference instantiate: wasmtime: failed to convert function to given type` (22:38). Only three
packages in the tree declare an inference at all (`gis` 1, `🀄️wfc` 5, `cad-extension-aec-building`
1 contributed), so the pin is `wfc`/`s.wfc.bitmap`/`s.wfc.bitmap.solve`; `wfc`'s component is
already staged and current (123 021 115 B, 09-18) and is verified by the same preflight, which now
checks **both** pinned plugins. `inference_list` asserts the roster CONTAINS the pinned service
instead of reading its first row.

Then the pin measured something better than a green row. Three runs, one probe
(`🐍️ce1-inference-probe.ts`, new — it drives `inference_list` + ONE named `inference_run` with a
wall clock, so a candidate never costs a whole 4-minute journey to reject):

| target | result |
| --- | --- |
| `gis` / `s.gis.gismap.inference` (the old `declared[0]`) | `INTERNAL: inference instantiate: wasmtime: failed to convert function to given type` |
| `cad-extension-aec-building` / `…building-structure-summary` | `NOT_FOUND … is declared for artifact kind s.cad.cad` in **1 s** — the contributed roster `inference_list` publishes is `gis` ×1 + `wfc` ×5 only; an extension's contributed inference never reaches it |
| `wfc` / `s.wfc.bitmap.solve`, COLD component | no answer within **900 000 ms** (23:08) |
| `wfc` / `s.wfc.bitmap.solve`, WARM component | `INTERNAL: inference instantiate: wasmtime: failed to convert function to given type` in seconds (23:18) |

The last two rows together are the finding: the 900 s was `wfc`'s 123 MB component being compiled by
wasmtime for the first time, and **once compiled its inference route fails exactly the way `gis`'s
does.** Two plugins, two different components (09-17 and 09-18), one identical ABI refusal — while
`🗒️note`, rebuilt today, dispatches through `action_prepare`/`action_invoke` without a murmur. So
this is a **component-age fault on the inference route**, not a per-plugin one and not a dispatch
one. The remedy is a rebuild of a declaring plugin on today's SDK, which is also a sweep item (§5).

The gate no longer *aborts* on it either: a `tools/call` that never answers used to reject out of
the journey, so the run printed no tally and no `FAIL` line at all — strictly worse than a named red
(WR4 §5.4's own words). The inference call now catches its own timeout and announces it as a row.

## 4. `🧩️puzzle` — the lazy `ExampleSource` is a WIRE-SHAPE change, and that is why it is not landed

**Not landed. `🔣️.json` is still 4 803 294 B.** What this slice adds is the reason the brief's
one-line framing ("make example sources lazy in the framework") does not close in one framework
change, established by reading the whole path rather than by trying and abandoning it.

The eager parse is real and is where A3 §3 put it: `📚️examples/🌙️capsule-dream/🦀️.rs:38` is
`static SOURCE: LazyLock<ExampleSource> = LazyLock::new(|| ExampleSource::new(ID, label(),
document_json(), ICON))`, and `document_json()` parses 3 035 200 B of DSL and re-serialises it. The
`LazyLock` is dereferenced by `subsets/✳️any/🦀️.rs:38`'s `examples()`, which builds the declaration
tree — so the parse happens while the bundle is being assembled, before `describe` starts.

Making `ExampleSource` hold `fn() -> String` instead of `String` is the easy half and it is not
enough. The body has to survive to the HOST:

| consumer | what it reads |
| --- | --- |
| `🛂️describe/🦀️.rs:52` `externalize_oversized_example_bodies` | `example.artifact_json.len()` — to decide inline vs asset — then `body.len()` and `sha256(body)` for the `AssetDeclaration` |
| `🎠️kernel/🟦️.ts:427` `scope contributions` | `example.artifactJson`, scanned for `neuron-kind=` |
| `📇️registry/✅️catalog-verification/🟦️.ts:43` | refuses an example row with neither `artifactJson` nor a declared example-body asset |
| the React `ShellHost` / the wgpu shell | the LIVE manifest's example bodies (`exampleArtifactSources`, `plugin.manifest.examples`) |

They all read one field, `ExampleDefinition.artifact_json: String`
(`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:3957`), which is a field of `PluginManifest` — a
`Serialize`/`ToValue` type that crosses the component ABI. `PluginManifest` is what
`__semio_describe_component` returns, so **the body is materialised by the act of serialising the
manifest**, at install and at describe alike. A deferred body therefore has to be expressible IN
`ExampleDefinition`, which means changing the shape of a WIT-projected manifest type: its TypeScript
twin (`🛂️manifest/🟦️.ts:1180`), its projection (`🧬️schema/📽️projection/🦀️.rs:596`), the generated
`🤖️manifest` bindings, and the `semio_s_plugin_*_component.js` bindings jco emits for every
component in `🧑‍💻dev/🧩️extension-modules/`. That is a tree-wide regeneration, not a framework edit,
and half of it landed would break every plugin.

EX1's own phrasing points at what the shape must be — *"body stays bytes, with its size/hash
declared at the source"*: the deferred variant has to carry the identity of the **authored source
bytes** (`DSL_TEXT`, already an `include_str!` static, 3 035 200 B), because `size_bytes`/`sha256`
over those bytes need no parse, while `sha256` over the derived JSON does. The asset a deferred row
declares is then the DSL source, not the JSON document — which is arguably the more honest
declaration, and is a second reason this is a shape decision rather than a refactor.

**One measurement this slice did NOT take, and the next owner should take first**: whether the 4.08 G
fuel A3b measured is the DSL parse at all. `export_capsule_dream_document_json_fixture`
(`🖐️5d/…/📸️snapshot/📝️text/🧪️tests/🔬️unit/🦀️.rs:54`) parses the same body natively. If that test is
seconds, a wasm parse cannot be 1 800 s and the cliff is somewhere else (a quadratic in the DSL
parser, or the DslValue→JSON re-serialisation); if it is a minute, laziness is the whole answer. It
was not run here because a cargo run would have collided with the describe sweep holding the fleet
wasm mutex (rule 7, one cargo at a time).

## 5. The descriptor sweep — remaining owners

Inherited state, recomputed rather than taken from A3b's prose: **60 registry plugins, 59 committed
descriptors** (`🗄️stdio` has none), **17 not regenerated on 09-20**. Two of the 17 are out of reach
(`🌍️gis` is GM1's and was never touched here; `🧩️puzzle` cannot be described, §4), leaving **15**,
run one at a time through the fleet mutex with A3's `📜️a3-describe.sh`, cheapest first. Every
`🔣️.json`/`🛂️.descriptor.semio` pair below is producer output; nothing was hand-edited.

| owner | rc | wall | `🔣️.json` before → after | pack before → after |
| --- | --- | ---: | ---: | ---: |
| `🗒️note` (§2d, the pinned component) | 0 | 546 s | 197 877 → **197 877** | 55 508 → 55 501 |
| `🀄️wfc` (§3b, the pinned inference) | 0 | 660 s | 964 457 → **772 000** | 232 043 → 195 720 |
| `🏭️process` | 0 | 166 s | 188 300 → **189 042** | 71 165 → 74 724 |
| `📏️layout` | 0 | 178 s | 199 861 → **164 002** | 51 661 → 47 626 |
| `📜️imperative` | 0 | 75 s | 205 066 → **141 509** | 48 158 → 37 561 |
| `🖨️raster` | 0 | 66 s | 229 357 → **158 884** | 54 960 → 43 938 |
| `🎥️shooting` | 0 | 107 s | 276 323 → **186 310** | 68 182 → 52 365 |
| `💠️lowpoly` | 0 | 111 s | 333 382 → **303 212** | 127 411 → 124 215 |
| `🪐️space` | 0 | 125 s | 454 019 → **379 541** | 114 418 → 101 122 |
| `🔋️energy` | 0 | 183 s | 471 401 → **478 143** | 192 817 → 200 456 |
| `📐️cad` | 0 | 130 s | 483 290 → **237 827** | 113 314 → 65 411 |
| `🌊️flow` | 0 | 94 s | 505 087 → **232 786** | 114 230 → 57 793 |
| `📸️remodel` | 0 | 231 s | 648 137 → **350 698** | 151 219 → 96 360 |
| `🏗️fem` | 0 | 352 s | 739 708 → **456 682** | 171 722 → 121 925 |
| `🌀️procedural` | 0 | 446 s | 1 241 651 → **834 074** | 279 397 → 199 916 |
| `🏛️architect` | 0 | 197 s | 1 302 865 → **392 710** | 243 337 → 106 985 |

**All 15 reachable owners landed, rc=0, 0 hand-edits.** Descriptors regenerated on 09-20/21 go
**42 of 60 → 58 of 60**; the only two that are not are `🌍️gis` (GM1's, untouched) and `🧩️puzzle`
(cannot be described), and `🗄️stdio` still has none.

Two things worth keeping:

- **Seven of the ten shrank**, `📜️imperative` by 31 % and `🀄️wfc` by 20 %: these generations replace
  09-17/09-18 ones that predate DS1's roster fix and A3b's declaration sweeps, the same effect A3b
  measured on `🔱️trinity`/`🎪️demonstrator`.
- **`🔋️energy` now has a staged component** as a side effect (its `describe` builds one), which is
  what the gate's three old dispatch reds were missing. It is no longer the gate's target, so this
  is housekeeping, not the fix — the fix was the pin.

## 6. Census + catalog diagnostics — **2 diagnostics, and the agent-facing numbers nearly doubled**

Same two instruments A3b used, so the rows are comparable: the staged `semio-os-mcp audit --folder
<repo>` (rebuilt here, §7) and A3's `🐍️m5a-descriptor-census.ts`. Captures
`🗑️generated/ce1-{audit-after,census-after}.txt`, both taken at 00:19 after the last describe.

| metric | A3b's close | CE1 | delta |
| --- | ---: | ---: | ---: |
| `[mcp registry] skipping plugin …` (one registry load) | 2 | **2** | 0 |
| the same over the catalog-load + routing pair (what `client-e2e` counts) | 4 | **4** | 0 |
| descriptors the audit decodes and inspects | 58 | **58** | 0 |
| registry rows the census decodes | 59 of 60 | **59 of 60** | 0 |
| capabilities in decodable descriptors | 1 943 | **1 967** | +24 |
| published to agents | — | **1 464** | — |
| with a description | 219 | **410** | **+191** |
| of those, EN+DE | 219 | **410** | +191 |
| with a declared `audience` | 66 | **115** | +49 |
| with `effects.destructive` | 44 | **82** | **+38** |
| gesture routes published to agents undeclared | 26 | **5** | **−21** |
| audit findings | 111 | **46** | **−65** |

The two surviving diagnostics are the two A3b named and neither moved, because neither is reachable
from here: `puzzle` (`🔣️.json did not decode as a PackageDescriptor: missing field artifactSchema`,
§4) and `stdio` (`NotFound: no committed descriptor` — GM1's).

What the sweep bought is the rest of the table: **the declaration sweeps A3b landed in source had
only reached 42 of 60 descriptors**, and running the remaining 15 through their own `describe`
doubled the agent-facing description coverage, nearly doubled the declared-destructive count, and
cut undeclared gesture routes to five (`puzzle` ×3, `block` ×1, `writer` ×1) and audit findings by
59 %. The five remaining gesture-risk rows and the 46 findings are declaration work in individual
plugins, not descriptor staleness.

## 7. Files changed

| file | change |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts` | `CLIENT_E2E_PINNED_{PLUGIN_ID,ARTIFACT_KIND,CAPABILITY_ID}` + `CLIENT_E2E_PINNED_INFERENCE_{PLUGIN_ID,ARTIFACT_KIND,SCHEMA}`; new `verifyStagedPluginComponent` (registry → descriptor `hashes.wasmSha256` → staged component SHA-256) and `capabilitySearchRankingVerdict`; a new first journey step that refuses by name; every dispatch row, the saga member and all four `headRevision` re-reads pinned; `capabilities_search` turned into a ranking step; `capabilities_describe` asserts the pinned artifact kind; `artifact_snapshot` reads the artifact the journey created; undo/redo assert `warnings`; the inference call catches its own timeout instead of aborting the journey; `jobId` read from a refused reply's `details` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs` | `RevisionStamp.head_edit_id` is the newest **applied** history entry (`:1793`) — the undo/redo revision defect |

Regenerated, never hand-edited: 16 `🔣️.json` + `🛂️.descriptor.semio` pairs (§5), each written by
`bun ./📜️script.ts describe` in its owner's own rust package. Ticket files: `🐍️ce1-inference-probe.ts`,
this report. Captures: `🗑️generated/ce1-{client-e2e-baseline,client-e2e-pinned1,client-e2e-pinned2,
client-e2e-pinned3,client-e2e-final,mcp-build,inference-probe-cad,inference-probe-wfc-rebuilt,
sweep-batch1,sweep-batch2}.txt` plus A3's shared `a3-describe-ledger.txt` / `a3-describe-<owner>.txt`.

## 8. Honest gaps

1. **`🧩️puzzle` is still 4 803 294 B and still cannot be described** (§4). The lazy `ExampleSource`
   is a wire-shape change to `ExampleDefinition`, i.e. a tree-wide WIT/jco regeneration; the design
   and the one measurement that should precede it are written down, nothing is half-landed.
2. **Catalog diagnostics are still 2** — `puzzle` (gap 1) and `stdio` (GM1's; nothing under
   `🗄️stdio`, `🌍️gis` or `.🧬semio/🌐hub/` was read, built or written here).
3. **`client-e2e` is 34/36, not 36/36.** Red 1 is gap 2. Red 2 is `🀄️wfc`'s own guest refusing its
   inference with `job kind "semio.infer" has no admitted explicit bounded state machine` — a
   declaration defect in a plugin this slice does not own, now reachable because the rebuild removed
   the ABI refusal that used to hide it.
4. **The headless lane's artifact identity is still split** and this slice only stopped the gate
   from lying about it: `artifact_create` persists a folder artifact while `action_prepare`/
   `action_invoke` drive the plugin's own session document, and `RevisionStamp.artifact_id` is the
   PLUGIN ID. An agent that follows the shipped `mutate-safely` prompt ("re-read the artifact and
   confirm the change landed") cannot: `artifact_snapshot` of the created artifact shows the
   pre-mutation bytes. This is the headless twin of WR4 §4 and is a design change, not a fix.
5. **The freshness oracle catches age, not ABI.** `verifyStagedPluginComponent` compares the staged
   component against the descriptor cut FROM it, so a plugin whose component and descriptor are both
   old agrees with itself — which is exactly how `🌍️gis` passed the preflight while its guest could
   not instantiate. The oracle is "descriptor describes this build", not "this build matches today's
   host ABI"; the latter needs a host-ABI version in the descriptor and nothing carries one.
6. **`🌍️gis` was deliberately not rebuilt**, so its `inference instantiate: wasmtime: failed to
   convert function to given type` is recorded, not fixed. `🀄️wfc`'s rebuild is the evidence that a
   rebuild is the remedy.
7. **Nothing here was proven against a live browser shell.** The measurements are the two-server
   `client-e2e` journey over real stdio, `🐍️ce1-inference-probe.ts`, the committed descriptors and
   the staged `semio-os-mcp` binary. `live-agent-loop-check` was not re-run, and the
   `RevisionStamp.head_edit_id` change is exercised by the gate's undo/redo rows only — it has no
   law of its own, and neither do the two new pure helpers (`verifyStagedPluginComponent`,
   `capabilitySearchRankingVerdict`), whose only exercise is the gate itself plus the ad-hoc calls
   recorded in §2b. Both are trivially unit-testable from `🌉️mcp/🟦️.ts`'s existing vitest region.
