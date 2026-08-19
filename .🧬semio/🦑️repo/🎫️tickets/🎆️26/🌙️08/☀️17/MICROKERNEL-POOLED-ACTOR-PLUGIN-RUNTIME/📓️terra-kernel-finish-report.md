# 📓️ terra-kernel-finish report

## Headline

`semio-framework-os-kernel --lib`: **37 → 1 error.** Exit 0 is NOT reached — one residual error is
a genuine cross-crate architectural blocker outside this packet's granted lease (full analysis below).
Everything else in the brief's characterisation (`🏪️store` fn-pointer cluster, `🚪️io` 2 errors, `📡️spr`
2 errors) is fixed and verified.

```
cargo check -p semio-framework-os-kernel --lib
CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-fanout
EXIT: 101, 1 error (down from 37 measured fresh at session start)
```

## The real root cause (bigger than the brief's per-module table, same shape as E4)

The brief characterised `🏪️store`'s residue as "mostly E0308 fn-pointer" (5 sites) "plus a cluster ...
(expected Shape/FieldValue, found future, no method map_err on opaque type)". Investigating the cluster
showed it was not a `🏪️store`-local bug at all: **`DslField::shape()` and `DslVariants::variants()`
in `🗣️dsl/🦀️component.rs` were blind-codemodded to `async fn`, but both are E4 by transitivity** —
`Shape::Record`/`Table`/`Statements` hold a bare `fn() -> RecordSpec`, and the derive macro's generated
`__dsl_spec()` (itself E4, stored as that fn pointer) calls `DslField::shape()`/`DslVariants::variants()`
synchronously to build the RecordSpec. An async `shape()`/`variants()` can never be called from inside a
sync `__dsl_spec()` — there is no executor to poll it. This was the source of ~30 of the 37 errors: every
`#[derive(DslRecord/DslArtifact/DslOps)]`-generated struct/enum in `🏪️store` hit it.

`to_value()`/`from_value()`/`to_named_record()`/`from_named_record()` are NOT E4 (nothing stores them in
a fn-pointer slot) and stay `async` per O1.

### Fixed (my owned paths only)

1. **`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️component.rs`** — `DslField::shape`/
   `DslVariants::variants` made sync (E4-tagged) on the trait and on every impl: `i8..u64`/`bool`/`f32`/
   `f64`/`String`/`Wire`/`Vec<T>`/`BTreeMap<String,T>`/`[T;N]`/`DslValue`, plus `__rt::newtype_variant_spec`
   (also E4 — cast `as fn() -> RecordSpec` at every newtype-variant call site) and the two `.await`s on
   `T::variants()` inside `variants_binary::encode_op`/`decode_op` removed.
2. **`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs`** — this is the
   ACTUALLY-COMPILED derive-macro source (`Cargo.toml` points `path = "📦️glue.rs"` directly, not via
   `#[path]` to `🦀️component.rs` like every other module in this repo — **`🗣️dsl/✨️derive/🦀️component.rs`
   is a stale, uncompiled duplicate**, flagged below, not touched). Rewrote `record_codegen`,
   `record_codegen_to_value_from_bindings`, `dsl_variants_codegen`, and the `DslRecord`/`DslArtifact`/
   `DslDiff`/`DslScalar`/`DslOps` derive templates: `shape_expr` needs no `.await` now;
   `to_value`/`from_value`/`to_named_record`/`from_named_record` calls get `.await`; `Vec`/`Map` field
   kinds (`VecList`/`VecTable`/`VecTuple`/`VecStatements`/`VecBlockStatements`/`MapField`) can't `.await`
   per-element inside `Iterator::map` (R10 residue shape 1) so those became sequential loops, matching the
   precedent already in `Vec<T>: DslField`/`BTreeMap<String,T>: DslField`. `__dsl_spec`/`__dsl_diff_spec`
   made sync (E4). Verified: `cargo check -p semio-framework-os-kernel-dsl-derive --lib` → exit 0.
3. **`🏪️store/🦀️component.rs`**: 4 hand-written `impl DslField for {ArtifactChild,OwnerRef,LinkPin,
   ArtifactLink}` `shape()` methods → sync (E4, matching the trait). 4 standalone `*_spec()` builder fns
   (`artifact_child_spec`/`owner_ref_spec`/`link_pin_spec`/`artifact_link_spec`) → sync + E4-tagged (same
   shape as the FieldSpec/RecordSpec builders already fixed pre-session — these were the literal
   `Shape::Record(artifact_child_spec)` fn-pointer-coercion errors at :642/689/771/788/818 in the brief's
   table). Ran `remove-bad-await.py --crate semio-framework-os-kernel --scope 🔨️modules/🏪️store` (dry-run
   then apply) to strip the resulting `.variants().await` residue — removed 12 spans cleanly, fixpoint on
   pass 2, 0 out-of-scope.
