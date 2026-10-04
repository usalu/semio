"""📡️ Ticket tool: writes the shared vectors of the React suite `📡️surface-survey` of the pets product.

Usage (from the repository root):
    python ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_survey_vectors.py"

Every scene is a small document tree with the box of each element, and the `surveyed` event a stage must receive for
it. The expectation is computed here by a second reading of design §6.3 and of the rules work package N added to it
(`📓️report-wp-n.md`), written from the text only (never from `🎯️targets/⚛️react/🔨️modules/📡️survey/🟦️.ts`): which
elements are controls or text blocks is decided by tag and attribute tables, not by a selector engine, and the
geometry is plain interval arithmetic. All boxes are multiples of 0.5 px, so every number is exact in binary floating
point and the suite compares without a tolerance.

The rules of work package N: everything is cut down to what its clipping ancestors let one see before it counts (a
keep-out that is scrolled away blocks nothing); the visible box of every surface element is a keep-out of its own,
grown by the keep-out margin to the left and the right only (pets stand on things, never in front of them or shoulder
to shoulder with their sides — which also cuts the edge of a body where a tab stands on it);
and the margin of a keep-out never reaches over an edge the element lies under (text at the top of a body does not
block the edge of that body), except for a focused surface, which keeps its whole halo.

The walls of the second round (design-v2 §16, §21, work package C1a): the left and the right side of every surface
element, then of every other element that the host names as a wall, each in document order, left before right, from the top
of what one can see of the element down to its bottom, but only where that side itself can be seen (a side a scrolling
ancestor cuts away is no wall), and only for an element of which something shows on the stage. A wall is named by its
element and its side (`-1` the left side, its air on the left; `1` the right side); its `surface` is the id of the
same element. An element the host names as a wall only is a wall and nothing else: no surface, no solid, no keep-out.

The fixtures of the second round (design-v2 §20, §21, work package C1b): the elements the host names as props
(`data-pet-prop` unless the scene names other attributes) that carry a non-empty `data-pet-prop` key, in document
order, with their whole box — but only an element that is neither inert nor hidden, that shows whole (no clipping
ancestor cuts any of it and it lies wholly on the stage), that is neither the focused element nor an ancestor of it,
that does not lie under the pointer (edges included) and that a copy can stand in for: no table part (`tr`, `td`, `th`,
`thead`, `tbody`, `tfoot`, `caption`, `col`, `colgroup`), nothing in it a canvas, a video, a sound, an inline frame,
an object, an embed or a custom element (a tag with a hyphen), and at most 80 elements in all, itself included.
"""

import io
import json
import os

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
OUT = os.path.join(ROOT, "🧰️framework", "🛍️products", "🐾️pets", "🧫️fixtures", "📡️surface-survey", "🔣️.json")

SURFACE_WIDTH = 48
KEEPOUT_MARGIN = 4
FOCUS_MARGIN = 8
FIXTURE_ELEMENTS = 80

UNCOPYABLE_TAGS = {"canvas", "video", "audio", "iframe", "object", "embed"}
TABLE_PART_TAGS = {"tr", "td", "th", "thead", "tbody", "tfoot", "caption", "col", "colgroup"}
CONTROL_TAGS = {"button", "input", "select", "textarea", "summary"}
CONTROL_ROLES = {"button", "link", "checkbox", "radio", "tab", "menuitem", "option", "slider", "switch"}
TEXT_TAGS = {"p", "li", "h1", "h2", "h3", "h4", "h5", "h6", "label", "figcaption", "td", "th", "legend", "dt", "dd", "blockquote", "pre", "code", "output", "progress", "meter"}
TEXT_ROLES = {"alert", "status"}


def node(key, tag, box=None, attributes=None, overflow=None, children=None):
    """🌳️ One element of a scene: `box` is (left, top, width, height) in viewport pixels, none means no box at all."""
    made = {"key": key, "tag": tag, "attributes": attributes or {}}
    if box is not None:
        made["box"] = {"left": box[0], "top": box[1], "width": box[2], "height": box[3]}
    if overflow is not None:
        made["overflow"] = overflow
    made["children"] = children or []
    return made


