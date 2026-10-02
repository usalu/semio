# Landed Taxonomy Input Boundary Review

Read-only source audit; current owned replay remains Root-owned. No jobs or edits.

Input owner now defines UnsafeDirectoryAncestorError/noFollowDirectoryChain once; taxonomy directly imports physical owner, avoiding IO reverse dependency. Loader first rejects raw parent segments/opaque/local escape, then checks all absolute root directories, then captures current schema bytes, then consults private content facts. Cache clone excludes path/input/matcher and fresh session creates matcher. No stale cached physical admission is apparent in sequential paths inspected.

## Native path behavior

noFollowDirectoryChain uses native parse(root).root and sep/join. Resolved POSIX paths start at /; Windows drive and UNC paths start at their parsed root. Each authored native component gets lstat and rejects links/non-directories. macOS /var→/private/var or /tmp symlink ancestors are deliberately rejected under strict full-ancestry policy; fixture output currently ticket under /Users, so it does not silently canonicalize these away. Caller needing an ordinary physical root must supply its actual canonical path through explicit authority contract, never silently realpath a hostile input. Windows junctions generally report isSymbolicLink through Node lstat; actual platform oracle still needed before universal execution claims.

Raw .. is rejected before resolve for both root/path after backslash normalization. This closes missing/linked-prefix before parent by refusing authored parent traversal, not by proving every raw component. Single-dot/redundant separators still normalize; do not describe raw dot traversal frontier as solved. Standalone noFollowDirectoryChain requires an absolute resolved coordinate; loader satisfies that.

## Nine actual authored cases

Taxonomy-input test makes a separate ticket-owned root, performs a valid warm load, then each authored mutation, using an independent Node child lstat/digest oracle. ordinary-warm/new-root positive rows prove fresh input/schema/matcher identity with equal bytes and changed physical path. Leaf/root/inner/ancestor links and deletion are actual filesystem mutations; missing-prefix and linked-prefix probe raw component independently. Broad toThrow refusal does not pinpoint guard order; source reasoning establishes current raw check and root-chain placement. Debug accepted value is fixture expectation, not an independently logged provider return for refusals; asserted throw supplies the behavioral proof.

Node child classifier probes the linked component itself (parent/root/ancestor), correctly avoiding a leaf read that would follow the link. Physical file oracle reads ordinary bytes and hashes them; it is a witness to retained bytes, not a second no-follow descriptor implementation. New-root shares warm parse facts across rows/process module cache but still captures fresh bytes before hit. The fixture tests no default language/compiler dependence.

Windows leaf symlink may require privilege/developer mode, while directory links use junction; current law does not skip on failure. This is honest strict proof but zero-touch portability needs actual environment capability evidence, not claiming the fixture passed on Windows. Node executable presence is required and existing toolchain supplies it; no new runtime library.

## Remaining race frontier

Absolute ancestor chain returns witnesses, but taxonomy discards them. Snapshot rechecks repoRoot and descendants only. A component above repoRoot may change between chain observation and snapshot/read without final ancestor identity comparison. Sequential symlink tests prove stable refusal, not concurrent root-ancestor replacement closure. Minimal additional contract: retain root-chain dev/ino witnesses and revalidate after actual snapshot before returning/caching a session; fail on linked/non-directory/changed ancestor. A deterministic injected lstat phase law can own this race without scheduling nondeterminism. File descriptor/name and child ancestry checks remain supplied by semanticOwnedInputFileSnapshot. POSIX directory-handle openat style absolute race elimination is a separate stronger platform mechanism; do not invent guarantees from lstat alone.

API/public types remain first-party/system; no physical input callback from normalization or reverse IO import found. Existing error identity rebindings preserve unsafe ancestry classification versus ordinary permission failures.
