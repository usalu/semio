# Shared UI Borders

## Delivered

- Shared coincident boundaries across tiled windows, bars, control groups, pane/tab chrome and menus. Partial overlaps resolve by segment rather than erasing whole component borders.
- Hover is the highest paint priority, independent of DOM order and parent/child ownership. Shared junctions remain connected and do not move when hover changes owners.
- Existing stepped silhouettes, chip cutouts, continuous active outlines, glass and transient effects remain; original duplicate effect paint is suppressed only after replacement ownership exists.
- Docked frames have no visual gutters/insets. Invisible centered resize regions, keyboard controls, dual-axis corner dragging and semantic control pointer ownership are retained.
- Context-menu fusion and viewport clamping update together; oversized keyboard-focused rows scroll into view. Popover/Select surfaces use shell-owned portals and preserve collision handling and explicit offsets.
- Marker-only bar/control roots work without windows, with shared lifecycle leases, cleanup and preserved forwarded refs.
- Native Dock/Shell helpers adopt matching ownership rules and zero visual sibling separation; detached floating geometry is preserved.

## Validation

See `validation.md` for exact evidence and reproduction inputs. Latest UI integration: 718 passing tests; lifecycle follow-up: 29 passing tests; real Chromium checks passed. Combined strict modified-source TypeScript check passes. Native dependency-free kernels: 11 Rust tests pass.

Full UI run: 1279 passing tests and one unchanged 20,000-node Diagram wall-clock timeout; that case passes isolated under an explicit longer diagnostic budget. Full native integration build remains blocked by repository native-source capture's path-length check, so GPU runtime parity is not claimed fully verified.

## Live Storybook Follow-up

The initial 6010 URL incorrectly reached a stale static server from another checkout. This branch now serves live Storybook at http://localhost:6011. Actual App inspection additionally exposed and fixed controlled active-outline propagation, contained Pane edge insets, and fractional-size normalization that prevented true ownership merging. Actual rendered App checks now verify both physical attachment and nonduplicated active/hover strokes; see storybook-followup.md.

## Reports and Lifecycle

`files.md` inventories modified/new source files and retained ticket inputs. Generated evidence/cache outputs were removed; only the running Storybook server log remains while that service is active. No AGENTS files, lockfiles, commits, staging, migrations or runtime dependencies were added. Native repo MCP is unavailable, so ticket registration/closure could not be performed; see `bootstrap.md`.
