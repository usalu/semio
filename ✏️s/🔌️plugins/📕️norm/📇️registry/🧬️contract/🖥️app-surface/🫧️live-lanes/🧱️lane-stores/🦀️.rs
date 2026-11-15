//! 🧱️ Lane stores: the framework `Presence` and `Transient` facets of every live norm editor, each a single opaque `state` line that is the canonical JSON of [`NormPresence`] / [`NormTransient`]. See [`super`].

use super::{NormPresence, NormTransient};
use semio_framework_value::{FromValue, ToValue};

#[doc(hidden)]
pub use pack as __pack;

#[doc(hidden)]
pub use semio_framework_pack_json as __pack_json;

#[doc(hidden)]
pub use semio_plugin as __plugin;

/// 🔄️ The canonical text of a lane payload; ephemeral lanes never persist, so the text is the only wire shape.
fn encode<T: ToValue>(value: &T) -> String {
    semio_framework_pack_json::to_json_string(&value.to_value())
}

fn decode<T: FromValue + Default>(state: &str) -> T {
    semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(state, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().and_then(|value| T::from_value(value).ok()).unwrap_or_default()
}

/// 🫧️ A framework `Transient` record that carries a [`NormTransient`].
pub trait LaneTransient: Sized {
    type Mutation;
    fn lane(&self) -> NormTransient;
    fn replace(lane: &NormTransient) -> Self::Mutation;
}

/// 👥️ A framework `Presence` record that carries a [`NormPresence`].
pub trait LanePresence: Sized {
    type Mutation;
    fn lane(&self) -> NormPresence;
    fn replace(lane: &NormPresence) -> Self::Mutation;
}

/// 🔄️ Canonical lane text of a transient.
pub fn transient_state(lane: &NormTransient) -> String {
    encode(lane)
}

/// 🔄️ Lane text read back; unreadable text is the empty transient (no preview).
pub fn transient_of_state(state: &str) -> NormTransient {
    decode(state)
}

/// 🔄️ Canonical lane text of a presence.
pub fn presence_state(lane: &NormPresence) -> String {
    encode(lane)
}

/// 🔄️ Lane text read back; unreadable text is the anonymous presence.
pub fn presence_of_state(state: &str) -> NormPresence {
    decode(state)
}

impl Default for NormPresence {
    fn default() -> Self {
        Self::new("")
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __norm_lane_store {
    ($module:ident, $record:ident, $diff:ident, $mutation:ident, $lane_trait:ident, $lane_ty:ty, $state:ident, $of_state:ident, $extension:literal, $owner:literal, $emoji:literal, $semantic:literal, $display:literal, $presence:tt) => {
        mod $module {
        use $crate::__pack as pack;
        #[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::SnapshotProjection, semio_framework_value::CanonicalJsonTree, semio_framework_value_derive::NativeReceive)]
        #[canonical_json(owner = $crate::__pack_json)]
        #[value(rename_all = "camelCase", default, by_value_projection)]
        pub struct $record {
            pub state: String,
        }

        $crate::__plugin::transient_root! {
            state: $record,
            mutation: $mutation,
            diff: $diff,
            owner: $owner,
            kind: $semantic,
            display_name: $display,
            payload_schema: "🧬️schema/🔣️.json",
            envelope: $extension,
            extension: $extension,
            retained: true,
            fields: { state: String },
        }

        semio_os_kernel::original_pack_value_operation_receiving!($mutation);

        impl $crate::__pack_json::ArtifactCanonicalJsonTree for $mutation {
            fn canonical_tree_node(&self) -> Result<$crate::__pack_json::ArtifactCanonicalJsonNode<'_>, semio_framework_value::ValueError> {
                Ok($crate::__pack_json::ArtifactCanonicalJsonNode::Object(2))
            }
            fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn $crate::__pack_json::ArtifactCanonicalJsonTree, semio_framework_value::ValueError> {
                static KIND: &str = "snapshot";
                match (self, ordinal) {
                    (_, 0) => Ok(&KIND),
                    (Self::Snapshot { transient }, 1) => Ok(transient),
                    _ => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical lane mutation ordinal is absent")),
                }
            }
            fn canonical_tree_key(&self, ordinal: usize) -> Result<$crate::__pack_json::ArtifactCanonicalJsonText<'_>, semio_framework_value::ValueError> {
                match ordinal {
                    0 => Ok("kind".into()),
                    1 => Ok("transient".into()),
                    _ => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical lane mutation key ordinal is absent")),
                }
            }
        }

        impl $crate::$lane_trait for $record {
            type Mutation = $mutation;
            fn lane(&self) -> $lane_ty {
                $crate::$of_state(&self.state)
            }
            fn replace(lane: &$lane_ty) -> $mutation {
                $mutation::Snapshot { transient: $record { state: $crate::$state(lane) } }
            }
        }

        $crate::__norm_lane_store!(@presence $presence, $record);
        }
        pub use $module::{$record, $diff, $mutation};
    };
    (@presence true, $record:ident) => {
        impl store::ArtifactPresenceSnapshot for $record {
            fn decode_presence_native(bytes: &[u8], owner: &mut store::io::control::NativeSnapshotDecodeOwner<'_, '_>) -> semio_framework_value::RetainedWorkOutcome<Self, store::io::control::NativeSnapshotFailure> {
                store::decode_presence_json(bytes, owner)
            }
        }
    };
    (@presence false, $record:ident) => {};
}

/// 🧱️ Declares the lane store types of one live norm editor in the invoking module: `NormLanePresence`, `NormLaneTransient` and their `Set` mutations. `owner` is the editor taxonomy path the mutation descriptors name.
#[macro_export]
macro_rules! norm_lane_stores {
    (presence_extension = $presence_extension:literal, transient_extension = $transient_extension:literal, owner = $owner:literal) => {
        $crate::__norm_lane_store!(__norm_lane_presence_store, NormLanePresence, NormLanePresenceDiff, NormLanePresenceMutation, LanePresence, $crate::NormPresence, presence_state, presence_of_state, $presence_extension, $owner, "👥️", "set-lane-presence", "Set Lane Presence", true);
        $crate::__norm_lane_store!(__norm_lane_transient_store, NormLaneTransient, NormLaneTransientDiff, NormLaneTransientMutation, LaneTransient, $crate::NormTransient, transient_state, transient_of_state, $transient_extension, $owner, "🫧️", "set-lane-transient", "Set Lane Transient", false);
    };
}

/// 🧱️ Declares the lane store of one read-only norm viewer in the invoking module: only `NormLaneTransient` and its `Set` mutation (ephemeral local-only, never shared, never persisted), no presence. `owner` is the viewer taxonomy path the mutation descriptor names.
#[macro_export]
macro_rules! norm_viewer_lane_stores {
    (transient_extension = $transient_extension:literal, owner = $owner:literal) => {
        $crate::__norm_lane_store!(__norm_lane_transient_store, NormLaneTransient, NormLaneTransientDiff, NormLaneTransientMutation, LaneTransient, $crate::NormTransient, transient_state, transient_of_state, $transient_extension, $owner, "🫧️", "set-lane-transient", "Set Lane Transient", false);
    };
}