def walk(nodes, ancestors=()):
    """🚶️ Every element in document order with its ancestors, outermost first."""
    for item in nodes:
        yield item, ancestors
        yield from walk(item["children"], ancestors + (item,))


def is_control(item):
    attributes = item["attributes"]
    if item["tag"] in CONTROL_TAGS:
        return True
    if item["tag"] == "a" and "href" in attributes:
        return True
    if attributes.get("role") in CONTROL_ROLES:
        return True
    if "tabindex" in attributes and attributes["tabindex"] != "-1":
        return True
    return "contenteditable" in attributes and attributes["contenteditable"] != "false"


def default_keepout(item):
    return is_control(item) or item["tag"] in TEXT_TAGS or item["attributes"].get("role") in TEXT_ROLES or "data-pet-keepout" in item["attributes"]


def unseen(item, ancestors):
    return any("inert" in each["attributes"] or "hidden" in each["attributes"] for each in ancestors + (item,))


def edges(item):
    box = item.get("box", {"left": 0, "top": 0, "width": 0, "height": 0})
    return box["left"], box["top"], box["left"] + box["width"], box["top"] + box["height"]


def clip_of(ancestors):
    """✂️ The intersection of the boxes of every ancestor that clips its overflow, none when nothing clips."""
    clip = None
    for each in ancestors:
        if each.get("overflow", "visible") == "visible":
            continue
        left, top, right, bottom = edges(each)
        clip = (left, top, right, bottom) if clip is None else (max(clip[0], left), max(clip[1], top), min(clip[2], right), min(clip[3], bottom))
    return clip


def visible(item, ancestors):
    """👀️ What one can see of an element: its box cut down to what its clipping ancestors let through, none when it has no area or nothing of it shows."""
    left, top, right, bottom = edges(item)
    if right - left <= 0 or bottom - top <= 0:
        return None
    clip = clip_of(ancestors)
    if clip is not None:
        left, top, right, bottom = max(left, clip[0]), max(top, clip[1]), min(right, clip[2]), min(bottom, clip[3])
    return (left, top, right, bottom) if right > left and bottom > top else None


def staged(region, stage):
    """⬛️ A region of the viewport as a box in stage pixels, none when it misses the stage."""
    x = region[0] - stage[0]
    y = region[1] - stage[1]
    width = region[2] - region[0]
    height = region[3] - region[1]
    if x < stage[2] and x + width > 0 and y < stage[3] and y + height > 0:
        return {"x": x, "y": y, "width": width, "height": height}
    return None


def grown(region, margin, lines, stage):
    """🚧️ A visible region grown by `margin` in stage pixels. Its margin never reaches over an edge it lies under: among the
    edges (`lines`, viewport pixels) that span it and lie at or above its top (within half a pixel), the lowest one caps the top."""
    top = region[1] - margin
    for line in lines:
        if region[1] >= line["y"] - 0.5 and top < line["y"] and region[0] < line["x1"] and region[2] > line["x0"]:
            top = line["y"]
    return staged((region[0] - margin, top, region[2] + margin, region[3] + margin), stage)


def walls_of(scene, stage, is_surface, is_wall):
    """🧱️ The walls of a scene: both sides of every visible surface element where that side shows, in document order, left before right; then those of the elements that are walls only."""
    walls = []
    for wanted in (is_surface, lambda item: is_wall(item) and not is_surface(item)):
        for item, ancestors in walk(scene["nodes"]):
            if not wanted(item) or unseen(item, ancestors):
                continue
            region = visible(item, ancestors)
            if region is None or staged(region, stage) is None:
                continue
            left, _, right, _ = edges(item)
            for side, x, shows in ((-1, left, region[0] == left), (1, right, region[2] == right)):
                if shows:
                    walls.append({"element": item["key"], "side": side, "x": x - stage[0], "y0": region[1] - stage[1], "y1": region[3] - stage[1]})
    return walls


def elements(item):
    """🔢️ How many elements a subtree holds, its root included."""
    return 1 + sum(elements(child) for child in item["children"])


