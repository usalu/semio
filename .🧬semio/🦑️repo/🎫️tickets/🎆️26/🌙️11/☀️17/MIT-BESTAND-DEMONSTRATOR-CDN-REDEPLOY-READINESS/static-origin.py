"""🌐 Serve the built CDN directory over persistent local HTTP connections."""
from functools import partial
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
import sys


class StaticAsset(SimpleHTTPRequestHandler):
    protocol_version = "HTTP/1.1"
    extensions_map = {
        **SimpleHTTPRequestHandler.extensions_map,
        ".wasm": "application/wasm",
        ".js": "text/javascript",
        ".mjs": "text/javascript",
        ".pbf": "application/x-protobuf",
        ".woff2": "font/woff2",
    }


class StaticOrigin(ThreadingHTTPServer):
    request_queue_size = 128


if __name__ == "__main__":
    directory, port = sys.argv[1:]
    with StaticOrigin(("127.0.0.1", int(port)), partial(StaticAsset, directory=directory)) as server:
        server.serve_forever()
