# Exploration Summary

- Course site `🎓️teaching/🏛️architecture/📚️course`: Vite 7 + React 19 + Tailwind 4, mounts `@semio-tech/course-react`. Nested cards via
  `LayeredOverview` (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx`) with live inert background grids, pointer
  parallax, hash routing (`replaceState`), `useElementsSurfaceChrome` themes, i18n bundles en/de with no default language.
- Tokens: `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json` (primary #ff344f, secondary #34d1bf, dark #001117, light #f7f3e3; fonts Anta,
  Kelly Slab, Share Tech Mono), generated CSS `🎨️styling/🤖️generated/🌓️palette-theme/🎨️.css`.
- Pets `🧰️framework/🛍️products/🐾️pets`: 2×3 affine FK (`🔨️modules/🦴️rig`), bezier-key tracks (`🔨️modules/🎞️animation`), SVG depiction
  with per-bone matrices. Pattern reused (not imported) for staffage rigs; quiz-pet playfulness informs the playground.
- Silhouettes `⚡️energy/🏗️authoring/🎨️visuals/🫟️silhouettes`: 12 unitless outlines, no real dimensions — not reusable as staffage.
- Writers: genuine Rust DWG AC1024 `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/…/📤️export/🦀️.rs` (`polylines_to_dwg_bytes`, oracle acadrust);
  Rust DXF R12; browser TS MP4 (`🧰️framework/🔨️modules/🖌️raster/🎥️video/🟦️.ts`, I_PCM H.264); TS stored ZIP stream; no TS PDF/DXF writer.
- Host: ffmpeg n8.1.2, rsvg-convert, pdftoppm, qpdf, node 24, bun 1.4.2 via `repo tools exec`, rustc wasm32-unknown-unknown target,
  Playwright 1.64 (browsers not verified). No ODA/LibreDWG. Python oracles in uv.lock: numpy, scipy, anastruct.
- Projects: `📋️project.json` targets `repo:script` → `repo ./📜️script.ts <cmd>`; vitest config `🧪️tests/🎚️config/🟦️.ts`; workspaces listed in
  `🎓️teaching/package.json`. `repo run @semio-tech/pets:test` took 9 min and failed (8 concurrent agent sessions); fast loop via
  `repo tools exec bun -- …`.
