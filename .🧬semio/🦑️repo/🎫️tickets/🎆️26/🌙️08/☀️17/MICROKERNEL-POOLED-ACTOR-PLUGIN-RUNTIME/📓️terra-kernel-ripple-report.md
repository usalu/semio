# 📓️ terra-kernel-ripple report

Packet: `kernel-ripple`. Goal: get `cargo check -p semio-framework-os-kernel --lib` green, then
report `semio-framework-plugin --lib`. **Not reached — still red.** This report documents what
moved, what's still broken, and exactly where the next packet should pick up.

## 1. Headline numbers (all measured live, foreground, this session)

| checkpoint | errors (`--lib`) | how measured |
|---|---:|---|
| coordinator's baseline (before I touched anything) | 808 | stated in brief, `spr` already asyncified there |
| after asyncifying the REST of my owned modules (`pack` os, `store`, `dsl`, `directory`, `inference`, `vcs`, `extension`) | 2967 | `asyncify-universal.py --apply` over each, then fresh `cargo check` |
| after the scoped `.await`-fixpoint (spr/store/io/pack/dsl/directory/inference/vcs/extension) | 1256 | `terra-scoped-await-loop.py` (see §2) to fixpoint |
| after manually clearing E0728 (misplaced-await, closures, struct-literal-shorthand bugs) | 1021 | this session's final `cargo check`, **exit 101**, pasted below |

```
$ CARGO_TARGET_DIR=<scratchpad>/target-kernel cargo check -p semio-framework-os-kernel --lib
   ... (1021 errors)
error: could not compile `semio-framework-os-kernel` (lib) due to 1021 previous errors; 17 warnings emitted
$ echo $?
101
```

**`semio-framework-plugin --lib` (the headline the coordinator is waiting on): still blocked.**
```
$ CARGO_TARGET_DIR=<scratchpad>/target-kernel cargo check -p semio-framework-plugin --lib
error: could not compile `semio-framework-os-kernel` (lib) due to 1021 previous errors; 17 warnings emitted
warning: build failed, waiting for other jobs to finish...
$ echo $?
101
```
`semio-framework-plugin` never gets past compiling its `semio-framework-os-kernel` dependency, so
this exact number (1021, exit 101) is what's blocking it. `--all-targets` and `cargo test` were not
run — `--lib` itself isn't green yet, and rule 26 only requires running the widening check once the
narrower one passes.

Error code histogram at the 1021 mark: E0308 285 · E0277 268 · E0609 194 · E0369 58 · E0599 57 ·
E0605 45 · **E0038 37** · E0600 29 · E0382 22 · E0499 9 · E0271 4 · E0505 3 · E0053 3 · E0716 2 ·
E0728 1 · E0507 1 · E0506 1 · E0311 1.

By module (primary-span file):

| module | errors remaining |
|---|---:|
| `🗣️dsl` (incl. `📖️grammar`, `🧬️schema`, `🖋️notation`, `🔤️token`, `👪️family/*`) | 316 |
| `📡️spr` (incl. `📜️history`, `🧪️testkit`, `🔌️io`, `💎️materialize`, `🎮️command`, `⌨️cli`, `🧵️channel`) | 275 |
| `🏪️store` | 239 |
| `🎒️pack` (os product module — NOT the framework `🎒️pack` crate, which stays green/untouched) | 76 |
| `🚪️io` (framework module) | 52 |
| `🌿️vcs` | 9 |
| `📇️directory` | 7 |
| `💡️inference` | 6 |
| `🧩️extension` | 4 |
| stdlib macro expansion (`assert_eq!` etc., attributed to the calling file really) | 35 |

## 2. What I actually did

**Asyncify.** `📡️spr` and `🚪️io` were already fully asyncified (0 converted on scan) — the coordinator's
work. I ran `asyncify-universal.py --apply` over the rest of my owned tree: `🎒️pack` (os, 170 fns),
`🏪️store` (726), `🗣️dsl` (798), `📇️directory` (57), `💡️inference` (42), `🌿️vcs` (26), `🧩️extension` (21).
This is what took the error count from 808 → 2967 (expected — every converted fn's call sites now
need `.await`).

