//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

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
                    pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::*;
                    #[path = "."]
                    pub mod authored {
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::formula;
                    }
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::inferences::*;
                        #[path = "."]
                        pub mod model_graph {
                            pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance;
                        }
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities;
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::inferences::phase_visibility;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families;
                        pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope;
                        #[path = "."]
                        pub mod element_solids {
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::*;
                        pub use semio_s_artifact_bim_model_mut_any_124::standards::v1::subsets::any::schema::mutations::create_site;
                        pub use semio_s_artifact_bim_model_mut_any_125::standards::v1::subsets::any::schema::mutations::create_building;
                        pub use semio_s_artifact_bim_model_mut_any_126::standards::v1::subsets::any::schema::mutations::create_storey;
                        pub use semio_s_artifact_bim_model_mut_any_173::standards::v1::subsets::any::schema::mutations::create_wall;
                        pub use semio_s_artifact_bim_model_mut_any_127::standards::v1::subsets::any::schema::mutations::create_railing;
                        pub use semio_s_artifact_bim_model_mut_any_128::standards::v1::subsets::any::schema::mutations::create_space;
                        pub use semio_s_artifact_bim_model_mut_any_159::standards::v1::subsets::any::schema::mutations::create_column;
                        pub use semio_s_artifact_bim_model_mut_any_160::standards::v1::subsets::any::schema::mutations::create_beam;
                        pub use semio_s_artifact_bim_model_mut_any_129::standards::v1::subsets::any::schema::mutations::create_slab;
                        pub use semio_s_artifact_bim_model_mut_any_130::standards::v1::subsets::any::schema::mutations::create_roof;
                        pub use semio_s_artifact_bim_model_mut_any_70::standards::v1::subsets::any::schema::mutations::create_column_type;
                        pub use semio_s_artifact_bim_model_mut_any_71::standards::v1::subsets::any::schema::mutations::create_beam_type;
                        pub use semio_s_artifact_bim_model_mut_any_72::standards::v1::subsets::any::schema::mutations::create_window_type;
                        pub use semio_s_artifact_bim_model_mut_any_73::standards::v1::subsets::any::schema::mutations::create_door_type;
                        pub use semio_s_artifact_bim_model_mut_any_74::standards::v1::subsets::any::schema::mutations::create_material;
                        pub use semio_s_artifact_bim_model_mut_any_75::standards::v1::subsets::any::schema::mutations::create_wall_type;
                        pub use semio_s_artifact_bim_model_mut_any_76::standards::v1::subsets::any::schema::mutations::create_slab_type;
                        pub use semio_s_artifact_bim_model_mut_any_77::standards::v1::subsets::any::schema::mutations::create_roof_type;
                        pub use semio_s_artifact_bim_model_mut_any_175::standards::v1::subsets::any::schema::mutations::create_opening;
                        pub use semio_s_artifact_bim_model_mut_any_131::standards::v1::subsets::any::schema::mutations::create_stair;
                        pub use semio_s_artifact_bim_model_mut_any_132::standards::v1::subsets::any::schema::mutations::create_grid_line;
                        pub use semio_s_artifact_bim_model_mut_any_144::standards::v1::subsets::any::schema::mutations::set_element_storey;
                        pub use semio_s_artifact_bim_model_mut_any_103::standards::v1::subsets::any::schema::mutations::set_element_phase;
                        pub use semio_s_artifact_bim_model_mut_any_163::standards::v1::subsets::any::schema::mutations::set_beam_axis;
                        pub use semio_s_artifact_bim_model_mut_any_164::standards::v1::subsets::any::schema::mutations::set_column_tilt;
                        pub use semio_s_artifact_bim_model_mut_any_165::standards::v1::subsets::any::schema::mutations::create_curtain_wall_type;
                        pub use semio_s_artifact_bim_model_mut_any_166::standards::v1::subsets::any::schema::mutations::set_curtain_wall_type;
                        pub use semio_s_artifact_bim_model_mut_any_165::standards::v1::subsets::any::schema::mutations::delete_curtain_wall_type;
                        pub use semio_s_artifact_bim_model_mut_any_39::standards::v1::subsets::any::schema::mutations::set_curtain_wall_type_of;
                        pub use semio_s_artifact_bim_model_mut_any_167::standards::v1::subsets::any::schema::mutations::set_curtain_wall_grid;
                        pub use semio_s_artifact_bim_model_mut_any_168::standards::v1::subsets::any::schema::mutations::create_curtain_panel_override;
                        pub use semio_s_artifact_bim_model_mut_any_169::standards::v1::subsets::any::schema::mutations::set_curtain_panel_override;
                        pub use semio_s_artifact_bim_model_mut_any_107::standards::v1::subsets::any::schema::mutations::delete_curtain_panel_override;
                        pub use semio_s_artifact_bim_model_mut_any_170::standards::v1::subsets::any::schema::mutations::set_beam;
                        pub use semio_s_artifact_bim_model_mut_any_172::standards::v1::subsets::any::schema::mutations::create_curtain_wall;
                        pub use semio_s_artifact_bim_model_mut_any_42::standards::v1::subsets::any::schema::mutations::set_family;
                        pub use semio_s_artifact_bim_model_mut_any_109::standards::v1::subsets::any::schema::mutations::set_family_parameter;
                        pub use semio_s_artifact_bim_model_mut_any_109::standards::v1::subsets::any::schema::mutations::remove_family_parameter;
                        pub use semio_s_artifact_bim_model_mut_any_110::standards::v1::subsets::any::schema::mutations::create_family_solid;
                        pub use semio_s_artifact_bim_model_mut_any_110::standards::v1::subsets::any::schema::mutations::delete_family_solid;
                        pub use semio_s_artifact_bim_model_mut_any_43::standards::v1::subsets::any::schema::mutations::set_family_solid;
                        pub use semio_s_artifact_bim_model_mut_any_57::standards::v1::subsets::any::schema::mutations::set_type_thermal_data;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::io::text::snapshot;
                    }
                    #[path = "../../../../🚪️io/📥️import/🦀️.rs"]
                    pub mod import;
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod ifc {
                            pub use semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::*;
                            pub use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::io::export::ifc::context;
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer;
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::io::export::ifc::brep;
                            pub use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::io::export::ifc::data;
                            #[cfg(test)]
                            #[path = "../../../../🚪️io/📤️export/🏗️ifc/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                            #[cfg(test)]
                            #[path = "../../../../🚪️io/📤️export/🏗️ifc/🧪️tests/🔭️schema4/🦀️.rs"]
                            pub mod schema4_tests;
                            #[path = "."]
                            pub mod energy {
                                pub use semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::energy::*;
                                #[cfg(test)]
                                #[path = "../../../../🚪️io/📤️export/🏗️ifc/🔥️energy/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        pub use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::io::export::holders;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bim {
        #[path = "."]
        pub mod entities {
            #[path = "../../../../✏️editor/🧩️entities/🧱️row-types/🦀️.rs"]
            pub mod row_types;
            pub use semio_s_artifact_bim_model_authored::editor::bim::entities::field_codecs;
            #[path = "../../../../✏️editor/🧩️entities/🕰️phasing/🦀️.rs"]
            pub mod phasing;
            #[path = "../../../../✏️editor/🧩️entities/🏗️frame/🦀️.rs"]
            pub mod frame;
            #[path = "../../../../✏️editor/🧩️entities/➕️create-rows/🦀️.rs"]
            pub mod create_rows;
        }
        pub use semio_s_artifact_bim_model_tests::editor::bim::terminology;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod family {
                        #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧬️family/✏️edit/🦀️.rs"]
                        pub mod edit;
                        #[path = "."]
                        pub mod vocabulary {
                            pub use semio_s_artifact_bim_model_wall_layout::editor::bim::modes::edit::windows::family::vocabulary::*;
                            #[cfg(test)]
                            #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧬️family/🏷️vocabulary/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod bim {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_bim_model_tests::viewer::bim::terminology;
        #[path = "."]
        pub mod modes {
            pub use semio_s_artifact_bim_model_family_tests::viewer::bim::modes::*;
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_bim_model_family_tests::viewer::bim::modes::view::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_bim_model_family_tests::viewer::bim::modes::view::windows::*;
                    #[path = "."]
                    pub mod plan {
                        pub use semio_s_artifact_bim_model_family_tests::viewer::bim::modes::view::windows::plan::*;
                        #[cfg(test)]
                        #[path = "../../../🎭️modes/👁️view/🪟️windows/🗺️plan/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod world {
                        pub use semio_s_artifact_bim_model_family_tests::viewer::bim::modes::view::windows::world::*;
                        #[cfg(test)]
                        #[path = "../../../🎭️modes/👁️view/🪟️windows/🧊️world/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod render {
    pub use semio_s_artifact_bim_model_tests_5::render::*;
    pub use semio_s_artifact_bim_model_plan_linework::render::plan;
}
