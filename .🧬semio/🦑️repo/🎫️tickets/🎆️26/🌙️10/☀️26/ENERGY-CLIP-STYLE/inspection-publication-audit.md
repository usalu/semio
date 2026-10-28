# Inspection Publication Concurrency Audit

Status: read-only source audit supports bounded same-process publication of distinct lesson IDs after canonical source and chapter assembly, subject to the ownership conditions below. No production code was changed for this audit.

## Ownership And Isolation

- `compose_videos` retains settings, plans, chapter groups, excerpt lookup, logical directories and destination paths in call-local variables. Stable full-catalog ordinals keep each lesson's logical directory distinct even when one lesson is selected. Its global rendering plan deduplicates canonical source identities, so concurrent source rendering is unnecessary once assembly is complete.
- `publish_presentation` creates a call-local `ToolSession` and candidate output. The compositor creates a UUID-named `p-*` work directory, and `_clean_attempt` checks the exact owned parent before deleting only that attempt. Different final lesson paths and their render sidecars are disjoint.
- `ToolSession` owns each task's process set behind its own lock. Cancellation stops its owned children. Concurrent calls must retain individual sessions rather than share cancellation state across unrelated publications.
- `hierarchy.capture_catalog` uses a `ContextVar`, deep copies the model and resets its token in `finally`. ThreadPool workers do not inherit caller context automatically. Use a **fresh** `contextvars.copy_context()` for each submitted job and run the job under that context. Reusing one Context concurrently raises a runtime error. Capture the catalog once in the orchestrating scope so all workers render the same finite snapshot.
- `_settings` and hierarchy resolution read the captured catalog and produce local dictionaries. No per-worker mutation of `os.environ`, output roots, language or global Manim configuration is required or safe. Set those once before creating worker threads.

## Browser And Shared State

- Browser `_call` creates a UUID-named `b-*` attempt under the caller's unique work directory. Every subprocess receives a deep-copied packet and writes its own request, response and log. Rendered frame paths remain under that owned work directory.
- The JavaScript browser adapter builds bundles in memory. Its only adapter module output is `browser-tools-${process.pid}.cjs` under its unique owned directory. Each invocation launches its own browser/page with its own request routing, paused clock, image map, pointer state, DOM, output list and metadata. There is no process-global browser shared by Python threads.
- Browser dependency metadata `_SOURCES`, `_IDENTITY`, `_SCAN_SOURCES` and `_MEMBERSHIP` are guarded by `_LOCK` (an RLock). Initial discovery can serialize workers; warm `source_files()` once before publication. Known-file hashes and candidate membership are rechecked on each call, preserving invalidation rather than freezing a stale dependency closure.
- Common-root raster `_HANDOFF_ASSETS` is bounded to 16 entries and protected by `_HANDOFF_LOCK`. Its key includes captured catalog, explicit locale, viewport, dependency identity, hierarchy and cursor state. Publishing distinct lessons in one Python process therefore shares the same exact actual-UI root raster safely. Cache replacement and eviction occur inside the lock.
- Browser provisioning is the remaining external shared effect: separate workers can all try to provision Chromium if it is absent. Provision the existing repository-managed browser once before worker publication, or let a completed first canonical capture establish availability. Existing renderer setup already uses the provisioned browser interface; do not replace it with an independent browser installation.

## Required Scheduling Conditions

1. Finish all 117 canonical scenes and all 38 logical chapters first. Confirm matching source/chapter certificates and required cursor/trace sidecars. Use existing `concat_only=True` with no force flag; stale/missing inputs must fail rather than spawn duplicate rendering work.
2. Ensure only one active publisher owns any given final lesson ID. Do not run a helper for the same unfinished IDs while the original serial `compose_videos` publisher remains active. The final sidecar/candidate replacement is atomic publication, not a cross-publisher mutex.
3. Shared excerpts must already be valid or absent requests must fail in concat-only mode. `excerpt_segment` uses a shared deterministic excerpt filename; concurrent fresh excerpt production would share its `.partial.mp4` and sidecar temporary path. Complete chapter assembly guarantees the required excerpts already exist.
4. Limit browser/FFmpeg concurrency under one existing machine capacity grant. Four publication workers can create four independent browsers, FFmpeg processes and transient bundles. Python thread count alone does not reserve memory or CPU, and nested lightweight repo calls must inherit the grant and agent session.
5. Keep production final validation intact: `publish_presentation` compares current content/cursor identities, the captured presentation state and recursively rehashed source inputs before publishing. Do not bypass it by calling the compositor directly or fabricating certificates.
6. A top-level cancellation handler must cancel active individual sessions or use the existing `execute_jobs` session ownership protocol. Cancelling queued Future objects alone does not terminate active Node/browser/FFmpeg children.

