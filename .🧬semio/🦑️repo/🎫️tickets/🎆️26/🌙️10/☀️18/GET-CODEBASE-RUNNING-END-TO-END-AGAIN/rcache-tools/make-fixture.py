#!/usr/bin/env python3
# generates the language-agnostic fixtures of the remote cache tests (digests computed with Python hashlib, the independent oracle)
import hashlib, json, os, sys
TOOLS = os.path.dirname(os.path.abspath(__file__))
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
FIX = os.path.join(TOOLS, "staged", B, "🌐️remote-cache", "🧪️tests", "🧫️fixtures")
os.makedirs(FIX, exist_ok=True)
H = lambda text: hashlib.sha256(text.encode()).hexdigest()
E = "SEMIO_REMOTE_CACHE_"

contract = {
    "version": 1,
    "accepted": [
        {"name": "off without an endpoint", "environment": {}, "contract": None},
        {"name": "empty endpoint is off", "environment": {E + "ENDPOINT": ""}, "contract": None},
        {"name": "other variables alone do not enable it", "environment": {E + "INSTANCE": "team-a", E + "ACCESS": "read-write"}, "contract": None},
        {"name": "endpoint alone takes every default", "environment": {E + "ENDPOINT": "http://127.0.0.1:6090"},
         "contract": {"version": 1, "endpoint": "http://127.0.0.1:6090", "instance": "main", "access": "read-only", "upload": True, "compression": True, "download": "toplevel", "timeoutSeconds": 30, "retries": 3, "auth": {"tokenEnvironment": "SEMIO_REMOTE_CACHE_TOKEN"}}},
        {"name": "every field overridden", "environment": {E + "ENDPOINT": "https://cache.example.test:8443", E + "INSTANCE": "team-a", E + "ACCESS": "read-write", E + "UPLOAD": "false", E + "COMPRESSION": "false", E + "DOWNLOAD": "all", E + "TIMEOUT_SECONDS": "60", E + "RETRIES": "0", E + "TOKEN_ENVIRONMENT": "CI_CACHE_TOKEN"},
         "contract": {"version": 1, "endpoint": "https://cache.example.test:8443", "instance": "team-a", "access": "read-write", "upload": False, "compression": False, "download": "all", "timeoutSeconds": 60, "retries": 0, "auth": {"tokenEnvironment": "CI_CACHE_TOKEN"}}},
        {"name": "empty override values fall back to the defaults", "environment": {E + "ENDPOINT": "http://[::1]:6090", E + "ACCESS": "", E + "UPLOAD": "", E + "TIMEOUT_SECONDS": ""},
         "contract": {"version": 1, "endpoint": "http://[::1]:6090", "instance": "main", "access": "read-only", "upload": True, "compression": True, "download": "toplevel", "timeoutSeconds": 30, "retries": 3, "auth": {"tokenEnvironment": "SEMIO_REMOTE_CACHE_TOKEN"}}},
    ],
    "refused": [
        {"name": "endpoint with a path", "environment": {E + "ENDPOINT": "http://cache.example.test/team"}},
        {"name": "endpoint with credentials", "environment": {E + "ENDPOINT": "http://user:secret@cache.example.test"}},
        {"name": "endpoint with a query", "environment": {E + "ENDPOINT": "http://cache.example.test?x=1"}},
        {"name": "endpoint with a foreign scheme", "environment": {E + "ENDPOINT": "grpc://cache.example.test"}},
        {"name": "endpoint without a scheme", "environment": {E + "ENDPOINT": "cache.example.test:443"}},
        {"name": "endpoint with whitespace", "environment": {E + "ENDPOINT": "http://cache.example.test --remote_executor=evil"}},
        {"name": "instance is a protocol route", "environment": {E + "ENDPOINT": "http://h", E + "INSTANCE": "cas"}},
        {"name": "instance has upper case", "environment": {E + "ENDPOINT": "http://h", E + "INSTANCE": "Team"}},
        {"name": "instance has a slash", "environment": {E + "ENDPOINT": "http://h", E + "INSTANCE": "a/b"}},
        {"name": "access is unknown", "environment": {E + "ENDPOINT": "http://h", E + "ACCESS": "write"}},
        {"name": "upload is not boolean", "environment": {E + "ENDPOINT": "http://h", E + "UPLOAD": "yes"}},
        {"name": "compression is not boolean", "environment": {E + "ENDPOINT": "http://h", E + "COMPRESSION": "1"}},
        {"name": "download is unknown", "environment": {E + "ENDPOINT": "http://h", E + "DOWNLOAD": "none"}},
        {"name": "timeout is zero", "environment": {E + "ENDPOINT": "http://h", E + "TIMEOUT_SECONDS": "0"}},
        {"name": "timeout is too long", "environment": {E + "ENDPOINT": "http://h", E + "TIMEOUT_SECONDS": "601"}},
        {"name": "timeout is not a number", "environment": {E + "ENDPOINT": "http://h", E + "TIMEOUT_SECONDS": "30s"}},
        {"name": "retries are negative", "environment": {E + "ENDPOINT": "http://h", E + "RETRIES": "-1"}},
        {"name": "retries are too many", "environment": {E + "ENDPOINT": "http://h", E + "RETRIES": "11"}},
        {"name": "token variable is lower case", "environment": {E + "ENDPOINT": "http://h", E + "TOKEN_ENVIRONMENT": "my_token"}},
    ],
    "message": "Closed remote cache contract required",
}

