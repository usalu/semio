# DS1 — `semio-s-plugin-stdio` descriptor vs. the 4 MiB descriptor contract bound

Slice DS1 (session 4). Owner: the descriptor contract between the plugin describe emitter, the hub's
trusted catalog and the OS directory schema.

Status: **(filling)** — sections are appended as each item lands, per preamble rule 12.

## 0. Headline

(filling)

## 1. Inherited state

C1b's `📓️c1-collaboration-e2e.md` §8 row 8 names this slice's blocker and points at a `§10.4` that
**was never written** — the worker died before writing it (confirmed against the committed copy of the
file at `48a8c69cdb`, which also stops at §10.3). There is therefore no measurement to inherit: the only
inherited evidence is `🗑️generated/c1b-trusted-bootstrap.txt`, whose last run died with
`fresh component exit at build (status=null, signal=SIGTERM)` — i.e. the run was cut by the session
outage, not by the descriptor bound. Everything in this report is measured by DS1 from scratch.

## 2. Where the bound lives

| # | constant | file | value |
|---|---|---|---|
| 1 | `FRESH_DESCRIPTOR_MAX_BYTES` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts:15` | 4 MiB |
| 2 | `TRUSTED_DESCRIPTOR_MAX_BYTES` | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:30` | 4 MiB |
| 3 | `DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES` | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1780` | 4 MiB |
| 4 | TS twin of 3 | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1440` | 4 MiB |

## 3. Measurement

(filling)

## 4. Decision

(filling)

## 5. Fix

(filling)

## 6. Proof

(filling)

## 7. Tests

(filling)

## 8. Files changed

(filling)

## 9. Honest gaps

(filling)
