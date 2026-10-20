#!/bin/sh
# 🌱 Seeded regressions on the post overlay of the umbrella plan: a wrong owner crate and a missing item must each add resolution or privacy errors to the differential.
# usage: sh seed.sh   (after overlay.sh umbrella; sandbox import paths are rewritten to `crate::__m_<crate>`)
set -eu
H="$(cd "$(dirname "$0")" && pwd)"; O=/private/tmp/gran-hubs; G="$H/../../🗑️generated/gran-hubs/oracle"
P="semio-framework-plugin-decl-registry"
F="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🌳️tree/🦀️.rs"
L="use crate::__m_semio_framework_manifest_app_surface::AppDefinition;"
nice -n 10 python3 "$H/oracle.py" run "$O/overlay/post" "$O/fam/umbrella.json" seedowner --cfgs lib --mutate "$P::$F@@$L@@use crate::__m_semio_framework_manifest_actions::AppDefinition;" > "$O/oracle-seedowner.log" 2>&1
nice -n 10 python3 "$H/oracle.py" run "$O/overlay/post" "$O/fam/umbrella.json" seeditem --cfgs lib --mutate "$P::$F@@$L@@use crate::__m_semio_framework_manifest_app_surface::AppDefinitionZ;" > "$O/oracle-seeditem.log" 2>&1
for l in seedowner seeditem; do echo "== pre vs $l"; python3 "$H/oracle_diff.py" "$G/gh-umbrellafam-pre-cur-lib-final.err" "$G/gh-umbrellafam-$l-cur-lib-final.err" | head -4 || true; done
