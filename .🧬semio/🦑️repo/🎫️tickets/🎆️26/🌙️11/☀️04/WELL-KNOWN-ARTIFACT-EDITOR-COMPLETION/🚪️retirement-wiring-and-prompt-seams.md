# Retirement Wiring And Test Seams

Read-only proposal. No product source was changed and no heavy test ran.

## Early Wiring

identity-document-ports/identity-and-directory-state creates openDocumentRef at237 and closeDocumentRef at238. core hook order is identity ports, backbone, directory, plugin lifecycle, focus/contributions, spawned session, host effects, shell URI/close guard, remaining shell phases, document transfer/persistence, canvas. Plugin reload and switch hooks therefore exist before both close prompts and first-save persistence. Declare captured-owner retirement/persist function refs beside the existing early document refs, assign later concrete implementations, and make the initial admission fail closed. Call ref.current at invocation time; do not capture the initial function while rendering earlier hooks.

Plugin reload receives core typed ShellCoreAfterDirectoryAndRoute; switch receives document-services ReturnType. Propagate only the early ref contracts through these types, rather than pretending late scope functions exist early. Canvas can consume the same captured-owner operation after normal late core assembly. Internal unconditional cleanup may keep closeDocumentRef; user-facing retirement must pass admission first.

## Existing Prompt Contract

ClosePromptV1 is runtimeKey,name,phase saving|confirm,failure code|null,onKeep,onDiscard. DocumentCloseDialog focuses Keep, Escape calls Keep, and saving shows Cancel only. Auto-flush then explicit discard already fits unbound persistence. A program-owner prompt token is UI identity only; never invent a persisted document ID or look it up as if it were an attached runtime key.

The shared guard immediately accepts any initial state whose kind is not unsaved. In particular unknown is currently treated as safe for the attached no-admission interpretation. An unbound captured document lacking storage proof must enter the guard as explicit unsaved (pending may be0), or the save step will be skipped.

## Document And Dirty Classification

The runtime AppDocumentIdentity parent_document_id null is the authoritative no-document result. A rejected identity read is failure/unknown, not no-document. Missing attachment is not proof of cleanliness. App role, natural codec declaration and nonempty artifact schema identify capabilities, not saved content. Viewer role alone is not no-document proof. HistoryPatch exposes optional editCount and canUndo; positive edits are evidence of changes, but zero edits may be meaningful initial genesis or an undo result. Do not treat zero as a universal safe-to-destroy rule without an explicit authored empty-genesis contract.

For a documented unbound owner, persist its full current archive through the shared captured-owner first-Save coordinator, with exact identity checks, cancellation, verified write/attach/catalog readback and no focus lookup/download. Storage failure goes to the existing confirm phase; Keep retains the original instance.

## Actual Test Seams

- Extend actual useCanvas fixture with zero open sessions, exact captured plugin/app/instance and deferred retirement promise. Keep/refusal/rejection must retain panel, instance, layout and shell-command log. Test focus movement, sibling close and generation replacement.
- Exercise actual close-state hook prompt through renderHook: unbound owner starts explicit unsaved; pending save shows Cancel, aborted save keeps without re-prompt, failure shows confirm, discard targets captured owner only.
- Exercise actual usePluginReloadCallbacks using mocked module acquisition/disposal and two owners. Collect and admit both before first destroy; second Keep must leave both original instances/attachments/module intact and release candidate.
- Exercise switch gate with ordinary discard versus documentSource archive handoff. Ordinary Keep retains predecessor; handoff preserves archive identity and member closure and only then retires predecessor.

The neutral proposal now has11cases in 🚪️unbound-retirement-scenarios.json, including explicit null identity, identity failure, meaningful zero-history genesis and focus-independent persistence. They are source-predicted regressions, not executed test evidence.
