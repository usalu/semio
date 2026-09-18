// 📜️ WFC 2D `snapshot` — the text channel's TypeScript facade: the `.wfc2d` document grammar — one envelope preamble plus the record body.
//
// The real codec is Rust (`../../🚪️io/📸️snapshot/📝️text/🦀️.rs`); the normative sidecar beside it is
// `../../🚪️io/📸️snapshot/📝️text/📖️.grammar.semio`. This leaf states the language id and the carrier type a
// TypeScript caller sees, so the plugin's TS barrel has one module per declared channel.

/** 🗣️ The `dsl::LanguageSpec` id this channel registers. */
export const WFC_2D_SNAPSHOT_TEXT_LANGUAGE = "wfc.wfc2d";

/** 🚚️ The carrier this channel's encode/decode speak. */
export type Wfc2dSnapshotText = string;
