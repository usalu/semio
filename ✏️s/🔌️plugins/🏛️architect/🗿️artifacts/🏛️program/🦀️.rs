//! 🏛️ Architect program artifact — the root program document: all 65 feature-area registers plus
//! meta, project, and governance (constitutional: general).
//!
//! Domain row types live under `🧬️schema/🗄️registers`; shared entity primitives under
//! `🧬️schema/🧱️kernel`. The persisted snapshot type is `ProgramSnapshot`.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
mod art_program_demo_tests;
extern crate semio_framework_schema as framework_schema;

#[cfg(test)]
use pack::snapshot::ArtifactDsl;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#region 🔖️Declaration
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.architect.program")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.architect.program.schema.artifact")?, ArtifactCapabilityKind::schema())
                .descriptor(b"s.architect.program")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.architect.program")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.architect.program.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.architect.program.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.architect.program.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.architect.program.composer.native")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.architect.program@1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.architect.program@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.architect.program.composer.zip")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.zip@2.0/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.zip@2.0/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.architect.program.composer.csv")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.csv@rfc4180/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.csv@rfc4180/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.architect.program.composer.xlsx")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.xlsx@ecma-376/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.xlsx@ecma-376/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.architect.program.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.architect.program.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"architect.program:architect")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "architect.program")?)?
                .claim(ArtifactIdentityClaim::codec_extension("architect.program", "architect")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.architect.program.localization.en")?, ArtifactCapabilityKind::localization())
                .descriptor(b"Architect")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Architect")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.architect.program.localization.de")?, ArtifactCapabilityKind::localization())
                .descriptor(b"Architekt")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Architekt")?)?,
        )
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(schema::program_artifact_schema_descriptor())
        .inferences([standards::v1::subsets::any::schema::inferences::program_artifact_inference_descriptor()])
        .composers(standards::v1::subsets::any::io::io_registry::entries())
        .document_codec::<semio_framework_plugin::EditorApp<editor::architect::ArchitectPlayApp>>()
        .try_build()
}
//#endregion 🔖️Declaration

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_architect_program_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_architect_program_tests::standards::v1::subsets::any::schema::kernel;
                    #[path = "."]
                    pub mod registers {
                        pub use semio_s_artifact_architect_program_registers::standards::v1::subsets::any::schema::registers::*;

                        pub use semio_s_artifact_architect_program_catalog::standards::v1::subsets::any::schema::registers::catalog;
                        pub use catalog::{RegisterKey, RegisterLabel, RegisterRow};
                    }
                    pub use semio_s_artifact_architect_program_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_architect_program_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_architect_program_inferences::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_architect_program_mutations::standards::v1::subsets::any::schema::mutations::*;

                        pub use semio_s_artifact_architect_program_schema::standards::v1::subsets::any::schema::mutations::create_register_row;
                        pub use semio_s_artifact_architect_program_schema::standards::v1::subsets::any::schema::mutations::delete_register_row;
                        pub use semio_s_artifact_architect_program_schema::standards::v1::subsets::any::schema::mutations::rename_register_row;
                        pub use semio_s_artifact_architect_program_schema::standards::v1::subsets::any::schema::mutations::replace_register_row;
                        pub use semio_s_artifact_architect_program_catalog::standards::v1::subsets::any::schema::mutations::rename_governance;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::replace_governance;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::rename_meta;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::replace_meta;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::rename_project;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::replace_project;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::connect_adjacency;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::connect_trace;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::disconnect_trace;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::disconnect_adjacency;
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_architect_program_panels::standards::v1::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_architect_program_tests::standards::v1::subsets::any::io::binary::*;
    pub use semio_s_artifact_architect_program_catalog::standards::v1::subsets::any::io::binary::diff;
    pub use semio_s_artifact_architect_program_tests_2::standards::v1::subsets::any::io::binary::inferences;
    pub use semio_s_artifact_architect_program_mutations::standards::v1::subsets::any::io::binary::mutations;
    pub use semio_s_artifact_architect_program_snapshot::standards::v1::subsets::any::io::binary::snapshot;
}
pub mod sqlite {
    pub use semio_s_artifact_architect_program_tests_2::standards::v1::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_architect_program_snapshot::standards::v1::subsets::any::io::sqlite::snapshot;
}

                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_architect_program_snapshot::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_architect_program_tests_exchange::standards::v1::subsets::any::io::import::deserializers::artifacts::csv;
                                pub use semio_s_artifact_architect_program_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_architect_program_panels::standards::v1::subsets::any::io::export::serializers::artifacts::zip;
                                pub use semio_s_artifact_architect_program_snapshot::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_architect_program_tests_exchange::standards::v1::subsets::any::io::export::serializers::artifacts::csv;
                                pub use semio_s_artifact_architect_program_tests_exchange::standards::v1::subsets::any::io::export::serializers::artifacts::tsv;
                                pub use semio_s_artifact_architect_program_panels::standards::v1::subsets::any::io::export::serializers::artifacts::xlsx;
                                pub use semio_s_artifact_architect_program_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub use semio_s_artifact_architect_program_tests_2::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod architect {
        pub use semio_s_artifact_architect_program_architect::editor::architect::*;
pub use semio_s_artifact_architect_program_tests_2::editor::architect::demo_session;

        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_architect_program_config::editor::architect::config::*;
pub mod io {
    pub use semio_s_artifact_architect_program_tests_2::editor::architect::config::io::*;
    pub mod binary {
        pub use semio_s_artifact_architect_program_tests::editor::architect::config::io::binary::*;
        pub use semio_s_artifact_architect_program_config::editor::architect::config::io::binary::mutations;
    }
    pub mod text {
        pub use semio_s_artifact_architect_program_tests::editor::architect::config::io::text::*;
        pub use semio_s_artifact_architect_program_config::editor::architect::config::io::text::mutations;
    }
}

            pub use semio_s_artifact_architect_program_tests_2::editor::architect::config::schema;
        }

        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_architect_program_presence::editor::architect::presence::*;
