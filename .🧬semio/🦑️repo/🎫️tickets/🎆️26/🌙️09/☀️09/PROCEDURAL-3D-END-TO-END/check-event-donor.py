"""📩 Verify original event delivery against Python's independent FIFO implementation."""
from collections import deque
from pathlib import Path
path=Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧬️wit-bridge/📥️pending-events/🧪️tests/🧫️fixtures/crossings.txt")
queue=deque()
for line in path.read_text().splitlines():
    if line.startswith("#"):continue
    incoming,consume,selected,remaining=line.split(",")
    if incoming!="-":queue.append(incoming)
    assert (queue[0] if queue else "-")==selected
    if consume=="yes":queue.popleft()
    assert len(queue)==int(remaining)
print("Six crossings matched collections.deque")
