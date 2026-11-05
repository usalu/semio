# Bounded Stdio Prerequisite Audit

Read-only inspection through registered light Node; no source edits, builds, tests, dependency synchronization, graph execution, process changes or extra queue. Scope is the same missing normal ui-contract dependency pattern exposed by JPG schema-2. This is source evidence only, not native compilation proof.

## Scope And Method

Read the shooting artifact workspace declared stdio path dependencies, then traversed existing production path dependencies restricted to stdio directories. Sixteen declared stdio roots led to140 stdio path manifests and22 schema-named package owners. An initial path-only traversal without workspace declaration roots omitted inherited stdio dependencies and was discarded as insufficient. The final scope is anchored to the shooting workspace declarations; unrelated repo directories were not scanned.

For each schema owner lacking a normal semio-framework-ui-contract dependency, inspected its lib root and recursively referenced explicit path source mounts, excluding test paths. Checked actual mirrors and selected mounted UI sources rather than treating any source file below an artifact directory as compiled by every package. This is a bounded explicit-mount audit, not a Cargo resolver or full Rust parser; it does not claim exhaustive target/cfg/macro or implicit-module analysis.

## Findings

No additional conclusive missing-ui-contract blocker was found for batching. Twelve owners lack the direct dependency: bmp-schema, gif-schema and gif-schema-2, jpg-schema, json-schema and json-schema-2, semio-drawing-schema, semio-image-schema, svg-schema and svg-schema-2/schema-3, and txt-schema. Their inspected explicit non-test source closures contain no semio_framework_ui_contract reference. These absences therefore do not justify adding dependencies.

The JSON geojson mirror does mount editor windows from its base subset, but the inspected base main window uses the existing plugin-owned BuiltNode, TreeWindowKit and related UI reexports plus the direct locale dependency. It does not import ui-contract by external crate name. GIF base schema-2 mounts schema/IO/example source rather than the JPG pattern of unconditional editor main windows with direct ui-contract imports. Those are concrete reasons not to infer a missing dependency from UI-related folder names or from a higher package dependency.

The other ten schema owners already declare the direct dependency: repaired jpg-schema-2, five PDF schema owners, PNG schema and mutations-schema, and both TIFF schema owners. The current JPG fix has its separate ownership review in jpg-current-audit.md.

## Limits And Next Action

No source correction is recommended by this audit. Native should retain its single actual retry and use its authoritative compiler outcome to identify further prerequisites. Existing aliases/reexports and actual mounted scope must continue to be checked before any future dependency addition. This result neither proves native compilation nor excludes an unrelated compiler failure. No native or browser pass is asserted.

## Follow-Up After Actual TIFF Hidden-Handler Failure

After fresh27392 actual E0433, a separate bounded scan followed the16 shooting-declared stdio roots through140 current production path manifests. For inferences-named owners with explicit mirrors, inspected directly mounted main-window sources for ::paint_region::action calls, then compared their actual editor mirror scope. The only such direct reference found was the already confirmed TIFF document main window in semio-s-artifact-stdio-tiff-inferences. Its mirror mounts config/modes/windows without the higher typed editor, matching the compiler failure. No additional conclusive same-pattern inferences blocker was found. This targeted explicit-mount scan does not cover arbitrary generated or implicit Rust modules and is not compiler proof. React/native were notified; no dependency/handler refactor or extra execution was started.
