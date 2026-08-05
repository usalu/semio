#!/usr/bin/env bash
# 🛠️ One-shot authoring aid for this ticket: expands norm's 📦️lib.rs wiring (pure #[path] mod tree +
# semio_plugin! registration) across the fifteen apps/artifacts. Prose and structure are handwritten
# below; only the per-app identity tokens repeat. Kept in the ticket folder per CLAUDE.md.
set -euo pipefail
OUT="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/📦️lib.rs"
ROWS=(
"din4108|📕️din4108|Din4108PlayApp" "din16798|📗️din16798|Din16798PlayApp" "din18599|📙️din18599|Din18599PlayApp"
"en1990|📘️en1990|En1990PlayApp" "en1991|📘️en1991|En1991PlayApp" "en1992|📘️en1992|En1992PlayApp"
"en1993|📘️en1993|En1993PlayApp" "en1994|📘️en1994|En1994PlayApp" "en1995|📘️en1995|En1995PlayApp"
"en1996|📘️en1996|En1996PlayApp" "en1997|📘️en1997|En1997PlayApp" "en1998|📘️en1998|En1998PlayApp"
"en1999|📘️en1999|En1999PlayApp" "iso16757|📓️iso16757|Iso16757PlayApp" "vdi3805|📔️vdi3805|Vdi3805PlayApp"
)
{
cat <<'HDR'
//! 📏️ Norm plugin — fifteen compliance-standard document apps (DIN 4108, DIN EN 16798, DIN V 18599,
//! EN 1990–1999, ISO 16757, VDI 3805) in one hot-swappable WASM plugin, each backed by a headless
//! `NormHost` that recomputes its `CheckReport` from the document on every read.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).
//!
//! 🫀️ `core` is unusually large for a plugin kernel here, and deliberately so: the fifteen standards are
//! structurally identical apps over fifteen genuinely different document schemas, so the *domain* kernel
//! (quantities, clause identity, check results, national annexes, the `NormFamily`/`NormHost` contract,
//! the generic whole-document operation and its text/binary codecs) and the *app-surface* kernel (the one
//! shared config, the media ports, the render primitives, the manifest constructors) each exist exactly
//! once, while every per-standard fact — schema, ids, labels, compute — lives in that standard's own
//! artifact and app nodes.

// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<Operation, NormConfigOperation>, Fault>`, the exact signature `DocumentApp::handle` and
// `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it here
// would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself (only
// on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#![allow(clippy::result_large_err)]

//#region 🫀️Core
/// 🤝️ The cross-artifact, cross-app kernel: the norm domain model plus everything all fifteen apps
/// share verbatim. Depends on no artifact and on no app.
#[path = "."]
pub mod core {
    #[path = "🫀️core/🦀️component.rs"]
    mod component;
    pub use component::*;

    #[path = "🫀️core/🦀️config.rs"]
    mod config;
    pub use config::*;

    #[path = "🫀️core/🦀️app.rs"]
    pub mod app;
}
//#endregion 🫀️Core

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
HDR
for row in "${ROWS[@]}"; do IFS='|' read -r MOD DIR _ <<< "$row"; cat <<EOF
    #[path = "."]
    pub mod $MOD {
        #[path = "🗿️artifacts/$DIR/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "🗿️artifacts/$DIR/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "🗿️artifacts/$DIR/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "🗿️artifacts/$DIR/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "🗿️artifacts/$DIR/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "🗿️artifacts/$DIR/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "🗿️artifacts/$DIR/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

EOF
done
cat <<'MID'
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
MID
for row in "${ROWS[@]}"; do IFS='|' read -r MOD DIR _ <<< "$row"; cat <<EOF
    #[path = "."]
    pub mod $MOD {
        #[path = "🎛️apps/$DIR/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "🎛️apps/$DIR/🎮️commands/📤️set-document/🦀️component.rs"]
            pub mod set_document;
            #[path = "🎛️apps/$DIR/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "🎛️apps/$DIR/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🎛️apps/$DIR/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🎛️apps/$DIR/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "🎛️apps/$DIR/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "🎛️apps/$DIR/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "🎛️apps/$DIR/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "🎛️apps/$DIR/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

EOF
done
cat <<'TAIL'
}
//#endregion 🎛️Apps

//#region 🔖️Plugin
/// 🗂️ Sole native setup hook for the whole plugin bundle — registers all fifteen family document kinds'
/// pack↔dsl codecs. Each app's document schema is the single source of truth for its own registration.
fn register_norm_exports() {
TAIL
for row in "${ROWS[@]}"; do IFS='|' read -r MOD _ STRUCT <<< "$row"
  echo "    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::$MOD::$STRUCT>(apps::$MOD::DOCUMENT_SCHEMA);"
done
cat <<'TAIL2'
}

semio_framework_plugin::semio_plugin! {
    id: "norm",
    label: "Norm",
    version: "0.1.0",
    setup: register_norm_exports,
    apps: [
TAIL2
for row in "${ROWS[@]}"; do IFS='|' read -r MOD _ STRUCT <<< "$row"
  echo "        apps::$MOD::create_${MOD}_app => apps::$MOD::$STRUCT,"
done
cat <<'TAIL3'
    ],
}
//#endregion 🔖️Plugin
TAIL3
} > "$OUT"
echo "wrote $(wc -l < "$OUT") lines"
