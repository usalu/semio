# W0 — Derived Profile Subset Audit

Audited: 2026-08-12. Scope: **30 named stdio profile subsets** (non-`✳️any`, non-`🧿️semio` typed subsets) under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/**`.

Baseline context from `📓️plan.md`: ~30 profile subsets already have substantial Rust conformance; principal gap is **2–7 line TypeScript metadata stubs** and missing subset-owned examples / integrated roundtrips.

---

## Pattern summary

All 30 profiles follow the same **derived archetype** already implemented in Rust:

1. **`pub use …subsets::any::schema::*`** at schema root — same snapshot/diff/mutation types as owning `✳️any`.
2. **`//#region 🏗️DerivedConstruction`** — subset-specific `ArtifactBuilder` wrapping the any builder.
3. **`check_*_conformance(&snapshot) -> Vec<Diagnostic>`** — hard vs advisory severities; `build()` fails on Error/Fatal.
4. **Optional analyzer hooks** reporting profile-specific diagnostics (e.g. PDF/A level detection as data, not dialect id).

**TS mirror:** uniformly **6–7 LOC** stubs re-exporting any types — confirms plan gap; W3 `xml valid` reference must expand TS to real conformance metadata.

**Inferences:** **0/30** profile subsets have `💡️inferences/` today (all stdio inference dirs are on `✳️any`). Derived profiles need new inference slugs (e.g. conformance summary, validator outline) per subset contract — coordinate with IIF (pure-fn likely sufficient).

---

## Full profile matrix

| Artifact | Standard | Subset | RS LOC | TS LOC | `pub use any` | Conformance gate | `expect_err` in schema tests |
|---|---|---|---:|---:|---|---|---|
| zip | 2.0 | iso21320 | 338 | 6 | yes | yes | yes |
| pptx | ecma-376 | strict | 351 | 6 | yes | yes | yes |
| pptx | ecma-376 | transitional | 321 | 6 | yes | yes | yes |
| svg | 1.1 | basic | 388 | 6 | yes | yes | yes |
| svg | 1.1 | tiny | 340 | 6 | yes | yes | yes |
| ifc | 2x3 | cobie | 313 | 2 | yes | yes | yes |
| ifc | 2x3 | cv20 | 417 | 2 | yes | yes | yes |
| ifc | 2x3 | sav | 267 | 2 | yes | yes | yes |
| pdf | 1.4 | a | 186 | 6 | yes | yes | **no** |
| pdf | 1.4 | x | 189 | 6 | yes | yes | **no** |
| pdf | 1.7 | a | 639 | 6 | yes | yes | yes |
| pdf | 1.7 | e | 420 | 6 | yes | yes | yes |
| pdf | 1.7 | h | 339 | 6 | yes | yes | **no** |
| pdf | 1.7 | ua | 402 | 6 | yes | yes | yes |
| pdf | 1.7 | vt | 370 | 6 | yes | yes | yes |
| pdf | 1.7 | x | 488 | 6 | yes | yes | yes |
| step | ap214 | cc1 | 271 | 6 | yes | yes | yes |
| step | ap214 | cc2 | 269 | 6 | yes | yes | yes |
| step | ap214 | cc3 | 269 | 6 | yes | yes | yes |
| step | ap214 | cc4 | 269 | 6 | yes | yes | yes |
| step | ap214 | cc5 | 269 | 6 | yes | yes | yes |
| step | ap214 | cc6 | 268 | 6 | yes | yes | **no** |
| xlsx | ecma-376 | strict | 368 | 6 | yes | yes | yes |
| xlsx | ecma-376 | transitional | 352 | 6 | yes | yes | yes |
| docx | ecma-376 | strict | 442 | 6 | yes | yes | yes |
| docx | ecma-376 | transitional | 333 | 7 | yes | yes | yes |
| xml | 1.0 | valid | 294 | 6 | yes | yes | yes |
| jpg | jfif-1.01 | baseline | 323 | 6 | yes | yes | yes |
| json | rfc8259 | i-json | 383 | 6 | yes | yes | yes |
| tiff | 6.0 | baseline | 292 | 6 | yes | yes | **no** |

### Aggregates

| Metric | Value |
|---|---|
| Profiles audited | 30 |
| Derived pattern (`pub use any` + gate module) | **30/30 (100%)** |
| TS LOC ≤ 7 (stub) | **29/30** (docx transitional = 7) |
| RS LOC range | 186 – 639 (median ~320) |
| Schema tests with `expect_err` / negative build | **23/30 (77%)** |
| Missing in-schema negative tests | **7/30** — see gap list |
| `💡️inferences/` present | **0/30** |

---

## Negative test gap list

These profiles have conformance gates but **no** `expect_err` / explicit negative build test in `🧬️schema/🦀️component.rs`:

| Profile | Risk | W3/W4 action |
|---|---|---|
| pdf 1.4/a, 1.4/x | older/smaller schemas; may rely on any-level tests only | add derived negative build test + vendored negative example |
| pdf 1.7/h | gate exists (`check_h_conformance`) | add `expect_err` case (e.g. missing required PDF/H marker) |
| step ap214/cc6 | cc1–cc5 have tests; cc6 missing | copy cc1 negative pattern |
| tiff baseline | gate + diagnostics | add baseline violation case (e.g. non-baseline compression tag) |

**Note:** Negative testing today is **in-schema unit tests**, not separate negative example assets. Plan requires **both** by seal — assign negative examples in W3 reference (`xml valid`) and replicate in W4.

Representative negative pattern (xml valid):

```rust
#[test]
fn missing_doctype_fails_build() {
    let err = XmlValidBuilderConstruction::from_text("<root/>")
        .expect("parses")
        .build()
        .expect_err("a document without a doctype must fail build()");
    assert!(err.iter().any(|d| d.code.0 == "stdio.xml.valid.doctype-missing"));
}
```

---

## Per-family notes

### PDF (9 profiles)

- Richest derived implementations (up to 639 RS LOC on 1.7/a).
- Tier-1 semantics: same `PdfSnapshot` type; profile is validation stamp + `MigrateDialect`, not new schema id (see pdf/a doc comment).
- 1.4/a,x lack in-schema negative tests — prioritize in W4 batch.
- None have profile-level inferences; any-level pdf inference tests **failing** (IIF owned).

### STEP AP214 (cc1–cc6)

- Highly uniform (~268–271 RS LOC) — good candidate for macro/template in W4.
- cc6 missing negative test — likely oversight, not intentional.

### IFC 2x3 (cobie, cv20, sav)

- TS stubs only **2 LOC** (lowest in set).
- All three have conformance gates + negative tests in schema.

### Office ECMA-376 (docx, xlsx, pptx × strict/transitional)

- strict/transitional pair pattern consistent.
- transitional tests include cross-profile rejection (e.g. strict-namespace workbook fails transitional `build()`).

### SVG (basic, tiny)

- Full derived construction + analyzer integration (~340–388 RS LOC).
- Need subset-owned examples (currently artifact-level demo stubs may be empty — see asset audit).

### JSON i-json, XML valid, JPG/TIFF baseline, ZIP iso21320

- W3 reference candidate: **xml valid** (plan assigned).
- i-json and iso21320 have strong Rust gates; TS stubs only.

---

## Derived vs owning `✳️any` (IO and engine)

Each profile also has `🚪️io/🦀️component.rs` sibling (verified in tree listing). IO typically re-exports or gates any codecs with profile sniff/analyze hooks.

**Engine location:** still under `🏅️standards/<standard>/⚙️engine/` today — relocation to `🪆️subsets/<subset>/engine/` is W4+ per plan (blocked on UCAS stdio stability).

---

## Work ordering for this ticket

1. **W3 reference:** `xml valid` — complete TS mirror, inference slug, positive + negative example, 11-stage roundtrip output.
2. **W4 batch:** office strict/transitional group, pdf group, step cc*, ifc triple, svg pair, json/tiff/jpg/zip singles.
3. **Parallel constraint:** do not edit stdio glue until UCAS roster frozen + IIF clears 5 baseline failures.
4. **Policy:** W2 add `PolicyRuleDerivedProfileCompleteness` (medium) — require gate + negative test + non-stub TS + inference slug before high promotion in W6.

---

## Related non-profile stdio subsets (out of scope for this table)

44 total non-any stdio subsets include **14 `🧿️semio` v1 typed subsets** (animation, audio, brep, cad, document, drawing, flow, image, mesh, model, presentation, text, value, video) — owning archetype, not derived profiles. Those collide with UCAS roster (13→18) and DKM (brep/drawing/mesh). Track separately in facet census, not in this profile table.
