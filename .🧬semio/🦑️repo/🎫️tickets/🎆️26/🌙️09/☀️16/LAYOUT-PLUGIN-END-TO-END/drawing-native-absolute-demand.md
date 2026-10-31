# Native Absolute Demand Source Packet

The canonical native ledger now exposes `admits_absolute(demand) -> bool` and infallible `require_absolute(demand)`. Cached producer demand is already in whole-ledger depth and copy units. Admission compares copy against actual unspent grant without subtracting held copy again; registration preserves the cached demand exactly. Relative `require_child` remains available for genuinely relative child demands.

The nested repeated-denial law and language-neutral absolute schema/data were authored before these methods. The law retains a real parent Vec reservation while a nested PagedList allocation is denied repeatedly, then admits the exact cached quote. This new law and changed source have not been executed. The earlier native 29-pass result remains historical.

Numeric insert/remove, generic trace, scene trace, preparation/resolved preparation and placement cached prechecks now use the absolute API. Rust owns its separate producer migrations. No TypeScript core/counter/bytes edits were made during the numeric freeze.

Packet SHA256: `e51904a825c839b98afbe561ba851a11cbf421f7d8ae63b1c2c474c62a51b715`. Packet records current native primitive/law and numeric production/schema inputs. Numeric test wallet migration remains outstanding; this packet is Source-only, not full Numeric or Drawing readiness.
