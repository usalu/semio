# Wave 5b — C2 unlink: verdict (orchestrator)

## Operational note
The investigating agent was mistakenly dispatched with worktree isolation
(my error). It correctly made zero edits and its findings file landed in
the worktree copy rather than the live ticket folder; copied into place
here, and the worktree + its branch were removed (confirmed empty of any
real change first). No impact on the codebase.

## Verdict: BLOCKED — third confirmed case of the same pattern

Full findings: `📓️w5b-c2-unlink.md`. Summary: unlinking procedural's 7
real Cargo dependencies on flow extensions requires four stacked pieces of
work, none of which exist as working infrastructure today:
1. No host-import exists for a plugin-world GUEST to invoke an
   extension-world component — the only two `host.invoke-action`
   implementations in the entire codebase are literal `"not implemented"`
   stubs.
2. `ExtensionRuntime` (proven to work in isolation by the prior wave's
   prototype) is never constructed anywhere in the real codebase outside
   its own unit test — no host-side registry loads/keys real extension
   components by id in any actual boot path.
3. None of the 7 flow-extension crates declare the `component-extension-guest`
   feature at all (all 7 `Cargo.toml`s checked, byte-identical `[features]`
   tables) — building them as real extension-world components requires
   authoring guest wiring from scratch per crate, not a feature flip.
4. The brep-kernel geometry sharing between procedural and the brep
   extension is a live, documented in-process coupling (`GeometryHandle`
   store) that a real component boundary would break unless resolved by
   design first.

This is the SAME shape of finding as the playbook relocation (Wave 5a) and
the flow-core relocation (Wave 5b phase 1): a real, evidence-backed
architectural blocker that genuinely can't be forced through in one pass.
Correctly declined the "just add the WIT stub" partial-credit option too —
without the (nonexistent) host-side registry, a lone stub is dead code
touching a WIT surface every guest shares, not real progress.

## Disposition
C2 (procedural → 7 flow-extension Cargo deps) remains as-is. It is a real,
narrow, well-documented plugin→extension violation, but fixing it requires
building genuine new runtime infrastructure (a host-side extension
registry wired into an actual boot sequence, guest-side component-extension
wiring for 7 crates, and a resolution for the shared brep-kernel coupling)
— substantial new-feature work, not a mechanical cleanup. Recommend a
dedicated follow-up ticket scoped as its own initiative, using this wave's
proven `ExtensionRuntime`/WIT mechanism as the foundation.

## Standing pattern across this ticket's three deferred items
Playbook relocation, flow-core relocation, and C2 unlink were ALL
originally scoped in the plan as mechanical moves/deletions. All three,
on real investigation, turned out to require either an architecture
decision (playbook/flow-core: who's allowed to depend on whom) or new
infrastructure (C2: a working extension-invocation runtime). In all three
cases the investigating agent made zero destructive/risky edits and
produced clear, evidence-backed reasoning — exactly the intended behavior
under this ticket's "stop and report rather than force it" instruction.
