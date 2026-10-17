//! 🍯️ Companion test crate of `semio-s-artifact-layout-layout-mutations-frames`: the mutation fixture laws of the frame leaves, part one. The test files stay at their taxonomy nodes and are mounted here in the module nest the parent used.

#![cfg(test)]

extern crate semio_framework_os_kernel as protocol;

#[path = "."]
pub mod change_frame_fill {
    #[path = "../../../🎨change-frame-fill/🧪️tests/🎨️repaints/🦀️.rs"]
    pub mod tests_repaints_the_rect_frame_fill;
}
#[path = "."]
pub mod create_frame {
    #[path = "../../../➕create-frame/🧪️tests/🔲️inserts/🦀️.rs"]
    pub mod tests_inserts_a_rect_frame_at_index_1;
}
#[path = "."]
pub mod delete_frame {
    #[path = "../../../➖delete-frame/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
    pub mod tests_removes_a_middle_row;
    #[path = "../../../➖delete-frame/🧪️tests/🚫️removes/🦀️.rs"]
    pub mod tests_removes_the_text_frame_and_its_layer_membership;
}
#[path = "."]
pub mod drag_frames {
    #[path = "../../../✋️drag-frames/🧪️tests/✋️drags-both-frames/🦀️.rs"]
    pub mod tests_drags_both_frames;
    #[path = "../../../✋️drag-frames/🧪️tests/⏸️keeps-a-zero-offset/🦀️.rs"]
    pub mod tests_keeps_a_zero_offset;
    #[path = "../../../✋️drag-frames/🧪️tests/🔁️refuses/🦀️.rs"]
    pub mod tests_refuses_a_repeated_frame;
    #[path = "../../../✋️drag-frames/🧪️tests/🚫️rejects/🦀️.rs"]
    pub mod tests_rejects_missing_frames;
    #[path = "../../../✋️drag-frames/🧪️tests/⚠️skips/🦀️.rs"]
    pub mod tests_skips_a_locked_and_a_missing_frame;
}
#[path = "."]
pub mod resize_frame {
    #[path = "../../../📏resize-frame/🧪️tests/📐️resizes-the-rect-frame/🦀️.rs"]
    pub mod tests_resizes_the_rect_frame;
}
#[path = "."]
pub mod rotate_frames {
    #[path = "../../../🔃️rotate-frames/🧪️tests/⏸️keeps-a-zero-angle/🦀️.rs"]
    pub mod tests_keeps_a_zero_angle;
    #[path = "../../../🔃️rotate-frames/🧪️tests/🔁️refuses/🦀️.rs"]
    pub mod tests_refuses_a_repeated_frame;
    #[path = "../../../🔃️rotate-frames/🧪️tests/⚠️skips/🦀️.rs"]
    pub mod tests_skips_a_locked_and_a_missing_frame;
    #[path = "../../../🔃️rotate-frames/🧪️tests/🌀️turns/🦀️.rs"]
    pub mod tests_turns_the_rect_about_its_centre;
}
#[path = "."]
pub mod update_grid {
    #[path = "../../../📐update-grid/🧪️tests/📐️sets/🦀️.rs"]
    pub mod tests_sets_an_18_point_baseline;
}