def copyable(item):
    """🪞️ Whether a copy can stand in for an element: no table part, nothing uncopyable or custom in it, at most the limit of elements."""
    if item["tag"] in TABLE_PART_TAGS:
        return False
    if any("-" in each["tag"] or each["tag"] in UNCOPYABLE_TAGS for each, _ in walk([item])):
        return False
    return elements(item) <= FIXTURE_ELEMENTS


def fixtures_of(scene, stage, is_prop):
    """🧸️ The fixtures of a scene: the props with a key that show whole on the stage, hold no focus, lie not under the pointer and can be copied, in document order."""
    holders = set()
    for item, ancestors in walk(scene["nodes"]):
        if item["key"] == scene.get("focus"):
            holders = {item["key"]} | {each["key"] for each in ancestors}
    pointer = scene.get("pointer")
    fixtures = []
    for item, ancestors in walk(scene["nodes"]):
        key = item["attributes"].get("data-pet-prop", "")
        if not is_prop(item) or key == "" or unseen(item, ancestors):
            continue
        box = edges(item)
        if visible(item, ancestors) != box:
            continue
        left, top, right, bottom = box
        if left < stage[0] or top < stage[1] or right > stage[0] + stage[2] or bottom > stage[1] + stage[3]:
            continue
        if item["key"] in holders:
            continue
        if pointer is not None and left <= pointer["x"] <= right and top <= pointer["y"] <= bottom:
            continue
        if copyable(item):
            fixtures.append({"element": item["key"], "key": key, "x": left - stage[0], "y": top - stage[1], "width": right - left, "height": bottom - top})
    return fixtures


def expectation(scene, is_surface, is_keepout, is_wall=lambda item: False, is_prop=lambda item: "data-pet-prop" in item["attributes"]):
    viewport = scene["viewport"]
    frame = scene.get("frame")
    stage = (frame["left"], frame["top"], frame["width"], frame["height"]) if frame and frame["width"] > 0 and frame["height"] > 0 else (0, 0, viewport["width"], viewport["height"])
    grounds = set()
    carriers = set()
    surfaces = []
    solids = []
    lines = []
    for item, ancestors in walk(scene["nodes"]):
        if not is_surface(item) or unseen(item, ancestors):
            continue
        grounds.add(item["key"])
        carriers.update(each["key"] for each in ancestors)
        region = visible(item, ancestors)
        if region is None or staged(region, stage) is None:
            continue
        solids.append({"x": region[0] - stage[0] - KEEPOUT_MARGIN, "y": region[1] - stage[1], "width": region[2] - region[0] + 2 * KEEPOUT_MARGIN, "height": region[3] - region[1]})
        if region[1] > edges(item)[1] or region[2] - region[0] < SURFACE_WIDTH:
            continue
        surfaces.append({"element": item["key"], "x0": region[0] - stage[0], "x1": region[2] - stage[0], "y": region[1] - stage[1]})
        lines.append({"x0": region[0], "x1": region[2], "y": region[1]})
    surfaces.append({"element": "floor", "x0": 0, "x1": stage[2], "y": stage[3]})
    lines.append({"x0": stage[0], "x1": stage[0] + stage[2], "y": stage[1] + stage[3]})
    keepouts = []
    for item, ancestors in walk(scene["nodes"]):
        if not is_keepout(item) or item["key"] in grounds or item["key"] in carriers or unseen(item, ancestors):
            continue
        region = visible(item, ancestors)
        box = None if region is None else grown(region, KEEPOUT_MARGIN, lines, stage)
        if box is not None:
            keepouts.append(box)
    keepouts.extend(solids)
    for item, ancestors in walk(scene["nodes"]):
        if item["key"] != scene.get("focus") or item["key"] in carriers or unseen(item, ancestors):
            continue
        region = visible(item, ancestors)
        box = None if region is None else grown(region, FOCUS_MARGIN, [] if item["key"] in grounds else lines, stage)
        if box is not None:
            keepouts.append(box)
    return {"kind": "surveyed", "width": stage[2], "height": stage[3], "surfaces": surfaces, "keepouts": keepouts, "walls": walls_of(scene, stage, is_surface, is_wall), "fixtures": fixtures_of(scene, stage, is_prop)}


def marked(attribute):
    return lambda item: attribute in item["attributes"]


