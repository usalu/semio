// 📜️ WFC 2D `mutations` — the text channel's TypeScript facade: the single-line op grammar — one keyword per declared kind, in `KINDS` order.
//
// The real codec is Rust (`../../../🧬️schema/🚪️io/📝️text/🧬️mutations/🦀️.rs`); the normative sidecar beside it is
// `../../../🧬️schema/🚪️io/📝️text/🧬️mutations/📖️.grammar.semio`. This leaf states the language id and the carrier type a
// TypeScript caller sees, so the plugin's TS barrel has one module per declared channel.

/** 🗣️ The `dsl::LanguageSpec` id this channel registers. */
export const WFC_2D_MUTATIONS_TEXT_LANGUAGE = "wfc.wfc2d.op";

/** 🚚️ The carrier this channel's encode/decode speak. */
export type Wfc2dMutationText = string;
