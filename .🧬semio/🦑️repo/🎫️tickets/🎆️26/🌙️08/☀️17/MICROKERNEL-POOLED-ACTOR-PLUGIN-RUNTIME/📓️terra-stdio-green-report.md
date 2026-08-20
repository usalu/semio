# 📓️ terra-stdio-green — report

Packet `stdio-green` on ticket `MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`. Scope:
`✏️s/🔌️plugins/🗄️stdio/**` exclusively. Mission: `cargo check -p semio-s-plugin-stdio --lib`
from the fleet-measured 18,757/18,758-error start toward EXIT 0.

**Verdict: NOT YET GREEN, but the SDK dependency chain went green mid-packet and this packet
resumed and drove the crate down to 5,695 (last tool-internal count) / independently reconfirming
at time of writing was blocked by a SECOND, unrelated live upstream break
(`semio-framework-os-kernel` ← `🗣️dsl/🔍️lexer`+`🔤️token`, a live peer session, not this
packet's).** Hypothesis CONFIRMED at scale and GENERALIZES further than first reported: beyond
the codec-helper/Fn-bound shape, this session also found and fixed (a) a regex gap that silently
skipped every GENERIC free/inherent function in both R9 tools, and (b) 76 in-scope-fixable
`store::ByteReader`/`ByteWriter` construction sites that were the single largest fan-out source
(1,464 of 5,965 errors at that checkpoint — 24.6% — traced to `PackError`/`ByteReader`/
`ByteWriter`), bridged with `resolve_ready` entirely inside `path_scope`, no framework lease
needed. Full chronology below; **the true final number for this wave awaits the `🗣️dsl` blocker
clearing** — do not read 5,695 as confirmed, it is the last number a tool self-reported.

## Headline numbers

| checkpoint | stdio errors (`cargo check -p semio-s-plugin-stdio --lib`) | how measured |
|---|---:|---|
| baseline, this packet's start (HEAD `bd1ce10b9b`) | **18,758** | direct, `terra-green-baseline.json`/`.stderr.txt`, exit 101 |
| after round-1 R9 revert (9,927 net top-level fns) | 27,290 | direct — EXPECTED regression, see "Method" |
| after E0728 fixpoint round 1 | 17,402 | direct, `terra-r9pass2.json` |
| after round-2 R9 revert (3,294 inherent-impl fns) + E0728 fixpoint round 2 | **12,077** | direct, `terra-r9pass3.json`/`.stderr.txt`, exit 101 |
| after `remove-bad-await.py` fixpoint (2,624 removed, 324 refused) | 0, claimed | tool-internal only — **proved FALSE**, see "Eighth tooling defect" below |
| — SDK dependency chain went green (coordinator-verified, `semio-framework-ui`/`-plugin`/`-os-kernel` all EXIT 0) — | | |
| **real post-`remove-bad-await.py` number** | **9,558** | direct, `terra-postgreen-measure1.json`, exit 101, could-not-compile line names `semio-s-plugin-stdio` itself |
| after `unwrap-bad-resolve-ready.py` (167+11+30 sites across several fixpoint rounds) + `remove-bad-await.py` re-run + 1 hand-fixed inner-span wrapper | **9,101** | direct, `terra-finalcheck.json`/stderr |
| after round-1.1 + round-2.1 generic-parameter fix (543 + 47 = 590 additional fns reverted) + E0728/bad-await/resolve_ready fixpoints | **5,965** | direct, `terra-innerscan2.json`, exit 101 |
| after 76 `ByteReader`/`ByteWriter` construction-site bridges + cleanup fixpoints | **5,695**, tool-internal only | remove-bad-await.py pass-1 self-report — **NOT yet independently re-verified**, blocked by the `🗣️dsl` upstream break, see "Second verification gap" |

**48.5%+ reduction confirmed (18,758 → 9,101, the last number independently reconfirmed via
direct `cargo check ... > file 2>&1; echo EXIT=$?`), likely higher once 5,695 (or whatever the
chain yields once green) is independently confirmed.** Every "direct" row above was obtained
without a pipe, per R23 counted from the full JSON diagnostic stream, not a text-line grep.

## Hypothesis test — CONFIRMED, with per-function evidence, not a blanket sweep

The brief's hypothesis: the dominant error shape (originally 7,242 E0271 "expected X to return
Y but it returns future" · 5,240 E0277 · 4,001 E0308, concentrated in the per-format
`🔺️diff`/`🧬️mutations`/`🚪️io` schema modules under `🗿️artifacts/**`) is **R9/E1 transitivity**:
the universal-async codemod made pure, I/O-free computation `async`, and its consumers are
language-barred from being async.

