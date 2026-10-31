# Rehearsal PDF Transport

Runtime RED73818 authoritatively failed before browser execution. The rehearsal Vite build emitted its chunks, then the actual PDF plugin's closeBundle refused: Published assignment PDF transport required. This was not a browser pass.

The E2E rehearsal branch called generic buildViteArtifact without runtimeChannels and created a sibling compiler control. Its target also lacked both PDF producer prerequisites. Normal site build/dev already used the retained published-PDF wrapper.

The exact E2E build now runs inside withCourseAssignmentPdfsV1 with its real runtimeChannels and actual retained control. Awaited build completion includes Vite closeBundle; the later site-artifact reader and static server remain outside that completed PDF transport scope. test-e2e declares the same English/German producer targets as build/dev, so normal release configuration produces real release PDFs before rehearsal. No PDF bytes or authentication are fabricated, and no transport/control guard changes.

Files:

- course/🎭️e2e/🟦️.ts
- course/📦️packages/🟦️typescript/📋️project.json
- course/🏗️builder/🌐️vite/📦️assignments/🧪️tests/{🟦️.ts,🔣️.json,📋️schema.json}

Here course means 🎓️teaching/🏛️architecture/📚️course. The closed language-agnostic corpus names three direct consumers, both producers and all four dependent targets. A genuine issued-control law plus recursive SQLite checks lineage, TypeScript AST checks the actual three consumer bridges, Ajv checks the closed corpus, and SQLite independently checks dependency membership.

Focused RED was terminal1:1passed2failed36assertions (missing rehearsal wrapper and producer dependencies). The same focused command after the narrow fix was terminal0:3tests42assertions. Logs generated/rehearsal-pdf-red.log and generated/rehearsal-pdf-fixed.log. Command: repo tools exec --light bun -- test './🎓️teaching/🏛️architecture/📚️course/🏗️builder/🌐️vite/📦️assignments/🧪️tests/🟦️.ts'.

Independent review found no concrete defect and confirmed all9projects/16specs/BOTH topology scope unchanged. These focused checks prove Source/control wiring, not a real rehearsal artifact. The same full unfiltered release BOTH gate remains the required actual positive plugin-build/browser proof.

The root's next full run12931 stopped before target/browser on a different legitimate Source change: print:deps-tex, NodeNative TS expected6e022b4c… observed e01bc29a…. Seven prerequisites passed; no actual rehearsal PDF positive resulted. The candidate is frozen pending the backend C11 Source checkpoint/fresh SDK. No rollback, unchanged retry or guard relaxation followed. Managed68016 remains separately live.
