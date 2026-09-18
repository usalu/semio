// 📦️ WFC 2D `diff` — the binary channel's TypeScript facade: the diff protocol spec; this subset's diff never rides its own envelope.
//
// The real codec is Rust (`../../🚪️io/🔺️diff/💾️binary/🦀️.rs`); the normative sidecar beside it is
// `../../🚪️io/🔺️diff/💾️binary/📡️.protocol.semio`. This leaf states the language id and the carrier type a
// TypeScript caller sees, so the plugin's TS barrel has one module per declared channel.

/** 🗣️ The `dsl::LanguageSpec` id this channel registers. */
export const WFC_2D_DIFF_BINARY_LANGUAGE = "wfc.wfc2d.diff";

/** 🚚️ The carrier this channel's encode/decode speak. */
export type Wfc2dDiffBinary = Uint8Array;
