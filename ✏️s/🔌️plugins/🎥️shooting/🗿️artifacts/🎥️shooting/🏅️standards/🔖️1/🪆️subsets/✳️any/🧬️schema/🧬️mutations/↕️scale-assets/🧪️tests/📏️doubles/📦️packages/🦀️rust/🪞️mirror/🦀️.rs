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
                    pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_shooting_shooting_schema::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_shooting_shooting_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod scale_assets {
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests_doubles_asset_hero_scale;
                        }
                        #[path = "."]
                        pub mod create_shot {
                            #[cfg(test)]
                            #[path = "../../../../../../📸️create-shot/🧪️tests/📸️appends/🦀️.rs"]
                            pub mod tests_appends_shot_macro;
                        }
                        #[path = "."]
                        pub mod delete_shot {
                            #[cfg(test)]
                            #[path = "../../../../../../🚮️delete-shot/🧪️tests/🚫️removes/🦀️.rs"]
                            pub mod tests_removes_trailing_shot_close;
                        }
                        #[path = "."]
                        pub mod rename_shot {
                            #[cfg(test)]
                            #[path = "../../../../../../🏷️rename-shot/🧪️tests/🔤️relabels/🦀️.rs"]
                            pub mod tests_relabels_shot_close_to_detail;
                        }
                        #[path = "."]
                        pub mod change_shot_width {
                            #[cfg(test)]
                            #[path = "../../../../../../📏️change-shot-width/🧪️tests/↔️widens/🦀️.rs"]
                            pub mod tests_widens_shot_close_to_1024;
                        }
                        #[path = "."]
                        pub mod change_shot_height {
                            #[cfg(test)]
                            #[path = "../../../../../../📐️change-shot-height/🧪️tests/↕️heightens/🦀️.rs"]
                            pub mod tests_heightens_shot_close_to_768;
                        }
                        #[path = "."]
                        pub mod change_shot_format {
                            #[cfg(test)]
                            #[path = "../../../../../../🖼️change-shot-format/🧪️tests/🎨️switches/🦀️.rs"]
                            pub mod tests_switches_shot_wide_to_svg;
                        }
                        #[path = "."]
                        pub mod change_shot_shape {
                            #[cfg(test)]
                            #[path = "../../../../../../✂️change-shot-shape/🧪️tests/⭕️rounds/🦀️.rs"]
                            pub mod tests_rounds_shot_wide_to_ellipse;
                        }
                        #[path = "."]
                        pub mod reorder_shots {
                            #[cfg(test)]
                            #[path = "../../../../../../🔃️reorder-shots/🧪️tests/⬆️moves/🦀️.rs"]
                            pub mod tests_moves_shot_close_to_front;
                        }
                        #[path = "."]
                        pub mod replace_shot_camera {
                            #[cfg(test)]
                            #[path = "../../../../../../📷️replace-shot-camera/🧪️tests/📷️rewrites/🦀️.rs"]
                            pub mod tests_rewrites_cam_wide_through_shot_wide;
                        }
                        #[path = "."]
                        pub mod create_saved_camera {
                            #[cfg(test)]
                            #[path = "../../../../../../🎥️create-saved-camera/🧪️tests/🎥️appends/🦀️.rs"]
                            pub mod tests_appends_saved_camera_top;
                        }
                        #[path = "."]
                        pub mod delete_saved_camera {
                            #[cfg(test)]
                            #[path = "../../../../../../🧹️delete-saved-camera/🧪️tests/🚫️removes/🦀️.rs"]
                            pub mod tests_removes_trailing_cam_close;
                        }
                        #[path = "."]
                        pub mod rename_saved_camera {
                            #[cfg(test)]
                            #[path = "../../../../../../🪪️rename-saved-camera/🧪️tests/🔤️relabels/🦀️.rs"]
                            pub mod tests_relabels_cam_close_to_tight;
                        }
                        #[path = "."]
                        pub mod replace_saved_camera_view {
                            #[cfg(test)]
                            #[path = "../../../../../../🎞️replace-saved/🧪️tests/📍️repositions/🦀️.rs"]
                            pub mod tests_repositions_cam_close_view;
                        }
                        #[path = "."]
                        pub mod reorder_saved_cameras {
                            #[cfg(test)]
                            #[path = "../../../../../../🔁️reorder-saved-cameras/🧪️tests/🔁️moves/🦀️.rs"]
                            pub mod tests_moves_cam_close_to_front;
                        }
                        #[path = "."]
                        pub mod set_active_shot {
                            #[cfg(test)]
                            #[path = "../../../../../../🎯️set-active-shot/🧪️tests/🎯️activates/🦀️.rs"]
                            pub mod tests_activates_shot_close;
                        }
                        #[path = "."]
                        pub mod set_active_asset {
                            #[cfg(test)]
                            #[path = "../../../../../../📌️set-active-asset/🧪️tests/📌️activates/🦀️.rs"]
                            pub mod tests_activates_asset_prop;
                        }
                        #[path = "."]
                        pub mod change_scene_sun_enabled {
                            #[cfg(test)]
                            #[path = "../../../../../../☀️change-scene-sun/🧪️tests/☀️switches/🦀️.rs"]
                            pub mod tests_switches_scene_sun_off;
                        }
                        #[path = "."]
                        pub mod change_scene_sun_azimuth {
                            #[cfg(test)]
                            #[path = "../../../../../../🧭️change-scene-sun/🧪️tests/🧭️turns/🦀️.rs"]
                            pub mod tests_turns_scene_sun_to_315_degrees;
                        }
                        #[path = "."]
                        pub mod change_scene_sun_elevation {
                            #[cfg(test)]
                            #[path = "../../../../../../🌅️change-scene-sun/🧪️tests/🌅️raises/🦀️.rs"]
                            pub mod tests_raises_scene_sun_to_60_degrees;
                        }
                    }
                }
            }
        }
    }
}
