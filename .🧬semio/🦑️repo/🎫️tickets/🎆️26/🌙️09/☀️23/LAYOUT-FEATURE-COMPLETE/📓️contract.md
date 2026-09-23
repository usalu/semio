# Layout feature complete

Audits: [hover](d7b786cd-d4df-42d1-bb85-566ef0355ef7), [placement](0922fb5c-6ed3-4d8e-ac4c-5549225c867d), [preview](43376143-8470-42d6-8a10-f43f04c83842), [gumball](9c9a91fc-e161-49ee-8718-6cf4d5ec56d5), [gaps](9ce7cfe8-599f-4a2d-9792-9021428a8fdb).

## Done in this pass

Hover and live paint no longer build a `LayoutEngine` or shape text.

- Pointer down and pointer move call `hit_test_page_frames` (bounds only, parent frames then page frames).
- Host canvas uses `build_interactive_display_list`. Stories are strings. Below zoom `0.35` story strings are omitted and baseline guides are skipped.
- A link with `proxyDataUrl` paints as a host `kind: "image"` layer (`dataUrl`). Missing links stay placeholder rects.
- Export and `build_display_list_for_page` still shape glyphs.

## Still required

1. Blueprint transform utility with a gumball. Host overlay math is Fem2d-scaled (`FEM2D_SCALE` / `FEM2D_ORIGIN` in `Canvas2dHost/🟦️GumballOverlay.tsx`). Layout page space needs a world-space gumball mode. Commands: `translateSelection` → `move-frame`, `scaleSelection` → `resize-frame`, rotate needs a new `rotate-frame` mutation (`bounds.rotation` exists, no verb). Utility id `transform` on the blueprint window, meta layers `meta:utility` and `meta:gumball`.
2. Native placement. Frames are only `rect` | `text` | `image`. Links are path + hash + proxy, not `ArtifactLink`. Catalogue drop accepts page/rect/text/image only. Each placed 2D artifact (png, pdf, drawing, raster, svg, and the other 2D snapshots) must be stored as a native link and drawn from its proxy at low zoom.
3. Do not put `LayoutEngine::new()` back on the pointer or canvas path.