4. **`🚪️io/🧬️schema/🦀️component.rs`** (2 errors from the brief's table): both were plain missing
   `.await`, not E4/R9 — `ArtifactKindId::parse` calling `is_canonical_artifact_kind(s)` under `!` without
   awaiting it first, and `ArtifactRef::to_uri` interpolating `self.dialect.to_coordinate()` into `format!`
   without awaiting. Both fns are themselves `async`, so this was a one-line fix each, no exception class
   needed.
5. **`📡️spr` granted lease** — `📡️replication/🔢️scalar/🦀️component.rs`'s `write_id`/`read_id` (the 2
   errors named in the brief, `📜️history/🦀️component.rs:623,629`): changed `intern`/`resolve` params from
   plain `Fn`/`FnMut` to `AsyncFnMut`/`AsyncFn` (their real arguments are `DictBuilder::intern`/
   `DictReader::resolve`, genuinely `async fn`), added `.await` on the internal calls. Hit a real rustc
   HRTB limitation along the way — `impl AsyncFn(u32) -> T` closures built via `async move {}` blocks or
   `std::future::ready(...)` fail with **"implementation of AsyncFn is not general enough"** — fixed by
   using genuine `async |args| body` closure syntax (stable on this repo's nightly toolchain), which
   sidesteps the HRTB gap entirely. Fixed the ONE production caller
   (`📡️spr/📜️history/🦀️component.rs`'s `write_id_field`/`read_id_field`) plus 6 test call sites in
   `📡️replication/🧾️wire/🦀️component.rs` that also needed the same closure-syntax conversion (found via
   `--all-targets`, not `--lib`, so not in the brief's count — but the ticket's rule 26 requires both, and
   `semio-framework-replication` was explicitly "currently GREEN, do not destabilise", so these had to be
   fixed to keep it green). **Verified BOTH ways, both directions**:
   `cargo check -p semio-framework-replication --lib` → exit 0 (0 errors, 59 warnings, same as baseline).
   `cargo check -p semio-framework-replication --all-targets` → exit 0 (0 errors, same 59 warnings, no
   duplicates beyond the lib set). Replication is exactly as green as it was before I touched it.

## 🔴️ The 1 remaining error — genuine STOP-and-report, not left half-done

```
error[E0599]: no method named `map_err` found for opaque type
  `impl Future<Output = Result<protocol::MutationEnvelope, protocol::ProtocolError>>` in the current scope
  --> 🏪️store/🦀️component.rs:3485:58
   |
3485 |     crate::os_spr::decode_envelope(&bytes, &mut pos).map_err(serde::de::Error::custom)
```

This is inside `mod operation_envelope_serde` (`🏪️store/🦀️component.rs:3469-3487`), the
`#[serde(with = "operation_envelope_serde")]` bridge for `ArtifactCommand::IngestRemote.envelope` (field
at :269-270). `serde::Deserializer`/`Serializer` callback signatures are externally fixed (**E1** — cannot
be async, no exceptions). `crate::os_spr::decode_envelope`/`encode_envelope` resolve to
**`📡️replication/🔗️causal/🦀️component.rs:362,378`** — outside this packet's granted lease, which named
`scalar::read_id`/`write_id` "for this specific signature change only".

**Both halves of the R9 test, shown explicitly, as required:**
- No I/O in `decode_envelope`/`encode_envelope` or their transitive closure (`wire::write_str`/`read_str`/
  `write_bytes`/`read_bytes`/`write_varint_u64`/`read_varint_u64`/`write_hash32`/`read_hash32`, `causal::
  encode_hlc`/`decode_hlc`, bottoming out in `codec::write_varint_u64`/`read_varint_u64`) — confirmed, all
  pure `Vec<u8>`/`&[u8]` cursor manipulation, zero `std::fs`/`tokio`/`reqwest`/`TcpStream`/`spawn`/`sleep`/
  `SystemTime` anywhere in the chain.
- The consumer (`operation_envelope_serde::deserialize`) is language-barred — E1, serde's fixed sync
  signature, not a judgement call.

**Why I did not just fix it, three things ruled out with evidence, not assumption:**
1. **Not a narrow fix.** The 6 shared leaf primitives in the chain (`write_str`/`read_str`/`write_bytes`/
   `read_bytes`/`write_varint_u64`/`read_varint_u64`) have **220 call sites across `📡️replication`**
   (37/34/19/26/51/53 respectively, measured by grep, not estimated) — used by frame encoding, dictionary
   ids, and every other wire message type in the crate, not just `MutationEnvelope`. Flipping them to sync
   is mechanically safe (every other caller stays `async` and just drops `.await`, exactly like my
   `DslField::shape` fix) but touches many files far outside the granted lease.
2. **This exact class of fix was already tried and reverted, for the same reason.** `📓️status.md`
   (~line 4850-4868, "❌️ …but first I got a call WRONG, and reverting was the right move"): a prior lease
   claiming `DictReader`/`DictBuilder`/`FrameCursor::prev_frame` were "pure accessors wrongly asyncified"
   was applied, broke the crate ("the R9 chain runs through the whole codec subtree, not the two accessors
   the lease named"), and was reverted — explicitly because replication was already green and reshaping it
   on an unverified premise was the wrong call. My situation differs in one respect (my consumer genuinely
   IS E1-language-barred, unlike that case, where R9 rule 3 applied and the fix belonged in the consumer)
   — but the **blast-radius risk to an already-green crate is identical**, and the ticket's own rule for
   this shape is explicit: "STOP and report instead."
3. **`poll_ready` (the repo's established "sync value from an always-ready future" bridge) was
   deliberately REMOVED**, not an option to reintroduce. `📓️status.md` ~4695-4703: `host-dedyn` removed it
   crate-wide specifically because it "polls once with a no-op waker and panics on `Pending`... becomes a
   live panic the moment an async runtime lands behind the same interface" — the repo's replacement is
   `block_on` at genuine thread roots only. `operation_envelope_serde::deserialize` is a per-call-site
   serde callback inside a turn, which **R4 explicitly lists as NEVER sanctioned** for `block_on`.

**What's actually needed** (coordinator-level decision, not mine to make unilaterally): either (a) expand
the lease to cover `📡️replication/🔗️causal/🦀️component.rs`'s `encode_envelope`/`decode_envelope` plus
their wire-primitive closure, accepting the ~220-call-site `.await`-removal sweep across `📡️replication`
(mechanically safe via `remove-bad-await.py`, but wide), or (b) redesign `ArtifactCommand::IngestRemote`'s
field to carry pre/post-encoded `Vec<u8>` instead of a typed `MutationEnvelope`, moving the encode/decode
to the (already-async) call sites that construct/consume the variant — I found 3 non-test and 3 test
construction/consumption sites in `🏪️store/🦀️component.rs` (lines 4043, 5021, 10477, 10603, 10619, plus
the match arm at 3849) that would each need auditing; did not attempt this given the correctness-critical
nature of VCS command dispatch and the ticket's explicit prohibition on scope-expanding architectural
changes without a lease.

## Acceptance — honest status against the brief's checklist

1. `cargo check -p semio-framework-os-kernel --lib` → **exit 101, 1 error** (not 0 — see above).
2. `--all-targets` / 3. `cargo test` / 4. `semio-framework-plugin --lib` headline: **all blocked**, same
   root cause — confirmed by actually running the plugin check:
   ```
   cargo check -p semio-framework-plugin --lib
   EXIT: 101
   error[E0599]: no method named `map_err` found for opaque type
     impl Future<Output = Result<protocol::MutationEnvelope, protocol::ProtocolError>>
     --> 🏪️store/🦀️component.rs:3485:58   (same error — semio-framework-plugin depends on
                                              semio-framework-os-kernel transitively)
   ```
   `semio-framework-plugin` cannot report a real number until this one error resolves — it is not a
   separate, parallel blocker.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
- `🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️component.rs`
- `🧰️framework/🔨️modules/📡️replication/🔢️scalar/🦀️component.rs` (granted lease)
- `🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️component.rs` (test call sites, granted lease's
  `--all-targets` fallout)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️component.rs` (granted lease's consumer side)

## Flagged, not fixed (out of this packet's line-item scope)

- **`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs` is dead code** — a full
  duplicate of `📦️glue.rs`'s content that diverged (still has the async `record_codegen`/
  `parse_container_attrs`/etc. that `📦️glue.rs` already had reverted to sync before this session, and now
  additionally missing all of THIS session's `.await`-insertion/E4 fixes). `Cargo.toml`'s `path =
  "📦️glue.rs"` means it is never compiled. Every other module in this repo uses `#[path]` from a thin
  `📦️packages/🦀️rust/📦️glue.rs` back to the real `🦀️component.rs` — this crate inverted that convention
  at some point. Left untouched (out of scope, does not affect the acceptance bar), but it is exactly the
  "artifact moved, its registration did not" shape rule 17 warns about and will confuse the next person who
  edits `🦀️component.rs` expecting it to take effect.

## Recommendation

Report this back to the coordinator with the E0599-at-3485 evidence above and ask for a decision between
the lease-expansion and field-redesign options. This is the single item scoping the `semio-framework-plugin`
headline number — nothing else in `kernel-finish`'s original characterisation is blocking anymore.
