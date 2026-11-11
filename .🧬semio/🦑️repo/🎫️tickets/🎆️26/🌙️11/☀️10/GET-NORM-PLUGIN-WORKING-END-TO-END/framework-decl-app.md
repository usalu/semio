# decl-app migration (semio-plugin-decl-app)

Result: `check -p semio-plugin-decl-app --lib` is clean (99 -> 0 errors, log `🗑️generated/decl-app-5.log`).

## Migrated (all under 📜️declaration)
- `🧩️app/🛬️lifted`: `♻️retirement` (+`🌱️birth`: `NativeAllocationIdentity` decoder scope, `with_native_receiving`, settled construction, effect close), `🚫️refusal`, `🧬️context`, `📸️fields`, `🎬️media-export` to `RetireOwned` seed/`RetirementCursor` with inherent `close_step(body,effect)` + `native_retirement_reply!`.
- `👷️job/🎟️request/{♻️retirement,♻️reserved,🏭️reserved}`: requests retire as nested tuples (19 and 13 fields), `OriginalRequestLease` for Arc fields; reserved job close via `close_original_effect_step`.
- `👷️job/🧾️bounded-proof-and-factory-registry`, `🔎️reserved-outcome-witnesses`, `🎮️command/🧬️op-binary`, `🎯️completion-and-owned-context/📥️provisional` (handle is its own `OriginalSourceProjection`).
- `🎛️artifact-app-trait`: `DraftMutation` now also requires `replication::OpBinaryReceiving`.

## API changes callers must follow (other crates, not edited)
- `ArtifactOwnedContextHandle::admit(original,grant,cx)`; `try_duplicate(effect)`; `duplicate_original(effect)`. Callers: 🧵️instance typed-command `⌨️typed-command` and `🚦️actions` (`admit` without cx).
- `ReservedJobOutcomes::close_step(effect)` -> `Result<RetainedCloneStep,ValueError>` (use `close_original_retained_effect_step`). Callers: 👷️job `🏛️reserved`, `📤️outcomes`, `🛬️lifted/🔄️full-operation`, `📜️declaration/📥️natural-file`, tests `agent-lane-preview`, `clipboard-and-retained-command-fixtures`.
- `ArtifactMediaExportRequestRetirement::close_step(effect)` -> `OriginalRetirementPoll`. Caller: 🧵️instance `📄️docs/📥️staging/🧵️instance-root`.
- `🌱️capture/🦀️.rs:28` (`advance_original_index(grant)`) not fixed: it runs inside `birth.built(cx,..)` closure that already borrows `cx`; needs a restructure in the instance crate.

## Not done
- Tests in `📸️fields` (`drain` uses old `step(grant)`) not migrated; `--lib` only.
