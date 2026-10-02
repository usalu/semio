"""🪣️ Independent Python flood-fill oracle for the shared pixel-editing corpus (`🔲️pixels/✍️editing/🧫️fixtures/🔣️.json`,
section `floodSelections`): 4-connected region from a seed whose RGBA channels all lie within `tolerance` of the seed's,
gated and weighted by an optional selection coverage. Writes the section; the Rust `flood_selection` and the TypeScript
`floodSelection` are both checked against it (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, S3-STROKES)."""
import json
import sys
from collections import deque

CORPUS = "🧰️framework/🔨️modules/🔲️pixels/✍️editing/🧫️fixtures/🔣️.json"


def flood(width, height, pixels, seed, tolerance, selection=None):
    target = pixels[(seed[1] * width + seed[0]) * 4:(seed[1] * width + seed[0]) * 4 + 4]
    mask = [0] * (width * height)
    seen = [False] * (width * height)
    queue = deque([seed[1] * width + seed[0]])
    seen[queue[0]] = True
    while queue:
        index = queue.popleft()
        color = pixels[index * 4:index * 4 + 4]
        if selection is not None and selection[index] == 0:
            continue
        if max(abs(a - b) for a, b in zip(color, target)) > tolerance:
            continue
        mask[index] = 255 if selection is None else selection[index]
        x, y = index % width, index // width
        for nx, ny in ((x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)):
            if 0 <= nx < width and 0 <= ny < height and not seen[ny * width + nx]:
                seen[ny * width + nx] = True
                queue.append(ny * width + nx)
    return mask


def px(*colors):
    return [channel for color in colors for channel in color]


RED, BLACK, WHITE = (255, 0, 0, 255), (0, 0, 0, 255), (255, 255, 255, 255)
GREY = (10, 10, 10, 255)
CASES = [
    ("A zero tolerance takes only the seed's exact colour", 3, 1, px(RED, BLACK, RED), (0, 0), 0, None),
    ("The full tolerance takes every reachable pixel", 3, 1, px(RED, BLACK, RED), (0, 0), 255, None),
    ("Regions are 4-connected: a diagonal neighbour of the same colour is not reached", 3, 3, px(WHITE, BLACK, WHITE, BLACK, WHITE, BLACK, WHITE, BLACK, WHITE), (0, 0), 0, None),
    ("Alpha counts in the channel distance", 2, 1, px(GREY, (10, 10, 10, 200)), (0, 0), 54, None),
    ("A distance equal to the tolerance is taken", 2, 1, px(GREY, (10, 10, 10, 200)), (0, 0), 55, None),
    ("An unselected pixel is never taken and blocks the region behind it", 3, 1, px(GREY, GREY, GREY), (0, 0), 0, [255, 0, 128]),
    ("A selected pixel is taken at its selection coverage", 3, 1, px(GREY, GREY, GREY), (1, 0), 0, [200, 100, 50]),
    ("A wall splits a region; the seed's side is taken", 4, 2, px(GREY, BLACK, GREY, GREY, GREY, BLACK, GREY, GREY), (3, 1), 0, None),
    ("A seed on an unselected pixel takes nothing", 2, 1, px(GREY, GREY), (1, 0), 255, [255, 0]),
]


def main():
    corpus = json.load(open(CORPUS, encoding="utf-8"))
    rows = []
    for name, width, height, pixels, seed, tolerance, selection in CASES:
        row = {"name": name, "image": {"width": width, "height": height, "pixels": pixels}, "seed": list(seed), "tolerance": tolerance}
        if selection is not None:
            row["selection"] = selection
        row["expected"] = flood(width, height, pixels, seed, tolerance, selection)
        rows.append(row)
    corpus["floodSelections"] = rows
    text = json.dumps(corpus, indent=2, ensure_ascii=False) + "\n"
    if "--check" in sys.argv:
        sys.exit(0 if open(CORPUS, encoding="utf-8").read() == text else 1)
    open(CORPUS, "w", encoding="utf-8").write(text)


main()
