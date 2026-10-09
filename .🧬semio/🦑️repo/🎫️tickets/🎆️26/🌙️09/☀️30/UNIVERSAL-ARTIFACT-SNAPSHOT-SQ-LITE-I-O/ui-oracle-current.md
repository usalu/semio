# Current UI Close and File Oracle Audit

Read-only source audit; before2 remains compiler-only and no Native BIM runtime is credited.

The two corrected original WGPU layout-session callers (`ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1495,2174`) pass the complete UI retirement grant, accept Pending or Complete only when the whole progress receipt fits that grant, and remove session ownership only after terminal_is_empty. Blocked or Refused keeps the original session. The cancellation path begins close on generation mismatch before making retirement progress. This preserves custody and does not confuse Complete carrying progress with an empty session.

Concrete limitation: Refused is folded into the same false admission result as Blocked. The ordinary layout path requeues and yields, and surface close remains Pending; a permanent refusal consequently has no surfaced reason or terminal failure in these branches. This is not a premature drop, but can become an endless close/requeue on a persistent invariant or policy failure. It should be assessed against intended UI refusal reporting rather than treated as successful cancellation.

The optional oracle `--file` suffix is parsed only at the final two arguments and selects readFileSync(file) instead of readFileSync(0). The successful stdout JSON/raw-binary contracts remain unchanged. Missing or malformed suffixes fall through normal mode arity rejection. Registered project.json sqlite-oracle target calls the owning script. At this read snapshot no launch.json invocation contains sqlite-oracle; root's stated GUI seed registration is still pending and cannot yet be credited.

Several unrelated original plugin callers still use obsolete WorkerJobCloseStep variants/signatures; they were observed in the shared tree but were not edited or used to retarget the BIM proof. Root and High own the actual compiler frontier.

This audit created only this short Markdown report and no source copies or generated output.
