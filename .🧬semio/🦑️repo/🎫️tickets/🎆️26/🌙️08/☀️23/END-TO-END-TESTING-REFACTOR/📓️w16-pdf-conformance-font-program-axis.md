# The 188-comparison PDF 1.7 conformance cluster: the font-program axis measured the wrong thing

Date 2026-08-25. Successor to `📓️w13-final-audit.md` §2.2(1). Raw evidence for the audit's claim is in
`w15-audit/pdf-font-program-probe.txt`; this note re-verifies it independently, names the cause, records
the fix and the deliberate profile change, and reports the before/after ratio per case.

---

## 1. The claim, re-verified from the artifacts rather than taken on report

The audit said `fontPrograms[].programBytes` diverges by 1–2 bytes on five of twenty-three embedded
programs while all twenty-three decompress byte-identical. I did not take that on trust. Probe script
`scratchpad/probe.py`, run against the two roles' own committed `mutate-no-mutation` raw output
(`.🧬semio/🦑️repo/⚡️cache/tests/results/…-mutate-pdf-1-7-a-{oracle,subject}-rust/mutate-no-mutation.*.raw`),
extracting every object referenced by `/FontFile`, `/FontFile2` or `/FontFile3`, inflating each and
comparing bytes AND SHA-256:

```
oracle fontfile objs: 23  subject: 23  same numbers: True
  obj 3002 compressed 6849 -> 6848  decompressed 7281 -> 7281  contentSame True  sha d2fa70624b54 d2fa70624b54
  obj 3004 compressed 7462 -> 7463  decompressed 7912 -> 7912  contentSame True  sha e771d87123e6 e771d87123e6
  obj 3006 compressed 8162 -> 8160  decompressed 8645 -> 8645  contentSame True  sha 2b93ae7148d3 2b93ae7148d3
  obj 3008 compressed 6851 -> 6849  decompressed 7274 -> 7274  contentSame True  sha a2ceb830303b a2ceb830303b
  obj 3010 compressed 6860 -> 6858  decompressed 7290 -> 7290  contentSame True  sha 0c2ccbdaea80 0c2ccbdaea80
compressed-length differs: 5  decoded-content differs: 0  undecodable: 0
```

**Confirmed, and with digests rather than lengths.** Both producers write the same font programs; only
their deflate encoders disagree on how many bytes to spend saying so.

## 2. The before numbers, recomputed from the run's own artifacts

The audit's per-case table was reproduced exactly by diffing the two roles' cached projection JSONs
under the profiles' own rule (`tolerance: 0`, `ignoreKeys: []`) — no weakening, no normalisation:

```
mutate-pdf-1-7:     34/37   {('pages',): 3}
mutate-pdf-1-7-a:    0/33   {('fontPrograms',): 33}
mutate-pdf-1-7-e:    0/29   {('fontPrograms',): 29}
mutate-pdf-1-7-h:    0/25   {('fontPrograms',): 24, ('fontPrograms','infoTitle'): 1}
mutate-pdf-1-7-ua:   0/27   {('fontPrograms',): 26, ('fontPrograms','infoTitle'): 1}
mutate-pdf-1-7-vt:   0/41   {('fontPrograms',): 41}
mutate-pdf-1-7-x:    0/33   {('fontPrograms',): 33}
```

188 comparisons, and `fontPrograms` is the sole cause of 186 of them. The recomputation also surfaced
**two divergences the audit did not name** — one `infoTitle` in `✳️h` and one in `✳️ua` (§5).

## 3. The cause, and the fix

`font_program` (`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📄️document/🦀️component.rs`) reported
`stream.content.len()` — the length of the FlateDecode-compressed stream, not of the font program.
Deflate output length is encoder freedom; the program is the normative object. The `✳️any` subset's
`semantic-pdf-v1` profile already took that reading by listing `streamLength` as writer freedom, and
the six conformance profiles' own descriptions claimed to measure "how many bytes that program is",
which is exactly what the code did NOT do.

