# 🏷️ Binary tags (wave 2)

Before (script, no --fix): duplicates in en1992 (149..157 family, 9 tags), en1993 (533, 534), en1994 (119), en1995 (137), en1998 (279, 127), en1999 (117..124, 142, 143, ...); protocol disagreements in vdi3805 (16), en1993 (303), en1995 (18), en1994 (2), en1998 (3), en1992 and en1999 (via the dups). Clean: en1990, en1991, en1997, din18599, din16798, din4108, en1996, iso16757.

Authority: the artifact's `💾️binary/🧬️mutations/📡️.protocol.semio` states "the only source of those tags"; Rust codecs derive tags from it at compile time (`dsl::protocol_record::tag_u8` or generated), no hand-written tag constants exist (checked all `🦀️.rs` under binary/mutations). Protocol tags are unique in every artifact (checked), so leaf JSON `binaryTag` was stale metadata. Fix: `binary_tags.py --fix` rewrote the leaf JSON to the protocol tag (protocol untouched; wire format and binary fixtures unchanged, so no fixture regeneration). en1991 tags exceed 255 (varint), so no u8 bound is asserted.

Touched artifacts: en1992, en1993, en1994, en1995, en1998, en1999, vdi3805 (leaf JSON `binaryTag`; twins regenerated via heavy.sh `--under <artifact path>`; en1998 snapshot twin drift written by the generator is another agent's concurrent edit). freeze.txt was absent when editing.

Law: `✏️s/🔌️plugins/📕️norm/⚖️compliance/🧪️tests/🏷️binary-tags/{🐍️.py,🥒️.feature}` fails on duplicate leaf tags, leaf/protocol disagreement, leaves without record (kinds matched ignoring dashes, because en1998 has `xm` vs `x-m` slugs with equal tags), and repeated protocol tags/kinds. Run: 2503 leaves, passes.

Gates (`evolve.py verify <a>`): twins-check-strict, schema-sync, handle-paths, cargo-authority, build-owners-sync ok for en1994; see final status below for the rest.

Final: all six artifacts pass the gates (strict twins, schema-sync, handle-paths, cargo-authority, build-owners-sync); vdi3805 twins identical (no evolve support needed). Appended '- ready for rebuild: binary tags' to build-enXXXX/latest.md.
