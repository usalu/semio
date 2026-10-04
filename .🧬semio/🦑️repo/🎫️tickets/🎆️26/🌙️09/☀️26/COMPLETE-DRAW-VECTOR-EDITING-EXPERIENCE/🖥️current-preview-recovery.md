# Current Draw Preview Recovery

The Browser skill connected successfully to the in-app browser. No existing tabs remained. Opening the previous stable Draw preview at `http://127.0.0.1:6065/` failed with `ERR_CONNECTION_REFUSED`; this is no longer a live preview. A listener check also found no server on the currently registered Draw launch port 6064.

The authoritative React launch entry is `🛠️dev🖍️draw🖍️drawing⚛️react`: `bun nx run workspace:dev -- draw`, with `S_OS_PORT=6064`, `SEMIO_PLUGIN=draw`, `SEMIO_RENDERER=react`, and `SEMIO_APP=s.draw.drawing@1/*#editor`.

The registered warm `draw served` route now runs on port 6064 in the framework's explicit `S_LOCAL_ONLY=1` mode with HMR disabled. This is supported local preview operation, not an authentication bypass. Native test handles are terminal; preview handle 31647 remains live. The server reports one staged component matching its source and activation receipt. No new Draw component was explicitly rebuilt or activated during this recovery.

The browser first showed Loading Plugins, then rendered the complete Demo canvas and artboard dimensions. Opening Utilities showed Direct Select with `aria-pressed=false`, despite Draw's implicit default tool; explicitly selecting it changed the accessible state to pressed and exposed selection method/mode controls. This mismatch remains a real UI defect.

After closing Utilities, clicking the painted orange area at viewport `(420,350)` eventually published **Orange Wedge** into the inspector. Name, actual orange fill, stroke width, affine controls, and path nodes were present. Captured browser warning/error logs were empty for this interaction. Selection publication is asynchronous; the immediate snapshot still showed the empty inspector, and the later snapshot showed the completed selection. Screenshot: [current Orange Wedge selection](🗑️generated/current-preview-orange-selection.png).

This confirms current painted-path selection in the existing mounted editor. It does not establish scheduled Boolean/trace canvas production, full native renderer parity, collaboration, exports, or complete end-user acceptance. Unrelated lazy plugin prefetch tasks reported materialization failures in the server log; the Draw canvas and this selection nevertheless completed. Preserve the live preview and tab for continued integration work.
