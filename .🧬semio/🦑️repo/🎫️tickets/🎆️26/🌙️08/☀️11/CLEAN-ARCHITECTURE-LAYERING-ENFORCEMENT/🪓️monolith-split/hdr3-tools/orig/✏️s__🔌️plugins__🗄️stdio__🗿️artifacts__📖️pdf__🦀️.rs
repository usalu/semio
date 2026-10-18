//! 🎪 `stdio.pdf` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

use standards::v1_7::subsets::base::schema::mutations::PdfMutation;
use standards::v1_7::subsets::base::schema::snapshot::{PdfSnapshot, STDIO_PDF17_DOCUMENT_SCHEMA};
use standards::v1_4::subsets::base::schema::snapshot::STDIO_PDF_DOCUMENT_SCHEMA;

#[path = "🧾️artifact-declaration/🦀️.rs"]
mod artifact_declaration;
pub use artifact_declaration::*;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_4 {
        pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_4::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_schema::standards::v1_4::subsets::base::schema::*;

                    pub use semio_s_artifact_stdio_pdf_snapshot::standards::v1_4::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_pdf_schema::standards::v1_4::subsets::base::schema::inferences;
                    pub use semio_s_artifact_stdio_pdf_diff::standards::v1_4::subsets::base::schema::diff;
                    pub use semio_s_artifact_stdio_pdf_mut_base::standards::v1_4::subsets::base::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_pdf_graph_projection::standards::v1_4::subsets::base::io::*;
pub mod sqlite {
    pub use semio_s_artifact_stdio_pdf_model::standards::v1_4::subsets::base::io::sqlite::*;
    pub use semio_s_artifact_stdio_pdf_snapshot::standards::v1_4::subsets::base::io::sqlite::snapshot;
}

                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_4::subsets::base::io::import::deserializers::artifacts::binary;
                                pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_4::subsets::base::io::import::deserializers::artifacts::deflate;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_4::subsets::base::io::export::serializers::artifacts::binary;
                                pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_4::subsets::base::io::export::serializers::artifacts::deflate;
                            }
                        }
                    }
                }
            }
            #[path = "."]
            pub mod a {
                // 🏅️ PDF/A (1.4) -- ISO 19005-1 (PDF/A-1), the honestly-scope-limited
                // reference case: `PdfSnapshot`(1.4) is a bare PageDoc, no object graph.
                // Added in ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W2.
                pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_4::subsets::a::io;
                pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_4::subsets::a::schema;
            }
            #[path = "."]
            pub mod x {
                // 🏅️ PDF/X (1.4) -- ISO 15930-1 (X-1a) / ISO 15930-3 (X-3), same
                // honestly-scope-limited schema-gap shape as ✳️a above. Added in ticket
                // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W2.
                pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_4::subsets::x::io;
                pub use semio_s_artifact_stdio_pdf_schema_3::standards::v1_4::subsets::x::schema;
            }
        }
    }

    #[path = "."]
    pub mod v1_7 {
        pub use semio_s_artifact_stdio_pdf_pdf_snapshot_value_tests::standards::v1_7::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                pub use semio_s_artifact_stdio_pdf_testkit::standards::v1_7::subsets::base::*;

                #[path = "."]
                pub mod modules {
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::lexer;
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::filters;
                    pub use semio_s_artifact_stdio_pdf_graph_projection::standards::v1_7::subsets::base::modules::xref;
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::encryption;
                    pub use semio_s_artifact_stdio_pdf_fonts::standards::v1_7::subsets::base::modules::fonts;
                    pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::base::modules::content;
                    pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::modules::colour;
                    pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::modules::images;
                    pub use semio_s_artifact_stdio_pdf_lift::standards::v1_7::subsets::base::modules::lift;
                    pub use semio_s_artifact_stdio_pdf_lower::standards::v1_7::subsets::base::modules::lower;
                    pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::modules::writer;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_schema_4::standards::v1_7::subsets::base::schema::*;

                    pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::schema::inferences;
                    pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::diff;
                    pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_pdf_tests_2::standards::v1_7::subsets::base::io::*;

                    #[path = "."]
                    pub mod import {
                        pub use semio_s_artifact_stdio_pdf_mut_family_11::standards::v1_7::subsets::base::io::import::*;

                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_stdio_pdf_tests_2::standards::v1_7::subsets::base::io::import::deserializers::artifacts::binary;
                                pub use semio_s_artifact_stdio_pdf_tests_2::standards::v1_7::subsets::base::io::import::deserializers::artifacts::deflate;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        pub use semio_s_artifact_stdio_pdf_testkit::standards::v1_7::subsets::base::io::export::*;

                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_stdio_pdf_testkit::standards::v1_7::subsets::base::io::export::serializers::artifacts::binary;
                                pub use semio_s_artifact_stdio_pdf_testkit::standards::v1_7::subsets::base::io::export::serializers::artifacts::deflate;
                            }
                        }
                    }
                }
            }
            #[path = "."]
            pub mod a {
                // 🏅️ PDF/A (ISO 19005-2/-3) — the FIRST real, non-✳️any subset in the
                // repo. Restructured from `✳️a-2b` in ticket
                // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W2: the conformance
                // LEVEL (2b/2u/3b/3u) is analyzer-detected DATA (`stdio.pdf.a.level`), not
                // part of the subset id. `schema` re-exports the ✳️any subset's
                // `PdfSnapshot` verbatim (same Rust type, same `s.stdio.pdf.1.7` schema
                // id); `io` reuses the ✳️any subset's binary/deflate DAG leaves rather than
                // duplicating them.
                pub use semio_s_artifact_stdio_pdf_mut_family_12::standards::v1_7::subsets::a::io;
                pub use semio_s_artifact_stdio_pdf_mut_family_15::standards::v1_7::subsets::a::schema;
            }
            #[path = "."]
            pub mod x {
                // 🏅️ PDF/X-4 -- ISO 15930-7:2010, based on PDF 1.6/1.7. Real
                // object-graph-backed analyzer/composer/builder (same shape as ✳️a).
                // Added in ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.
                pub use semio_s_artifact_stdio_pdf_mut_family_12::standards::v1_7::subsets::x::io;
                pub use semio_s_artifact_stdio_pdf_mut_family_15::standards::v1_7::subsets::x::schema;
            }
            #[path = "."]
            pub mod e {
                // 🏅️ PDF/E-1 -- ISO 24517-1:2008, based on PDF 1.6. Added in ticket
                // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.
                pub mod io {
    pub use semio_s_artifact_stdio_pdf_mut_family_13::standards::v1_7::subsets::e::io::*;
    pub mod binary {
        pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::e::io::binary::*;
        pub use semio_s_artifact_stdio_pdf_mut_family_9::standards::v1_7::subsets::e::io::binary::mutations;
    }
    pub mod text {
        pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::e::io::text::*;
        pub use semio_s_artifact_stdio_pdf_mut_family_9::standards::v1_7::subsets::e::io::text::mutations;
    }
}
                pub use semio_s_artifact_stdio_pdf_mut_family_15::standards::v1_7::subsets::e::schema;
            }
            #[path = "."]
            pub mod ua {
                // 🏅️ PDF/UA-1 -- ISO 14289-1:2014. Added in ticket
                // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.
                pub use semio_s_artifact_stdio_pdf_mut_family_14::standards::v1_7::subsets::ua::io;
                pub use semio_s_artifact_stdio_pdf_mut_family_15::standards::v1_7::subsets::ua::schema;
            }
            #[path = "."]
            pub mod vt {
                // 🏅️ PDF/VT-1/-2 -- ISO 16612-2:2010, layered on PDF/X-4 (ISO 15930-7):
                // this subset's analyzer calls `x::analyzer::check_x_conformance`
                // directly rather than duplicating those checks. Added in ticket
                // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.
                pub use semio_s_artifact_stdio_pdf_schema_5::standards::v1_7::subsets::vt::io;
                pub use semio_s_artifact_stdio_pdf_schema_5::standards::v1_7::subsets::vt::schema;
            }
            #[path = "."]
            pub mod h {
                // 🏅️ PDF/H -- AIIM/ASTM PDF Healthcare Best Practices Guide (2008);
                // industry best-practice, never ISO; all-soft profile, no hard checks,
                // composer always Ok (pass-through). Added in ticket
                // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.
                pub mod io {
    pub use semio_s_artifact_stdio_pdf_mut_family_15::standards::v1_7::subsets::h::io::*;
    pub mod binary {
        pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::h::io::binary::*;
        pub use semio_s_artifact_stdio_pdf_mut_family_11::standards::v1_7::subsets::h::io::binary::mutations;
    }
    pub mod text {
        pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::h::io::text::*;
        pub use semio_s_artifact_stdio_pdf_mut_family_11::standards::v1_7::subsets::h::io::text::mutations;
    }
}
                pub use semio_s_artifact_stdio_pdf_schema_5::standards::v1_7::subsets::h::schema;
            }
        }
    }
}

