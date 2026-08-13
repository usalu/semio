# Required follow-up for `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️component.rs`

**Status: REQUIRED, not optional** — without this, `semio-s-plugin-flow-extension-draw` fails to
compile after this wave's change. Not applied directly because `✏️s/🔌️plugins/🌊️flow/**` is
SMO-claimed in `📌️important.md`'s hot-file table ("do not enter; file a `sharedFileRequests`
entry").

## What changed upstream

`🧰️framework/🔨️modules/◻2d/🗄️store/🦀️component.rs` (`DrawingStore`/`DrawingEngine`, the parallel
non-artifact store this ticket targets) was deleted. Its store-specific vocabulary
(`DrawingKernel`, `DrawingStore`, `DrawingEngine`, `DrawingHandle`, `DrawingKind`, `DrawingNode`,
`SceneNode`, `DrawingScene`, `FillStyle`, `StrokeStyle`, `GradientStop`, `LineCap`, `LineJoin`,
`Affine2D`) relocated — unchanged in shape/behavior — into
`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️component.rs` (flow's own private
ephemeral node-evaluation kernel, mirroring the already-existing `📐️brep-geometry` precedent) and
is re-exported at `flow_extension_sdk::*` (this crate already depends on `flow_extension_sdk` =
`semio-framework-os-flow`, so no `Cargo.toml` change is needed).

`PathSegment`/`Vec2`/`DrawingError`/`block_on` stay exactly where they were, in
`semio_framework_2d::*` — genuinely shared geometry-kernel primitives also used by the framework's
own `booleans`/`trace` modules and by the unrelated `🖍️draw` (non-flow) plugin.

## Required edit

`✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️component.rs`, lines 1, 3–23 (module doc link, imports,
`kind_label`):

```diff
-//! 🖊️ Flow draw module: 2D vector-graphics operators backed by [`semio_framework_2d::DrawingStore`].
+//! 🖊️ Flow draw module: 2D vector-graphics operators backed by [`flow_extension_sdk::DrawingStore`].
+
-use semio_framework_2d::{block_on, DrawingError, DrawingHandle, DrawingKernel, DrawingStore, FillStyle, GradientStop, LineCap, LineJoin, StrokeStyle, Vec2};
+use semio_framework_2d::{block_on, DrawingError, Vec2};
+use flow_extension_sdk::{DrawingHandle, DrawingKernel, DrawingStore, FillStyle, GradientStop, LineCap, LineJoin, StrokeStyle};
 use neural_engine::{channel_output, Atom, ChannelSpec, Dictionary, EvalError, FieldSpec, Operator, OperatorImpl, OperatorInfo, Registry, Schema, Value, ValueType};
 use flow_extension_sdk::with_drawing_kernel as with_kernel;

 // #region 🔖️Helpers

 fn map_kernel_error(error: DrawingError) -> EvalError {
     EvalError::InvalidInput(error.to_string())
 }

-fn kind_label(kind: semio_framework_2d::DrawingKind) -> &'static str {
+fn kind_label(kind: flow_extension_sdk::DrawingKind) -> &'static str {
     match kind {
-        semio_framework_2d::DrawingKind::Rect => "rect",
-        semio_framework_2d::DrawingKind::Ellipse => "ellipse",
-        semio_framework_2d::DrawingKind::Circle => "circle",
-        semio_framework_2d::DrawingKind::Line => "line",
-        semio_framework_2d::DrawingKind::Polygon => "polygon",
-        semio_framework_2d::DrawingKind::Path => "path",
-        semio_framework_2d::DrawingKind::Text => "text",
-        semio_framework_2d::DrawingKind::Group => "group",
+        flow_extension_sdk::DrawingKind::Rect => "rect",
+        flow_extension_sdk::DrawingKind::Ellipse => "ellipse",
+        flow_extension_sdk::DrawingKind::Circle => "circle",
+        flow_extension_sdk::DrawingKind::Line => "line",
+        flow_extension_sdk::DrawingKind::Polygon => "polygon",
+        flow_extension_sdk::DrawingKind::Path => "path",
+        flow_extension_sdk::DrawingKind::Text => "text",
+        flow_extension_sdk::DrawingKind::Group => "group",
     }
 }
```

Nothing else in the file changes — every other reference (`DrawingHandle`, `DrawingKernel`,
`DrawingStore`, `FillStyle`, `GradientStop`, `LineCap`, `LineJoin`, `StrokeStyle`, `Vec2`,
`DrawingError`) is a bare (unqualified) identifier already brought into scope by the `use`
statements above, so no other line in the ~1170-line file needs touching — verified by re-reading
the full file: all remaining usages are unqualified (`FillStyle::Solid { .. }`,
`StrokeStyle { .. }`, `with_kernel(|k| ...)`, etc.), never written as
`semio_framework_2d::FillStyle` etc. elsewhere in the file.

## Verification this wave could not run

`cargo check -p semio-s-plugin-flow-extension-draw --all-targets` was not run against this plugin
(read-only boundary). The sibling crate this wave DID edit and verify,
`semio-framework-os-flow` (home of the relocated kernel + its own copy of every test above,
`drawing_kernel_tests`), passed `cargo check --all-targets` and `cargo test --lib` clean — see
`📓️wave5-reports/2d-store-deletion-report.md`. The diff above is a mechanical import-repoint only;
no call site's argument/return shape changed.
