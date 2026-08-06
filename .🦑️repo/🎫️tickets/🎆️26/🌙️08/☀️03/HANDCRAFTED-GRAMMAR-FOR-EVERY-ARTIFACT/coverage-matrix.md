# Coverage matrix (handcrafted grammar program)

| Artifact | dsl spec | pack protocol | TS facade | LSP | Writer opens |
|----------|----------|---------------|-----------|-----|--------------|
| dag | handcrafted graph | dag.pack/spr proof | stub | jack idiom | jack example |
| fem2d | handcrafted sheet | stub | stub | — | — |
| note | handcrafted scene | stub | stub | — | — |
| * (remaining) | seeded stub | seeded stub | seeded stub | — | — |

P5 `DocumentDsl`/`OpText` derive removal is **gated**: all non-pilot artifacts still parse/print through `#[derive(DslDocument)]` until W4 replaces stubs with handcrafted parsers and allowlists reach empty.
