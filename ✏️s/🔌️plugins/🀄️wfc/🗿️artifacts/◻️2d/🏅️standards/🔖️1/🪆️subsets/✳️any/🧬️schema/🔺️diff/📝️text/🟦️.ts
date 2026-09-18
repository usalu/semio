// 📜️ WFC 2D `diff` — the text channel's TypeScript facade: the diff grammar — an artifact mark followed by one line per changed lane.
//
// The real codec is Rust (`../../🚪️io/🔺️diff/📝️text/🦀️.rs`); the normative sidecar beside it is
// `../../🚪️io/🔺️diff/📝️text/📖️.grammar.semio`. This leaf states the language id and the carrier type a
// TypeScript caller sees, so the plugin's TS barrel has one module per declared channel.

/** 🗣️ The `dsl::LanguageSpec` id this channel registers. */
export const WFC_2D_DIFF_TEXT_LANGUAGE = "wfc.wfc2d.diff";

/** 🚚️ The carrier this channel's encode/decode speak. */
export type Wfc2dDiffText = string;
