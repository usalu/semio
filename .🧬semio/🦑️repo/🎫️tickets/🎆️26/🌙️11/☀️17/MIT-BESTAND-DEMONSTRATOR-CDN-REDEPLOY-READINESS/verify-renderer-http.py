"""🌐 Verify every current Renderer support byte over the static CDN server."""
import argparse
from functools import partial
import hashlib
import importlib.util
import json
from pathlib import Path, PurePosixPath
import sys
import threading
from urllib.parse import quote
from urllib.request import ProxyHandler, build_opener


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("site", type=Path)
    parser.add_argument("inventory", type=Path)
    args = parser.parse_args()
    spec = importlib.util.spec_from_file_location("static_origin", Path(__file__).with_name("static-origin.py"))
    origin = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(origin)
    rows = json.loads(args.inventory.read_text(encoding="utf-8"))
    paths = [entry["path"] for entry in rows]
    if not paths or len(paths) != len(set(paths)):
        raise ValueError("Require unique authored Renderer support paths")
    for path in paths:
        decoded = PurePosixPath(path)
        if decoded.parts[0] != "renderer-modules" or ".." in decoded.parts or decoded.as_posix() != path:
            raise ValueError(f"Invalid Renderer support URL {path}")
    fonts = [path for path in paths if path.lower().endswith((".otf", ".ttf", ".woff", ".woff2"))]
    if not fonts:
        raise ValueError("Require the complete Renderer font inventory")

    handler = partial(origin.StaticAsset, directory=args.site.resolve(), access_logger=origin.access_logger(sys.stderr))
    opener = build_opener(ProxyHandler({}))
    response_types = {}
    with origin.StaticOrigin(("127.0.0.1", 0), handler) as server:
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        try:
            base = f"http://127.0.0.1:{server.server_port}/"
            for entry in rows:
                with opener.open(base + quote(entry["path"], safe="/"), timeout=10) as response:
                    size = entry["bytes"]
                    body = response.read(size + 1)
                    if response.status != 200 or int(response.headers["Content-Length"]) != size or len(body) != size or hashlib.sha256(body).hexdigest() != entry["sha256"]:
                        raise ValueError(f"Wrong static response for {entry['path']}")
                    response_types[entry["path"]] = response.headers["Content-Type"]
        finally:
            server.shutdown()
            thread.join()
    print(json.dumps({"requests": len(paths), "fonts": len(fonts), "responseTypes": response_types}, ensure_ascii=False))


if __name__ == "__main__":
    main()
