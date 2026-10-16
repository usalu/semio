"""🛬️ Lift L10 (found by the orphan scan, not in Appendix J): `impl RetireOwned for ChildContentView` (and the read retirement it needs) sits in a file of the composition tree that `📜️declaration/🪆️child/🦀️.rs` textually includes; the impl must follow the type into decl-views."""
import os, sys, re
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import lift3
from lift3 import write, delete, rd, rollback, rebase_literals
P = '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/'
try:
    src = P + '🧩️composition/♻️root/👁️read/🦀️.rs'
    dest = P + '📜️declaration/🪆️child/🛬️lifted/👁️read/🦀️.rs'
    parent = P + '📜️declaration/🪆️child/🦀️.rs'
    text = rd(src)
    assert not os.path.exists(dest)
    doc = ('//! 👁️ Lift file: the original child read retirement and its `RetireOwned for ChildContentView` impl, of `🧩️composition/♻️root/👁️read/🦀️.rs`, that belong to crate `decl-views` (WP2).\n'
           '//!\n'
           '//! WP3 owns the source file and cuts it here: the impl names `ChildContentView` (a `decl-views` type) and `RetireOwned` is foreign, so the orphan rule keeps the impl with the type; the plan listed the file under `composition-emission`, whose crate could not host it. The parent `📜️declaration/🪆️child/🦀️.rs` mounted the file by `include!`; it now mounts this lift as a child module. Not in Appendix J.\n')
    body = rebase_literals(text, os.path.dirname(src), os.path.dirname(dest))
    os.makedirs(os.path.dirname(dest), exist_ok=True)
    write(dest, doc + 'use super::*;\n\n' + body.lstrip('\n').rstrip('\n') + '\n')
    inc = 'include!("../../🧩️composition/♻️root/👁️read/🦀️.rs");'
    ptxt = rd(parent)
    assert ptxt.count(inc) == 1
    write(parent, ptxt.replace(inc, '#[path = "🛬️lifted/👁️read/🦀️.rs"]\nmod lifted_read;\npub use lifted_read::*;'))
    delete(src)
    print('L10 done', len(text.split('\n')), 'lines')
except BaseException:
    rollback(); raise