**Measured before editing anything** (gltf diff file, the single largest cluster, 1,816 of the
18,758 baseline errors): 320 `async fn`, 530 pre-existing `.await`, **zero** I/O markers
(`std::fs`/`tokio`/`reqwest`/`ureq`/`File::`/`TcpStream`/`spawn`/`sleep`/`SystemTime`). Its
consumers are `enc_*`/`dec_*` pure string/JSON helpers (`enc_str`, `dec_json`, `enc_json`, ...)
passed as bare fn-item VALUES to generic `Fn`-bound higher-order combinators
(`decode_option_option(extensions, dec_json)`, `encode_option(&d.uri, |v| enc_str(v))`) — a hard
language barrier no `.await` can cross, plus `format!`/`Display` sites and the `?` operator on
non-`Try` futures. This is R9 rule 2 exactly ("pure AND at least one consumer is E1/E3/E4"), one
level more specific than the brief's own worked hypothesis (Display/Iterator/serde/operator):
**the actual dominant consumer here is the `Fn` trait bound of generic combinators**, a sibling
hard-barrier class the brief didn't explicitly name but the same rule covers.

Broader confirmation across the whole crate (not just gltf): a dry-run scan for column-0
(module-top-level) `async fn` declarations found **9,947 candidates across 1,969 files**, with
only **22 files** failing the I/O-marker gate (skipped, untouched). A broader marker sweep
(`std::net`, `std::io::Read/Write`, `async_std`, `mio`, `hyper`, `curl`, `.lock().await`,
`RwLock`, `Mutex`, `mpsc::`, `oneshot::`, `.recv().await`, `actor::`, `host::`, `effects::`,
`.send(...).await`, `include_bytes!`) across all 1,969 matched files: **zero hits**. Spot-checked
the `🖊️dwg` `🚪️io` module specifically (215 top-level hits, the name "io" being the most
plausible false-positive risk) — its own doc comment states outright: *"Pure byte<->byte
algorithms with no `DwgSnapshot` dependency of their own"* (LZ77-variant decompression, LCG
header decrypt). The taxonomy folder name `🚪️io` in this codebase means "codec entry point", not
"performs I/O" — confirmed by content, not by name.

**Verdict: the hypothesis is correct and generalizes across the whole crate, not just the one
worked example the brief anticipated.** R9 is the dominant lever; missing-`.await` insertion
(what `insert-await.py`'s prior packets already exhausted) is not where the remaining volume is.

## Reversion criteria applied — written down for reuse by other fleet packets (sol's request)

Two tools, two passes, in the ticket folder:

1. **`terra-green-codec-r9.py`** (round 1): reverts a `(pub(crate) )?(pub )?async fn NAME(`
   declaration **only when it starts at column 0** (zero leading whitespace) in a file that
   passes the whole-file I/O-marker gate. Column-0 reliably selects module-top-level free
   functions and never a trait-impl method (every trait/inherent-impl method body in this
   codebase is indented). This is a **syntactic anchor, never a name list** (R10-safe) — it
   strips exactly the literal `async ` token immediately before `fn` on an already-qualifying
   line, and inserts a `// 🚫️async: E1 ...` tag once. It never touches a call site.
2. **`terra-green-codec-r9-inherent.py`** (round 2): extends round 1 to INHERENT `impl Type { }`
   methods (not trait impls, not trait declarations) in the same I/O-gated files, using a
   brace-nesting tracker that strips string/char-literal and comment content per line before
   counting `{`/`}` (so `format!("[{}]", …)` — extremely common in this exact file family — never
   desyncs the depth counter). A method is eligible only if **no** `trait_impl`, `trait_decl`, or
   `test_mod` stack frame encloses it, AND it is not immediately preceded (through blank/doc/
   attribute lines) by an attribute containing the substring `test` other than `cfg(test)` itself.
3. **Fallout cleanup, both rounds**: the ticket's existing `remove-bad-await.py` (E0277 "is not a
   future", diagnostic-driven) plus a new sibling this packet wrote,
   **`terra-remove-e0728-await.py`** — same span-keyed mechanics, but for `E0728 "await is only
   allowed inside async functions and blocks"`, which is the SAME defect (an orphaned `.await`
   after an R9 revert) reported under a DIFFERENT diagnostic code once the enclosing caller
   itself becomes non-async (the syntactic `.await`-requires-`async`-context check fires before
   the "is this a future" type check). `remove-bad-await.py`'s docstring only ever scoped E0277;
   this crate's revert volume was large enough to hit the E0728 variant at real scale (18,909
   sites on the first pass alone) and needed its own tool rather than a one-off hand fix.

