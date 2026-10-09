# Current Mutation Codec Boundary Audit

Current `rg --files` inventory selects 7854 authored Rust mutation sources outside physical IO, tests, fixtures, and independent oracles. The textual scan finds 21 candidate rows in 11 sources. The broader scan also saw 1242 rows already under IO or oracles; those are not taxonomy violations. These are source review candidates, not native acceptance.

- 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧵️canonical/🦀️.rs:13

```rust
use semio_framework_pack_json::{ArtifactCanonicalJsonTree,ArtifactCanonicalJsonNode as Node,ArtifactCanonicalJsonText};
```

- 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧵️canonical/🛂️authority/🦀️.rs:3

```rust
use semio_framework_pack_json::ArtifactCanonicalJsonText;
```

- 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧵️canonical/🔏️seal/🦀️.rs:4

```rust
use semio_framework_pack_json::{ArtifactCanonicalJsonTree,ArtifactCanonicalJsonTreeCursor};
```

- 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:1630

```rust
/// `reconcile_stack` already bound `Mutation: ToValue` and hash via `to_json_string`, which needs
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📡️contributed-mutation-wire/🧬️mutations/🦀️.rs:22

```rust
        serde_json::to_vec(self).map_err(|error| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string())).into())
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📡️contributed-mutation-wire/🧬️mutations/🦀️.rs:26

```rust
        serde_json::from_slice(bytes).map_err(|error| match (u32::try_from(error.line()), u32::try_from(error.column())) { (Ok(line), Ok(column)) => store::PackError::from(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(line, column))), _ => store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, error.to_string())) }.into())
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config/🦀️.rs:26

```rust
            selected: serde_json::from_str(value)
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config/🦀️.rs:31

```rust
        format!("{} {}", Self::OPCODE, serde_json::to_string(&self.selected).expect("nullable string serializes"))
```

- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧭️move-selection/🦀️.rs:134

```rust
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes")
```

- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧭️move-selection/🦀️.rs:138

```rust
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
```

- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧭️move-selection/🦀️.rs:142

```rust
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed JSON parses")
```

- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧭️move-selection/🦀️.rs:227

```rust
            let decoded: Fem2dDiff = semio_framework_pack_json::from_json_str(diff, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the committed diff decodes");
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️testing/🧬️job-test-mutations/🧬️mutations/🦀️.rs:19

```rust
        serde_json::to_vec(self).map_err(|error| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string())).into())
```

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️testing/🧬️job-test-mutations/🧬️mutations/🦀️.rs:22

```rust
        serde_json::from_slice(bytes).map_err(|error| match (u32::try_from(error.line()), u32::try_from(error.column())) { (Ok(line), Ok(column)) => store::PackError::from(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(line, column))), _ => store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, error.to_string())) }.into())
```

- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧭️move-selection/🦀️.rs:179

```rust
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes")
```

- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧭️move-selection/🦀️.rs:183

```rust
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
```

- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧭️move-selection/🦀️.rs:187

```rust
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed JSON parses")
```

- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧭️move-selection/🦀️.rs:272

```rust
            let decoded: Fem3dDiff = semio_framework_pack_json::from_json_str(diff, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the committed diff decodes");
```

- ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉️wall-geometry/🦀️.rs:259

```rust
    use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
```

- ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉️wall-geometry/🦀️.rs:268

```rust
        from_json_str(text, JsonMemberPolicy::Reject).expect("fixture decodes")
```

- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:8

```rust
//! The `serde_json::Value` bridge (`🔖️ValueBridge`) and the play app's `Puzzle5dPlaySnapshot`
```

