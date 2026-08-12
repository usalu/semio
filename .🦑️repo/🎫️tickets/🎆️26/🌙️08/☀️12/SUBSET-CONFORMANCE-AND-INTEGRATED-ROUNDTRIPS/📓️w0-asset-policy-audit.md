# W0 — Asset Policy Audit

Audited: 2026-08-12. Policy targets from `📓️plan.md`:

- **Preference:** ≤ 256 KiB per vendored example asset
- **Hard cap:** ≤ 2 MiB (reject or replace in W4/W5)
- Require license/provenance metadata; prefer CC0/public domain; record hashes; no runtime fetch

Method: scan `✏️s/🔌️plugins/**` for example/fixture binary assets (docx, tiff, dwg, pdf, gltf, gif, mp4, png, jpg, wav, epw, bcf, step, ifc, etc.) and inspect example registration sources for provenance fields.

---

## Executive summary

| Category | Count | Action |
|---|---|---|
| **Hard cap violations (> 2 MiB)** | 3 | replace or truncate urgently |
| **Preference violations (256 KiB – 2 MiB)** | 4 | replace or re-encode before W3 references |
| **Empty placeholder assets (0 bytes)** | 4+ | replace with minimal valid fixtures |
| **Acceptable (≤ 256 KiB)** | majority of native binaries | keep; add provenance metadata |
| **Dedicated LICENSE/PROVENANCE files in plugins** | **0 found** | add per-example provenance in `🦀️component.rs` + manifest |

Provenance today lives in **doc comments** and decode logic — not structured license fields or content hashes in example manifests.

---

## Heaviest assets (plugins tree)

| Size | Path | Notes |
|---:|---|---|
| **8.5 MiB** | `🗄️stdio/🎞️gif/…/89a/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` | serialized snapshot DSL — **hard cap violation** |
| **6.1 MiB** | `🗄️stdio/📄️pdf/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf` | real PDF 1.5 thesis — **hard cap violation** |
| **4.2 MiB** | `🗄️stdio/🎞️gif/📚️examples/💃️dancing/🖼️assets/🖼️dancing.gif` | real animated GIF89a — **hard cap violation** |
| **4.2 MiB** | `🗄️stdio/🎞️gif/…/89a/📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` | pack encoding of gif demo — **hard cap violation** |
| 331 KiB | `📐️cad/🧫️fixtures/🖼️concrete-forest-reference.png` | cad plugin fixture — preference violation |
| 278 KiB | `🗄️stdio/🧊️gltf/📚️examples/🌱️metabolism/🖼️assets/🧊️base.glb` | real glTF binary — preference violation |
| 145 KiB | `🗄️stdio/🖊️dwg/📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg` | acceptable under 256 KiB |
| 84 KiB | `🗄️stdio/🎥️mp4/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` | acceptable |
| 42 KiB | `🗄️stdio/🎥️mp4/…/example.mp4` | acceptable |
| 19 KiB | `🎞️animate/📦️packages/🦀️rust/partial_movie_files/*/000000.png` | build artifact debris — **delete** (not licensed examples) |

---

## Hard cap violations — recommendations

### 1. `bachelor-thesis.pdf` (6.1 MiB)

**Location:** `🗄️stdio/📄️pdf/📚️examples/🎓️bachelor-thesis/`

**Current provenance:** doc comment cites real PDF 1.5, ~6.3MB, decoded via 1.7 engine; no license statement.

**Recommendation:** **replace**

- Extract 1–3 representative pages into a new ≤ 200 KiB PDF (PDF 1.5 or 1.7) covering text, vector, embedded font subset.
- Record: author permission or public-domain source, SHA-256, creation tool.
- Keep example id `bachelor-thesis` only if license allows redistribution; otherwise rename to `textbook-excerpt-cc0` and update labels.

**Do not truncate binary in place** — breaks page/object graph tests unpredictably.

### 2. `dancing.gif` (4.2 MiB)

**Location:** `🗄️stdio/🎞️gif/📚️examples/💃️dancing/`

**Current provenance:** doc comment — 54 frames, 800×800, NETSCAPE2.0 loop; no license.

**Recommendation:** **replace**

- Re-encode to ≤ 256 KiB: reduce frames (e.g. 8–12), dimensions (320×320), optimize palette; preserve GIF89a animation + loop extension for codec tests.
- Prefer CC0 clip art or internally generated fixture.

### 3. GIF demo DSL/pack pair (~8.5 MiB / ~4.2 MiB)

**Location:** `🎞️gif/89a/📚️examples/🎬️demo/`

**Recommendation:** **regenerate after gif binary replaced**

- DSL/pack are derived encodings — should be regenerated from smaller native asset, not hand-edited.
- Until regen, exclude from subset roundtrip stage 1 (provenance/non-empty) in CI or mark example deprecated.

---

## Preference violations (256 KiB – 2 MiB)

