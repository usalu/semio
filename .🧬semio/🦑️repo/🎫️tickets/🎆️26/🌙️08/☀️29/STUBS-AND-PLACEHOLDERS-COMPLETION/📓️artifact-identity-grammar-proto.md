# 📖️ Artifact Identity Fix — `.g4` / `.ebnf` / `.proto`

Scope: `🅰️component.g4`, `🔤️component.ebnf`, `🛰️component.proto` only, under `✏️s/🔌️plugins`,
excluding the real `🗄️stdio/🗿️artifacts/🔣️json` artifact. `.json`/`.graphql` were a sibling agent's
scope (see `📓️artifact-identity-json-graphql.md`); `.semio` (`component.grammar.semio` /
`component.protocol.semio`) were a separate prior ticket run (see `📓️semio-grammar-identity.md`,
266 files, already closed) and were **not** touched here.

## Defect confirmed

```
rg -l -e 'stdio\.json' -e 'stdio_json' -e 'Stdio_json' --glob '!node_modules/**' \
   -g '*.g4' -g '*.ebnf' -g '*.proto' "✏️s" | grep -v '🗄️stdio/🗿️artifacts/🔣️json'
```
→ 477 files (169 leaf directories: 156 with `.g4`+`.ebnf`+`.proto`, 13 proto-only). All were a
byte-identical copy of the real `stdio.json` artifact's generic text-envelope grammar, e.g. (before
fix) `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/…/📸️snapshot/📝️text/🅰️component.g4`:
```
grammar Stdio_json_snapshot;
DOCUMENT: 'schema' [ ]+ 'stdio.json' ;
```
— a DIN 18599 document would only be accepted if it declared itself `stdio.json`.

## Conventions derived (evidence)

Real `stdio.json` originals gave the base pattern
(`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧬️schema/💡️inferences/📝️text/`):
```
🅰️component.g4:    grammar Stdio_json_inference;
                    DOCUMENT: 'schema' [ ]+ 'stdio.json.inference' ;
🔤️component.ebnf:  (* ebnf stdio.json.inference *)
                    document = header, body ;
                    header = 'schema', space, 'stdio.json.inference', newline ;
🛰️component.proto: syntax = "proto3";
                    package semio.s.stdio.json.inference_text;
                    message Artifact { string schema = 1; bytes payload = 2; }
```
This shape is corroborated by an **already-fixed non-stdio precedent** in the same repo state,
`✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/`
(identical structure, slug `writer.writer.inference`), and by non-text sibling protos already fixed
elsewhere, e.g. `…/📙️din18599/…/📸️snapshot/🛰️component.proto` → `package semio.s.norm.din18599.snapshot;`.

Derived rules, applied uniformly:
- **slug** = the full 3-part dotted identity `<plugin>.<artifact>.<facet>`, read verbatim from the
  co-located `🟦️component.ts` docstring (`` /** …representation for `<slug>`. */ ``) — the sibling
  ts-fixing agent's output is the ground truth, and it already carries the exact facet word used
  (`snapshot`/`diff`/`mutations`; note `mutations` is plural but `inference` is singular — no
  guessing from folder names).
- **`.g4` grammar name** = Pascal_Snake(slug): dots → `_`, uppercase only the first character
  (`norm.din18599.snapshot` → `Norm_din18599_snapshot`), matching `Stdio_json_snapshot` from
  `stdio.json.snapshot`.
- **`.g4`/`.ebnf` header literal** = slug verbatim, in `DOCUMENT: 'schema' [ ]+ '<slug>' ;` /
  `header = 'schema', space, '<slug>', newline ;`.
- **`.proto` package**, text-envelope shape (`message Artifact { string schema=1; bytes payload=2; }`)
  = `semio.s.<plugin>.<artifact>.<facet>_text` (dots throughout, `_text` suffix only on the facet
  segment) — matches the fixed `writer.writer.inference_text` / `stdio.json.inference_text` precedent
  exactly.
- **13 proto-only outliers** (`forms`, `layout`, `playbook`, `draw`, `raster` — `🧬️schema/{snapshot,diff,mutations}/🛰️component.proto` directly, no `📝️text` sub-facet, no `.g4`/`.ebnf`, no `.semio`
  sibling): their `.ts` sibling is still a generic `JsonSnapshot`/`JsonDiff`/`JsonMutation` interface
  (no dotted-slug docstring — not yet identity-fixed by anyone), so the slug came from the artifact's
  own `🦀️component.rs` instead, e.g. `forms/🗿️artifacts/📋️forms/🦀️component.rs:22`:
  `artifact_kind: "s.forms.forms"` → plugin.artifact = `forms.forms` (verified for all 5: `layout.layout`,
  `playbook.playbook`, `draw.draw`, `raster.raster` — no `energy.model`/`space.home`-style divergence
  in this subset). Package fixed to `semio.s.<plugin>.<artifact>.<facet>` (singular facet: `snapshot`/
  `diff`/`mutation`, matching the din18599 non-text precedent) — **message body/name left untouched**
  (`JsonSnapshot { string schema=1; string value_wire=2; }` etc.), since only the package-level
  identity claim was in scope, not a schema redesign.

## Identity vs. reference discrimination

