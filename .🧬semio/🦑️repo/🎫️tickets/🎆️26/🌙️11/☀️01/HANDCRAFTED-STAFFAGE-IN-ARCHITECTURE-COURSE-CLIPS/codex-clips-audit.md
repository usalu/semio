# Course Staffage Coverage Audit

Inspected current source on 2026-11-05.

- Python course coverage test: `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🐍️course_staffage.py`.
- Shared language agnostic fixture exists at `🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🎓️course-coverage/🔣️.json`.
- Claimed Vitest twin `🧍️staffage/🧪️tests/🎓️course-coverage` is absent.
- Synthetic code coverage assertion fails independently of catalog state: fixture codes include asset-ambiguous and scale-denominator without corresponding invalid vectors; the assertion adds manifest-version but fixture codes omit it.
- Add an ambiguous catalog vector, representation denominator mismatch vector, and version vector; then implement TypeScript twin against the same fixture.
- Current validator also assumes thermal timeline is an array of objects and does not reject empty/unsorted timelines; transport stroke and trail numeric ranges are not validated. Add explicit schema vectors if these are intended authoring guarantees.

No production files edited by this audit. Test execution coordinated by root through repo tools exec uv and the authoring Python project.

## Current Manifest Data Check

Independent Node inspection loaded 152 catalog assets and all 97 current course casts. All casts resolve to an existing asset, an authored selected scale, nonempty geometry for their selected projection, and an existing named non-key pose or clip. No defects found in these checks. This does not establish appearance, dimensional clearance, source-reference completeness, or runtime animation quality.
