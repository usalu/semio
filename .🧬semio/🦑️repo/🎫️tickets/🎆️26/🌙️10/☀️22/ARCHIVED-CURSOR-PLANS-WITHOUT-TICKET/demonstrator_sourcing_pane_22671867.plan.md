---
name: demonstrator sourcing pane
overview: The Aussuchen (sourcing-curate) pane actually boots fine — a stale-ref race in the OS shell's boot effect dispatches a bogus "Keine Plugins geladen" error that then replaces the whole pane UI. Fix the boot effect to gate on the install's real outcome, fix the error/loading render precedence that hides genuine failures, and verify all six panes boot in the background.
todos:
  - id: ticket
    content: Read repo://goals and open/reopen the ticket for this work; use its folder for all probes and logs
    status: completed
  - id: repro
    content: Run the demonstrator dev server on 6029 and a headless probe that boots all six panes, capturing per-pane rendered text and console output
    status: completed
  - id: confirm
    content: Add temporary [DEBUG] logs to the boot effect (install outcome, loadedPluginsRef length, in-flight set) and confirm the false noPluginsLoaded on the Aussuchen pane
    status: completed
  - id: fix-boot
    content: Make installPlugin return its outcome and gate the noPluginsLoaded dispatch on that instead of the render-synced loadedPluginsRef
    status: completed
  - id: fix-render
    content: Swap the !session / error render precedence so real boot errors surface, and clear error when a session is established
    status: completed
  - id: verify
    content: "Re-run the probe: all six panes must show real app UI, Aussuchen showing its pool table; screenshot each and cross-check the standalone sourcing playground on 6081"
    status: completed
  - id: close
    content: Remove temporary [DEBUG] logs, re-verify, and close the ticket with summary and file list
    status: completed
isProject: false
---

created plan with name demonstrator sourcing pane, overview The Aussuchen (sourcing-curate) pane actually boots fine — a stale-ref race in the OS shell's boot effect dispatches a bogus "Keine Plugins geladen" error that then replaces the whole pane UI. Fix the boot effect to gate on the install's real outcome, fix the error/loading render precedence that hides genuine failures, and verify all six panes boot in the background., plan # Fix the Demonstrator's Aussuchen (Sourcing) Pane

## What the evidence says

All six demonstrator panes share **one** plugin (`demonstrator`, one wasm component bundling procedural3d/cad/puzzle3d/sourcing-curate/process3d/gis2d — see [Cargo.toml](✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust/Cargo.toml)), one module URL, and one refcounted worker lease. So a genuine plugin-load failure could not hit only Aussuchen while the other five work.

The decisive clue is the render order in [the React OS shell](🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/⚛️react/⚡️implementations/🟦️typescript/📦️index.tsx):

```9735:9741:🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/⚛️react/⚡️implementations/🟦️typescript/📦️index.tsx
    if (!session) return <p className="p-double text-sm text-muted-foreground">{shellLabel("ui.common.loadingPlugins")}</p>;
    if (error)
      return (
        <p className="p-double text-sm text-destructive" role="alert">
          {error}
        </p>
      );
```

"Keine Plugins geladen" is only reachable when a **session already exists**. The pane therefore did load the plugin, create the `sourcing-curate` instance, and establish its session — and then had its entire UI replaced by a false error.

## Root cause

The boot effect decides "no plugins loaded" by reading a ref that is only refreshed **during render**:

```6485:6494:🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/⚛️react/⚡️implementations/🟦️typescript/📦️index.tsx
  useEffect(() => {
    if (!primaryPluginId) return;
    if (loadedPluginsRef.current.some((entry) => entry.handle.pluginId === primaryPluginId)) return;
    void (async () => {
      await installPlugin(primaryPluginId);
      if (!loadedPluginsRef.current.some((entry) => entry.handle.pluginId === primaryPluginId)) {
        dispatch({ type: "SET_ERROR", value: shellLabel("ui.common.noPluginsLoaded") });
      }
    })();
  }, [primaryPluginId, installPlugin]);
```

`loadedPluginsRef.current = loadedPlugins` is assigned in the render body (line 5879), so it only reflects `installPlugin`'s `UPSERT_LOADED_PLUGIN` dispatch once React has committed a re-render. Immediately after `await installPlugin(...)` that commit may not have happened yet, so the check fires a false `noPluginsLoaded`. Two independent paths produce the same false positive:

