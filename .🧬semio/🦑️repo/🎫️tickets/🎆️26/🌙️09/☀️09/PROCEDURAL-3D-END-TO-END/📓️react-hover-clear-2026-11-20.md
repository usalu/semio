# React Flow reverse hover clearing

The Generation 3D viewport already removes the node graph hover from the scene on exit. React now treats a disappearing hover as a worker clear when the scene still owns the hover. The interaction ledger adopts the clear before a worker read can publish it back to the plugin. Repeated absent scenes, reflected local hover, and a newer graph pointer or keyboard hover do not issue a scene clear.

Canvas attachment and regular scene updates use the same hover reconciliation. Node, edge, port, and clear mutations share one observed worker task lane, so changes replace pending mutations across hover kinds. The hover publication validates the scene revision captured by its read, while selection publication retains its own existing check.

## Verification

The portable JSON sequence exercises a viewport channel disappearing, stale worker readback after a clear, a pointer intervening before readback, a local pick of the same target, an echoed local mark, and an explicit empty mark. Its new Vitest assertions failed before the reconciliation and revision checks were implemented. The focused replay and adjacent interaction publication suite then passed together using the repo Bun launcher, with 23 passing tests and 103 assertions. The Playwright browser flow was outside this focused verification.
