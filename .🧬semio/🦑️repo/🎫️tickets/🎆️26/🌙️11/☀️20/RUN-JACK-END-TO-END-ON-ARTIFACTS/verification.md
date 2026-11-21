# Jack on Artifacts

## Runtime paths

- The Trinity editor runs Jack against the selected retained graph child. The installed Hub composition supplies the Semio graph implementation. The shell uses the same evaluator with an in-memory scene.
- `MATCH` joins existing bindings. `CREATE` and `MERGE` bind created or matched entities. `SET` and `DELETE` act on matched rows. The executor calculates graph effects against a staged graph, and the child store publishes semantic operations as one edit.
- The results window is selected at command admission. The document query, selected-root change and graph edit publish together when the job completes. A failed batch preserves the document and its child.
- Topology and flat positions read an exact retained graph identity. The live inference API accepts the graph from the active composition, so it sees edits as soon as the child store publishes them. Rust and TypeScript produce the same deterministic results for the shared fixtures.
- The Hub activates both Jack surfaces through `s.trinity.jack`. Other artifact kinds need their own graph adapter and mutation mapping to serve as Jack content.

## Verification

- The pinned Bun runtime passed 63 portable tests across eight files covering inference, graph mutation, mounted-publication fixtures, typed JSON transport, archive carriers, examples and original-record ownership. Graphology independently reproduced the topology and position fixtures. SQLite independently reproduced the query results, mutation states and undo fixtures.
- The separate document contract and graph-window configuration oracles passed. The document contract now checks the before and after snapshot pair for each mutation fixture.
- The native Rust graph-inference package passed its five unit laws. The initial command attempted doctests through a missing `rustdoc` executable on PATH.
- Bun with SQLite and Ajv, and Python, passed the language-neutral `SetRoot` clear, inverse and no-op fixtures.
- A Node check parsed the Hub descriptor and verified one canonical Jack activation shared by the editor and viewer.
- The final native aggregate passed all 104 artifact tests, all 15 executor tests, and all four shell tests. The artifact suite exercised all 15 SQLite snapshot laws with the pinned Bun runtime in PATH.
- The native dev composition passed all 22 content and reconstruction laws, including recovery of the original snapshot field allocations through paid close.
- The Semio graph mutation codec passed its standalone native round-trip law using the original binary receiving cursor.
- All seven mounted query tests passed. They cover explicit and focused results-window routing, persisted query text shared across editors, separate results per window, undo and reopen. A rejected editor-window target retired without publishing or blocking the app.
- The mounted mutation law verified `CREATE`, rollback of an invalid mutation query, inference from the live edited graph, and one undo restoring the original graph. Root `DELETE` removed the incident edges and cleared the document root in the same edit. A single undo restored the root, graph, and edges. The fixture verified paid close after both edits.

## Build fallback

The repository fleet queued on graph and capacity admission during the run. Its first planning attempt exposed a missing cold WASM output declaration for the Jack language server. The authored producer now declares its JavaScript and type outputs, and the full workspace registry resolved the real worker import without reading generated files.

Native Cargo used the pinned compiler and an isolated ticket target. The fallback exposed current framework API changes in the original retirement and snapshot receiving contracts. The Jack adapters and affected fixture drivers were updated against those contracts. The Jack and Semio workspace lockfiles were refreshed for the internal crate graph. Their third-party package versions were unchanged.

The mounted editor test exposed a catalog join failure for generic tool factories. The join now checks the declared outer factory name while keeping the exact typed identity check.

The query worker and its registration share an execution contract that funds indivisible original-source admission. The command and parent snapshot admit through bounded native turns. The child qualifier checks the exact physical source binding and validates the mounted edit independently through the child lookup. The typed and erased references use the same retained parent projection for the read stamp and mutation authorization. Read-only queries skip original mutation custody.

The Semio graph adapter transfers its owned mutations into the child group. The worker qualifies those mutations using the original command, parent snapshot, and exact child read. Semio graph member creation also installs its original binary mutation receiver and semantic publication and replay factories. The Semio graph codec and protocol mirrors share tagged PackValue encoding, and the registration check covered all 18 mutation types.

The child publication path now quotes the original receiving turn before it advances the decoder. It also uses work available inside that loan. The installed operation receiver accounts for its allocation once. The mounted publication loop lends each child group step its remaining preparation budget, including nested decoder scopes. The committed group's cleanup returns its visibility aliases through the funded retirement API while history keeps its read reference. Both portable child group laws passed against the independent JSON patch oracle.

The typed decoder returns progress before starting a subsequent action that exceeds the remaining work allowance. The typed-command route validates the requested results window before taking ownership of the command and child views. A refused command keeps its original source lease until retirement, which lets the editor admit a later command.

Live maintenance now selects occupied displaced-owner queue entries. The parent store can reserve slots during a grouped edit without triggering the terminal queue check. Full store retirement still requires all slots and queued owners to be returned.

The mounted Jack editor's document store and the installed Semio graph member both register their original semantic preparation and binary receiving owners. This lets the root mutation and its child edits share one private publication and one undo entry.

Concurrent image source paging changes exposed missing native builder and contract fields during the Jack rebuild. The Rust builder, field tables and kernel JSON projection now accept the retained source carrier. The image cases also matched the independent Ajv oracle.

## Coverage limits

The native runs used macOS. The portable fixtures use Graphology, SQLite, Ajv and JSON Patch as independent oracles. Semio mutation planning currently keeps the final conversion to native edit operations in one worker turn after the paged query evaluation. The remaining planning bound and a paged design are documented in [content-adapter.md](content-adapter.md).
