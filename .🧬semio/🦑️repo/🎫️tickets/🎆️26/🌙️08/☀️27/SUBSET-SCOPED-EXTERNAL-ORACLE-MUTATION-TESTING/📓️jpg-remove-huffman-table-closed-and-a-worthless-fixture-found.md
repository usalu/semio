# 🪓️ `jpg::remove-huffman-table` closed — and the fixture it already had proved nothing

## My negative was wrong for the third time in the same way

I had recorded `remove-huffman-table` as blocked because "no writer emits a JPEG whose DHT list can be
varied while the result stays decodable", resting on: Pillow's Huffman accessors are empty and
deprecated, and `zune-jpeg` does not expose the tables.

Both of those are true and neither was the question. They survey LIBRARIES. Protocol v2 lists
`third-party-cli` as a qualifying oracle kind, and the installed toolchain answers it outright:

```
$ djpeg -verbose -outfile /dev/null color.jpg
Define Huffman Table 0x00
Define Huffman Table 0x10
Define Huffman Table 0x01
Define Huffman Table 0x11
```

`djpeg` enumerates the table list explicitly. This is failure mode (c) for the third time — **the
inventory was scoped and the scope was never stated** — and the subset had *already* registered
`libjpeg-jpg-jfif-1-01-marker-cli` for its sibling kinds, so the oracle was sitting there unused.

## The writer, and the honest limit of what the pair isolates

`jpegtran` cannot do it: it is a lossless transcoder and never drops a table (`-optimize` on a
progressive file silently converted it to baseline, 10 DHT → 4, changing the frame). `cjpeg -scans`
can, because in JPEG **a Huffman table is defined BECAUSE a scan references it**:

| scan script | tables |
|---|---|
| DC(0,1,2) + AC(0) + AC(1) | `0x00 0x01 0x10 0x11` |
| DC(0,1,2) + AC(0) | `0x00 0x01 0x10` |

Exactly one table removed (AC chroma `0x11`), both halves decode with no error. The generator asserts
all three facts rather than trusting them: exactly one table removed, no table appears that was not
there before, and both halves still decode.

**The limit is stated in the manifest, not papered over.** The pair does not ISOLATE the table from the
scan — no conforming writer can emit a file whose table list differs by one entry while the scan list
is identical, because the table's existence is entailed by the scan. The pair witnesses the mutation's
observable effect on the table list; it does not separate it from the scan that entailed it.

## The fixture it already had was worth nothing

While binding the oracle I checked the fixture already registered for this kind:

```
remove-huffman-table-applied
  expected-before-jpg  sha256:70e16bf0…  1278 bytes
  expected-after-jpg   sha256:70e16bf0…  1278 bytes
```

**The same file twice**, declared `outcome: "applied"`. The oracle could compare those halves forever
and never see the mutation, and every coverage dimension counted it as a fixture.

A repo-wide sweep found 23 fixtures with byte-identical halves out of 450 pairs. Most are correct:
`no-mutation` and the docx `-no-op-` fixtures are identical BY CONSTRUCTION — identical halves are
exactly what they assert. Four more are honest records of an ENCODER gap, self-documented in the
recipe list (`"this subset's own production encoder regenerates DQT fresh from re_encode_quality on
every write"`), and each of their kinds separately carries a genuinely differing `marker-`/`libjpeg-`
pair, so no coverage claim rests on them.

`remove-huffman-table` was the only one where the degenerate pair was the kind's ONLY evidence. It has
been replaced by the scan-script pair, and the stale recipe retired.

## The gate, and the gate's own test

Two checks added to `🧪️verify/📜️script.ts` (119 → **121**):

* `fixture/applied-pair-must-differ` — no `applied` mutation may be evidenced ONLY by a byte-identical
  pair. Deliberately not "no fixture may have identical halves": that would fire on `no-mutation`,
  the one kind for which a DIFFERING pair would be the bug.
* `fixture/degenerate-only-pair-is-caught` — the injection companion, because a gate that is never
  shown catching its fault is an untested gate. A synthetic mutation carrying only a degenerate pair
  must be flagged; the same pair alongside a genuine one must not be.

## Result

| | before | after |
|---|---|---|
| externalOracleCoverage | 600/614 | **601/614 (97.88%)** |
| oracleEvidenceCoverage | 600/614 | **601/614 (97.88%)** |
| harness | 119/119 | **121/121** |

13 remain — `mathematical` 9 and `sequence` 4 — and both are the same EXPORT defect, not an oracle or
fixture gap. See `📓️last-14-are-export-defects-not-oracle-gaps.md`.