def scene(name, nodes, viewport=(1200, 800), frame=None, focus=None, surfaces=None, keepouts=None, walls=None, props=None, pointer=None):
    """🎬️ A scene with its expectation; `surfaces` and `keepouts` name the attributes that replace or extend the defaults, `walls` those of the elements that are walls besides the surfaces, `props` those that replace the default props; `pointer` is where the pointer rests in viewport pixels."""
    made = {"name": name, "viewport": {"width": viewport[0], "height": viewport[1]}, "nodes": nodes}
    if frame is not None:
        made["frame"] = {"left": frame[0], "top": frame[1], "width": frame[2], "height": frame[3]}
    if focus is not None:
        made["focus"] = focus
    if pointer is not None:
        made["pointer"] = {"x": pointer[0], "y": pointer[1]}
    options = {}
    is_surface = marked("data-pet-surface")
    is_keepout = default_keepout
    if surfaces is not None:
        options["surfaces"] = ", ".join(f"[{attribute}]" for attribute in surfaces)
        is_surface = lambda item: any(attribute in item["attributes"] for attribute in surfaces)
    if keepouts is not None:
        options["keepoutsBesidesDefaults"] = ", ".join(f"[{attribute}]" for attribute in keepouts)
        is_keepout = lambda item: default_keepout(item) or any(attribute in item["attributes"] for attribute in keepouts)
    is_wall = lambda item: False
    if walls is not None:
        options["walls"] = ", ".join(f"[{attribute}]" for attribute in walls)
        is_wall = lambda item: any(attribute in item["attributes"] for attribute in walls)
    is_prop = marked("data-pet-prop")
    if props is not None:
        options["props"] = ", ".join(f"[{attribute}]" for attribute in props)
        is_prop = lambda item: any(attribute in item["attributes"] for attribute in props)
    if options:
        made["options"] = options
    made["expected"] = expectation(made, is_surface, is_keepout, is_wall, is_prop)
    return made


SURFACE = {"data-pet-surface": ""}


def card(key, box, extra=None, children=None):
    attributes = dict(SURFACE)
    attributes.update(extra or {})
    return node(key, "article", box, attributes, children=children)


