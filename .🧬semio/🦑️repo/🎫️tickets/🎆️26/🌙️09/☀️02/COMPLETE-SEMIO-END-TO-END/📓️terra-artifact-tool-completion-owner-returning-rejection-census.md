# Artifact Tool Completion Deep-Retirement Re-Audit

## Verdict

**RED — returning an `ArtifactToolCompletionRejection` fixed only the first
loss boundary. The generic completion jobs and migrated plugin callers still close just the
`ChildEmit` list and then whole-drop the rejected typed `Emit`,
`EphemeralEmit`, and `Fault`; `complete_download` and `take_emit` can lose a
download owner before a closer even exists.** The newly staged Puzzle5d
closer is a useful type-specific counterexample, not a generic fix.

This is a current-source audit only. No build, native law, or runtime path was
run by this lane.

## Current ownership model

- `ArtifactToolCompletionValue<A>` stores either `Emit` or `Download` with an
  `EphemeralEmit` at
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:13122-13125`.
  Stored faults are bounded to 256 bytes, but *rejected* inputs remain their
  original typed values.
- `ArtifactToolCompletion::complete` correctly returns the original
  `Result<Emit<…>, Fault>`, ephemeral owner, and rejection fault on lock
  contention or duplicate assignment (`13315-13319`, `13336-13359`). It does
  not itself close them.
- `Emit` contains three typed mutation vectors, two optional strings, effects,
  events, child emits, and tasks (`10213-10240`). `EphemeralEmit` contains
  typed presence and transient vectors (`10368-10395`). `Fault` contains
  arbitrary strings, optional scoped IDs, and a vector of cause strings
  (`🧰️framework/🔨️modules/⚠️diagnostic/🦀️.rs:340-365`). Closing only a child
  does not make any of those other owners terminal.
- `Emit::close_child_one` is deliberately just the child authority
  (`10488-10498`); it cannot certify or retire arbitrary app mutation,
  effect, event, task, presence, or transient types.

## Confirmed remaining loss paths

| Boundary | Current behavior | Why it remains RED |
|---|---|---|
| `ArtifactRetainedCommandJob::close_step`, `…/🧵️retained-command/🦀️.rs:594-615` | Drives `emit.close_child_one`, then `retire_one!(emit)` and `retire_one!(ephemeral)` call `drop(self.$field.take())`. Its rejection branch at `518-522` restores `emit.ok()` and ephemeral but immediately drops `rejected.fault`. | A duplicate/busy completion with an ordinary mutation, effect, event/task, nonempty ephemeral lane, or input `Err(Fault)` still has a wholesale post-child drop. The existing child fixture only proves child bytes disappear before that drop. |
| `TypedCommandFullOperationJob`, `…/🔌️plugin/🦀️.rs:16873-16882`, `16911-16948` | Same restore-then-child-only behavior; it directly drops `rejected.fault`, then `drop(self.emit.take())` / `drop(self.ephemeral.take())`. | This is the framework-owned app-factory job, so it is a primary generic path, not a test-only shortcut. |
| `RetainedPuzzleCommandJob`, `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs:588-590`, `661-675` | It now retains the rejection, but closes only `rejected.emit` children and assigns `pending_completion_rejection = None`. | The rejection's mutation lanes, ephemeral owner, and fault are still destroyed at `674`. |
| Draw and Writer command jobs | Both retain the rejection (`🖍️draw/…/✏️editor/🦀️.rs:620-622`, `✒️writer/…/✏️editor/🦀️.rs:569-571`) but each close path only calls `close_child_one` and then clears the whole field (`Draw:644-658`; `Writer:616-630`). | Their new tests prove child-first order only. They do not make a non-child typed value safe to drop. |
| `FlowHostEffectJob`, `🌊️flow/…/✏️editor/🦀️.rs:1336-1350` | `completion.complete(...).is_err()` consumes and drops the entire new rejection. | It is still an immediate full loss of `Emit`, ephemeral owner, and rejection fault. |
| `ArtifactToolCompletion::complete_download`, `…/🔌️plugin/🦀️.rs:13368-13374` | On busy/duplicate, returns bare `Fault`; the passed `Result<ArtifactDownloadOutput, Fault>` and `EphemeralEmit` are dropped. | The API is not symmetric with `complete`, so no caller can retain/close metadata, a unique segmented output, an input fault, or ephemeral lanes. |
| Layout download caller, `📏️layout/…/📤️export/🦀️.rs:2436-2441` | Explicitly ignores that bare error. `inner.output_chunks` is a clone and its normal close path drains the shared queue first (`2460-2505`), so the sole current chunk queue is not shown lost. | Filename/MIME/encoding and any future nonempty ephemeral input are still unowned on rejection; generic correctness cannot depend on Layout retaining another clone. |
| `take_emit`, `…/🔌️plugin/🦀️.rs:13381-13386` | Takes the cell, and when it contains `Download`, creates an error while destructuring `_`; download chunks/metadata and ephemeral are dropped. | Clipboard and media import callers (`22302-22307`, `23307-23313`) cannot put a wrong-kind completion into a bounded rejection closer. Moreover their subsequent `?` checks can whole-drop a successfully extracted emit/ephemeral on validation or store-apply failure. |
| Mounted typed-operation retirement, `…/🔌️plugin/🦀️.rs:16507-16572` | It deeply drains `ChildEmit` and download chunks/metadata, but then pops typed mutation/effect/event/task/presence/transient values and ultimately clears the publication. | Cancellation/rejection of a mounted operation can still whole-drop app-defined typed lanes. Fixed `ArtifactBoundedToolFault` itself is safe, but it does not make the other branches safe. |

`ArtifactToolCompletion::complete_download` is the only production call found
in this audit. `take_emit` has exactly two production consumers: framework
clipboard and `import-media`; no other caller may be silently exempted.

## What is actually improved

Puzzle5d now preserves the exact rejection for Copy/Cut/Paste/Import and has a
kind-specific close owner at
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:3282-3327`.
It delegates child bytes, exact known mutations/effects, presence backing, and
fault strings/cause strings to bounded routines (`3126-3263`). Each of its
four jobs drives that owner before ordinary job state (`2257-2271`,
`2337-2351`, `2617-2629`, `3874-3886`). The reported neutral oracle is not a
native acceptance claim here.

