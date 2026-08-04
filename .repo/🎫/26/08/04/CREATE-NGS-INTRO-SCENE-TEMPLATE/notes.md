# NGS Intro Scene Template

- Goal: `🎯r2602🎯updateddocs🎯updateduserdocs🎯updatedtutorials`
- Institute: https://www.iek.uni-hannover.de/ngs
- Smoke render: `tutorial/intro/media/videos/intro_scene/480p15/Demo_Intro_Kuehllast.mp4`
- Usage: subclass `NGSIntro` and set `topic_de`, `topic_explain_de`, `series_de`
- Repo MCP was unavailable in this session; ticket folder created manually.

## Redesign pass

- Removed the glow circle, top accent line and the line-art house mark.
- Border chrome: double hairline rectangle + cyan corner ticks, drawn with `Create`.
- Watermark: `welfenschloss (1) (1).png` recolored to a white stencil at 13% and revealed
  as 30 vertical slices wiping left→right behind a travelling cyan sweep line.
- Corner mark: `csm_leibniz-binaerzahlen_13f738b2c9 (1).png` recolored to cyan, top-left with hairline.
- Both PNGs are black-on-transparent, so `_stencil()` rewrites RGB and scales alpha;
  Manim 0.20.1 `ImageMobject.set_opacity` multiplies `orig_alpha_pixel_array`, so transparency survives.
- Copy order: university → diamond rule → parent institute → institute → url → topic plate.
- Preview stills: `preview/final.png`, `preview/final_th.png`.