Only files claiming `stdio.json` **as their own identity** (grammar name, header terminal, package)
were touched. No file legitimately *referencing* the real `stdio.json` artifact as a dependency was
found in this file-type set — the real stdio/json protos that other artifacts' `.proto` files
`import`/reference always name it correctly (e.g. `semio.s_stdio_json.snapshot.JsonValue`); none of
the 477 candidates were of that shape, all were straight unedited copies of stdio.json's own
generic-envelope template with the literal `stdio.json`/`stdio_json` left in.

## Counts

| File type | Count |
|---|---|
| `🅰️component.g4` | 156 |
| `🔤️component.ebnf` | 156 |
| `🛰️component.proto` (text-envelope shape) | 152 |
| `🛰️component.proto` (13 outlier dirs, package-only fix) | 13 |
| **Total** | **477** |

`git status --porcelain` over exactly this 477-file list: **477** (confirms no scope creep; the
~1600-file repo-wide status is other concurrent sessions).

## Verification (actual output)

1. **Re-grep, mandatory**: `rg -l -e 'stdio\.json' -e 'stdio_json' -e 'Stdio_json' -g '*.g4' -g '*.ebnf' -g '*.proto' "✏️s" | grep -v '🗄️stdio/🗿️artifacts/🔣️json'` → **0 hits** (exit code 1/empty). No `.g4`/`.ebnf`/`.proto` outside the real stdio-json artifact still claims that identity.

2. **Structural well-formedness** (script-checked all 477 files against the exact known-good shape —
   confirmed `.g4` grammar-name regex + header regex, `.ebnf` 3-line regex, `.proto` brace-balance +
   `syntax`/`package` line shape): **477/477 pass**, 0 malformed.

3. **Header-literal cross-check against the normative `📖️component.grammar.semio`** (the *runtime
   load-bearing* line, `header = "schema" SP "<literal>" NL`, not the cosmetic `grammar <name>` title
   line — the two are independent fields; only the header line is what the M5 recognizer actually
   parses against): **129/129 pass, 0 mismatches**, over every leaf where `.grammar.semio` has that
   line. The remaining 40 of 169 leaf dirs are explained, not silently skipped:
   - **13** have no `.g4`/`.semio` at all (the proto-only outlier dirs above; identity verified via `.rs` instead).
   - **27** have a `.grammar.semio` that has *already* moved past the generic template to a fully
     hand-crafted grammar for that facet (`mathematical`/`gisterrain`/`gismap`/`sequence` mutations,
     `lowpoly`/`forms`/`layout`/`playbook`/`draw`/`raster` snapshot or mutations,
     `imperative`/`trinity·rewrite`/`trinity·jack` mutations, `puzzle` 2d/3d/5d mutations, `space·home`
     mutations) — e.g. `forms/…/🚪️io/🧬️mutations/📝️text/📖️component.grammar.semio` is a real
     `mutation = artifact-mark canvas-op+ …` op-language grammar, not the generic `header/body/payload`
     envelope. My `.g4`/`.ebnf` there now carry the **correct identity** (slug from `.ts`, cross-checked
     against `.rs` for a handful, e.g. `fem.fem2d` confirmed via
     `🏗️fem/🗿️artifacts/◻2d/🦀️component.rs:15` `artifact_kind: "s.fem.fem2d"`) but are still
     structurally the generic opaque-envelope stub, **not** a transcription of the richer grammar.semio
     production rules. That divergence pre-dates this fix (the `.g4`/`.ebnf` were already the generic
     stub before I touched them, just with the wrong identity baked in) and transcribing ~27 hand-crafted
     PEG-style grammars into real ANTLR4/EBNF is a materially larger task than this ticket's stated
     mechanical identity fix — **flagged as unfinished, not attempted.**

4. **Repo test harness**: confirmed by code search (`rg 'include_str!.*component\.g4\|component\.ebnf'` →
   0 hits repo-wide; none of the 165 `.proto` files touched are `include_str!`'d by any co-located
   `.rs`) that **no cargo test in this repo actually exercises `.g4`/`.ebnf`/`.proto` files** — they are
   inert interop artifacts today. The repo's real conformance suite,
   `m5_handcrafted_grammar_conformance` / `m5_handcrafted_protocol_conformance`
   (`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs:1128,1208`), recognizes
   fixtures against `component.grammar.semio` / `component.protocol.semio` — files this ticket did not
   modify. Per the coordinator, a sibling agent is running that suite as the cross-check for the
   `.semio` identity fix; not duplicated here.
   I attempted `cargo test -p semio-s-plugin-norm din18599` (both the shared `target/` and an isolated
   `CARGO_TARGET_DIR`) as a general build-health sanity check; neither completed within available
   foreground time (up to ~1300s combined) — consistent with the known heavy concurrent-session load on
   this workspace's shared dependency graph, and expected for a cold isolated target dir regardless.
   **I did not get actual cargo output and am not claiming a compile/test pass** — stating this plainly
   rather than implying verification that didn't happen. Given point 4 above (no harness reads these
   file types), this gap does not bear on the correctness of the change itself.

## Honest summary

- 477/477 files fixed, identity-correct, well-formed, and — everywhere `.grammar.semio` has a
  comparable header line — in exact agreement with it (129/129).
- 27 leaves have a real, pre-existing structural gap between the (now correctly identified) generic
  `.g4`/`.ebnf` stub and a richer hand-crafted `.grammar.semio` grammar for the same facet — not
  introduced by this change, not fixed by this change, explicitly out of this ticket's mechanical
  scope, and worth a follow-up ticket.
- cargo test was attempted but did not finish in foreground time; not claimed as verified.
