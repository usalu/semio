# Drawing Boundary Owner Closure

The registered `repo-lib:lint-artifact-io-ownership` request `artifact-io-live-audit-3` completed with five breaches after checking 30,131 entries. Three breaches came from current Drawing source: a schema snapshot's `retire_with` attribute depended on SQLite I/O, the schema imported pure constructors through text I/O, and pure mutations imported native JSON decode/render bridges. The two remaining breaches were PDF conformance support and diff source, assigned to the PDF worker.

Drawing's semantic identity and document constructors now live in the schema owner. Native JSON apply/undo wrappers now live in text mutation I/O and call a semantic diff-and-apply helper. The binary operation trait implementation now lives in binary mutation I/O. Consumers are updated through the ticket's `runtime-drawing/📜️script.ts restore-owners` command; no compatibility reexports are introduced.

The value derive was inspected before changing retirement: `FromValue::retire_decoded` uses `retire_with`, while the separate `RetireOwned` derive ignores that attribute and retires fields through deferred first-party cursors. Thus the original hook was a physical dependency, not a demonstrated recursive retirement loop. The new semantic snapshot hook owns the first-party cursor directly; SQLite rollback uses that same semantic owner without depending on a host factory.

A neutral native representation fixture and independent tree-sitter test require each concrete Rust codec trait or macro in its matching text, binary, or SQLite representation. The registered test first completed RED: ten existing tests passed and the new representation test failed because a binary codec under text I/O produced no breach. The implementation now reports this dedicated breach; its GREEN request and a fresh complete ownership scan are pending.

Native Drawing and the final ownership audit remain pending. Live test output now stays in command sessions as well as ticket-owned output because external cleanup repeatedly removed the entire generated folder during active runs. PNG/BMP parity requests ended with exit 1 after their logs disappeared; the underlying cause has not been recovered and those runs are not successful evidence.

The registered constructor-consumer rewrite completed with `[DEBUG] Canonical Drawing domain/native owner consumers updated: 33`. This is source authoring evidence, not native runtime verification.

Nx's retained terminal outputs recovered the partial image results: PNG's oracle executed twelve exhaustive scenarios successfully and BMP's oracle executed nine. Both subject hosts failed preparation with exit 101, so neither run proves parity. The error output ends after roughly 64 KiB of machine Cargo artifacts; its compiler diagnostic was not recoverable there. The native image workers retain separate compile requests to establish the actual cause.