**Self-correcting safety net, stated explicitly because it changes the risk calculus**: a
wrongly-reverted trait method throws an immediate, function-named compile error (signature
mismatch against the trait) — loud, not silent, unlike the `wrap-sync-closure-await.py` bugs a
prior packet found. A brace-depth desync in round 2 would show up as a parse error (self-revealing
per R18) or a miscount versus manual `grep -c` sanity checks. Both were checked before trusting
the tool's own report (see below).

**Evidence discipline, per function class, both halves shown**:
- I/O check: whole-file marker grep (round 1) / same file already gated (round 2) — zero hits in
  every file this packet edited.
- Language-barrier check: verified directly for the worked gltf example (Fn-bound combinators);
  generalized via the broader concurrency/IO marker sweep (zero hits across all 1,969 files) and
  the specific `🖊️dwg` doc-comment cross-check.

## Self-inflicted-corruption class found and repaired — the most transferable finding here

**Round 1's column-0 heuristic wrongly stripped `async` from 20 functions that were
`#[semio_framework_async_macros::async_test]`-attributed test entry points**, across 4 files
(`🖊️dwg/…/🚪️io/🦀️component.rs` ×1, `📄️pdf/…/🎓️bachelor-thesis/🧪️tests/🦀️test.rs` ×10,
`🎞️gif/…/💃️dancing/🧪️tests/🦀️test.rs` ×6, `🧊️gltf/…/🌱️metabolism/🧪️tests/🦀️test.rs` ×3).
Root cause: these test files apparently are not run through the same formatter as production
code, so their `#[test]`-attributed fns sit at column 0 despite conceptually being "inside" a
test module — column-0-only was not a sufficient proxy for "not a trait/impl method" in that one
corner. Found by a targeted grep (`grep -rB1 "🚫️async: E1 pure codec/computation helper" ... |
grep "#\[" | grep -v "cfg(test)"`, distinguishing the harmless `#[cfg(test)]` conditional-
compilation attribute — which is fine to revert past, since it's not a test *entry point* — from
the actual `#[test]`/`#[...::async_test]` harness attribute, which almost certainly requires its
input to stay a literal `async fn`). All 20 repaired by hand-scripted diagnostic (not compiler-
diagnostic — a pure text-pattern repair, since these were never compiled in a state that would
surface the mismatch as a normal error; verified zero-remaining by a full crate-wide sweep
afterward, and independently by confirming **zero** `#[...async_test]`-attributed functions
anywhere in `✏️s/🔌️plugins/🗄️stdio/**` are non-`async fn`).

Round 2's tool was written WITH the exclusion built in from the start (test_mod classification +
attribute lookback) — confirmed zero instances of the same defect class in its own 3,294-fn
output by the same cross-check.