## Assessment

The existing browser capture and catalog APIs appear safe for four threads publishing different lesson IDs under these conditions. Parallelism preserves the same source and presentation certificates and can reuse the shared root raster. Retaining the canonical serial path is also correct. This is a source-level assessment; the independent full-frame and endpoint verifier remains the acceptance check for the actual outputs.

Reviewed sources: runtime/🐍️.py; presentation/🐍️.py; presentation/🌐️browser/🐍️.py; presentation/🌐️browser/🟨️render.mjs; presentation/🥞️hierarchy/🐍️.py; presentation/🥞️hierarchy/🖌️render/🐍️.py.

## Source-Only Pre-Gate Revision

The proposed task-local stage can safely assemble chapters in parallel through the existing `compose_videos` API after checking the 111 non-optimization canonical sources and preparing all eight semantic excerpts serially. Full-catalog ordinals preserve distinct per-video chapter, content, settings and final paths; the production composer rechecks source and authored state before and after each chapter/body assembly. Optimization lessons may use normal rendering with one source worker per video, provided their six canonical source directories are disjoint and only those selected IDs permit rendering. Other lessons remain concat-only. A final production guard must recheck all 117 sources, 38 chapters and 17 deliveries against current inputs; no earlier pre-gate substitutes for that check. The original publisher must be stopped before the staged process begins. This paragraph reviews the production API and proposed schedule; the final task-local helper revision still requires inspection after it is written.

The independent verifier now validates its metadata cache against media size and modification time, rejects a media replacement during full decoding, and rescans every final output identity before accepting the cross-catalog root equality. The same-path replacement regression passes without manual cache eviction.

## Actual Staged Helper Review

Reviewed the now-written 12,506-byte `inspection-publish.py` (SHA-256 d07f378f5bc2ca5c0e1046feb25a69eead78b2b4df2d0b79023ec425be1f448e). It passes the proposed schedule review: source-only 111-source pre-gate; all eight clause excerpts and cursors prepared serially; a normal physical composer seed followed by forced publication only when that seed had already been current; four distinct-video workers with fresh copied contexts; full rendering permitted only for the six disjoint optimization source directories 120–125 with one nested source worker; concat-only handling for the other lessons. The final loop checks all 117 canonical sources, 38 chapters and 17 current output certificates, content/cursor identities and captured presentation states through the existing production interfaces. Input-change retries preserve the process root-raster cache and never rewrite certificates manually. Source changes during a build remain failures/retries under production checks. No production changes were made for staging. The supervising repo process must retain whole-owned-tree cancellation, including workers waiting for an explicit retry or source change.

## Resumed Ordinary Composer Scope

After the tool runtime reset, doctor inventory confirmed no surviving publisher or verifier. Root explicitly authorized restarting the same process/cache design with ordinary no-force `compose_videos` for all seventeen IDs. The revised helper SHA-256 is 73b8042d3922a26ae8f36014b4c382cf48384766f19643c152cdd2fd8a820d37. Current inventory reports twelve stale canonical sources: the six unique BIM scenes and six unique optimization scenes. These have disjoint source directories. Shared clause selections are still prepared and guarded serially before the worker pool. Production guards and the final 117-source/38-chapter/17-delivery gate remain active. The executor confirmed that any actual concurrently invalidated shared canonical source will be coordinated through serial recovery rather than duplicate writers. First diagnostic stacks show normal authored dependency AST parsing during physical assembly, with no observed deadlock. Earlier helper hash/scope above remains historical evidence. Fresh BIM and optimization imagery must supersede their earlier baseline samples.

Final retention must also wait for the staged `publish-complete.json` post-gate marker and match every final file SHA to the independent measurements. The watcher rescans all media identities before its cross-catalog root assertion, but a later post-gate retry can replace an output after that first complete scan. Any such SHA mismatch requires re-verification before retention.

## Chapter Export Gate Review

The revised task-only publisher validates all38 chapter exports against their actual current raw logical state, parent presentation state, cursor activity, authored ownership/order and independent finite chapter clock structure. Final native decode is deliberately delegated to the independent verifier. A separate-process physical smoke can be reused only subject to actual decoded LCA equality; no manual raster seed or certificate rewrite is authorized. The final marker requires chapter_exports38 as well as117 source/38 logical chapter coverage.