That code shows the essential rule: the closer must know the concrete
mutation/effect/presence shapes. It is not reusable by turning those fields
into `Any`, by rebuilding a mutation from source, or by an unbounded queue.

## Smallest correct framework packet

1. Replace the field-public `ArtifactToolCompletionRejection<A>` with a
   non-clone, non-droppable-before-terminal owner covering both variants:

   ```text
   ArtifactToolCompletionRejectedOutput<A>
     = Emit { emit: Result<Emit<…>, Fault>, ephemeral: EphemeralEmit<A> }
     | Download { download: Result<ArtifactDownloadOutput, Fault>, ephemeral: EphemeralEmit<A> }
   ArtifactToolCompletionRejection<A> { output, admission_fault }
   ```

   `complete` and `complete_download` must both return it. A busy/duplicate
   error retains the exact submitted branch; a stored `Err(Fault)` remains
   part of that branch until bounded retirement. Do not add a parallel
   download rejection type with a second lifecycle.

2. Add one typed, closed contract to `ArtifactApp`, not an erased global
   disposer:

   ```text
   type CompletionRejectionRetirement:
       ArtifactCompletionRejectionRetirement<Self>;
   fn begin_completion_rejection_retirement(
       rejection: ArtifactToolCompletionRejection<Self>
   ) -> Self::CompletionRejectionRetirement;
   ```

   The retirement type owns the complete rejection and exposes only
   `begin_close`, bounded `close_step(maximum_items, maximum_bytes)`, and
   `terminal_is_empty`. `Drop` must reject a nonterminal owner, matching the
   repository's strict retained-owner convention. It must retire, in order:
   all child emits; every concrete typed output/ephemeral/fault field; then
   vector/string backing; finally the enclosing owner. The default must be
   unavailable: a type with unproven mutation/effect/task semantics cannot
   honestly inherit a whole-value implementation.

