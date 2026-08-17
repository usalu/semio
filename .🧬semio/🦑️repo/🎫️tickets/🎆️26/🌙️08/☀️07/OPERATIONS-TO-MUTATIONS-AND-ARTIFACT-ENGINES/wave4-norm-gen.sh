#!/usr/bin/env bash
set -euo pipefail
ROOT="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
artifacts=(
  "din4108:📕️din4108:Din4108"
  "din16798:📗️din16798:Din16798"
  "din18599:📙️din18599:Din18599"
  "en1990:📘️en1990:En1990"
  "en1991:📘️en1991:En1991"
  "en1992:📘️en1992:En1992"
  "en1993:📘️en1993:En1993"
  "en1994:📘️en1994:En1994"
  "en1995:📘️en1995:En1995"
  "en1996:📘️en1996:En1996"
  "en1997:📘️en1997:En1997"
  "en1998:📘️en1998:En1998"
  "en1999:📘️en1999:En1999"
  "iso16757:📓️iso16757:Iso16757"
  "vdi3805:📔️vdi3805:Vdi3805"
)

for entry in "${artifacts[@]}"; do
  IFS=: read -r mod folder type <<<"$entry"
  base="$ROOT/$folder/🧬️mutations"
  mkdir -p "$base/📤️set-document/🦠️mutation" "$base/📤️set-document/🔺️diff" "$base/📤️set-document/↩️inverse"

  cat >"$base/🦀️component.rs" <<RS
//! 🧬️ $type artifact — document mutation dispatch (`SetDocument` only).

pub use crate::document::SetDocumentMutation;

use crate::artifacts::$mod::Document;

/// @emoji 🧬️ Whole-document replace — the only norm-family document mutation today.
pub type ${type}Mutation = SetDocumentMutation<Document>;
RS

  cat >"$base/🟦️component.ts" <<TS
/** @emoji 🧬️ $type document mutations (WASM wiring stub). */
export {};
TS

  cat >"$base/📤️set-document/🦠️mutation/🦀️component.rs" <<RS
//! 📤️ $type mutation — \`SetDocument\` payload + builder + apply.
use crate::artifacts::$mod::Document;
use crate::artifacts::$mod::mutations::${type}Mutation;

pub fn set_document(document: Document) -> ${type}Mutation {
    ${type}Mutation::SetDocument { document }
}

pub fn apply(projection: &mut Document, document: &Document) {
    *projection = document.clone();
}
RS

  cat >"$base/📤️set-document/🦠️mutation/🟦️component.ts" <<TS
/** @emoji 📤️ Set-document mutation leaf (stub). */
export {};
TS

  cat >"$base/📤️set-document/🔺️diff/🦀️component.rs" <<RS
//! 🔺️ Diff fragment for \`SetDocument\` on $type.
use crate::artifacts::$mod::Document;

pub type Diff = crate::document::DocumentDiff<Document>;
RS

  cat >"$base/📤️set-document/🔺️diff/🟦️component.ts" <<TS
/** @emoji 🔺️ Set-document diff leaf (stub). */
export {};
TS

  cat >"$base/📤️set-document/↩️inverse/🦀️component.rs" <<RS
//! ↩️ Inverse for \`SetDocument\` on $type.
use crate::artifacts::$mod::Document;
use crate::artifacts::$mod::mutations::${type}Mutation;

pub fn inverse(base: &Document, _replacement: &Document) -> Vec<${type}Mutation> {
    vec![${type}Mutation::SetDocument { document: base.clone() }]
}
RS

  cat >"$base/📤️set-document/↩️inverse/🟦️component.ts" <<TS
/** @emoji ↩️ Set-document inverse leaf (stub). */
export {};
TS
done

echo "Generated mutations for ${#artifacts[@]} artifacts"
