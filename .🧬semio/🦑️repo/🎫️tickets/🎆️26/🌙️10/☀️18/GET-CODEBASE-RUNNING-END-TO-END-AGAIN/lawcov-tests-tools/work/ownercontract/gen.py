"""📝 Authoring aid: writes the owner-contract conformance corpus (corpus.json)."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
T, F = True, False
ANY = {"anyOf": [{"type": "null"}, {"type": "boolean"}, {"type": "integer"}, {"type": "string"}, {"type": "array", "items": {"type": "integer"}}, {"type": "object"}]}


def v(value, accepted, differs=False):
    return {"value": value, "accepted": accepted, **({"ajv": "differs"} if differs else {})}


def row(name, contract, *values):
    return {"name": name, "contract": contract, "values": list(values)}


rows = [
    row("a string constant", {"const": "a"}, v("a", T), v("b", F), v(1, F), v(None, F), v(["a"], F)),
    row("a numeric constant", {"const": 1}, v(1, T), v("1", F), v(True, F), v(2, F)),
    row("a null constant", {"const": None}, v(None, T), v(0, F), v("", F), v(False, F)),
    row("an object constant matches by content", {"const": {"a": 1, "b": [1, 2]}}, v({"a": 1, "b": [1, 2]}, T), v({"b": [1, 2], "a": 1}, T), v({"a": 1}, F), v({"a": 1, "b": [2, 1]}, F)),
    row("the null type", {"type": "null"}, v(None, T), v(0, F), v("null", F), v({}, F)),
    row("a string enum", {"enum": ["a", "b"]}, v("a", T), v("c", F), v(None, F), v(["a"], F)),
    row("an enum with a null and a number", {"enum": [1, None]}, v(None, T), v(1, T), v("1", F), v(True, F)),
    row("an enum of objects matches by content", {"enum": [{"a": 1}, [1, 2]]}, v({"a": 1}, T), v([1, 2], T), v([2, 1], F), v({"a": 2}, F)),
    row("an enum decides before the type", {"type": "string", "enum": ["a"]}, v("a", T), v(1, F), v("b", F)),
    row("a constant decides before an enum", {"const": "a", "enum": ["a", "b"]}, v("a", T), v("b", F)),
    row("a constant and a type must both hold", {"type": "string", "const": "5"}, v("5", T), v(5, F), v("6", F)),
    row("the boolean type", {"type": "boolean"}, v(True, T), v(False, T), v(0, F), v("true", F), v(None, F)),
    row("the integer type", {"type": "integer"}, v(0, T), v(-7, T), v(1.5, F), v("1", F), v(True, F), v(None, F), v(9007199254740991, T), v(9007199254740992, F, True)),
    row("integer bounds are inclusive", {"type": "integer", "minimum": 0, "maximum": 5}, v(0, T), v(5, T), v(6, F), v(-1, F), v(3, T)),
    row("integer bounds are optional", {"type": "integer", "minimum": 2}, v(2, T), v(1, F), v(10 ** 12, T)),
    row("string length bounds", {"type": "string", "minLength": 1, "maxLength": 3}, v("", F), v("a", T), v("abc", T), v("abcd", F), v(1, F)),
    row("string length counts code points", {"type": "string", "minLength": 2, "maxLength": 3}, v("😀", F), v("😀😀", T), v("😀😀😀", T), v("😀😀😀😀", F), v("é", F), v("éé", T)),
    row("a pattern is searched, not anchored", {"type": "string", "pattern": "a"}, v("bab", T), v("b", F), v("", F), v(1, F)),
    row("an anchored target name pattern", {"type": "string", "pattern": "^[a-z][a-z0-9-]*$"}, v("build-x", T), v("a", T), v("Build", F), v("1a", F), v("", F), v("a b", F), v("a\n", F)),
    row("a pattern runs with the unicode flag", {"type": "string", "pattern": "^.$"}, v("😀", T), v("a", T), v("ab", F)),
    row("a project pattern without colon, space or NUL", {"type": "string", "pattern": "^[^:\\s\\u0000]+$"}, v("a/b", T), v("a:b", F), v("a b", F), v("a\u0000b", F), v("", F)),
    row("an array of strings", {"type": "array", "items": {"type": "string"}}, v([], T), v(["a", "b"], T), v([1], F), v(["a", 1], F), v("a", F), v({}, F), v(None, F)),
    row("array length bounds", {"type": "array", "minItems": 1, "maxItems": 2, "items": {"type": "integer"}}, v([], F), v([1], T), v([1, 2], T), v([1, 2, 3], F)),
    row("uniqueness compares by value", {"type": "array", "uniqueItems": True, "items": ANY}, v(["a", "a"], F), v([1, "1"], T), v([[1, 2], [2, 1]], T), v([{"a": 1}, {"a": 1}], F), v([None, None], F), v([], T), v([1, 2, 3], T)),
    row("uniqueness ignores the key order of objects", {"type": "array", "uniqueItems": True, "items": {"type": "object"}}, v([{"a": 1, "b": 2}, {"b": 2, "a": 1}], F), v([{"a": 1, "b": 2}, {"a": 1, "b": 3}], T), v([{"a": [1, {"x": 1, "y": 2}]}, {"a": [1, {"y": 2, "x": 1}]}], F)),
    row("an array without an items rule accepts any items", {"type": "array", "minItems": 1}, v([], F), v([1], T), v(["a", None, {"b": 1}], T)),
    row("an array of arrays", {"type": "array", "items": {"type": "array", "items": {"type": "boolean"}}}, v([[True], []], T), v([[1]], F), v([True], F)),
    row("a closed object", {"type": "object", "required": ["a"], "additionalProperties": False, "properties": {"a": {"type": "integer"}}}, v({"a": 1}, T), v({}, F), v({"a": 1, "b": 2}, F), v({"a": "x"}, F), v([], F), v(None, F), v("a", F)),
    row("an open object", {"type": "object", "properties": {"a": {"type": "integer"}}}, v({"a": 1, "b": 2}, T), v({}, T), v({"b": 2}, T), v({"a": 1.5}, F)),
    row("optional properties are checked when present", {"type": "object", "required": ["a"], "properties": {"a": {"type": "string"}, "b": {"type": "integer"}}}, v({"a": "x"}, T), v({"a": "x", "b": 1}, T), v({"a": "x", "b": "y"}, F), v({"b": 1}, F)),
    row("a null property value counts as present", {"type": "object", "required": ["a"], "properties": {"a": {"type": "null"}}}, v({"a": None}, T), v({}, F), v({"a": 0}, F)),
    row("required names are own properties only", {"type": "object", "required": ["toString", "constructor"]}, v({}, F), v({"toString": 1, "constructor": 2}, T), v({"toString": 1}, F)),
    row("a property named like a prototype member", {"type": "object", "required": ["a"], "additionalProperties": False, "properties": {"a": {"type": "integer"}}}, v({"a": 1, "constructor": 1}, F), v({"a": 1, "__proto__": 1}, F), v({"a": 1, "toString": 1}, F)),
    row("nested objects and arrays", {"type": "object", "required": ["kids"], "properties": {"kids": {"type": "array", "items": {"type": "object", "required": ["name"], "additionalProperties": False, "properties": {"name": {"type": "string", "minLength": 1}}}}}}, v({"kids": [{"name": "a"}]}, T), v({"kids": [{"name": ""}]}, F), v({"kids": [{"name": "a", "x": 1}]}, F), v({"kids": [{}]}, F), v({"kids": []}, T)),
    row("any of several rules", {"anyOf": [{"type": "null"}, {"type": "integer"}]}, v(None, T), v(1, T), v("x", F), v(1.5, F)),
    row("any of decides before every sibling rule", {"anyOf": [{"const": "a"}], "type": "integer"}, v("a", T, True), v(1, F)),
    row("any of with enumerations", {"anyOf": [{"enum": ["a"]}, {"enum": ["b"]}]}, v("a", T), v("b", T), v("c", F)),
    row("the null type decides before a constant", {"type": "null", "const": "a"}, v(None, T, True), v("a", F)),
    row("a constant without a type is a literal", {"const": "kind"}, v("kind", T), v("other", F)),
    row("a rule without type, constant or enumeration admits nothing", {}, v(None, F, True), v(1, F, True), v({}, F, True), v("x", F, True)),
    row("a description alone admits nothing", {"description": "x"}, v("x", F, True)),
    row("an unsupported number type admits nothing", {"type": "number"}, v(1, F, True), v(1.5, F, True), v("1", F)),
    row("an unsupported property rule closes the object", {"type": "object", "properties": {"a": {"type": "number"}}}, v({}, T), v({"a": 1}, F, True)),
]
json.dump({"version": 1, "contracts": rows}, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(rows), sum(len(r["values"]) for r in rows))
