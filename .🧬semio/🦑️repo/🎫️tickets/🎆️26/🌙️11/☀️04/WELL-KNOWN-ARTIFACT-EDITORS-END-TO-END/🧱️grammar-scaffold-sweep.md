# 🧱️ Grammar/Protocol Scaffold Sweep (T15e, static)

Every snapshot protocol that feeds a native receipt is real; none is a stub.

Stubs and scaffolds below do not feed receipts. All must be completed to describe the real codec, or removed (no unfinished artifacts):

| Owner | Artifact | Files |
| --- | --- | --- |
| G | wav | text snapshot/mutations/diff `.ebnf`, `.g4`, `📖️.grammar.semio` (diff); binary snapshot/mutations/diff `.abnf`, `.ksy`, `.spicy`; binary diff `📡️.protocol.semio` (magic-only) |
| G | mp3 | binary snapshot `.abnf`, `.ksy`, `.spicy`; binary diff `📡️.protocol.semio` (magic-only); text diff grammar (from the T15 review) |
| G | avi | `🎛️hdrl/🚪️io/🦀️.rs:1` and `🟦️.ts:1` carry a "🚧 scaffolded by W1b" marker |
| A | html | `🚪️io/🦀️.rs:1,69` and `🟦️.ts:1` scaffold markers; binary diff `📡️.protocol.semio` (magic-only) |
| B | tsv | `🚪️io/🦀️.rs:1` and `🟦️.ts:1` scaffold markers |
| E | ply | text mutations `📖️.grammar.semio` |
| H | semio flow | text snapshot `📖️.grammar.semio` |
