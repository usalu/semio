# G1 Server Interface Verification

The g1 server has Ethernet on `en0` at `192.168.178.70` and Wi-Fi on `en1` at `192.168.178.71`. Wi-Fi was powered off at the start. Turning it on restored `192.168.178.71`.

Both addresses accepted TCP connections on NFS port 2049 and returned `/Users/Shared/semio-store` from `showmount -e`. Before the cache change, only Ethernet answered on the published cache port, 6094.

The cache uses one store and one fleet authorization policy across its listeners. It binds the IPv4 addresses present at startup in the fleet network and publishes them in the g1 cache facts. An interface that acquires its address after startup requires a cache sidecar restart to add its listener.

After the supervised restart, the g1 facts listed both `http://192.168.178.70:6094` and `http://192.168.178.71:6094`. The ticket's `verify.py` confirmed HTTP 200 and the same health response with curl and Python on both endpoints. Both endpoints rejected an anonymous cache read. The verification ran from the server. A worker-side mount and cache request remain unverified.
