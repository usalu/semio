# 📓️ What is oracle-able today, and what is blocked — measured, not estimated

The repository has **1711 mutation leaves across 122 subsets**. The question that decides the shape of
the remaining work is not "which library could judge this?" but "does the artifact write bytes a
third-party reader can read at all?"

| | subsets | mutations |
| --- | ---: | ---: |
| Real, non-JSON standard-format carrier — **oracle-able today** | 47 | **300** |
| No real carrier — blocked on writing an exporter first | 75 | **1411** |

> **These numbers replace an earlier, wrong pair (664 / 1047).** The gate that produced them was missing
> a fourth stub shape; correcting it moved 364 mutations out of "reachable". The largest single
> correction is `architect/program`'s 266. The details are below, because a measurement that moved this
> far is worth showing rather than quietly restating.

JSON is excluded deliberately. A JSON export of our own schema is our own schema in JSON syntax; a
validator confirms its shape, not that a mutation computed the right answer. Counting it would inflate
the reachable figure by roughly 370 mutations that no independent reader can actually adjudicate.

## The stub gate was under-reporting by 86%

`stubSerializerBreaches` shipped finding 80 broken exporters. It now finds **149**. Four false-negative
classes and one false-POSITIVE class were closed; each was found by using the gate for something,
not by reviewing it:

1. **DSL text under a standard extension** (99) — `serialize_bytes` returns `print_dsl(..).into_bytes()`.
   The original finding.
2. **Envelope transmute** (31) — `encode_pack` the SOURCE snapshot, then `decode_pack` those very bytes
   **as the target type**. `shooting → png` reinterprets a `ShootingSnapshot` envelope as a
   `PngSnapshot`. This is not an unimplemented export, it is type confusion: at best it fails on the
   envelope id, at worst a lenient decode yields a structurally valid document full of noise. Rated
   high priority unconditionally.
3. **Text-only serializers** — several owners deleted their byte path entirely and kept only
   `serialize_text` returning `print_dsl`. Keying the gate on `serialize_bytes` skipped every one.
   `cad` exports STL, OBJ, glTF, IFC and STEP this way, and the whole owner read as having real carriers.
4. **Serde coercion** (19) — the quietest of all, and the most expensive. `serde_json::to_value` the
   source, `from_value` into the target:

   ```rust
   let value = serde_json::to_value(snapshot)?;  // ProgramSnapshot: 66 registers
   serde_json::from_value(value)?                // -> XlsxSnapshot { schema, opc: default, workbook: default }
   ```

   Every non-`schema` field of these targets carries `#[serde(default)]` and unknown keys are ignored,
   so the conversion SUCCEEDS and yields an EMPTY document for every input. `architect/program`'s xlsx
   export turns 266 mutable registers into a workbook with no sheets and reports `Ok`. It neither prints
   DSL nor transmutes an envelope, so the three earlier detectors all passed it — and 266 mutations were
   counted as reachable on the strength of an exporter that can never show a mutation.

**And one false POSITIVE class.** `#[cfg(test)]` round-trip PROOFS legitimately call the same functions
the detector watches for, so two genuinely real carriers — `semio@v1/cad → step` and
`semio@v1/drawing → svg` — were being reported as stubs. Test modules are now excluded before judging.
A gate must be wrong in neither direction: a false stub hides a usable carrier exactly as a missed stub
invents one.

All five are pinned by harness checks so they cannot drift back.

By format: `png` 18, `dwg` 13, `md` 13, `stl` 12, `obj` 12, `svg` 8, `gltf` 7, `pdf` 7, `csv` 6,
`las` 5, `ply` 5, `zip` 5, `dxf` 3, `step` 3, `ifc` 2, `docx` 2, and one each of `xlsx`, `gif`, `jpg`,
`tiff`, `bmp`.

## The consequence for sequencing

664 mutations sit behind subsets that already write a real carrier — for those the work is oracle
registration, fixtures and probes, the same three steps the `mesh` and `brep` pilots just went through.
The largest single block is `architect/program` at 266 mutations over real `xlsx` and `zip` exports.

The other 1047 are blocked on an exporter being written first, and no amount of library research moves
them. Registering an oracle against a stub would produce a green result standing on bytes the reader
never understood — which is exactly the failure the per-mutation requirement exists to prevent.

## Reconciliation — one authoritative denominator

Two counts in this ticket disagreed: a repo sweep said 290 mutations were reachable, a per-subset
worklist said ≈364. Neither was wrong; they counted different things, and the disagreement was worth
resolving rather than averaging.

* The sweep counted **mutation-leaf DIRECTORIES** across all 122 subsets with mutations, including
  subsets already manifested.
* The worklist counted **CATALOG KINDS** — `mutationCatalogs[].kinds`, the same source
  `test manifest` reads — for subsets NOT yet manifested.

Catalog kinds are authoritative, because that is what the manifest generator and the coverage matrix
both consume. Recounted on that basis:

| | subsets | mutation kinds |
| --- | ---: | ---: |
| Already manifested | 13 | 174 |
| **Reachable — a real carrier exists, no exporter needed** | **35** | **357** |
| Blocked on an exporter being written first | 97 | 1616 |
| **Total** | **145** | **2147** |

357 against the worklist's ≈364 is a seven-kind difference in how the eight in-progress subsets were
attributed. The two independent measurements agree.

**The denominator for this goal is 2147 mutation kinds, not 1711.** The directory-based figure
undercounts, and it is the one quoted in the earlier sections above; those sections are left as written
because the corrections they describe are still the real history of the gate.

Top of the reachable list, all needing only fixtures, probes and a manifest:
`gif@89a` 21 · `semio/document` 18 · `semio/drawing` 17 · `pdf@1.7` 16 · `semio/cad` 16 ·
`mathematical` 15 · `las@1.0` 15 · `semio/presentation` 15 · `draw` 14 · `bcf@2.1` 14.
