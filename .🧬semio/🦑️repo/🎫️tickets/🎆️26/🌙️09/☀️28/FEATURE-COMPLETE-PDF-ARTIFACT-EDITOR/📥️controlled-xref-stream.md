# Controlled xref stream section

The section cursor frames the original indirect object, scans its borrowed dictionary by entry, locates the stream body, drives filtered output custody, and emits one typed row per turn. It keeps the original dictionary with the frame owner and transfers it after row publication and decoder retirement. The ordinary xref reader drains these stages synchronously. Its compatibility path reuses the already-framed source span and dictionary for decoder format or unsupported-filter refusals.

The dedicated xref-stream package owns the section and row cursors. The graph-projection owner reexports both, and its measured test closure is 27 inputs and 3,813 active lines. One-unit tests on a genuine compressed incremental revision cover all four rows, typed trailer order, and exact backing retirement on cancellation. A raw DEFLATE variation checks the existing recovery path. MuPDF independently produced the same 28 decoded cross-reference bytes and identified the freed entry and the updated document metadata.

The isolated harness passed all 37 tests against the actual production sources, including the new normal-object handshake and both xref filter recovery paths. It used repository-built DEFLATE and value dependencies, with unrelated workspace derives omitted.
