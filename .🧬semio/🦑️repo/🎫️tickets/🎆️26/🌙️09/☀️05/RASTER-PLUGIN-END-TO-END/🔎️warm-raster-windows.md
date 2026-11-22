# Warm Raster Preview Windows and Examples

Chromium opened the existing local preview at port 6060 in a fresh context. The Settings language control reported English, and the page language was `en`. The real dock exposed exactly the two authored window IDs.

| Window | Visible canvas backing | Published layers | Asset census | Renderer alert |
| --- | --- | --- | --- | --- |
| Composite | 1152 × 939 pixels for both canvases | Backdrop with `semio-emblem`, Brighten | Empty | Image rendering failed: Layer image is missing |
| Navigator | 448 × 939 pixels | Backdrop with `semio-emblem`, Brighten | Empty | Image rendering failed: Layer image is missing |

Both windows mount and publish the example layer identities. Their image render is blocked on the known missing emblem asset in the warm guest. The logged browser diagnostic filter saw no additional page errors or GPU warnings in this run.

The English Example combobox opened via ArrowDown and presented exactly one accessible option, Demo. Demo was initially selected. Enter on that option retained Demo and both document witnesses; the run recorded one completed `setActiveExample` invocation with one effect. The proof cannot attribute that one invocation to re-selection because the shell also invokes Demo during boot. The initial pointer attempt timed out waiting for the combobox to stabilize, before the keyboard run isolated Vite reloads.

The running list matches `RasterPlayApp::examples`, which registers one document example. A separately authored `demo-session` source exists but it is absent from that catalogue. The navbar leaves its optional No Example entry disabled. The supported `example` query accepts an explicit registered option, and an empty or unknown query still falls back to Demo. The current catalogue offers no second selectable document for a switch test.

[Reusable probe](🔬️raster-windows.ts)