patterns = {
    "Endpoint": {"accepted": ["http://127.0.0.1:6090", "https://cache.example.test", "https://cache.example.test:8443", "http://[::1]:6090", "http://localhost"],
                 "refused": ["", "http://", "http://h/", "http://h/x", "http://u@h", "ftp://h", "http://h:", "http://h:123456", "http:/h", "HTTP://h x"]},
    "Instance": {"accepted": ["main", "team-a", "a", "ci.1_x", "0"], "refused": ["", "Main", "-a", "_a", "a/b", "ac", "cas", "v1", "healthz", "status", "x" * 65]},
    "Token": {"accepted": ["a", "tok-123", "A.b_c~d+e/f=g", "x" * 4096], "refused": ["", "bad token", "tok\"en", "tok'en", "tok$en", "x" * 4097, "tök"]},
    "EnvironmentName": {"accepted": ["A", "SEMIO_REMOTE_CACHE_TOKEN", "CI_CACHE_TOKEN2"], "refused": ["", "a", "1A", "A-B", "A B", "A" * 65]},
}

def helper_sh(name):
    return "#!/bin/sh\ncat >/dev/null\nprintf '{\"headers\":{\"Authorization\":[\"Bearer %s\"]}}\\n' \"$" + name + "\"\n"

def helper_cmd(name):
    return "@echo off\r\nmore >nul\r\necho {\"headers\":{\"Authorization\":[\"Bearer %" + name + "%\"]}}\r\n"

def flags(base, instance, upload, compression, download, timeout, retries):
    return ["--remote_cache=" + base + "/" + instance, "--remote_instance_name=" + instance, "--" + ("" if upload else "no") + "remote_upload_local_results", "--" + ("" if compression else "no") + "remote_cache_compression", "--remote_download_outputs=" + download, "--remote_timeout=" + str(timeout) + "s", "--remote_retries=" + str(retries)]

