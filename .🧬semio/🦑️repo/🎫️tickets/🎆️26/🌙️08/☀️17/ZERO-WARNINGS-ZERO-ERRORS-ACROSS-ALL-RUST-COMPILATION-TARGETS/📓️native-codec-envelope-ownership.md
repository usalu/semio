# Native Codec Envelope Ownership

Pass 354 fixes both E0507 failures in native339. The GIS/VCS genesis fixture checks now consume ArtifactEnvelope::into_owners before inspecting or moving its cursor. This also detaches the guarded terminal shell before assertions can unwind. These fixtures explicitly assert empty history and a genesis cursor, so owned fields remain bounded; no runtime Drop contract is weakened.

Runtime integration checks are pending.

- ✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🧪️tests/🦀️.rs
- ✏️s/🔌️plugins/🌿️vcs/📇️native-codecs/🧪️tests/🦀️.rs
