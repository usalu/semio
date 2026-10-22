# Native Repository MCP Root Scope

## Authored Test Boundary

The actual CLI MCP repository crate now mounts one native #[test] in its own root through cfg(test). It uses the production RepoRepository::open(provided_root, explicit_profile), repository_server and serve, then sends initialize and resources/read for repo://files and repo://folders. The semantic test is in CLI/mcp-verb/root-scope/tests rather than the unmounted adapter handshake test.

Schema and portable fixture precede implementation. Three bounded private roots (first, second, empty) are allocated beneath required SEMIO_TEST_ARTIFACT_DIR; each populated root has two handcrafted Rust/TypeScript DATA files and a distinct child folder. Two explicit profiles produce six real server conversations and twelve resource reads. Exact expected relative path sets exclude the sibling root and ambient checkout; an empty provided root must remain empty. The process cwd and authored file contents are asserted unchanged. Fixture descendants are bounded to256 UTF-16 path units, output to64KiB, and fixture files to256 bytes; no AGENTS file, source copy, Git call, global cwd change or external process is involved.

The existing serde_json dependency independently parses actual JSON-RPC wire lines and checks unique expected response IDs/envelopes. Actual resource MIME is application/json. The independent serde_json parser checks payloads and exact closed metadata keys against the shared files/folders JSON schema before exact path comparisons. All dependencies already belong to the crate; no Cargo or lock edge was added.

## Runtime Reachability and Remaining Proof

The existing @semio-tech/repo-cli-rs:test target dispatches its actual TestScript to runRepositoryCargoTests for seven named native crates (the prior six plus the shared workspace root-selection crate), including semio-framework-repo-cli-mcp-verb. The new cfg(test) mount therefore reaches this law in that declared native gate; no new target or Source43 is needed. Actual compiler/admission/body/whole evidence is pending and must name provided_root_bounds_real_repository_mcp_files_and_folders. This in-process law does not itself prove installed frontend provenance, SDK stdio behavior or external process shutdown. The same declared gate now separately runs the typed owned SDK suite after native success and genuine official installation; its six bodies and six real child conversations remain unrun. The existing OS canonical stdio probe exercises a different OS server route and cannot be relabeled as repository MCP root-scope proof.

## Independent Source Review

Low accepted the mounted test/API/fixture boundary without finding an obvious compile or scope blocker. Current Codebase discovery considers the handcrafted source extensions and derives child folders while excluding the root dot folder, matching the portable expectations. Source acceptance is conditional on the genuine native gate: provided-root adapter scope is distinct from installed frontend provenance and its current_dir/find_repo_root launch binding, which this in-process law does not exercise.

## Current Launch Contract

See oct21-installed-repo-mcp-scope-design.md for the authored explicit-root selector, actual seven-crate plus typed SDK mount, authenticated publication proof and current runtime limitations. No Source43 was added.
