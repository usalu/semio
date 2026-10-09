# Kernel Full16 Complete Diagnostics

The original complete Kernel all-targets mutation-testing compiler terminated RED8 production diagnostics before test compilation/runtime. Original roster and deadlines retained; raw log `🗑️generated/fd/kernel-full16.log`. No native runtime law accepted.

## 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs

| Span | Exact Diagnostic | Raw Log Line | Conflicting/Related Span |
|---|---|---:|---|
| 2622:105 | error[E0425]: cannot find type RetirementDemand in this scope | 5657 | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2623:9 |
| 2623:9 | error[E0422]: cannot find struct, variant or union type RetirementDemand in this scope | 5674 |  |
| 29161:104 | error[E0119]: conflicting implementations of trait semio_framework_value::retirement::RetireOwned for type os_store::component::SpaceMemberPin | 16674 | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/🦀️.rs:186:1; 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:29176:104; 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/🦀️.rs:187:1 |
| 29176:104 | error[E0119]: conflicting implementations of trait semio_framework_value::retirement::RetireOwned for type os_store::component::SpaceCheckpoint | 16687 | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/🦀️.rs:187:1; 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:29189:104; 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/🦀️.rs:188:1 |
| 29189:104 | error[E0119]: conflicting implementations of trait semio_framework_value::retirement::RetireOwned for type os_store::component::SpaceAlternative | 16700 | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/🦀️.rs:188:1; 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:29213:113; 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/🦀️.rs:189:1 |
| 29213:113 | error[E0119]: conflicting implementations of trait semio_framework_value::retirement::RetireOwned for type os_store::component::SpaceHistorySnapshot | 16713 | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/🦀️.rs:189:1; 🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs:36:51; 🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/🦀️.rs:586:114 |

## 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/🦀️.rs

| Span | Exact Diagnostic | Raw Log Line | Conflicting/Related Span |
|---|---|---:|---|
| 184:1 | error[E0119]: conflicting implementations of trait semio_framework_value::retirement::RetireOwned for type os_vcs::Author | 16648 | 🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:38:104; 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🏠️owner/🧬️schema/🦀️.rs:6:58; 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/🦀️.rs:112:1 |

## 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🏠️owner/🧬️schema/🦀️.rs

| Span | Exact Diagnostic | Raw Log Line | Conflicting/Related Span |
|---|---|---:|---|
| 6:58 | error[E0119]: conflicting implementations of trait semio_framework_value::retirement::RetireOwned for type artifact_child_owner_schema::OwnerRef | 16661 | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/🦀️.rs:112:1; 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:29161:104; 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/🦀️.rs:186:1 |

