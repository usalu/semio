# Flow5 Current Receiving Refusal

Literal registered full owning command physically closed with Nx/Bun1 and Cargo101. Original controls and full package all-target selection retained. No runtime target accepted.

{
  "code": 101,
  "reason": "exit",
  "exact": true,
  "producer": true,
  "sources": 1014,
  "owning": false,
  "whole": false,
  "errors": 7,
  "changed": []
}

```text
error[E0432]: unresolved import `crate::retirement::factory`
 --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🔗️shared/🏭️factory/🦀️.rs:2:253
  |
2 | ...,admit_retained_clone_close},retirement::factory::FactoryAuthority};
  |                                             ^^^^^^^ could not find `factory` in `retirement`

error[E0433]: cannot find `factory` in `retirement`
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🔗️shared/🏭️factory/🦀️.rs:27:84
   |
27 | ...   if let Some(unique)=self.unique.as_ref(){let mut demand=crate::retirement::factory::factory_ticket_demands(unique,copy)?;deman...
```

```text
error[E0433]: cannot find `factory` in `retirement`
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🔗️shared/🏭️factory/🦀️.rs:27:84
   |
27 | ...   if let Some(unique)=self.unique.as_ref(){let mut demand=crate::retirement::factory::factory_ticket_demands(unique,copy)?;deman...
   |                                                                                  ^^^^^^^ could not find `factory` in `retirement`

error[E0433]: cannot find `factory` in `retirement`
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🔗️shared/🏭️factory/🦀️.rs:53:60
   |
53 | ...   if self.unique.is_some(){return crate::retirement::factory::close_factory_ticket(&mut self.unique,child).map(|step|RetainedClo...
```

```text
error[E0433]: cannot find `factory` in `retirement`
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🔗️shared/🏭️factory/🦀️.rs:53:60
   |
53 | ...   if self.unique.is_some(){return crate::retirement::factory::close_factory_ticket(&mut self.unique,child).map(|step|RetainedClo...
   |                                                          ^^^^^^^ could not find `factory` in `retirement`

error[E0425]: cannot find function `arc_bytes` in module `super::super`
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🔗️shared/🏭️factory/🦀️.rs:25:115
   |
25 | ...es:size_of::<T>(),release_bytes:super::super::arc_bytes::<T>(),depth:1,..Default::default()});}
```

```text
error[E0425]: cannot find function `arc_bytes` in module `super::super`
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🔗️shared/🏭️factory/🦀️.rs:25:115
   |
25 | ...es:size_of::<T>(),release_bytes:super::super::arc_bytes::<T>(),depth:1,..Default::default()});}
   |                                                  ^^^^^^^^^ not found in `super::super`
   |
help: consider importing this function
   |
 2 + use crate::retirement::shared::arc_bytes;
   |
```

```text
error[E0425]: cannot find function `arc_bytes` in module `super::super`
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🔗️shared/🏭️factory/🦀️.rs:42:121
   |
42 | ...eased_bytes=if unique.is_some(){super::super::arc_bytes::<T>()}else{0};
   |                                                  ^^^^^^^^^ not found in `super::super`
   |
help: consider importing this function
   |
 2 + use crate::retirement::shared::arc_bytes;
   |
```

```text
error: could not compile `semio-framework-value` (lib) due to 5 previous errors; 44 warnings emitted
warning: build failed, waiting for other jobs to finish...
warning: `semio-framework-value` (lib) generated 44 warnings
error: could not compile `semio-framework-value` (lib) due to 5 previous errors; 44 warnings emitted

```

```text
error: could not compile `semio-framework-value` (lib) due to 5 previous errors; 44 warnings emitted

```
