# 📓️ terra-dedyn-census — independent repo-wide dyn/async census

Packet: `dedyn-census`. **Measurement only — no source files edited.** Method: python3 over
absolute paths (two differently-implemented queries for every headline number — a unified-regex
walk and a separate token-split walk — both re-run fresh from disk; shell `grep` was **not** used
for counting, only for one-off spot checks). Scripts live in the scratchpad, not the ticket
folder, since they are throwaway: `/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/{census.py,census_bucket.py,census_fn_breakdown.py}`.

## 0. Headline numbers

| metric | value |
|---|---:|
| **Total first-party `dyn` remaining today** | **173** (both queries agree exactly) |
| Total std/lang `dyn` (`Future`/`Fn`/`FnMut`/`FnOnce`/`Any`/`Error`/`Iterator`) | 132 — permitted baseline, not a failure |
| Unclassified `dyn X` tokens found, resolved by hand | 2 (`dyn for<'a> Fn(...)` HRTB false-positive; `dyn wasmtime::ResourceLimiter`, an external crate trait) — **zero residual after reclassification** |
| **True original baseline** (`sol-dyn-families.json`, 97 families, pre-conversion) | **985** |
| Ticket-quoted "starting inventory" (fleet 282 + framework/hub 294) | **576** — see §3, this **undercounts the true baseline by 409 dyn-uses across 58 families that were never assigned to any of the 18 packets** |
| `cargo check -p semio-framework-os-kernel --lib` | **exit 0** (417 warnings) — the SDK gate has cleared |
| `cargo check -p semio-framework-plugin --lib` | **exit 101** — blocked by `semio-framework-ui` (169 errors), never reaches the plugin crate itself |

## 1. Repo-wide first-party `dyn` census, by trait

Method: collected every `trait Name` declaration under `🧰️framework`, `✏️s`, `🌎️hub` (10,533 `.rs`
files scanned, excluding `.🧬semio`, `target`, `node_modules`, `.nx`, `storybook-static`, `.venv`),
234 distinct first-party trait names found. Then counted `dyn <name>` (qualified-path forms too,
e.g. `Arc<dyn crate::mod::Name>`) on code lines only — full-line and trailing `//` comments
stripped, block comments (`/* */`) stripped first. Cross-checked with an independently-coded
token-split scanner (splits on `(<>,;&` instead of regex, looks up the token after each bare `dyn`
against the same name set). **Both give 173, and identical per-trait counts.**

