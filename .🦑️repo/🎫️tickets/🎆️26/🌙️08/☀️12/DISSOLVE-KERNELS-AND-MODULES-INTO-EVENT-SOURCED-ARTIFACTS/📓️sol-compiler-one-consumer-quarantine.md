# Compiler One-Consumer Quarantine

## Live consumer disposition

The stale census reports `🧰️framework/🔨️modules/📚️compiler` as a zero-consumer module. Live source and Cargo resolution instead show exactly one terminal production component: OS Infinite Canvas.

The only production call sites are in `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🦀️component.rs`:

- `compile_snippet_to_svg` for mathematical icon notation;
- `compile_emoji_to_svg` for emoji icons;
- `compile_text_to_svg` for text icons.

`compile_code_to_svg` has no production consumer. All other compiler source files are private syntax, embedded-font, text-shaping, math-layout, and SVG-emission implementation reached through those functions. Tests and package glue do not increase consumer count.

Under the binding consumer rule, this package cannot survive as a framework module. The three live responsibilities must become private Infinite Canvas implementation; the code-only renderer must be deleted unless a live consumer appears before the lease baseline. No compatibility facade or forwarding package is permitted.

## Current blocker

The authored compiler tree, root `Cargo.toml`, Infinite Cargo manifest, and Infinite source tree are clean. Atomic package dissolution nevertheless requires coordinator-owned removal of the explicit root workspace member and lock regeneration. `Cargo.lock` is currently dirty and belongs to another active lease. Therefore no implementation packet is issued yet.

Current control hashes:

- root `Cargo.toml`: `f98a8196bcef9da0bbe552484d65e873d12575272a17f3fe3ebe1a0d6f106255`;
- compiler Cargo manifest: `f127c93f99b347637d9dfcc4db343df35fdd52de5e4b9d03f74598a54943412e`;
- compiler glue: `95b821e94ffbf15eb2c1ebf9af8ee4068c8f9e3115a974944e5cbdcdeaab98e8`;
- Infinite Cargo manifest: `ee69a0bba93cd1d0676bbdc3146e0648d34d70126c22da90fa6e5fe761f560fd`.

The lease remains quarantined until the dirty lock owner releases it and the coordinator rereads both the lock diff and all consumer hashes.
