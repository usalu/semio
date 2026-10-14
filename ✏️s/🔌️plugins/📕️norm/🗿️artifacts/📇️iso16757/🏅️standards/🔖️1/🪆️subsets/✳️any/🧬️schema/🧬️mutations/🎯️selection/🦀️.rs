//! 🎯️ ISO 16757 selection mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "."]
pub mod add_selection_constraint {
    #[path = "../🔒️add-selection-constraint/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔒️add-selection-constraint/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../🔒️add-selection-constraint/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod change_part_number_input {
    #[path = "../🎛️change-part-number-input/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🎛️change-part-number-input/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../🎛️change-part-number-input/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod change_selection_class {
    #[path = "../🎯️change-selection-class/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🎯️change-selection-class/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../🎯️change-selection-class/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod change_selection_series {
    #[path = "../🧵️change-selection-series/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🧵️change-selection-series/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../🧵️change-selection-series/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod remove_part_number_input {
    #[path = "../🔌️remove-part-number-input/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔌️remove-part-number-input/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../🔌️remove-part-number-input/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod remove_selection_constraint {
    #[path = "../🔓️remove-selection-constraint/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔓️remove-selection-constraint/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../🔓️remove-selection-constraint/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod replace_part_number_rule {
    #[path = "../🧮️replace-part-number-rule/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🧮️replace-part-number-rule/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../🧮️replace-part-number-rule/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
//#endregion 🪢️Mounts
