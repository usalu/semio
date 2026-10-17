//! 🍰️ Companion test crate of `semio-s-artifact-layout-layout-mutations-frames`: the mutation fixture laws of the frame leaves, part two. The test files stay at their taxonomy nodes and are mounted here in the module nest the parent used.

#![cfg(test)]

extern crate semio_framework_os_kernel as protocol;

#[path = "."]
pub mod change_frame_columns {
    #[path = "../../../🔢change-frame-columns/🧪️tests/🔤️splits/🦀️.rs"]
    pub mod tests_splits_the_text_frame_into_two_columns;
}
#[path = "."]
pub mod change_frame_stroke {
    #[path = "../../../🖊️change-frame-stroke/🧪️tests/🖊️adds/🦀️.rs"]
    pub mod tests_adds_a_stroke_to_the_rect_frame;
}
#[path = "."]
pub mod change_frame_wrap_mode {
    #[path = "../../../🔤change-frame-wrap-mode/🧪️tests/🔤️switches/🦀️.rs"]
    pub mod tests_switches_the_text_frame_to_column_wrap;
}
#[path = "."]
pub mod move_frame {
    #[path = "../../../🕹️move-frame/🧪️tests/📍️moves-the-rect-frame/🦀️.rs"]
    pub mod tests_moves_the_rect_frame;
}
#[path = "."]
pub mod rotate_frame {
    #[path = "../../../🔄️rotate-frame/🧪️tests/🌀️rotates/🦀️.rs"]
    pub mod tests_rotates_the_rect_frame;
}
#[path = "."]
pub mod rotate_frames {
    #[path = "../../../🔃️rotate-frames/🧪️tests/🔃️orbits/🦀️.rs"]
    pub mod tests_orbits_both_frames_a_quarter_turn;
    #[path = "../../../🔃️rotate-frames/🧪️tests/🚫️rejects/🦀️.rs"]
    pub mod tests_rejects_a_missing_page;
}
#[path = "."]
pub mod scale_frames {
    #[path = "../../../🗜️scale-frames/🧪️tests/🗜️doubles/🦀️.rs"]
    pub mod tests_doubles_both_frames_about_their_centroid;
    #[path = "../../../🗜️scale-frames/🧪️tests/⏸️keeps-unit-factors/🦀️.rs"]
    pub mod tests_keeps_unit_factors;
    #[path = "../../../🗜️scale-frames/🧪️tests/🫓️refuses/🦀️.rs"]
    pub mod tests_refuses_a_zero_factor;
    #[path = "../../../🗜️scale-frames/🧪️tests/🚫️rejects/🦀️.rs"]
    pub mod tests_rejects_missing_frames;
    #[path = "../../../🗜️scale-frames/🧪️tests/⚠️skips/🦀️.rs"]
    pub mod tests_skips_a_locked_and_a_missing_frame;
    #[path = "../../../🗜️scale-frames/🧪️tests/↔️stretches/🦀️.rs"]
    pub mod tests_stretches_the_rect_sideways;
}
#[path = "."]
pub mod set_frame_flags {
    #[path = "../../../🔒set-frame-flags/🧪️tests/🔒️locks-frame-1/🦀️.rs"]
    pub mod tests_locks_frame_1;
}
