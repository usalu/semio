import sys
from pathlib import Path

root = Path("🧰️framework")
record = root / "🔨️modules/🌱️value/✨️derive/🛬️receive/🦀️.rs"
dsl = root / "🛍️products/💻️os/🔨️modules/🗣️dsl/🫴️receiving/🦀️.rs"
law = dsl.parent / "📦️pack-value/🧪️tests/🦀️.rs"
fixture = dsl.parent / "🧪️tests/🧫️fixtures/🏷️tagged.json"
sources = {p: p.read_text() for p in (record, dsl, law, fixture)}

def edit(p, old, new):
    text = sources[p]
    if text.count(old) != 1:
        raise SystemExit(f"cannot place receiving follow-up in {p}: {old!r}")
    sources[p] = text.replace(old, new)

edit(
    record,
    "  if q==0||q>maximum||!#value::retained_clone::retained_clone_grant_admits(grant,d)",
    "  if (q==0&&action!=3)||q>maximum||!#value::retained_clone::retained_clone_grant_admits(grant,d)",
)
edit(
    dsl,
    "pub fn receive_to_completion<M:OpBinaryReceiving>(source:&[u8],range:(usize,usize),mut stock:ReceivingStock)->Result<M,OriginalReceivingPrimary>{",
    """pub fn receive_to_completion<M:OpBinaryReceiving>(source:&[u8],range:(usize,usize),stock:ReceivingStock)->Result<M,OriginalReceivingPrimary>{
 receive_to_completion_with_body(source,range,stock,stock.quantum.maximum_copy_bytes)
}
fn receive_to_completion_with_body<M:OpBinaryReceiving>(source:&[u8],range:(usize,usize),mut stock:ReceivingStock,body:usize)->Result<M,OriginalReceivingPrimary>{""",
)
edit(dsl, "cursor.step_work_demand(source,stock.quantum.maximum_copy_bytes)", "cursor.step_work_demand(source,body)")
edit(dsl, "cursor.step(source,native,stock.quantum.maximum_copy_bytes,grant)", "cursor.step(source,native,body,grant)")
edit(
    law,
    "assert_eq!(receive_to_completion::<TaggedPackValueMutation>(&wire, (0, wire.len()), bounded).unwrap(), large);",
    "assert_eq!(super::super::receive_to_completion_with_body::<TaggedPackValueMutation>(&wire, (0, wire.len()), bounded, 1 << 14).unwrap(), large);",
)
edit(fixture, "[1, 8, 24, 48, 96]", "[1, 8, 24, 48, 96, 160, 256, 512]")
if sys.argv[1:] == ["--write"]:
    for path, contents in sources.items():
        path.write_text(contents)
print("Zero-Work record acknowledgement and independent bounded fixture body changes validated")
