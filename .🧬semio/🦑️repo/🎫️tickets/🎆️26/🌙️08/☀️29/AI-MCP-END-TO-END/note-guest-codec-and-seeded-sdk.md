# Note guest codec and SDK history

The replay command links the workspace's Note snapshot, mutation, SPR and store crates from the same guest build and runs as a WASI Preview 2 command through Jco. The producer encoded the Note with identity `note-seeded-sdk`, stamped the `s.note.note` dialect into its journal, and yielded a 306-byte Pack and 216-byte SPR. Replaying the two files reconstructed `note.document` with the original identity, zero blocks, a visible grid, spacing 32 and snapping disabled.

An independent native folder writer admitted that pair. The official MCP SDK opened it through the native server and observed artifact kind `s.note.note`, schema `note.document`, plugin `note`, and editor `s.note.note@1/*#editor`. The same session resolved the seeded artifact as active, described Add Block and read back both byte streams. The SDK then submitted action preparation. The unmodified create path runs in another session.

The SDK baseline Pack and SPR matched every original base64 character. With the first published Note component the SDK timed out after 300 seconds waiting for `action_prepare`. No mutation or history result was established by this attempt.

In a fresh run both the original and restarted native processes returned the expected persisted bytes, dialect and editor, and both answered SDK ping. The harness checks that their process IDs differ.

The first component timed out at 300 seconds on `artifact_export` in both runs. Those calls established no exported archive.

The ticket inputs are [guest codec linker](note-mcp-wasi-replay.py), [canonical replay](note-mcp-replay.rs), [SDK history](note-seeded-history.ts) and [history verifier](note-mcp-replay-verify.ts).
