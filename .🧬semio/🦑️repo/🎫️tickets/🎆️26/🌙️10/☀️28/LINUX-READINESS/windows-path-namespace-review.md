# Windows Observer Path Namespace Review

Read-only existing Node win32 path operations executed through repo; no compiler, graph, native watcher or Source edits.

Actual win32.resolve/relative vectors: ordinary C:\\repo to C:\\repo\\out yields out; case-variant c:\\REPO\\out also yields out; ordinary UNC \\\\server\\share\\repo to its child yields out. When only target is verbatim \\\\?\\C:\\repo\\out or \\\\?\\UNC\\server\\share\\repo\\out, win32.relative returns an absolute namespaced target and isAbsolute is true. Thus direct relative-containment checks reject these equivalent namespace spellings unless normalized together first. These are lexical Node vectors on Linux, not claims of native Windows junction execution.

Existing owned semantics: workspace Go core normalizeRootPath (:84), launcher Go FinalPathSpelling (:94), and resident watch Rust strip_verbatim (:210) convert verbatim drive to plain drive and verbatim UNC to ordinary UNC. Important: simply deleting four prefix characters from verbatim UNC produces invalid UNC\\server..., so UNC needs its distinct conversion. Do not strip arbitrary device/Volume namespace prefixes into filesystem paths.

Existing TypeScript helpers: process native storage directory Windows host nativeWindowsExtendedPathV1 validates canonical DOS/UNC and adds/accepts extended namespace, but does not reverse it. Filesystem descriptor artifactWindowsRelativePublicTargetV1 (:100) is private and decodes NT substitute targets (\\??\\ forms), not a reusable public generic path normalizer. Repository taxonomy normalization repositoryLocalSymlinkTargetPath handles ordinary drive/UNC case comparisons but lacks verbatim projection and pulls domain-specific taxonomy contracts. No exported generic TypeScript reverse normalizer suitable for direct observer reuse was found.

Recommendation: place the closed plain-Windows-path projection at the shared filesystem/path boundary, using existing Go/Rust semantic corpus as cross-implementation reference. Normalize both workspace and readlink-resolved target before containment; retain path casing for actual I/O and use platform relative semantics for case-aware namespace containment. Include drive/UNC verbatim equivalents, different share/drive, prefix near-miss, malformed device/volume forms and parent traversal refusal. Keep the strict full256 UTF16 path law after projection. Existing post-readlink cancellation check is fixed according to Root/current Source checkpoint.


## Shared TypeScript Projection Review

The new filesystem/📂️paths/🪟️windows helper now supplies the previously absent shared TypeScript reverse projection and is used by the System observer before Windows relative containment. Read current Source: bounded plain DOS/UNC projection preserves casing, handles verbatim/NT UNC separately, refuses device namespace and alternate-stream colon forms, and bounds the postprojection path. No duplicate taxonomy dependency introduced. One small syntax omission sent to Root: double quote is absent from its invalid-character regex. Actual native Windows execution remains a separate required gate; lexical helper tests on Linux do not establish Windows watcher readiness.
