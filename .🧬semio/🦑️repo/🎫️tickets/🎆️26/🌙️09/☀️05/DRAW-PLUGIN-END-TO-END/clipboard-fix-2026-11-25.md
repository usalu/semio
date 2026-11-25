# Clipboard: three causes (2026-11-25)

1. Selection: layer rows were keyed by a row id the interaction topology does not know, so the stored selection was pruned (see `draw-2026-11-24.md`, r39).
2. Trap with a real selection (`retained source is closed: retained source original owner is absent`, guest panic in r39, found with `probe-canvas-copy.mjs`): `SnapshotReadSourceStaging` publishes its source in two turns. The first only places an unborrowable shell (`publication_pending`). `DrawingClipboardJob::seal_source` took any present source for sealed and borrowed it. It now waits until the staging is terminal.
3. Stuck refusal: the refusal also ran on the unfinished shell. The job then closed a source whose publication was half done: it is never terminal-empty and nothing is left to close, so the operation stayed in its worker stage (`term=true`, native `browser_grants_copy_without_selection…`, 4096 turns). The close now finishes the publication before it retires the workspace.

Also: a refused clipboard verb completes its job with the authored fault instead of a job fault whose text is only an id, so the notice carries the message (en/de).
