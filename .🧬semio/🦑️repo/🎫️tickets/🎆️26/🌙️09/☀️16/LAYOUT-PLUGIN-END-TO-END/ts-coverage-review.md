# TypeScript Coverage Review

2026-10-28. Bounded source-only scan, excluding node_modules/generated/build trees and declaration files. No runtime, build, test or graph executed. Followed relative literal imports through both .ts and .mjs; did not count fixture/helper files as executable suites.

The runner declares16Bun entry files:15preexisting Bun suites plus behavior.test.ts. All discovered Bun suites are included. The aggregate explicitly registers14portable calls. Following imported portable runners (including .mjs oracle bridges) still leaves20behavior test files and21exported checks unreachable from the declared target. Their bodies contain real fixture/oracle assertions and cancellation/ownership laws, so they are not merely helper/type files. Existing53passing tests therefore establish the selected runner, not complete portable layout behavior coverage.

## Exact Declared Bun Entries

- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/📄️paragraph/🧪️tests/🟦️.ts
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🔤️text-fonts/🧪️tests/🟦️.ts
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🖼️frame-planning/🧪️tests/🟦️.ts
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🎨️fidelity/🟦️.ts
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/📄️large-sheet/🟦️.ts
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🟦️.ts
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖼️canvas/🧵️paragraph-work/🧪️tests/🟦️.ts
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/👁️layer-order/🟦️.ts
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔣️transport/🟦️.ts
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/✅️validation/🟦️.ts
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🏗️structure/🟦️.ts
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🕸️graph/🟦️.ts
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🧪️frame-selection/🟦️.ts
- 🧪️tests/🔬️typescript-package-commands/behavior.test.ts

## Aggregate Calls

- runLayoutDrawingSegmentTests
- runLayoutDrawingExportTests
- verifyPageDrawing
- runLayoutFlowProjectionTests
- runLayoutTextFlowTests
- runLayoutExportAffineTests
- runLayoutPdfStreamTests
- runLayoutCanonicalTests
- testLayoutDrawingComposition
- verifyMarkedTextScope
- verifyParentCreationOutput
- testLayoutDiffAlgebraOracle
- testPageCache
- testLayoutWindowOwnershipOracle

## Unreachable Behavior Entries

- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/🧭️source-plan/🧪️tests/🟦️.ts: verifyImportSourcePlan
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📐️step/📐️geometry/🧪️tests/🟦️.ts: verifyLayoutStepGeometry
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📐️step/🖍️background/📐️geometry/🧪️tests/🟦️.ts: verifyBackgroundGeometry
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/🪆️drawing/🧪️tests/🟦️.ts: verifyDrawingLift
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/🪆️drawing/🌱️birth/🧪️tests/🟦️.ts: verifyDrawingBirthPreparation
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📦️idml/🔤️text/🧪️tests/🟦️.ts: verifyIdmlText
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📦️idml/📎️overrides/🧪️tests/🟦️.ts: verifyIdmlOverrides
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📕️pdf/🖍️paths/🧪️tests/🟦️.ts: verifyPdfPaths
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪆️drawing/🌱️birth/📤️transfer/🧪️tests/🟦️.ts: testDrawingBirthTransfer
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪆️drawing/🌱️birth/🧪️tests/🟦️.ts: testLayoutDrawingBirths
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️retained/🔎️target/🧪️tests/🟦️.ts: verifyLayoutTargetLookup
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/🪢️chained-plan/🧪️tests/🟦️.ts: verifyChainedPlan
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/🚚️list-move/🧪️tests/🟦️.ts: verifyListMove
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/🪡️apply/🧪️tests/🟦️.ts: verifyNestedRetainedWitnesses
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/📋️keyed-plan/🧪️tests/🟦️.ts: verifyKeyedPlan
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/📍️positional-plan/🧪️tests/🟦️.ts: verifyPositionalPlan
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/🪢️positional-chain/🧪️tests/🟦️.ts: verifyPositionalChain
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/🪡️setters/🧪️tests/🟦️.ts: verifySetters
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🧵️drawing/🧪️tests/🧵️graph.ts: testLayoutDrawingGraph, testPaidDrawingGraph
- 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🗃️plan/🧪️tests/🟦️.ts: testExportPlan

