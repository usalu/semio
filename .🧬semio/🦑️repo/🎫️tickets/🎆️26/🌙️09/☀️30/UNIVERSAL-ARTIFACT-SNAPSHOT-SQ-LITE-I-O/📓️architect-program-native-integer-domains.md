# Architect Native Integer Domains

Read-only authoritative native type inventory. This does not generate SQL or infer bindings. Each unsigned64 domain needs a queryable exact unsigned decimal companion plus its signed64 word representation; native unsigned32 domains fit a range-checked INTEGER.

| Entity | Field | Native Type |
| --- | --- | --- |
| Equipment | lifecycle_years | Option<u32> |
| ScheduleRequirement | float_days | Option<u32> |
| ScheduleRequirement | contingency_days | Option<u32> |
| GrowthPlan | horizon_years | u32 |
| PriorityRecord | rank | Option<u32> |
| AnalysisRecord | duration_ms | Option<u64> |
| SearchFilter | use_count | u64 |
| Survey | response_count | u32 |
| TemplateRecord | usage_count | u64 |
| KnowledgeRecord | usage_count | u64 |
| BenchmarkRecord | sample_size | Option<u32> |
| BenchmarkRecord | collection_year | Option<u32> |

## 2026-10-02 07:17 UTC — Executed Unsigned32 Counterexamples

The schema-only eight-law invocation genuinely ran five passes and three failures,including the native-u32 admission counterexample. The expanded nine-law invocation genuinely ran five passes and four failures (1.99s),demonstrating both the formal JSON declaration and the actual typed Source parser accepted lifecycleYears=-1. Imported enum definitions caused one test-only strict cast error; the schema lookup operand now widens through unknown,retaining all assertions.

After these actual failures, both authoritative artifact/snapshot JSON schemas now hand-declare integer/minimum0/maximum4294967295 for the eight exact fields. Their six actual entity parser functions validate those exact properties through the existing integer guard with the same bounds. Equipment lifecycle_years now is a range-checked nullable INTEGER; its mistaken floating companions are removed. Source public and nine-laws fresh verification remains pending. Unbounded u64 canonical Source transport alignment remains a separate unfinished obligation.

## 2026-10-02 07:20 UTC — Corrected Unsigned32 Runtime Evidence

The fresh registered nine-law invocation actually ran seven passes and only the two existing missing complete-provider failures (1.55s). The exact eight formal integer declarations and all six actual typed entity parsers now pass minimum/maximum and all invalid fractional/negative/overflow cases against independent Ajv. Existing fixture equality and child identities remained verified. Strict public verification reports no current type errors. This does not prove unsigned64 Source coverage or complete SQLite directions.
## Full-Width Source Admission Baseline

The neutral language-agnostic corpus now spells eight unsigned64 frontier words as exact decimal strings and two overflowing/negative words. Four actual entity parsers are exercised with canonical bigint values in their native-width fields, and must refuse Number/String/fraction/nonfinite/Boolean coercions. These typed domains are independent from JSON transport parsing; a provider-specific alternate snapshot cannot replace canonical ProgramArtifact. The registered uncached source baseline is running before changing canonical parser admission. Complete JSON transport alignment and full typed provider remain separate work.

## Canonical Unsigned64 Admission Execution on 2026-10-02 at 08:11 UTC

The registered uncached Source law executed after the four canonical entity field types and exact bigint guards were mounted. All eight decimal-word boundaries through 18446744073709551615 survived admission; negative bigint, overflow bigint, Number, decimal string, fractional Number, Boolean, NaN and infinity were refused. The whole Source suite executed 10 laws: nine passed and the still-unimplemented semantic SQLite facade law failed. The independent strict Source consumer target completed without type errors. This is exact scalar admission evidence; the parent SQLite provider and explicit JSON transport remain unfinished. Native seven-law unsigned64 expansion has not run yet.

Command and compiler output are retained temporarily in `🗑️generated/root-architect-u64-canonical-admitted.log`; they will be removed when the whole ticket is finished.

## Native Unsigned64 and Child Projection Baseline Executed

The registered uncached native gate executed eight selected laws in Nextest, 1.647 seconds of assertions. Six passed, including all eight unsigned64 words across four actual persisted register entities, each through ordinary Binary and Text native codecs (32 whole parent cases, 64 directions). Three controlled EntityId metadata/input/output laws and the two original persisted parent laws also passed. Two genuine failures remain: missing full relational capability and independent local/target child identities rejected by the host's ChildRestoreProjection::InvalidReference. The 2096 other package laws were filtered by the owning selected target; this is not full-package validation.

After that actual counterexample the host projection removed only its local/target equality requirement. Exact target dialect and identity, local child uniqueness, declared slot kind, bounded strings, complete reference sets and singular slot cardinality remain enforced. A member match now compares the declared slot and literal target ArtifactRef. The Source twin/independent Ajv oracle was aligned to the same neutral fixture. Its unrelated static initial-child identity prerequisite failure remains unverified. A fresh nine-law native retry also includes core and first-five-register projection/reconstruction through a real SQLite file and independent Bun SQLite queries/foreign-key check.
