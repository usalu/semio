"""🕸️ Check the private origin never falls back within a selected module."""
from functools import partial
from io import StringIO
from pathlib import Path
import runpy
from tempfile import TemporaryDirectory
from threading import Thread
from urllib.error import HTTPError
from urllib.parse import quote
from urllib.request import urlopen

ticket = Path(__file__).resolve().parent
probe = runpy.run_path(str(ticket / "probe-owner-site.py"))
static = probe["static"]
generated = ticket / "🗑️generated"

with TemporaryDirectory(dir=generated, prefix="ot-") as scratch:
    directory = Path(scratch)
    site = directory / "site"
    overlay = directory / "overlay"
    module = Path("🔌️plugin-modules/🎪️demonstrator")
    (site / module).mkdir(parents=True)
    (overlay / module).mkdir(parents=True)
    (site / "index.html").write_text("captured page")
    (site / module / "old.js").write_text("stale fallback")
    (site / module / "actor.js").write_text("stale actor")
    (overlay / module / "actor.js").write_text("selected actor")
    (overlay / module / "actor.core.wasm.gz.bin").write_bytes(b"fixed encoded bytes")
    log = StringIO()
    handler = partial(probe["OverlayAsset"], directory=str(site), overlay=overlay, mounts=[module],
                      access_logger=static["access_logger"](log))
    with static["StaticOrigin"](("127.0.0.1", 0), handler) as server:
        thread = Thread(target=server.serve_forever, daemon=True)
        thread.start()
        try:
            origin = f"http://127.0.0.1:{server.server_port}"
            with urlopen(origin + "/") as response:
                assert response.read() == b"captured page"
            with urlopen(origin + quote(f"/{module.as_posix()}/actor.js") + "?v=7") as response:
                assert response.read() == b"selected actor"
            with urlopen(origin + quote(f"/{module.as_posix()}/actor.core.wasm.gz.bin")) as response:
                assert response.read() == b"fixed encoded bytes"
            try:
                urlopen(origin + quote(f"/{module.as_posix()}/old.js"))
            except HTTPError as error:
                assert error.code == 404
            else:
                raise AssertionError("Selected module fell through to stale captured files")
        finally:
            server.shutdown()
            thread.join()
    access = log.getvalue()
    assert ' 404 ' in access
    without_missing_path = "\n".join(line for line in access.splitlines() if ' 404 ' not in line)
    assert probe["verify_owner_requests"](without_missing_path, [module.as_posix()]) == 3
    without_core = "\n".join(line for line in without_missing_path.splitlines() if ".wasm.gz.bin" not in line)
    try:
        probe["verify_owner_requests"](without_core, [module.as_posix()])
    except ValueError as error:
        assert "Selected guest core was not fetched" in str(error)
    else:
        raise AssertionError("Private origin accepted a case with no selected core fetch")

print("Private origin selected the gzip guest while rejecting stale fallback.")
