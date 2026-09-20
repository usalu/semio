# 📓️ Audit — Artifact / App / Extension Coverage of Play (session 4, 2026-09-20 01:55, Sonnet, read-only)

Ground truth from source: 34 plugin directories; 32 with a root `🔣️.json` (missing: `📖️playbook`, `🗄️stdio`). Registry:
34 plugin build targets, 26 extension targets, 1 host config (`space`: landing `home`, host `studio`), 68 playground
rows. Play: 60 catalog panes, 26 lanes in `📋️project.json`. Editor ⇄ viewer apps are 1:1 everywhere (exceptions:
demonstrator's four re-exported editors, space `studio`). No viewer-only dialect exists.

## Findings
- RED NOW: `playpanecoverage` "lists every playground app exactly once" — stdio's playground row (`variant = "stdio"`)
  is in the registry (61 expected variants) and its wasm links (242 MB `semio_s_plugin_stdio.wasm`), but there is no
  stdio pane in the catalog, no `prepare/activate-stdio-react-dev` lane in `📋️project.json`, no stdio `🔣️.json`.
- Only `stdio` lacks a pane / reachable editor apps. No artifact kind lacks a pane-reachable editor.
- Viewer apps: reachable through the ShellHost editor⇄viewer surface switch (`🏛️ShellHost/🔀️surface-switch/🟦️.ts`,
  `surfaceRoleAppsV1`); play's `FrameworkOsShell` mount does not suppress it. No play test asserts this.
- Extensions: all 26 reached through the `consumes`/`contributes` closure; imperative `consumes = ["imperative.module"]`
  is landed.
- Exclusions of the `s` host row and the six `entwerfen-mit-bestand-*` rows are justified. No catalog drift.
- Law gaps: the acceptance suite and "every plugin directory has a pane" stay green with stdio missing (only the
  variant-equality law catches it); nothing in play tests checks viewer reachability.
- Four descriptors (architect, dag, imperative, trinity-rewriting) predate a `setActiveExample` action already in
  their Rust → need an `activate-*-react-dev` re-run. `📋️design.md` "58 panes" is stale.

## Actions
1. play-stdio: descriptor + pane + lanes. 2. play-coverage: viewer-reachability law, strict descriptor gate, refresh
the four stale descriptors. 3. Coordinator: update design.md pane count at the end.
