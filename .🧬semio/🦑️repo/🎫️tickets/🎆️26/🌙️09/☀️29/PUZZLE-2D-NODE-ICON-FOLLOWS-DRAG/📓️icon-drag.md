# Puzzle 2d Node Icon Follows Drag

Puzzle 2d paints node bodies live and node icons from `BoardHost.world_content_cache`. A drag updates `node.x` / `node.y` and bumps `content_scene_generation` on every move. The cache rebuild retires the previous icon scene one command at a time and refuses the next rebuild while that retirement is still open. Fills and strokes are painted from the live coordinates, so the circle moves. The icon scene stays on the center it was baked at.

The cache now records one span per node, baked at that node's center. While a newer generation is waiting on retirement, each span is drawn with a translation of `current - baked`, so the icon stays inside the node. A pure translation does not allocate a replacement icon scene.