#[path = "."]
pub mod examples {
    pub use semio_s_artifact_stdio_pdf_model::examples::demo;
    #[path = "."]
    pub mod bachelor_thesis {
        pub use semio_s_artifact_stdio_pdf_testkit::examples::bachelor_thesis::*;

    }
}

#[path = "."]
pub mod editor {
    pub use semio_s_artifact_stdio_pdf_tests_2::editor::page;
    #[path = "."]
    pub mod pdf14a {
        pub use semio_s_artifact_stdio_pdf_schema_5::editor::pdf14a::*;

        pub use semio_s_artifact_stdio_pdf_model::editor::pdf14a::presence;
        pub use semio_s_artifact_stdio_pdf_model::editor::pdf14a::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_pdf_schema_3::editor::pdf14a::modes::edit::*;

                pub use semio_s_artifact_stdio_pdf_schema_3::editor::pdf14a::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf14 {
        pub use semio_s_artifact_stdio_pdf_schema_3::editor::pdf14::page;
        pub use semio_s_artifact_stdio_pdf_schema_5::editor::pdf14::*;

        pub use semio_s_artifact_stdio_pdf_model::editor::pdf14::presence;
        pub use semio_s_artifact_stdio_pdf_model::editor::pdf14::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_pdf_schema_3::editor::pdf14::modes::edit::*;

                pub use semio_s_artifact_stdio_pdf_schema_3::editor::pdf14::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf14x {
        pub use semio_s_artifact_stdio_pdf_schema_5::editor::pdf14x::*;

        pub use semio_s_artifact_stdio_pdf_model::editor::pdf14x::presence;
        pub use semio_s_artifact_stdio_pdf_model::editor::pdf14x::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_pdf_schema_3::editor::pdf14x::modes::edit::*;

                pub use semio_s_artifact_stdio_pdf_schema_3::editor::pdf14x::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17a {
        pub use semio_s_artifact_stdio_pdf_schema_5::editor::pdf17a::*;

        pub use semio_s_artifact_stdio_pdf_model::editor::pdf17a::presence;
        pub use semio_s_artifact_stdio_pdf_snapshot::editor::pdf17a::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17a::modes::edit::*;

                pub use semio_s_artifact_stdio_pdf_tests_2::editor::pdf17a::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17 {
        pub use semio_s_artifact_stdio_pdf_schema_5::editor::pdf17::*;

        pub use semio_s_artifact_stdio_pdf_snapshot::editor::pdf17::presence;
        pub use semio_s_artifact_stdio_pdf_snapshot::editor::pdf17::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_pdf_schema_4::editor::pdf17::modes::edit::*;

                pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17e {
        pub use semio_s_artifact_stdio_pdf_schema_5::editor::pdf17e::*;

        pub use semio_s_artifact_stdio_pdf_snapshot::editor::pdf17e::presence;
        pub use semio_s_artifact_stdio_pdf_snapshot::editor::pdf17e::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_pdf_schema_4::editor::pdf17e::modes::edit::*;

                pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17e::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17h {
        pub use semio_s_artifact_stdio_pdf_schema_5::editor::pdf17h::*;

        pub use semio_s_artifact_stdio_pdf_snapshot::editor::pdf17h::presence;
        pub use semio_s_artifact_stdio_pdf_snapshot::editor::pdf17h::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_pdf_schema_4::editor::pdf17h::modes::edit::*;

                pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17h::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17ua {
        pub use semio_s_artifact_stdio_pdf_schema_5::editor::pdf17ua::*;

        pub use semio_s_artifact_stdio_pdf_snapshot::editor::pdf17ua::presence;
        pub use semio_s_artifact_stdio_pdf_snapshot::editor::pdf17ua::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_pdf_schema_4::editor::pdf17ua::modes::edit::*;

                pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17ua::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17vt {
        pub use semio_s_artifact_stdio_pdf_pdf_snapshot_value_tests::editor::pdf17vt::*;

        pub use semio_s_artifact_stdio_pdf_snapshot::editor::pdf17vt::presence;
        pub use semio_s_artifact_stdio_pdf_snapshot::editor::pdf17vt::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_pdf_schema_4::editor::pdf17vt::modes::edit::*;

                pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17vt::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17x {
        pub use semio_s_artifact_stdio_pdf_schema_5::editor::pdf17x::*;

        pub use semio_s_artifact_stdio_pdf_snapshot::editor::pdf17x::presence;
        pub use semio_s_artifact_stdio_pdf_snapshot::editor::pdf17x::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_pdf_schema_4::editor::pdf17x::modes::edit::*;

                pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17x::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod pdf14a {
        pub use semio_s_artifact_stdio_pdf_schema_5::viewer::pdf14a::*;

        pub use semio_s_artifact_stdio_pdf_snapshot::viewer::pdf14a::presence;
        pub use semio_s_artifact_stdio_pdf_diff::viewer::pdf14a::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_pdf_tests::viewer::pdf14a::modes::view::*;

                pub use semio_s_artifact_stdio_pdf_tests::viewer::pdf14a::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf14 {
        pub use semio_s_artifact_stdio_pdf_schema_5::viewer::pdf14::*;

        pub use semio_s_artifact_stdio_pdf_diff::viewer::pdf14::presence;
        pub use semio_s_artifact_stdio_pdf_diff::viewer::pdf14::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_pdf_tests::viewer::pdf14::modes::view::*;

                pub use semio_s_artifact_stdio_pdf_tests::viewer::pdf14::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf14x {
        pub use semio_s_artifact_stdio_pdf_mut_family_15::viewer::pdf14x::*;

        pub use semio_s_artifact_stdio_pdf_diff::viewer::pdf14x::presence;
        pub use semio_s_artifact_stdio_pdf_diff::viewer::pdf14x::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_pdf_schema_2::viewer::pdf14x::modes::view::*;

                pub use semio_s_artifact_stdio_pdf_tests::viewer::pdf14x::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17a {
        pub use semio_s_artifact_stdio_pdf_pdf_snapshot_value_tests::viewer::pdf17a::*;

        pub use semio_s_artifact_stdio_pdf_diff::viewer::pdf17a::presence;
        pub use semio_s_artifact_stdio_pdf_diff::viewer::pdf17a::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17a::modes::view::*;

                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17a::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17 {
        pub use semio_s_artifact_stdio_pdf_pdf_snapshot_value_tests::viewer::pdf17::*;

        pub use semio_s_artifact_stdio_pdf_diff::viewer::pdf17::presence;
        pub use semio_s_artifact_stdio_pdf_diff::viewer::pdf17::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17::modes::view::*;

                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17e {
        pub use semio_s_artifact_stdio_pdf_pdf_snapshot_value_tests::viewer::pdf17e::*;

        pub use semio_s_artifact_stdio_pdf_diff::viewer::pdf17e::presence;
        pub use semio_s_artifact_stdio_pdf_diff::viewer::pdf17e::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17e::modes::view::*;

                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17e::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17h {
        pub use semio_s_artifact_stdio_pdf_pdf_snapshot_value_tests::viewer::pdf17h::*;

        pub use semio_s_artifact_stdio_pdf_diff::viewer::pdf17h::presence;
        pub use semio_s_artifact_stdio_pdf_diff::viewer::pdf17h::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_pdf_graph_projection::viewer::pdf17h::modes::view::*;

                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17h::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17ua {
        pub use semio_s_artifact_stdio_pdf_pdf_snapshot_value_tests::viewer::pdf17ua::*;

        pub use semio_s_artifact_stdio_pdf_diff::viewer::pdf17ua::presence;
        pub use semio_s_artifact_stdio_pdf_diff::viewer::pdf17ua::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_pdf_graph_projection::viewer::pdf17ua::modes::view::*;

                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17ua::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17vt {
        pub use semio_s_artifact_stdio_pdf_viewer::viewer::pdf17vt::*;

        pub use semio_s_artifact_stdio_pdf_diff::viewer::pdf17vt::presence;
        pub use semio_s_artifact_stdio_pdf_diff::viewer::pdf17vt::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_pdf_graph_projection::viewer::pdf17vt::modes::view::*;

                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17vt::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17x {
        pub use semio_s_artifact_stdio_pdf_pdf_snapshot_value_tests::viewer::pdf17x::*;

        pub use semio_s_artifact_stdio_pdf_diff::viewer::pdf17x::presence;
        pub use semio_s_artifact_stdio_pdf_diff::viewer::pdf17x::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_pdf_graph_projection::viewer::pdf17x::modes::view::*;

                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17x::modes::view::windows;
            }
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod root_tests;

#[cfg(test)]
#[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_0;
#[cfg(test)]
#[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🔬️page-domain/🦀️.rs"]
mod hoisted_tests_1;
#[cfg(test)]
#[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_2;
