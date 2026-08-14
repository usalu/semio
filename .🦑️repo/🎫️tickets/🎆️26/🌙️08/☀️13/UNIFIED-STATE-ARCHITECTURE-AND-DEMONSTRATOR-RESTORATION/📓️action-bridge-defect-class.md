# The Action-Bridge Defect Class — Declared Actions That Reach No Command

## What the defect is

`ArtifactApp::command_from_action` has a **rejecting default**: it admits only the framework-reserved
ids (history/clipboard/revert/filter/noteShellCommand) and faults on everything else with
`app.command.unsupported`. Any app that declares its own actions and does not override it ships a UI
whose every app-owned control faults at dispatch. Nothing at compile time catches this — the default
makes the trait impl valid.

This is not theoretical. It is the same defect twice over:

- **📐️cad** — `setActiveExample` rejected, all four CAD 3D windows empty. The conversion existed only
  under `#[cfg(test)]`, so production had no bridge (fixed in `📓️cad-example-action-bridge-regression.md`).
- **🪵️sourcing curate** — `setActiveExample` rejected, the `demo-stock` example could not load and the
  Aussuchen pane sat empty. **Zero** `command_from_action` implementations for 15 declared command rows.

Console signature (verbatim, from the demonstrator on :6029):

```
[DEBUG] action failed setActiveExample {exampleId: demo-stock}
message: action 'setActiveExample' is not a framework-reser…ommand channel now (see `dispatch_typed_command`)
```

## The fix that landed here (sourcing)

`sourcing_curate_command_from_action` in `✏️s/🔌️plugins/🪵️sourcing/🎛️apps/🗂️curate/🦀️component.rs`
(region `🔖️Commands`), wired through the trait method. It joins two genuinely different vocabularies:
the manifest's camelCase **arg names** (`exampleId`, `moduleId`, `objectId`, `columnId`) and the
payloads' snake_case **fields** (`example_id`, `module_id`, …). Every field is read defensively and
coerced, because several `sourcing_action(..)` rows pass `None` args and the host fills them at
dispatch from the interaction itself (a text input's `value`, a slider's `delta`, a drop's `objectId`).

**Why not generate this in `app_commands!`** — the macro has both the action id and the payload type,
so a serde-based `from_action` looks tempting. It is wrong: bridging camelCase args onto snake_case
fields needs a key-mangling pass, and that pass corrupts any payload with a map-typed field, whose
keys are *data* (`"someId"` → `"some_id"`). The arg-name↔field-name join is genuinely app-specific
information; it belongs in the app.

Verification — `semio-s-plugin-sourcing --lib` **80 passed / 0 failed**, with three new laws:

| law | what it pins |
| --- | --- |
| `the_production_action_bridge_admits_every_declared_command` | every one of the 15 command rows converts, through `<SourcingCurateApp as ArtifactApp>` (the seam the host calls), not the free function |
| `the_action_bridge_reads_the_declared_arg_names` | `exampleId`→`example_id`, `moduleId`+`enabled`, and an undeclared action faults rather than silently no-opping |
| `every_rendered_action_bridges_through_the_framework_harness` | the framework's own `testkit::assert_declared_actions_bridge_to_commands` |

## The framework already has the law — most apps never invoke it

`semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<A>(manifest)`
(`🔌️plugin/🦀️component.rs:3065`) walks the actions the app's **window kinds actually render**, stages
each one's declared args exactly as the host does, skips the 23 framework-injected ids, and asserts
both that the action bridges and that `command_id` round-trips. It is strictly stronger than
enumerating command rows: it also catches chrome declaring an action that no command row backs.

Measured across `✏️s` after the sourcing fix:

```
apps with a command_from_action override        16
apps invoking the framework conformance law     13
apps declaring command rows with NO override    34   ← 442 declared rows with no production bridge
```

### The 34 unbridged apps (declared command rows, descending)

```
47  💠️lowpoly/💠️lowpoly     38  🎥️shooting        36  🗒️note         35  🌊️flow
27  📋️forms                 26  🖍️draw            20  📏️layout       20  ✒️writer
19  🏗️fem/◻2d               18  🏗️fem/🧊️3d        18  🎞️animate      17  🎬️sequence
16  🖨️raster                13  🕸️dag             11  📜️imperative   10  💡️reasoning/🔌️wires
10  🌿️vcs                    9  📖️playbook         7  ➗️mathematical
 3  📕️norm × 15 apps (din18599, en1990-1999, din16798, din4108, vdi3805, iso16757)
```

**None of these is a demonstrator pane** — all six panes are bridged (cad, sourcing, puzzle 3d,
process 3d, gis 2d, procedural 3d), so this does not block the demonstrator. The count is a latent
UI-dead-on-arrival surface everywhere else.

Caveat on the number: 442 counts *declared command rows*, which is an upper bound on the damage. An
app whose rows are all driven through `dispatch_typed_command` and never rendered as chrome actions
needs no bridge. Only the framework law can settle it per app — which is exactly why the remedy below
leads with the law rather than with the implementation.

## Remedy (per app, in this order)

1. Add `assert_declared_actions_bridge_to_commands::<TheApp>(create_the_app)` to the app's existing
   test module. This *measures* rather than assumes: an app that needs no bridge goes green immediately.
2. For each app the law fails, implement `command_from_action` by reading the manifest's declared arg
   names — `📐️cad`'s `cad_command_from_action` and sourcing's twin are the two worked references.
3. Consider a policy rule (`📜️script.ts`) that flags an `impl ArtifactApp` which declares command rows
   and neither overrides `command_from_action` nor invokes the law — this class is grep-detectable and
   would have caught both the cad and the sourcing instance before they reached a browser.

Step 3 is the durable fix: the rejecting default is correct behaviour, but nothing today forces an app
to notice it applies to them.
