# Flood Large Close Boundary

## Actual Public TS Red

Added schema-first neutral case and public regression under `🧰️framework/🔨️modules/🔲️pixels/✍️editing/🪣️flood/🧪️tests`:

- large-close.json
- large-close.schema.json
- large-close.test.ts

Fixture uses1024×1024 uniform RGBA, seed0,0; three allocation-only turns, cancellation before initialization/BFS, then eight close turns. No million-node oracle or full-region traversal runs. Strict Ajv validates the fixture. Public methods only; no private field inspection, source edits, grant widening, or extent reduction.

Executed `repo tools exec --light bun -- test './🧰️framework/🔨️modules/🔲️pixels/✍️editing/🪣️flood/🧪️tests/large-close.test.ts'`: actual exit1,0pass/1fail,20 assertions. Failure is expected closed=true versus actualfalse.

Observed allocation retained6,291,456 bytes (seen1MiB + mask1MiB + queue4MiB). Cancellation returned canceled, result remained unpublished. Initial close demand was {copyBytes:512,capacityBytes:0,releaseBytes:4194304,depth:1}. Every one of eight close turns returned successful zero work, zero receipt, zero released bytes; closed remainedfalse. Repeating the unchanged grant cannot alter this fixed-point denial.

Real grant copied from MountedOwnerPolicyV1::INTERACTIVE_COMPONENT in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner/🦀️.rs:37`:32items,32,768copy bytes,16,777,216capacity bytes,1,048,576release bytes,4096depth. This is not the previous focused test convenience grant.

## Native Source Evidence And Extent Nuance

Shared Rust cursor `pixels/✍️editing/🪣️flood/🦀️.rs:15` has Vec<u32> queue, so1024² capacity is4MiB, not8MiB. Allocation reserves count u32 values at line35 and records actual Vec capacity×4. Close selects its retained queue owner at lines52–55. This is source evidence of the same monolithic backing risk, not a claimed Native result. Native95837 remains source-frozen; no Native launch or production change occurred here.

Generic validate_extent in `pixels/✍️editing/🦀️.rs:90` and TS constructor accept1024² within authored side/pixel limits. Thus this is a valid generic public extent independently of Lowpoly's current texture convention.

Lowpoly retained admission in `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained-commands/🦀️.rs:25,128` accepts layer.pixels length≤4MiB. This admits the byte length of1024² RGBA. However, its snapshot currently fixes LOWPOLY_PAINT_TEXTURE_SIZE=256 (`🧬️schema/📸️snapshot/🦀️.rs:50`) and old paint commands still call model flood_fill (paint owner line52). There is no existing separate new Lowpoly cursor wired yet; Runtime confirmed this. Therefore do not describe the4MiB len cap alone as proof of an existing mounted1024-dimensional layer path. It is an invariant ambiguity plus independently valid generic kernel extent, not an excuse to truncate generic acceptance.

## Required Design Direction

Runtime and root were notified immediately with actual TS red and corrected native queue size. Runtime owns Rust paged design. Every retained data backing must have an admitted retirement route under real release authority; a single queue4MiB cannot be made payable by granting more turns of1MiB. At larger valid generic extents even seen/mask backings exceed the release grant. Page queue, seen, mask and any dynamic page directory/control backing so no indivisible allocation exceeds the minimum supported close authority. Quote and charge page birth, lookup/copy work and reference retirement without hidden whole-image allocation or page-table growth. Do not globally widen grants, truncate accepted extents, or merely raise test close grants.

This red test intentionally remains failing pending the paged implementation. Existing small focused suite green does not override this boundary failure. TS releases logical backing ownership for GC; no synchronous physical GC or native paid-drop claim is made.
