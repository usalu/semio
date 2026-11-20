# Controlled indexed object stream members

The checked-in Flate PDF publishes a language-independent input with the following expected member selections.

| Authored index | Object number | Decoded byte span | Expected COS value |
| --- | --- | --- | --- |
| 0 | 6 | 26–70 | Dictionary with Title `Compressed` and Subject `Stream` |
| 1 | 7 | 70–102 | Array containing true, name Object, string packed, and reference 6 0 R |

A forward selection must match the authored object number at its ordinal. Only the selected range may be parsed. The cursor must support single-work grants and report actual allocation and release, hand over the typed value once, and retire its own capacity on cancellation or after transfer.

The cursor holds one pair of lookahead and one active COS owner. An identity mismatch leaves the pair available to the correct selection. A selected member ends at the next authored offset or the end of the decoded stream. After transfer, parser scratch retires under release grants before the cursor accepts the next index.

The production-source xref harness passed all 82 tests. The compound tests matched the fixture's typed outputs under single-work grants, reconciled capacity after transfer and cancellation, and rejected a mismatched object number and a trailing COS value. The ordinary Flate object-stream resolver also passed. PyMuPDF 1.24.14 opened the same PDF without repair and independently matched both typed objects and their exact decoded byte spans.
