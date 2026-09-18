// 📦️ WFC 2D `snapshot` — the binary channel's TypeScript facade: the `.wfc2d` pack envelope — magic, fixed header, varint payload, footer.
//
// The real codec is Rust (`../../🚪️io/📸️snapshot/💾️binary/🦀️.rs`); the normative sidecar beside it is
// `../../🚪️io/📸️snapshot/💾️binary/📡️.protocol.semio`. This leaf states the language id and the carrier type a
// TypeScript caller sees, so the plugin's TS barrel has one module per declared channel.

/** 🗣️ The `dsl::LanguageSpec` id this channel registers. */
export const WFC_2D_SNAPSHOT_BINARY_LANGUAGE = "wfc.wfc2d.pack";

/** 🚚️ The carrier this channel's encode/decode speak. */
export type Wfc2dSnapshotBinary = Uint8Array;
