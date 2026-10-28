# Clip Output Audit

The example SVG exists among 128 Pango text cache files under the repository-root `media/texts` directory. Git lists this directory as untracked in the current checkout; none of its files is currently in the shared index. The ignore file has no root Manim media rule.

The canonical segment renderer already writes a compact Manim config, but direct authoring imports and some tests use upstream Manim defaults and create `media/texts` relative to the current directory. The TypeScript launcher sets an output root only for tests, while the Python runtime ignores the cache-root override when selecting its default output. Speech entry points also fall back to writing generated audio, traces and timing files beside sources.

The fix centralizes default output selection, configures unconfigured Manim before shared typography creates text, preserves explicit render configuration, and routes speech and trace artifacts into the same ignored root. Portable JSON cases precede implementation and are checked with real Manim/Pango output and Git ignore matching.

Repository graph inspection failed in unrelated source admission (`.cache/b/l/fixed.dir` missing); validation will use the registered `repo tools exec` interface and existing teaching environment.

The final reader audit also found that canonical narration lookup still read clause recordings beside scene source. It now reads the generated speech location, includes recordings and measured timing in render fingerprints, and passes a stable speech root into workers whose trace output is scoped by audience. A failing narration lookup case was added before this repair; the existing FFmpeg narration integration fixtures now use the generated location.
