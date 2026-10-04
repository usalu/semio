# Independent Intrinsic Media Transport Audit

## 2026-10-03 — Read-Only Source Snapshot

No Cargo lane or runtime execution was started. Concurrent mounts are incomplete; the observations below describe source read during this audit, not a final build verdict.

## Concrete Findings

1. **Preexisting fidelity defect, repair explicitly held.** Root `🧰️framework/🔨️modules/🎒️pack/🌱️value/🦀️.rs:569–572` sorts dynamic Object occurrences. Root controlled `🌱️value/🛫️encode/🦀️.rs:86` and actual kernel controlled `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🛫️encode/🦀️.rs:90` do the same. A `[(z,1),(a,2),(z,3)]` owner becomes `[(a,2),(z,1),(z,3)]`. Duplicate occurrences survive, but owner order does not. This is separate from canonical schema Map ordering.

2. **Actual producer/import mounting gap.** At inspection, plugin `🦀️.rs:35555–35577` still routes Intrinsic through JsonWriteCursor and declares Binary. The new helpers at `14584–14597` are not yet invoked there. This loses raw IEEE words/octet types and occurrence semantics at the actual boundary. `consume_media` matches at `15059` and `35599` omit the new wire variant; the natural-file branch at `35609` accepts Binary only. These are known in-progress mounts, not allegations about the final patch.

3. **New helper verification gap.** Native neutral law in root `🌱️value/🧪️tests/🎞️intrinsic-media/🦀️.rs:26–39` invokes ordinary record encode, controlled record encode, and uncontrolled exact decode. It does not invoke the new borrowed `encode_value_record_body_controlled`, controlled exact decode, or MediaArtifact helpers. Wrong field count/identity, trailing bytes, cancellation and caller ownership refusals therefore have no assertions in this newly mounted law. Runtime behavior is unverified by this audit.

4. **Inherited framing-depth asymmetry worth a boundary law.** Borrowed controlled encoder starts dynamic depth at 2 (`root 🌱️value/🛫️encode/🦀️.rs:99,111`), while controlled exact decoder starts at 0 (`root 🌱️value/🦀️.rs:3244`). Decoder accepts two more nesting levels than the same configured encoder can reproduce. Existing uncontrolled terminal decode also starts at 0; this is inherited grammar accounting rather than a newly invented scalar loss.

## Source Evidence That Looks Correct

The borrowed helper traverses the original value reference, admits symbol storage, measures bytes, admits output allocation, and checks measured/emitted length. It does not first clone into a synthetic record. Controlled exact decode checks maximum file length, requires one expected field ID and TAG_VALUE, and rejects trailing bytes. UInt/Int use distinct tags, Float writes raw little-endian f64 bytes, Bytes retains octets, and decoder Object appends occurrences in wire order. Descriptor port/schema copying is admitted through caller control. MediaArtifactError retains a ValueError source. These conclusions are source inspection only.

## Independent Corpus Scope

The neutral fixture explicitly includes all nine literal categories, UInt64/Int64 limits, nine binary64 words, NUL/Unicode text, octets, unsorted Object occurrences and duplicate names. The Bun test uses strict AJV schema validation and a genuine Bun SQLite serialize/reopen cycle, then reconstructs ordered occurrence rows. It stores float/octet bytes directly and integer literal strings, avoiding JSON number loss. Its SQLite schema is test-only independent materialization, not actual owner Snapshot SQL coverage. Native serde_json readback operates on neutral tagged values with float words represented as hex; it validates the independent semantic representation, not direct intrinsic JSON fidelity. No new test result is claimed here.
