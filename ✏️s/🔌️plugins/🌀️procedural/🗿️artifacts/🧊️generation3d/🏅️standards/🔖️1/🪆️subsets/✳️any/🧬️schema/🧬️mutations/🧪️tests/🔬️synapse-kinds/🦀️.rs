//! 🧪️ Kind fixture laws of `connect-synapse`, `update-synapse`, `disconnect-synapse`, `delete-widget-position`: each committed scenario applied, inverted and diffed through the mutation union.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../🔗️connect-synapse/🧪️tests/🔌️wires/🦀️.rs"]
mod connect_synapse_wires;
#[path = "../../🔄️update-synapse/🧪️tests/📡️repoints/🦀️.rs"]
mod update_synapse_repoints;
#[path = "../../✂️disconnect-synapse/🧪️tests/✂️cuts/🦀️.rs"]
mod disconnect_synapse_cuts;
#[path = "../../✂️disconnect-synapse/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod disconnect_synapse_removes_a_middle_row;
#[path = "../../🧹️delete-widget-position/🧪️tests/🧹️unpins/🦀️.rs"]
mod delete_widget_position_unpins;
