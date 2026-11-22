# Cargo First Error Retention

The native WGPU build failed twice while compiling shell actions. Its bounded task log retained the E0425 summary and the final 200 lines of warnings. The cache owner later reclaimed the lane that contained the complete Cargo diagnostic file. A focused workspace Cargo check now captures structured compiler diagnostics in this ticket.

The staged diagnostic change retains the first suppressed compiler error block within the existing tail-sized budget. It replays that block only when the command fails and skips lines already present in the newest tail. The live line allowance and successful output remain unchanged. Ajv checked the neutral fixture, and SQLite independently selected the expected line indexes for the first blocked error, an error already in the newest tail, and an error already forwarded live. The first law failed on the original code. All three passed against the staged production class.

- [Neutral fixture](🔣️cargo-first-error.json)
- [Fixture schema](🔣️cargo-first-error.schema.json)
- [Staging and regression test](📜️cargo-first-error-law.ts)
- [Native build output](🗑️generated/native-wgpu-save/wgpu-host-current-document-picker.log)
- [Focused shell action compiler output](🗑️generated/native-wgpu-save/wgpu-shell-actions-diagnostics.log)

The change is now installed in the repository diagnostic owner. The focused Cargo check captured an independent E0460 dependency metadata failure before reaching shell actions. A new WGPU artifact run uses the corrected diagnostic owner and retains its own output in the ticket.

- [Subsequent native WGPU artifact run](🗑️generated/native-wgpu-save/wgpu-host-first-error.log)

The subsequent genuine build retained `E0425` at `shell-actions/backbone-and-check-in/🦀️.rs:247` after suppressing more than 113,000 lines. Shared source has since replaced the removed `S_SPACE_INDEX_DOCUMENT_SCHEMA` constant at that call site. It now resolves the mounted program's registered space index, validates its socket schema and uses its document schema for check-in. A fresh governed host build still needs to establish WGPU compilation and desktop startup. The earlier focused Cargo check stopped at E0460 before reaching that source.
