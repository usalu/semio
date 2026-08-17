#!/usr/bin/env bash
# 🛠️ One-shot authoring aid for this ticket (NOT a repo script, NOT a migration shim): expands the
# hand-written per-node templates below across norm's fifteen structurally-identical compliance apps.
# The templates ARE the handcrafted source; this file only substitutes the seven per-app identity
# tokens (module, dir, struct, app id, label, variant, family) that genuinely differ. Kept in the
# ticket folder per CLAUDE.md; nothing under ✏️s/ references it.
set -euo pipefail
ROOT="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm"
T="$(cd "$(dirname "$0")" && pwd)/🧩️templates"

# MOD|DIR|STRUCT|APPID|LABEL|VARIANT|FAMILY|EMOJI
ROWS=(
"din4108|📕️din4108|Din4108PlayApp|norm-din-4108-play|DIN 4108|din4108|Din4108Family|🌡️"
"din16798|📗️din16798|Din16798PlayApp|norm-din-en-16798-play|DIN EN 16798|din16798|DinEn16798Family|🌬️"
"din18599|📙️din18599|Din18599PlayApp|norm-din-v-18599-play|DIN V 18599|din18599|DinV18599Family|🏢️"
"en1990|📘️en1990|En1990PlayApp|norm-en-1990-play|EN 1990|en1990|En1990Family|⚖️"
"en1991|📘️en1991|En1991PlayApp|norm-en-1991-play|EN 1991|en1991|En1991Family|🏋️"
"en1992|📘️en1992|En1992PlayApp|norm-en-1992-play|EN 1992|en1992|En1992Family|🧱️"
"en1993|📘️en1993|En1993PlayApp|norm-en-1993-play|EN 1993|en1993|En1993Family|🔩️"
"en1994|📘️en1994|En1994PlayApp|norm-en-1994-play|EN 1994|en1994|En1994Family|🧲️"
"en1995|📘️en1995|En1995PlayApp|norm-en-1995-play|EN 1995|en1995|En1995Family|🪵️"
"en1996|📘️en1996|En1996PlayApp|norm-en-1996-play|EN 1996|en1996|En1996Family|🧿️"
"en1997|📘️en1997|En1997PlayApp|norm-en-1997-play|EN 1997|en1997|En1997Family|⛰️"
"en1998|📘️en1998|En1998PlayApp|norm-en-1998-play|EN 1998|en1998|En1998Family|🌍️"
"en1999|📘️en1999|En1999PlayApp|norm-en-1999-play|EN 1999|en1999|En1999Family|✨️"
"iso16757|📓️iso16757|Iso16757PlayApp|norm-iso-16757-play|ISO 16757|iso16757|Iso16757Family|📗️"
"vdi3805|📔️vdi3805|Vdi3805PlayApp|norm-vdi-3805-play|VDI 3805|vdi3805|Vdi3805Family|🚰️"
)

emit() { # emit <template> <destination>
  sed -e "s|@MOD@|$MOD|g" -e "s|@DIR@|$DIR|g" -e "s|@STRUCT@|$STRUCT|g" -e "s|@APPID@|$APPID|g" \
      -e "s|@LABEL@|$LABEL|g" -e "s|@VARIANT@|$VARIANT|g" -e "s|@FAMILY@|$FAMILY|g" \
      -e "s|@EMOJI@|$EMOJI|g" -e "s|@CMDENUM@|$CMDENUM|g" "$T/$1" > "$2"
}

for row in "${ROWS[@]}"; do
  IFS='|' read -r MOD DIR STRUCT APPID LABEL VARIANT FAMILY EMOJI <<< "$row"
  CMDENUM="${STRUCT%PlayApp}Command"
  A="$ROOT/🎛️apps/$DIR"
  mkdir -p "$A/🎮️commands/📤️set-document" "$A/🎮️commands/🧮️evaluate" "$A/🎮️commands/☑️selected-check" \
           "$A/🎭️modes/✏️edit/🪟️windows/📥️inputs" "$A/🎭️modes/✏️edit/🪟️windows/📊️results" \
           "$A/📌️panels/📄️document" "$A/📌️panels/📚️catalogue" "$A/📌️panels/🔍️inspection"
  emit "app.rs.tpl"          "$A/🦀️component.rs"
  emit "cmd-set-document.rs.tpl"  "$A/🎮️commands/📤️set-document/🦀️component.rs"
  emit "cmd-evaluate.rs.tpl"      "$A/🎮️commands/🧮️evaluate/🦀️component.rs"
  emit "cmd-selected-check.rs.tpl" "$A/🎮️commands/☑️selected-check/🦀️component.rs"
  emit "mode-edit.rs.tpl"    "$A/🎭️modes/✏️edit/🦀️component.rs"
  emit "window-inputs.rs.tpl"  "$A/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"
  emit "window-results.rs.tpl" "$A/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"
  emit "panel-document.rs.tpl"   "$A/📌️panels/📄️document/🦀️component.rs"
  emit "panel-catalogue.rs.tpl"  "$A/📌️panels/📚️catalogue/🦀️component.rs"
  emit "panel-inspection.rs.tpl" "$A/📌️panels/🔍️inspection/🦀️component.rs"
done
echo "emitted $(find "$ROOT/🎛️apps" -name '🦀️component.rs' | wc -l | tr -d ' ') app component files"
