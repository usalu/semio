//! 🏛️ Architect plugin — the architectural-programming document app, bundled as a hot-swappable WASM
//! plugin.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault>`, the exact signature
// `DocumentApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned
// error type; boxing it here would diverge from the trait it must satisfy, and the lint does not fire on
// the trait impl itself (only on the free functions the taxonomy split creates), so this is a pure
// artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod program {
        #[path = "../../🗿️artifacts/🏛️program/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧱️kernel/🦀️component.rs"]
        pub mod kernel;
        #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🗄️registers/🦀️component.rs"]
        pub mod registers;

        #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/🏛️program/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🏛️program/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
        #[path = "../../🗿️artifacts/🏛️program/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod stakeholders {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/👥stakeholders/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/👥stakeholders/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/👥stakeholders/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod users {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🧑users/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🧑users/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🧑users/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod activities {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🏃activities/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🏃activities/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🏃activities/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod functions {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/⚙️functions/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/⚙️functions/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/⚙️functions/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod elements {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🧱elements/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🧱elements/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🧱elements/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod quantities {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔢quantities/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔢quantities/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔢quantities/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod relationships {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔗relationships/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔗relationships/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔗relationships/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod adjacencies {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔀adjacencies/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔀adjacencies/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔀adjacencies/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod processes {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔄processes/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔄processes/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔄processes/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod flows {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🌊flows/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🌊flows/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🌊flows/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod access_rules {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔑access-rules/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔑access-rules/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔑access-rules/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod operations {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📋operations/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📋operations/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📋operations/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod equipment {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🛠️equipment/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🛠️equipment/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🛠️equipment/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod resources {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📦resources/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📦resources/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📦resources/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod storage {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🗄️storage/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🗄️storage/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🗄️storage/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod environmental {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🌿environmental/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🌿environmental/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🌿environmental/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod human_factors {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🧠human-factors/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🧠human-factors/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🧠human-factors/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod accessibility {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/♿accessibility/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/♿accessibility/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/♿accessibility/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod privacy {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔒privacy/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔒privacy/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔒privacy/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod safety {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🦺safety/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🦺safety/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🦺safety/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod security {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🛡️security/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🛡️security/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🛡️security/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod regulatory {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📜regulatory/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📜regulatory/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📜regulatory/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod site_context {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📍site-context/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📍site-context/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📍site-context/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod organizational {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🏢organizational/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🏢organizational/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🏢organizational/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod services {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🛎️services/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🛎️services/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🛎️services/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod infrastructure {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🏗️infrastructure/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🏗️infrastructure/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🏗️infrastructure/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod information {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/ℹ️information/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/ℹ️information/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/ℹ️information/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod communication {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📡communication/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📡communication/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📡communication/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod wayfinding {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🧭wayfinding/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🧭wayfinding/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🧭wayfinding/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod schedules {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📅schedules/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📅schedules/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📅schedules/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod flexibility {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🧩flexibility/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🧩flexibility/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🧩flexibility/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod growth {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📈growth/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📈growth/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📈growth/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod sustainability {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/♻️sustainability/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/♻️sustainability/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/♻️sustainability/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod resilience {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/💪resilience/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/💪resilience/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/💪resilience/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod costs {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/💰costs/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/💰costs/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/💰costs/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod delivery {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🚚delivery/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🚚delivery/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🚚delivery/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod risks {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/⚠️risks/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/⚠️risks/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/⚠️risks/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod conflicts {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/⚔️conflicts/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/⚔️conflicts/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/⚔️conflicts/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod requirements {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📌requirements/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📌requirements/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📌requirements/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod priorities {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/⭐priorities/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/⭐priorities/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/⭐priorities/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod scenarios {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🎬scenarios/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🎬scenarios/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🎬scenarios/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod options {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/⚖️options/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/⚖️options/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/⚖️options/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod decisions {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/✅decisions/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/✅decisions/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/✅decisions/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod validations {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/✔️validations/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/✔️validations/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/✔️validations/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod performance {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📊performance/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📊performance/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📊performance/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod quality {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/💎quality/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/💎quality/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/💎quality/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod documents {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📄documents/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📄documents/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📄documents/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod changes {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📝changes/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📝changes/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📝changes/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod collaboration {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🤝collaboration/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🤝collaboration/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🤝collaboration/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod analyses {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔬analyses/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔬analyses/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔬analyses/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod reports {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📑reports/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📑reports/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📑reports/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod search_filters {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔍search-filters/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔍search-filters/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🔍search-filters/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod status_records {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📶status-records/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📶status-records/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📶status-records/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod workshops {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🎓workshops/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🎓workshops/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🎓workshops/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod surveys {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🗳️surveys/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🗳️surveys/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🗳️surveys/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod issues {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🐛issues/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🐛issues/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🐛issues/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod audit_events {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📒audit-events/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📒audit-events/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📒audit-events/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod templates {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📐templates/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📐templates/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📐templates/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod knowledge {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📚knowledge/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📚knowledge/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📚knowledge/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod benchmarks {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🏁benchmarks/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🏁benchmarks/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🏁benchmarks/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod assumptions {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/💭assumptions/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/💭assumptions/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/💭assumptions/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod constraints {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🚧constraints/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🚧constraints/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🚧constraints/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod compliance_records {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🛂compliance-records/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🛂compliance-records/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🛂compliance-records/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod approvals {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/👍approvals/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/👍approvals/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/👍approvals/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod meetings {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🗓️meetings/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🗓️meetings/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🗓️meetings/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod traces {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🧵traces/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🧵traces/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🧵traces/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod update_meta {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🏷️update-meta/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🏷️update-meta/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🏷️update-meta/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod update_project {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📁update-project/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📁update-project/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/📁update-project/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod update_governance {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🏛️update-governance/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🏛️update-governance/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🏛️update-governance/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_adjacency {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🗺️set-adjacency/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🗺️set-adjacency/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🗺️set-adjacency/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod clear_adjacency {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🧹clear-adjacency/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🧹clear-adjacency/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🧹clear-adjacency/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🖼️set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🖼️set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏛️program/🧬️mutations/🖼️set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/🏛️program/🗣️dsl/🦀️component.rs"]
        pub mod dsl;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/🏛️program/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;
            #[path = "../../🗿️artifacts/🏛️program/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }
        #[path = "../../🗿️artifacts/🏛️program/📡️spr/🦀️component.rs"]
        pub mod spr;

        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/↔️adjacency/🦀️component.rs"]
            pub mod adjacency;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/🔬️analyze/🦀️component.rs"]
            pub mod analyze;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/📤️exchange/🦀️component.rs"]
            pub mod exchange;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/🎁️outputs/🦀️component.rs"]
            pub mod outputs;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/📄️report/🦀️component.rs"]
            pub mod report;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/🔍️search/🦀️component.rs"]
            pub mod search;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/📊️status-summary/🦀️component.rs"]
            pub mod status_summary;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/📐️template/🦀️component.rs"]
            pub mod template;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/🧭️trace/🦀️component.rs"]
            pub mod trace;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/✅️validate/🦀️component.rs"]
            pub mod validate;
        }
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod architect {
        #[path = "../../🎛️apps/🏛️architect/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/🏛️architect/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/🏛️architect/🎨️chrome/🦀️component.rs"]
        pub mod chrome;
        #[path = "../../🎛️apps/🏛️architect/🗂️catalog/🦀️component.rs"]
        pub mod catalog;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🏛️architect/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🏛️architect/🎮️commands/📋️register/🦀️component.rs"]
            pub mod register;
            #[path = "../../🎛️apps/🏛️architect/🎮️commands/↔️adjacency/🦀️component.rs"]
            pub mod adjacency;
            #[path = "../../🎛️apps/🏛️architect/🎮️commands/📐️template/🦀️component.rs"]
            pub mod template;
            #[path = "../../🎛️apps/🏛️architect/🎮️commands/📤️exchange/🦀️component.rs"]
            pub mod exchange;
            #[path = "../../🎛️apps/🏛️architect/🎮️commands/🏗️element/🦀️component.rs"]
            pub mod element;
            #[path = "../../🎛️apps/🏛️architect/🎮️commands/🔬️analysis/🦀️component.rs"]
            pub mod analysis;
            #[path = "../../🎛️apps/🏛️architect/🎮️commands/🕸️graph/🦀️component.rs"]
            pub mod graph;
            #[path = "../../🎛️apps/🏛️architect/🎮️commands/🔍️search/🦀️component.rs"]
            pub mod search;
        }

        #[path = "."]
        pub mod modes {
            #[path = "../../🎛️apps/🏛️architect/🎭️modes/🔍️review/🦀️component.rs"]
            pub mod review;
            #[path = "../../🎛️apps/🏛️architect/🎭️modes/📊️report/🦀️component.rs"]
            pub mod report;

            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/🏛️architect/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/🏛️architect/🎭️modes/✏️edit/🪟️windows/↔️adjacency/🦀️component.rs"]
                    pub mod adjacency;
                    #[path = "../../🎛️apps/🏛️architect/🎭️modes/✏️edit/🪟️windows/🕸️graph/🦀️component.rs"]
                    pub mod graph;
                    #[path = "../../🎛️apps/🏛️architect/🎭️modes/✏️edit/🪟️windows/📋️register/🦀️component.rs"]
                    pub mod register;
                    #[path = "../../🎛️apps/🏛️architect/🎭️modes/✏️edit/🪟️windows/📄️report/🦀️component.rs"]
                    pub mod report;
                    #[path = "../../🎛️apps/🏛️architect/🎭️modes/✏️edit/🪟️windows/🧭️trace/🦀️component.rs"]
                    pub mod trace;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🏛️architect/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🏛️architect/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/🏛️architect/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }
}
//#endregion 🎛️Apps

//#region 🔖️Plugin
pub use artifacts::program::engine::register_architect_exports;

#[path = "../../🔌️plugin/🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/🏛️program/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_program_demo;
    #[path = "../../🎛️apps/🏛️architect/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_architect_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
