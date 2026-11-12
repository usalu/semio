import pathlib
import sys

path = pathlib.Path(sys.argv[1])
text = path.read_text(encoding="utf-8")


def swap(old, new):
    global text
    assert text.count(old) == 1, old
    text = text.replace(old, new)


swap('"endpoint": {\n      "type": "object",', '"endpoint": {\n      "type": ["object", "null"],')
swap('"required": ["id", "intent", "source", "target", "relation", "correspondence", "recipe", "timing"],', '"required": ["id", "intent", "source", "target", "relation", "recipe", "timing"],')
swap(
    '"recipe": {"enum": ["parts", "hull", "scatter", "gather", "split", "unfold", "retext", "flight", "move"], "x-code": "recipe", "x-fix": "use one of parts, hull, scatter, gather, split, unfold, retext, flight, move"},',
    '"recipe": {"enum": ["parts", "hull", "scatter", "gather", "split", "unfold", "retext", "flight", "move", "replace", "vanish", "appear"], "x-code": "recipe", "x-fix": "use one of parts, hull, scatter, gather, split, unfold, retext, flight, move, replace, vanish, appear"},\n        "copy": {"type": "boolean", "default": false, "x-code": "copy", "x-fix": "use true (source stays, Manim TransformFromCopy) or false (source is replaced)"},',
)
anchor = '        {"if": {"properties": {"relation": {"enum": ["one-to-many", "many-to-one"]}}'
extra = (
    '        {"if": {"properties": {"recipe": {"const": "replace"}}, "required": ["recipe"]}, "then": {"properties": {"relation": {"enum": ["one-to-one", "many-to-many"], "x-code": "recipe-relation", "x-fix": "recipe \\"replace\\" (Manim ReplacementTransform) swaps matched objects, so use relation one-to-one or many-to-many"}}}},\n'
    '        {"if": {"properties": {"recipe": {"const": "vanish"}}, "required": ["recipe"]}, "then": {"properties": {"relation": {"enum": ["one-to-one"], "x-code": "recipe-relation", "x-fix": "recipe \\"vanish\\" removes one entity, so use relation one-to-one"}, "target": {"type": "null", "x-code": "absence-endpoint", "x-fix": "recipe \\"vanish\\" morphs into nothing: set \\"target\\": null"}, "source": {"type": "object", "x-code": "absence-endpoint", "x-fix": "recipe \\"vanish\\" needs a source entity"}, "correspondence": false}}},\n'
    '        {"if": {"properties": {"recipe": {"const": "appear"}}, "required": ["recipe"]}, "then": {"properties": {"relation": {"enum": ["one-to-one"], "x-code": "recipe-relation", "x-fix": "recipe \\"appear\\" introduces one entity, so use relation one-to-one"}, "source": {"type": "null", "x-code": "absence-endpoint", "x-fix": "recipe \\"appear\\" comes from nothing: set \\"source\\": null"}, "target": {"type": "object", "x-code": "absence-endpoint", "x-fix": "recipe \\"appear\\" needs a target entity"}, "correspondence": false}}},\n'
    '        {"if": {"properties": {"recipe": {"enum": ["parts", "hull", "scatter", "gather", "split", "unfold", "retext", "flight", "move", "replace"]}}, "required": ["recipe"]}, "then": {"required": ["correspondence"], "properties": {"source": {"type": "object", "x-code": "endpoint-required", "x-fix": "only recipes vanish/appear take a null endpoint; give this recipe a source entity"}, "target": {"type": "object", "x-code": "endpoint-required", "x-fix": "only recipes vanish/appear take a null endpoint; give this recipe a target entity"}}, "x-code": "correspondence-required", "x-fix": "add a \\"correspondence\\" object (identity, key, order, pairs or recipe)"}},\n'
    '        {"if": {"properties": {"copy": {"const": true}}, "required": ["copy"]}, "then": {"properties": {"source": {"type": "object", "x-code": "copy-endpoints", "x-fix": "copy keeps the source and needs both a source and a target entity"}, "target": {"type": "object", "x-code": "copy-endpoints", "x-fix": "copy keeps the source and needs both a source and a target entity"}}}},\n'
)
swap(anchor, extra + anchor)
path.write_text(text, encoding="utf-8", newline="\n")