rw = {E + "ENDPOINT": "https://cache.example.test:8443", E + "INSTANCE": "team-a", E + "ACCESS": "read-write", E + "TOKEN_ENVIRONMENT": "CI_CACHE_TOKEN"}
bazel = [
    {"name": "default is read-only with every switch spelled", "environment": {E + "ENDPOINT": "http://127.0.0.1:6090"}, "directory": "/h", "platform": "linux", "restoresOutputs": True,
     "flags": flags("http://127.0.0.1:6090", "main", False, True, "toplevel", 30, 3), "tokenEnvironment": {}, "helper": None, "uploadEnforcedBy": "client"},
    {"name": "read-write with a token pushes and uses the credential helper scoped to the cache host", "environment": {**rw, "CI_CACHE_TOKEN": "tok-123"}, "directory": "/h", "platform": "linux", "restoresOutputs": True,
     "flags": flags("https://cache.example.test:8443", "team-a", True, True, "toplevel", 30, 3) + ["--credential_helper=cache.example.test=/h/remote-cache-credentials.sh"], "tokenEnvironment": {"CI_CACHE_TOKEN": "tok-123"},
     "helper": {"path": "/h/remote-cache-credentials.sh", "mode": 493, "content": helper_sh("CI_CACHE_TOKEN")}, "uploadEnforcedBy": "none"},
    {"name": "the Windows helper is a cmd file", "environment": {**rw, "CI_CACHE_TOKEN": "tok-123"}, "directory": "C:\\h", "platform": "win32", "restoresOutputs": True,
     "flags": flags("https://cache.example.test:8443", "team-a", True, True, "toplevel", 30, 3) + ["--credential_helper=cache.example.test=C:\\h\\remote-cache-credentials.cmd"], "tokenEnvironment": {"CI_CACHE_TOKEN": "tok-123"},
     "helper": {"path": "C:\\h\\remote-cache-credentials.cmd", "mode": 420, "content": helper_cmd("CI_CACHE_TOKEN")}, "uploadEnforcedBy": "none"},
    {"name": "read-write credentials with upload off stay read-only on the client", "environment": {**rw, E + "UPLOAD": "false"}, "directory": "/h", "platform": "linux", "restoresOutputs": True,
     "flags": flags("https://cache.example.test:8443", "team-a", False, True, "toplevel", 30, 3), "tokenEnvironment": {}, "helper": None, "uploadEnforcedBy": "client"},
    {"name": "compression off, all outputs, long timeout", "environment": {E + "ENDPOINT": "http://127.0.0.1:6090", E + "COMPRESSION": "false", E + "DOWNLOAD": "all", E + "TIMEOUT_SECONDS": "120", E + "RETRIES": "5"}, "directory": "/h", "platform": "linux", "restoresOutputs": True,
     "flags": flags("http://127.0.0.1:6090", "main", False, False, "all", 120, 5), "tokenEnvironment": {}, "helper": None, "uploadEnforcedBy": "client"},
    {"name": "IPv6 endpoint scopes the helper to the bare address", "environment": {E + "ENDPOINT": "http://[::1]:6090", E + "TOKEN_ENVIRONMENT": "CI_CACHE_TOKEN", "CI_CACHE_TOKEN": "tok-123"}, "directory": "/h", "platform": "linux", "restoresOutputs": True,
     "flags": flags("http://[::1]:6090", "main", False, True, "toplevel", 30, 3) + ["--credential_helper=::1=/h/remote-cache-credentials.sh"], "tokenEnvironment": {"CI_CACHE_TOKEN": "tok-123"},
     "helper": {"path": "/h/remote-cache-credentials.sh", "mode": 493, "content": helper_sh("CI_CACHE_TOKEN")}, "uploadEnforcedBy": "client"},
    {"name": "minimal is refused when the adapter restores snapshots", "environment": {E + "ENDPOINT": "http://127.0.0.1:6090", E + "DOWNLOAD": "minimal"}, "directory": "/h", "platform": "linux", "restoresOutputs": True, "refused": "needs remote cache download toplevel or all"},
    {"name": "minimal is rendered when nothing is restored", "environment": {E + "ENDPOINT": "http://127.0.0.1:6090", E + "DOWNLOAD": "minimal"}, "directory": "/h", "platform": "linux", "restoresOutputs": False,
     "flags": flags("http://127.0.0.1:6090", "main", False, True, "minimal", 30, 3), "tokenEnvironment": {}, "helper": None, "uploadEnforcedBy": "client"},
    {"name": "a token with shell metacharacters is refused", "environment": {E + "ENDPOINT": "http://127.0.0.1:6090", "SEMIO_REMOTE_CACHE_TOKEN": "a b"}, "directory": "/h", "platform": "linux", "restoresOutputs": True, "refused": "Closed remote cache token required"},
]

