# Norm EN1990 Host Vertical Gate

Read-only current-source plan, 2026-11-04. No server, browser or build launched. Root probe established port 6094 is not serving.

## Authoritative Start Commands

The repo invocation router recognizes framework-os-dev prepare/activate/serve/dev/build targets of form <variant>-<react|wgpu>-<prototype|release>; it projects SEMIO_PLUGIN, SEMIO_RENDERER and SEMIO_BUILD_MODE. Dev also watches its matching activate target. The current Specific generated catalog declares en1990 -> norm, app s.norm.en1990@1/*#editor, React port 6094 and wgpu port 6194. These are generated variants rather than explicit framework-os-dev project.json target rows.

Use the fleet current session/shared capacity environment and explicit locale:

    SEMIO_LOCKED_LOCALE=en repo --service=off run @semio-tech/framework-os-dev:dev-en1990-react-prototype --catalog ✏️s/🧑‍💻dev/🧩️catalog/🤖️generated

To separate preparation and serving, use the same command with activate-en1990-react-prototype, then serve-en1990-react-prototype. Specific full-shell rebuild data separately declares activate-s-react-prototype with this catalog followed by verify-staged --variant s. That broader activation owns the full Specific shell; en1990 is the narrow declared playground.

## URL and Locale

Open http://127.0.0.1:6094/?plugin=en1990&role=editor&example=road-bridge-compliant . Current native example source declares ID road-bridge-compliant. React is already built for one variant; plugin is parity spelling, while role/example are parsed navigation axes. App comes from catalog default s.norm.en1990@1/*#editor or VITE_SEMIO_APP_ID server pin; an app query is not established as a React override.

SEMIO_LOCKED_LOCALE projects to VITE_SEMIO_LOCKED_LOCALE and boot locks.locale. The inspected boot-query contract has no locale query. For bilingual testing omit this server lock and use harness --locales en,de: it creates fresh contexts with navigator.language for each locale. An English-locked server cannot demonstrate German switching.

## Existing Browser Harness

Registered @semio-tech/framework-os-dev:time-travel invokes the plugin-neutral Playwright/Chromium history journey. It accepts --universal --variant --serve --renderer --locales --chords --out, reusing an answering server or activating/starting the declared variant itself. A ready existing-harness command is:

    repo --service=off run @semio-tech/framework-os-dev:time-travel -- --universal --variant en1990 --serve 'http://127.0.0.1:6094/?plugin=en1990&role=editor&example=road-bridge-compliant' --renderer react --locales en,de --chords en,de --out .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🗑️generated/host-e2e

The URL must remain one shell argument. Reports/verdict rows/screenshots are written; failed verdicts, page errors and hard guest faults fail acceptance. Console digest checks document restore failures. Universal controls are discovered from actual history editor DOM/mirror. Registered two-human/two-human-cross-undo provide broader shared-history journeys but require hub/users and do not substitute for a norm numerical witness.

No norm-owned Playwright or timeTravelScenarioV1 module was found by the current source scan. Existing universal history coverage therefore does not itself prove a norm field edit changes the correct engineering result.

## Required Norm-Specific Witness

1. Open declared EN1990 example and observe settled checks and outliner/model/properties/diagrams without bootstrap/restore faults.
2. Edit a real typed numeric field; record before/after document value and a dependent check quantity/utilization. History mutation alone does not prove recomputation.
3. Undo/redo through the shell and compare both document and check result; one user edit must be one undoable operation.
4. Reload/reopen the same artifact, compare persisted document/checks, and confirm ephemeral selection/view changes are excluded from document mutations.
5. Exercise EN/DE, keyboard focus/check stepping and read-only viewer selection/diagrams; viewer actions publish ephemeral local changes only.

Current evidence is source registration and harness availability. No render, numerical edit/result, undo or persistence acceptance pass is claimed. Native compilation remains separately owned by root.

## Current Source Basis

Repo build/invocation target matcher; playground/preferences locale projection; Specific generated catalog/playgrounds and rebuild chain; OS dev boot-query and boot locale/app defaults; OS dev time-travel project target, CLI, universal and console-digest; EN1990 road-bridge-compliant native source ID.
