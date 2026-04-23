"""
JSON-RPC 2.0 (NDJSON) client for the :mod:`semio` ``semio-store`` sidecar.
Transport matches :file:`semio/store/jsonrpc.rs` and the wasm ``KitStoreHandle`` method surface.
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
import threading
import typing
from queue import SimpleQueue
from types import TracebackType

_DEFAULT_EXE = "semio-store.exe" if sys.platform == "win32" else "semio-store"


def resolve_semio_store_binary() -> str:
    p = os.environ.get("SEMIO_STORE_BIN", "").strip()
    if p:
        return p
    here = os.path.dirname(os.path.abspath(__file__))
    for rel in (os.path.join("..", "..", "target", "release", _DEFAULT_EXE),):
        c = os.path.normpath(os.path.join(here, rel))
        if os.path.isfile(c):
            return c
    return _DEFAULT_EXE


class StoreClient:
    """
    Spawns :program:`semio-store` (one kit per process). :meth:`call` sends a JSON-RPC
    request and returns the ``result`` (or raises on ``error``). ``event`` lines are
    dropped unless you use :attr:`_on_event` via :meth:`on_event` (kept in sync
    in the read loop; production callers should set ``SEMIO_STORE_NO_EVENTS=1`` on
    the child to avoid backpressure on full pipes if they do not read events).
    """

    def __init__(self, binary: str | None = None) -> None:
        self._binary = binary or resolve_semio_store_binary()
        self._p: subprocess.Popen[bytes] | None = None
        self._lock = threading.Lock()
        self._next_id = 1
        self._pending: dict[int, SimpleQueue[dict[str, typing.Any]]] = {}
        self._reader: threading.Thread | None = None
        self._on_event: None | (typing.Callable[[dict[str, typing.Any]], None]) = None

    def __enter__(self) -> "StoreClient":
        self._ensure_proc()
        return self

    def __exit__(
        self,
        exc_type: type[BaseException] | None,
        exc: BaseException | None,
        tb: TracebackType | None,
    ) -> None:
        self.close()
        return None

    def on_event(
        self, handler: typing.Callable[[dict[str, typing.Any]], None] | None
    ) -> None:
        self._on_event = handler

    def _ensure_proc(self) -> None:
        with self._lock:
            if self._p and self._p.poll() is None:
                return
            self._p = subprocess.Popen(  # noqa: S603
                [self._binary],
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=sys.stderr,
                env={**os.environ, "RUST_LOG": os.environ.get("RUST_LOG", "error")},
            )
            if self._p.stdin is None or self._p.stdout is None:
                raise RuntimeError("subprocess did not get pipes")
            t = threading.Thread(
                target=self._read_loop,
                args=(self._p.stdout,),
                daemon=True,
            )
            t.start()
            self._reader = t

    def _read_loop(self, stream: typing.BinaryIO) -> None:
        while True:
            line = stream.readline()
            if not line:
                break
            s = line.decode("utf-8", errors="replace").strip()
            if not s:
                continue
            try:
                v = json.loads(s)
            except json.JSONDecodeError:
                continue
            if v.get("method") == "event" and "id" not in v:
                h = self._on_event
                if h:
                    p = v.get("params")
                    if isinstance(p, dict):
                        h(p)
                continue
            rid = v.get("id")
            if isinstance(rid, int) and rid in self._pending:
                self._pending[rid].put(v)

    def _next_req_id(self) -> int:
        with self._lock:
            n = self._next_id
            self._next_id += 1
            return n

    def call(self, method: str, params: dict[str, typing.Any] | list[typing.Any] | None) -> typing.Any:
        self._ensure_proc()
        if not self._p or self._p.stdin is None:
            raise RuntimeError("no stdin")
        rid = self._next_req_id()
        q: SimpleQueue[dict[str, typing.Any]] = SimpleQueue()
        self._pending[rid] = q
        msg: dict[str, typing.Any] = {
            "jsonrpc": "2.0",
            "id": rid,
            "method": method,
        }
        if params is not None:
            msg["params"] = params
        line = json.dumps(msg, ensure_ascii=False) + "\n"
        self._p.stdin.write(line.encode("utf-8"))
        self._p.stdin.flush()
        try:
            out = q.get(timeout=300.0)
        finally:
            self._pending.pop(rid, None)
        if "error" in out:
            e = out["error"]
            raise RuntimeError(
                f"jsonrpc {e.get('code')}: {e.get('message')}" if isinstance(e, dict) else str(e)
            )
        if "result" in out:
            return out["result"]
        return None

    def close(self) -> None:
        with self._lock:
            p = self._p
            self._p = None
        if p and p.poll() is None:
            try:
                if p.stdin:
                    p.stdin.write(
                        b'{"jsonrpc":"2.0","id":999999,"method":"server.shutdown"}\n'
                    )
                    p.stdin.flush()
            except (BrokenPipeError, OSError):
                pass
            p.wait(timeout=5)


def _ndjson_read_until_id(stream: typing.BinaryIO, want_id: int) -> dict[str, typing.Any]:
    while True:
        line = stream.readline()
        if not line:
            raise RuntimeError("unexpected EOF on semio-store stdout")
        s = line.decode("utf-8", errors="replace").strip()
        if not s:
            continue
        v = json.loads(s)
        if v.get("id") == want_id and "result" in v:
            return v
        if "error" in v and v.get("id") == want_id:
            e = v["error"]
            raise RuntimeError(
                f"jsonrpc {e.get('code')}: {e.get('message')}" if isinstance(e, dict) else str(e)
            )


def load_kit_via_io(
    method: str,
    params: dict[str, typing.Any],
    binary: str | None = None,
) -> dict[str, typing.Any]:
    """
    One-off import + :meth:`kit.snapshot` in a private sidecar
    (``SEMIO_STORE_NO_EVENTS=1``; no ``event`` flood). ``method`` is e.g. ``"io.importFromFile"``.
    """
    b = binary or resolve_semio_store_binary()
    child = subprocess.Popen(  # noqa: S603
        [b],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        env={**os.environ, "SEMIO_STORE_NO_EVENTS": "1", "RUST_LOG": "error"},
    )
    if child.stdin is None or child.stdout is None:
        raise RuntimeError("no pipes on semio-store")
    try:
        msg1: dict[str, typing.Any] = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params,
        }
        child.stdin.write((json.dumps(msg1, ensure_ascii=False) + "\n").encode("utf-8"))
        child.stdin.flush()
        _ndjson_read_until_id(child.stdout, 1)
        msg2: dict[str, typing.Any] = {
            "jsonrpc": "2.0",
            "id": 2,
            "method": "kit.snapshot",
        }
        child.stdin.write((json.dumps(msg2, ensure_ascii=False) + "\n").encode("utf-8"))
        child.stdin.flush()
        v = _ndjson_read_until_id(child.stdout, 2)
        r = v.get("result")
        if not isinstance(r, dict):
            raise TypeError("kit.snapshot: expected object")
        return r
    finally:
        try:
            if child.stdin:
                child.stdin.write(
                    b'{"jsonrpc":"2.0","id":3,"method":"server.shutdown"}\n'
                )
                child.stdin.flush()
        except (BrokenPipeError, OSError):
            pass
        child.wait(timeout=5)
