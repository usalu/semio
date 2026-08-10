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
        #[path = "."]
        pub mod schema {
            #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod snapshot {
                #[path = "../../🗿️artifacts/🏛️program/🧬️schema/📸️snapshot/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🏛️program/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🏛️program/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod diff {
                #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🔺️diff/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod mutations {
                #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                pub mod binary;
                #[path = "."]
                pub mod information {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/ℹ️information/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/ℹ️information/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/ℹ️information/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod sustainability {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/♻️sustainability/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/♻️sustainability/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/♻️sustainability/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod accessibility {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/♿accessibility/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/♿accessibility/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/♿accessibility/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod conflicts {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/⚔️conflicts/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/⚔️conflicts/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/⚔️conflicts/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod options {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/⚖️options/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/⚖️options/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/⚖️options/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod functions {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/⚙️functions/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/⚙️functions/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/⚙️functions/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod risks {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/⚠️risks/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/⚠️risks/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/⚠️risks/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod decisions {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/✅decisions/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/✅decisions/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/✅decisions/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod validations {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/✔️validations/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/✔️validations/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/✔️validations/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod priorities {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/⭐priorities/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/⭐priorities/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/⭐priorities/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod flows {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🌊flows/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🌊flows/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🌊flows/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod environmental {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🌿environmental/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🌿environmental/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🌿environmental/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod workshops {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🎓workshops/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🎓workshops/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🎓workshops/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod scenarios {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🎬scenarios/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🎬scenarios/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🎬scenarios/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod benchmarks {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🏁benchmarks/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🏁benchmarks/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🏁benchmarks/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod activities {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🏃activities/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🏃activities/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🏃activities/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod infrastructure {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🏗️infrastructure/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🏗️infrastructure/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🏗️infrastructure/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod update_governance {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🏛️update-governance/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🏛️update-governance/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🏛️update-governance/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod organizational {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🏢organizational/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🏢organizational/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🏢organizational/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod update_meta {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🏷️update-meta/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🏷️update-meta/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🏷️update-meta/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod issues {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🐛issues/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🐛issues/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🐛issues/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod approvals {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/👍approvals/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/👍approvals/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/👍approvals/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod stakeholders {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/👥stakeholders/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/👥stakeholders/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/👥stakeholders/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod quality {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/💎quality/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/💎quality/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/💎quality/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod resilience {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/💪resilience/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/💪resilience/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/💪resilience/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod assumptions {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/💭assumptions/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/💭assumptions/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/💭assumptions/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod costs {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/💰costs/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/💰costs/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/💰costs/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod update_project {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📁update-project/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📁update-project/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📁update-project/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod documents {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📄documents/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📄documents/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📄documents/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod schedules {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📅schedules/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📅schedules/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📅schedules/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod growth {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📈growth/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📈growth/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📈growth/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod performance {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📊performance/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📊performance/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📊performance/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod operations {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📋operations/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📋operations/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📋operations/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod requirements {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📌requirements/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📌requirements/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📌requirements/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod site_context {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📍site-context/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📍site-context/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📍site-context/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod templates {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📐templates/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📐templates/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📐templates/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod reports {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📑reports/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📑reports/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📑reports/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod audit_events {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📒audit-events/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📒audit-events/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📒audit-events/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod knowledge {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📚knowledge/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📚knowledge/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📚knowledge/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod regulatory {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📜regulatory/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📜regulatory/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📜regulatory/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod changes {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📝changes/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📝changes/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📝changes/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod communication {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📡communication/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📡communication/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📡communication/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod resources {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📦resources/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📦resources/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📦resources/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod status_records {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📶status-records/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📶status-records/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/📶status-records/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod adjacencies {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔀adjacencies/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔀adjacencies/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔀adjacencies/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod processes {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔄processes/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔄processes/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔄processes/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod search_filters {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔍search-filters/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔍search-filters/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔍search-filters/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod access_rules {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔑access-rules/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔑access-rules/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔑access-rules/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod privacy {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔒privacy/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔒privacy/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔒privacy/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod relationships {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔗relationships/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔗relationships/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔗relationships/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod quantities {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔢quantities/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔢quantities/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔢quantities/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod analyses {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔬analyses/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔬analyses/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🔬analyses/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_snapshot {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🖼️set-snapshot/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🖼️set-snapshot/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🖼️set-snapshot/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod storage {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🗄️storage/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🗄️storage/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🗄️storage/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod meetings {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🗓️meetings/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🗓️meetings/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🗓️meetings/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod surveys {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🗳️surveys/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🗳️surveys/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🗳️surveys/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_adjacency {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🗺️set-adjacency/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🗺️set-adjacency/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🗺️set-adjacency/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod delivery {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🚚delivery/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🚚delivery/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🚚delivery/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod constraints {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🚧constraints/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🚧constraints/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🚧constraints/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod compliance_records {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🛂compliance-records/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🛂compliance-records/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🛂compliance-records/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod services {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🛎️services/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🛎️services/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🛎️services/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod equipment {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🛠️equipment/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🛠️equipment/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🛠️equipment/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod security {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🛡️security/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🛡️security/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🛡️security/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod collaboration {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🤝collaboration/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🤝collaboration/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🤝collaboration/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod safety {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🦺safety/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🦺safety/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🦺safety/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod users {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🧑users/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🧑users/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🧑users/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod human_factors {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🧠human-factors/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🧠human-factors/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🧠human-factors/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod flexibility {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🧩flexibility/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🧩flexibility/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🧩flexibility/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod wayfinding {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🧭wayfinding/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🧭wayfinding/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🧭wayfinding/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod elements {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🧱elements/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🧱elements/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🧱elements/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod traces {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🧵traces/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🧵traces/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🧵traces/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod clear_adjacency {
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🧹clear-adjacency/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🧹clear-adjacency/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏛️program/🧬️schema/🧬️mutations/🧹clear-adjacency/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
            }
        }
        pub mod op { pub use crate::artifacts::program::schema::mutations::text::*; pub use crate::artifacts::program::schema::mutations::ProgramMutation; }
        pub mod dsl { pub use crate::artifacts::program::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::program::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::program::schema::diff::*; pub use crate::artifacts::program::schema::diff::text::*; pub mod schema { pub use crate::artifacts::program::schema::diff::*; } pub mod text { pub use crate::artifacts::program::schema::diff::text::*; } }
        pub mod mutations { pub use crate::artifacts::program::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::program::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::program::schema::snapshot::binary::*; } }
        #[path = "../../🗿️artifacts/🏛️program/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🏛️program/🪓️decomposer/🦀️component.rs"]
        pub mod decomposer;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/🏛️program/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod import {
                #[path = "."]
                pub mod deserializers {
                    #[path = "."]
                    pub mod artifacts {
                        #[path = "."]
                        pub mod csv {
                            #[path = "../../🗿️artifacts/🏛️program/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🏛️program/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod xlsx {
                            #[path = "../../🗿️artifacts/🏛️program/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📕️xlsx/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod zip {
                            #[path = "../../🗿️artifacts/🏛️program/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod export {
                #[path = "."]
                pub mod serializers {
                    #[path = "."]
                    pub mod artifacts {
                        #[path = "."]
                        pub mod csv {
                            #[path = "../../🗿️artifacts/🏛️program/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🏛️program/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod xlsx {
                            #[path = "../../🗿️artifacts/🏛️program/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📕️xlsx/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod zip {
                            #[path = "../../🗿️artifacts/🏛️program/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod csv {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::program::io::export::serializers::artifacts::csv::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::program::io::import::deserializers::artifacts::csv::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::program::io::export::serializers::artifacts::json::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::program::io::import::deserializers::artifacts::json::*;
                }
            }
            #[path = "."]
            pub mod xlsx {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::program::io::export::serializers::artifacts::xlsx::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::program::io::import::deserializers::artifacts::xlsx::*;
                }
            }
            #[path = "."]
            pub mod zip {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::program::io::export::serializers::artifacts::zip::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::program::io::import::deserializers::artifacts::zip::*;
                }
            }
        }
        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/↔️adjacency/🦀️component.rs"]
            pub mod adjacency;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/✅️validate/🦀️component.rs"]
            pub mod validate;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/🎁️outputs/🦀️component.rs"]
            pub mod outputs;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/📄️report/🦀️component.rs"]
            pub mod report;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/📊️status-summary/🦀️component.rs"]
            pub mod status_summary;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/📐️template/🦀️component.rs"]
            pub mod template;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/📤️exchange/🦀️component.rs"]
            pub mod exchange;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/🔍️search/🦀️component.rs"]
            pub mod search;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/🔬️analyze/🦀️component.rs"]
            pub mod analyze;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/🧭️trace/🦀️component.rs"]
            pub mod trace;
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

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/🏛️architect/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🏛️architect/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/🏛️architect/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🏛️architect/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
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
