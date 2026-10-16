"""✂️ Small structural edits, glue roots and the distribution of the retirement integration impls to the type owners."""
import os, re
from rlib import *
import rcrates as C

def edit_text(rel, pairs):
    t = read_stage_or_base(rel)
    for old, new in pairs:
        assert t.count(old) == 1, (rel, old[:70], t.count(old))
        t = t.replace(old, new)
    write(rel, t)

def small_edits():
    edit_text(R + "⚙️codec/🦀️.rs", [(
        '#[path="🛬️retained/🦀️.rs"]\nmod retained_wire_field;\npub use retained_wire_field::{RetainedWireFieldDecode,RetainedWireFieldStep,RetainedWireByteSource,OriginalWireSourceIdentity};\npub(crate) use retained_wire_field::RetainedWireUtf8Validation;\n', "")])
    edit_text(R + "🚪️io/🦀️.rs", [('\n#[path = "👥️presence/🔍️borrowed/🦀️.rs"]\npub mod presence;\n', "")])
    edit_text(R + "🚪️io/💾️binary/🦀️.rs", [('#[path="🔗️causal/🦀️.rs"]\npub mod causal;\n\n', "")])
    edit_text(R + "🚪️io/📝️text/🦀️.rs", [('\n#[path = "🧵️canonical/🦀️.rs"]\npub mod canonical;\n\n#[path="🔗️causal/🦀️.rs"]\nmod causal;\n', "")])
    edit_text(R + "🔗️causal/🦀️.rs", [('#[path = "🔀️transition/🦀️.rs"]\npub mod transition;\npub use transition::*;\n\n', "")])
    edit_text(R + "🔗️causal/🔀️transition/🔁️fold/🦀️.rs", [(
        '#[path = "🗂️index/🦀️.rs"]\nmod index;\npub use index::{HistoryFoldIndexInsert,HistoryFoldIndex,HistoryFoldIndexIntoIter,HistoryFoldIndexIter,HistoryFoldSet,HistoryFoldSetIter,HistoryFoldSetIntoIter};\n',
        'pub use semio_framework_replication_history_index::{HistoryFoldIndexInsert,HistoryFoldIndex,HistoryFoldIndexIntoIter,HistoryFoldIndexIter,HistoryFoldSet,HistoryFoldSetIter,HistoryFoldSetIntoIter};\n')])

TEST_ALLOC = ("#[cfg(test)]\npub(crate) use semio_framework_trace_memory_testing as test_allocation;\n"
              "#[cfg(test)]\n#[global_allocator]\nstatic REQUESTED_ALLOCATOR: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;\n")
def rel_from(pkg_root_file, target):
    return os.path.relpath(target, os.path.dirname(pkg_root_file))

def mod_line(root, target, name, vis="", glob=False):
    s = '#[path = "' + rel_from(root, target) + '"]\n' + vis + "mod " + name + ";\n"
    if glob: s += "pub use " + name + "::*;\n"
    return s

def glue_roots(msg_glue_text):
    out = {}
    root = C.libroot("codec")
    out["codec"] = ("//! ⚙️ Codec glue — wiring only. Domain lives at the owner `🦀️.rs` files.\n//!\n//! This crate root is the former `protocol::codec` namespace; `io` hosts the `OpBinary` and `OpText` trait family beside it.\n\n"
                    + "\n" + TEST_ALLOC + "\n"
                    + mod_line(root, R + "⚙️codec/🦀️.rs", "component", glob=True) + "\n"
                    + mod_line(root, R + "⚙️codec/🆔️ids/🦀️.rs", "ids", vis="pub ", glob=True).replace("pub use ids::*;", "pub use self::ids::*;") + "\n"
                    + mod_line(root, R + "🚪️io/🦀️.rs", "io", vis="pub ") + "pub use io::{DiffBinary, DiffCodec, DiffText, OpBinary, OpBinaryReceiving, OpBinaryReceivingCursor, OpText};\n")
    root = C.libroot("wire-core")
    out["wire-core"] = ("//! 🧾️ Wire-core glue — wiring only. Domain lives at the owner `🦀️.rs` files.\n//!\n//! Limits, record kinds, flags, merge and undo policies and the primitive codecs, with the scalar timestamp and id codec, the dictionaries and the retained wire fields beside them.\n\n"
                        + "\n" + TEST_ALLOC + "\n"
                        + mod_line(root, R + "🧾️wire/🦀️.rs", "wire", glob=True) + "\n"
                        + mod_line(root, R + "🔢️scalar/🦀️.rs", "scalar", vis="pub ", glob=True) + "\n"
                        + mod_line(root, R + "📖️dictionary/🦀️.rs", "dictionary", vis="pub ", glob=True) + "\n"
                        + mod_line(root, R + "⚙️codec/🛬️retained/🦀️.rs", "retained", vis="pub ") + "pub use retained::{RetainedWireFieldDecode, RetainedWireFieldStep, RetainedWireByteSource, OriginalWireSourceIdentity};\n")
    root = C.libroot("format")
    out["format"] = ("//! 📐️ Format glue — wiring only. Domain lives at the owner `🦀️.rs` files.\n//!\n//! The `.spr` record file format with the slice helpers that ship a byte-exact record range.\n\n"
                     + "\n"
                     + mod_line(root, R + "📐️format/🦀️.rs", "component", glob=True) + "\n"
                     + mod_line(root, R + "🦀️.rs", "slice", glob=True))
    root = C.libroot("causal")
    out["causal"] = ("//! 🔗️ Causal glue — wiring only. Domain lives at the owner `🦀️.rs` files.\n//!\n//! Envelopes, diffs, inverses and frontiers, with their binary and text codecs beside them.\n\n"
                     + "\n" + TEST_ALLOC + "\n"
                     + mod_line(root, R + "🔗️causal/🦀️.rs", "component", glob=True) + "\n"
                     + mod_line(root, R + "🚪️io/💾️binary/🔗️causal/🦀️.rs", "binary", vis="pub ") + "\n"
                     + mod_line(root, R + "🚪️io/📝️text/🔗️causal/🦀️.rs", "text"))
    out["mutation-messages"] = msg_glue_text
    for k, v in out.items():
        write(C.libroot(k), v)
    return out

def append_allocator(key_root_files):
    for rel in key_root_files:
        t = read_stage_or_base(rel)
        write(rel, t.rstrip("\n") + "\n\n" + TEST_ALLOC)

def presence_mounts():
    p = R + "📡️wire/👥️presence/🦀️.rs"
    t = read_stage_or_base(p)
    t = t.rstrip("\n") + "\n\n" + (mod_line(p, R + "🚪️io/👥️presence/🔍️borrowed/🦀️.rs", "borrowed", vis="pub ")
        + "pub use borrowed::{borrow_presence_metadata, BorrowedPresenceMetadata, BorrowedPresenceInteraction, BorrowedPresenceDomain, BorrowedPresenceStrings};\n\n"
        + mod_line(p, R + "👕️peer-overlay/🦀️.rs", "peer_overlay", vis="pub ", glob=True))
    write(p, t)