nxsrv = "NX_SELF_HOSTED_REMOTE_CACHE_SERVER"
nxtok = "NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN"
unsupported = ["compression", "download", "timeoutSeconds", "retries"]
nx = [
    {"name": "off clears both variables so nothing stale can enable a cache", "environment": {nxsrv: "http://stale", nxtok: "stale"},
     "variables": {nxsrv: None, nxtok: None}, "applied": [], "unsupported": [], "uploadEnforcedBy": "none"},
    {"name": "default is read-only: the server is the origin plus the namespace and uploads are left to the server", "environment": {E + "ENDPOINT": "http://127.0.0.1:6090"},
     "variables": {nxsrv: "http://127.0.0.1:6090/main", nxtok: None}, "applied": [nxsrv + "=http://127.0.0.1:6090/main"], "unsupported": unsupported, "uploadEnforcedBy": "server"},
    {"name": "read-write with a token", "environment": {**rw, "CI_CACHE_TOKEN": "tok-123"},
     "variables": {nxsrv: "https://cache.example.test:8443/team-a", nxtok: "tok-123"}, "applied": [nxsrv + "=https://cache.example.test:8443/team-a", nxtok + "=<redacted>"], "unsupported": unsupported, "uploadEnforcedBy": "none"},
    {"name": "read-write with upload off is enforced by the server", "environment": {**rw, E + "UPLOAD": "false", "CI_CACHE_TOKEN": "tok-123"},
     "variables": {nxsrv: "https://cache.example.test:8443/team-a", nxtok: "tok-123"}, "applied": [nxsrv + "=https://cache.example.test:8443/team-a", nxtok + "=<redacted>"], "unsupported": unsupported, "uploadEnforcedBy": "server"},
]

def dump(name, value):
    with open(os.path.join(FIX, name), "w", encoding="utf8") as handle:
        json.dump(value, handle, ensure_ascii=False, indent=2)
        handle.write("\n")

dump("🔣️.json", {"version": 1, "contract": contract, "patterns": patterns, "bazel": bazel, "nx": nx})

