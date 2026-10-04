# Graph Property Ownership And Caller Census

Read-only current source review 2026-10-03. No Cargo or source changes.73 Rust files contain PropertyBag or PropertyValue::Object;41 are outside /🧪️tests/. Counts below are exact token occurrences, not required edit counts or proof of all semantic aliases. Unrelated BTreeMap indexes in those files must not be blanket replaced.

Canonical authority is `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs`: PropertyValue49–56 has Null/Bool/f64 Number/String/ordered Array/unique String-key Object; Object is BTreeMap, as_object80 exports its storage, and PropertyBag252 aliases it. Ordinary DSL conversion102–142 recurses and clones into Vec/BTreeMap; DslField146–166 defines ordinary to/from only. Exact paid controlled recursion cannot be established by changing the alias alone. Physical owns that implementation; Shared owns enclosing Manifest/PropertyDef/ValueType factories.

Required natural domain map operations observed include empty/default construction, sorted owned iteration, borrowed iter/keys/values, get/get_mut, membership, insert/remove, consuming extraction and collection construction. Handcraft exact Vec-backed storage with preserved sorted unique key semantics and equal-key replacement authority. Controlled builders admit actual tuple slot buffers before requests; full replacement buffer charge must precede growth, with incoming/displaced recursive payload guards. No entry/index/BTree storage conversion shim is justified.

Nontrivial actual ports: OS Infinite board directed `➕️normal/🦀️.rs:3158` calls range(Excluded(after.clone()),Unbounded).next() for child audit; rewrite to domain successor/ordered iterator and keep reserved child replacement through get_mut.3250 consumes pop_first. Directed board DAG2424 and artifact DAG retained263 also consume pop_first during retirement. Prefer a domain consuming pop mechanism; repeated removal of vector first element can otherwise make wide retirement quadratic. The controlled DAG seven-law fixture at root tests SQLite66/72 explicitly constructs BTreeMap top-level properties and nested Object; handcraft those owners, retaining all literal/IEEE witness values. Sequence editor447 conversion uses ordinary ToValue/FromValue with default-empty fallback; a new map type must retain genuine conversion authority rather than let fallback erase properties.

Existing first-party value `🗂️ordered/🦀️.rs` is a persistent Arc-owned AVL (Entry16/Node17/OrderedMap76); its controlled allocation implementation estimates Arc header bytes. It is not an exact single owned vector PropertyBag authority. Shared/OS DSL RecordFields `🧬️schema/🧩️record/🗂️fields/🦀️.rs` provides exact owned Vec/capacity/binary-search design evidence but is u16/FieldValue-specific; do not wrap it to claim graph property ownership. Generic slot+128 estimates provide no physical proof.

Meaningful neutral laws should declare sorted unique keys, duplicate policy, empty/null distinction, nested arrays/objects and literal keys/strings including NUL/Unicode, and exact IEEE word companions. Compare semantic SQL with independent SQLite/DataView; compare ordered map behavior with an independent test-only map implementation. Native allocation observation must measure actual constructor/replacement requests for cardinalities0/1/3/growth transitions, exact/minus-one limits and denial before request. Deep cancellation/refused replacement must retain parent ownership and reclaim actual recursive payload addresses on a narrow stack. Existing DAG7 does not by itself establish every newly introduced property map denial branch or exact backing request. Root remains sole owning Native runner.

