# 📓️ What `png` 0.18.1 can actually witness — 12 of 15

Researched by a peer session against the crate source in the local cargo registry, and independently
re-verified here before being relied on. Recorded so the retrofit does not repeat the lookup and does not
over-claim.

## Cannot be witnessed — `png-1-2-mutate-uncarried`

| kind | why |
| --- | --- |
| `change-timestamp` | `png::Info` in 0.18.1 has **no `tIME` field**. Verified: no `tIME` and no `pub time` anywhere in `src/common.rs`. |
| `insert-unknown-chunk` | The decoder **skips** unrecognized ancillary chunks rather than surfacing them — `src/decoder/stream.rs:98`: *"Skipped an ancillary chunk because it was unrecognized…"*. No public accessor exists. |
| `remove-unknown-chunk` | Same reason: what was never surfaced cannot be observed to have gone. |

A reader cannot witness a change to something its public API never exposes. These three are honestly
un-oracled through this crate, and inventing a probe that appeared to check them would be worse than the
gap.

## Witnessable — the other 12

Through `Info`'s public fields, all confirmed present in `common.rs`:
`bit_depth`, `color_type`, `interlaced`, `width`, `height`, `trns`, `gama_chunk`, `chrm_chunk`, `srgb`,
`pixel_dims`, `bkgd`, `uncompressed_latin1_text` — plus `replace-pixels` through the decoded sample
buffer itself.

## Why this is recorded rather than just used

It is the shape the whole protocol asks for: the carrier decides what is oracle-able, not the domain and
not the mutation. Twelve of fifteen is the honest answer for this subset, and the three that fall out do
so for a reason anyone can check in the crate's own source in under a minute.