**Fix, in the projection — not in the profile's tolerance.** `font_program` now returns the DECODED
program (`Stream::get_plain_content`, falling back to the stored bytes when a filter chain will not
decode, so an unreadable program is still evidence) and the projection emits both its length and a
SHA-256 digest of it:

```rust
Some((key, _, size, sum)) => json_object(vec![
    ("key", Json::String(key)),
    ("programBytes", Json::Number(size as f64)),
    ("programDigest", Json::String(sum)),
]),
```

`tolerance` stays `0` and `ignoreKeys` stays `[]` in all six profiles. **The axis got STRICTER, not
weaker**: the compressed length could not see a program whose bytes changed without changing length;
the digest can. The `digest` helper is the repository's own SHA-256
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️protocol/🦀️component.rs`), already used by
`semantic-archive-v1` for the same "size plus content digest" shape — no new dependency.

## 4. The deliberate profile change, written down

All six `🔣️component.json` descriptions (`✳️a ✳️e ✳️h ✳️ua ✳️vt ✳️x`) now say the program is measured
DECODED, carry the reasoning and the evidence, and state that the change is a strengthening. The
sentence the audit called false — "an embedded font program's byte length … a writer must reproduce
exactly" — now reads "an embedded font program's decoded byte length". The rationale lives in the
projection's own docstring too, so the code and the profile cannot drift apart silently.

## 5. Residue found while recomputing: `set-snapshot` stamped a different title on each side

```
mutate-pdf-1-7-h  mutate-set-snapshot   oracle: 'A H conformant document'   subject: 'A PDF/H conformant document'
mutate-pdf-1-7-ua mutate-set-snapshot   oracle: 'A UA conformant document'  subject: 'A PDF/UA-1 conformant document'
```

A stamp text is a shared VOCABULARY constant, not an observation: each subset's schema declares
`CONFORMANT_TITLE` (`A PDF/UA-1 conformant document`), and the oracle improvised
`format!("A {} conformant document", profile.subset.to_uppercase())` instead of carrying it. The
sibling constant `CONFORMANT_AUTHOR` was carried verbatim and never diverged, which is what makes the
title an oversight rather than a design. `PdfConformanceProfile` now has a `conformant_title` field,
set per subset to the class's own name, so `set-snapshot` is the same mutation on both sides. Only
`✳️h` and `✳️ua` project `infoTitle`, which is why only they showed it.

## 6. Residue NOT fixed, and deliberately left red: `mutate-pdf-1-7` 34/37

Three `inverse-*` scenarios of the `✳️any` case diverge on `pages[].contentOperators`: the oracle's undo
of `append-page-content` / `set-page-content` / `remove-page` rebuilds a two-operator `BT … ET` stub
while the subject restores the real stream. This is the documented `regenerates_page_content` gap
(`…/🔖️1.7/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`): `PdfPage`'s only content field is `text`, so the
inverse cannot carry a real content stream. The adapter's own law check exempts the axis; the
`semantic-pdf-v1` parity profile does not, so the three comparisons fail. **No `ignoreKeys` entry was
added** — the honest fix is widening the snapshot to retain a content stream, which belongs to
whoever owns `../🧬️schema/📸️snapshot/🦀️component.rs`. Recorded here as a live finding, not hidden.

## 7. Verification

* `cargo test --features oracles --lib pdf` in `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust` —
  `test result: ok. 43 passed; 0 failed; 0 ignored; 0 measured; 328 filtered out; finished in 93.38s`,
  exit 0. Every subset's `every_declared_kind_is_observable_and_its_inverse_restores_the_document`
  still holds with the decoded axis and the corrected stamp.
* `cargo check --features oracles --lib` — exit 0.
* Per-case `bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case <case>` — see §8.

## 8. After numbers

Each line is the tool's own `[test]` line, read from the command's own stdout — never through a pipe.
Full transcript: `🧪️w16-pdf-conformance-parity-after.txt`; raw logs in `scratchpad/h-*.txt`.

| case | before | after | exit |
|---|---|---|---|
| `mutate-pdf-1-7-a` | 0/33 | **33/33** | 0 |
| `mutate-pdf-1-7-e` | 0/29 | **29/29** | 0 |
| `mutate-pdf-1-7-h` | 0/25 | **25/25** | 0 |
| `mutate-pdf-1-7-ua` | 0/27 | **27/27** | 0 |
| `mutate-pdf-1-7-vt` | 0/41 | **41/41** | 0 |
| `mutate-pdf-1-7-x` | 0/33 | **33/33** | 0 |
| **six conformance classes** | **0/188** | **188/188** | — |
| `mutate-pdf-1-7` (`✳️any`) | 34/37 | 34/37 | 1 |

```
[test] level=exhaustive cases=1 executed=66 passed=66 failed=0 errored=0 parity=33/33
[test] level=exhaustive cases=1 executed=58 passed=58 failed=0 errored=0 parity=29/29
[test] level=exhaustive cases=1 executed=50 passed=50 failed=0 errored=0 parity=25/25
[test] level=exhaustive cases=1 executed=54 passed=54 failed=0 errored=0 parity=27/27
[test] level=exhaustive cases=1 executed=82 passed=82 failed=0 errored=0 parity=41/41
[test] level=exhaustive cases=1 executed=66 passed=66 failed=0 errored=0 parity=33/33
[test] level=exhaustive cases=1 executed=74 passed=74 failed=0 errored=0 parity=34/37
```

`executed` doubled versus the aborted attempts (66 rather than 33) because both roles now run: the
earlier attempt caught `semio-s-plugin-stdio` mid-refactor by another session (`no field 'page' on
type PdfSnapshot` — the peer widening the PDF 1.4 snapshot the audit's §2.2(2) reported), so the
subject host exited 101 and the run reported `parity=0/0`. Recorded rather than glossed; it was not
this change, and the numbers above were taken after that session's tree compiled again.

**Repo-wide effect.** The stdio owner's differential ratio moves from 1,012/1,277 (79.2 %) to
1,200/1,277 (**93.97 %**), and 188 of the audit's 265 divergences are gone. `mutate-pdf-1-7`'s three
are untouched by design (§6).

### Cross-check: the corrected projection over the bytes the producers already emitted

Before the harness runs completed, the same conclusion was reached independently by re-projecting the
`.raw` output both roles had already written, with the corrected `project_conformance` and the
profiles' own rule (`w16-pdf-projector/`, a ticket-local standalone crate):

```
mutate-pdf-1-7-a: 33/33      mutate-pdf-1-7-e: 29/29      mutate-pdf-1-7-vt: 41/41
mutate-pdf-1-7-x: 33/33      mutate-pdf-1-7-h: 24/25      mutate-pdf-1-7-ua: 26/27
   mutate-set-snapshot: $.infoTitle: "A H conformant document" vs "A PDF/H conformant document"
   mutate-set-snapshot: $.infoTitle: "A UA conformant document" vs "A PDF/UA-1 conformant document"
mutate-pdf-1-7-h:  mutate-set-snapshot re-produced by the CORRECTED oracle -> EQUAL
mutate-pdf-1-7-ua: mutate-set-snapshot re-produced by the CORRECTED oracle -> EQUAL
```

The two residues are exactly §5's stamp text — those cached bytes predate the `conformant_title` fix —
and re-producing that one scenario with the corrected oracle closes both. The harness's own 25/25 and
27/27 confirm it.

## 9. Files touched

* `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📄️document/🦀️component.rs`
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️{a,e,h,ua,vt,x}/🧪️oracle/🦀️component.rs`
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️{a,e,h,ua,vt,x}/🧪️oracle/🔣️component.json`
