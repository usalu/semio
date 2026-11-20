import http.client
import json
import subprocess
import sys
from urllib.parse import urlsplit

facts = json.load(sys.stdin)
for endpoint in sys.argv[1:]:
    assert endpoint in facts["cache"]["endpoints"], f"{endpoint} is missing from fleet facts"
    url = urlsplit(endpoint)
    health = endpoint + "/healthz"
    curl = json.loads(subprocess.check_output(
        ["curl", "--noproxy", "*", "--fail", "--silent", "--show-error", "--max-time", "5", health],
        text=True,
    ))
    client = http.client.HTTPConnection(url.hostname, url.port, timeout=5)
    try:
        client.request("GET", "/healthz")
        response = client.getresponse()
        assert response.status == 200
        assert json.load(response) == curl == {"status": "ok"}
        client.request("GET", "/v1/cache/dual-interface-verification")
        response = client.getresponse()
        assert response.status in (401, 403), response.status
        response.read()
    finally:
        client.close()
    print(f"{endpoint} responded and rejected anonymous cache access")