**`terra-scoped-await-loop.py`** (new, in this ticket folder). `insert-await.py` aborts its ENTIRE
run the moment ANY E0728 shows up ANYWHERE in the crate, even outside `--scope`. I hit exactly that:
an E0728 rooted in `🧰️framework/🔨️modules/📡️replication/📖️dictionary/🦀️component.rs` (not my path)
blocked the shared tool for my whole packet. My driver reuses `insert-await.py`'s own
`run_check`/`collect_await_edits`/`apply_edits`/`in_scope` (imported, not copied) and only changes
the abort policy: an E0728 whose primary span is inside my `--scope` still aborts (that means MY
code needs asyncify-first, correctly); one whose span is outside my scope is reported and skipped so
other scoped work keeps progressing. I also added a second collector for rustc's
`help: consider `await`ing on both/all `Future`s` diagnostic — `insert-await.py`'s
`collect_await_edits` treats the multiple spans in that ONE combined suggestion as mutually
exclusive candidates ("ambiguous"), when they are actually all meant to be applied together; my
`collect_both_futures_edits` recognizes that specific message and applies every span in it. Running
this to fixpoint across all 9 owned modules took 808→1256-ish over several passes; the residual is
what the tool cannot do (structural, not textual).

**Manual fixes — five recurring bug classes, found repeatedly across the tree:**

1. **Misplaced `.await` on a reused binding.** `let mut out = ByteWriter::new(); out.await.write_u8(1); out.await.write_u8(2); ...` — `.await` landed on the BINDING instead of each method call, so every use after the first is a use-after-move (and the first use silently drops the needed trailing `.await` on the method itself). Correct shape: `let mut out = ByteWriter::new().await; out.write_u8(1).await; ...`. Wrote two throwaway scripts for this (`fix_reused_await.py`, `add_missing_await.py`, both in the ticket folder) and ran them for `out`/`reader`/`writer`/`payload`/`indexed_out`/`args_writer`/`sub` across `📡️spr/📜️history`, `📡️spr/💎️materialize`, `📡️spr/🔌️io`, `🏪️store`, `🎒️pack/🔢️value`, `🎒️pack/⌨️cli`, `🗣️dsl/🧬️schema`. ~180 individual `.await` repositioned this way, verified zero `.await.await` afterward.
2. **Invalid struct-literal shorthand from the async codemod**: `Struct { id.await, other_field }` instead of `Struct { id: id.await, other_field }` — `IDENT.await` is not a valid shorthand field init. Found and fixed ~15 instances by hand across `🏪️store`, `📡️spr/📜️history`, `📡️spr/🧪️testkit`, `📡️spr/🎮️command`, `🎒️pack/🧪️testkit`, `🗣️dsl/🧬️schema`, `🗣️dsl/📖️grammar`.
3. **`.await` inside a sync closure** (`Iterator::map`/`any`/`filter`/`find`/`fold`, `Option::ok_or_else`/`filter`, `Result::map_err`) — E0728, since std combinator closures are externally-fixed sync signatures. Restructured ~20 sites into explicit `for` loops or pre-computed locals (`🏪️store`: `apply_ops_binary`, `print_edit_lines`/`print_ops_log`'s inverse mapping, `history_op_payloads`, composition-pins mapping, `parse_document_spr`'s `decode_op`, `build_history_columns`, `amend_command`'s `amend_target`, four `mutation_envelope_from_edit` mappings, `receive()`, `assert_operation_round_trip`; `🗣️dsl/🧬️schema`: `print_table`, the `Statements`/`Tuple` JSON-schema arms; `🗣️dsl/📖️grammar`: `header_fixed_size`, `walk_fields`'s reserved-tail sum).
4. **E4 fn-pointer-slot violations** (R2/O1): a value stored in a bare `fn` pointer field cannot be `async`. `DslIdiom` trait + `GreetIdiom` impl + `hooks_for`/`passthrough_hooks` + the `IDIOM_REGISTRY` cluster (`🗣️dsl/🦀️component.rs`) were blindly asyncified even though `IdiomHooks.canonicalize/classify/complete` are plain `fn`. Reverted to sync, tagged `E4`. Same pattern in `🗣️dsl/📖️grammar`'s `MacroMatcher.try_match: fn(&str) -> bool`: `macro_table_ok`/`macro_quantity_ok`/`macro_props_ok`/`macro_hex_ok` reverted to sync (tagged E4); the one closure that genuinely needs a real async call (`parse_edge_text`) is bridged via `crate::os_io::resolve_ready` (the SAME `resolve_ready` the `🚪️io` module already uses for this exact pattern — reused the existing one, did not add a second E5 bridge).
5. **E1/R9 pure-accessor reversions**, each verified no-I/O and check for out-of-scope consumers before reverting: `TouchedPaths::{new,segments,intersects_prefix,intersects_any}` (`📡️spr/🎮️command`, consumed inside `Iterator::any` closures); `Symbol::{intern,as_str}` + `interner()` (`🗣️dsl/🔤️token` — 112 sync call sites vs. 8 wrongly-`.await`ed ones, reverted, fixed the 8); 14 helper fns in `🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs` (a `#[proc_macro_derive]` crate — proc-macro entry points are E3, and everything reachable from their synchronous call graph must stay sync too; this is the SAME defect class `dyn-enum-macro`'s finding 5 flagged in `semio-framework-schema-derive`/`draw-fsm-macros`).
   - **Caught and corrected my own overreach**: I first reverted the WHOLE `🗣️dsl` language-registry cluster (`language_registry`/`preflight_languages`/`register_languages`/etc.) to sync alongside the idiom cluster, then found `🔌️plugin/🦀️component.rs` (a live ATOMIC packet's file, not mine) already awaits `preflight_languages`/`register_languages`. Reverted my reversion for those two + their shared `language_registry` accessor back to `async fn` (R9 rule 3: the external consumer can be async, so that wins), while keeping the E4-forced idiom cluster and the local-only `language`/`language_for_extension`/`language_for_semio_content` lookups sync. Verified with `rg` that no other owned file calls the language-registry fns with a shape that would break either way.

**E0733 (recursion in an async fn requires boxing)**: 4 sites, all real self- or mutual-recursion
newly exposed once the call sites were correctly awaited — `Box::pin`'d each: `🗣️dsl/🧬️schema`'s
`shape_json_schema` (self-recursive via `List`/`Map`/`Tuple`/`Block`) and its mutual-recursion edge
through `collect_record_spec_properties`; `🎒️pack/🔢️value`'s `encode_dsl_value` (self-recursive via
`Array`/`Object`); `🏪️store`'s `LocalStorageBackbonePort::{read,write}` (recurses through the
enum-dispatched `BackbonePorts` when the host port happens to resolve back to the same concrete
type).

