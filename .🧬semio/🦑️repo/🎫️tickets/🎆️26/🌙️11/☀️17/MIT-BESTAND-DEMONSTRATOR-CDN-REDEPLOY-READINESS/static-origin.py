"""🌐 Serve the built CDN directory over persistent local HTTP connections."""
from contextlib import ExitStack
from functools import partial
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
import logging
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

    def __init__(self, *args, access_logger, **kwargs):
        self.access_logger = access_logger
        super().__init__(*args, **kwargs)

    def log_message(self, message, *args):
        self.access_logger.info("%s - - [%s] %s", self.address_string(), self.log_date_time_string(), message % args)


class StaticOrigin(ThreadingHTTPServer):
    request_queue_size = 128


def access_logger(stream):
    logger = logging.Logger("static-origin", logging.INFO)
    logger.addHandler(logging.StreamHandler(stream))
    return logger


if __name__ == "__main__":
    directory, port, *log_path = sys.argv[1:]
    with ExitStack() as resources:
        stream = resources.enter_context(open(log_path[0], "w", encoding="utf-8")) if log_path else sys.stderr
        handler = partial(StaticAsset, directory=directory, access_logger=access_logger(stream))
        with StaticOrigin(("127.0.0.1", int(port)), handler) as server:
            server.serve_forever()
