# PDF pixel oracle portability

The PDF writer's exact pixel laws currently run Poppler through their test adapters. The available macOS executable came from the local Codex runtime. The repository tool declarations and devcontainer search found no Poppler provisioner.

I evaluated the existing PDF.js and native canvas dependencies at 512 by 256, sampling the exact centers from the Poppler contract. On the gradient alpha case, PDF.js returned opaque interpolation colors `191,0,64,255` and `64,0,191,255`. Poppler rendered the intended alpha compositing on white as `239,191,207,255` and `112,64,207,255`. The first channel differed by 48 against the fixture tolerance of 3.

The PDF.js comparison experiment was removed and the authored neutral PDF writer test passed again with Poppler. The PDF.js browser proof still covers visible output and changed painting after the rectangle gesture. Portable provisioning for the separate gradient-alpha pixel law remains open.

## PDFium replacement

The ticket captured 25 PDFs from the unchanged production writer, including the eight neutral rows at three work grants and the shared raster image. The cached `pypdfium2==5.15.2` wheel is the same pin already used by the repository's teaching document oracles. The direct library probe rendered all 25 PDFs and met every fixture tolerance. Gradient alpha was within one channel value, and the shared raster alpha was exact.

The Rust and TypeScript PDF-writer laws now use one Python PDFium oracle through the repo-managed `uv` tool. The oracle accepts length-framed PDF bytes and returns two RGBA samples per document. The neutral cases batch across their three work grants. The existing PDF.js and SVG comparisons remain in the TypeScript test. This is test-only. Rust syntax and the two unaffected PDF cancellation tests passed after the change. The managed interpreter route and native test execution still need current results.

The committed Python framing code passed all 25 requests through the repo-managed Python interpreter using the installed pinned wheel. Every reply matched the direct PDFium C API. The existing Node PDF.js comparison also passed all 21 comparable documents against PDFium, and strict TypeScript compilation passed. The repository wheel inventory includes macOS arm64 and x64, manylinux arm64 and x64, and Windows arm64 and x64. Execution evidence is from this macOS host. The isolated `uv run` launcher is still queued under the shared capacity governor.

The refreshed TypeScript PDF writer suite passed all 10 tests, including both pixel laws and four fixture ownership checks, with the pinned wheel already in its cache. The ticket preload changes only the provisioner for this run. It executes the same Python oracle source through the repo-managed interpreter and the same PDFium wheel.

The complete updated TypeScript PDF suite then passed all 10 tests through the actual isolated `uv` launcher with no preload. The short local run used the documented `SEMIO_HOST_CAPACITY=off` option for the cached interpreter proof because a larger pending Cargo request held up the tool query. The native builds and the dev-host activation kept their existing capacity admission.