**This is this ticket's eighth documented instance of a tool's own edit needing a self-audit
before being trusted (R16/R20's family)** — recorded here explicitly so the next fleet packet
that reuses these two scripts inherits the constraint (exclude `#[test]`/`#[...test]`-shaped
attributes, not just `#[cfg(test)]`) rather than rediscovering it.

## Sanity checks run before trusting the applied edits

- **Brace-balance sweep, whole crate, all 2,738 `.rs` files** in `✏️s/🔌️plugins/🗄️stdio/**`,
  using the same string/comment-stripping tokenizer round 2's tool uses to count `{`/`}`:
  **0 unbalanced files**. No gross truncation or brace-depth corruption from either tool.
- **Global `#[...async_test]` audit**: every attributed function in the whole scope is still
  `async fn` — **0 violations** — after the 20-site repair above.
- Both R9 tools are **idempotent line-signature edits** (never a whole-file rewrite) — the
  precise category of tool the ticket's rules warn hardest against when done carelessly (a prior
  session's brace-matching whole-file rewriter deleted ~16,000 lines this week); this packet's
  tools only ever rewrite the single matched `fn` line plus, once, insert a tag comment above it.

## First verification gap — RESOLVED, and the false "0" fully explained (eighth tooling defect)

The last independently confirmed number before the chain first went red was **12,077**
(`terra-r9pass3.json`/`.stderr.txt`, exit 101). Immediately after, this packet ran
`remove-bad-await.py` (E0277 fixpoint: pass 1 removed 2,624 of 2,948 flagged sites, 324 refused —
see "What's left"; pass 2's own internal re-check reported `errors=0`). That `errors=0` was
**not banked** — the very next independent `cargo check` failed at workspace manifest load (a
wrong relative-path count in a live sibling's `Cargo.toml` edit, since fixed by the coordinator),
and this report flagged the number as unverified rather than carrying it forward.

**The coordinator confirmed the chain green at 15:39** (`semio-framework-ui-scene`,
`semio-framework-ui`, `semio-framework`, `semio-framework-plugin`, `semio-framework-os-kernel` all
independently EXIT 0). Re-measuring directly: **`cargo check -p semio-s-plugin-stdio --lib` →
9,558**, not 0 (`terra-postgreen-measure1.json`/`.stderr.txt`, exit 101, could-not-compile line
names `semio-s-plugin-stdio` itself — the build genuinely reached and compiled this crate's own
files, so 9,558 is a real own-crate count, not an upstream artifact per R21).

**Eighth documented instance of a tool self-report needing independent verification, with a
diagnosed mechanism, not just a restated warning.** `remove-bad-await.py`'s `run_check()`:

```python
proc = subprocess.run(cmd, cwd=REPO, env=env, capture_output=True, text=True)
out = []
for line in proc.stdout.splitlines():
    ...
    if msg.get("reason") == "compiler-message" and msg.get("message"):
        out.append(msg["message"])
return out
```

never inspects `proc.returncode`, and a **workspace manifest load failure prints its error as
plain text to stderr — never as a `--message-format=json` line on stdout**, regardless of that
flag being passed. So when a manifest fails to load, `proc.stdout` contains zero JSON lines,
`out = []`, `diags = []`, `errors = []`, and the tool reports **"errors=0" — a false positive
indistinguishable, from inside the tool, from a genuinely clean build.** This is the SAME failure
shape this ticket has now named four times under different guises (R12's narrow grep pattern,
R21's "own errors=0 meaningless when the build aborts upstream", R23's `^error` undercount, and
now this): **a query that cannot report its own failure is not evidence of the thing it failed to
measure.** The timing corroborates the mechanism exactly — the coordinator fixed the offending
`Cargo.toml` path at 15:26, and this packet's own first post-"0" `cargo check` (run within
seconds of the tool's pass 2) is the one that hit the identical manifest error, meaning the bad
path was very likely already live when `remove-bad-await.py`'s pass 2 ran.

**One-line fix for the shared tool** (not applied by this packet — `remove-bad-await.py` and
`terra-remove-e0728-await.py` are shared infrastructure, editing them is a registrar-adjacent
change; flagging the fix here for whoever owns that decision): in `run_check`, check
`proc.returncode` and treat "zero diagnostics AND a non-zero exit that produced zero
compiler-message lines" as **inconclusive, not zero** — abort the pass rather than reporting a
fixpoint. Every packet on this ticket that has ever used `remove-bad-await.py`'s multi-pass
fixpoint loop is exposed to this exact failure mode.

## Second verification gap — OPEN as of this report

After the chain went green, this packet drove the crate from 9,558 down through
9,101 → 6,452 → 6,019 → 5,965 → **5,695** (see headline table; every step through 5,965 was
independently direct-measured, 5,695 is `remove-bad-await.py`'s pass-1 self-report only). The very
next independent `cargo check -p semio-s-plugin-stdio --lib` failed again — this time
`semio-framework-os-kernel` (36→35 errors observed across two retries, all in `🗣️dsl/🔍️lexer` and
`🗣️dsl/🔤️token`). **Confirmed by the coordinator to be their OWN authorized, live, in-progress
lease work** (an R9 revert of `lex`/`unescape_text`, scoped too narrowly on the first pass,
missing 23 orphaned-`.await` call sites at the consumer end — the coordinator's own fix, in
progress, lease extended). Correctly not touched by this packet (out of `path_scope`, and now
confirmed live rather than stale per a direct coordinator liveness statement, not just this
packet's own guess). **5,695 remains unbanked until this clears.** Whoever next re-measures this
crate: `cargo check -p semio-s-plugin-stdio --lib > f 2>&1; echo EXIT=$?`, check the tail for
`could not compile <other crate>` first (R21), and only then trust the own-error count.

## Round 3 (post-green) — the regex gap, and the empirical answer to the `ByteReader` lease question

Once the SDK chain went green, this packet acted on both open items from its own "What's left"
list, plus found a THIRD, previously-unsuspected defect in its own tooling.

### Ninth tooling defect — the generic-parameter regex gap in BOTH R9 tools

Both `terra-green-codec-r9.py` and `terra-green-codec-r9-inherent.py` matched fn declarations with
`async fn NAME(` — requiring the opening paren **immediately** after the name. Neither tool
accounted for a generic parameter list between them (`async fn foo<T>(...)`, `async fn
deserialize<'de, D: Deserializer<'de>>(...)`). A crate-wide grep for
`async fn [a-zA-Z_]+<` found **517 such declarations still un-reverted**, including several serde
`deserialize_with` helpers — textbook E1 (serde calls them synchronously, by path) — and pure
`Fn`-bound combinator callbacks (`apply_group_diff<T: HasFaces>`, passed by name to
`absorb_named_membership`), the exact shape this packet exists to fix.

**Fix, not a wider regex** (a wider regex was the actual R10-adjacent trap here — one real site
nests, `async fn parse_i<T: std::str::FromStr<Err = std::num::ParseIntError>>`, which a
non-nesting `<[^>]*>` would truncate at the first inner `>`): both tools now match the fixed
prefix, then hand-scan the optional `<...>` with bracket-depth counting, refusing (not guessing)
if the list isn't closed on the same line. **Before re-applying, both tools were also given the
`#[test]`/`#[async_test]` exclusion check** (round 1 never had it — see the eighth-corruption
section above; re-running it unprotected would have reintroduced the same 20 sites, confirmed by
re-checking that the SAME 20 test-fn names appeared in a pre-fix dry-run and were absent after).

Applied: round 1 (543 additional column-0 fns, test-exclusion holding — 0 of the 20 known test
names present in the dry-run output) + round 2 (47 additional inherent-impl fns). Re-verified: a
full crate-wide classification pass afterward found **zero remaining real candidates** — every
`async fn` still standing in an I/O-gated file is now either a legitimate trait-impl/decl method
(4,077) or correctly test-excluded (4,172, all irrelevant to `--lib`). Rounds 1 and 2 are now
exhaustive over their own criteria.

### The `ByteReader`/`ByteWriter` lease question, answered empirically instead of granted

1,464 of 5,965 errors at that checkpoint (24.6%) traced to `PackError`/`ByteReader`/`ByteWriter` in
their diagnostic text or children. Root cause, confirmed by inspection: now-sync stdio helpers
construct `let mut r = store::ByteReader::new(bytes);` / `let mut inner =
dsl::ByteWriter::new();` and never resolve the value — since `ByteReader`/`ByteWriter` remain
`async fn` (framework-owned, out of `path_scope`, blocked on the standing lease by two sites in
`🗣️dsl/🦀️component.rs`), the binding is `impl Future<Output = ByteReader>`/`ByteWriter`, and every
later use of it (`.read_varint_u64()`, `.into_bytes()`, `?`, iteration, ...) produces one
downstream diagnostic — one unresolved construction, many symptom errors.

**Fixed entirely in-scope, no lease needed**: a new tool, `terra-wrap-bytereader-construct.py`,
wraps the CONSTRUCTION call (not every downstream use) in
`semio_framework_plugin::resolve_ready(...)` — sound because both prior packets already confirmed
`ByteReader`/`ByteWriter` are I/O-free (zero suspension points), so resolving them immediately at
construction is correct regardless of whether the framework file itself ever gets reverted. Scoped
to the 89 files the CURRENT diagnostics actually named (not a blind crate-wide sweep of all
`ByteReader::new`/`ByteWriter::new` occurrences, which would have included call sites inside
functions that are legitimately still async and already correctly awaiting) — found 76 genuinely
unwrapped sites across 36 files, applied, zero refusals on the second pass (the first regex
version missed the common `store::`/`dsl::`-qualified-path form, fixed before applying).

**This answers the coordinator's question directly: the lease is not needed to make meaningful
progress against this residue class.** Fixing the call sites this packet owns is strictly better
than mutating a dependency shared by everything downstream of it, and the fan-out this ONE class
caused (a quarter of the crate's error count from ~94 unresolved constructions) is itself a
striking demonstration of how much a single unaddressed R9 site can cost once code depends on it
transitively — worth remembering for any future crate in this program hitting the same shape.

## Residue taxonomy — code histogram evolution

| checkpoint | E0277 | E0308 | E0599 | E0271 | other |
|---|---:|---:|---:|---:|---:|
| 12,077 (round 2 done) | 6,439 | 3,271 | 1,824 | 326 | 217 |
| 9,558 (post-green, real remove-bad-await result) | 3,947 | 3,264 | 1,824 | 326 | 197 |
| 9,101 (post resolve-ready unwrap + hand fix) | 3,490 | 3,264 | 1,824 | 326 | 197 |
| 5,965 (post generic-param round 3) | 1,433 | 2,363 | 1,750 | 260 | 159 |
| 5,695 (post ByteReader construct wrap, unverified) | — | — | — | — | — |

Dominant remaining shapes at the 5,965 checkpoint (`terra-innerscan2.json`): 2,325 "mismatched
types", 528 `Label: From<impl Future<Output = Label>>`, 402+235+80 `map_err` on opaque
`Future<Output = Result<_, PackError>>` (the ByteReader fan-out this round's construct-wrap
directly targets), 327 `?`-on-non-`Try`, 176 `UiNode`/`ComponentTree` future-resolution mismatches
(a DIFFERENT family — UI contract types, not this packet's R9 shape, likely belongs to the
`ui-w4-core`/scene-relocation work observed live this session), 129 `MutationApplyError` opaque
methods, 104-62 "not an iterator" pairs. File concentration is still the same `🔺️diff`/`🧬️mutations`
family (gltf, dxf-r12, docx, png, semio/document/presentation, bcf, pdf, jpg, obj — unchanged
shape from baseline), confirming the remaining volume is exhausting the SAME classes, not
surfacing a new one.

## What's left — honest inventory, not exhaustive

1. **The 22 I/O-flagged files skipped by both R9 rounds** (`std::fs` present) — inspected this
   session: in 21 of 22, the I/O is confined to `#[cfg(test)]` fixture-loading helpers (real disk
   reads of committed fixture files), irrelevant to the `--lib` gate this packet is measured
   against. These files DO still contain un-reverted PRODUCTION candidates (409 counted across 21
   files in an earlier pass, before the generic-parameter/test-attribute fixes — not re-measured
   with the final tool versions). Recommend: apply the same file-level gate but scoped to only the
   NON-test portion of each file (skip `#[cfg(test)] mod tests { }` blocks specifically rather than
   the whole file) — the round-2 tool's `test_mod` classification already exists for this and,
   after the `MOD_RE` fix made mid-session (see below), correctly detects `mod tests { }` blocks;
   it was not applied to these 22 files this packet because the file-level I/O gate short-circuits
   before that classification ever runs. A follow-up pass loosening the gate to "I/O confined to
   `#[cfg(test)]`" rather than "I/O anywhere" is the highest-leverage next step for these 22 files.
2. **A latent bug fixed mid-session, worth flagging for anyone reusing round 2's tool standalone**:
   `terra-green-codec-r9-inherent.py`'s `MOD_RE` originally required a trailing `{` in the matched
   text, but the classifier calls `classify_opener` on `pending_header` BEFORE appending the `{`
   itself — so `MOD_RE` could never match, and `test_mod` classification was dead code for this
   packet's first application (harmless there only because the file-level I/O gate already
   protected every file `MOD_RE` would have covered). Fixed (`\{` requirement dropped, matched
   against end-of-accumulated-text instead) as part of investigating item 1 above.
3. **`UiNode`/`ComponentTree` future-resolution errors** (176+ at the 5,965 checkpoint) — a
   distinct family, not this packet's R9 shape (these are UI-contract type mismatches, not a pure
   codec helper wrongly made async). Likely intersects with the live `ui-w4-core`/scene-relocation
   work observed this session in `🖱️ui/**`. Not investigated further — flagged, not fixed.
4. **The 261-error entry for `🔌️plugin/🦀️component.rs`** noted at the 12,077 checkpoint is almost
   certainly a cargo message-replay artifact (multiple crates' diagnostics interleave in one
   `--message-format=json` stream) rather than fresh damage — the stderr summary line for that same
   run named only `semio-s-plugin-stdio` as failing. Not chased further.

## Regression guards — NOT independently re-run this packet

The upstream chain was red (for two different, both out-of-scope reasons) for most of this
packet's session. The standing regression gates (`semio-framework-os-kernel --lib`/`test`,
`semio-framework-os-kernel-db`, etc.) were not re-verified directly this session — this packet made
zero edits outside `✏️s/🔌️plugins/🗄️stdio/**`, so there is no mechanism by which its own edits could
regress them, but the coordinator's own message notes `semio-framework-os-kernel` is CURRENTLY red
(the 🗣️dsl lease's orphaned-`.await` fallout, their fix in progress) — so "779/0" is explicitly NOT
current as of this report. Stated as a reasoned absence for THIS packet's edits, not a claim that
the gate is green right now.

## Forced-rebuild dropped-future census (R12/R13/R17) — NOT RUN, correctly

The crate is still red (last independently confirmed: 9,101; last tool-self-reported: 5,695,
unverified). R17 is explicit: a census taken through a red crate is meaningless. Not attempted.
**Hard condition for this packet's acceptance, per the coordinator**: the moment
`semio-s-plugin-stdio --lib` reaches EXIT 0, run `cargo clean -p semio-s-plugin-stdio`, rebuild,
grep the exact phrase `unused implementer of`, plus the `let _ = ` grep (R13's corollary) — with
13,221+ functions de-asyncified in this crate this session, the inverse risk (a call that
genuinely needed awaiting and now silently never runs) is real, compiles clean, and is exactly the
class that has produced every genuine production bug found on this ticket so far.

## Lease-requests

**The standing `store::ByteReader`/`ByteWriter` lease-request is WITHDRAWN by the coordinator's
own ruling, not by this packet** — answered empirically instead (see "Round 3" above): 76
in-scope construction-site bridges captured a large share of the fan-out without touching the
framework file, and the coordinator has explicitly closed the lease rather than granting it. No
other lease-requests open from this packet.

## Tools written this packet (all in this ticket folder, each documents its own defect/method in its docstring)

- `terra-green-codec-r9.py` — round 1, column-0 top-level R9 revert. Amended mid-packet twice:
  (a) added the `#[test]`/`#[async_test]`-attribute exclusion after the 20-site corruption was
  found and hand-repaired; (b) added the bracket-depth-aware generic-parameter matcher after the
  517-site regex gap was found.
- `terra-green-codec-r9-inherent.py` — round 2, inherent-impl-method R9 revert with brace-nesting
  classification and test-attribute exclusion. Amended mid-packet: same generic-parameter matcher
  fix, plus a `MOD_RE` bug fix (never matched — see "What's left" item 2).
- `terra-remove-e0728-await.py` — E0728 sibling of the ticket's existing `remove-bad-await.py`.
- `terra-wrap-bytereader-construct.py` — bridges unresolved `ByteReader::new`/`ByteWriter::new()`
  construction sites with `resolve_ready`, scoped to compiler-confirmed-affected files only.

Reused as-is, unmodified: `remove-bad-await.py` (its `run_check` false-"0" defect diagnosed and
documented above, but the tool itself not edited — shared infrastructure, registrar-adjacent),
`unwrap-bad-resolve-ready.py`, `strip-redundant-resolve-ready-await.py` (present, not needed this
packet — no matching residue shape appeared).

## Files touched (production code)

No individual enumeration — across all rounds this packet touched roughly **13,221 + 590 = 13,811
function signatures** (async→sync reversions) plus **76 `ByteReader`/`ByteWriter` construction
wraps** and **one hand-fixed inner-span `resolve_ready` unwrap** (the DWG bit-cursor `.map()`
closure site), spread across on the order of 2,000 files under
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/**` (plus `🚪️io`, `📚️examples/…/🧪️tests`), all mechanical
single-line edits plus tag-comment insertions, followed by several rounds of the diagnostic-driven
await-removal/resolve_ready-cleanup tools (tens of thousands of span-keyed edits across the same
file set, exact per-round counts in the headline table and body above). Every edit stayed inside
`✏️s/🔌️plugins/🗄️stdio/**`; nothing outside scope was ever written to, including no `Cargo.toml`
anywhere, confirmed repeatedly via `git status` filtered to this packet's own scope.