Static import reachability is not itself runtime success. Helpers exported as runTestScript/runOwnedVerifyScript/runMediaAdoptionSourceScript are infrastructure commands, not omitted behavior suites. The prior53pass result remains the execution evidence, not this read-only audit.

## Required Coverage Repair

Add each unreachable behavior export to an owned aggregate call with its independent oracle implementation; do not merely add bare files to Bun test operands because these exports do not register Bun tests themselves. There are existing oracle.mjs siblings for many of these interfaces; retain third-party oracle boundaries and declared runtime source entries. Re-run strict compilation and the complete expanded suite after integration. The import graph here follows literal relative imports in .ts/.mjs sources; it does not treat dynamic strings or external package imports as test execution.

## Exact Import and Argument Handoff

No listed export has an indirect relative-import caller reachable from the current16Bun entries, including through the eight aggregate runner modules and their .mjs oracle bridges. Each listed function has a concrete behavior assertion body; infrastructure runners and fixtures have been excluded.

### 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/🧭️source-plan/🧪️tests/🟦️.ts

Import from `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/🧭️source-plan/🧪️tests/🟦️.ts`.

- `verifyImportSourcePlan(oracle:SourcePlanOracle)`


Existing independent oracle files: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/🧭️source-plan/🧪️tests/oracles.mjs`.

### 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📐️step/📐️geometry/🧪️tests/🟦️.ts

Import from `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📐️step/📐️geometry/🧪️tests/🟦️.ts`.

- `verifyLayoutStepGeometry(oracle:FrameGeometryOracle)`


Existing independent oracle files: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📐️step/📐️geometry/🧪️tests/oracle.mjs`.

### 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📐️step/🖍️background/📐️geometry/🧪️tests/🟦️.ts

Import from `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📐️step/🖍️background/📐️geometry/🧪️tests/🟦️.ts`.

- `verifyBackgroundGeometry(oracle:BackgroundGeometryOracle)`


Existing independent oracle files: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📐️step/🖍️background/📐️geometry/🧪️tests/oracle.mjs`.

### 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/🪆️drawing/🧪️tests/🟦️.ts

Import from `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/🪆️drawing/🧪️tests/🟦️.ts`.

- `verifyDrawingLift(oracle:LiftOracle)`


Existing independent oracle files: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/🪆️drawing/🧪️tests/oracles.mjs`.

### 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/🪆️drawing/🌱️birth/🧪️tests/🟦️.ts

Import from `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/🪆️drawing/🌱️birth/🧪️tests/🟦️.ts`.

- `verifyDrawingBirthPreparation(oracle:BirthOracle)`


Existing independent oracle files: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/🪆️drawing/🌱️birth/🧪️tests/oracles.mjs`.

### 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📦️idml/🔤️text/🧪️tests/🟦️.ts

Import from `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📦️idml/🔤️text/🧪️tests/🟦️.ts`.

- `verifyIdmlText(oracle: TextXmlOracle)`


Existing independent oracle files: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📦️idml/🔤️text/🧪️tests/oracle.mjs`.

### 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📦️idml/📎️overrides/🧪️tests/🟦️.ts

Import from `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📦️idml/📎️overrides/🧪️tests/🟦️.ts`.

- `verifyIdmlOverrides(oracle: OverrideXmlOracle)`


Existing independent oracle files: none in the immediate test directory; construct the required interface from existing independent library witnesses.

### 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📕️pdf/🖍️paths/🧪️tests/🟦️.ts

Import from `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📕️pdf/🖍️paths/🧪️tests/🟦️.ts`.

- `verifyPdfPaths(oracle:PdfPathOracle)`


Existing independent oracle files: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📕️pdf/🖍️paths/🧪️tests/oracle.mjs`.

### 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪆️drawing/🌱️birth/📤️transfer/🧪️tests/🟦️.ts

Import from `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪆️drawing/🌱️birth/📤️transfer/🧪️tests/🟦️.ts`.

- `testDrawingBirthTransfer(equal:(a:unknown,b:unknown)=>boolean)`


Existing independent oracle files: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪆️drawing/🌱️birth/📤️transfer/🧪️tests/🔮️oracle.mjs`.

### 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪆️drawing/🌱️birth/🧪️tests/🟦️.ts

