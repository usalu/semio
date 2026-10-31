# Deployment Fixture Contract Repair — 2026-10-29

Root98543 ran the actual complete course unit suite:809passed/11failed. Eight failures belonged to the deployment owner. The unchanged production checks correctly rejected six synthetic site cases missing the two required authored assignment PDFs. Two Docker tests correctly found that the language-neutral expected context listed only five of the current ten quizzes. These were stale test fixtures, not a reason to weaken runtime/artifact guards.

## Frozen Changes

- Publication fixture now declares the exact two assignment PDF paths, a finite blank PDF fixture, and present/missing/invalid document cases. Its new closed JSON schema precedes the test implementation.
- Shared released() synthetic CDN builder includes both declared PDF fixtures. Existing staging tests now hash/compare those bytes as well, using native crypto and the independent Noble SHA implementation. Source/staging separation, cancellation and immutable-artifact assertions remain.
- Sealed publication tests add six exact PDF-present/missing/invalid cases, validate fixture schema with Ajv, and compare the declared PDF roster with actual course assignment metadata. The existing intentionally incomplete site test keeps all previous missing-file expectations and adds the now-required missing PDF errors.
- Docker context fixture adds the five authored parametric, Grasshopper, Excel, BIM and optimization quiz JSON paths. Existing picomatch Dockerignore checks and exact courseImageContent roster assertions remain; no broad inclusion pattern replaces them.

Production deployment, artifact, runtime, Docker or image guards are unchanged. No budgets, assertion bodies, compiler Source caps, Original requirements or retained storage policies were relaxed. No server/peer was stopped.

## Baseline And Oracle

The real red baseline is course-unit-funded-pressure-fixed.log from root98543; it includes actual missing-PDF errors and the five-versus-ten context mismatch. The new corpus validates under strict Ajv. Retained ticket input deployment-pdf-oracle.cjs independently derives the same six problem lists through Ajv required-property and PDF-signature validation and exited0. This tests the signature/presence contract, not arbitrary semantic PDF validity or production assignment rendering. Generated oracle output/log remain in generated.

Canonical focused request53929 is live: repo run @teaching/architecture-course:test --configuration=prototype --parallel=1 -- quick 🧪️tests/🧪️deploy/🟦️.ts. It uses the normal target, SDK, control and Vitest owner. No result is claimed before its authoritative terminal status and actual test summary. Log course-deploy-fixtures-fixed.log.

## Scope Ledger

Changed production-tree files are all tests/data:
- course/🧫️fixtures/🌐️publication/🔣️.json
- course/🧫️fixtures/🌐️publication/🧬️schema/🔣️.json (new)
- course/🧫️fixtures/🐳️context/🔣️.json
- course/🧪️tests/🧪️deploy/🧰️deploy-fixtures/🟦️.ts
- course/🧪️tests/🧪️deploy/🌐️sealed-publication-and-bundle/🟦️.ts

The persistent canonical backend41024 remains owned and live. Its all-ten-quizzes/four-challenges real HTTP proof is separate; these synthetic deployment tests do not prove an actual site artifact, Docker image, offline bundle, public deployment or browser acceptance. Root owns the separate course data repairs and complete-suite rerun; no overlapping data/dashboard edits are made here.

## Authoritative Focused Result

Session53929 exited0. Actual Vitest executed the owning deployment file with41passed/0failed,1of1file; all six declared prerequisites passed and Nx reports target test success. This includes the new exact assignment-document law and all original staging/cancellation/context/budget/bundle assertions. Evidence course-deploy-fixtures-fixed.log lines1386–1387 and1405. No filtered-out deployment failure is claimed resolved from absence alone: the complete deployment owner file ran. The full course suite remains the coordinator's next gate after separate course-data/dashboard/preview work is ready. Server41024 remains owned/live.
