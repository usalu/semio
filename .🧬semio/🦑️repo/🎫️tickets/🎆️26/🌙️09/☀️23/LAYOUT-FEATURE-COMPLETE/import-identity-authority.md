# Import Identity Authority

The actual original transaction identity owner is os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority. It exposes encode(|control| ...), carrying the original observer and cumulative receipt. Inside that closure, entity_identity::edit_scoped_id(edit_id: &str, ordinal: u32, control: &mut NativeEncodeControl) produces the requested identity. There is no authority.edit_scoped_id method. Child and artifact identities require distinct original transaction ordinals. The import job must receive its original authoring seed and authority from the owning host; the existing Layout structure collision-loop helper is not a birth issuer.

Source paths: framework/products/os/modules/vcs/io/binary/entity-identity/control and entity-identity native implementation (emoji taxonomy in repository). This is source inspection, not native execution.
