# Exact Preview Link Digest

Root owns the shared procedural streaming preview-link digest, keeping the actual committed Flow tree, committed generation-state tree and raw selected-generation text. Each part ends in byte 0x1f under the original FNV-1a64 definition; the final digest XORs the wrapping provisional-generation product. Work revision is not this digest.

Eight language-neutral cases were authored before implementation, including Unicode, raw unquoted selection bytes, embedded separators, signed zero and maximum epoch. Native tests must compare both original preview_eval_digest and the existing third-party fnv::FnvHasher; authored fixtures are not an executed proof. Original Pack tree byte parity also remains a required native prerequisite.

## Mounted Native Contract

The producer is now mounted only in the P2 editor as retained_preview_digest. Construction is allocation-free and retains no copied scene. The actual source prepares its original Store read, then the cursor projects and writes the genuine committed fixture and optional generation state through ArtifactCanonicalJsonTreeCursor. A fixed 64-byte scratch buffer feeds one funded FNV byte event at a time. The selected id remains raw UTF-8, is bound to its original pointer and length, and receives its own separator. Final output includes the original wrapping epoch XOR. Cancellation preserves and explicitly closes the original aliases/writer/output; Drop requires terminal emptiness.

The native tests are AUTHORED, NOT RUN. They compare the eight cases with the original preview_eval_digest and existing third-party fnv::FnvHasher. The integrated law prebirths a genuine Generation2dSnapshot registry issuer, uses the real op97146/gen17/32768-fuel caller policy, checks original source identity, limits all execution plus cursor/source/returned-root/registry/decoder closure to 200 caller opportunities, observes exact physical heap deltas, tests cancellation prefixes and denied close currencies, and never replenishes stock/fuel.

Registered Bun actually passed all eight schema/Ajv and independent SQLite FNV outputs. SQLite computes the hash via independent 32-bit word arithmetic; the outer epoch multiplication/XOR is checked with JavaScript BigInt here and with native wrapping arithmetic in the unexecuted Rust/FNV law. This is fixture evidence only, not a native or end-to-end verdict. Runtime owns the native queue.
