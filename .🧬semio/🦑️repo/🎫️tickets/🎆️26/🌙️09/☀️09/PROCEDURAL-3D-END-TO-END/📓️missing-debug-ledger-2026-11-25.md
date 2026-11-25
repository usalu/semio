# Missing Debug Ledger (2026-11-25)

- `semio-os-store-member-open-store` failed to compile on g1-m02: three sources (`🌱️value/🛬️decode/🧳️owned/{🏗️record-machine,🧳️original}/🦀️.rs`, `🏪️store/🧩️composition/🚪️open/🏭️operation/🫴️record/🦀️.rs`) mount `#[path]` `debug-ledger.rs` from the ignored `🗑️generated` folder of ticket `26/07/23/GET-PROCESS-WORKING-END-TO-END`. Ignored folders are not replicated, so every machine but the author's breaks.
- The probe was still being edited at 09:19, so it was left in place. An inert local stand-in of that file (ignored, not replicated) lets this machine compile; `on()` is `false`, so behaviour is unchanged.
- Owner action (process ticket): remove the mounts and `dbg::` calls from the three sources. Then delete the stand-in.
