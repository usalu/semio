# Drawing Export Frame and Transport Funding

The Draw editor now quotes the registered native serializer constructor before admission and uses the initiating retained `StepContext` for construction, terminal output or fault handoff, and transport. The factory allocates `size_of::<DrawingOwnedSerializerJob>()` once and quotes the same size when the fully retired frame can be released.

## Measured frame sizes

Two native prototype test lanes emitted ARM64 Mach-O objects for the Draw serializer. Their `terminal_frame_release_bytes` methods both load 864 with `movz w8, #864`, apply `movk w8, #32, lsl #16`, and pass the resulting **2,098,016 bytes** as the value of `then_some`. The command-local Cargo store has since reclaimed both lanes.

| Native lane | Draw writer object |
| --- | --- |
| `p988182` | `semio_s_artifact_draw_drawing_write-25dac6a849ca1199.0lf6cc620tjlt0653iql22sza.1phic9b.rcgu.o` |
| `p682866` | `semio_s_artifact_draw_drawing_write-a441552199d00fbb.a4ghsw1qfvckudd1lw0rvv866.1g2al9f.rcgu.o` |

The browser-published WASI component at `.cache/b/p/m/browser/prototype/components/285067b377def16ed6e8c388/📦️component.wasm` uses **1,270,080 bytes**. In the stripped core module, padded signed LEB references to the actual Drawing source-authority and allocation-failure strings identify the constructor functions. Function 289552 writes depth 1 and packs capacity 1,270,080 in the upper 32 bits of its success discriminant word. Function 289557 passes alignment 16 and that capacity to the actual allocator. Function 289577 writes the same extent into its terminal `Option<usize>`. The retained WASI export-contract component agrees. The independently linked 173 MiB component snapshot allocates 1,270,048 bytes.

Both measured targets exceed the previous 1,048,576-byte mounted close policy. Both fit the 16,777,216-byte Draw admission capacity. A Draw close policy using the same local size limit can release either frame while still being clamped by the originating caller. The current publication payload table alone has 256 ARM64 rows of 24 bytes and does not explain the complete size.

## Owner path

The registered serializer carries either the original output or the original diagnostic until the editor spends the reported one-item transfer. The editor debits the same caller before performing another turn. It then allocates the segmented queue through `original_birth_admit`, stages each Base64 page under its precise copy and capacity bounds, transfers the original vector through `push_original`, and seals using `seal_original`. A queue backing allocation can consume a separate turn while the encoded page remains in its existing allocation. A staged page is part of Download Work retirement.

The neutral transport fixture covers one full raw page and a partial tail in both paged and contiguous source representations. The Rust lifecycle checks denied grant behavior, heap receipts, original vector identity after queued transfer, and cancellation with a staged vector. Node's independent Buffer implementation agreed with the fixture for both page sizes and the combined output. The scoped Git whitespace check passed. The direct editor test package exceeded its existing precompiler source budget at 269,971 bytes of 262,144. The three Work laws now mount in the existing Draw editor second companion. The editor exposes their owner access only under its existing `testkit` feature. The native companion run uses the `export_work_tests` filter.

The formatter bodies are an independent retained allocation seam. The existing PDF and SVG writer calls can allocate formatter pages, and the legacy PNG encoder can allocate a whole contiguous output buffer, before either reports that capacity to the parent. The 4 KiB segmented transport now accounts for its own pages and metadata.

## Native companion build

The focused companion source admission passed the unmodified crate limits. Its first native compile reached the DWG geometry dependency before failing on three absent methods of `semio_framework_mesh_engine::MeshData`. The live source contains both `normalize_vertex_normal` and `accumulate_triangle_normal` on that exact type. A fresh registered Cargo build was started to capture the peer update. The prototype build uses `-p semio-s-artifact-draw-drawing-editor-tests-2 --lib` with test filter `export_work_tests`. Compiler outputs remain in the ticket’s generated folder.

## Header preparation

Work now quotes and posts the actual filename, MIME, route, options backing, and captured-request transfer. The filename helper writes the same sanitized ASCII stem into a 256-byte stack buffer. If a heap reservation refuses, the successful earlier reservations and their receipt remain on the original Work, and the refusal closes that header phase. The source lease remains typed until all metadata has been prepared under the caller grant. Cancellation can turn the completed original request back into the same typed read for retirement.

The companion source-read law checks the shared fixture labels and dialect, exact heap capacity and caller receipt, a denial on each applicable grant axis, and pointer identity after the move. The Node Buffer fixture check, Rust grammar check, and scoped Git whitespace check passed. The in-flight companion snapshot was captured before the header update.
