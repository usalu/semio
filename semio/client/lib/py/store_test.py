"""Smoke test: :mod:`store` talks to :program:`semio-store` if built."""

from __future__ import annotations

import os

import pytest

import semio.client.lib.py.store as store


@pytest.mark.skipif(
    not os.path.isfile(store.resolve_semio_store_binary()),
    reason="semio-store binary not found; set SEMIO_STORE_BIN or cargo build -p semio-store",
)
def test_generate_id_roundtrip() -> None:
    with store.StoreClient() as c:
        s = c.call("semio.generateId", {})
        assert isinstance(s, str) and len(s) > 8
        dto = {
            "id": s,
            "name": "py-store-test",
            "types": [],
            "designs": [],
        }
        c.call("kit.create", {"dto": dto})
        snap = c.call("kit.snapshot", None)
        assert isinstance(snap, dict)
        assert snap.get("name") == "py-store-test"
