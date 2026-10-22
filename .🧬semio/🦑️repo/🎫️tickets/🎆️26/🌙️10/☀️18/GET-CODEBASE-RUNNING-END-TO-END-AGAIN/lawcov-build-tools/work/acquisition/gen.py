"""📝 Authoring aid: writes the acquisition corpus (corpus.json)."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
ARCHIVE = [
    {"path": "package/package.json", "text": "{\"name\":\"nx\",\"version\":\"23.2.0\"}\n", "mode": 420},
    {"path": "package/lib/index.js", "text": "module.exports = 1;\n", "mode": 420},
    {"path": "package/bin/nx.js", "text": "#!/usr/bin/env node\n", "mode": 493},
    {"path": "package/empty", "text": "", "mode": 420},
]
OTHER = [{"path": "other/package.json", "text": "{}\n", "mode": 420}]
EXACT = "Exact pinned Nx registry package required"
OK_LOCK = {"packages": {"nx": ["nx@23.2.0", "", {}, "@integrity"]}}


def lock(row):
    return {"packages": {"nx": row}}


def sc(name, lock_=None, root="workspace", offline=False, elapsed=60000, status=200, length="@length", body=None, chunk=0, abort=None, archive="main", expect=None, tamper=False):
    return {"name": name, "lock": OK_LOCK if lock_ is None else lock_, "authority": {"root": root, "offline": offline, "elapsed": elapsed}, "response": {"status": status, "contentLength": length, "body": body if body is not None else {"archive": True}, "chunk": chunk},
            "abortAfterReads": abort, "archive": archive, "tamper": tamper, "expect": expect or {"ok": True}}


def err(text):
    return {"error": text}


c = {"version": 1, "archives": {"main": ARCHIVE, "other": OTHER}, "maximumCompressedBytes": 33554432, "registry": "https://registry.npmjs.org/nx/-/nx-"}
bad_integrity = "sha512-" + "A" * 86 + "=="
c["scenarios"] = [
    sc("a correct tarball is authenticated and decoded in one chunk", expect={"ok": True, "chunks": "whole"}),
    sc("the same tarball in chunks of seven bytes", chunk=7),
    sc("the same tarball byte by byte", chunk=1),
    sc("a response without a length header is read to its end", length=None),
    sc("a response that advertises less than it sends is refused", length="5", expect=err("Pinned Nx package download length changed")),
    sc("a response that advertises more than it sends is refused", length="99999", expect=err("Pinned Nx package download length changed")),
    sc("a length header above the compressed ceiling is refused before reading", length="33554433", expect=err("Nx package compressed ceiling exceeded")),
    sc("a length header that is not a number is refused", length="abc", expect=err("Nx package compressed ceiling exceeded")),
    sc("a length header with a sign is refused", length="-5", expect=err("Nx package compressed ceiling exceeded")),
    sc("a stream above the ceiling is cut off while reading", length=None, body={"zeros": 33554433}, expect=err("Nx package compressed ceiling exceeded")),
    sc("a stream of exactly the ceiling is read and fails its integrity", length=None, body={"zeros": 33554432}, expect=err("Locked Nx package integrity changed")),
    sc("a tampered tarball fails the locked integrity", tamper=True, expect=err("Locked Nx package integrity changed")),
    sc("a lock with another integrity refuses the real tarball", lock_=lock(["nx@23.2.0", "", {}, bad_integrity]), expect=err("Locked Nx package integrity changed")),
    sc("a tarball with a foreign package prefix is refused by the decoder", archive="other", expect=err("*")),
    sc("an unavailable registry is reported with its status", status=404, expect=err("Pinned Nx package download refused: 404")),
    sc("a server error is reported with its status", status=503, expect=err("Pinned Nx package download refused: 503")),
    sc("an answer without a body is refused", body={"none": True}, expect=err("Pinned Nx package download refused: 200")),
    sc("a lock without packages is refused", lock_={}, expect=err(EXACT)),
    sc("a lock that is not an object is refused", lock_="lock", expect=err(EXACT)),
    sc("a lock without the nx row is refused", lock_={"packages": {}}, expect=err(EXACT)),
    sc("a row of three columns is refused", lock_=lock(["nx@23.2.0", "", {}]), expect=err(EXACT)),
    sc("a row with a range is refused", lock_=lock(["nx@^23.2.0", "", {}, "@integrity"]), expect=err(EXACT)),
    sc("a row for another package is refused", lock_=lock(["other@23.2.0", "", {}, "@integrity"]), expect=err(EXACT)),
    sc("a row with a registry in its second column is refused", lock_=lock(["nx@23.2.0", "https://example.org", {}, "@integrity"]), expect=err(EXACT)),
    sc("an integrity of another digest is refused", lock_=lock(["nx@23.2.0", "", {}, "sha256-" + "A" * 43 + "="]), expect=err(EXACT)),
    sc("an integrity that is too short is refused", lock_=lock(["nx@23.2.0", "", {}, "sha512-AAAA=="]), expect=err(EXACT)),
    sc("a pre-release version is refused", lock_=lock(["nx@23.2.0-beta.1", "", {}, "@integrity"]), expect=err(EXACT)),
    sc("a checkout other than the authority's is refused", root="other", expect=err("Original package Source workspace required")),
    sc("an offline grant is refused", offline=True, expect=err("Original network grant required for pinned Nx package")),
    sc("a cancelled control stops the read and releases the stream", abort=2, length=None, chunk=5, expect=err("@abort")),
    sc("a control without a deadline reads to the end", elapsed=None),
]
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(c["scenarios"]))
