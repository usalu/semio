// 📦️ WFC 2D `mutations` — the binary channel's TypeScript facade: the op wire protocol; a kind's binary TAG is its position in `KINDS`.
//
// The real codec is Rust (`../../🚪️io/🧬️mutations/💾️binary/🦀️.rs`); the normative sidecar beside it is
// `../../🚪️io/🧬️mutations/💾️binary/📡️.protocol.semio`. This leaf states the language id and the carrier type a
// TypeScript caller sees, so the plugin's TS barrel has one module per declared channel.

/** 🗣️ The `dsl::LanguageSpec` id this channel registers. */
export const WFC_2D_MUTATIONS_BINARY_LANGUAGE = "wfc.wfc2d.spr";

/** 🚚️ The carrier this channel's encode/decode speak. */
export type Wfc2dMutationBinary = Uint8Array;
