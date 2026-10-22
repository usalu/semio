"""📝 Authoring aid: writes the Cargo TOML boundary corpus (corpus.json)."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"


def ok(name, text, expected, bun="same"):
    return {"name": name, "text": text, "expected": expected, "bun": bun}


def refused(name, text):
    return {"name": name, "text": text, "expected": None, "bun": "same"}


rows = [
    ok("a package with inline dependency tables", '[package]\nname = "a"\nversion = "0.1.0"\nedition = "2021"\n\n[dependencies]\nserde = { version = "1", features = ["derive"] }\nb = { path = "../b" }\n',
       {"package": {"name": "a", "version": "0.1.0", "edition": "2021"}, "dependencies": {"serde": {"version": "1", "features": ["derive"]}, "b": {"path": "../b"}}}),
    ok("arrays of tables", '[[bin]]\nname = "x"\npath = "src/x.rs"\n[[bin]]\nname = "y"\npath = "src/y.rs"\n', {"bin": [{"name": "x", "path": "src/x.rs"}, {"name": "y", "path": "src/y.rs"}]}),
    ok("target tables with a cfg expression and a triple", "[target.'cfg(unix)'.dependencies]\nlibc = \"0.2\"\n[target.\"x86_64-pc-windows-msvc\".dependencies]\nwinapi = { path = \"../w\" }\n",
       {"target": {"cfg(unix)": {"dependencies": {"libc": "0.2"}}, "x86_64-pc-windows-msvc": {"dependencies": {"winapi": {"path": "../w"}}}}}),
    ok("a cfg expression with parentheses and commas", "[target.'cfg(any(unix, windows))'.dependencies]\nlibc = \"0.2\"\n", {"target": {"cfg(any(unix, windows))": {"dependencies": {"libc": "0.2"}}}}),
    ok("dotted keys at the top and in tables", 'a.b.c = 1\n[x]\ny.z = "q"\n', {"a": {"b": {"c": 1}}, "x": {"y": {"z": "q"}}}),
    ok("dotted keys with spaces", "a . b = 1\n", {"a": {"b": 1}}),
    ok("dotted keys inside a table", "[a]\nb.c = 1\nb.d = 2\n", {"a": {"b": {"c": 1, "d": 2}}}),
    ok("integers in every base", "i = 1_000\nh = 0xff\no = 0o17\nb = 0b101\nneg = -5\nplus = +3\nz = 0\n", {"i": 1000, "h": 255, "o": 15, "b": 5, "neg": -5, "plus": 3, "z": 0}),
    ok("floats", "a = 3.14\nb = 1e2\nc = -1.5\nd = 6.626e-34\ne = 5e+22\nf = 1_0.5\n", {"a": 3.14, "b": 100, "c": -1.5, "d": 6.626e-34, "e": 5e+22, "f": 10.5}),
    ok("a trailing comma in an array", "a = [1, 2, 3,]\n", {"a": [1, 2, 3]}),
    ok("a multi line array with comments and a trailing comma", 'features = [\n  "a",\n  "b", # why\n  "c",\n]\n', {"features": ["a", "b", "c"]}),
    ok("a multi line array without a trailing comma", 'features = [\n  "a",\n  "b"\n]\n', {"features": ["a", "b"]}),
    ok("empty arrays and tables", "a = []\nb = {}\n", {"a": [], "b": {}}),
    ok("arrays of inline tables", "c = [ {x=1}, {x=2} ]\n", {"c": [{"x": 1}, {"x": 2}]}),
    ok("nested inline tables", "t = { a = 1, b = { c = 2 } }\n", {"t": {"a": 1, "b": {"c": 2}}}),
    ok("quoted keys", "\"a.b\" = 1\n'c d' = 2\n[ \"e\" . f ]\ng = 3\n", {"a.b": 1, "c d": 2, "e": {"f": {"g": 3}}}),
    ok("an empty quoted key", '"" = 1\n', {"": 1}),
    ok("bare keys with dashes and underscores", "a-b_c1 = 1\n", {"a-b_c1": 1}),
    ok("spaces in a table header and CRLF line ends", '[ package ]\r\nname = "w"\r\n', {"package": {"name": "w"}}),
    ok("comments and a hash inside a string", '# c\na = 1 # trailing\nb = "#notcomment"\n', {"a": 1, "b": "#notcomment"}),
    ok("booleans", "t = true\nf = false\n", {"t": True, "f": False}),
    ok("a workspace with inherited dependencies", '[workspace]\nmembers = ["a", "b/*"]\nexclude = ["c"]\n[workspace.dependencies]\nx = { path = "crates/x", package = "real" }\ny = "1"\n',
       {"workspace": {"members": ["a", "b/*"], "exclude": ["c"], "dependencies": {"x": {"path": "crates/x", "package": "real"}, "y": "1"}}}),
    ok("a super table defined after its sub table", "[a.b]\nx=1\n[a]\ny=2\n", {"a": {"b": {"x": 1}, "y": 2}}),
    ok("nested arrays of tables", "[[a.b]]\nx=1\n[[a.b]]\nx=2\n", {"a": {"b": [{"x": 1}, {"x": 2}]}}),
    ok("an array of tables with a sub table", "[[a]]\nx=1\n[a.sub]\ny=2\n[[a]]\nx=3\n", {"a": [{"x": 1, "sub": {"y": 2}}, {"x": 3}]}),
    ok("equals signs and brackets inside strings", "a = \"x=y\"\nb = '[z]'\n", {"a": "x=y", "b": "[z]"}),
    ok("raw unicode and a four digit escape", 'k = "é"\ns = "\\u00e9"\nj = "日本"\n', {"k": "é", "s": "é", "j": "日本"}),
    ok("common escapes", 'a = "q\\"uote"\nb = "l\\nm"\nc = "r\\rs"\nd = "b\\\\s"\n', {"a": 'q"uote', "b": "l\nm", "c": "r\rs", "d": "b\\s"}),
    ok("literal strings keep backslashes", "a = 'C:\\path\\n'\n", {"a": "C:\\path\\n"}),
    ok("a literal multi line string", "l = '''raw\\n'''\n", {"l": "raw\\n"}),
    ok("an empty document", "", {}),
    ok("a tab escape", 'a = "x\\ty"\n', {"a": "x\ty"}, "differs"),
    ok("a form feed escape", 'a = "x\\fy"\n', {"a": "x\fy"}, "differs"),
    ok("an eight digit unicode escape", 'a = "\\U0001F600"\n', {"a": "😀"}, "differs"),
    ok("a multi line basic string drops its first newline", 's = """\nline1\nline2"""\n', {"s": "line1\nline2"}, "differs"),
    ok("a line ending backslash joins lines", 'a = """x \\\n   y"""\n', {"a": "x y"}, "differs"),
    ok("nested arrays", 'b = [[1,2],["a"]]\n', {"b": [[1, 2], ["a"]]}, "refuses"),
    refused("a duplicate key", "a = 1\na = 2\n"),
    {**refused("a table defined twice", "[a]\nx = 1\n[a]\ny = 2\n"), "bun": "differs"},
    refused("an unterminated string", 'a = "x\n'),
    refused("a key without a value", "a =\n"),
    {**refused("a bare word value", "a = nope\n"), "bun": "differs"},
    refused("a missing equals sign", 'a "x"\n'),
    refused("an unclosed table header", "[a\nx = 1\n"),
    refused("an unclosed array", "a = [1, 2\n"),
    refused("a dotted key redefining a value", "a = 1\na.b = 2\n"),
    {**refused("a leading zero integer", "a = 01\n"), "bun": "differs"},
]
refusals = [
    {"name": "a refused file reports the file, its hash and the parser", "text": "a = 1\nb = !\n", "line": 1}
]
json.dump({"version": 1, "documents": rows, "refusals": refusals}, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(rows))