# wire protocol: language-agnostic request/response sequences
hello, world, empty = "hello", "world", ""
dh, dw, de = H(hello), H(world), H(empty)
nxk = "10625049053885554827"
auth = lambda token: {"authorization": "Bearer " + token}
wire = {
    "version": 1,
    "digests": {"hello": dh, "world": dw, "empty": de},
    "scenarios": [
        {"name": "open server: Bazel content-addressed blobs verify their digest", "server": {"tokens": {}}, "steps": [
            {"name": "unknown blob is a miss", "method": "GET", "path": "/cas/" + dh, "status": 404},
            {"name": "blob with a matching digest is stored", "method": "PUT", "path": "/cas/" + dh, "body": hello, "status": 200},
            {"name": "blob reads back byte for byte", "method": "GET", "path": "/cas/" + dh, "status": 200, "body": hello, "headers": {"content-type": "application/octet-stream", "content-length": "5"}},
            {"name": "HEAD answers the length without a body", "method": "HEAD", "path": "/cas/" + dh, "status": 200, "body": "", "headers": {"content-length": "5"}},
            {"name": "blob whose bytes do not match the digest is refused", "method": "PUT", "path": "/cas/" + dw, "body": hello, "status": 400},
            {"name": "the refused blob was not stored", "method": "GET", "path": "/cas/" + dw, "status": 404},
            {"name": "storing a blob again is idempotent", "method": "PUT", "path": "/cas/" + dh, "body": hello, "status": 200},
            {"name": "the empty blob has the empty digest", "method": "PUT", "path": "/cas/" + de, "body": "", "status": 200},
            {"name": "the empty blob reads back empty", "method": "GET", "path": "/cas/" + de, "status": 200, "body": "", "headers": {"content-length": "0"}},
            {"name": "digest must be 64 lower-case hex digits", "method": "GET", "path": "/cas/" + dh.upper(), "status": 404},
            {"name": "digest must not be short", "method": "GET", "path": "/cas/" + dh[:63], "status": 404},
        ]},
        {"name": "open server: Bazel action results are overwritten by the newest value", "server": {"tokens": {}}, "steps": [
            {"name": "unknown action is a miss", "method": "GET", "path": "/ac/" + dh, "status": 404},
            {"name": "action result is stored", "method": "PUT", "path": "/ac/" + dh, "body": "result-1", "status": 200},
            {"name": "action result reads back", "method": "GET", "path": "/ac/" + dh, "status": 200, "body": "result-1"},
            {"name": "action result is replaced", "method": "PUT", "path": "/ac/" + dh, "body": "result-2", "status": 200},
            {"name": "the newest value wins", "method": "GET", "path": "/ac/" + dh, "status": 200, "body": "result-2"},
            {"name": "blobs and actions are separate keyspaces", "method": "GET", "path": "/cas/" + dh, "status": 404},
        ]},
        {"name": "open server: Nx task archives are write-once", "server": {"tokens": {}}, "steps": [
            {"name": "unknown hash is a miss", "method": "GET", "path": "/v1/cache/" + nxk, "status": 404},
            {"name": "archive is stored", "method": "PUT", "path": "/v1/cache/" + nxk, "body": "archive-1", "status": 200, "headers": {}},
            {"name": "archive reads back as octet-stream", "method": "GET", "path": "/v1/cache/" + nxk, "status": 200, "body": "archive-1", "headers": {"content-type": "application/octet-stream"}},
            {"name": "overwriting is a conflict", "method": "PUT", "path": "/v1/cache/" + nxk, "body": "archive-2", "status": 409},
            {"name": "the first archive survives", "method": "GET", "path": "/v1/cache/" + nxk, "status": 200, "body": "archive-1"},
            {"name": "the hash may not contain upper case", "method": "GET", "path": "/v1/cache/ABC", "status": 404},
            {"name": "the route needs a hash", "method": "GET", "path": "/v1/cache/", "status": 404},
            {"name": "other methods are refused", "method": "DELETE", "path": "/v1/cache/" + nxk, "status": 405, "headers": {"allow": "GET, HEAD, PUT"}},
        ]},
        {"name": "namespaces partition every keyspace", "server": {"tokens": {}}, "steps": [
            {"name": "store in the default namespace", "method": "PUT", "path": "/v1/cache/" + nxk, "body": "default", "status": 200},
            {"name": "the same hash under another namespace is a miss", "method": "GET", "path": "/team-a/v1/cache/" + nxk, "status": 404},
            {"name": "store in the other namespace", "method": "PUT", "path": "/team-a/v1/cache/" + nxk, "body": "team-a", "status": 200},
            {"name": "each namespace keeps its own value", "method": "GET", "path": "/team-a/v1/cache/" + nxk, "status": 200, "body": "team-a"},
            {"name": "the default namespace is unchanged", "method": "GET", "path": "/v1/cache/" + nxk, "status": 200, "body": "default"},
            {"name": "Bazel blobs honour namespaces too", "method": "PUT", "path": "/team-a/cas/" + dh, "body": hello, "status": 200},
            {"name": "the default namespace has no such blob", "method": "GET", "path": "/cas/" + dh, "status": 404},
            {"name": "a namespace may not be a protocol route", "method": "GET", "path": "/status/cas/" + dh, "status": 404},
            {"name": "a namespace may not have upper case", "method": "GET", "path": "/Team/cas/" + dh, "status": 404},
        ]},
        {"name": "tokens: read token reads, write token writes, nothing else passes", "server": {"tokens": {"read": "read-secret", "write": "write-secret"}}, "steps": [
            {"name": "no credentials", "method": "GET", "path": "/v1/cache/" + nxk, "status": 401, "headers": {"content-type": "text/plain", "www-authenticate": "Bearer"}},
            {"name": "wrong token", "method": "GET", "path": "/v1/cache/" + nxk, "headers": auth("nope"), "status": 401},
            {"name": "basic scheme is not accepted", "method": "GET", "path": "/v1/cache/" + nxk, "headers": {"authorization": "Basic cmVhZC1zZWNyZXQ="}, "status": 401},
            {"name": "write token may write", "method": "PUT", "path": "/v1/cache/" + nxk, "headers": auth("write-secret"), "body": "archive", "status": 200},
            {"name": "read token may read", "method": "GET", "path": "/v1/cache/" + nxk, "headers": auth("read-secret"), "status": 200, "body": "archive"},
            {"name": "write token may read", "method": "GET", "path": "/v1/cache/" + nxk, "headers": auth("write-secret"), "status": 200, "body": "archive"},
            {"name": "read token may not write", "method": "PUT", "path": "/v1/cache/" + nxk + "0", "headers": auth("read-secret"), "body": "x", "status": 403},
            {"name": "the refused write stored nothing", "method": "GET", "path": "/v1/cache/" + nxk + "0", "headers": auth("read-secret"), "status": 404},
            {"name": "Bazel blobs need the same token", "method": "PUT", "path": "/cas/" + dh, "body": hello, "status": 401},
            {"name": "read token may not write blobs", "method": "PUT", "path": "/cas/" + dh, "headers": auth("read-secret"), "body": hello, "status": 403},
            {"name": "write token stores blobs", "method": "PUT", "path": "/cas/" + dh, "headers": auth("write-secret"), "body": hello, "status": 200},
            {"name": "health needs no token", "method": "GET", "path": "/healthz", "status": 200},
            {"name": "status needs a token", "method": "GET", "path": "/status", "status": 401},
            {"name": "status answers with a token", "method": "GET", "path": "/status", "headers": auth("read-secret"), "status": 200},
        ]},
        {"name": "bounds: an entry above the entry bound is refused before it is stored", "server": {"tokens": {}, "maximumBytes": 4096, "maximumEntryBytes": 16}, "steps": [
            {"name": "a small archive fits", "method": "PUT", "path": "/v1/cache/small", "body": "0123456789abcdef", "status": 200},
            {"name": "a larger archive is refused", "method": "PUT", "path": "/v1/cache/large", "body": "0123456789abcdefX", "status": 413},
            {"name": "the refused archive was not stored", "method": "GET", "path": "/v1/cache/large", "status": 404},
        ]},
    ],
}
def norm(step):
    request = {"method": step["method"], "path": step["path"]}
    response = {"status": step["status"]}
    headers = step.get("headers", {})
    if "authorization" in headers:
        request["headers"] = {"authorization": headers["authorization"]}
    if any(name != "authorization" for name in headers):
        response["headers"] = {name: value for name, value in headers.items() if name != "authorization"}
    if "body" in step:
        (request if step["method"] == "PUT" else response)["body"] = step["body"]
    return {"name": step["name"], "request": request, "response": response}

