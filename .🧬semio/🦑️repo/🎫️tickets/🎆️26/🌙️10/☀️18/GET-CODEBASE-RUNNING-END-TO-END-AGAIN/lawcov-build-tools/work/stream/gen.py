"""📝 Authoring aid: writes the Bun stream corpus (corpus.json)."""
import hashlib, json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
REQ = hashlib.sha256(b"request").hexdigest()
OTHER = hashlib.sha256(b"another").hexdigest()


def text(value):
    return {"text": value}


def rep(value, times):
    return {"repeat": value, "times": times}


def f(name, content):
    return {"name": name, "content": content}


def raw(content):
    return (content["repeat"] * content["times"]).encode() if "repeat" in content else content["text"].encode()


def frame(**row):
    return json.dumps(row, separators=(",", ":"), ensure_ascii=False)


tiny = [f("source.js", text("export const a=1;\n")), f("metadata.json", text("{}"))]
sha = lambda c: hashlib.sha256(raw(c)).hexdigest()
import base64
tiny_frames = [
    frame(type="file", name="source.js", byteLength=len(raw(tiny[0]["content"])), sha256=sha(tiny[0]["content"])) + "\n",
    frame(type="chunk", data=base64.b64encode(raw(tiny[0]["content"])).decode()) + "\n",
    '{"type":"end"}\n',
    frame(type="file", name="metadata.json", byteLength=2, sha256=sha(tiny[1]["content"])) + "\n",
    frame(type="chunk", data=base64.b64encode(b"{}").decode()) + "\n",
    '{"type":"end"}\n',
    frame(type="complete", requestSha256=REQ) + "\n",
]
LEN = len(raw(tiny[0]["content"]))
c = {"version": 1, "requestSha256": REQ, "otherSha256": OTHER}
c["sets"] = [
    {"name": "tiny", "files": tiny, "chunks": 2, "frames": tiny_frames},
    {"name": "three files with an empty map", "files": [f("source.js", text("a")), f("source.js.map", text("")), f("metadata.json", text("{\"inputs\":[]}"))], "chunks": 2, "frames": None},
    {"name": "exactly one chunk", "files": [f("source.js", rep("ab", 16384)), f("metadata.json", text("{}"))], "chunks": 2, "frames": None},
    {"name": "one byte past a chunk", "files": [f("source.js", rep("ab", 16384) | {} if False else {"repeat": "ab", "times": 16384}), f("source.js.map", rep("z", 1)), f("metadata.json", text("{}"))], "chunks": 3, "frames": None},
    {"name": "three chunks with a short tail and multibyte text", "files": [f("source.js", rep("é🧪", 20000)), f("metadata.json", text("{}"))], "chunks": 5, "frames": None},
]
c["sets"][3]["files"][0]["content"] = {"repeat": "ab", "times": 16385}
c["sets"][3]["chunks"] = 4
c["sets"][4]["chunks"] = 5
c["wire"] = [{"name": "the whole stream at once", "sizes": []}, {"name": "one byte at a time", "sizes": [1]}, {"name": "irregular pieces", "sizes": [7, 64, 1000, 3]}]
c["generator"] = [
    {"name": "one file is not enough", "files": ["source.js"], "error": "Complete bounded Bun output required"},
    {"name": "the first file must be the source", "files": ["metadata.json", "metadata.json"], "error": "Complete bounded Bun output required"},
    {"name": "the last file must be the metadata", "files": ["source.js", "source.js.map"], "error": "Complete bounded Bun output required"},
    {"name": "an unknown name is refused", "files": ["source.js", "other.js", "metadata.json"], "error": "Bun output file capacity refused"},
    {"name": "a repeated name is refused", "files": ["source.js", "source.js", "metadata.json"], "error": "Bun output file capacity refused"},
    {"name": "more files than the maximum are refused", "files": ["source.js", "source.js.map", "source.js.map", "metadata.json"], "error": "Complete bounded Bun output required"},
    {"name": "a malformed request digest is refused", "files": ["source.js", "metadata.json"], "error": "Complete bounded Bun output required", "requestSha256": "not-a-digest"},
]
c["hostile"] = [
    {"name": "chunk-before-file", "set": "tiny", "ops": [{"op": "drop", "at": 0}], "error": "Bun chunk lacks an accepted file"},
    {"name": "duplicate-file", "set": "tiny", "ops": [{"op": "insert", "at": 3, "frame": {"type": "file", "name": "source.js", "byteLength": LEN, "sha256": sha(tiny[0]["content"])}}], "error": "Bun file order or capacity refused"},
    {"name": "bad-hash", "set": "tiny", "ops": [{"op": "replace", "at": 0, "frame": {"type": "file", "name": "source.js", "byteLength": LEN, "sha256": OTHER}}], "error": "Bun file digest or completeness refused"},
    {"name": "missing-complete", "set": "tiny", "ops": [{"op": "drop", "at": 6}], "error": "Complete Bun output stream required", "finish": True},
    {"name": "overlong-partial", "set": "tiny", "ops": [{"op": "append-bytes", "bytes": 70000}], "error": "Bun partial frame capacity exceeded"},
    {"name": "wrong-request", "set": "tiny", "ops": [{"op": "replace", "at": 6, "frame": {"type": "complete", "requestSha256": OTHER}}], "error": "Bun request completion refused"},
    {"name": "after-complete", "set": "tiny", "ops": [{"op": "insert", "at": 7, "frame": {"type": "end"}}], "error": "Bun output continued after completion"},
    {"name": "metadata-too-large", "set": "tiny", "ops": [{"op": "replace", "at": 3, "frame": {"type": "file", "name": "metadata.json", "byteLength": 16777217, "sha256": OTHER}}], "error": "Bun file order or capacity refused"},
    {"name": "metadata first is not a source", "set": "tiny", "ops": [{"op": "replace", "at": 0, "frame": {"type": "file", "name": "metadata.json", "byteLength": LEN, "sha256": sha(tiny[0]["content"])}}], "error": "Bun file order or capacity refused"},
    {"name": "a file after the metadata is refused", "set": "tiny", "ops": [{"op": "insert", "at": 6, "frame": {"type": "file", "name": "source.js.map", "byteLength": 0, "sha256": hashlib.sha256(b"").hexdigest()}}], "error": "Bun file order or capacity refused"},
    {"name": "a frame with an unknown field is not closed", "set": "tiny", "ops": [{"op": "replace", "at": 2, "frame": {"type": "end", "extra": 1}}], "error": "Closed Bun data frame required"},
    {"name": "a frame of an unknown type is not closed", "set": "tiny", "ops": [{"op": "replace", "at": 2, "frame": {"type": "flush"}}], "error": "Closed Bun data frame required"},
    {"name": "a chunk longer than its file is refused", "set": "tiny", "ops": [{"op": "replace", "at": 1, "frame": {"type": "chunk", "data": base64.b64encode(b"x" * 40).decode()}}], "error": "Bun chunk bytes refused"},
    {"name": "a short file ends with the wrong digest", "set": "tiny", "ops": [{"op": "drop", "at": 1}], "error": "Bun file digest or completeness refused"},
    {"name": "a chunk that is not canonical base64 is not closed", "set": "tiny", "ops": [{"op": "replace", "at": 1, "frame": {"type": "chunk", "data": "!!!!"}}], "error": "Closed Bun data frame required"},
    {"name": "a line that is not JSON", "set": "tiny", "ops": [{"op": "replace-text", "at": 2, "text": "not json\n"}], "error": "SyntaxError"},
    {"name": "a sink that is asynchronous is refused", "set": "tiny", "ops": [], "error": "Synchronous Bun data sink required", "sink": "async"},
    {"name": "a stream that failed once refuses everything after", "set": "tiny", "ops": [{"op": "drop", "at": 0}], "error": "Bun chunk lacks an accepted file", "again": "Bun data stream already refused"},
    {"name": "an unfinished stream cannot finish", "set": "tiny", "ops": [{"op": "drop", "at": 5}, {"op": "drop", "at": 5}], "error": "Complete Bun output stream required", "finish": True},
]
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
