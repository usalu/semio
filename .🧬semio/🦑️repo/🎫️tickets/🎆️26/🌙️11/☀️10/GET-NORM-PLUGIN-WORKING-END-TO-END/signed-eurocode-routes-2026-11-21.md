# Signed Eurocode route verification

## Retained publication

The port 6034 delivery recheck passed for all three signed families from `wa041jP`. Their wrapper receipts and SHA-256 digests match the earlier Chromium route sweep. The server returned the exact matching browser receipt, bridge, JavaScript bindings and Wasm core for each component.

| Family | Signed wrapper SHA-256 | Served browser core SHA-256 |
| --- | --- | --- |
| `norm-eurocode` | `cd30e0caa34c514e4c5f2d6f5c5ed4942a22959b41121a593bed1c370c6ffbc2` | `d78a58639c5a6e8d628d1e2ff8f4c3ca5bd774e025fc58a7aa2e0ff414f40942` |
| `norm-eurocode-structural` | `612b6afeb983341a19bd031b7a9c831c7421825c47e86ce8d5d6059c97c0ed6b` | `40961c43db2616fd04b3adcc5157599a5fb56ddbf5924f085c57da44effc522b` |
| `norm-eurocode-materials` | `ba785ac9542ae3e6e310ddf71a010d89fa13e2fe6461bce466eae6b88e77157e` | `22a4c454081fd85a8e4f52f8529b6bdc0fc337db5426bdfc28e922ccba540b4a` |

The earlier sweep drew every retained Eurocode route at current geometry revision 1, with zero browser faults and zero failed requests. All draws used a 665 × 609 canvas.

| Variant | Elapsed seconds | Drawn vertices |
| --- | ---: | ---: |
| `en1990` | 44.3 | 24 |
| `en1991` | 46.7 | 96 |
| `en1992` | 124.1 | 864 |
| `en1993` | 31.9 | 72 |
| `en1994` | 35.4 | 1,200 |
| `en1995` | 28.1 | 24 |
| `en1996` | 30.4 | 24 |
| `en1997` | 29.8 | 1,440 |
| `en1998` | 30.7 | 24 |
| `en1999` | 32.8 | 580 |

## Successor publication

All three `wB34Xuf` components finished compiling and entered descriptor emission. The build owner retired the optional publication while the signed `wa041jP` receipts remained active. Its visual-chrome dependency recorded the earlier 9,291-byte source file. The final gallery fix used the later 10,604-byte source file.

The final port 6034 delivery recheck exited successfully. For all three families, its wrapper digest and served receipt, bridge, bindings and browser-core digests exactly matched the earlier `wa041jP` Chromium run and this watch's first delivery recheck. The ten current-revision draws above are from that earlier run against these same signed bytes.

The later gallery fix passed 332 checks with zero failures in a separately rebuilt WASI leaf. That test exercised all nine authored EN 1990 examples in English and German, including paired first and last marker and data rows for the Road Bridge Showcase matrix. See [Gallery and viewport verification](eurocode-gallery-and-viewport-final.md).
