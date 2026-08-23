# P4d R9/R11 Independent Acceptance Audit — 2026-08-24

## Verdict

**GREEN — accept the B1–B3 P4d remediation.** The current source removes the R7/R8-era
cross-generation restore clobber, binds worker ingress to the raw/decoded/live registry owner
before a drive transition, and allocates the P4d semantic revision/generation pair with checked,
nonzero, permanent-exhaustion semantics. No new P4d acceptance blocker was found. This verdict is
limited to the requested P4d envelope/identity/exhaustion scope; P4e/P5b were not inspected as new
work or started.

## Evidence And Gates

- Read root and Puzzle instructions; the R7/R8 rejection; the implementation report's R9/R11
  section; earlier P4d rejection reports; current production, action reachability, fixtures, and
  permanent verifier source/diff.
- `rustfmt --edition 2021 --check` for P4d precompute, fill, and geometry: **PASS** (silent).
- Scoped working, staged, and `HEAD` `git diff --check` for those sources, the fill action, and
  `📜️script.ts`: **PASS** (silent).
- `bun 📜️script.ts verify interactivity --self-test`: **PASS**. It reported DENY mode clean with
  only the pre-existing structurally invisible test-only allowlist record.
- No Cargo, Nx, Wasm, browser, runtime, network, or broad build was run. Rust fixtures below are
  source evidence, not runtime-pass claims.

## B1 — Cross-Generation Restore And Terminal Progress: PASS

`restore_persisted_fill` decodes the complete nonzero token, holds the registry while it validates
the exact slot authority and immutable token page, and rejects before writing any session/engine
field when `fill_job` or a terminal handle identifies a different still-live request
(`precompute/🦀️component.rs:1478-1509`). It also rejects checkout of the requested authority.
Consequently restoring B into a session mounted on live A cannot clear A, replace `fill_job`, or
lose A's terminal owner. A stale local request may be replaced only after it is absent from the
registry. The only allowed same-token mount uses the exact request and leaves the builder
registry-exclusive; the checked-out terminal-handle route remains the one exact resume path.

The direct phase fixture covers Measuring, Admitted, Complete, Cancelled, Fault, and Closed,
asserts rejection preserves A's request/observation/phase/aggregate credit, and drains both A and
B to every slot empty plus aggregate credit zero (`:2325-2380`). The separate partial-Closing
fixture preserves the exact retirement-cursor address and cleared checkout state across rejection,
then drains both producers to zero (`:2383-2417`). This proves no reserved-slot livelock under the
requested hostile matrix and preserves R8 lost-handle reclamation.

## B2 — Exact Worker Producer Binding: PASS

The entry cursor takes `context.id()` then constructs a fault guard from the raw fixed-width
envelope before bounded decode (`:459-490`, `:1756-1771`). Before any `drive_fill_envelope` call,
`bind` requires all of: decoded `request.job == context.id()`, raw request equality with the
decoded request, and a live authority whose full request equals it (`:475-486`, `:1769-1789`). A
failed decode/bind drops the still-armed raw-owner guard, which records Fault only through the
raw token's exact slot/job/registry-generation identity; no job-only lookup remains. A stale raw
identity has no matching terminal intent and therefore cannot alter the replacement.

The malformed/wrong-context/stale fixtures respectively show the source owner faults while an
unrelated owner stays Admitted, the decoded producer faults before drive, and a stale identity
cannot fault its new slot occupant (`:2611-2685`). The permanent predicate and self-test mutate
the binding, stale check, and all three fixtures (`📜️script.ts:5682, 5710-5712, 5758-5760`).

## B3 — Semantic Identity Exhaustion: PASS

Every P4d Fill `Operation` construction is fed by `allocate_fill_identity`: it uses checked
increments, rejects zero, writes neither counter until both values are valid, and returns `None`
on every post-maximum attempt (`precompute/🦀️component.rs:775-789, 843-850`). The former wrapping
writes are absent. Rebuild and refresh are the only Fill operation construction paths, so they
both permanently refuse exhaustion without reset, saturation, or ABA. Token decoding independently
rejects zero registry generation, job, operation, semantic generation, and base revision
(`:84-123, 136-144`). Aggregate credit release is checked rather than saturating (`:665`).

The max/+1/repeated-refusal fixture proves both generation and revision behavior and preserves the
unconsumed paired counter on failed revision allocation (`:2741-2757`); the adjacent token fixture
rejects zero semantic fields and distinguishes exhausted identities (`:2759-2771`). The verifier
mutates both checked allocations back to wrapping form, zero acceptance, saturation, and the
fixtures (`📜️script.ts:5689, 5723-5726, 5761-5762`).

## Preserved P4d Invariants

R7 remains exclusive: post-admission reads borrow the registry owner, restore sets
`engine.fill = None`, and the only production `authority.fill.clone()` is the bounded worker-local
opportunity, dropped before return. R8's `Closing` owner remains rediscoverable exactly once via
`take_closed`, terminal-handle Drop records Closed intent then releases checkout, and close retains
one cursor through terminal empty. The fixed backed collection/credit and supersession fixtures
remain required by the permanent predicate; two action callers remain the sole production ingress.

No blockers.
