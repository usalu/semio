//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.


#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bim {
        
        
        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub mod families {
                #[cfg(test)]
                #[path = "../../../🦀️.rs"]
                pub mod tests;
            }
        }
        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub mod create_entity {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🏗️create-entity/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod create_view {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🔭️create-view/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod delete_selection {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🗑️delete-selection/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod rename_entity {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🏷️rename-entity/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod set_field {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🩹️set-field/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod set_view {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🪟️set-view/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod edit_family {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🧬️edit-family/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod edit_schedule {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/📋️edit-schedule/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod export_schedule_csv {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/📊️export-schedule-csv/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod analyse_model {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🔎️analyse-model/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod export_model {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/📤️export-model/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod export_sheets {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/📄️export-sheets/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod set_camera {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🎥️set-camera/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod canvas_pointer_down {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🖱️canvas-pointer-down/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod canvas_commit_draft {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/✅️canvas-commit-draft/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod canvas_escape {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🚪️canvas-escape/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod world_pointer_down {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🌍️world-pointer-down/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod world_pointer_move {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🌐️world-pointer-move/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod arm_utility {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🛠️arm-utility/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod flip_walls {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🔃️flip-walls/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod attach_walls {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🔗️attach-walls/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod move_storey {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🪜️move-storey/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod split_wall {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/✂️split-wall/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod set_property {
                #[cfg(test)]
                #[path = "../../../../../../../🎮️commands/🧾️set-property/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
    }
}
