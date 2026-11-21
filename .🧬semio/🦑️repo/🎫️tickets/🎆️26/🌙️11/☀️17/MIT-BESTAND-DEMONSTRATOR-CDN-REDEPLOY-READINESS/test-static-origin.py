"""🌐 Check the CDN test origin with a burst of independent HTTP connections."""
from concurrent.futures import ThreadPoolExecutor
from http.client import HTTPConnection
from pathlib import Path
import socket
import subprocess
import sys
import tempfile
import time

ticket = Path(__file__).resolve().parent
with tempfile.TemporaryDirectory(dir=ticket / "🗑️generated") as directory:
    site = Path(directory)
    (site / "index.html").write_bytes(b"site")
    (site / "guest.wasm").write_bytes(b"core")
    (site / "tile.pbf").write_bytes(b"tile")
    with socket.socket() as sock:
        sock.bind(("127.0.0.1", 0))
        port = sock.getsockname()[1]
    command = [sys.executable, "-m", "http.server", str(port), "--bind", "127.0.0.1", "--directory", directory] if "--baseline" in sys.argv else [sys.executable, str(ticket / "static-origin.py"), directory, str(port)]
    with subprocess.Popen(command, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL) as process:
        try:
            for _ in range(100):
                try:
                    with socket.create_connection(("127.0.0.1", port), timeout=.2):
                        break
                except OSError:
                    time.sleep(.025)
            def fetch(_):
                conn = HTTPConnection("127.0.0.1", port, timeout=4)
                try:
                    for path, status, mime, body in [("/guest.wasm",200,"application/wasm",b"core"),("/tile.pbf",200,"application/x-protobuf",b"tile"),("/gone",404,"text/html",None)]:
                        conn.request("GET",path)
                        response=conn.getresponse()
                        actual=response.read()
                        assert response.version==11 and (status==404 or not response.will_close)
                        assert response.status==status and response.getheader("Content-Type").split(";")[0]==mime
                        assert body is None or actual==body
                finally:
                    conn.close()
            with ThreadPoolExecutor(max_workers=48) as pool:
                list(pool.map(fetch, range(96)))
        finally:
            process.terminate()
            process.wait()
print("288 static requests passed.")
