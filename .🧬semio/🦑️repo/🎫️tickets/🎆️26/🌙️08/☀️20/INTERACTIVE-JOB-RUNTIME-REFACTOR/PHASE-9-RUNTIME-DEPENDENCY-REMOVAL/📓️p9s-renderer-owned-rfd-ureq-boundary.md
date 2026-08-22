# Renderer Owned Rfd and Ureq Boundary

## Scope and census

The WGPU target had two native direct third-party rows: `rfd = "0.15.4"` and `ureq = "2.12.1"`. Renderer source had five native dialog families in Shell: media export, studio save, folder selection, generic single/multi-file open, and media-frame video selection. Renderer source had zero `ureq::` calls: directory HTTP already flows through OS-kernel's owned `NativeDirectoryTransport`; the WGPU `ureq` row duplicated the transport feature's implementation edge.

## Implementation

- Added the owned `NativeFileDialogRequest::{Open,Save,Folder}` schema and `select_native_paths` platform boundary in `semio-framework-ui-host`. Extension filters normalize, sort, and deduplicate before entering the platform implementation.
- Converted all five native Shell dialog families to async platform requests. UI callbacks submit the futures to the renderer's existing `WorkerPool` `Lane::Io` seam through `submit_shell_io_future`; they only poll `PendingShellIo` and enqueue completed actions later.
- `PendingShellIo` retains the pool task and cancels it on disconnect or drop. The wasm download/open/media branches are unchanged.
- Removed both direct WGPU manifest rows. The renderer exports no `rfd` type and contains zero `rfd::`/`ureq::` runtime calls. `rfd` is confined to UI-host's platform implementation; directory HTTP remains confined to OS-kernel's transport feature, rather than a second renderer client.
- Removed the unused WGPU `wasm-bindgen-test` dev edge. No direct `naga` dev edge existed.

## Parity evidence

- `📝️p9s-ui-host-file-dialog-tests-3.txt`: 3/3 request-schema, save/folder, and `Send` future tests pass.
- `📝️p9t-ui-host-wasm-check.txt`: UI-host wasm32 check passes; native dialog implementation and dependency remain cfg-exclusive.
- `📝️p9t-dependency-ratchet.txt`: clean, 211 current third-party dependencies versus the 238 baseline; no new third-party dependency.
- Direct manifest/source census is zero for WGPU `pollster`, `rfd`, `ureq`, `wasm-bindgen-test`, and direct `naga` rows/calls.

## External compiler boundary

`📝️p9r-wgpu-native-check-2.txt` does not reach WGPU: it stops in 14 Flow diagnostics and 2,740 Puzzle async-migration diagnostics. No Flow/Puzzle repair is in this packet. Native WGPU debug/release/binary gates must be rerun from the warm isolated target once those owners clear their compiler walls.

