#!/usr/bin/env python3
"""🌐️ CDN-like static server for a built site: threaded, large listen backlog, no SPA/dev routes. usage: static-serve.py <root> <port>"""
import functools, http.server, sys

class Server(http.server.ThreadingHTTPServer):
    request_queue_size = 1024
    daemon_threads = True

root, port = sys.argv[1], int(sys.argv[2])
Server(("127.0.0.1", port), functools.partial(http.server.SimpleHTTPRequestHandler, directory=root)).serve_forever()