| Current source file | PropertyBag tokens | PropertyValue::Object tokens | BTreeMap tokens (may be unrelated) |
| --- | --- | --- | --- |
| `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | 3 | 0 | 5 |
| `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs` | 4 | 0 | 0 |
| `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🧪️tests/🔬️unit/🦀️.rs` | 3 | 0 | 0 |
| `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️connect-nodes/🦠️mutation/🦀️.rs` | 4 | 0 | 0 |
| `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗃️replace-node-properties/🧪️tests/🧪️rejects/🦀️.rs` | 2 | 0 | 0 |
| `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗃️replace-node-properties/🦠️mutation/🦀️.rs` | 4 | 0 | 0 |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` | 1 | 0 | 2 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs` | 3 | 0 | 4 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` | 2 | 1 | 0 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs` | 10 | 10 | 35 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs` | 1 | 1 | 1 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/🦀️.rs` | 1 | 0 | 0 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️.rs` | 5 | 0 | 11 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧬️schema/📸️snapshot/🦀️.rs` | 6 | 0 | 0 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧬️schema/📸️snapshot/🪶️sqlite/🛫️projection/🦀️.rs` | 0 | 3 | 0 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧬️schema/📸️snapshot/🪶️sqlite/🛬️reconstruction/🦀️.rs` | 0 | 3 | 23 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧪️tests/🪶️sqlite/🦀️.rs` | 0 | 1 | 3 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧵️retained/🦀️.rs` | 2 | 1 | 0 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🔗️connect-nodes/🦀️.rs` | 2 | 0 | 0 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🗃️replace-node-properties/🦀️.rs` | 2 | 0 | 0 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🦀️.rs` | 4 | 0 | 0 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧪️tests/🔬️dag-vcs/🦀️.rs` | 6 | 0 | 0 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧪️tests/🔬️dag-direct/🦀️.rs` | 2 | 0 | 0 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs` | 16 | 1 | 0 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧹️retirement/🦀️.rs` | 1 | 0 | 2 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` | 3 | 1 | 29 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs` | 4 | 0 | 6 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/💡️inferences/🕸️connectivity/🦀️.rs` | 2 | 0 | 7 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/➕️normal-internals/➡️directed/🦀️.rs` | 11 | 0 | 3 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/➕️normal-internals/➡️directed/🧪️tests/🔬️unit/🦀️.rs` | 6 | 0 | 0 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/➕️normal-internals/↔️undirected/🦀️.rs` | 13 | 0 | 3 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/➕️normal-internals/↔️undirected/🧪️tests/🔬️unit/🦀️.rs` | 4 | 0 | 0 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🔧️operators-internals/🦀️.rs` | 12 | 0 | 32 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🔧️operators-internals/🧪️tests/🔬️unit/🦀️.rs` | 23 | 0 | 0 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🔌️ports-internals/➡️directed/➕️normal/🦀️.rs` | 18 | 0 | 7 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🔌️ports-internals/➡️directed/➕️normal/🧪️tests/🔬️unit/🦀️.rs` | 7 | 0 | 0 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🔌️ports-internals/↔️undirected/🦀️.rs` | 17 | 0 | 7 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🔌️ports-internals/↔️undirected/🧪️tests/🔬️unit/🦀️.rs` | 10 | 0 | 0 |
| `🧰️framework/🔨️modules/🕸️graph/⚙️engine/🦀️.rs` | 44 | 0 | 42 |
| `🧰️framework/🔨️modules/🕸️graph/⚙️engine/🧪️tests/🔬️unit/🦀️.rs` | 11 | 0 | 1 |
| `🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🦀️.rs` | 14 | 5 | 10 |
| `🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs` | 0 | 1 | 0 |
| `🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🧪️tests/🔬️wire-unit/🦀️.rs` | 8 | 1 | 0 |
| `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/♻️retirement/🦀️.rs` | 0 | 1 | 0 |
| `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs` | 6 | 4 | 5 |
| `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🧪️tests/🔬️unit/🦀️.rs` | 0 | 2 | 2 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | 2 | 0 | 0 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🐚️shell/📦️packages/🦀️rust/📦️bin.rs` | 0 | 1 | 0 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🧪️tests/🔬️unit/🦀️.rs` | 16 | 1 | 1 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🐚️shell/🧪️tests/🔬️bin-unit/🦀️.rs` | 3 | 0 | 0 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛️flat-position/🧪️tests/🔬️unit/🦀️.rs` | 22 | 0 | 0 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🦀️.rs` | 5 | 0 | 0 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🧪️tests/🔬️unit/🦀️.rs` | 5 | 0 | 0 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs` | 4 | 1 | 0 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🧪️tests/🔬️unit/🦀️.rs` | 14 | 0 | 0 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs` | 0 | 3 | 0 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | 0 | 1 | 15 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` | 0 | 5 | 12 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-node/🧪️tests/✏️keeps/🦀️.rs` | 2 | 0 | 0 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-node/🧪️tests/📍️keeps/🦀️.rs` | 2 | 0 | 0 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🦀️.rs` | 0 | 1 | 0 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs` | 0 | 1 | 0 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-data-property/🧪️tests/🏷️keeps/🦀️.rs` | 2 | 0 | 0 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-data-property/🧪️tests/🧹️keeps/🦀️.rs` | 2 | 0 | 0 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛜️wire-runtime/🦀️.rs` | 4 | 1 | 0 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | 3 | 0 | 0 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛜️wire-runtime/🧪️tests/🔬️unit/🦀️.rs` | 8 | 2 | 2 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs` | 2 | 0 | 0 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🧪️tests/🔬️unit/🦀️.rs` | 7 | 1 | 2 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🪜️execution/🦀️.rs` | 6 | 3 | 2 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🦀️.rs` | 6 | 0 | 3 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗣️language-service/🦀️.rs` | 6 | 2 | 0 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️.rs` | 7 | 0 | 5 |

