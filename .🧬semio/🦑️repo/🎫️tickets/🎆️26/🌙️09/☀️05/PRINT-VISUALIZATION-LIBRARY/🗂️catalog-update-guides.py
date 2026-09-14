"""🗂️ Rewrites the GUIDES-owned catalogue entries: taxonomy sections 50 and 77 and the `axis` kinds
of section 78. Reads the whole file immediately before writing it back atomically, because other
agents edit their own sections of the same file concurrently."""
import io, json, os, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.normpath(os.path.join(HERE, *([os.pardir] * 7)))
PATH = os.path.join(REPO, "\U0001f9f0️framework", "\U0001f6cd️products", "\U0001f4d3️print",
                    "\U0001f5bc️assets", "\U0001f523️viz-catalog.json")

# 📐 The §78 axis capabilities, one option set per capability.
AXIS = {
    "cartesian": {"variant": "cartesian", "ticks": 5, "grid": True},
    "polar": {"variant": "polar", "ticks": 6},
    "ternary": {"variant": "ternary", "ticks": 4},
    "geographic": {"variant": "geographic", "ticks": 5, "grid": True},
    "multiple-axes": {"variant": "multiple-axes", "ticks": 5, "y2": "val2"},
    "broken-axes": {"variant": "broken-axes", "ticks": 5, "broken": "3,6"},
}
# 🏷️ Titles for the taxonomy leaves the catalogue did not carry yet, in both languages.
TITLES = {
    "78/polar": {"en": "Polar", "de": "Polar"},
    "77/zoom-inset": {"en": "Zoom inset", "de": "Zoom-Einschub"},
    "77/small-multiples": {"en": "Small multiples", "de": "Kleine Vielfache"},
}

# 🗒️ The §50 annotation-driven kinds; each variant reaches a different annotation vocabulary.
ANNOTATION = {
    "annotated-chart": {"variant": "annotated-chart", "threshold": 6},
    "story-chart": {"variant": "story-chart"},
    "narrative-chart": {"variant": "narrative-chart"},
    "scrollytelling-style-static-sequence": {"variant": "scrollytelling-style-static-sequence"},
    "explainer-diagram": {"variant": "explainer-diagram"},
    "callout-chart": {"variant": "callout-chart", "emphasis": 2},
    "highlight-chart": {"variant": "highlight-chart", "emphasis": 4},
    "threshold-chart": {"variant": "threshold-chart", "threshold": 6},
    "reference-line-chart": {"variant": "reference-line-chart", "threshold": 5},
    "reference-band-chart": {"variant": "reference-band-chart", "bandLow": 3, "bandHigh": 6},
    "event-marker-chart": {"variant": "event-marker-chart", "event": 4},
    "label-rich-chart": {"variant": "label-rich-chart"},
    "directly-labeled-chart": {"variant": "directly-labeled-chart"},
}

# 🧩 The §77 composition kinds; each variant reaches a different composition operator.
COMPOSITION = {
    "layer": {"variant": "layer"},
    "overlay": {"variant": "overlay"},
    "facet": {"variant": "facet", "columns": 2},
    "repeat": {"variant": "repeat", "columns": 2},
    "concatenate-horizontally": {"variant": "concatenate-horizontally", "gap": 4},
    "concatenate-vertically": {"variant": "concatenate-vertically", "gap": 4},
    "grid-composition": {"variant": "grid-composition", "gap": 3},
    "nested-composition": {"variant": "nested-composition", "gap": 4},
    "inset": {"variant": "inset"},
    "zoom-inset": {"variant": "zoom-inset"},
    "shared-axes": {"variant": "shared-axes", "columns": 2},
    "independent-axes": {"variant": "independent-axes", "columns": 2},
    "shared-legend": {"variant": "shared-legend", "columns": 2},
    "independent-legends": {"variant": "independent-legends", "columns": 2},
    "linked-annotations": {"variant": "linked-annotations"},
    "cross-panel-reference-lines": {"variant": "cross-panel-reference-lines"},
    "small-multiples": {"variant": "small-multiples", "columns": 4, "gap": 2},
    "dashboard-assembly": {"variant": "dashboard-assembly", "gap": 3},
    "figure-subfigure-assembly": {"variant": "figure-subfigure-assembly", "gap": 4},
}


def sort_key(entry):
    section, _, slug = entry["id"].partition("/")
    return (int(section) if section.isdigit() else 10 ** 6, slug)


def main():
    path = os.path.normpath(PATH)
    with io.open(path, encoding="utf-8") as handle:
        catalog = json.load(handle)
    kinds = catalog["kinds"]
    by_id = {entry["id"]: entry for entry in kinds}
    changed = []

    def apply(section, kind, namespace, family, table):
        for slug, options in table.items():
            identifier = "%s/%s" % (section, slug)
            entry = by_id.get(identifier)
            if entry is None:
                if identifier not in TITLES:
                    raise SystemExit("no title for the new catalogue entry " + identifier)
                entry = {
                    "id": identifier,
                    "slug": slug,
                    "title": TITLES[identifier],
                    "kind": kind,
                    "namespace": namespace,
                    "family": family,
                    "options": {},
                    "data": "demo",
                    "covers": [identifier],
                }
                kinds.append(entry)
                by_id[identifier] = entry
            entry["kind"] = kind
            entry["namespace"] = namespace
            entry["family"] = family
            entry["options"] = dict(options)
            changed.append(identifier)

    apply("78", "axis", "kernel/guide", "axis", AXIS)
    apply("50", "chart", "kernel/annotation", "annotated-chart", ANNOTATION)
    apply("77", "layout", "kernel/composition", "composition", COMPOSITION)

    kinds.sort(key=sort_key)
    handle = tempfile.NamedTemporaryFile("w", encoding="utf-8", newline="\n", dir=os.path.dirname(path), delete=False, suffix=".tmp")
    json.dump(catalog, handle, ensure_ascii=False, indent=2)
    handle.write("\n")
    handle.close()
    os.replace(handle.name, path)
    print("updated %d entries" % len(changed))


if __name__ == "__main__":
    main()
