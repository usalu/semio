# Explicit Canvas Font Families

## Implemented source

The shared native canvas text lowerer consumes an explicit catalog family from its JSON text record. The first-party text face enum accepts Anta, Kelly Slab, Share Tech Mono and Noto Emoji; missing or unknown serialized families refuse admission. Browser canvas text consumes the same four explicit names. Draw's decorative artboard dimension text supplies Anta explicitly, and its authored text path resolution remains the font lane's responsibility.

Noto Emoji assets are independent subset faces. The original single-family registration selected the wrong subset face for 😀 and produced an 11.71875 advance instead of the original face's 30.46875 advance. Each subset now has a private distinct family identity; the requested emoji face supplies the ordered full subset stack. The generic emoji fallback uses the full stack as well.

The shared catalog fixture names four authored glyph cases and rejects implicit system families. Native tests compare actual atlas advances against Swash reading the original font assets. Browser tests validate family/size serialization against JSDOM CSSOM and validate the fixture using Ajv.

## Actual verification

The isolated Nx ui-test engine gate exited 0. Its five selected native tests passed: original font face advances, transformed atlas quads, affine glyph/image corners, actual GPU production shader validation, and actual image framebuffer parity against Tiny Skia for five transforms and 5,120 pixels. Frozen evidence: 🗑️generated/owner-ui-test-engine.log, 🗑️generated/ui/semio-nextest-w6L4Ao.

The isolated Nx renderer-family-test gate exited 0: one Vitest file, ten tests passed. Evidence: 🗑️generated/owner-renderer-family-test.log. Earlier invocations selected zero tests and failed. The owner config used Windows backslashes in absolute include globs; include paths are now normalized to slash separators. The isolated selector uses the real source suffix without parent-directory filter segments. Executable routes are registered in the existing ticket Nx script/project and launch configuration.

These are targeted native shared UI and browser canvas tests. They do not prove mounted Draw application runtime, full native Plugin compilation, joined emoji shaping, or every font glyph.