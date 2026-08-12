# subset! usage excerpt

```rust
macro_rules! subset {
    (
        $vis:vis owning dialect $artifact:literal / $standard:literal / $subset:literal {
            spec $spec:ident {
                construction: $construction:ty,
                analysis: $analysis:ty,
                composition: $composition:ty,
            }
            builder: $builder:ident,
            analyzer: $analyzer:ident,
            composer: $composer:ident,
            $(io: [$($io_entry:expr),+ $(,)?],)?
            $(validator: $validator:ty,)?
            $(examples: [$($example:expr),+ $(,)?],)?
        }
    ) => {
        $crate::derive_artifact_facets! {
            $vis spec $spec {
                construction: $construction,
                analysis: $analysis,
                composition: $composition,
            }
            builder: $builder,
            analyzer: $analyzer,
            composer: $composer,
        }

        #[doc(hidden)]
        pub mod __subset_registration {
            use super::*;
            use std::sync::{Once, OnceLock};

            pub const SUBSET_DIALECT: $crate::Dialect = $crate::Dialect {
                artifact_kind: $artifact,
                standard: $crate::StandardId($standard),
                subset: $crate::SubsetId($subset),
            };
            pub const KIND: $crate::SubsetKind = $crate::SubsetKind::Owning;
            static REGISTERED: Once = Once::new();
            $(static VALIDATOR_ENTRY: OnceLock<$crate::SubsetValidatorEntry> = OnceLock::new();)?

            $(fn validator_entry() -> &'static $crate::SubsetValidatorEntry {
                VALIDATOR_ENTRY.get_or_init(|| $crate::subset_validator_entry_of::<$validator>())
            })?

            pub fn register() {
                REGISTERED.call_once(|| {
                    let mut entries = vec![$crate::composer_entry_of::<$composer>()];
                    $(entries.extend([$($io_entry),+]);)?
                    $crate::register_composer_entries(&entries);
                    $($crate::register_subset_validator(validator_entry());)?
                });
            }

            $(pub const EXAMPLES: &'static [$crate::ExampleSource] = &[$($example),+];)?

            #[cfg(test)
```
