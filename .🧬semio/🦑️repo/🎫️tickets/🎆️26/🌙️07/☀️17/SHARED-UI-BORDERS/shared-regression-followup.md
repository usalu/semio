# Shared Regression Follow-up

## Scope and Changes

Updated only the assigned shared-test files beneath `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/`:

- `📐️shell-components-4/🟦️.tsx`: three selected-stack silhouette expectations now require `active` before interaction. Inactive sibling expectations remain `normal`. Pre-existing zero-gutter/corner/padding edits were preserved.
- `🗂️shell-components-5/🟦️.tsx`: keyboard clipping fixture now uses a real controlled host (`reactHostPort.useState` and `onActiveWindowChange`). Retained clip/payload/gap/focus assertions; added both tabs' selected-state and active-silhouette assertions immediately after keyboard selection, before pointer interaction.
- `📚️shell-components-8/🟦️.tsx`: contained engagement Pane meets the host's top/left frame at zero.
- `🧱️shell-components-9/🟦️.tsx`: contained built-in panes, measures and utility overlays meet their host frame at zero; renamed inset test to describe shared frame. Panel safe-area defaults, collision displacement, rail hit testing and utility height clearance assertions unchanged.
- `🔎️shell-components-10/🟦️.tsx`: contained Pane zero anchor bases and responsive width/height expressions; middle-anchor centering, fold behavior and Panel tests unchanged. Renamed positioning test to describe shared frame.

Inspected supplied App activation and Pane geometry reports, relevant instructions, repository status, exact affected tests and current Pane/anchor flow before editing. No implementation, configuration, service, dependency or modifying Git operations.

## Validation

Accepted initial evidence: owner's relevant 718-test run reported eleven obsolete shared-suite failures. Reused that red evidence without repeating the broader run.

Ran the normal runner twice, including after strengthening keyboard assertions:

```text
/home/clstb/.local/share/semio/launcher/bin/repo tools exec --light node -- '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️17/SHARED-UI-BORDERS/run-ui-tests.mjs' '🧪️owned-locale-detector-retirement/🟦️.tsx'
```

Final result: **1 file passed; 565/565 tests passed**, 23.26 seconds. Reviewed scoped final Git diff; scoped `git diff --check` passed through the repo launcher.

## Handoff

No remaining blockers in this assignment. No assertions relaxed or removed. No new feature or runtime dependency was introduced; existing language-neutral acceptance and third-party numeric-oracle evidence remains in the supplied feature reports. Lead retains broader integration, generated-output cleanup and ticket closure ownership; shared ticket artifacts were not deleted.