3. Keep the framework-only pieces inside the shared closer rather than
   duplicating them in every plugin: download chunks are closed one chunk at a
   time, then filename/MIME/encoding scalar/backing; bounded stored fault
   bytes are fixed; child uses `close_child_one`. The app's associated closer
   must receive the live typed `Emit`, `EphemeralEmit`, and incoming `Fault`
   branch without re-encoding or rebuilding them.

4. Make `take_emit` a typed ownership transfer, not a lossy conversion. It
   must return an enum (or accept a typed expected-kind selector that returns
   the unaccepted output owner). A clipboard/import wrong-kind result must be
   installed in the same closer before returning the `unexpected-download`
   fault. The reserved commit routines must retain their extracted output in
   that closer across every later `ensure_*`, presence, transient, or dispatch
   error.

5. Migrate all rows above to store one
   `Option<A::CompletionRejectionRetirement>` before emitting a terminal
   fault; their close order is:

   ```text
   pending rejection closer → input/raw/checkpoint → ordinary job owners → completion handle
   ```

   A rejected completion is final. Busy/duplicate must never replay an app
   reducer, re-run a session, or synthesize a replacement emit. Puzzle5d can
   adapt its current exact closer to the associated contract; Draw, Writer,
   generic Puzzle, generic retained command, full operation, Flow, Layout,
   and mounted runtime need real implementations/adapters, not a child-only
   fallback.

6. In the mounted runtime, use that closer for an unconsumed publication on
   cancellation, pre-ACK fault, undeclared lane, outbox saturation, and final
   retirement. The existing raw `pop()` chain at `16518-16528` cannot remain
   the fallback after this packet.

## Required non-vacuous proof matrix

| Law | Required evidence |
|---|---|
| Neutral schema/oracle | One language-neutral `artifact-tool-completion-retirement-v2` corpus with Emit/Download × accepted/duplicate/busy × `Ok`/`Err`, explicit child/mutation/effect/event/task/presence/transient/fault/download-field counts, fixed grant traces, and exact terminal state. Independent JS/AJV model must prove no retry and no kind conversion. |
| Generic source/native | Instantiate a strict fixture app whose document/config/draft/presence/transient mutations, effect/event/task, and `Fault` each panic if whole-dropped. Exercise duplicate and held-lock busy; prove every path reaches terminal under grants `0`, `1`, and a bounded byte grant, preserving the already-filled cell. Existing `35141-35238` must be expanded beyond its child-only assertion. |
| Download native | Seal multiple chunks; force duplicate and busy `complete_download`; verify exact same chunk/metadata/ephemeral owner returns, only one closer drains each chunk, and wrong-kind `take_emit` retains rather than destructs it. Include `Download(Err(Fault))`. |
| Caller family lifecycle | Actual handler → rejected completion → cancellation/close for generic full operation, generic retained command, generic Puzzle, Draw, Writer, Flow host-effect, Layout export, and Puzzle5d Copy/Cut/Paste/Import. Each law must prove handler executed once, normal prefilled completion remains readable, no publication/dispatch occurs, every close step respects its grant, and terminal is empty. |
| Mounted runtime | Drive a real typed operation through accepted publication, cancellation before ACK, undeclared lane, saturated effect/event receiver, and delayed ACK. Assert typed lanes are retired by the associated closer rather than a `Vec::pop`/whole drop. |
| Reserved routes | Clipboard/import wrong-kind, validation failure, presence failure, transient failure, and dispatch failure retain then close the extracted owner before returning. No raw `?` may bypass that closer. |

## Scope and nonclaims

- Puzzle5d's source-level closer is the only current deep type-specific
  pattern observed; its reported neutral source oracle and staged native laws
  were not executed by this audit lane.
- This report does not claim a chunk leak in the current Layout path because
  its inner job retains a shared output-chunk clone. It identifies the generic
  API/lifecycle hole that makes the path unprovable and unsafe for another
  caller.
- No generic framework closer can safely retire arbitrary plugin types without
  the closed `ArtifactApp`-owned associated retirement contract above.
