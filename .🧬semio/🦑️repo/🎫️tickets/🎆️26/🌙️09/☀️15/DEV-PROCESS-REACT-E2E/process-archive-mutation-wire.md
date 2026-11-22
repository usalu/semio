# Process3d archive mutation wire

The native conflict-history law reproduced the archive failure before any conflict edit. The `delete-machine/empties` fixture seeded `deleteMachine` for `saw`. The saved history reached `DecodeForward`, where hydration submitted the preserved binary payload to `OpBinaryReceiving`. The old producer emitted `02 08 03 73 61 77`, a six-byte compact record. The original receiver expected a complete Pack value document and rejected the header as truncated at offset 6. JSON envelope history settlement happens on a separate route.

Process3d now publishes one complete framework Pack value document for binary operations. The value contains the authored `mutation` discriminator and its typed fields. Binary decode, native command and archive receiving, and the hex-wrapped JSON envelope all consume that representation. The envelope retains its original hex backing until the typed cursor and its allocations have completed the granted close. Cancellation and refusal use that same cursor before releasing the bytes and any typed value. The checked-in protocol describes the Pack value wire, and the unused compact reader and its standalone subordinate crate were removed.

The production regression reuses the exact language-neutral `delete-machine/empties` input. It checks the decoded Pack fields against the third-party serde_json interpretation, rejects the former six-byte shape and a truncated canonical payload, and drives original native receiving for that operation and all 16 Process3d variants. Node independently read the fixture and reproduced the six-byte prior input. A scan for Pack and SPR magic found no checked-in binary goldens in the Process3d workspace. The existing examples declare operations as text.

Rustfmt parsed the edited Rust sources, and `git diff --check` passed for the changed Process3d files and compiler owner index. Native compilation and execution await the existing editor and compiler storage work.

```sh
repo cargo --store=shared --configuration=prototype --parallelism=1 -- test -p semio-s-artifact-process-process3d-editor --lib unit_tests::production_envelope::
repo cargo --store=shared --configuration=prototype --parallelism=1 -- test -p semio-s-artifact-process-process3d-editor --test process3d_bearbeiten_pane_smoke context::conflict_history_edits_end_to_end
```

## Original receiving work and refusal settlement

The original Job decoder measures the native work delta around every callback, including a refusal. `OriginalDecodeState::run_scoped_on_caller` posts that observed work to the original Source escrow. `OriginalReturnBudget::observe_scope` adds it to the original work counter and deducts its fresh portion from caller fuel. `StepContext::settle_receiving_scope` reconciles the separate physical receipt. A debit by Process3d would charge the same native work twice.

The envelope callback now qualifies the Pack child's reported work against the native scope's actual work delta on success and refusal. It places a child refusal in the scoped call's own result. Previously a separate diagnostic slot could return the child refusal before observing a simultaneous Source posting refusal. After physical settlement, the existing `OriginalNativeReceivingOutcome::into_result` projection gives the Source refusal priority, otherwise reports the native call outcome.

Rustfmt parsed the edited host source, and its scoped Git diff passed the whitespace check. Cargo was not started for this follow-up.
