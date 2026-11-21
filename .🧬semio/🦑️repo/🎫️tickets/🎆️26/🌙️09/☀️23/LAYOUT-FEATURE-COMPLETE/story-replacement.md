# Layout Story Replacement

The linked Layout build found that the document-command crate called a private frame-command helper after the commands were split. The Layout model's existing story-splice module now supplies one replacement calculation to both crates. The patch-document structure command, text-chunk command and patch-frame command all use scalar offsets and share the same native implementation.

The native calculation walks shared prefix and suffix scalars directly on borrowed UTF-8 and allocates the changed text. The TypeScript model implements the same scalar coordinate contract on UTF-16 strings. It handles changes inside emoji sequences, non-BMP characters, combining marks, an unchanged story, and insertions and deletions at the end.

The 11-case fixture first failed on missing model functionality. The implementation passed the fixture and independent Lodash oracle. Strict TypeScript checking passed. The four changed Rust files passed the pinned parser, and the model has a native law consuming the same JSON fixture. A new authenticated WASI guest is compiling the consolidated callers.

The selected guest `wL2Cmhj` emitted the actual WASI model and the editor commands-2 crate, including both document command callers. The guest compiled the replacement while its independent PDF editor and Drawing Store providers were undergoing shared API changes.