pub mod io {
    pub use semio_s_artifact_architect_program_tests_2::editor::architect::presence::io::*;
    pub mod binary {
        pub use semio_s_artifact_architect_program_tests::editor::architect::presence::io::binary::*;
        pub use semio_s_artifact_architect_program_presence::editor::architect::presence::io::binary::mutations;
    }
    pub mod text {
        pub use semio_s_artifact_architect_program_tests::editor::architect::presence::io::text::*;
        pub use semio_s_artifact_architect_program_presence::editor::architect::presence::io::text::mutations;
    }
}

            pub use semio_s_artifact_architect_program_registers::editor::architect::presence::schema;
        }
        pub use semio_s_artifact_architect_program_tests_exchange::editor::architect::catalog;
        pub use semio_s_artifact_architect_program_snapshot::editor::architect::chrome;

        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_architect_program_tests_exchange::editor::architect::commands::adjacency;
            pub use semio_s_artifact_architect_program_tests_exchange::editor::architect::commands::analysis;
            pub use semio_s_artifact_architect_program_tests_exchange::editor::architect::commands::element;
            pub use semio_s_artifact_architect_program_mutations::editor::architect::commands::example;
            pub use semio_s_artifact_architect_program_tests_exchange::editor::architect::commands::exchange;
            pub use semio_s_artifact_architect_program_tests_exchange::editor::architect::commands::graph;
            pub use semio_s_artifact_architect_program_panels::editor::architect::commands::register;
            pub use semio_s_artifact_architect_program_mutations::editor::architect::commands::search;
            pub use semio_s_artifact_architect_program_panels::editor::architect::commands::template;
        }

        #[path = "."]
        pub mod modes {
            pub use semio_s_artifact_architect_program_tests_2::editor::architect::modes::report;
            pub use semio_s_artifact_architect_program_tests_2::editor::architect::modes::review;

            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_architect_program_tests_exchange::editor::architect::modes::edit::*;

                pub use semio_s_artifact_architect_program_tests_exchange::editor::architect::modes::edit::windows;
            }
        }

        pub use semio_s_artifact_architect_program_panels::editor::architect::panels;
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod architect {
        pub use semio_s_artifact_architect_program_mutations::viewer::architect::*;

        pub use semio_s_artifact_architect_program_tests_2::viewer::architect::presence;
        pub use semio_s_artifact_architect_program_tests_2::viewer::architect::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_architect_program_schema::viewer::architect::modes::view::*;

                pub use semio_s_artifact_architect_program_schema::viewer::architect::modes::view::windows;
            }
        }
    }
}

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
