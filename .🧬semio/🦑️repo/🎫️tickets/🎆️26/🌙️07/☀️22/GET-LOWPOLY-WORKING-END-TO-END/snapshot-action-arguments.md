# Snapshot Action Arguments

Read-only source audit plus authored neutral declaration projection regression, 2026-11-05. No production, Native, server or browser changes.

## Public Renderer Cannot Supply Missing JSON

Both actions are ActionDefinition entries, not CommandDefinition entries: actual committed editor has commands:[], and public app actions importSnapshotJson/replaceSnapshotJson have inPalette:true and args:[].

Renderer engine prefix `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`:

- ShellHelpers/window-panes/🟦️.tsx:191 resolves window actions, filters inPalette, and maps declared action.args through label/argument resolution. It does not invent arguments from Rust command schemas.
- Same owner line76 calls actionRequiresStagedForm. With[] it returnsfalse, so row click calls onExecute({controllerId,action:action.id}) with no args. This is the concrete public action palette path.
- ShellHelpers/window-panes/staged-args/🟦️.tsx:332 checks only existing visible argument declarations. An empty list cannot produce a JSON field or required-value gate.
- keyboard-chords/🟦️.ts:107 similarly fires an argless action directly.
- Global command-registry resolves separate AppDefinition.commands/mode commands and maps definition.args (line143); its command bar forms also depend on declared args (lines447–496). It cannot repair missing ActionDefinition metadata, and these Lowpoly actions are not public commands in that registry.

Lowpoly bridge folds value→json only if provided, then decodes required json:String payloads. There is no default JSON, no handler inference from prior document, and no renderer JSON input for the current[] declaration. Programmatic public dispatch with supplied json is a separate path and still faces the349,838vs16,384byte budget gap documented in snapshot-import-budget.md. Fixing argument metadata alone does not fix that admission gap.

## Bare / Typed / Emitted Declaration Boundary

Lowpoly manifest source declares mutations but never .action_args for either ID. The typed assembly stages previously reviewed stamp faults and cancellation; they do not synthesize argument definitions from command fields. Generated command/proof registration is an admission mechanism, not a renderer form declaration. Current emitted final descriptor independently corroborates both empty arg lists on actual editor ID s.lowpoly.lowpoly@1/*#editor.

No actual assembled Rust manifest was executed here. This report distinguishes deterministic source assembly review plus the committed final projection from a Native manifest test result.

## Actual Independent Descriptor Red

Authored small language-neutral inputs snapshot-arguments.json and strict snapshot-arguments.schema.json expect one required json argument for each public action, schema.kind=string, localized native label English “Snapshot JSON” / German “Snapshot-JSON”. Both actions remain explicitly public inPalette:true. The schema forbids extra projected fields, requires all fields/locales and exactly one JSON argument.

snapshot-arguments.mjs uses independent strict Ajv against expected fixture first, then projects real current committed descriptor fields by exact controller/action ID. It does not synthesize the observed expected argument or inspect source substrings. Missing/duplicate public action is a refusal. On invalid observed schema it retains diagnostics and sets exitCode1.

Executed:

```powershell
repo tools exec --light node -- './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️22/GET-LOWPOLY-WORKING-END-TO-END/snapshot-arguments.mjs'
```

Actual exit1. Expected fixture valid; observed actions both args:[]; Ajv failure /actions/0/args minItems1. This is genuine committed-descriptor projection red, not Native/action-handler/browser red. Root and Runtime were notified before any production declaration change.

The proper source correction, once coordinated with Runtime's snapshot transport/budget work, is an explicit required user-facing string argument whose ID is json and whose label includes both languages. Add a public assembled manifest regression and renderer staged-form behavior test alongside this projection oracle; then regenerate descriptor through its owner. Do not add defaults which submit a fake empty document merely to make an argless action fire.

## Natural Native IO Remains Absent After Typed Assembly

This conclusion is not based only on bare AppIo:
- ArtifactEditor default natural_file_codec in framework plugin declaration/surface/artifact-editor-trait/🦀️.rs:470 returnsNone.
- Focused Lowpoly plugin/composition Rust source contains no override. EditorApp forwarding delegates to E::natural_file_codec.
- Typed editor_surface (instance/lifted/tree/🦀️.rs:42–45) mounts natural-file actions only when that method returnsSome. native_editor_surface line89 has the same gate.
- Thus current Lowpoly typed assembly does not add a natural codec/action pair later. Static final descriptor has importFormats:[],exportFormats:[] and no paired natural Open/Save actions.
- Actual React naturalFileFormatV1 (ShellHelpers/transfer-checkin-tutorial/🟦️.ts:65) requires those paired actions plus matching format declarations and returnsnull without them.

Separate native/text/JSON/SQLite codec support does not imply a published natural-file editor action. Future generic host IO cannot count as a passing current Lowpoly alternative. No Native natural-file capability invocation is claimed.
