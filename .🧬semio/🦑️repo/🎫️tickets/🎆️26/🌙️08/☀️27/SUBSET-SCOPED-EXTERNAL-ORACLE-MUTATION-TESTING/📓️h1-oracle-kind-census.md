# 📓️ H1 — oracle-kind census (independent pass)

An independent read-only pass over all 170 contribution files, run concurrently with — and blind to —
the executor that applies `kind`/`engine`. Its value is as a cross-check, and it is recorded here with
its errors intact because two of them are instructive.

## What it found

**138 registered oracles.**

| Proposed classification | Count |
| --- | --- |
| `standards-reference-tool` | 69 |
| `cross-semio-implementation` | **57** |
| `third-party-library` | 12 |

## The number that matters

**57 of 138 registered oracles — 41% — are second implementations written inside this repository.**

Every one has `"package": ""` and a rationale saying so in as many words, e.g.
`writer-python-independent`: *"A second implementation of the `s.writer.writer` document and its four
typed mutations, in Python"*. Others in the same shape: `procedural-2d-python-independent`,
`semio-text-python-independent`, `semio-mesh-typescript-three-independent`.

Under Protocol v2 these are **supplemental, never qualifying**. Both halves read the same schemas, so
they catch transcription errors and cannot catch a misread specification. Every mutation whose only
"oracle" is one of these is, as of today, an un-oracled mutation — and after the registry upgrade
`oracleRequirementBreaches` says so on every contract run instead of leaving it to a reader's judgement.

That single figure is the honest scale of the program the plan describes: the framework and the pilot
are done, and roughly two fifths of the repository's registered evidence still needs a genuine
third-party reference behind it.

## Two errors in this pass, kept as guidance

**`standards-reference-tool` was over-applied (69 is far too many).** The pass assigned it to any
library whose rationale mentions ISO / ECMA / RFC or a format name — `ruststep`, `lopdf`, `image`,
`png`, `gif`, `zip`, `csv`, `calamine`, `quick-xml`, `html5ever`, `semver`, `clsx`. A crate that parses
PNG is a `third-party-library`; `standards-reference-tool` means tooling published BY a standards body
or its official implementation project as the conformance reference (a W3C validator, a NIST CAx
harness, veraPDF). If you cannot name the publishing body, it is a library. Both kinds are qualifying,
so no coverage number moved — but the taxonomy would have been wrong, and a wrong taxonomy is what
makes the next reader's judgement wrong.

**`engine.family` was set to the LANGUAGE (`rust`, `python`, `javascript`).** That carries no
independence information whatsoever, which is the field's entire purpose: two references sharing a
family are ONE oracle. The family is the kernel or parser — `opencascade`, `lopdf`, `image-rs`,
`stepcode-independent`, `libpng` — and only an in-repository second implementation gets `none`.

Both corrections were issued to the executing agent before it wrote anything; see
`📓️s2-oracle-registry-v2.md` for the applied classification and the resulting independence ledger.