| Asset | Size | Recommendation |
|---|---:|---|
| `cad/…/concrete-forest-reference.png` | 331 KiB | **keep with metadata** or lossless optimize to < 256 KiB — cad-owned, not stdio subset example |
| `gltf/…/base.glb` (metabolism) | 278 KiB | **keep** if metabolism example stays; add CC-BY or PD note in example source; optional mesh simplify to < 256 KiB |
| `dwg/…/architectural.dwg` | 145 KiB | **keep** — good architectural roundtrip candidate after FIX-STDIO-DWG lands |
| `mp4/demo` DSL | 84 KiB | **keep** |

---

## Empty / invalid placeholder assets (0 bytes)

Detected under stdio `🎬️demo` examples:

| Asset | Path |
|---|---|
| example.docx | `📜️docx/📚️examples/🎬️demo/🖼️assets/` |
| example.tiff | `🖼️tiff/…` |
| example.png | `📷️png/…` |
| example.pdf | `📄️pdf/…` (demo; separate from bachelor-thesis) |

**Recommendation:** **replace** each with minimal valid file:

| Format | Target size | Source strategy |
|---|---|---|
| docx | 5–20 KiB | single paragraph OOXML strict/transitional pair |
| tiff | 10–40 KiB | baseline uncompressed 32×32 RGB |
| png | 1–5 KiB | 1×1 or 16×16 RGBA |
| pdf | 10–30 KiB | single-page vector PDF 1.4 |

Empty files fail plan stage 1 ("verify provenance/non-empty content") and block W3 references (docx office, tiff binary archetype).

---

## Small acceptable native assets (keep)

| Asset | Size | Notes |
|---|---:|---|
| jpg demo | 801 B | minimal JFIF — good |
| gltf demo | 682 B | JSON glTF — good |
| dwg demo | 22 B | likely invalid stub — **replace** with minimal real DWG or drop demo |
| step demo | 239 B | minimal STEP — verify parses |
| ifc demo | 213 B | minimal IFC — verify parses |

---

## Animate plugin debris

**Path:** `🎞️animate/📦️packages/🦀️rust/partial_movie_files/**/000000.png` (~19 KiB × multiple hashes)

**Recommendation:** **delete** (not example assets; no license; pollutes repo). Not subset-owned — flag to animate plugin owner; exclude from subset ticket scope unless W5 structural cleanup expands.

---

## Provenance / license posture

| Finding | Detail |
|---|---|
| LICENSE files under `✏️s/🔌️plugins` | **none found** in audit scan |
| Example `🔣️component.json` manifests | not present on sampled pdf/gif examples — metadata in Rust only |
| bachelor-thesis | no redistribution license documented |
| dancing.gif | no attribution |
| metabolism glb | no license in example rs |

**Recommendation for W3+ example contract:**

Add to each example `🦀️component.rs` (or future manifest):

```rust
pub const PROVENANCE: &str = "CC0-1.0"; // or SPDX id
pub const SOURCE_URL: &str = "…";       // optional
pub const CONTENT_SHA256: &str = "…";   // of FIXTURE_BYTES
```

Enforce in W2 policy: `PolicyRuleExampleProvenance` (medium) — missing license or hash → breach.

---

## Format-specific guidance for W3 reference subsets

| W3 reference | Example strategy | Size target |
|---|---|---|
| TIFF binary | replace 0-byte demo; add baseline + non-baseline negative | ≤ 40 KiB each |
| DOCX office | strict + transitional minimal docs | ≤ 20 KiB each |
| CSV text | generate in-repo | < 4 KiB |
| EN 1990 norm | norm plugin fixture — verify separate | keep small |
| semio mesh | use existing small mesh fixture | ≤ 64 KiB |
| CAD any | cad fixtures — separate from stdio | ≤ 256 KiB |
| XML valid derived | hand-crafted valid + invalid pair | < 2 KiB each |

---

## DSL / pack sidecar files

Many stdio examples store large `🗣️example.dsl.semio` / `🎒️example.pack.semio` beside tiny native files. These are **derived** and should be:

1. Regenerated from native asset after resize (deterministic export).
2. Size-budgeted same as native (hard cap applies to what ships in repo).
3. Not manually edited in migration.

Top offenders beyond gif: gif pack 4.2 MiB, pdf bachelor pack (check regen after pdf replace).

---

## Action priority list

| Priority | Action | Owner wave |
|---|---|---|
| P0 | Replace 0-byte docx/tiff/png/pdf demo assets | W3 docx/tiff refs |
| P0 | Replace bachelor-thesis.pdf with licensed ≤ 200 KiB excerpt | W4 pdf subset migration |
| P0 | Replace dancing.gif + regen gif demo dsl/pack | W4 gif |
| P1 | Add PROVENANCE + SHA256 to all stdio examples | W2 policy + W4 |
| P1 | Optimize or document metabolism glb | W4 gltf |
| P2 | Remove animate partial_movie_files debris | out of scope — notify animate |
| P2 | cad png optimize optional | cad plugin |

---

## Verification command (repeat before seal)

```bash
find "✏️s/🔌️plugins" -type f \( -name "*.pdf" -o -name "*.gif" -o -name "*.docx" -o -name "*.tiff" -o -name "*.glb" -o -name "*.dwg" \) \
  -exec stat -f "%z %N" {} \; | sort -rn | head -20
```

Gate: no file > 2 MiB; ≤ 5 files > 256 KiB with documented exception in example PROVENANCE.
