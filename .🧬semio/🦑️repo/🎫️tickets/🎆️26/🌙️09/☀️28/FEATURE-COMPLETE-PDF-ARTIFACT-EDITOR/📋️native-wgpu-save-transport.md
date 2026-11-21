# Native WGPU Save Transport

The current WGPU native and web shell handle `Effect::DownloadMediaExport` in the Window Actions effect funnel. The funnel passes its data directly through the kernel's ordinary media encoding decoder and then calls the file or browser presentation door. The native door selects a destination and writes the whole supplied Vec. WGPU has neither a shell path for the resumable media-export commands nor the React Save selection and attached-revision checks.

The native ProgramBridge already owns the needed `KernelClient` and its bounded `exchange_commands` call. It reads the actual `AppDocumentIdentity` using that path and validates the issued instance. The SPR channel already has `SubmitMediaExport`, `PollMediaExport`, `TakeMediaExportChunk` and `CancelMediaExport` with exact u64 operation, generation and revision fields. The replies include the same handle, total, descriptor, MIME, status, optional page and separate empty terminal. These are the same guest operations the React host uses. No actor or WIT addition is needed for the checked native Save driver.

Stage the driver below the existing native ProgramBridge exchange. The outer shell owns prompt, progress, cancellation, rechecking the exact document identity and final delivery. The receiver grants one page at a time and confirms reply address, mime, descriptor, cumulative admitted bytes and terminal frontier. The native sink should open an exclusive temporary file next to the user-selected destination, write each page, sync, recheck the identity and only then rename. Cancellation and refusal must settle the guest and remove the temporary file. The immutable browser Blob delivery is already implemented on the React side.

The native WGPU requested app still needs its ordinary build and mounted test. The native PDF acceptance law currently exercises its actual actor-backed Open/Edit/Save/reopen through the product SDK. Driver work and mounted WGPU confirmation remain separate. The existing headless WGPU smoke verifies shell boot and window documents.

- `🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🧳️wasm-program-exchange/🦀️.rs`
- `🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🌉️program-bridge-entry/🦀️.rs`
- `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions/⚙️settings-and-dock-persistence/🦀️.rs`
- `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📂️doc-open/🎨️theme-file-import-and-accessibility/🦀️.rs`