Import from `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪆️drawing/🌱️birth/🧪️tests/🟦️.ts`.

- `testLayoutDrawingBirths(equal:(left:unknown,right:unknown)=>boolean)`


Existing independent oracle files: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🪆️drawing/🌱️birth/🧪️tests/🔮️oracle.mjs`.

### 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️retained/🔎️target/🧪️tests/🟦️.ts

Import from `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️retained/🔎️target/🧪️tests/🟦️.ts`.

- `verifyLayoutTargetLookup(oracle:TargetOracle)`


Existing independent oracle files: none in the immediate test directory; construct the required interface from existing independent library witnesses.

### 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/🪢️chained-plan/🧪️tests/🟦️.ts

Import from `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/🪢️chained-plan/🧪️tests/🟦️.ts`.

- `verifyChainedPlan(oracle:Oracle)`


Existing independent oracle files: none in the immediate test directory; construct the required interface from existing independent library witnesses.

### 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/🚚️list-move/🧪️tests/🟦️.ts

Import from `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/🚚️list-move/🧪️tests/🟦️.ts`.

- `verifyListMove(oracle:Oracle)`


Existing independent oracle files: none in the immediate test directory; construct the required interface from existing independent library witnesses.

### 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/🪡️apply/🧪️tests/🟦️.ts

Import from `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/🪡️apply/🧪️tests/🟦️.ts`.

- `verifyNestedRetainedWitnesses(oracle:Oracle)`


Existing independent oracle files: none in the immediate test directory; construct the required interface from existing independent library witnesses.

### 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/📋️keyed-plan/🧪️tests/🟦️.ts

Import from `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/📋️keyed-plan/🧪️tests/🟦️.ts`.

- `verifyKeyedPlan(oracle:Oracle)`


Existing independent oracle files: none in the immediate test directory; construct the required interface from existing independent library witnesses.

### 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/📍️positional-plan/🧪️tests/🟦️.ts

Import from `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/📍️positional-plan/🧪️tests/🟦️.ts`.

- `verifyPositionalPlan(oracle:Oracle)`


Existing independent oracle files: none in the immediate test directory; construct the required interface from existing independent library witnesses.

### 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/🪢️positional-chain/🧪️tests/🟦️.ts

Import from `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/🪢️positional-chain/🧪️tests/🟦️.ts`.

- `verifyPositionalChain(oracle:Oracle)`


Existing independent oracle files: none in the immediate test directory; construct the required interface from existing independent library witnesses.

### 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/🪡️setters/🧪️tests/🟦️.ts

Import from `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/🪡️setters/🧪️tests/🟦️.ts`.

- `verifySetters(oracle:Oracle)`


Existing independent oracle files: none in the immediate test directory; construct the required interface from existing independent library witnesses.

### 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🧵️drawing/🧪️tests/🧵️graph.ts

Import from `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🧵️drawing/🧪️tests/🧵️graph.ts`.

- `testLayoutDrawingGraph(oracle:{equal:(a:unknown,b:unknown)=>boolean;affine:(t:any)=>readonly number[];compose:(outer:readonly number[],inner:readonly number[])=>readonly number[]})`

- `testPaidDrawingGraph()`


Existing independent oracle files: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🧵️drawing/🧪️tests/🧵️graph-oracle.mjs`, `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🧵️drawing/🧪️tests/oracle.mjs`.

### 🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🗃️plan/🧪️tests/🟦️.ts

Import from `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🗃️plan/🧪️tests/🟦️.ts`.

- `testExportPlan(order:(items:typeof sheet.document.pages[0]['frames'],layers:typeof sheet.document.pages[0]['layers'])=>typeof sheet.document.pages[0]['frames'])`


Existing independent oracle files: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🗃️plan/🧪️tests/🔮️oracle.mjs`.

## Expanded Aggregate Follow-Up

Source-only recheck after TypeScript worker integration: all21previously omitted exports now execute through aggregate calls (testExportPlan via portable-oracles.runExportPlan, the others through imported calls); no audited export remains omitted. This supersedes the omission finding above for the current aggregate. Worker reports36aggregate checks and75complete-suite tests passing plus strict compile; those execution results belong to typescript.md, not this source-only recheck.
