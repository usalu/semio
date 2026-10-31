# Actor Boundary Production Review

Read-only source preflight, 2026-10-29. No builds, tests, native jobs, cache changes, or source edits were performed. The existing native run was left untouched.

## Current Contract

`🧰️framework/🔨️modules/📡️replication/🆔️ids/🦀️.rs:23` defines the replication `ActorId` as a tuple containing first-party `semio_framework_value::SharedUtf8`. Its `FromValue` implementation at lines 74–77 decodes that same first-party type. This is distinct from the numeric runtime actor and machine actor types.

`🧰️framework/🔨️modules/🌱️value/📝️shared-utf8/🦀️.rs:55–56` provides `From<String>` and `From<&str>`. The inspected identifier definition does not provide `ActorId::from_str`; direct tuple construction must supply `SharedUtf8`. For a String or str source, the concrete correction is `ActorId(source.into())`; an already shared source can move directly into the tuple.

## Selected Production Boundaries

No concrete String-versus-SharedUtf8 constructor mismatch or ActorId FromStr call was found in the selected Layout, Writer, plugin SDK, and shared tool-machine production boundaries.

- Layout transform start and resume (`✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️blueprint/🪛️utilities/🔄️transform/🦀️.rs:297,309`) already convert their string sources using `.into()`.
- Writer production source search found no direct ActorId construction or ActorId FromStr call.
- SDK settlement (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/🧵️settlement/🦀️.rs:26`) and typing edit (`⌨️typing/✍️edit/🦀️.rs:37` under the same tool-machine root) already construct `ActorId(actor_text.into())` and `ActorId(text.into())`.
- SDK restored checkpoint actor (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📸️checkpoint/🦀️.rs:100–113`) explicitly admits the decoded String through `SharedUtf8::admit`, then constructs ActorId from the returned shared text.
- Mounted kernel receipt (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🦀️.rs:321,396,400`) declares its author owner as `Option<SharedUtf8>` and moves that shared value into ActorId.
- SDK ActionMeta (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/📨️emission/🦀️.rs:702`) already declares `actor: SharedUtf8`; staging constructors using `ActorId(meta.actor.clone())` therefore match this contract.

## Scope and Limits

Searches were bounded to repository framework source and then selected plugin SDK, Layout, Writer, and tool-machine roots. Cache, node_modules, build target, and ticket contents were excluded from the source walks; test directory matches were not treated as production findings. The inspection verifies only the listed type boundaries. It does not establish that all production code compiles, and SDK/Layout native gates remain necessary.