- **Stale ref**: install succeeded, React hasn't re-rendered yet.
- **In-flight short-circuit**: `installPlugin` returns immediately when `pluginOpInFlightRef` already holds the id (line 6024), so a second effect run resolves instantly with the ref still empty.

Whether a given pane loses this race depends on how much async work its `establishPrimarySession` does before returning (worker round-trips for `createApp`, layout seeding) — which is why it lands consistently on one pane, and why six shells contending for one shared plugin worker makes it reproducible. Sourcing is the cheapest of the six (a table app, no 3D kernel replay), so it returns fastest and gives React the least room to commit.

Secondary bug, same area: because `!session` is checked before `error`, a *real* boot failure never shows its message — the pane sits on "Plugins werden geladen…" forever. That inverted precedence is why this class of bug is hard to diagnose.

## Fix

In [the React OS shell](🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/⚛️react/⚡️implementations/🟦️typescript/📦️index.tsx), inside the existing `🔌️PluginRuntime` region:

- Have `installPlugin` return its outcome (e.g. `"loaded" | "already-loaded" | "in-flight" | "failed"`) instead of `void`, derived from the values it already has locally (`handle`, the early-return branches). No new state.
- Boot effect gates the `noPluginsLoaded` dispatch on that returned outcome being `"failed"` — never on `loadedPluginsRef`.
- Swap the `!session` / `error` precedence so `error` wins, and give the error paragraph a `[DEBUG]`-free but distinguishable role for probing.
- Clear `error` when a session is successfully established (`establishPrimarySession` success path), so a late transient never sticks.

No demonstrator-side change is expected; [📦️index.tsx](♻️mit-bestand/🧺️demonstrator/📦️index.tsx) already mounts all six panes correctly via `useSequentialPaneBoot` (1500 ms idle queue, row-major order).

## Verification

Runtime only — no claim of "works" without console evidence.

1. Ticket first: read `repo://goals`, then `ticket_reopen`/`ticket_open`; all probes, logs and screenshots live in that ticket folder.
2. Add temporary `[DEBUG]` logs in the boot effect (install outcome, `loadedPluginsRef` length at check time, `pluginOpInFlightRef` contents) and confirm the false positive on the Aussuchen pane **before** fixing.
3. Run the demonstrator dev server (`MIT_BESTAND_DEMONSTRATOR_PORT=6029`) and drive a headless Playwright probe (`.mts` in the ticket folder, same shape as the prior ticket's `probe-landing-final.mts`) that waits out the full boot queue and dumps, per pane, the rendered text plus all console output.
4. Assert all six panes reach real app UI: Aussuchen must show its pool table ("Pool", "Glulam GL24h 200×400", module filter chips), not an error or "Plugins werden geladen…". Screenshot each.
5. Cross-check the standalone `sourcing` playground (port 6081) to confirm the app itself was never broken.
6. Remove the temporary `[DEBUG]` logs, re-run the probe, then `ticket_close` with the summary and file list.

## Note (out of scope unless it blocks probing)

The readiness beacon writes to `document.documentElement.dataset.semioOsReady` (line ~9892), which is page-global — six co-mounted shells overwrite each other's beacon. The probe will assert on rendered pane DOM instead of the beacon. Worth its own ticket., todos [{"id":"ticket","content":"Read repo://goals and open/reopen the ticket for this work; use its folder for all probes and logs"},{"id":"repro","content":"Run the demonstrator dev server on 6029 and a headless probe that boots all six panes, capturing per-pane rendered text and console output"},{"id":"confirm","content":"Add temporary [DEBUG] logs to the boot effect (install outcome, loadedPluginsRef length, in-flight set) and confirm the false noPluginsLoaded on the Aussuchen pane"},{"id":"fix-boot","content":"Make installPlugin return its outcome and gate the noPluginsLoaded dispatch on that instead of the render-synced loadedPluginsRef"},{"id":"fix-render","content":"Swap the !session / error render precedence so real boot errors surface, and clear error when a session is established"},{"id":"verify","content":"Re-run the probe: all six panes must show real app UI, Aussuchen showing its pool table; screenshot each and cross-check the standalone sourcing playground on 6081"},{"id":"close","content":"Remove temporary [DEBUG] logs, re-verify, and close the ticket with summary and file list"}]