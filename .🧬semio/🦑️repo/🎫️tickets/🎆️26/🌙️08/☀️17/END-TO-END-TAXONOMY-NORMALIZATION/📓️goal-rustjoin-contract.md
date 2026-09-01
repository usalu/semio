# Rust path-join provability contract (rust-path-join-unproven)

Source: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts`
(`inspectRustManifestPathReferences`, `inspectRustManifestPathCandidates`, `inspectRustJoinArgumentSpans`),
consumed by `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`
(`rustManifestReferenceTokens`, `rustFiniteManifestTargets`).

## What the engine accepts as a "proven immutable manifest-relative base"

A Rust path expression is provable — and therefore rewritable during taxonomy
normalization — only if every `.join(...)` argument that contributes to a
filename literal is one of:

1. A **string literal token** (`"…"`, no backslash escapes), chained directly
   off:
   - `std::path::Path::new(env!("CARGO_MANIFEST_DIR"))`, or
   - `std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))`,
   - optionally through one level of parentheses,
   - optionally continuing through further `.join("literal")` calls.
2. A **local variable bound via `let name = <chain above>;`**, where the
   right-hand side is *entirely* consumed by the parser (must end exactly at
   the `;` boundary). The variable becomes usable as a new join base later in
   the same lexical scope. A `let` binding whose right-hand side is anything
   else (a bare literal, a runtime function call, a struct field) does NOT
   register as a proven base — even if the value happens to be a string.
3. **Exactly one exception for loop variables**: a `for` loop written
   literally as `for <single identifier> in ["lit1", "lit2", …] { … }`
   (the array must appear directly in the loop header, not through a bound
   variable or an iterator adapter such as `.iter().enumerate()`) is provable
   *only if*, across the entire loop body:
   - the identifier is referenced **exactly once** as a raw identifier token,
   - that one reference is the **sole argument** to **exactly one** `.join(...)`
     call,
   - the identifier is **never captured inside a format/print/assert/panic
     macro** (e.g. `"{name}"` inside `format!`, `assert!(cond, "{name}")`,
     `panic!("{name}")`). Any such capture — even alongside the valid join
     usage — invalidates the whole loop for provability, because the scanner
     treats format-string interpolation as a second, unprovable use.

Anything else that reaches a `.join(...)` call — a tuple-destructured loop
variable (`for (a, b, dir) in tuples`), a variable bound from a runtime
expression (`let dir = tuples[i].2;`), a JSON-derived value
(`descriptor["payloadSchema"].as_str()`), or a `.iter().enumerate()` loop —
is emitted as `rust-path-join-unproven` (code
`"Rust join argument has no proven immutable manifest-relative base"` for
join arguments not otherwise covered, or `"Rust finite candidate has no
writable literal authority"` for finite-candidate literals the engine
couldn't fully resolve).

## Consequence for our tests

`for (kind, variant, directory, tag, outcomes) in direct_owners { let owner =
mutation_root.join(directory); … }` can never be proven, no matter what the
loop body does, because the loop header binds a **tuple**, not a bare
identifier, over a **named variable** (`direct_owners`), not a literal array
written in the header. The only conforming fix is to eliminate the loop
entirely for the path-touching section and inline one literal `.join("…")`
call per owner directory (duplicating the same literal into a `let
directory = "…";` binding used only for messages/assertions, never for the
`.join()` argument itself — a bare `let` binding to a string literal does not
register as a proven base, so it must never be passed into `.join()`).

Loops of the form `for surface in ["a.ts", "b.graphql", …] { … owner.join(surface)
… }` ARE already provable *iff* `surface` is used exactly once and never
appears inside a `"{surface}"`-style format capture. Several of the affected
files phrase their failure messages as `"missing direct surface {surface}"`,
which alone is enough to break provability — the fix there is only to drop
the interpolation (or restate the message without the loop variable), never
to unroll the loop.

## Per-cause classification of this slice's files

- **Tuple-destructured loop over a named array** (needs full unroll, one
  literal block per owner): writer, gis/gisterrain, vcs, sequence,
  imperative, trinity/jack, trinity/rewrite, s/space, sourcing/curate; and
  stdio/pdf 1.4 `a`/`base`/`x` (`for (index, owner) in owners.iter().enumerate()`
  — an iterator-adapter loop, same defect class).
- **Single-owner file, only a `"{loopvar}"` format capture poisons an
  otherwise-valid surface loop** (message-only fix, no unroll): s/space/home,
  stdio/dwg architectural test (`for relative in […] { … "{relative}: …" … }`).
- **Genuinely dynamic join argument, unrelated to any loop** (rewrite the
  assertion to avoid `.join()` on a runtime value): energy/model
  (`owner.join(descriptor["payloadSchema"].as_str()...)` compared against
  `owner.join("🔣️payload.schema.json")` — replaced with a direct string
  comparison of the two values, which is `.join()`-free and semantically
  identical since both sides share the same `owner` prefix).
- **Already provable, no change needed**: demonstrator/playground (single
  literal owner, its surface loop never captures the loop variable in a
  message).

No filename literals were renamed. `🦀️component.rs`, `🔣️component.json`, etc.
stay in their pre-normalization form; the taxonomy engine rewrites them once
it can prove the containing expression.
