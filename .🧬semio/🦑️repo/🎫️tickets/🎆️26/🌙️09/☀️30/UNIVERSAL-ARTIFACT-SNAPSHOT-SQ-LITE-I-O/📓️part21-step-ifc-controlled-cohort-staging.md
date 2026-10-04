# Part21, STEP And IFC Controlled Cohort Staging

## Actual Authorities

The actual shared Part21 owner is `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21/🦀️.rs`, consumed through `semio_s_artifact_stdio_contract::part21`. No second shared provider or scalar authority was authored. Its nine explicit value variants are Ref, Str, Enum, Int, Real, List, Typed, Unset and Derived; Real owns `negative`, literal `coefficient`, u32 `scale` and optional i32 `exponent`. The shared manual Part21Value and Part21Instance Value codecs currently have no genuine controlled input/output overrides. The derived decimal/header/document codecs cannot make their recursive manual children controlled by themselves.

STEP AP214 and IFC4 own distinct binary64 snapshot models and their existing handwritten SQL schemas. Their native Text and Binary bodies are external Part21 physical files and restore canonical schema/header population through their actual ordinary decoders. Exact typed semantic SQLite therefore must be distinguished from that documented external normalization boundary. IFC2x3 owns a Part21Document and all sixteen optional EDM fields; its existing native logical Text/Binary codec is distinct from external IFC physical-file import/export. Its Binary form retains exact decimal coefficient/scale/exponent. A u32MAX scale must never be expanded via Display before controlled Text admission.

Shared typed Decimal currently accepts owned literal coefficient states even when they are not printable physical decimal syntax; constructor laws cover empty/non-digit coefficient fields without using Display or to_f64. SQL lexical validation and external grammar acceptance are separate obligations, not a reason to silently coerce owned fields.

## Neutral Inputs And Native Laws

Added shared language-neutral fixture and closed draft7 schema beneath Part21:

- `🧫️fixtures/🚦️sqlite-cohort/🔣️.json`
- `🧬️schema/🚦️sqlite-cohort/🔣️.json`

They retain all nine kinds, exact signed/unsigned boundaries as decimal strings, u32MAX scale, both i32 exponent bounds, optional exponent presence, leading-zero coefficients, owned nonphysical decimal fields, literal NUL/Unicode strings, long Unicode work, twelve exact declarations and the three real native envelope identities.

Added four shared Native laws at `🧪️tests/🚦️sqlite-cohort/🦀️.rs`, mounted only as tests by the existing Part21 unit module. They test controlled input/output of every variant and repeated/empty instance names; both directions expose real interior Unicode copying and cumulative refusal. Independent serde JSON checks every authored variant payload, decimal field/optional exponent, exact integer/reference word, nested item order, instance id and repeated/empty entity names against a separately handwritten serde constructor. Production controlled constructors remain absent pending Root's genuine runtime baseline.

Added two laws to each existing owner Native SQLite suite: positive genuine controlled native input/output must equal the actual ordinary external or logical carrier; interior DecodeNative/EncodeNative cancellation and tiny ownership limits must be honored. They compare semantic projections for decoded external carriers rather than assert that the wire retains mutable schema text. Existing full-owned typed SQL laws remain unchanged.

Exact registered selections:

- `@semio-tech/stdio-artifact-contract-rs:test quick --lib part21_cohort_`: four newly authored laws. Both launch catalogs register this existing command at 408.790.
- `@semio-tech/stdio-step-rs:test-snapshot-sqlite-native`: nine authored laws (seven existing plus two cohort).
- `@semio-tech/stdio-ifc-rs:test-snapshot-sqlite-native`: sixteen authored laws (nine IFC2x3, seven IFC4).

No native production hook was mounted. No Cargo command or Native assertion was executed by this agent. Four new/changed test files passed rustfmt parser-only verification; that does not establish type-checking or runtime behavior. Root owns the sole Native lane and must measure missing-owner refusals before implementation.

## Source Runtime Receipt

Two laws appended to the actual IFC2x3 Source suite consume the shared neutral fixture/schema. Ajv validates the fixture independently; Bun SQLite checks a genuine SQLite3 file, integrity, foreign keys, every nine-kind projection, exact decimal fields and reconstruction. The second exercises the actual long Unicode ownership frontier and keeps u32MAX scale compact throughout semantic SQLite.

First registered Source run: nine passed, one failed, 56 assertions. The new cancellation harness incorrectly assumed byte units, whereas the existing shared Source Unicode measuring frontier reports UTF-16 code units (16,384-unit checkpoints). The fixture has 60,000 code units/140,000 UTF-8 bytes. Correcting that explicit harness threshold preserved interior cancellation assertions and required no production change.

Fresh registered `@semio-tech/stdio-ifc-rs:test-snapshot-sqlite-source --skip-nx-cache`, explicit quick level: **10/10 passed, 59 assertions, 1.46s Bun tests, 16.5s uncached Nx**, covering both IFC standards. The route's four dependencies and actual owner command ran Bun generators/Source tests; no Cargo compilation or Native test was invoked. Logs are `🗑️generated/part21-ifc-cohort-source-baseline.log` and `part21-ifc-cohort-source-frontier-fixed.log`.

The owning strict public package gate has not been rerun for the new cohort in this scope; no new public package completion claim. STEP Source was unchanged and not rerun. Universal149 and every Native control obligation remain incomplete.

## Unmounted Implementation Plan

Shared canonical controlled Part21 Value construction/projection should be hand-authored once its direct four-law baseline executes: borrow literal tagged entries, explicitly admit each typed field/vector/text ownership, preserve ValueRefusalKind, guard partially completed children, retire recursive List/Typed trees iteratively, and bind instance id/entities without ordinary clone or conversion fallback. The shared contract owns these actual types, not STEP or IFC providers. Physical Part21 parsing/writing requires separately paid token/UTF-8/decimal-expansion work; a before/after wrapper around parse_part21/write_part21 is insufficient. STEP/IFC4 must explicitly map their real binary64 snapshots to that admitted physical grammar, while IFC2x3 must retain its own complete logical schema/document/EDM codec. No draft production stub, fictitious header, native byte SQL carrier, reflective schema inference or duplicate provider was created.

## Current Expanded Stage Readback

The later owned-draft stage supersedes the initial selector counts above: shared contract part21_cohort_ now8 staged laws, STEP9, IFC2x3 10 andIFC4 7 (IFCcombined17). The extra native IFC2x3 law establishes actual arbitrary coefficient owned acceptance before semantic SQL projection; production still has the stricter coefficient guard and no claim of passing that law is made. Controlled shared drafts are adjacent and unmounted. The fresh extended-fixture Source route passed10/10 laws59assertions (2.37s Bun/14.9s Nx), logged at generated/part21-ifc-cohort-expanded-controls-source.log. Full current field/order/allocation/cold-retirement findings and exact mounting obligations are recorded in `📓️part21-owned-native-draft-readiness.md`. Native remains unrun in this worker lane.
