# WGPU Browser Origin Isolation

The WGPU config served the page and renderer JavaScript without cross-origin isolation headers. The launch probe requires the policy on the document, and the worker-backed browser shell uses shared memory when available. Neither the completed WGPU configuration nor the repository Vite listener added the headers.

The WGPU server now installs the repository's existing playground header middleware before the static body routes. Both the page and the mounted module receive `Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: credentialless`.

A language-neutral two-route fixture started real pinned Vite with the actual WGPU server config and small staging fixtures. Both requests initially returned 200 with absent headers. Both passed after the change, including the mounted renderer JavaScript response. The live Layout probe still verifies the real component and renderer artifacts once they publish.
