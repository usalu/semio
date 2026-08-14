# CAD Example Action Bridge Regression

## Symptom

The demonstrator loaded all six shells, but CAD's selected example never reached the document. The
four CAD 3D windows consequently rendered empty scenes. Browser logs repeated:

```text
[DEBUG] action failed setActiveExample: action 'setActiveExample' is not a framework-reserved action
```

## Cause

The shell correctly sent the declared action through the typed-command host envelope. CAD's complete
action-to-`CadCommand` conversion existed only in its `#[cfg(test)]` test kit, however. Production
therefore inherited `ArtifactApp::command_from_action`, which rejects app-owned actions and reserves
that fallback for framework actions.

## Repair

- Promoted CAD's full declared-action conversion into production code.
- Connected `CadPlayApp::command_from_action` to that converter.
- Kept the existing test-kit API as a thin wrapper around the production converter so tests cannot
  drift onto a second implementation.
- Added a regression test proving `setActiveExample` becomes the typed `SetActiveExample` command and
  that unknown actions remain faults.

## Verification

- `SEMIO_TEST_BUDGET_MS=120000 bun nx run @semio-tech/cad-plugin:test --skip-nx-cache`
  completed with **141/141 passed** and one skipped test.
- In-browser reproduction before the repair confirmed the exact production rejection and visually
  confirmed that the four focused CAD 3D windows were empty.
- A post-repair demonstrator WASM build was attempted twice. Both attempts compiled the repaired CAD
  source without errors but were killed by the shared build budget after 20 and 40 minutes while a
  concurrent wasm-release build held and heavily contended the shared Cargo target directory. A third
  attempt was invalidated before Cargo by a concurrent registry regeneration temporarily removing the
  generated playground module. The staged browser component therefore remains pre-repair, and no
  post-repair visual claim is made here.

## Files

- `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🦀️component.rs`

