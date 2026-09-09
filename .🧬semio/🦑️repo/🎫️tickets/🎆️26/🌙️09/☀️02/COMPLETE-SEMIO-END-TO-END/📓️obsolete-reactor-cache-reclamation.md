# Obsolete Reactor Cache Reclamation

## Scope And Authority

Home, the owning execution agent, confirmed that the ticket-generated
`reactor-lifecycle-native-target` directory is an obsolete Cargo-only cache from
the already-qualified document-backbone/binding packet. It is not a source input
and is not planned for reuse. Current proposal, creation and process builds use
the separate protected `hub-target`.

The clean skill was read, but its workspace-wide command cannot be applied while
other agents and current ticket outputs are active. No global clean command is
being run. Only the exact owner-confirmed cache is in scope.

Read-only checks found13GiB of Cargo cache content: `debug`, empty `tmp`,
`CACHEDIR.TAG`, rustc metadata and future incompatibility output. The directory
is not a symlink. Process argument inspection found only the validation command
itself; lsof completed with exit1 and zero bytes of output, indicating no open
files and no reported access error. The cache can be rebuilt; source, reports,
fixtures, current Hub target and other agents' targets are preserved.

The forced removal form was rejected by the execution tool before running.
The permitted non-forced removal of this exact directory then completed with
exit0 (session43753). The target is absent, and post-delete disk availability
was19GiB, up from roughly6.5GiB immediately before reclamation. Only this
rebuildable Cargo cache was removed; the source and proof reports remain.
