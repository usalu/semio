# Puzzle 3D Editor Audit

Owner: editor_execution_b.

Read root, s and puzzle AGENTS.md. The artifact lives under ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d. Its Rust editor has real registry-backed dispatch, selection, transform, catalogue, snapshot exchange and reopen tests. The artifact package also owns SQLite snapshot codec laws and independent source checks.

Verification queued through repo CLI because this session exposes only ticket/goal repo MCP tools: @semio-tech/puzzle-3d-rs:verify-original-work-construction and @semio-tech/puzzle-3d-rs:test. Host capacity currently delays both behind existing jobs. No source changes yet.

## Import Identity Regression

Added a portable eight-row corpus and a registry-backed native import test with serde_json as the independent parser. Foreign schemas previously passed through conversion to LoadDocument; import now admits only the two declared 3D schema identities. Wrong object shapes continue to fail typed decoding.
