# 🧱️ Stub & Placeholder Census

Repo-wide census of unfinished implementation surface, taken at ticket start
(HEAD `bb06c41f73`, branch `🐙ueli/⛳wip`).

## Method

`rg` sweep over source extensions (`.rs .ts .tsx .py .go .cs .cpp .hpp`), excluding
`node_modules/`, `target/`, `storybook-static/`, for the marker families:
`todo!(`, `unimplemented!(`, `TODO`, `FIXME`, `HACK`, `XXX`, `not yet implemented`,
`NotImplemented`, `stub`, `placeholder`, `coming soon`, `dummy`.

Raw hit count: **1023 lines**. After classification, the great majority are false
positives; the genuine unfinished surface is listed under *Actionable* below.

macOS note: there is no `timeout` binary; a `perl` alarm wrapper was used for
bounded CLI probes.

## Classification summary

| Class | Hits | Verdict |
|---|---|---|
| `MediaError::NotImplemented` (Rust) | ~206 | **False positive.** A designed protocol error variant. Default trait methods return it so apps can override; every site is documented. Not a stub. |
| `SCAFFOLD` generator markers (`pub const SCAFFOLD: bool = true`) | 2 | **Already done.** Only the two generators in `📜️script.ts` and the plugin-registry script still contain the marker string; zero generated scaffold leaves remain in the tree. |
| `♻️mit-bestand/recherche/_archive/**`, `_neo4j/review/**` (Python) | 78 | **Out of scope.** Archived one-shot research/migration scripts, not codebase implementation. |
| Go `t.Skip` | 110 | **False positive.** All in one file, all `-short`-mode or platform guards. |
| Rust `#[ignore]` | 17 | **Mostly benign** — dev-only fixture generators and integration tests needing pre-built artifacts. **One genuine** stale case (CAD, below). |
| `HACK` / `coming soon` | 9 | **False positive.** Doc prose and test fixture text. |
| Genuine unfinished implementation | — | **Actionable**, below. |

## Actionable stubs

### 1. Repo CLI dispatch was entirely dead — FIXED (this session)

`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/command/command.go`

Every subcommand of the repo client fell through to the usage banner. Two defects in
the repo's own hand-rolled (dependency-free) cobra replacement:

1. `Execute()` / `ExecuteC()` dispatched `command.args`, which is only ever populated by
   `SetArgs`. The real entry point never calls `SetArgs`, so the argument vector was
   always empty → root had no `RunE` → `Help()`. **No subcommand could ever run.**
   Fixed by adding `dispatchArgs()`, falling back to `os.Args[1:]`, with an explicit
   `argsSet` field so "never set" stays distinguishable from "set to empty".