## 3. Genuine blockers — not fixable inside my `path_scope`, flagged for lease/next packet

**`lease-request` #1 — `🧰️framework/🔨️modules/📡️replication/📖️dictionary/🦀️component.rs`.**
`DictBuilder`/`DictReader` (both, all methods: `new`/`intern`/`len`/`is_empty`/`entries_since`/
`extend`/`resolve`) are pure in-memory `Vec<String>`/`HashMap` wrappers with zero I/O, blindly
asyncified. This blocks `📡️spr/📜️history`'s `read_id_field` (needs to pass `dict.resolve(idx)` into
`crate::os_pack::...::read_id`'s `resolve: impl Fn(u32) -> Result<&'r str, PackError>` parameter — a
SYNC closure signature fixed by `read_id`, also in replication) — the one remaining E0728 I could not
clear (`📡️spr/📜️history/🦀️component.rs:607`). Fix: revert every method on both types to sync (R9 E1),
same pattern as `Symbol`/`interner()` above.

**`lease-request` #2 — `🧰️framework/🔨️modules/📡️replication/📐️format/🦀️component.rs`, `FrameCursor::prev_frame`.**
Same shape of problem, different consumer: `📡️spr/📜️history`'s `HistoryEditIterator::next` implements
`std::iter::Iterator` (E1, externally-fixed sync signature) and calls `ready.cursor.prev_frame()`
without being able to await it. I did not chase whether `prev_frame`/`RecordFrame` do real I/O
(replication crate, out of scope, and I was already deep in budget) — flagging for whoever owns that
file next; if it's pure buffer parsing (likely, matching the ByteReader/ByteWriter pattern already
established), revert to sync.

**Not done — `E0038`, all 37 in `🚪️io/🦀️component.rs`, four traits: `ResourceResolver`,
`PayloadSource`, `PayloadSink`, `RandomAccessPayload`.** This is real de-dyn work but I did NOT apply
`#[dyn_enum]`/`dyn_enum_close!` and want to be explicit about why rather than force a bad mechanical
fit: these four traits are a genuinely OPEN host-extension boundary (a plugin/host embedding this
codec framework supplies its own resolver/source/sink at a point this crate cannot see at compile
time) — `rg` confirms exactly ONE impl of each, and all four are `#[cfg(test)]` fixtures
(`TestPayload`/`TestSink`/`TestResolver`); there is no closed, enumerable set of concrete types to
hand `dyn_enum_close!`. The `dyn-enum-macro` report's own guidance for this shape (`Migration`'s
open `&[&dyn Migration]` list) says a closed enum "does not apply as-is; needs a per-call-site closed
enum designed by hand" or a redesign — and R1 explicitly BANS the other obvious escape hatch
(`Pin<Box<dyn Future>>` in trait-method return position, i.e. hand-rolled `#[async_trait]`). I also
found and fixed four instances of an unrelated, trivial bug while reading this region (invalid struct
literal shorthand `Self { policy, budget.await, resolver: None }` → `budget: budget.await`,
`DecodeContext`/`EncodeContext` constructors) — those are fixed; the trait-object redesign is not.
Recommend this becomes its own packet: it needs an actual design decision (manual vtable erasure
matching this file's own `ComposeFuture`/`AsyncComposeFn` pattern, or collapsing to generics at the
top-level API and dropping `dyn` entirely), not a mechanical sweep.

