# TC1 — trusted catalog to `artifactAuthority` ready

Slice TC1 (session 5b/6, 2026-09-20). Owner: the critical path of outcomes 2 and 3 — a hub whose
`artifactAuthority` is ready so two users can open ONE shared document.

## 0. HUB HANDOFF (top of report, for the collaboration worker)

| field | value |
|---|---|
| `/readyz` 200 | **NO** — no hub is running, because no catalog is published (§4c). |
| what IS done | **the blocker this slice was given is gone**: the gis closed browser actor now builds. Two runs took `derive` to `8/8`, and run 2's generation is on disk with a real **63 584 335-byte `closed-actor.mjs`** at `.🧬semio/🌐hub/tc1-boot/trusted-catalog/generations/d1ac9205…/packages/gis/browser/`. Nobody had ever produced that file. |
| what blocks `/readyz` | the mandatory pre-publication law `genuine_gis_component_cold_loads_and_patches_the_exact_tiled_map_surface` is **red** (`map scene omits cold-before`) — a `🗺️gismap` rendering matter, not this slice's, and unskippable by design (§4c). |
| resume, one line | `cd /Users/ueli/Documents/semio && nohup zsh ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/📜️tc1-hub-boot.sh" 7611 > /dev/null 2>&1 & disown` — it publishes AND boots the hub in one go the moment that law is green. ~37 min materialise + ~6 min proof. |
| port | 7611 (free) |
| data root | `.🧬semio/🌐hub/tc1-boot` |
| port | 7611 |
| data root | `.🧬semio/🌐hub/tc1-boot` |
| publish command | `nohup zsh .🧬semio/…/OS-HUB-COLLABORATION-AI-END-TO-END/📜️tc1-hub-boot.sh 7611 > /dev/null 2>&1 & disown` |
| live log | `🗑️generated/tc1-hub-dev.txt` |
| hub pid, once up | `🗑️generated/tc1-hub-pid.txt` |
| readiness body | `🗑️generated/tc1-hub-readyz.txt` |
| binary | `⚡️cache/cargo/target-tc1/debug/os-hub`, built by the bootstrap itself — **a hub built before this change cannot serve this catalog** (§3 item 4) |
| second hub | copy `<tc1-boot>/trusted-catalog/` into the second data root, `chmod 700`, realpath, never share a root (DS1 §10.9) |

## 1. Inherited state (measured)

- Both `wasm-release` components from DS1's session are on disk and **reused, not rebuilt** as the
  measurement subject of §2: `⚡️cache/cargo/target-ds1/wasm32-wasip2/wasm-release/semio_s_plugin_gis.wasm`
  (47 272 589 B, 08:32) and `…/semio_s_plugin_stdio.wasm` (47 496 132 B, 07:30).
- `.🧬semio/🌐hub/ds1-boot/trusted-catalog/` is an **empty** private directory — DS1's staging was
  cleaned when the run failed, so no generation was ever published there.
- `🗑️generated/ds1-hub-dev.txt` ends at the 14:40 failure quoted in DS1 §10.12b. Port 7611 free,
  42 GiB free on `/System/Volumes/Data`.
- No hub process of DS1's survives.

## 1b. Verification that ran (not "written, not run")