for scenario in wire["scenarios"]:
    scenario["steps"] = [norm(step) for step in scenario["steps"]]
dump("📡️wire.json", wire)

# store bounds and least-recently-used eviction: Python model of the same sequence is run by the test as the independent oracle
k = lambda name: H(name)
store = {
    "version": 1,
    "cases": [
        {"name": "least recently written entry is evicted first", "maximumBytes": 100, "maximumEntryBytes": 60, "ops": [
            {"put": "a", "size": 40}, {"put": "b", "size": 40}, {"put": "c", "size": 40}], "survivors": ["b", "c"]},
        {"name": "a read protects an entry from eviction", "maximumBytes": 100, "maximumEntryBytes": 60, "ops": [
            {"put": "a", "size": 40}, {"put": "b", "size": 40}, {"read": "a"}, {"put": "c", "size": 40}], "survivors": ["a", "c"]},
        {"name": "replacing an action result updates its size", "maximumBytes": 100, "maximumEntryBytes": 80, "ops": [
            {"put": "a", "size": 30}, {"put": "b", "size": 30}, {"put": "a", "size": 70}], "survivors": ["a"]},
        {"name": "eviction frees down to nine tenths", "maximumBytes": 100, "maximumEntryBytes": 60, "ops": [
            {"put": "a", "size": 30}, {"put": "b", "size": 30}, {"put": "c", "size": 30}, {"put": "d", "size": 30}], "survivors": ["b", "c", "d"]},
    ],
    "keys": {name: k(name) for name in "abcd"},
}
dump("💾️store.json", store)
print("fixtures written to", FIX)