## 4. Tools added (ticket folder, reusable by later packets)

- `terra-scoped-await-loop.py` — scoped `.await` fixpoint driver, see §2. Usage matches
  `insert-await.py`'s `--scope`, but accepts multiple `--scope` args and never hard-aborts on an
  out-of-scope E0728.
- `fix_reused_await.py` <file> <varname> [--apply] — finds `let (mut )?NAME = EXPR;` (not already
  `.await`ed) declarations, locates the enclosing fn by brace-matching, awaits the declaration, and
  strips `NAME.await` → `NAME` for every subsequent use in that fn.
- `add_missing_await.py` <file> <ident> <method1,method2,...> [--apply] — the necessary follow-up to
  the above: adds `.await` after bare `IDENT.method(args)` calls (paren-matched, not regex-fragile)
  for a given method allowlist, skipping ones that already have it. **Use both together, in that
  order** — `fix_reused_await` alone strips the misplaced `.await` but does not restore it at the
  call, which silently breaks the build worse if you stop halfway (I did this once, caught it via
  `cargo check`, and had to run the second script to repair it — see the tool's own docstring).

## 5. Files touched (owned paths only)

`🏪️store/🦀️component.rs` · `📡️spr/🎮️command/🦀️component.rs` · `📡️spr/📜️history/🦀️component.rs` ·
`📡️spr/💎️materialize/🦀️component.rs` · `📡️spr/🔌️io/🦀️component.rs` · `📡️spr/🦀️component.rs` (asyncify only) ·
`🎒️pack/🔢️value/🦀️component.rs` · `🎒️pack/⌨️cli/🦀️component.rs` · `🎒️pack/🧪️testkit/🦀️component.rs` (asyncify only) ·
`🗣️dsl/🦀️component.rs` · `🗣️dsl/🧬️schema/🦀️component.rs` · `🗣️dsl/📖️grammar/🦀️component.rs` ·
`🗣️dsl/🔤️token/🦀️component.rs` · `🗣️dsl/🧠️lsp/🦀️component.rs` · `🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs` ·
`📇️directory/🦀️component.rs`, `💡️inference/🦀️component.rs`, `🌿️vcs/🦀️component.rs`, `🧩️extension/🦀️component.rs` (asyncify only) ·
`🧰️framework/🔨️modules/🚪️io/🦀️component.rs` (E4 `resolve_ready` fixes + struct-literal fixes; the E0038 trait redesign is NOT done here).

Ticket-folder additions: `terra-scoped-await-loop.py`, `fix_reused_await.py`,
`add_missing_await.py`, `terra-kernelripple-check*.json` (scratch diagnostics, not committed
anywhere meaningful — safe to ignore/delete), this report.

## 6. What the next packet should do, in order

1. Lease `🧰️framework/🔨️modules/📡️replication/📖️dictionary/🦀️component.rs` and
   `🧰️framework/🔨️modules/📡️replication/📐️format/🦀️component.rs` (`FrameCursor` only — check for real
   I/O first), revert their pure accessors to sync. This clears the last E0728 and likely a chunk of
   the `📡️spr` E0308/E0277 count.
2. Re-run `terra-scoped-await-loop.py` with the same 9 `--scope` args — the mechanical fixpoint has
   more headroom now that entire dependency chains stopped being permanently-unresolved futures.
3. Design and implement the `🚪️io` E0038 fix (§3) as likely its own packet given the design work
   involved.
4. Once `--lib` is green, run `--all-targets` (rule 26 — hits test-only code I deliberately deferred:
   dozens of `#[cfg(test)]` fns across `🏪️store`/`🗣️dsl`/`📡️spr` still have missing/misplaced `.await`
   from the same codemod, e.g. `🏪️store/🦀️component.rs` lines 10500+/11200+ — same bug classes as §2,
   just not blocking `--lib`).
5. Baseline `cargo test -p semio-framework-os-kernel --lib` against the historical 779-passed number
   once it compiles — expect drift given how much changed; name the failing tests, don't just diff
   counts (rule 11).