| check | command | result |
|---|---|---|
| jco import measurement | `bun 🐍️tc1-actor-imports.ts <component>` | captures `tc1-gis-imports.txt`, `tc1-stdio-imports.txt`, `tc1-gis-imports-admitted.txt` |
| TypeScript, os product | `tsc -p 🧰️framework/🛍️products/💻️os/tsconfig.json --noEmit` | **0 errors in any file TC1 touched** (48 remaining lines are peers' pre-existing errors elsewhere) |
| TypeScript, hub package | `tsc -p 🌎️hub/📦️packages/🟦️typescript/tsconfig.json --noEmit` | 0 lines matching browser/wasi/interfaces |
| the new laws | `testBrowserWasiActivation(repoRoot)` | **PASS** — `🗑️generated/tc1-wasi-activation.txt`: `AJV=1 TypeScript=1 Preview2=1 actors=2 laws=17 resources=256 waiters=128`. This includes the test's own strict `ts.createProgram` pass over the edited shim. |
| Rust twin compiles | `cargo check -p semio-framework-os-kernel --lib` | `Finished dev profile in 4.51s`, warnings emitted (so expansion really ran) |
| hermeticity of the peer's `write: false` removal | `Bun.build` with no `outdir`, no `write` | nothing written to disk — the evidence directory is still written only by `writeFileSync`; **no revert needed** |
| the whole browser-bundle law suite | `testClosedBrowserComponentFactory(repoRoot)` | **exit 0, all 8 groups** — `🗑️generated/tc1-browser-bundle-laws.txt`. Includes `browser-actor-factory: AJV=1 JCO=1 native-Wasm-oracle=1 … artifact-laws=20 bytes=115698`, i.e. `buildClosedBrowserActorArtifactV1` built and closed a real actor with the changed closure template. |

**A FIFTH copy of the bound, found only because that suite was run** (it would have killed the
bootstrap an hour in, at the policy seal): `🌐️browser-bundle/🧬️schema/🔣️.json` caps
`importInterfaces` at 16 in three places — `:26` (`ActorImportV1`), `:243`
(`BrowserActorCodegenManifestV1`, with a regex that **enumerates every admitted name**) and `:317`
(`CodegenPolicyV1.options`). All three are now 18, and the `:243` regex admits
`clocks/(monotonic-clock|wall-clock)@0.2.0` and `wasi:random/insecure-seed@0.2.9`. So the vocabulary has
**six** copies in all (TS allowlist, shim record, directory-schema TS, directory-schema Rust,
directory-schema JSON, browser-bundle JSON) — the §3 law ties the first four; the two JSON documents are
tied by the suite above.

## 2. The blocker — `browser actor artifact: unsupported import interface`

**Measured, not inferred.** Probe `🐍️tc1-actor-imports.ts` runs jco 1.27.0's `generate` with exactly the
policy options `buildClosedBrowserActorArtifactOwned` uses (`instantiation: async`, `jspi`,
`base64Cutoff: 0`, `map` = the frozen allowlist) against the two `wasm-release` components DS1 built
today, and prints `output.imports` minus the allowlist. Captures:
`🗑️generated/tc1-gis-imports.txt`, `🗑️generated/tc1-stdio-imports.txt`.

```
gis   (47 272 589 B, 08:32)  unexpected = ["wasi:clocks/wall-clock", "wasi:random/insecure-seed"]
stdio (47 496 132 B, 07:30)  unexpected = ["wasi:clocks/wall-clock", "wasi:random/insecure-seed"]
```

**The two imports are IDENTICAL for stdio and gis, so this is not a gis-specific accident and not the
peer's `✏️s/🔌️plugins/🌍️gis/` feature-gate edit.** They are the baseline `wasm32-wasip2` Rust `std`
import set: `std::time::SystemTime` → `wasi:clocks/wall-clock`, and `HashMap`'s `RandomState`
(`hashmap_random_keys`) → `wasi:random/insecure-seed`. Every Rust plugin component has them, so the
closed browser actor stage could never have admitted ANY real plugin — the allowlist was written against
a synthetic fixture component (`🧫️fixtures/🌊️actor-import/🔣️.json`), which has neither.

**The exact wire names** (read off the generated `browser-actor.js` import destructuring, not guessed —
jco normalises the reported name to whatever the `map` entry spells, so the allowlist string decides the
shim key):

| interface | generated destructure | functions needed |
|---|---|---|
| `wasi:clocks/wall-clock@0.2.0` | `const { now: now$1 } = imports['wasi:clocks/wall-clock@0.2.0']` | `now() -> { seconds: bigint, nanoseconds: number }` |
| `wasi:random/insecure-seed@0.2.9` | `const { insecureSeed } = imports['wasi:random/insecure-seed@0.2.9']` | `insecureSeed() -> [bigint, bigint]` |

Those two strings are **exactly** what the repo's own native plugin host already admits for the same two
interfaces — `🔌️plugin/🖥️host/🦀️.rs:1548` (`wall-clock@0.2.0`), `:1540` (`insecure-seed@0.2.9`) and
`🔌️plugin/🧠️interpreter/🦀️.rs:910`. Admitting them makes the wasm actor and the native host name the
same vocabulary; the version skew (`0.2.0` vs `0.2.9`) is the component's own, measured from the core
module import object (`'wasi:clocks/wall-clock@0.2.0'` / `'wasi:random/insecure-seed@0.2.9'`).

With those two admitted, `unexpected = []` for gis (`🗑️generated/tc1-gis-imports-admitted.txt`).

## 3. Decision and fix

**Admit the two exact names; do not widen, do not touch the gis plugin.** They are not an accidental
dependency import (stdio, which has no map/terrain/native-codec surface at all, imports exactly the same
two), so removing them "at the source" would mean removing `SystemTime` and `HashMap` from Rust `std` —
there is no source to remove them from. The closed browser actor genuinely needs both, and the shim set
did not implement either, so they are implemented here rather than stubbed.

Landed (file + line):

1. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi/🟦️.ts`
   - `:50` `browserWasiInterfaces` += `wasi:clocks/wall-clock@0.2.0`, `wasi:random/insecure-seed@0.2.9`
     (14 → 16; with the two `semio:framework/*` names the actor allowlist is 18).
   - `:2` `BrowserWasiPort` += `wallNs(): bigint` — the wall clock is injected by the host exactly as
     `nowNs` is, so the activation keeps its "no ambient authority" property and stays testable.
   - `wall()` reads `port.wallNs()` through the same `u64` admission and splits it into the WIT
     `datetime` (`{ seconds, nanoseconds }`); `resolution()` reports the 1 ms granularity of the
     browser's own wall clock.
   - `insecureSeed()` answers ONE per-activation constant drawn from `crypto.getRandomValues(new
     BigUint64Array(2))`. Per-activation is the WIT contract ("intended to only be called once … to
     initialize DoS protection"), and it is deliberately NOT host-supplied: a host that could fix the
     seed could force guest `HashMap` collisions.
2. `🌐️browser-bundle/📜️script.ts:436-437` — the generated actor requires and forwards `wallNs`.
3. `🌐️browser-bundle/🧵️child/👷️worker/🟦️.ts:26` — the real child worker supplies
   `wallNs() = BigInt(Date.now()) * 1_000_000n` behind the same retired-phase guard as `nowNs`.
4. The vocabulary has **four** copies and all four now agree:
   `📇️directory/🧬️schema/🌐️browser-actor/🟦️.ts:3` (16 → 18),
   `📇️directory/🧬️schema/🌐️browser-actor/🦀️.rs:41` (`[&str; 16]` → `[&str; 18]`),
   `📇️directory/🧬️schema/🔣️.json` `DocumentBrowserActorInterfaces` (`maxItems` 16 → 18 + enum).
   The Rust copy is what the HUB BINARY enforces when it loads a published generation, so a hub built
   before this change refuses the new actor record — see §4.
5. **The law that pins the admitted set** —
   `🌐️browser-bundle/🧪️tests/🌐️wasi-activation/🟦️.ts`: an explicit 16-name literal is asserted
   `deepEqual` against (a) `browserWasiInterfaces`, (b) the shim record's own keys, and (c)
   `DOCUMENT_BROWSER_ACTOR_INTERFACES` minus the two `semio:framework/*` names. Adding an interface to
   any one copy now fails the law until all four and the shim agree. Plus behaviour laws: the exact
   `datetime` split, the fixed `resolution`, the seed's u64 range, its per-activation stability, its
   difference across two activations, `u64` rejection of a hostile `wallNs`, and `closed` after
   retirement for both.

**On the peer's uncommitted edits in that file** (`git diff` read in full): they are a TypeScript-version
repair, not a behaviour change — `Awaited<ReturnType<typeof reader.read>>`, `subscribeInstant/Duration`
parameters widened to `unknown` (the shim's own `u64()` still rejects everything but a bigint in range,
so the runtime admission is unchanged), `value.slice()` before `crypto.subtle.digest`, and narrowing
fixes in two tests. **The `write: false` removals at `:193`/`:492` are NOT a hermeticity regression:**
`Bun.build` only writes to disk when `outdir` is set, and neither call sets one, so both builds still
stay in memory and the digest-pinned evidence directory is still written only by
`writeFileSync(join(evidence, …))`. Nothing of theirs was restored or reverted.

## 4. Bootstrap to completion / publish / boot

Run: `📜️tc1-hub-boot.sh 7611`, launched detached 15:00:26, **inside the fleet wasm build mutex**
(preamble rule 27 — it drives two `wasm-release` component builds, two `describe` runs and a jco
codegen). It queued behind `f3`/`a3`/`b3e` and acquired the lock at **15:37:35**. Private
`CARGO_TARGET_DIR=⚡️cache/cargo/target-tc1`, data root `.🧬semio/🌐hub/tc1-boot`, log
`🗑️generated/tc1-hub-dev.txt`.

The hub binary is the one `TrustedStdioGisBootstrapScript` builds itself
(`runCargo(["build", …, "--bin", "os-hub"])`, `📜️script.ts:12620`) into that private target dir. It
finished at **15:38** — ~80 s, because the shared `build.build-dir` had everything cached. That is the
only correct binary for this run: a hub built before §3 item 4 carries the 16-name
`DOCUMENT_BROWSER_ACTOR_INTERFACES` and would refuse the candidate's 17-interface gis actor record, so
the copy-the-coordinator's-binary route (preamble rule 26) is not available for a catalog whose actor
vocabulary just changed.

**THE BLOCKER IS CLEARED, observed at 16:17.** `stdio` completed all eight stages, and — the thing DS1
never reached — **`gis` completed `derive 7/8 → 8/8 → complete 8/8`**. `derive` IS
`buildClosedBrowserActorArtifactV1`, the stage that raised `browser actor artifact: unsupported import
interface` at 14:40. With `wasi:clocks/wall-clock@0.2.0` and `wasi:random/insecure-seed@0.2.9` admitted
and implemented, jco 1.27.0 transpiled, the manifest parsed, the bundle closed, and
`packages/gis/browser/closed-actor.mjs` was staged. The run then moved on to `verify-generation 8/8`
(`proveTrustedGisColdMapComponentV1` under `trusted-catalog/validation/gis-…`), which is the
pre-publication native-cargo proof, followed by the candidate hub start and `publishTrustedBootstrapCurrent`.

Run 1 log kept as `🗑️generated/tc1-hub-dev-run1.txt`. Stage receipts, both packages, in order:

```
capture-codecs 0/8 → build 1/8 → snapshot 2/8 → extract-core 2/8 → snapshot-core 3/8 → inspect-wit 3/8
  → build-descriptor-emitter 4/8 → emit-descriptor 5/8 → verify → stage-component 6/8
  → stage-descriptor 7/8 → derive 8/8 → complete 8/8          (stdio, then gis — gis derive is the actor)
→ verify-generation 8/8
```

### 4a. The NEXT blocker, found because run 1 got past the first one, and fixed

Run 1 then died at 16:17:17, `🗑️generated/tc1-hub-dev-run1.txt:14839`:

```
error: Exact Cargo laws require an absolute artifactDir or SEMIO_TEST_ARTIFACT_DIR inside a generated directory
  at runExactCargoLaws        (🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts:2152)
  at proveTrustedGisColdMapComponentV1 (🌎️hub/📦️packages/🦀️rust/📜️script.ts:9674)
  at validateAndPublishTrustedStdioGisCandidate (…:10356)
  at run (…:12624)
```

**This is a product defect on the production path, not an environment mistake.**
`proveTrustedGisColdMapComponentV1` passed `artifactDir: join(artifactRoot,
"gis-component-cold-map-patch-exact")` where `artifactRoot` is
`trustedBootstrapValidationRoot(dataRoot)` = `<OS_HUB_DATA>/trusted-catalog/validation`. `isGeneratedPath`
(`📚️library/⚡️caching/🟦️.ts:13`) admits a path only if some segment is one of the policy's
`generatedDirectories` (`🔣️policy.json`: `dist`, `build`, `target`, `⚡️cache`, `🗑️generated`, …). A hub
data root is **never** such a path — `.🧬semio/🌐hub/…` has no admitted segment — so
`trusted-stdio-gis-bootstrap` could never publish into a real `OS_HUB_DATA`. It only ever worked for
`proveGisMapProposalProcess`, which builds its data root **inside** `SEMIO_TEST_ARTIFACT_DIR`. Nobody hit
it before because nobody had ever got a gis actor past `derive`.

Fixed at the root, not worked around with an env var: cargo-law evidence belongs in the repo cache, not
in a hub data root. `🌎️hub/📦️packages/🦀️rust/📜️script.ts:9674` now passes
`repoCacheDirectory(repoRoot, "hub", "trusted-gis-cold-map", current.generationId)` (`repoCacheDirectory`
was already imported, `:12`), and the now-meaningless fourth parameter is gone from the function
(`:9632`) and from its one call (`:10356`). The source-text law that **pins that call** in
`proveTrustedGisPublicationFixture` (`:8416`) was updated in the same pass, so its five hostile mutations
still derive from the real call text.

### 4b. Run 2 — the fix held, and it exposed a THIRD defect one line further on

Relaunched 16:20:55, lock acquired 17:00:37, materialisation complete 17:37 (**37 min**, both packages,
gis `derive 8/8` again). The §4a fix worked — evidence now lands in
`⚡️cache/hub/trusted-gis-cold-map/<generationId>/exact-cargo-laws-…`, and the run reached the cargo law
itself, where it failed (`🗑️generated/tc1-hub-dev-run2.txt:14863`):

```
error: exact Cargo law build failed: status=101 …
error: target `component_cold_map_patch` in package `semio-s-plugin-gis`
       requires the features: `component-receipt-acceptance`
```

`✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml:69` declares
`required-features = ["component-receipt-acceptance"]` for that test target, and the feature is
deliberately off by default (`:79-83`). The hub passed only `cargoArgs: ["--no-default-features"]`, so
the mandatory pre-publication proof **could never build**. Fixed at
`🌎️hub/📦️packages/🦀️rust/📜️script.ts:9680` →
`["--no-default-features", "--features", "component-receipt-acceptance"]`.

**That fix is corroborated, not guessed:** the plugin's OWN runner,
`✏️s/🔌️plugins/🌍️gis/🧪️tests/🌉️component-cold-map-patch/🟦️.ts:154`, carries the byte-identical
correction as an **uncommitted peer edit right now** (`git show HEAD:` that file still has the bare
`["--no-default-features"]`). So both callers of this law were broken in committed state, a peer is
repairing their copy as I repair the hub's, and the two now agree exactly.

Verified by running it myself rather than by relaunching the 37-minute chain:
`cargo build -p semio-s-plugin-gis --test component_cold_map_patch --no-default-features --features
component-receipt-acceptance` → `Finished dev profile in 5m 42s`, into the same `target-tc1` the chain
uses, so the chain's build is now a cache hit.

### 4c. The FOURTH blocker — the law itself is red, and it is not this slice's

With the flags corrected I ran the two laws directly against run 2's generation on disk
(`SEMIO_GIS_COMPONENT_WASM=<gen>/packages/gis/component.wasm`, plus the two digests read out of
`trusted-catalog.json`):

```
genuine_gis_component_rejects_stale_cold_authority_before_loading            ok
genuine_gis_component_cold_loads_and_patches_the_exact_tiled_map_surface     FAILED
  panicked at ✏️s/🔌️plugins/🌍️gis/🧪️tests/🌉️component-cold-map-patch/🦀️.rs:209
  map scene omits cold-before
test result: FAILED. 1 passed; 1 failed; finished in 307.91s
```

So the genuine gis component **cold-loads and runs** under wasmtime (that is the law that passed, and the
failing one got as far as rendering a tiled-map patch), but the first scene it renders does not carry the
fixture's `cold-before` marker, which `🦀️.rs:209` asserts on **every** scene it decodes inside the
256-turn loop rather than on the final one.

This is not TC1's change and not the actor allowlist: the law runs the component natively through
wasmtime, where none of the six files this slice edited is loaded. It is also almost certainly the
**first execution this law has ever had** — neither of its two callers could build it in committed state
(§4b) and a plain `cargo test -p semio-s-plugin-gis` skips it because the feature is off by default. It
belongs to whoever owns `🗺️gismap`'s editor; that directory has eight uncommitted peer edits right now,
including the runner fix above.

**`validateAndPublishTrustedStdioGisCandidate` calls this proof before staging or starting the candidate,
and `proveTrustedGisPublicationFixture` (`:8416-8431`) pins that ordering against five hostile mutations
— missing, late, substituted, detached and swallowed. There is no legitimate way to publish around it.**
So the catalog is not published and `/readyz` is not 200.

## 5. Generalising the verb for outcome 1+3 (design)

The target: `materializeTrustedCatalogBundle(repoRoot, dataRoot, requests)` where `requests` is an
N-row list, so note/draw/writer can enter a trusted catalog and outcome 1 (every plugin hosted) meets
outcome 3 (two users on one document) for a document kind that is not gis Map. The eight welded places
(C1c run 4's six + the two in `materializeTrustedStdioGisBundle` itself) and what each becomes:

| # | welded today | generalised |
|---|---|---|
| 1 | `requests` literal, `:9437` | a `TrustedCatalogRequestV1[]` parameter; `pluginId`, `cargoPackage`, `componentPackageId`, `outputName` already are fields — only the array is hardcoded |
| 2 | `if (request.pluginId === "gis")` derives the actor, `:9452` | `request.renderer === "wasm"` derives it; the renderer is already a per-request fact (`request.pluginId === "gis" ? "wasm" : "react"`, `:9483`) |
| 3 | `projectTrustedBootstrapCodecsV1(stdio, gis)` → `Record<"gis"\|"stdio", Codec[]>` with two literal JSON paths | `Record<string, Codec[]>` keyed by `pluginId`, each request carrying its own `nativeCodecsPath` |
| 4 | `codecs.stdio.length !== 26 \|\| codecs.gis.length !== 2` | per-request `expectedCodecCount`, still exact, still a refusal |
| 5 | one `target` literal (gis Map) | `request.openTargets: TrustedOpenTargetV1[]`; the profile's open-target count becomes their sum, and `open_target_count() > 0` stops meaning "gis" |
| 6 | `selectedClosure` / `packageSummary` two hand-written rows with literal `codecCount` | built by mapping `requests`, order = the request order (the closure domain already frames order explicitly) |
| 7 | `file(plugin: "gis"\|"stdio", …)`, `openTargets: plugin === "gis" ? [target] : []` | `file(pluginId: string, …)`, `openTargets: request.openTargets` |
| 8 | rotation reader refuses `packages?.length !== 2` | refuses `packages.length !== current.packageCount`, with `packageCount` framed into the generation record so a truncated bundle is still refused |

The guard laws are the real work, not the function. Three source-text laws assert on the literals:
`processConforms` matching `"await materializeTrustedStdioGisBundle("` (`:7194`), the retained-GIS proof
requiring `'"packages/gis/browser/closed-actor.mjs"'` (`:8245-8249`), and `:8427-8431` pinning the exact
`validateAndPublishTrustedStdioGisCandidate(...)` call text in three callers. Each must be rewritten to
prove the SAME property over an arbitrary request list — e.g. the retained-actor proof becomes "for every
request whose renderer is `wasm`, the bundle carries `packages/<pluginId>/browser/closed-actor.mjs` whose
`sourceComponentSha256` equals that package's component sha", which is strictly stronger than the string
match it replaces. `trustedBootstrapBrowserActorEncoding` already frames the actor path as a field, so
the wire format needs no change.

**Cost, measured, and why the builds are left detached.** `produceFreshComponentV1`
(`🖨️describe/🏭️fresh-component/🟦️.ts:256-259`) runs `cargo rustc --target wasm32-wasip2 --profile
wasm-release` with `CARGO_INCREMENTAL=0` into a per-run private `CARGO_TARGET_DIR`. That coldness IS the
provenance guarantee the trusted catalog publishes, so it cannot be short-circuited with a component
already on disk. stdio took 61 min and gis 62 min; note+draw+writer is three more of those, and every one
of them must go through the fleet wasm mutex (rule 27). They belong in a detached mutex-wrapped chain,
one build step per mutex acquisition, not inside a session.

**Landed from this section: nothing.** TC1's budget went to the blocker in §2 and the run in §4, and a
half-landed rewrite of this function is worse than none — C1c stopped at exactly this line for the same
reason, and the file is edited by peers. The table above is the hand-off: it is the complete list, with
current line numbers, and every row's replacement is mechanical once §4 has proven the one-package-with-
an-actor path end to end.

## 6. Files changed

Product code:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi/🟦️.ts` — `BrowserWasiPort.wallNs`, `wall()`, the per-activation `seed`, two new entries in `browserWasiInterfaces` and two new entries in the shim record.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts` — `:436-437`, the closure template requires and forwards `wallNs`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/👷️worker/🟦️.ts` — `wasiPort.wallNs`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧬️schema/🔣️.json` — `:26`, `:243` (bound + name regex), `:317`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌐️browser-actor/🟦️.ts` — `DOCUMENT_BROWSER_ACTOR_INTERFACES`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌐️browser-actor/🦀️.rs` — the Rust twin, `[&str; 16]` → `[&str; 18]`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json` — `DocumentBrowserActorInterfaces` bound + enum.

Laws:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌐️wasi-activation/🟦️.ts` — the four-copy pin and the wall-clock / insecure-seed behaviour laws; port literal gains `wallNs`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌊️actor-import/🟦️.ts` — the in-process actor port gains `wallNs`.

- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — `:9674` cargo-law evidence moved out of the hub data root into the repo cache; `:9632`/`:10356` the dead fourth parameter dropped; `:8416` the source-text law that pins that call updated with it; `:9680` the missing `--features component-receipt-acceptance`.

Ticket-owned (not product): `🐍️tc1-actor-imports.ts`, `📜️tc1-hub-boot.sh`, this report, and the
`🗑️generated/tc1-*` captures (`tc1-hub-dev-run1.txt`, `tc1-hub-dev-run2.txt`, `tc1-gis-imports.txt`,
`tc1-stdio-imports.txt`, `tc1-gis-imports-admitted.txt`, `tc1-wasi-activation.txt`,
`tc1-browser-bundle-laws.txt`).

**Nothing of a peer's was reverted.** The uncommitted edits in `📜️script.ts`, `🌐️wasi/🟦️.ts` and the
two tests are theirs and were left exactly as found (§3, last paragraph).

## 7. Honest gaps

1. **`/readyz` was never observed at 200, and `open-plan` was never called.** No hub was started by TC1
   at all — `📜️tc1-hub-boot.sh` step 2 is gated on step 1's exit code, and step 1 has never returned 0.
   Nothing in this report claims a running hub.
2. **The gis cold-map law is red and TC1 did not fix it** (§4c). It is one assertion in a `🗺️gismap`
   rendering path with eight uncommitted peer edits in that directory, and each verification cycle is a
   5-minute wasmtime run; an unverified edit there would be worse than none. The precise hand-off is
   `🦀️.rs:209` asserting the marker on **every** decoded scene inside a 256-turn loop — the first thing
   to establish is whether the component never emits the marker, or emits it on a later turn than the
   first scene the loop decodes.
3. **The new wall-clock and insecure-seed shims are proven by law, not by a browser.** The 63 MB
   `closed-actor.mjs` exists and the whole `testClosedBrowserComponentFactory` suite passes, but no
   browser has instantiated THIS actor — that needs a published catalog, so it is downstream of gap 2.
4. **§5's generalisation is designed, not landed.** Zero of its eight rows were changed.
5. **Fixture counts left alone:** `🌊️actor-import/🔣️.json`'s `expectedImports` stays at 15 and the
   `🧬️stdio-gis-bootstrap` fixture's actor rows stay 2-name — both are synthetic components that
   genuinely do not import the two new interfaces, and both still validate under the widened bounds.
6. **TC1 started nothing that is still running.** Both bootstrap runs exited; the wasm mutex was
   released on each exit (`cat /tmp/semio-wasm-build.lock/owner` showed other slices afterwards); no
   hub, no serve, no cargo of TC1's survives.