| trait | count | files | owning area |
|---|---:|---:|---|
| Element | 20 | 8 | `✏️s/🔨️modules/🏗️fem` (not a "plugin" — never in the fleet's `🔌️plugins/**` scope) |
| OsBackbonePort | 18 | 2 | 14 in `framework/…/💻️os/🖥️host/component.rs` (declaration file) + 4 in `✏️s/🔌️plugins/🪐️space` (fleet:space's lease-requested residue) |
| Emit | 15 | 5 | `framework/…/💻️os/🛢️db/{engine,actor,artifact,security,version-graph}` |
| HostAsyncRuntime | 10 | 1 | `framework/…/🔌️plugin/🖥️host/⚡️effects/component.rs` — the exact 10 sites fw:os-hostasync named as out-of-scope |
| BlobStore | 9 | 3 | `framework/…/💻️os/🏃️run/{component.rs,bin.rs}` (7) + `framework/…/💻️os/🪐️space/component.rs` (2) — a **different consumer of the same trait name** than `🖥️server`'s own `BlobStore` declaration, which fw:server closed to 0; these two `os`-side files were never in fw:server's scope and were never assigned to anyone |
| SpaceBackbonePort | 9 | 2 | 8 in `framework/…/💻️os/🪐️space/component.rs` (declaration file) + 1 in `✏️s/🔌️plugins/🪐️space` (fleet:space's lease-requested residue) |
| BrepKernel | 8 | 1 | `framework/…/💻️os/🌊️flow/📐️brep-geometry/component.rs` — a **framework**-side consumer, distinct from both the stdio declaration and the cad-plugin consumer that fleet:cad already fixed |
| AuditSink | 7 | 2 | `framework/…/💻️os/🌉️mcp/{📒️audit,component.rs}` |
| HttpBody | 7 | 1 | `framework/…/💻️os/🛎️services/component.rs` |
| ArtifactChannel | 5 | 2 | `framework/…/💻️os/🌉️mcp/🔀️dispatch` |
| DirectoryWsConnection | 5 | 1 | `framework/…/💻️os/📇️directory/🔌️client` |
| HttpTransport | 5 | 2 | `framework/…/💻️os/🛎️services` (4) + `📇️directory` (1) |
| RouterEffectHandler | 5 | 2 | `framework/…/💻️os/🔌️plugin/🖥️host/⚡️effects` |
| VersionGraph | 5 | 3 | `framework/…/💻️os/🛢️db/🕸️version-graph` |
| Operator | 4 | 2 | `framework/…/💻️os/{🌊️flow(3),🧠️neural(1)}` — the trait's own declaration/registry, exactly the "sync `Operator::evaluate`, `Registry`/`OperatorImpl` dyn erasure" cross-packet finding fleet:flow and fleet:imperative both flagged and neither could fix (framework-owned) |
| AuthzHook | 4 | 2 | `framework/…/💻️os/🛢️db/📄️artifact` |
| GatewayBackend | 3 | 2 | `framework/…/💻️os/🌉️mcp/🧭️protocol` |
| JoinHandleLike | 3 | 1 | `framework/…/💻️os/🛢️db/🎭️actor` |
| ToolRegistry / ResourceRegistry / PromptRegistry | 2 each | 1 each | `framework/…/💻️os/🌉️mcp/🧭️protocol` |
| EnvelopeInjector / BackboneTransport / CapabilityChecker / StorageBackend / EffectMetricsRecorder | 2 each | 1 each | all in `framework/…/💻️os/🔌️plugin/🖥️host/⚡️effects/component.rs` — **the same one file as HostAsyncRuntime and RouterEffectHandler above; this single file holds 7 untouched trait families, 25 dyn uses total** |
| ThreadSpawner, QuerySource, AsyncHttpTransport | 2 each | 1 each | `🛢️db/🎭️actor`, `🛢️db/🔍️query`, `🛎️services` respectively |
| DynEngine, MediaCache, CompletionSink, ConflictOracle, Signer, SignatureVerifier, Backbone, MeshExporter, MeshImporter | 1 each | 1 each | scattered `framework/…/💻️os/*` and two `framework/🔨️modules/*` crates |

Full per-file paths for every row: `census_result.json` / `census_buckets.json` in the scratchpad
(not reproduced here in full to keep this table readable).

**Bucket-level summary (directory, not trait name)** — this is the more useful cut because it
tells you *whose path scope* the residue sits in:

| bucket | current residual |
|---|---:|
| `framework/products/💻️os/🛢️db` | 34 |
| `framework/products/💻️os/🔌️plugin` (the `⚡️effects` file) | 25 |
| `framework/products/💻️os/🌉️mcp` | 21 |
| `✏️s/modules/🏗️fem` | 20 |
| `framework/products/💻️os/(root)` — `🖥️host/component.rs` | 17 |
| `framework/products/💻️os/🛎️services` | 14 |
| `framework/products/💻️os/🌊️flow` | 11 |
| `framework/products/💻️os/🪐️space` | 10 |
| `framework/products/💻️os/🏃️run` | 8 |
| `framework/products/💻️os/📇️directory` | 6 |
| `✏️s/plugins/🪐️space` | 5 |
| `framework/products/💻️os/⚙️engine`, `🧠️neural` | 1 each |
| **every other bucket (all 10 other fleet plugins, `🖥️server`, `🕸️graph`, `🌎️hub`, `🔄️machine`, `🎯️action-bus`, `🖱️ui`, `🦑️repo`)** | **0** |

**148 of the 173 remaining first-party `dyn` uses (85.5%) sit inside
`🧰️framework/🛍️products/💻️os/**`.** Of the 19 packets summarized to me, only three
(`fw:os-hostasync`, `fw:os-guestruntime`, `fw:os-spacemember`) touched anything under `os` at all,
and their own reports account for only ~76 sites. The other ~72 of these 148 were never even
described as *existing* in any report I was given.

## 2. std/lang `dyn` — the permitted baseline

| trait | count |
|---|---:|
| Fn | 48 |
| FnMut | 23 |
| Future | 25 |
| Any | 14 |
| FnOnce | 10 |
| Error | 8 |
| Iterator | 4 |
| **total** | **132** |

Spot-checked the two ambiguous scanner hits by hand: `dyn for<'a> Fn(&'a IoKey, …) -> …` in
`🚪️io/component.rs:1127` (the `IoFallback` fn-pointer erasure — explicitly R1-legal, "(ii) the
return type of fn-pointer thunks in erasure tables"), and `dyn wasmtime::ResourceLimiter` in the
plugin host's `⏳️imports.rs`/`component.rs` — a **wasmtime** trait, not first-party, R1-legal.
No `dyn Future` was found in trait-method return position (the one thing R1 bans outright) —
confirmed by inspecting all 25 `dyn Future` sites; every one is either an `IoFallback`/
`ComposeFuture`-style fn-pointer thunk return type or a `HostFuture<T>` argument-position box.

## 3. Before/after — and a correction to the ticket's own "starting inventory"

The ticket brief gave me: **fleet 282 across 11 plugins, framework+hub 294 across 8 areas = 576**.
Every one of those 576 traces to a real subset of `sol-dyn-families.json`'s 97 families — but that
JSON's own total is **985**, not 576. **409 dyn-uses across 58 families were part of the real
pre-conversion baseline and were never assigned to any of the 19 named packets.** Two independent
scans (§1) confirm the residue left today from that unassigned pool is **124** (i.e. ~285 of those
409 were fixed anyway, apparently by earlier/other U-program packets not in my list of 18 —
`store-dedyn`, `db-dedyn`, `math-dedyn`, `host-dedyn`, `kernel-ripple` are named in R5's slug list
and match exactly: `BackbonePort`/`BackboneChannelPort` → 0 (store-dedyn, confirmed by fleet:space's
own report citing it by name), the six `🛢️db/🗄️storage` traits `DbStorage`/`IndexStorage`/
`LeaseStorage`/`WalStorage`/`SnapshotStorage`/`PayloadStorage`/`CatalogStorage` (135 baseline) → 0
(db-dedyn), the eight `🧮️math/🎯️sampling` traits `LogitsProcessor`/`RandomSource`/
`TokenTextAdapter`/`TokenSampler`/`StopCondition`/`SamplingObserver`/`Collective`/`Denoiser` (86
baseline) → 0 (math-dedyn — the same packet R10 names as the source of the 250-edit
name-keyed-await corruption incident), `PluginApp` (19 baseline) → 0, and the four `🚪️io` traits
`PayloadSource`/`PayloadSink`/`RandomAccessPayload`/`ResourceResolver` (17 baseline, matching R11's
own "17 use sites" text exactly) → 0, which is R11's own worked example already landed.)

### Per-area/plugin table (19 named areas, exactly as given in the brief)

| area | baseline (brief) | current residual (census) | status |
|---|---:|---:|---|
| animate | 155 | 0 | confirmed |
| norm | 26 | 0 | confirmed |
| cad | 24 | 0 | confirmed |
| draw | 18 | 0 | confirmed |
| process | 15 | 0 | confirmed |
| procedural | 12 | 0 | confirmed |
| **space** | 11 | **5** | fleet:space's own reported residue (`OsBackbonePort` ×4 + `SpaceBackbonePort` ×1, lease-requested) — **confirmed, exact match** |
| stdio | 7 | 0 | confirmed |
| flow (plugin) | 6 | 0 | confirmed — **do not confuse with `framework/…/💻️os/🌊️flow`, a different module with the same name, 11 residual, never assigned (see below)** |
| imperative | 4 | 0 | confirmed |
| sourcing | 4 | 0 | confirmed |
| os | 225 | **148** | **only 76 of the 225 was ever assigned to a named packet** (hostasync 23 + guestruntime 38 + spacemember 15); the packets that ran are internally confirmed clean, but 138 of the 148 remaining sites belong to trait families no report ever named |
| server | 26 | 0 | confirmed — fw:server's `cargo test`/`--all-targets` green claim is consistent with this |
| graph | 20 | 0 | confirmed |
| hub | 8 | 0 | confirmed |
| machine | 7 | 0 | confirmed |
| repo | 5 | 0 | confirmed |
| action-bus | 2 | 0 | confirmed |
| ui | 1 | 0 | confirmed |
| **subtotal, 19 named areas** | **576** | **153** | |
| **unnamed area: `✏️s/🔨️modules/🏗️fem`** (`Element`, never in any fleet packet's scope) | 20 | 20 | **0% converted, never assigned** |
| **unnamed residue inside `985`-baseline but outside the 19 named areas, already resolved by other (unlisted) packets** | 389 | 0 | store-dedyn / db-dedyn / math-dedyn / an io fix (kernel-ripple's own R11 worked example) — not this fleet's doing |
| **TOTAL (true baseline)** | **985** | **173** | |

The 173 figure is authoritative (both scans agree, cross-checked against 97 named families with
zero unattributed families — every current-residual trait name traces to a `sol-dyn-families.json`
entry).

## 4. Async-literal census

| area | `async fn` | plain `fn` | ratio async |
|---|---:|---:|---:|
| `🧰️framework` | 9,892 | 8,423 | 54.0% |
| `✏️s` | 56,693 | 978 | 98.3% |
| `🌎️hub` | 251 | 90 | 73.6% |

`// 🚫️async: E<n>` tags found: `🧰️framework` — E1 129, E3 19, E4 86, E5 13 (**247 total**);
`✏️s` — E3 41; `🌎️hub` — none.

**Caveat, stated plainly rather than asserted as a precise defect count**: 247 tags vs. 8,423 plain
`fn`s in framework is not directly comparable — R4 clause 5 sanctions `#[test] fn` bodies as
executor entry points **without requiring an individual tag on each one** (only the crate's *one*
designated bridge fn needs the E5 tag). A heuristic breakdown (attribute-scan + crude impl-block
tracking, itself imprecise — it misses `Deref`/`DerefMut`/operator-trait impls, and doesn't
special-case whole proc-macro crates like `semio-framework-dispatch-macros` or
`semio-framework-schema-derive` where every fn is compile-time-only and E3-adjacent by
construction) finds, of framework's 8,423 plain fns:

| classification (heuristic) | count |
|---|---:|
| `#[test]`-attributed (R4 clause 5 territory) | 1,866 |
| immediately inside an `impl <ExternalTrait> for …` block (crude E1 proxy — misses `Deref`/ops traits) | 396 |
| `const fn` | 16 |
| `fn main` | 20 |
| **unclassified — the real residue-under-R2 candidate pool** | **6,125** |

(`✏️s`: 708 external-trait-impl-adjacent, 262 unclassified, 8 `fn main`, 0 test-attributed — matches
the earlier "11,737 `#[test] async fn` → `#[async_test]`" centralised sweep, i.e. fleet tests are
already converted and don't show up as plain `fn` at all. `🌎️hub`: all 90 unclassified.)

**I am not reporting "6,125 + 262 + 90 = 6,477 R2 defects" as a hard number** — the heuristic's
false-positive rate is unmeasured and likely large (proc-macro crates, `Deref`/ops impls, and
trait-declaration-only signatures without bodies all inflate it). What I can say with confidence:
the framework's async-literal conversion is **roughly half done** (54%) while the fleet plugins are
essentially done (98.3%), and the tag discipline (247 tags) is nowhere near sufficient to account
for even the conservative floor of the residue (6,125 unclassified alone). This area needs its own
dedicated, tool-assisted audit — a job for a packet, not a byproduct of this census.

## 5. Compile state

All commands run in the **foreground**, single turn, `CARGO_TARGET_DIR` pointed at the session
scratchpad (`/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-census`), exit codes read from `$?` immediately after the bare command
(no pipe to `tail`/`tee` in the code path that produced the reported exit code — the first run did
pipe through `tee`+`tail` and I discarded that exit code and re-ran clean per rule 10).

```
$ CARGO_TARGET_DIR=…/target-census cargo check -p semio-framework-os-kernel --lib
warning: `semio-framework-os-kernel` (lib) generated 417 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 9 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 49.13s
$ echo $?
0
```

**`semio-framework-os-kernel --lib` is GREEN (exit 0).** The SDK gate that all 18 packets cited as
blocking has cleared since their reports were written — in particular the missing-`.await` at
`🏪️store/component.rs:3485` that fleet:norm, fleet:cad, fleet:process, fleet:procedural,
fleet:sourcing, fw:os-spacemember all independently hit and named is **no longer present** (0
errors in this run; 417 warnings remain, mostly `unused implementer of Future that must be used` —
i.e. more missing-`.await` bugs, but ones that only *warn*, not block, compilation).

```
$ CARGO_TARGET_DIR=…/target-census cargo check -p semio-framework-plugin --lib
error: could not compile `semio-framework-ui` (lib) due to 169 previous errors; 2 warnings emitted
warning: build failed, waiting for other jobs to finish...
$ echo $?
101
```

**`semio-framework-plugin --lib` still fails (exit 101)**, but not for a reason any of the 18
packets could have fixed within their own path scope: the failure is entirely inside
`semio-framework-ui` (169 errors — 74×E0308, 24×E0600, 24×E0277, 7×E0599, 4×E0382, 2×E0369,
1×E0529, 1×E0271 — the same count fleet:animate and fw:os-spacemember both independently observed
and named), and `semio-framework-plugin` itself is never reached. This exactly matches the ticket's
documented COMPILE REALITY warning.

Per the task's own conditional ("if the SDK is green, also try one fleet crate") — the SDK
(`semio-framework-plugin`) is **not** green, so I did not run a fleet-crate check; running one
would only reproduce the identical `semio-framework-ui` blocker one layer further downstream.

## 6. Every discrepancy found, ranked by materiality

1. **The ticket's quoted "starting inventory" (576) is not the true starting inventory (985).**
   409 dyn-uses across 58 families were never assigned to any of the 19 named packets. This is not
   any individual packet's error — no packet claimed coverage it didn't have — but it means the
   program's own tracking of "how much is left" has been wrong by 41.5% of the true total since
   before this fleet started. (§3)

2. **`framework/products/💻️os/**` is 85.5% of all remaining first-party dyn (148 of 173), and
   only 3 of the 19 named packets ever touched it, covering ~76 of the original 225.** The
   remaining ~138 sites, across ~30 distinct trait families (`Emit`, `AuditSink`, `HttpBody`,
   `ArtifactChannel`, `GatewayBackend`, `ToolRegistry`/`ResourceRegistry`/`PromptRegistry`,
   `VersionGraph`, `AuthzHook`, `JoinHandleLike`, `ThreadSpawner`, `QuerySource`, `ConflictOracle`,
   `Signer`, `SignatureVerifier`, `DirectoryWsConnection`, `HttpTransport`, `AsyncHttpTransport`,
   `CompletionSink`, `DynEngine`, `MediaCache`, `MeshExporter`, `MeshImporter`, `Backbone`, plus the
   framework-side halves of `OsBackbonePort`/`SpaceBackbonePort`/`BrepKernel`/`Operator`), have
   **never appeared in any packet report at all**. This is the single biggest actionable gap. (§1, §3)

3. **One file, `framework/…/💻️os/🔌️plugin/🖥️host/⚡️effects/component.rs`, holds 7 entirely
   untouched trait families and 25 dyn uses** (`HostAsyncRuntime` 10, `RouterEffectHandler` 5,
   `EnvelopeInjector`/`BackboneTransport`/`CapabilityChecker`/`StorageBackend`/
   `EffectMetricsRecorder` 2 each). `fw:os-hostasync` correctly named this file and its
   `HostAsyncRuntime` count (10, exact match) as out-of-scope for a sibling — but no sibling packet
   for it exists in the 18 I was given, and the other 6 families in the same file were never named
   by anyone. (§1)

4. **`framework/…/💻️os/🌊️flow` (a framework module) is a false cognate of `✏️s/🔌️plugins/🌊️flow`
   (the fleet plugin fleet:flow correctly zeroed).** The framework module has its own 11-site
   residue (`BrepKernel` 8, `Operator` 3) that has nothing to do with the plugin and was never
   claimed or disclaimed by fleet:flow (reasonably — out of their path scope — but worth surfacing
   since the name collision could cause a future packet to believe "flow is done"). Same pattern
   for `framework/…/💻️os/🪐️space` vs `✏️s/🔌️plugins/🪐️space` (fleet:space correctly scoped to the
   plugin only; the framework module's own 18-site residue — `OsBackbonePort` 14 +
   `SpaceBackbonePort` 8 minus overlap, see the table — was never assigned). (§1, §3)

5. **`Operator` and `BrepKernel`'s framework-side declaration/registry residue is exactly the
   architectural gap fleet:flow and fleet:imperative (for `Operator`) and fleet:cad (for
   `BrepKernel`'s stdio-side `async_trait`) already flagged as cross-packet findings** — confirmed
   still present and unaddressed. Both fleet reports were accurate; nobody has picked up the
   flagged work yet. (§1)

6. **`✏️s/🔨️modules/🏗️fem` (the `Element` trait, 20 sites) was never in the fleet's scope at
   all** — the fleet inventory's own scoping rule was `✏️s/🔌️plugins/**`, and `fem` lives under
   `✏️s/🔨️modules/**`, a sibling tree the fleet's path-based ownership convention structurally
   excludes. 0% converted. No packet report mentions it. (§1, §3)

7. **The SDK gate has cleared since the 18 reports were written.** `semio-framework-os-kernel
   --lib` is green today (exit 0); several packets (norm, cad, process, procedural, sourcing,
   os-spacemember) reported it red with the identical `🏪️store/component.rs:3485` missing-`.await`
   `E0599`. That specific defect is gone. `semio-framework-plugin` is still blocked, but now purely
   by `semio-framework-ui` (169 errors), one layer further down the graph — consistent across every
   packet that hit it. (§5)

8. **No packet self-report was found to overstate its own zero-dyn claim.** Every one of the 18
   packets' "N → 0" claims for their own owned path was reproduced exactly by this independent
   census — including the one packet (fleet:space) that reported nonzero residue, whose exact
   number (5: `OsBackbonePort` ×4 + `SpaceBackbonePort` ×1) this census also reproduces exactly.
   The gap in §1–§6 is a **program-tracking/assignment gap**, not a **packet honesty** problem.

9. **`SpaceMember` (baseline 101, the second-largest family after `Sobject`) is fully at 0**, but
   the packets that touched it (`fw:os-spacemember`: 15, `fleet:stdio`: 2) only account for 17 of
   the 101. The other 84 were resolved by a prior, unlisted packet that replaced the whole
   `ChildStoreFactory` registry with the `space_members!`-macro-generated closed enum — confirmed
   by `fleet:stdio`'s own report text ("framework had already replaced that whole registry … as
   part of a separate, already-landed framework change"). Not a discrepancy, just worth recording
   so nobody re-derives it as a mystery. (§3)

## Files touched

None outside this report and the scratchpad. Scripts and intermediate JSON:
`/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/{census.py,census_bucket.py,census_fn_breakdown.py,census_result.json,census_buckets.json,fn_breakdown.json,census-oskernel-check2.txt,census-plugin-check.txt}`.
