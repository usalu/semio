# Rotated Image Proxy Sampling

The canvas derives horizontal and vertical sample density from the inverse rotation followed by the document-to-source scale. The length of each transformed axis gives the source pixels traversed per document unit. This preserves the source columns and rows when a linked PNG turns through 90° and keeps the existing 128-pixel bound per axis.

The shared fixture uses a two-by-three RGBA image in a 100-by-20 frame. The old sizing produces a one-by-fifteen preview. The regression expects a three-by-two preview and checks every RGBA channel and the host layer bounds through the public canvas renderer.

The standalone Pillow reference reproduced the fixture. It is executable with `repo tools exec --light uv -- run --no-project --no-cache --script <path to rotated-proxy-oracle.py>`.

The first native attempt stopped during Cargo source admission because the framework's generated `icon_pack_digest` module was missing. The test did not execute on that attempt.

The Rust source and companion test parsed with `rustfmt --emit stdout`. The scoped diff passed `git diff --check`.

The first combined native run passed source admission after regenerating the framework prerequisites, then stopped on a concurrent framework API edit before the Layout tests ran.

The combined native retry yielded its host-capacity slot before running the canvas test. The focused Layout diff WASI gate has since passed. The full WASI guest build is queued, and the physical browser journey still needs the new guest and its descriptors.
