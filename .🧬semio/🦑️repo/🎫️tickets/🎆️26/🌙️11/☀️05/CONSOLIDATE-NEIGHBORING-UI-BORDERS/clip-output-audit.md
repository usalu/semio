# Clip Output Audit

The effective default output root in this workspace is D:/semio/.🧬semio/🎓️teaching/⚡️cache/videos; the directory did not exist at inspection. The repo-managed process had neither SEMIO_TEACHING_OUTPUT_ROOT nor SEMIO_TEACHING_CACHE_ROOT set. Explicit output roots and externally deployed copies were not discoverable, so their freshness is unknown. No architecture-energy MP4s were found among tracked files.

The entry point is 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🖌️render/🐍️.py. It requires --language en|de, and supports --audience, --video, --chapter, --output-root, --jobs, quality, resolution, and frame rate. The runtime writes chapter and lesson MP4s to <output-root>/<language>/<audience>/rendered/ by default. Presentation state includes compiler-discovered UI/CSS source dependencies; ordinary rerendering invalidates a stale presentation when these inputs change. --force also rerenders source scenes and is not required solely to refresh changed UI.

Rendering requires the Python/Manim and video toolchain and the browser React, esbuild, Tailwind, Playwright, and Chromium dependencies. The currently active dependency setup has not finished; no render or deployment freshness check was performed. Existing externally published copies require rerendering and republishing to reflect code changes.
