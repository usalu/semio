//! 🧪️ Kind fixture laws of `connect-synapse`, `disconnect-synapse`, `replace-synapse`: each committed scenario applied, inverted and diffed through the mutation union.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../✂️disconnect/🧪️tests/✂️severs/🦀️.rs"]
mod disconnect_severs_link_ab_leaving_both_notes;
#[path = "../../✂️disconnect/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod disconnect_removes_a_middle_row;
#[path = "../../🔗️connect-synapse/🧪️tests/🔗️joins/🦀️.rs"]
mod connect_synapse_joins_note_b_to_note_c_at_index_1;
#[path = "../../🔄️replace/🧪️tests/🔗️repoints/🦀️.rs"]
mod replace_repoints_link_ab_onto_the_alt_port;