SCENES = [
    scene(
        "cards carry pets, the floor closes the list, controls and text are kept free",
        [
            node("header", "header", (0, 0, 1200, 56), children=[
                node("brand", "a", (16, 12, 120, 32), {"href": "#"}),
                node("menu", "button", (1100, 12, 84, 32)),
            ]),
            node("main", "main", (0, 56, 1200, 744), children=[
                card("first", (100, 200, 300, 180), children=[
                    node("first-title", "h2", (116, 216, 268, 28)),
                    node("first-text", "p", (116, 252.5, 268, 60)),
                    node("first-go", "button", (116, 330, 96, 32)),
                ]),
                card("second", (500.5, 260, 400, 200), children=[
                    node("second-title", "h2", (516.5, 276, 368, 28)),
                    node("second-list", "ul", (516.5, 312, 368, 96), children=[
                        node("second-one", "li", (516.5, 312, 368, 24)),
                        node("second-two", "li", (516.5, 336, 368, 24)),
                    ]),
                ]),
                node("plain", "div", (100, 500, 300, 100)),
                node("note", "div", (950, 300, 200, 40), {"data-pet-keepout": ""}),
            ]),
        ],
    ),
    scene(
        "inert pages and hidden parts carry nothing and keep nothing free",
        [
            node("backdrop", "section", (0, 0, 600, 800), {"inert": "", "aria-hidden": "true"}, children=[
                card("behind", (40, 100, 300, 200), children=[node("behind-text", "p", (56, 116, 268, 40))]),
                node("behind-button", "button", (40, 320, 96, 32)),
            ]),
            node("drawer", "div", (600, 0, 600, 300), {"hidden": ""}, children=[
                card("folded", (640, 40, 300, 200)),
                node("folded-text", "p", (640, 250, 300, 40)),
            ]),
            card("gone", (640, 320, 300, 100), {"hidden": ""}),
            card("alive", (640, 480, 300, 200), children=[node("alive-text", "p", (656, 496, 268, 40))]),
        ],
    ),
    scene(
        "narrow, boxless and off-stage elements are no surfaces, a surface may reach beyond the stage",
        [
            card("narrow", (20, 100, 47.5, 100)),
            card("just-wide-enough", (100, 100, 48, 100)),
            card("boxless", None),
            card("right-of-stage", (1200, 300, 300, 100)),
            card("below-stage", (100, 800, 300, 100)),
            card("above-stage", (100, -200, 300, 200)),
            card("reaching-left", (-50, 400, 300, 100)),
            card("reaching-down", (500, 760, 300, 100)),
            node("far-text", "p", (1300, 20, 200, 40)),
            node("edge-text", "p", (1196, 20, 200, 40)),
            node("empty-text", "p", (300, 20, 0, 40)),
        ],
    ),
    scene(
        "a scrolling pane lets only the edges through that it shows",
        [
            node("pane", "div", (100, 100, 600, 400), overflow="auto", children=[
                node("content", "div", (100, 20, 600, 900), children=[
                    card("scrolled-away", (120, 60, 300, 100)),
                    card("shown", (120, 200, 300, 100)),
                    card("cut-left", (40, 320, 300, 100)),
                    card("cut-to-a-sliver", (660, 320, 300, 100)),
                    card("at-the-lower-edge", (120, 500, 300, 100)),
                    card("below", (120, 620, 300, 100)),
                ]),
            ]),
            node("clipped-card", "div", (800, 100, 300, 300), overflow="hidden", children=[
                node("inner-pane", "div", (800, 150, 300, 200), overflow="scroll", children=[
                    card("under-both", (780, 180, 400, 80)),
                    card("above-the-inner", (820, 120, 200, 40)),
                ]),
            ]),
        ],
    ),
    scene(
        "a surface and whatever contains it are never keep-outs, what it contains is",
        [
            node("list", "ul", (0, 100, 1200, 300), children=[
                node("entry", "li", (20, 100, 400, 300), children=[
                    node("link", "a", (20, 100, 400, 300), {"href": "#first"}, children=[
                        card("pressable", (20, 100, 400, 300), {"role": "button", "tabindex": "0"}, children=[
                            node("pressable-text", "p", (36, 116, 368, 40)),
                        ]),
                    ]),
                ]),
                node("other-entry", "li", (440, 100, 400, 300)),
            ]),
        ],
    ),
    scene(
        "the focused control is kept free twice, by its own box and by the wider focus halo",
        [
            card("host", (100, 200, 400, 300), children=[
                node("field", "input", (116, 216, 200, 32)),
                node("send", "button", (116, 260, 96, 32)),
            ]),
        ],
        focus="send",
    ),
    scene(
        "a focused surface is kept free although it carries pets otherwise",
        [
            card("focused-card", (100, 200, 400, 300), {"tabindex": "0"}),
            card("other-card", (600, 200, 400, 300), {"tabindex": "0"}),
        ],
        focus="focused-card",
    ),
    scene(
        "a focused region that contains surfaces keeps nothing free",
        [
            node("region", "main", (0, 56, 1200, 744), {"tabindex": "-1"}, children=[
                card("inside", (100, 200, 400, 300)),
                node("inside-text", "p", (600, 200, 400, 60)),
            ]),
        ],
        focus="region",
    ),
    scene(
        "the host may name its own surfaces and more keep-outs",
        [
            node("quiz-main", "main", (0, 56, 1200, 744), children=[
                node("layered", "div", (40, 100, 320, 200), {"data-layered-card": ""}),
                node("page-card", "section", (400, 100, 320, 200), {"data-card": ""}),
                node("unmarked", "article", (760, 100, 320, 200), {"data-pet-surface": ""}),
                node("item", "div", (400, 340, 320, 48), {"data-quiz-item": ""}),
                node("drop", "div", (400, 400, 320, 48), {"data-quiz-drop": ""}),
                node("words", "p", (40, 340, 320, 48)),
            ]),
        ],
        surfaces=["data-layered-card", "data-card"],
        keepouts=["data-quiz-item", "data-quiz-drop"],
    ),
    scene(
        "a framed stage measures from its own corner",
        [
            card("in-frame", (300, 250, 300, 150), children=[node("in-frame-text", "p", (316, 266, 268, 40))]),
            card("left-of-frame", (20, 250, 150, 150)),
            card("across-the-frame-edge", (900, 300, 300, 150)),
            node("outside-text", "p", (20, 20, 150, 40)),
        ],
        frame=(200, 100, 800, 500),
    ),
    scene(
        "a frame without an area falls back to the viewport",
        [card("only", (100, 200, 300, 150))],
        viewport=(1024, 600),
        frame=(0, 0, 0, 0),
    ),
    scene(
        "a silhouette of parts: the tab and the body each carry pets, each part is solid, and what lies under an edge does not reach over it",
        [
            node("bar", "header", (0, 0, 1200, 30), {"data-bar": ""}, children=[
                node("mark", "span", (4, 4, 22, 22)),
            ]),
            node("window", "section", (100, 199.5, 400, 150.5), children=[
                node("tab", "div", (100, 199.5, 90, 26.5), {"data-part": ""}, children=[
                    node("tab-title", "h2", (122, 202.5, 64, 19)),
                ]),
                node("body", "div", (100, 226, 400, 100), {"data-part": ""}),
                node("content", "div", (100, 199.5, 400, 150.5), children=[
                    node("first-line", "p", (107, 226, 386, 40)),
                    node("second-line", "p", (107, 270, 386, 16)),
                ]),
                node("action", "button", (420, 326, 80, 24)),
            ]),
            node("next", "section", (600, 199.5, 300, 100), children=[
                node("next-tab", "div", (600, 199.5, 40, 26.5), {"data-part": ""}),
                node("next-body", "div", (600, 226, 300, 73.5), {"data-part": ""}),
                node("caption", "p", (660, 220, 100, 4)),
            ]),
            node("footer", "footer", (0, 774, 1200, 26), {"data-part": ""}, children=[
                node("legal", "a", (1100, 778, 90, 18), {"href": "#legal"}),
            ]),
        ],
        surfaces=["data-part"],
        keepouts=["data-bar"],
    ),
    scene(
        "what a pane scrolls away blocks nothing, and what it cuts blocks only where it shows",
        [
            node("list", "div", (0, 100, 600, 500), overflow="auto", children=[
                node("above", "p", (20, 20, 300, 40)),
                node("across-the-top", "p", (20, 80, 300, 40)),
                node("inside", "button", (20, 300, 96, 32)),
                node("across-the-bottom", "p", (20, 580, 300, 40)),
                node("below", "h2", (20, 640, 300, 40)),
                node("off-to-the-right", "a", (620, 300, 96, 32), {"href": "#"}),
                card("cut-below", (340, 500, 200, 300)),
                card("cut-above", (340, 40, 200, 100)),
            ]),
            node("clipped", "div", (700, 100, 300, 300), overflow="hidden", children=[
                node("wide", "p", (650, 150, 400, 40)),
            ]),
            node("free", "p", (700, 500, 300, 40)),
        ],
    ),
    scene(
        "a focused control under an edge keeps its halo below that edge",
        [
            card("holder", (100, 200, 400, 300), children=[
                node("top-field", "input", (116, 200, 200, 32)),
            ]),
            card("neighbour", (600, 200, 400, 300), {"tabindex": "0"}),
        ],
        focus="top-field",
    ),
    scene(
        "read-outs and announcements are kept free, bare text only where the host marks it",
        [
            node("notice", "div", (100, 100, 500, 40), {"role": "alert"}, children=[
                node("notice-detail", "code", (116, 108, 200, 24)),
            ]),
            node("saved", "span", (700, 100, 120, 24), {"role": "status"}),
            node("done", "progress", (100, 260, 200, 16)),
            node("level", "meter", (350, 260, 120, 16)),
            node("sum", "output", (520, 260, 80, 24)),
            node("bare", "div", (700, 260, 200, 24)),
            node("marked", "span", (950, 260, 120, 24), {"data-pet-keepout": ""}),
            node("decoration", "span", (100, 400, 120, 24), {"role": "img"}),
            card("below", (100, 300, 900, 200)),
        ],
    ),
    scene(
        "walls: both sides of every part of a silhouette, from its top down to its bottom, a narrow part included",
        [
            node("window", "section", (100.5, 199.5, 400, 150.5), children=[
                node("window-tab", "div", (100.5, 199.5, 90, 26.5), {"data-part": ""}, children=[
                    node("window-title", "h2", (122, 202.5, 64, 19)),
                ]),
                node("window-body", "div", (100.5, 226, 400, 124), {"data-part": ""}, children=[
                    node("window-text", "p", (107, 232, 386, 40)),
                    node("window-go", "button", (420, 318, 72, 24)),
                ]),
            ]),
            node("chip", "div", (620, 260, 32, 20), {"data-part": ""}),
            node("footer", "footer", (0, 774, 1200, 26), {"data-part": ""}),
        ],
        surfaces=["data-part"],
    ),
    scene(
        "walls in a scrolling pane: a side the pane cuts away is no wall, a top it cuts shortens the wall, and what it scrolls away has none",
        [
            node("pane", "div", (100, 100, 600, 400), overflow="auto", children=[
                node("pane-content", "div", (100, 20, 600, 900), children=[
                    card("pane-gone", (120, 20, 300, 60)),
                    card("pane-under-the-top", (450, 60, 200, 100)),
                    card("pane-whole", (120, 200, 300, 100)),
                    card("pane-cut-left", (40, 320, 200, 60)),
                    card("pane-cut-right", (560, 320, 300, 60)),
                    card("pane-under-the-bottom", (300, 450, 200, 120)),
                    card("pane-below", (120, 560, 300, 100)),
                ]),
            ]),
        ],
    ),
    scene(
        "the host may name more walls: their sides are walls and nothing else, measured from a framed stage's corner, and a wall element that is hidden has none",
        [
            node("rail", "aside", (40, 120, 120, 500), {"data-pet-wall": ""}),
            node("rail-hidden", "aside", (1000, 120, 120, 500), {"data-pet-wall": "", "hidden": ""}),
            card("frame-card", (300, 250, 300, 150), {"data-pet-wall": ""}, children=[node("frame-card-text", "p", (316, 266, 268, 40))]),
            node("rail-off-stage", "aside", (1040, 700, 100, 80), {"data-pet-wall": ""}),
        ],
        frame=(20, 100, 1000, 600),
        walls=["data-pet-wall"],
    ),
    scene(
        "fixtures: the rows a pet may play with, by their key, whole and in document order; a row without a key, a table row and what holds a film, a custom element or more than eighty elements are none, a surface may be one",
        [
            node("tasks", "ol", (300, 120, 600, 220), children=[
                node("task-1", "li", (300, 120, 600, 30), {"data-pet-prop": "heating/u-values"}, children=[
                    node("task-1-index", "span", (306, 125, 16, 20)),
                    node("task-1-glyph", "svg", (330, 126, 18, 18)),
                    node("task-1-title", "span", (356, 125, 300, 20)),
                ]),
                node("task-2", "li", (300, 153.5, 600, 30), {"data-pet-prop": "heating/heating-load-and-demand"}),
                node("task-keyless", "li", (300, 187, 600, 30), {"data-pet-prop": ""}),
                node("task-film", "li", (300, 220.5, 600, 30), {"data-pet-prop": "heating/film"}, children=[
                    node("task-film-holder", "span", (306, 222, 60, 26), children=[node("task-film-video", "video", (306, 222, 40, 26))]),
                ]),
                node("task-custom", "li", (300, 254, 600, 30), {"data-pet-prop": "heating/custom"}, children=[node("task-custom-glyph", "quiz-glyph", (306, 256, 20, 20))]),
                node("task-crowded", "li", (300, 287.5, 600, 30), {"data-pet-prop": "heating/crowded"}, children=[node(f"task-crowded-{index}", "i") for index in range(FIXTURE_ELEMENTS)]),
                node("task-full", "li", (300, 321, 600, 19), {"data-pet-prop": "heating/full"}, children=[node(f"task-full-{index}", "i") for index in range(FIXTURE_ELEMENTS - 1)]),
            ]),
            node("table", "table", (300, 400, 600, 60), children=[
                node("table-body", "tbody", (300, 400, 600, 60), children=[
                    node("table-row", "tr", (300, 400, 600, 30), {"data-pet-prop": "heating/row"}, children=[node("table-cell", "td", (300, 400, 600, 30))]),
                ]),
            ]),
            card("badge", (950, 150, 120, 60), {"data-pet-prop": "heating"}),
        ],
    ),
    scene(
        "fixtures: what a scrolling pane cuts, what lies off the stage, what is inert or hidden, what holds the focus and what lies under the pointer — its very edge included — are none",
        [
            node("pane", "div", (100, 100, 400, 200), overflow="auto", children=[
                node("pane-content", "div", (100, 100, 400, 400), children=[
                    node("pane-whole", "div", (110, 110, 380, 40), {"data-pet-prop": "cooling/air-change-rates"}),
                    node("pane-cut", "div", (110, 270, 380, 40), {"data-pet-prop": "cooling/cooling-load-and-demand"}),
                    node("pane-gone", "div", (110, 350, 380, 40), {"data-pet-prop": "cooling"}),
                ]),
            ]),
            node("off-stage", "div", (1100, 50, 200, 30), {"data-pet-prop": "physics"}),
            node("backstage", "section", (600, 100, 300, 100), {"inert": ""}, children=[node("backstage-row", "div", (610, 110, 280, 30), {"data-pet-prop": "physics/powers"})]),
            node("folded-row", "div", (600, 250, 300, 30), {"data-pet-prop": "physics/energies", "hidden": ""}),
            node("focus-row", "div", (600, 400, 300, 40), {"data-pet-prop": "demand/final-energy"}, children=[node("focus-field", "input", (610, 405, 100, 30))]),
            node("pointer-row", "div", (600, 500, 300, 40), {"data-pet-prop": "demand/standard-profiles"}),
            node("free-row", "div", (600, 600, 300, 40), {"data-pet-prop": "demand"}),
        ],
        focus="focus-field",
        pointer=(900, 540),
    ),
    scene(
        "fixtures: a host may name its own props — the key still comes from data-pet-prop — measured from a framed stage's corner",
        [
            node("toy", "div", (200, 100, 300, 40), {"data-toy": "", "data-pet-prop": "demand"}),
            node("toy-keyless", "div", (200, 200, 300, 40), {"data-toy": ""}),
            node("prop-no-toy", "div", (200, 300, 300, 40), {"data-pet-prop": "demand/final-energy"}),
            node("toy-beyond-frame", "div", (50, 400, 100, 40), {"data-toy": "", "data-pet-prop": "demand/standard-profiles"}),
            node("toy-last", "div", (700, 500.5, 250, 40.5), {"data-toy": "", "data-pet-prop": "cooling"}),
        ],
        frame=(100, 50, 1000, 700),
        props=["data-toy"],
    ),
]

