# H35 — hash / flatten-merkle / quality-sum census
> Agent returned this report inline (it ran with a read-only toolset and could not write).
> Transcribed verbatim by the coordinator. **Claims marked ⚠️ were independently re-verified by the
> coordinator — see `📓️w1-coordinator-verification.md`.**

# A. HASH

## 1. Fixture `compose/fixture/hash.cases.compose.json` (22 lines)
4 named cases:
- `kitHash` — kit-level hash of `kit/dev/metabolism/wip/initialKit/kit.compose.json`
  - Expected: `a19a962ee064dd7bd5e389263f576696ced699a5afc62e47422dc3329e23e44e`
  - ExpectedNet48: `3640c4cf3718af017555369e6f2396e7c3b8e0f74830599dad4e3f2ee2c39268`
- `kitDiffHash` — JSON diff `{"name":"updated","description":null}` → `d9ee3052111fec2e0fe08119eee6b8d5b6f5578a940f6d5c6bb1806e6e0f36a5`
- `designName` — "Nakagin Capsule Tower"
- `sha256Known` — reference SHA256 vectors (empty string, "abc")
- `kitDiffTypeAddition` — type metadata fields

**Note the `ExpectedNet48` field: the .NET Framework 4.8 client produces a DIFFERENT hash than the
others. Cross-implementation hash parity is already broken in compose.**

## 2. `compose/fixture/_debug_hash.txt`
Single line: `001a1ced4db6968ea00efddd1a9161a43e42a66acc31375be98e461ef1a5b436`.
Committed debug artifact (commit dd90479f7e). `grep` finds zero readers → stale. Migration status:
`obsolete-with-explicit-approved-reason`.

## 3. Rust implementation — `compose/client/lib/rs/lib.rs:3152-3216`
Algorithm: **BLAKE3** (`crate::external_adapters::blake3::Hasher`).
- `h()` :3156 — joins parts with `\x1f`, blake3 hex digest
- `format_number_for_hash()` :3166 — canonical f64: NaN→`"nan"`, ±Inf→`"inf"`/`"-inf"`,
  integral ≤1e15 → no decimal point, else 12 decimals trailing-zero-trimmed, `-0`→`"0"`
- `merkle_node_str()` :3196 — **sorts children** before combining
- `merkle_collection()` :3211 — `merkle_node_str(&["RelayCollection"], children)`

## 4. Canonicalization
Strict. Floats canonicalized; child digests sorted → HashMap-iteration-order independent.
Field order matters only for the `own` slice.

## 5. ⚠️ Do schema-domain labels feed the hash? — **YES**
`merkle_node_str(own: &[&str], …)` consistently receives entity-type labels
(`&["Quality", …]` :595, `&["Typology", …]` :1842, `&["Position", …]` :1085).
**Consequence: renaming Type→PartKind, Piece→Part, Connection→Fastener CHANGES every raw hash.**
Raw hash equality across the terminology change is impossible by construction. The migration must
record old-normalized-digest and new-canonical-digest as two separate values (plan §8.3).

# B. FLATTEN-MERKLE

## 6. Fixture `compose/fixture/flatten-merkle.cases.compose.json` (208 lines)
13 cases. Parity block pins planeHash+centerHash for 2 Nakagin pieces:
- `7dc5b737-3b6b-4068-b315-b7bacc91c2e1` plane `ad8a9237…` center `334c87ee…`
- `5f0266bc-856b-4ef2-9eb0-16ef5e1fb952` plane `8abc6a80…` center `8ed02a0f…`

11 mutation variants establish the **sensitivity matrix** (which input perturbation moves which hash):
| perturbation | planeHash | centerHash |
|---|---|---|
| description change | stable | stable |
| piece `plane.origin.*` | changes | stable |
| piece `center.*` | stable | changes |
| connection `gap`/`rotation`/`tilt`/`shift` | changes (connected pieces) | stable |
| connection `u`/`v` | stable | changes (connected pieces) |

This matrix is a **ready-made oracle** for the migrated flatten solver — it constrains which
geometry inputs may influence which output channel.

## 7. Rust — `compose/client/lib/rs/lib.rs:1085-1095`
`compute_hash()` formats 11 floats at `{:.9}` (center.u/v + plane origin xyz + x_axis xyz +
y_axis xyz), sorts `[center_hash, plane_hash]`, then `merkle_node_str(&["Position", id, flat], ch)`.
Tree is built over the FLATTENED design (`flatten_design_positions()` :1393). Order-stable.

Go reference `compose/client/lib/go/main.go:15016-15089`: `hashPlaneRoot` :15017,
`hashPlaneChain` :15039 (parent hash + connector + gap/shift/rise/rotation/turn/tilt),
`hashCenterRoot` :15065, `hashCenterChain` :15080 (parent + connector z + u,v).
**Go uses SHA256, Rust uses BLAKE3** — another cross-implementation divergence.

# C. QUALITY-SUM

## 8. Fixture `compose/fixture/quality-sum.cases.compose.json` (13 lines)
1 case: `sum_effective_floor_area`, kit `…/initialKit/kit.compose.json`,
design "Nakagin Capsule Tower", quality "effective floor area",
**expected `2349.53`, tolerance `0.01`**.

## 9. Rust — `compose/client/lib/rs/lib.rs:5279-5281`
```rust
pub async fn quality_sum(&self, _quality_id: Id) -> f64 {
    0.0
}
```

## 10. ⚠️ ORACLE CONFLICT — **CONFIRMED CONFLICT**
Rust returns constant `0.0`; fixture expects `2349.53`. Per oracle precedence (plan §6.1) the
committed fixture wins and the **Go implementation is the real specification**:
`SumQualityInDesign` `compose/client/lib/go/main.go:5639-5678` — sums piece `.Props` values parsed
as f64 by quality id, falling back to the piece's Type props when the piece lacks the quality.
Test `TestDesignQualitySum()` :2286-2316 validates against the fixture with tolerance.
**Migration MUST implement real summation. Do not port the Rust stub.**

## 11. Quality data model — `compose/client/lib/rs/lib.rs:518-554`
`id`, `key` (e.g. "effective floor area"), `value` (optional string), `unit`, `definition`,
`description`, `icon`, `benchmarks` (ranges), `attributes` (kv).
Lives on **types and pieces** as properties; pieces reference via `.Props` (prop id + quality id + value).
Values are **strings parsed to f64**, not native numbers — a migration decision point.

## 12. Other languages
- JS `compose/client/lib/js/index.ts:3097,3240` — `declare qualitySum: () => Promise<number>`,
  `readKitBranchNumber(frag,"design","qualitySum")`. Client binding only, no implementation.
- Python `compose/client/lib/py/main.py` — wraps compose-gql binary via GraphQL, no local impl.
- No dedicated unit tests outside Go's `main_test.go`.

# Summary
| Family | Algorithm | Canonical | Order-stable | Status |
|---|---|---|---|---|
| hash | BLAKE3 (Rust) / SHA256 (Go) | strict | yes | ⚠️ cross-impl divergence |
| flatten-merkle | as above, 9 decimals | strict | yes | usable oracle + sensitivity matrix |
| quality-sum | — | — | — | ⚠️ **Rust stub 0.0 vs fixture 2349.53 — Go is the spec** |
