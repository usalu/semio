# Layout native check

Host: React playground `http://127.0.0.1:6079/?plugin=layout`.

Commands:

- `SEMIO_OFFLOAD=never repo cargo --store=shared --configuration=prototype --parallelism=2 -- check --message-format=short --tests --manifest-path 🌎️hub/🧩️compositions/📏️layout/📦️packages/🦀️rust/Cargo.toml -p semio-hub-layout`
- `repo run @semio-tech/framework-os-dev:serve-layout-react-prototype`

Blocker found: `semio-framework-value-native-owned` failed `error[E0433]` because `🌳️value/🔀️variant/🦀️.rs` matched `NativeRecordKey` in a debug scan without importing it. Import added from `crate::owned`.

Recheck: `Finished prototype profile in 14m 27s`, `EXIT:0`, `semio-hub-layout` lib test checked. The debug scan in the variant selector now names `NativeRecordKey`.

`cargo test -p semio-hub-layout` failed `error[E0449]` in `semio-s-artifact-draw-drawing-model`: `FillStyle` variant fields were marked `pub`. rustc 1.99 nightly rejects that because enum variant fields share the enum visibility. Those qualifiers are removed. Retest is running.

Serve `repo run @semio-tech/framework-os-dev:serve-layout-react-prototype` spent the graph inventory on one owner, then the slice worker was cut at 6614 MB (plan limit 6553 MB, step 2 of 6). The read continued with inferred projects kept. Hub tests are still compiling; no `error[` yet after the `FillStyle` qualifier removal.

Hub retest `EXIT:101`: `semio-plugin-interpreter-runtime` `error[E0308]` expected `ValueError`, found `Box<ValueError>` at `PluginHostError::NativeIo(Box::new(operation_authority::refusal(...)))`. Current sources already agree (`NativeIo` holds `Box<ValueError>`, `refusal` returns `ValueError`), so the failure was a mid-compile signature change. Retest waits until the serve graph read is not near the memory cut.

Serve graph read was cut at 6614 MB (step 2 of 6) and again at 10073 MB (step 3 of 6). The smaller wave reached `taken 513 of 534`, then a membership change started another inventory. Port 6079 is not listening yet.