DOCUMENT = {
    "$comment":"📡️ Shared vectors of the React suite 🧪️tests/📡️surface-survey: document trees with the box of every element and the `surveyed` event a stage must receive for them. Generated by `python .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_survey_vectors.py` from a second reading of design §6.3 written in Python (tag and attribute tables instead of a selector engine, interval arithmetic); never edited by hand. `expected.surfaces[].element` names the element of the scene whose surface it is (`floor` for the floor); `options.keepoutsBesidesDefaults` extends the default keep-outs.",
    "margins": {"surfaceWidth": SURFACE_WIDTH, "keepout": KEEPOUT_MARGIN, "focus": FOCUS_MARGIN, "fixtureElements": FIXTURE_ELEMENTS},
    "scenes": SCENES,
}

os.makedirs(os.path.dirname(OUT), exist_ok=True)
with io.open(OUT, "w", encoding="utf-8", newline="\n") as handle:
    json.dump(DOCUMENT, handle, ensure_ascii=False, indent=2)
    handle.write("\n")
print(json.dumps({"scenes": len(SCENES), "surfaces": sum(len(each["expected"]["surfaces"]) for each in SCENES), "keepouts": sum(len(each["expected"]["keepouts"]) for each in SCENES), "walls": sum(len(each["expected"]["walls"]) for each in SCENES), "fixtures": sum(len(each["expected"]["fixtures"]) for each in SCENES)}))
