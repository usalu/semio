# Norm TypeScript Contract and Host Audit

Date: 2026-11-04. Read-only source inspection; no compilation or full suite run.

## Confirmed Contract Failures

Under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`, the `⚙️insert-rail-derailment`, `⚙️insert-dust-explosion`, and `⚙️insert-rail-load-group` twins import `RailDerailment`, `DustExplosion`, and `RailLoadGroup` from the snapshot twin. None of those names exists in that source. All three readers instead call `parseRotatingMachine` for item. Their JSON schemas reference existing snapshot `$defs/RailDerailment`, `$defs/DustExplosion`, and `$defs/RailLoadGroup`; the generated snapshot twin does not preserve those schema-owned names. Inspect generation deduplication and restore the schema-owned type/reader contract; changing imports alone must also establish structural equivalence for runtime validation. Add independent Ajv and production-reader witnesses for all three payloads.

The EN1990 `🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts` projection imports six types absent from its snapshot source: `DistributionKind`, `TestDistribution`, `PermanentKind`, `StructureKind`, `SupervisionLevel`, and `InspectionLevel`. These are static module contract failures even though type-only imports are erased at runtime. Derive the owned literal types from actual snapshot property types or declare the correct schema-owned aliases.

The inspection scanned 3,314 TypeScript files and checked named relative import/export symbols against target source text, excluding star-export targets. These nine absent names were the complete result of that narrow check. This is not a TypeScript semantic compilation and cannot establish that other imports or assignment types are sound.

## Host Integration Observations

The generated dev plugin catalog registers norm as an isolated Rust component using `🌎️hub/🧩️compositions/📕️norm/🧩️component/📦️packages/🦀️rust`; that path exists. The component entry calls the framework plugin export macro with the interactive component policy, the composition plugin, and NormApps. Norm activation events include all fifteen computation and persisted artifact kinds. No missing host mount path was established by this inspection.

The generated playground registry assigns the same mixed list of examples to every norm variant: for example din16798 includes railway, masonry, concrete and VDI examples. This is a source-level catalog concern requiring a generator/source audit and route execution before calling it a proven runtime failure. The norm package manifests export built dist files; those files being absent in a source checkout is not itself evidence of a broken build.

## Follow-up AST Export Census

A later read-only scan used the installed TypeScript parser over 3,317 norm TypeScript files and compared each relative named import with explicit target declarations/reexports. It found 25 missing exported symbols across three sqlite projection files (the earlier EN1991 mutation and EN1990 projection issues had been concurrently repaired).

- EN1996 sqlite snapshot: `LoadCaseSituation`, `ImposedCategory`; the snapshot exports `En1996LoadCaseSituation` and `En1996ImposedCategory`.
- EN1991 sqlite snapshot: `FireOccupancy`; the snapshot exports `En1991FireOccupancy`.
- EN1997 sqlite snapshot: `SoilLayer`, `SpreadFoundation`, `FoundationLoadCase`, `Pile`, `PileTestProfile`, `RetainingWall`, `Slope`, `UpliftCase`, `Anchor`, `AnchorTest`, `EmbeddedWall`, `PileGroup`, `LabTest`, `LabPoint`, `LayerWater`, `Placement`, `CaseRow`, `CaseNumber`, `CaseText`, `Investigation`, `InvestigationReading`, `CharacteristicBasis`. Restore aliases from their actual exported schema-owned names in the projection rather than adding legacy names to generated schema contracts.

All paths above end in `🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts` under the named artifact owner. The AST census skips star-export targets, directory imports, external package aliases, and missing paths; it proves only this explicit export property.

The host composition source registers fifteen `.editor` calls and fifteen corresponding `.editor_mutation_roster` calls, with fifteen editor variants in its closed app fleet. Runtime admission/loading still requires compilation and route execution. No additional source-level missing mounted editor was established.
