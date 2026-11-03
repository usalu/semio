#!/usr/bin/env python3
"""Apply from the repo root in a patch window. `PagedProjection<P>` already places `P::Output` generically through
`ProjectionOutput::place`, but its `SemanticProjection` impl demanded `Output=P`; every derived `SnapshotProjection`
has `Output=Box<Self>`, so the boxed-output law (replay-operation native test) could not name the driver."""
p="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🔌️drivers/🦀️.rs"
s=open(p).read()
a="impl<P:SnapshotProjection<Output=P>> SemanticProjection<P> for PagedProjection<P>{"
assert s.count(a)==1
open(p,'w').write(s.replace(a,"impl<P:SnapshotProjection> SemanticProjection<P> for PagedProjection<P>{"))