2. `Flags()` did not resolve inherited persistent flags (cobra's does). Root registers
   `--json`/`--format`/`--repo` as *persistent*; `PersistentPreRunE` read them via
   `cmd.Flags().GetBool("json")` on the *selected* subcommand and silently got `false`.
   Output format flags were therefore inert. Fixed with a `FlagSet.resolve` that falls
   back to the owning command's ancestor persistent flags.

Consequences of the fix: the repo CLI works again, `--json` is honoured, the
`📜️script.ts policy` gate runner (which shells out to `client … graphql`) is unblocked,
and the `semio` repo **MCP server** — which is `client mcp` — can dispatch at all.

Regression tests added: `internal/command/command_test.go` (5 tests). Verified that
3 of them fail against a pre-fix copy of the package and all 5 pass after.

macOS trap recorded: overwriting the shared `client` binary in place gets it `SIGKILL`ed
by a stale code-signature cache. Replace via `rm` → `cp` → `codesign -s - -f`.

### 2. Repo GraphQL mutation resolvers — ⚠️ CENSUS ENTRY WAS WRONG

Original reading: ~25 `mutationResolver` methods in `…/💻️client/⌨️cli/🐹️component.go`
(~33757–34026) return `fmt.Errorf("not implemented")`, so the repo MCP ticket/goal/todo tools
are unimplemented.

**That was a false positive from grepping the string.** Every one of those methods delegates to
`r.Ctx.X(...)` on the `RepoContext` interface; the `fmt.Errorf("not implemented")` is only the
**nil-`Ctx` guard**, which never fires in production (`NewResolver`/`NewResolverWithContext`
always install a real context). The real implementations live on `*repoContext`, and the cobra
subcommands reach them through the very same resolver — the resolver *is* the shared
implementation, so there was nothing to deduplicate.

Checked individually, **24 of 25 were already fully implemented** (real file I/O, git/GitHub
calls, event emission). Exactly one genuine stub existed:

- `(*repoContext).TodoChange` — was a bare `return nil, fmt.Errorf("not implemented")`.
  Now implemented: locates the todo via `ScanTodos` (shared with `TodoCreate`/`TodoDelete`),
  merges `Name`/`Description`, rewrites the source line in place for both the `.todos.md` entry
  and the inline `// TODO name: desc` comment form (new `replaceLineInMarkdown` /
  `replaceLineInFile` helpers mirroring the existing `removeLineFrom*` pair), and emits
  `EventTodoChangeEnded`. ✅ done.

Lesson for the next census: a `not implemented` string is not evidence of a stub until you read
the call path — guard clauses look identical to stubs under `rg`.

### 3. `assert_outcome_policy_matrix` never landed

`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs` exports the 2-D
`assert_policy_matrix` but not the 1-D outcome variant. ~10 plugin files carry a
`TODO(1-D testkit laws pending)` comment deferring a `MergePolicy` × `Severity` law per
verb family, in: 🪵️sourcing/🗂️curate, 💠️lowpoly, 🪐️space/🏠️home, ✒️writer,
💡️reasoning/🔌️wires, 🖍️draw, 🏭️process/🧊️process3d, 🖨️raster, 📐️cad,
🔱️trinity/♻️rewrite, 🔱️trinity/🔌️jack.

### 4. TypeScript typegen is broken, forcing a hand-written placeholder

`🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts` ~627–765 hand-mirrors ~21 `Tutorial*`
types from Rust because `cargo test --features typegen` does not compile. The in-code
TODO blames `IconName`, which is **stale**; the real blocker is `CapabilityToken`
(u128 newtype, `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs` ~line 23) lacking a
`ts_rs::TS` derive while being a field of `BrokerCapabilityGrant` (~line 972) that
derives `TS`. A second TODO (~line 1160) defers folding `tutorials` into
`GeneratedAppDefinition`.

### 5. Graph DSL: WITH / UNWIND / CALL parse but never execute

`🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🦀️component.rs` ~line 2142 — the three clauses reach
the executor and are rejected with `GraphDslError::UnsupportedClause`, marked
`TODO(unify-architect)`.

### 6. Stale ignored CAD round-trip test

`✏️s/🔌️plugins/📐️cad/…/🧬️schema/📸️snapshot/📝️text/🦀️component.rs` ~line 38:
`#[ignore = "fixture predates the model/drawing composition rewrite; regenerate via
print_dsl before re-enabling"]`.

### 7. Hardcoded plugin icon metadata (layering violation)

`🧰️framework/🔨️modules/🖼️assets/🎯️concepts/🟦️component.ts` and
`🧰️framework/🔨️modules/🖱️ui/🖼️assets/🟦️icon_concepts.ts` both hold
`PLUGIN_DOMAIN_ICON_CONCEPTS`, a framework-side hardcoded roster of installed
apps/windows that should be plugin-declared manifest metadata.

### 8. `pack_cli` schema registry

`🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️component.rs` ~62: a fixed 2-entry
demonstration table marked `TODO(wave2)` against a real 49-kind registry said to be owned
by app crates. Needs a verdict: either wire the real registry, or make the ownership
boundary definitive and drop the stale TODO.

## TS mutation mirrors: what is and isn't a stub

Measured across `✏️s/🔌️plugins`, counting `🧬️mutations` leaf directories (`🦠️mutation`,
`🔺️diff`, `↩️inverse`):

- **334** leaf dirs have a `🟦️component.ts` mirror.
- **2650** have Rust but **no** TS mirror at all.

So a mirror is present for only ~11% of leaves. **An absent TS mirror is therefore the norm,
not a defect** — the mirrors are selective (where a WASM facade needs one), not a universal
parity requirement. Bulk-creating the missing 2650 would be fabrication, not completion.

The genuine stubs in this dimension are the files that **exist and describe themselves as
stubs** — 121 leaves whose entire body is a docstring saying "WASM facade stub" /
"WASM wiring stub" plus `export {};`. Those are the ones assigned out. Distribution:
🌀️procedural 68 (2d 25 / 3d 43), 🖍️draw 18, 📕️norm 15, 🪵️sourcing 6, 🧩️puzzle 3,
🗄️stdio 2, 🏗️fem 2, 🔋️energy 2, 🎪️demonstrator 2, and one each in 💡️reasoning, 🌍️gis, 🎥️shooting.

### Verdict from the repo's own gates

Read directly out of `📜️script.ts` rather than inferred:

- **`policyMutationTsMirrorBreaches`** (~28337) emits TWO breach families, both `priority: "low"`:
  `mutation-ts-mirror-stub-*` for `export {};` leaves under `🧬️mutations/`, and
  `mutation-ts-mirror-absent-*` for a Rust leaf with no `.ts` beside it at all (added
  deliberately so "a facet that never scaffolded its .ts leaves" doesn't look cleaner than one
  that did). Its own doc comment calls the stub shape "near-universal today", keeps it advisory
  "rather than seeding ~1000+ file paths", and states the intended fix: *"once the DSL TS codegen
  for this triad lands."*
  **Conclusion: the long-term answer here is codegen, not hand-authoring 2650 mirrors.** Filling
  the 121 self-described stubs is the tracked burn-down and is worth doing; bulk-generating the
  absent ones by hand is not.
- **`policyTsFacadeBreaches`** (~24083) only fires on facades that *throw* a WASM placeholder
  (`policyTsFacadeIsScaffoldStub`: `/to plugin WASM/i` or `throw new Error("…WASM…")`), and only
  outside constitutional facets. Plain `export {};` at a facet root is **not** a breach.
  **Conclusion: the 965 `export {};` facet-root leaves are tolerated mount markers — leave them.**

### Throwing WASM facades — genuine runtime stubs

Ten TS facades whose every exported function body is `throw new Error("wire … to plugin WASM")`
while the docstring claims it "delegates to the plugin Rust crate". These fail at runtime, so
they are real stubs regardless of gate priority:

- `📐️cad` ×5 under `…/🪆️subsets/✳️any/🧬️schema/` — `🔺️diff/📝️text`, `🧬️mutations/{📝️text,💾️binary}`,
  `📸️snapshot/{📝️text,💾️binary}`
- `🗒️note` ×5 under `…/🪆️subsets/✳️any/🚪️io/` — the same five shapes

`📝️text` wraps `🗣️dsl parseDsl/printDsl`; `💾️binary` wraps `📡️spr` and `🎒️pack` encode/decode.

### Empty presence schemas — verdict: NOT stubs

The 10 `export interface *Presence {}` leaves (norm, imperative, fem/2d, fem/3d, vcs,
animate/present, playbook, forms, space/home, mathematical) each have a sibling
`👥️presence/🦀️component.rs` whose `pub struct XPresence {}` is **also empty, by design**.
Each artifact routes its would-be presence state elsewhere:

- norm, vcs, home, forms, mathematical — view state is per-user local config, nothing shareable.
- imperative, animate/present, playbook — their one shareable field (step/tile/block selection)
  already moved to the framework's generic `PresenceInteraction` / `PresencePeer.interaction`
  (ticket `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`).
- fem/2d, fem/3d — selection is command-transient; camera/result display live on
  `Fem3dConfig`/`Fem2dConfig` as local UI state.

Rust is the source of truth and it is empty for a documented reason, so no fields were
fabricated. Each interface kept its empty body and gained a definitive doc comment citing the
`.rs` rationale, so none still reads as an unexplained blank.

## Deliberately excluded (documented seams, not stubs)

## Deliberately excluded (documented seams, not stubs)

- `🛢️db/🌐️cluster` — `db_query` / `db_preview` wave deferrals, explicit and scoped.
- `🛢️db/🗄️storage/🌐️neo4j` — documented transaction/concurrency seam.
- `🗣️dsl/🧪️fixture-sweep` — the word "stub" appears in its *detection heuristics*.
- `⏳️async` web-host counterpart — documented future seam.
- `unimplemented!()` in `🖼️render/🦀️element.rs`, `🔄️machine/🦀️.rs` and the draw FSM —
  deliberate test-double panics ("not exercised by this test", "host tests never step a
  machine").
- React `hostApp` `"Space"` literal — a working default; label-sourcing follow-up only.

## Operational notes

- `🗑️generated` folders are swept repo-wide mid-session. Long-running command output must
  go to the session scratchpad; only markdown reports belong in the ticket folder.
- The cargo workspace is shared with concurrent sessions. Build per-crate (`-p`), never by
  workspace total, and treat unrelated crate breakage as someone else's in-flight work.
